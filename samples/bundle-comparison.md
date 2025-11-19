# Sigstore Bundle Format Comparison

## Two Samples - Two Different Verification Methods

### Sample 1: RFC3161 Timestamp (13531551)
```
File: actions-attest-build-provenance-attestation-13531551.sigstore.json
```

**Verification Material:**
- ✅ RFC3161 Timestamp Authority signature
- ❌ NO Rekor transparency log entry
- Uses: `verificationMaterial.timestampVerificationData.rfc3161Timestamps`

**How to Verify:**
1. Verify RFC3161 timestamp signature
2. Verify certificate was valid at timestamp time
3. Verify DSSE envelope signature

---

### Sample 2: Rekor Transparency Log (13532655)
```
File: actions-attest-build-provenance-attestation-13532655.sigstore.json
```

**Verification Material:**
- ✅ Rekor transparency log entry with Merkle proof
- ❌ NO RFC3161 timestamp (empty object)
- Uses: `verificationMaterial.tlogEntries[0]`

**Rekor Data (from bundle):**
```json
{
  "logIndex": "707288064",
  "integratedTime": "1763454699",
  "inclusionProof": {
    "logIndex": "585383802",
    "treeSize": "585383803",
    "rootHash": "r/bTFC+gN/oyGdCqEBRUuOvBsDWm4p86X6DohvgjbD4=",
    "hashes": [17 Merkle sibling hashes]
  },
  "inclusionPromise": {
    "signedEntryTimestamp": "MEYCIQCSuPFqwGmU3GBpDWjOAQLRs/LJKrCdYuKEJ63LU3KxJgIhAPNmSiE5ySag1+t8G3IHX9WXL7Q/B5A+FL1TFJyZLd75"
  }
}
```

**How to Verify:**
1. Verify certificate was valid at `integratedTime` (Unix: 1763454699 = 2025-11-18 08:31:39 UTC)
2. Verify DSSE envelope signature
3. Verify Merkle inclusion proof against Rekor tree root
4. (Optional) Verify signed entry timestamp

---

## Mapping to Solidity AttestationRequest.RekorEntry

From **Sample 2** (`13532655.sigstore.json`):

```solidity
struct RekorEntry {
    uint64 logIndex;              // 707288064 (top-level logIndex)
    bytes32 logRoot;              // Base64 decode "r/bTFC+gN/oyGdCqEBRUuOvBsDWm4p86X6DohvgjbD4="
    bytes32[] inclusionProof;     // Base64 decode each of the 17 hashes
    bytes32 entryHash;            // Compute from canonicalizedBody or use logIndex as UUID
    bytes32 signedEntryTimestamp; // Base64 decode "MEYCIQCSuPFqwGmU3GBpDWjOAQLRs/LJKrCdYuKEJ63LU3KxJgIhAPNmSiE5ySag1+t8G3IHX9WXL7Q/B5A+FL1TFJyZLd75"
    uint256 integratedTime;       // 1763454699
}
```

### Extracting the Data

#### logRoot
```bash
echo "r/bTFC+gN/oyGdCqEBRUuOvBsDWm4p86X6DohvgjbD4=" | base64 -d | xxd -p
# Output: aff6d3142fa037fa3219d0aa10145458b8ebc1b035a6e29f3a5fa0e886f8236c3e
```

#### inclusionProof (array of 17 hashes)
Each hash in the `hashes` array needs to be base64 decoded to bytes32.

Example for first hash:
```bash
echo "2JejhxsQcOK8j9HxJNLEr05wm32xHmeFpQNHBns+TJo=" | base64 -d | xxd -p
# Output: d897a3871b1070e2bc8fd1f124d2c4af4e709b7db11e6785a503470e7b3e4c9a
```

#### signedEntryTimestamp
```bash
echo "MEYCIQCSuPFqwGmU3GBpDWjOAQLRs/LJKrCdYuKEJ63LU3KxJgIhAPNmSiE5ySag1+t8G3IHX9WXL7Q/B5A+FL1TFJyZLd75" | base64 -d | xxd -p
# This is a DER-encoded ECDSA signature
```

---

## Recommendation for Your Contract

**Use Sample 2 (Rekor-based) as the primary verification path** because:

1. ✅ Rekor data is **already embedded** in the bundle
2. ✅ No external API calls needed - all data is self-contained
3. ✅ Your contract already implements Merkle proof verification
4. ✅ Matches the existing `AttestationTypes.RekorEntry` structure

For RFC3161 support (Sample 1), you would need to:
- Implement RFC3161 timestamp verification (complex ASN.1 parsing)
- Add support for timestamp authority public keys
- Modify the contract architecture

**Start with Rekor (Sample 2), add RFC3161 support later if needed.**
