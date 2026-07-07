---
name: cryptographic_primitive_selection.md
description: Use when the agent is choosing or using cryptographic primitives — hashing, MACs, symmetric or asymmetric encryption, key derivation, random number generation, password storage, token signing, TLS configuration, or JWT signing — and must select the right algorithm, mode, key length, and library rather than inventing or combining crypto. Also use when reviewing code that performs encryption, signature verification, password hashing, token generation, or randomness, or when deciding whether to roll custom crypto versus use a vetted library.
---

# Cryptographic Primitive Selection

Cryptography is unusual among engineering disciplines because the wrong choice usually looks correct. A developer hashes a password with SHA-256 and feels secure; the hash is fast, unsalted, and brute-forceable in minutes. A developer encrypts with AES-ECB and the output is deterministic and leaks plaintext structure. A developer seeds a random number generator with the current time and produces predictable tokens. The defining property of cryptographic mistakes is that the code runs, the tests pass, and the output looks like gibberish — yet the security guarantee is absent. There is no compiler error for using a broken primitive, no test failure for a weak key, no runtime warning for a predictable nonce. The defect is invisible until an attacker exploits it, which may be years later.

Agents tend to treat cryptography as a matter of picking an algorithm name they have heard of (AES, RSA, SHA) and calling a library function. That is not enough. Security lives in the mode of operation, the key derivation, the nonce handling, the constant-time comparison, the authenticated-versus-encrypted distinction, and the provenance of the randomness. A correct algorithm in the wrong mode, or with a reused nonce, or compared with a non-constant-time equality check, is broken. The judgment problem is recognizing that cryptography is a field where "I implemented it and it works" is not evidence of security, and where the safe path is almost always to use a high-level, audited library or protocol rather than to assemble primitives by hand.

## Core Rules

### Never Invent Cryptography; Use Vetted High-Level Constructions

The single most important rule is: do not design cryptographic protocols or combine primitives yourself, even if each individual primitive is sound. Cryptographic composition is where subtle flaws appear — authenticated encryption assembled from separate encrypt-and-MAC steps, key agreement built from manual Diffie-Hellman, signature schemes glued together. Use a high-level library or protocol that has been designed and audited as a whole.

- **Prefer authenticated encryption (AEAD) modes.** For symmetric encryption, use AES-GCM or ChaCha20-Poly1305, which provide both confidentiality and integrity in one vetted construction. Never use a non-authenticated mode (ECB, CBC, CTR, stream ciphers without a MAC) for data that must resist tampering.
- **Prefer protocol libraries over primitive libraries.** TLS libraries, JWT libraries with verified-algorithm allow-lists, age or libsodium for file/message encryption, and established KDFs (Argon2, scrypt, bcrypt, PBKDF2) exist precisely so that you do not assemble primitives. Use them.
- **If you must use a primitive directly, document why a high-level construction was insufficient.** The exception must be justified, not the default.

### Match The Primitive To The Goal, And Know The Goal Exactly

Cryptographic primitives serve distinct goals, and using a primitive for the wrong goal is a common and severe error. Name the goal before choosing the primitive:

- **Confidentiality** (hide data): symmetric encryption (AES-GCM, ChaCha20-Poly1305) for data at rest or in transit with a shared key; asymmetric encryption (RSA-OAEP, ECIES) when only the recipient's public key is available.
- **Integrity and authenticity** (detect tampering, prove origin): MAC (HMAC) for shared-key settings; digital signatures (Ed25519, ECDSA, RSA-PSS) for public-key settings. A hash alone provides neither — it is unkeyed and anyone can compute it.
- **Password storage** (verify a password without storing it): a slow, salted, memory-hard KDF (Argon2id preferred, then scrypt, bcrypt, PBKDF2 with high iterations). Never store password hashes with a fast hash like SHA-256 or MD5.
- **Token/session identifier generation**: a CSPRNG producing sufficient entropy (at least 128 bits). Not a hash of a timestamp, not a UUIDv4 from a non-crypto RNG, not a sequential counter.
- **Key derivation from a password or low-entropy input**: a KDF designed for low-entropy inputs (Argon2, scrypt, bcrypt, PBKDF2), never a plain hash. For deriving multiple keys from a high-entropy master key, use HKDF.
- **Commitment / fingerprinting** (detect that content changed, no secrecy): a cryptographic hash (SHA-256, SHA-3, BLAKE2/3). Add a key (HMAC) if authenticity against an active attacker is required.

### Handle Nonces, IVs, And Initialization Vectors Correctly

Symmetric encryption modes that use a nonce or IV (GCM, CTR, CBC) are catastrophically broken if the nonce is reused with the same key. GCM with a reused nonce leaks the authentication key and the XOR of plaintexts. This is not a theoretical concern — it has broken real systems.

- **Never reuse a nonce with the same key.** For GCM, the 96-bit nonce must be unique per key. Use a counter, or generate a random nonce (acceptable if the key is rotated before the birthday bound of 2^32 messages).
- **Do not derive the nonce from predictable or colliding inputs.** A nonce based on message length, a low-resolution timestamp, or a hash of the plaintext will collide.
- **Prefer schemes with built-in nonce management.** XChaCha20-Poly1305 and AES-GCM-SIV are designed to be safe even if a nonce is accidentally reused, reducing a catastrophic failure to a confidentiality-only one. Prefer them when nonce misuse is plausible.

### Use Constant-Time Comparison For Secrets

Comparing a supplied token, MAC, or password hash with an expected value using a standard equality operator (`==`) leaks information through timing: the comparison returns faster on the first differing byte, enabling a byte-by-byte timing attack that recovers the secret. This has broken real authentication systems.

