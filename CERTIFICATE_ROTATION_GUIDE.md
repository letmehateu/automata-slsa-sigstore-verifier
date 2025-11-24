# Certificate Rotation Guide

This document provides analysis and recommendations for fetching Fulcio and TSA issuer chains from the trusted root.

## Analysis Summary

Based on the certificates in `samples/trusted_root.jsonl`, here are the validity patterns:

### GitHub Fulcio CA & TSA

**Rotation Pattern:**
- Average validity period: **206 days (~6.9 months)**
- Range: 165-255 days
- Overlap period: **~20 days** (grace period for smooth transitions)
- Certificates rotate together (Fulcio and TSA in sync)

**Historical Rotation Timeline:**
```
2023-10-27 → 2024-05-25 (210 days) ─┐
                                      ├─ 12 day overlap
2024-05-13 → 2024-10-25 (165 days) ──┤
                                      ├─ 18 day overlap
2024-10-07 → 2025-06-19 (255 days) ──┤
                                      ├─ 23 day overlap
2025-05-27 → 2025-12-09 (196 days) ──┤
                                      ├─ 26 day overlap
2025-11-13 → [CURRENT]               ─┘
```

### Sigstore Public Good (sigstore.dev)

**Rotation Pattern:**
- Fulcio CA validity: **~2 years** (long-lived)
- TSA validity: **~10 years** (very long-lived)
- These change infrequently

## Recommendations

### 📅 Fetch Frequency

#### For GitHub Attestations
```
Recommended:  Every 60-90 days (2-3 months)
Minimum:      Every 103 days (~3.5 months)
Critical:     At least 20 days before expiry
```

**Rationale:**
- GitHub rotates every ~6-7 months
- 20-day overlap provides grace period
- Fetching every 2-3 months ensures you're never caught off guard
- Allows time to test and deploy new roots before old ones expire

#### For Sigstore Public Good
```
Recommended:  Every 90-180 days (3-6 months)
Minimum:      Every 6 months
```

**Rationale:**
- Certificates are very long-lived
- Changes are infrequent and usually announced
- Quarterly checks are sufficient for most use cases

### ⚡ Best Practices

1. **Automated Fetching**
   - Set up a cron job or scheduled task
   - Fetch both Fulcio and TSA chains together (they rotate in sync)
   - Log all fetch operations for auditing

2. **Verification**
   - Always verify new roots before replacing old ones
   - Check certificate chain validity
   - Ensure proper X.509 encoding

3. **Version Management**
   - Keep N-1 versions during grace periods
   - Maintain backwards compatibility during transitions
   - Document which version is currently in use

4. **Monitoring**
   - Set up alerts for certificates expiring within 30 days
   - Monitor fetch failures and retry with exponential backoff
   - Track certificate rotation patterns over time

5. **Event-Driven Updates**
   - Subscribe to GitHub's security advisories
   - Monitor Sigstore's announcements
   - Implement webhook-based updates if available

### 🚨 Warning Signs

Take immediate action if:
- Latest certificate expires in < 30 days
- Fetch operations consistently fail
- Certificate chain validation fails
- You receive security advisories about certificate updates

### 📊 Implementation Example

```bash
# Cron job for fetching roots (runs every 60 days)
0 0 1 */2 * /path/to/fetch-trusted-roots.sh

# Script should:
# 1. Fetch latest trusted_root.json from TUF repository
# 2. Verify signatures and metadata
# 3. Extract Fulcio and TSA certificates
# 4. Validate certificate chains
# 5. Update local trust store
# 6. Keep backup of previous version
# 7. Log operation results
```

### 🔄 GitHub Certificate Rotation Characteristics

- **Predictable**: Rotates every 5-8 months
- **Overlap**: 2-3 week grace period
- **Synchronized**: Fulcio and TSA rotate together
- **Multiple Active**: 2-3 certificates valid during transition

### 📈 Current Status

As of 2025-11-24:
- **GitHub**: Latest certificate (2025-11-13) has no expiry date set (currently active)
- **Sigstore**: Long-lived certificates still valid

### 🔧 Tools

Use the provided analysis scripts to check your local trusted roots:

```bash
# Quick analysis
./analyze_cert_validity.sh

# Detailed analysis with recommendations
python3 analyze_cert_rotation.py
```

## References

- [Sigstore Trust Root Specification](https://github.com/sigstore/root-signing)
- [TUF (The Update Framework)](https://theupdateframework.io/)
- [GitHub Artifact Attestations](https://docs.github.com/en/actions/security-guides/using-artifact-attestations)

## Notes

- This analysis is based on historical data in `samples/trusted_root.jsonl`
- Patterns may change; always verify against current documentation
- When in doubt, fetch more frequently rather than less
- Certificate rotation is a critical security operation; don't skip it
