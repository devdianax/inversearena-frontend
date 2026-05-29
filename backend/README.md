# Backend Payout Execution

This folder contains the Soroban payout execution layer for winner distributions.

## Security

### HTTP Security Headers

The backend API is configured with comprehensive security headers via Helmet:

- **HSTS (Strict-Transport-Security)**: Forces HTTPS for 1 year, including subdomains and preload
- **Referrer-Policy**: Set to `strict-origin-when-cross-origin` for privacy and analytics balance
- **Cross-Origin-Opener-Policy**: Set to `same-origin` for improved isolation
- **Cross-Origin-Resource-Policy**: Set to `cross-origin` to allow frontend resource loading
- **X-Frame-Options**: Prevents clickjacking attacks
- **X-Content-Type-Options**: Prevents MIME-sniffing
- **X-DNS-Prefetch-Control**: Controls DNS prefetching
- **X-Permitted-Cross-Domain-Policies**: Disables Adobe Flash/PDF cross-domain policies

Content Security Policy (CSP) is disabled in the backend as it's handled by the Next.js frontend.

---

## Feature flags and env

Set these values in deployment secrets (never commit private keys):

- `PAYOUTS_LIVE_EXECUTION` (`true`/`false`): submit transactions to Soroban when `true`.
- `PAYOUTS_SIGN_WITH_HOT_KEY` (`true`/`false`): enable hot-key signing in service.
- `PAYOUT_HOT_SIGNER_SECRET`: optional hot signer secret (only for controlled environments).
- `PAYOUTS_MAX_GAS_STROOPS`: max accepted prepared transaction fee.
- `PAYOUTS_MAX_ATTEMPTS`: max worker submit retries before marking failed.
- `PAYOUTS_CONFIRM_POLL_MS`: confirmation polling interval.
- `PAYOUTS_CONFIRM_MAX_POLLS`: max confirmation polls.
- `PAYOUT_CONTRACT_ID`: Soroban payout contract.
- `PAYOUT_METHOD_NAME`: contract method (default `distribute_winnings`).
- `PAYOUT_SOURCE_ACCOUNT`: payout source account.
- `STELLAR_NETWORK_PASSPHRASE`: network passphrase.
- `SOROBAN_RPC_URL`: Soroban RPC endpoint.

## Key management approach

- Preferred production mode: `PAYOUTS_SIGN_WITH_HOT_KEY=false`.
- Build unsigned XDR server-side, then sign in an external KMS/HSM signer.
- Return signed XDR to `queueSignedTransaction` for worker submission.
- If hot signing is enabled, keep `PAYOUT_HOT_SIGNER_SECRET` in secret manager only.
