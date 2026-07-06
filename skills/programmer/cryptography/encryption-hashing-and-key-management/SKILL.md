---
name: encryption_hashing_and_key_management.md
description: Use when the agent is protecting sensitive data such as passwords, tokens, personal data, payment data, health records, or secrets; choosing between encryption and hashing; selecting symmetric versus asymmetric encryption; hashing or verifying passwords; adding salts or peppers; generating, storing, rotating, or retiring encryption keys; deciding where keys live (KMS, HSM, environment variables, secrets manager); encrypting data at rest or in transit; choosing authenticated encryption (AEAD) such as AES-GCM or ChaCha20-Poly1305; signing or verifying messages; or reviewing whether a cryptographic choice is correct. Also covers the distinction between confidentiality and integrity, why passwords must be hashed not encrypted, what encryption does and does not guarantee, and the rule against inventing custom crypto.
---

# Encryption, Hashing, And Key Management

Cryptography is a set of tools that each solve a different problem, and most security mistakes come from reaching for the wrong tool or expecting it to guarantee something it does not. Encryption keeps data confidential; hashing proves a value matches without storing it; signing proves who produced a message; key management keeps the secret that makes any of this work out of the hands of an attacker. These are not interchangeable. Encrypting a password stores it reversibly, which is exactly wrong; hashing data you later need to read back makes it permanently unreadable; signing without verifying provides no protection at all. The recurring root cause is not weak algorithms — it is mismatched intent: the developer picked a mechanism that does not solve the threat they actually face.

Agents tend to under-invest here because "encrypt it" feels like a complete answer and the code compiles either way. The harm is severe and often invisible until breach: passwords stored reversibly get exfiltrated wholesale, data encrypted without integrity gets tampered with undetected, keys left in source code or environment variables turn a single config leak into total compromise, and hand-rolled schemes that "look cryptographic" fall to attacks the author never imagined. The judgment problem is deciding, for each piece of sensitive data, what threat you are protecting against, which mechanism solves that threat, and how the key that protects it is generated, stored, and rotated over its whole life.

This skill is about selecting and applying cryptography correctly. It is deliberately conservative: when uncertain, prefer well-reviewed libraries and standard constructions over invention, and prefer asking a security reviewer over confident improvisation.

## Core Rules

### Decide The Threat First, Then The Mechanism

Before choosing any algorithm, name the threat you are protecting against. Different threats demand different tools, and picking the tool before the threat is how data ends up "protected" in a way that solves the wrong problem.

- **Confidentiality at rest** (an attacker who reads the disk or database should not read the data) → encryption with a key they do not have.
- **Confidentiality in transit** (an attacker on the network should not read or tamper) → TLS, which is encryption plus integrity plus authentication.
- **Password storage** (an attacker who steals the user database should not recover passwords) → password hashing with a slow, salted function.
- **Integrity / tamper detection** (an attacker should not modify data undetected) → a MAC or signature.
- **Authenticity / non-repudiation** (proof of who produced data) → asymmetric signatures or a MAC shared between known parties.
- **Verifying a value without storing it** (confirm a token or file matches) → a hash, keyed where forgery matters.

Write the threat down. "Encrypt the database column" is a mechanism, not a threat model; the real question is who you are keeping it secret from, and whether you also need to detect tampering or prove origin.

### Never Hash What You Must Read Back, Never Encrypt What You Must Never Recover

The most common classification error is confusing hashing with encryption. They have opposite reversibility, and the choice is determined by whether you ever need the original value back.

- **Hashing is one-way.** You cannot recover the input from the hash. Use it when you only need to verify: password storage (you verify a submitted password matches, you never need to read it back), integrity checks, content addressing. If you hash data you later need to display, process, or decrypt, it is gone.
- **Encryption is two-way (with the key).** You can recover the plaintext. Use it when you must read the data back later: encrypted columns you query and decrypt, encrypted backups, encrypted tokens you issue and later parse. If you encrypt a password, you are storing it reversibly, which is a vulnerability.