- **Always compare secrets with a constant-time comparison function** provided by the crypto library (`crypto.subtle.timingSafeEqual`, `CRYPTO_memcmp`, `hmac.Equal`, `constant_time_compare`).
- **Apply this to MAC verification, signature verification results, token checks, and password hash comparisons** — anywhere a secret or its derived value is compared against attacker-controlled input.
- **Hash the input first if comparing long secrets.** For password verification, the KDF's compare function handles this; for API token comparison, compare HMACs of both values, or use the library's constant-time compare directly.

### Generate Randomness From A Cryptographically Secure Source

Predictable randomness breaks tokens, keys, nonces, and session identifiers. The mistake is using a non-cryptographic PRNG (a language's default `Math.random`, `rand`, `random.randint`) for a security-sensitive value.

- **Use a CSPRNG for all security values:** keys, tokens, session ids, nonces, password reset codes, CSRF tokens, salts (salts need uniqueness, not secrecy, but a CSPRNG satisfies both).
- **Request enough entropy.** A 64-bit token is brute-forceable; use at least 128 bits (16 bytes, 32 hex characters) for unguessable tokens. Shorter codes (e.g., 6-digit OTPs) are acceptable only when paired with rate limiting and expiry.
- **Do not seed a CSPRNG yourself.** Let the operating system provide entropy. Manually seeding (with time, PID, or a fixed value) destroys the security guarantee.

### Choose Key Sizes And Algorithms For The Long Term

Key sizes and algorithms must resist attack not just today but for the lifetime of the protected data. A key that is borderline today will be breakable in five years; data encrypted now may still be sensitive then.

- **Symmetric keys: at least 128 bits; 256 bits for long-term or high-value data.** AES-128 is currently adequate; AES-256 provides margin against future cryptanalysis and is required for some compliance regimes.
- **Hash functions: SHA-256 or SHA-3 (256-bit) for general use; SHA-384/512 where a larger security level is needed.** Avoid SHA-1 and MD5 for any security purpose; both have practical collisions.
- **Asymmetric algorithms: prefer modern choices.** Ed25519 for signatures (fast, secure, no parameter pitfalls); RSA with at least 3072 bits where RSA is required; ECDH over Curve25519 for key agreement. Avoid RSA-1024 and legacy curves (secp... curves with poor parameter choices).
- **Rotate keys on a schedule and on compromise.** Design for key rotation from the start; a system that cannot rotate a key without downtime is a system that will run a compromised key.

## Common Traps

### Using A Fast Hash For Passwords

Storing passwords hashed with SHA-256 or MD5. These are designed for speed, which is exactly what an attacker brute-forcing a stolen password database wants. Use a slow, memory-hard KDF (Argon2id).

### Encrypting Without Authenticating

Using AES-CBC or AES-CTR or a raw stream cipher without a MAC, leaving the ciphertext vulnerable to bit-flipping and padding-oracle attacks. Use an AEAD mode (GCM, ChaCha20-Poly1305).

### Comparing Tokens With `==`

Leaking the secret byte-by-byte through timing because the equality operator short-circuits. Use the library's constant-time comparison.

### Reusing A Nonce In GCM Or CTR

Reusing a nonce with the same key in a stream-cipher mode, which is catastrophic and leaks plaintext and (for GCM) the authentication key. Generate or count nonces correctly; prefer misuse-resistant modes.

### Using A Non-Cryptographic RNG For Security Values

Generating session tokens or keys with `Math.random` or a default PRNG, producing predictable values that attackers can guess. Use a CSPRNG.

### Trusting Client-Side Encryption

Encrypting data in the browser and treating it as authenticated or trusted on the server. Client-side encryption can protect data in transit to a third party, but it is not authentication — the client controls the key and the algorithm, and a malicious client can send anything.

### Hardcoding Or Committing Keys

Embedding API keys, encryption keys, or passwords in source code, config files committed to version control, or container images. Use a secrets manager; see the secret-management skill.

### Rolling Custom Crypto Because "Standard Libraries Are Hard"

Implementing RSA by hand, building a custom MAC, or designing a token format because the vetted library's API felt verbose. The verbosity exists to prevent the mistakes you are about to make.

## Self-Check

- [ ] No custom cryptographic protocol or primitive combination is used; a vetted high-level library, AEAD mode, or established protocol (TLS, JWT with allow-listed algorithms, libsodium/age, a standard KDF) is used instead, and any direct use of a primitive is justified.
- [ ] The cryptographic goal (confidentiality, integrity/authenticity, password storage, token generation, key derivation, commitment) is explicitly identified, and the primitive matches that goal — hashes are not used for authenticity without a key, fast hashes are not used for passwords, and a MAC or signature is used where tampering must be detected.
- [ ] Passwords are stored only with a slow, salted, memory-hard KDF (Argon2id preferred), never with a fast hash (SHA-256, MD5) or unsalted.
- [ ] Nonces and IVs are never reused with the same key in stream or block cipher modes; misuse-resistant modes (XChaCha20-Poly1305, AES-GCM-SIV) are preferred where accidental reuse is plausible.
- [ ] All secret comparisons (tokens, MACs, password hashes, signatures) use a constant-time comparison function from the crypto library, never a short-circuiting `==`.
- [ ] All security-sensitive random values (keys, tokens, nonces, session ids, salts) are generated with a CSPRNG with sufficient entropy (at least 128 bits for unguessable tokens), never a non-cryptographic PRNG or a manually seeded generator.
- [ ] Key sizes and algorithms are chosen for the data's sensitivity lifetime (symmetric ≥128/256 bits; SHA-256/SHA-3 or larger; Ed25519 or RSA≥3072; SHA-1 and MD5 avoided for security), and key rotation is designed in from the start.
- [ ] No keys or secrets are hardcoded in source, committed to version control, or baked into images; secrets are managed via a secrets manager or KMS.
