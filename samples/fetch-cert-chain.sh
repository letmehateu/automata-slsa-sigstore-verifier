#!/bin/bash
set -e

# Fetch certificate chain for a given Sigstore bundle
# Usage: ./fetch-root-cert.sh <path-to-bundle.json>
# Outputs: Comma-separated DER-encoded certificate chain (hex) to stdout
#          Chain order: intermediate, root (and any other CAs in between)
#
# Chain structure:
#   GitHub Fulcio (fulcio.githubapp.com):
#     [0] Fulcio Intermediate l2 (issued by l1)
#     [1] Fulcio Intermediate l1 (issued by root)
#     [2] Internal Services Root (self-signed)
#
#   Public Sigstore (fulcio.sigstore.dev):
#     [0] sigstore-intermediate (issued by root)
#     [1] sigstore root (self-signed)

if [ "$#" -ne 1 ]; then
    echo "Error: Missing bundle path" >&2
    echo "Usage: $0 <bundle.json>" >&2
    exit 1
fi

BUNDLE_PATH="$1"

if [ ! -f "$BUNDLE_PATH" ]; then
    echo "Error: File not found: $BUNDLE_PATH" >&2
    exit 1
fi

# Extract certificate from bundle and decode
CERT_BASE64=$(jq -r '.verificationMaterial.certificate.rawBytes' "$BUNDLE_PATH")

if [ -z "$CERT_BASE64" ] || [ "$CERT_BASE64" = "null" ]; then
    echo "Error: Could not extract certificate from bundle" >&2
    exit 1
fi

# Decode certificate to temp file
TEMP_CERT=$(mktemp)
trap "rm -f $TEMP_CERT" EXIT
# Try -D for macOS, fall back to -d for Linux
if echo "$CERT_BASE64" | base64 -D > "$TEMP_CERT" 2>/dev/null; then
    : # Success with -D
elif echo "$CERT_BASE64" | base64 -d > "$TEMP_CERT" 2>/dev/null; then
    : # Success with -d
else
    echo "Error: Failed to decode certificate" >&2
    exit 1
fi

# Extract issuer CN (Common Name)
# Format can be "Issuer: CN=..." or "issuer=...CN=..."
ISSUER_CN=$(openssl x509 -in "$TEMP_CERT" -inform DER -noout -issuer | \
    sed -n 's/.*CN[= ]*\([^,]*\).*/\1/p')

if [ -z "$ISSUER_CN" ]; then
    echo "Error: Could not extract issuer CN from certificate" >&2
    exit 1
fi

# Determine which Fulcio instance to fetch chain from based on issuer
case "$ISSUER_CN" in
    "Fulcio Intermediate l2")
        # GitHub's Fulcio instance
        ROOT_URL="https://fulcio.githubapp.com/api/v2/trustBundle"
        ;;
    "sigstore-intermediate")
        # Public-good Sigstore instance
        ROOT_URL="https://fulcio.sigstore.dev/api/v2/trustBundle"
        ;;
    *)
        echo "Error: Unknown issuer CN: $ISSUER_CN" >&2
        exit 1
        ;;
esac

# Fetch trust bundle and extract full certificate chain
TEMP_BUNDLE=$(mktemp)
trap "rm -f $TEMP_CERT $TEMP_BUNDLE" EXIT

# Fetch the trust bundle
curl -s "$ROOT_URL" > "$TEMP_BUNDLE"

# Get the number of certificates in the chain
CERT_COUNT=$(jq -r '.chains[0].certificates | length' "$TEMP_BUNDLE")

if [ -z "$CERT_COUNT" ] || [ "$CERT_COUNT" = "null" ] || [ "$CERT_COUNT" -eq 0 ]; then
    echo "Error: Could not fetch certificate chain from $ROOT_URL" >&2
    exit 1
fi

# Build comma-separated hex chain
CHAIN=""
for ((i=0; i<CERT_COUNT; i++)); do
    # Extract certificate to temp PEM file
    TEMP_PEM=$(mktemp)
    TEMP_DER=$(mktemp)
    trap "rm -f $TEMP_CERT $TEMP_BUNDLE $TEMP_PEM $TEMP_DER" EXIT

    jq -r ".chains[0].certificates[$i]" "$TEMP_BUNDLE" > "$TEMP_PEM"

    # Convert PEM to DER
    openssl x509 -in "$TEMP_PEM" -outform DER -out "$TEMP_DER"

    # Convert to hex (WITHOUT 0x prefix - vm.ffi auto-decodes it!)
    CERT_HEX=$(xxd -p -c 256 "$TEMP_DER" | tr -d '\n')

    # Add to chain (comma-separated)
    if [ -z "$CHAIN" ]; then
        CHAIN="$CERT_HEX"
    else
        CHAIN="$CHAIN,$CERT_HEX"
    fi

    # Clean up temp files for this iteration
    rm -f "$TEMP_PEM" "$TEMP_DER"
done

# Output the full chain
echo "$CHAIN"