The decisive question: "Do I ever need the original value back?" If no → hash. If yes → encrypt. Passwords are always no. Payment card numbers, health records, and confidential documents are usually yes.

### Hash Passwords With A Slow, Salted Function — Never Encrypt Or Fast-Hash Them

Password storage has a strict correct answer: use a purpose-built password hashing function (Argon2id, bcrypt, scrypt) with a per-password random salt and parameters tuned to be deliberately slow. Everything else is a mistake:

- **Never store plaintext or reversibly encrypted passwords.** Encryption means a key compromise recovers every password; password reuse then compromises users across other services.
- **Never use a fast hash (MD5, SHA-1, SHA-256 alone) for passwords.** Fast hashes are designed for speed; an attacker with the hash file can try billions of guesses per second. Password hashing must be slow to make brute force expensive.
- **Always use a per-password random salt.** The salt prevents precomputation (rainbow tables) and ensures identical passwords produce different hashes. The salt is not secret; it is stored alongside the hash.
- **Tune the cost.** Pick parameters (memory, iterations, parallelism) so a single verification takes a deliberate fraction of a second on your hardware, and raise them as hardware improves.

A pepper (a server-side secret combined with the password before hashing) can add defense if the database is stolen but the application secret is not, but it is not a substitute for a slow salted hash, and it introduces key-management obligations of its own.

### Prefer Authenticated Encryption (AEAD); Encryption Without Integrity Is Incomplete

Confidentiality without integrity is a trap: an attacker who cannot read the ciphertext may still be able to modify it in ways that produce predictable changes in the decrypted plaintext (bit-flipping), or substitute one ciphertext for another. Encryption alone does not guarantee the data was not tampered with.

Use authenticated encryption, which provides both confidentiality and integrity in one construction:

- **AEAD modes** such as AES-GCM or ChaCha20-Poly1305 produce a ciphertext plus an authentication tag; decryption verifies the tag and fails if the data was modified.
- **Never use a raw block cipher mode without authentication** (ECB, or CBC/CTR without a separate MAC) for anything an attacker can influence. ECB additionally reveals patterns and must never be used.
- **Include all context that must be authenticated** in the associated data (AAD) or in the signed/protected payload: algorithm, version, key id, message type. Otherwise an attacker may replay a valid ciphertext in a context it was not intended for.

The rule: if you need confidentiality and you cannot tolerate undetected tampering — which is almost always — you need authenticated encryption, not encryption alone.

### Choose Symmetric Versus Asymmetric By Who Needs To Do What

Symmetric and asymmetric cryptography solve different coordination problems. Choose by the relationship between the parties.

- **Symmetric (one shared key).** Fast, simple, best when the encryptor and decryptor are the same party or already share a secret. The default for encrypting your own data at rest: one key, held by you, encrypts and decrypts. The hard part is key distribution — both sides need the key, securely.
- **Asymmetric (public/private key pair).** Slower, but solves problems symmetric cannot: anyone can encrypt to or verify a public key, only the holder of the private key can decrypt or sign. Use it for signatures (prove origin), key exchange (establish a shared symmetric key over an insecure channel, as TLS does), and situations where parties have not pre-shared a secret.

