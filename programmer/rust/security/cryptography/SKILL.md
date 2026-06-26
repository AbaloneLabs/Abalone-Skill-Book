---
name: security-cryptography
description: Rust cryptography rules - never roll your own, use vetted crates, and common pitfalls with hashing, encryption, secrets, and randomness. Use when hashing passwords, encrypting data, generating tokens, handling secrets, or choosing crypto crates in Rust.
---

# Cryptography

Cryptographic correctness is unforgiving - a tiny mistake is invisible until it's exploited. Rust doesn't save you here. Keep these rules sacred.

## Golden Rules

1. **Never implement your own crypto.** Use vetted crates: `ring`, `rustls`, `argon2`, `aes-gcm`, `rand`.
2. **Never use a custom RNG for security.** Always `rand::rngs::OsRng` or `ring::rand::SystemRandom`.
3. **Never hardcode secrets in source.** Use environment variables or a secrets manager.

## Password Hashing

Use Argon2id (memory-hard, modern). Never MD5/SHA for passwords.
```rust
use argon2::{Argon2, PasswordHasher};
let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default().hash_password(pw, &salt)?;
```
- Each password gets a unique random salt.
- Store the full PHC string (includes salt + params); you don't manage the salt separately.

## Encryption

- **At rest, authenticated**: use AES-GCM or ChaCha20-Poly1305. Never raw AES-CBC without a MAC (tamperable).
- **Never reuse a nonce/key pair** with AES-GCM. Reusing a nonce leaks the key.
- **TLS in transit**: use `rustls`, never disable certificate verification.

## Common Traps

### Constant-time comparison for secrets
Comparing tokens/hashes with `==` leaks timing information. Use `subtle::ConstantTimeEq` or `ring::constant_time::verify_slices_are_equal`.

### Using `rand::thread_rng()` where `OsRng` is needed
`thread_rng()` is fine for most uses but for key generation, prefer `OsRng` to be safe against compromised RNG state.

### Storing secrets in logs or error messages
`tracing::info!("auth token: {}", token)` - secrets in logs are a breach. Redact secrets before logging.

### ECB mode
ECB is insecure (reveals patterns). Never use it. Use GCM or authenticated modes.

## Self-Check

- [ ] Are you using vetted crypto crates (not hand-rolled)?
- [ ] Are passwords hashed with Argon2id (not MD5/SHA)?
- [ ] Is the RNG `OsRng`/`SystemRandom` for security-sensitive randomness?
- [ ] Are encryption modes authenticated (GCM/Poly1305, not ECB/CBC-without-MAC)?
- [ ] Are nonces unique per key (never reused)?
- [ ] Are secret comparisons constant-time?
- [ ] Are secrets excluded from logs and error messages?