Most practical systems are hybrid: asymmetric cryptography establishes or wraps a symmetric key, and symmetric cryptography does the bulk encryption. Do not force asymmetric encryption onto large data directly — encrypt a symmetric key, or use a protocol (TLS, age, libsodium's sealed boxes) that already does this.

### Treat Key Management As The Real Problem, Not The Algorithm

A correct algorithm with a compromised key provides no security. The key is the secret; everything else is public and assumed known to the attacker. Key management — how keys are generated, stored, used, rotated, and destroyed — is usually harder and more important than algorithm selection.

For each key, answer across its whole lifecycle:

- **Generation.** Generated with a cryptographically secure random source (CSPRNG), with enough entropy, never derived from a password or predictable value unless using a proper key-derivation function.
- **Storage.** Where does the key live at rest? Options, from strongest to weakest: a hardware security module (HSM) or cloud KMS where the key never leaves the device; a secrets manager or KMS-backed envelope encryption; and — only for low-stakes or development — environment variables or config (the weakest, because they leak in process listings, container images, logs, and crash dumps).
- **Use.** Who and what can read the key? Apply least privilege: only the service that needs it, scoped narrowly, audited.
- **Rotation.** How and how often is the key replaced? Rotation limits the blast radius of a single key compromise and is often a compliance requirement.
- **Retirement.** How is an old key destroyed so it cannot be recovered? Encrypted data outliving its key must either be re-encrypted under the new key or accepted as unrecoverable.

Prefer envelope encryption for application data: a master key lives in KMS/HSM and never touches the application; the application generates a per-object data key, encrypts the data with it, and asks KMS to encrypt (wrap) the data key. The master key is centralized and rotatable; data keys are disposable.

### Never Invent Cryptography; Use Reviewed Libraries And Standard Constructions

Cryptographic correctness is unforgiving in a way most programming is not. A scheme that looks secure can be trivially broken by a property the author did not consider: nonce reuse in GCM exposes the key, predictable IVs in CBC enable plaintext recovery, homebrew MAC constructions fail to specific forgeries, custom random generators are biased. These failures are not caught by normal testing, because the output still "looks encrypted."

- **Use established, audited libraries** (libsodium, Tink, OpenSSL/BoringSSL via high-level APIs, language-standard crypto modules) rather than calling low-level primitives directly.
- **Use high-level constructions** ("secretbox", "sealed box", AEAD, password hashing function) rather than assembling primitives yourself. The library's high-level API is far less likely to be misused.
- **Never invent a new algorithm, mode, or protocol** for production. "I think my scheme is secure" is not evidence; decades of broken custom crypto by skilled people is.
- **Never reuse a key, nonce, or IV across different purposes or messages** where the construction forbids it. Nonce reuse in AEAD is catastrophic.

If the requirement seems to demand something non-standard, that is a signal to consult a security reviewer or cryptography specialist, not to improvise.

### Understand What Cryptography Does Not Guarantee

Cryptography solves specific problems and is silent on others. A system can be "fully encrypted" and still insecure, because encryption does not address threats outside its scope:

- **Encryption does not provide integrity or authenticity unless authenticated encryption or a MAC/signature is used.** Confidential data can still be tampered with.
- **Encryption does not prove identity.** Knowing the key lets you decrypt; it does not prove who encrypted it. Authenticity needs signatures or MACs.
- **Encryption does not protect against the holder of the key.** If the application server holds the key and is compromised, the attacker reads the data. Encryption at rest protects against disk theft, not against application compromise.
- **Hashing does not make data confidential if the input space is small.** A hash of a yes/no field, a small enum, or a short identifier can be brute-forced by trying all inputs. Use a keyed hash (HMAC) where the attacker should not be able to test guesses.
- **TLS protects data in transit, not at rest.** Data decrypted and stored unencrypted on either end is unprotected at rest.

Name what you need (confidentiality, integrity, authenticity, non-repudiation) and confirm the chosen mechanism actually provides each one.

## Common Traps

### Encrypting Passwords

Storing passwords encrypted (reversibly) so they can be "recovered," or using a fast hash like SHA-256. Passwords must be hashed with a slow, salted password-hashing function. Reversible storage means any key compromise exposes every user's password, and password reuse makes that a cross-service breach.

### Hashing Data You Need To Read Back

Hashing an email or card number to "secure" it, then discovering you can never display, query, or process the original again. If you need the value back, encrypt it; if you only need to match it, hash it. Decide by reversibility, not by which sounds more secure.

### Using Encryption Without Authentication

Encrypting with AES-CBC or CTR and no MAC, or with ECB. The ciphertext is confidential but not integrity-protected: an attacker can modify it and the decryption succeeds with altered, possibly attacker-controlled plaintext. Use an AEAD mode (AES-GCM, ChaCha20-Poly1305) so tampering is detected.

### Reusing A Nonce, IV, Or Key Across Messages

Reusing a nonce/IV in GCM or CTR exposes the key or lets an attacker recover plaintext by XORing ciphertexts. Nonce uniqueness per key is a hard requirement, not a recommendation. Generate nonces correctly (random for GCM within its limit, or a monotonic counter) and never reuse a key across algorithms or purposes.

### Storing Keys In Code, Config, Or Environment Variables For Real Systems

Hardcoding keys, committing them to source, or relying on environment variables for production secrets. These leak through version control, container images, process listings, logs, and crash dumps. Use a KMS, HSM, or secrets manager; keep master keys out of the application entirely via envelope encryption.

### Never Rotating Keys

Generating one key and using it forever. A single compromise then exposes all data ever encrypted under it, and rotation may be a compliance requirement. Plan rotation: version keys, re-encrypt data under new keys, and retire old keys on a schedule.

### Inventing A Custom Scheme Or Random Generator and confusing TLS With At-Rest Protection

Combining primitives by hand, designing a "clever" protocol, or using a general-purpose RNG for keys/nonces. Custom crypto fails in ways testing cannot catch, and biased or predictable randomness silently destroys security. Use audited libraries and CSPRNGs only.

Assuming that because traffic uses HTTPS, stored data is protected. TLS protects data in transit between endpoints; once decrypted and stored, it is unprotected at rest unless separately encrypted. These are independent layers.

### Assuming "Encrypted" Means "Tamper-Proof" and brute-Forceable Hashes Of Low-Entropy Values

Believing that because data is encrypted, an attacker cannot alter it. Without authenticated encryption or a MAC, ciphertext can be modified to produce predictable plaintext changes. Confidentiality and integrity are separate properties; confirm you have both where tampering matters.

Hashing a boolean, a small status enum, or a short ID and treating the hash as confidential. Because the input space is tiny, an attacker hashes every possible input and matches. Where the attacker can test guesses, use a keyed hash (HMAC) or add a secret pepper so they cannot run the comparison.

## Self-Check

- [ ] The threat was named before the mechanism: for each protected value, it is clear whether the goal is confidentiality at rest, confidentiality in transit, password storage, integrity, authenticity, or verification — and the chosen tool solves that specific threat.
- [ ] Passwords are hashed with a slow, salted password-hashing function (Argon2id/bcrypt/scrypt) with tuned cost; none are stored plaintext, reversibly encrypted, or fast-hashed.
- [ ] Data that must be read back is encrypted (not hashed); data that only needs matching is hashed (not encrypted) — the reversibility decision is explicit for each field.
- [ ] Encryption uses an authenticated mode (AEAD such as AES-GCM or ChaCha20-Poly1305); no raw ECB/CBC/CTR-without-MAC is used for attacker-influenced data, and associated data covers algorithm, version, and context.
- [ ] Symmetric versus asymmetric was chosen by the relationship between parties; bulk data uses symmetric encryption, with asymmetric reserved for signatures, key exchange, or where no shared secret exists.
- [ ] Every key has a defined lifecycle: generated by a CSPRNG, stored in KMS/HSM/secrets manager (not code/config/env for production), used under least privilege, rotated on a schedule, and retired securely.
- [ ] Envelope encryption is used for application data so the master key stays in KMS/HSM and never reaches the application; data keys are per-object and disposable.
- [ ] No custom algorithm, mode, protocol, or random generator is used; only audited libraries and high-level constructions are in play, and nonces/IVs are never reused per key.
- [ ] The guarantees cryptography actually provides are confirmed: integrity and authenticity are present where tampering or origin matters, and encryption is not mistaken for protection against key-holder compromise or for at-rest protection.
- [ ] Low-entropy hashed values (booleans, small enums, short IDs) use a keyed hash (HMAC) or pepper so attackers cannot brute-force the input space.
