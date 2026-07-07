---
name: random_and_nonce_generation.md
description: Use when the agent is generating cryptographic keys, nonces, IVs, initialization vectors, tokens, password-reset secrets, session identifiers, or salts; choosing between a CSPRNG and a general-purpose RNG; ensuring seed entropy at startup or in containers and VMs; enforcing nonce or IV uniqueness for AEAD modes such as AES-GCM and ChaCha20-Poly1305; deciding between counter-based and random nonces; or reviewing code for predictable tokens, reused nonces, Math.random() used for security, or low-entropy secrets. Also covers the catastrophic failure mode of nonce reuse in GCM, deterministic output from mis-seeded RNGs, and why weak randomness silently destroys otherwise-correct cryptography.
---

# Random And Nonce Generation

Randomness is the raw material of cryptography, and it is the assumption that fails silently. Every key, every nonce, every token, every password-reset secret depends on being unpredictable to an attacker, and the cryptographic construction around it is correct only if that unpredictability holds. Use AES-GCM with a perfect key, but reuse a nonce, and the key is recoverable. Hash a password with a perfect salt, but generate the salt with a predictable RNG, and the salt is worthless. The recurring failure is not a weak algorithm — it is entropy that was assumed but never actually present: a `Math.random()` call used because it was convenient, a CSPRNG seeded from a low-entropy source at boot, a nonce generated from a clock that an attacker can influence, or a token based on a timestamp that is guessable. The cryptography "works" — it produces output that looks random — while being predictable enough to break.

Agents tend to under-invest here because the output of a weak RNG looks identical to the output of a strong one, and the bug is invisible until an attacker exploits it. The defects are severe and often total: a single reused nonce in AES-GCM exposes the authentication key and the plaintext of both messages; a session token from a predictable generator lets an attacker forge any session; a CSPRNG seeded at container startup with insufficient entropy produces keys that are brute-forceable; a counter-based nonce that resets on restart reuses nonces and breaks the cipher. The judgment problem is treating randomness as a security-critical, adversarially-tested component — never a convenience — and treating nonce uniqueness as a hard invariant, not a best-effort property.

This skill is about generating random values for cryptographic use and maintaining nonce/IV uniqueness. It complements the encryption skill (which covers AEAD mode selection) and the key-management overview. Here the question is where your randomness comes from, whether it has enough entropy, and whether your nonces can ever repeat.

## Core Rules

### Use A CSPRNG For Everything Security-Critical; Never A General-Purpose RNG

The single most important rule: any value whose unpredictability protects security — keys, nonces, IVs, tokens, salts, reset secrets, session ids — must come from a cryptographically secure pseudo-random number generator (CSPRNG), not from a general-purpose RNG. The two are built for different goals and are not interchangeable.

- **A CSPRNG** (the OS entropy source: `/dev/urandom`, `getrandom()`, `CryptGenRandom`/`BCryptGenRandom`, or language APIs that wrap them: `secrets`, `crypto.randomBytes`, `SecureRandom`, `crypto/rand`) is designed so that output is computationally indistinguishable from random even to an attacker who observes part of the output.
- **A general-purpose RNG** (`Math.random()`, `rand()`, `random.random()`, `java.util.Random`, Mersenne Twister) is designed for speed and statistical distribution, not unpredictability. Its output is predictable if the internal state is known, and the state is often small and reconstructable from observed outputs. Using it for a key or token is equivalent to publishing the value.
- **The output of a weak RNG looks random.** Statistical tests pass; the values look uniform. The defect is predictability to an adversary, which no normal test catches. Never judge an RNG by whether its output "looks random."
- **Never seed a CSPRNG yourself** with a value you chose, and never roll your own RNG. Use the platform CSPRNG directly; it is seeded and re-seeded by the operating system from entropy sources you cannot easily replicate.

If a value protects security and it did not come from a CSPRNG, treat it as compromised and regenerate it. There is no "almost secure" RNG.

### Ensure Seed Entropy, Especially At Startup And In Constrained Environments

A CSPRNG is only as good as the entropy that seeded it. Most platforms handle this transparently, but several environments are notorious for low entropy at the moment security matters most: process startup, virtual machines, containers, and embedded devices.

- **At boot or process start, the entropy pool may be insufficient.** A CSPRNG queried before the OS has gathered enough entropy can produce predictable output. Modern `/dev/urandom` and `getrandom()` block until seeded on Linux (post-5.6 for `getrandom`), but older behavior and some APIs do not — confirm the behavior of your platform and library.
- **Virtual machines and containers often clone entropy state.** A VM cloned from a snapshot, or many containers started from the same image at once, may share or start with low entropy. Do not generate long-lived keys immediately at startup in such environments without ensuring the CSPRNG is seeded; prefer keys generated out-of-band or delayed until entropy is confirmed.
- **Embedded and IoT devices without good entropy sources** (no disk, no network, deterministic boot) are a recognized hard problem; they may need a hardware RNG or entropy injected during manufacturing. Do not assume `/dev/urandom` is sufficient on a device with no entropy sources.
- **Do not "help" the CSPRNG by mixing in your own entropy** unless you understand entropy estimation; a misguided attempt to add entropy can reduce it. Rely on the OS, and ensure the conditions for the OS to do its job (enough uptime, enough events) are met.

The failure mode is silent: keys generated under low entropy look fine and work fine, until an attacker who knows the limited entropy space brute-forces them. For high-value keys, generate them on a machine with confirmed entropy, not in a freshly started container.

### Treat Nonce And IV Uniqueness As A Hard Invariant

For stream-cipher and AEAD modes (AES-GCM, AES-CTR, ChaCha20-Poly1305), the nonce (or IV) must never repeat under the same key. This is not a recommendation; it is a mathematical invariant whose violation is catastrophic.

- **Nonce reuse in AES-GCM is catastrophic.** Reusing a (key, nonce) pair leaks the authentication key and allows forgery, and XORing the two ciphertexts recovers the plaintext of both. A single reuse can compromise all data under that key. This is the highest-stakes randomness rule in symmetric cryptography.
- **Nonce reuse in CTR mode** XORs the two keystreams, so XORing the ciphertexts cancels the keystream and reveals the XOR of the two plaintexts — often enough to recover both.
- **The requirement is uniqueness, not secrecy.** The nonce does not need to be secret; it can be transmitted in the clear. It must simply never repeat for a given key. But a predictable nonce is also dangerous in some modes (CBC with a predictable IV enables chosen-plaintext attacks), so prefer unpredictable nonces where the mode allows.
- **Uniqueness must hold across restarts, processes, and machines.** A counter that resets on restart, or random nonces generated independently on two nodes, can collide. The uniqueness scope is the key's lifetime, not the process's.

Design the nonce strategy before writing encryption code: decide whether nonces are counter-based or random, how uniqueness is guaranteed across restarts and distribution, and what happens if a collision is even suspected (the key must be treated as compromised and rotated).

### Choose Counter-Based Versus Random Nonces By The Constraint You Can Guarantee

There are two sound strategies for nonce generation, and the choice depends on which invariant you can reliably maintain.

- **Counter-based nonces** (a monotonically increasing counter persisted across restarts) guarantee uniqueness as long as the counter never resets and never exceeds the nonce width. They are the safest choice when you control the encryption path and can persist the counter. The risk: a counter that resets (restart without persistence) or wraps (counter too narrow for the data volume) causes reuse.
- **Random nonces** (a CSPRNG-generated value per message) avoid the need to persist state, but uniqueness is probabilistic. For AES-GCM's 96-bit nonce, the birthday bound limits you to roughly 2^32 messages under one key before a collision is likely — after which the key must be rotated. Random nonces are appropriate when message volume is bounded and below that limit, and when persisting a counter is impractical.
- **Match the nonce width to the strategy.** GCM's standard 96-bit nonce is designed for counter or random use within its limits; do not invent a different nonce length unless you understand the security proofs. Misusing the nonce field (e.g., extra ciphertext length bits) breaks GCM's security.
- **For distributed systems, partition the nonce space** (give each node a distinct prefix or counter range) so independent nodes cannot collide, or centralize nonce/key assignment. Two nodes generating random 96-bit nonces under the same key will eventually collide; plan for it.

Never mix strategies carelessly: a counter that sometimes falls back to random, or a random nonce that sometimes repeats under concurrency, is a latent catastrophic bug. Pick one strategy per key and enforce it.

### Make Tokens And Reset Secrets Unguessable And Single-Source

Security tokens — session ids, password-reset tokens, CSRF tokens, API keys, invitation codes — are credentials, and their security is entirely their unpredictability. A predictable token is a forgery.

- **Generate tokens with a CSPRNG at sufficient length** (typically 128 bits or more of entropy; a 32-byte random value base64url-encoded is a common choice). Length alone is not enough if the source is predictable; both CSPRNG source and adequate bit length matter.
- **Never derive tokens from predictable inputs** — timestamps, user ids, sequential counters, hashes of the time, or any value an attacker can guess or influence. A reset token that is `hash(timestamp + userid)` is guessable; an attacker who can estimate the time can forge it.
- **Bind tokens to their purpose and context** (account, expiry, single-use) and verify all of those, not just that the token matches. A valid token reused in a different context or after expiry must fail.
- **Avoid exposing the RNG's internal state.** Do not return raw RNG output that could let an attacker reconstruct future values; use the standard token-generation APIs that produce independent values per call.

The failure mode is a token that "looks random" but is derived from a small or guessable space, so an attacker brute-forces or predicts it. Treat every token as a bearer credential and generate it accordingly.

## Common Traps

### Using Math.random() Or A General-Purpose RNG For Security Values

Calling `Math.random()`, `rand()`, `java.util.Random`, or Mersenne Twister to generate a key, token, nonce, or salt. The output is predictable once the internal state is known, and the state is small and reconstructable. Use the platform CSPRNG (`secrets`, `crypto.randomBytes`, `SecureRandom`, `crypto/rand`) for anything security-critical.

### Reusing A Nonce In AES-GCM Or CTR

Encrypting two messages under the same key and nonce, which in GCM leaks the authentication key and allows forgery and plaintext recovery, and in CTR reveals the XOR of the two plaintexts. Nonce uniqueness per key is a hard invariant; design the nonce strategy and guarantee it across restarts and distribution.

### Counter-Based Nonce That Resets On Restart

Persisting no counter state, so a process restart resets the counter to zero and reuses nonces from the previous run. Persist the counter durably, or use random nonces within their collision limit, and treat any suspected reuse as a key compromise.

### CSPRNG Seeded With Low Entropy At Startup

Generating keys or tokens in a freshly started VM, container, or embedded device before the OS entropy pool is ready, producing predictable output. Confirm the CSPRNG is seeded before generating high-value keys; prefer generating them out-of-band on a machine with confirmed entropy.

### Random Nonces Used Past The Collision Limit

Generating random 96-bit GCM nonces indefinitely under one key, eventually hitting a collision (around 2^32 messages) that breaks the cipher. Rotate the key before the birthday bound, or use counter-based nonces if volume is high.

### Predictable Tokens Derived From Timestamps Or User Data

A password-reset token, session id, or API key derived from a timestamp, user id, sequential counter, or hash thereof, so an attacker who estimates the inputs can forge it. Generate tokens from a CSPRNG at adequate length and bind them to purpose, expiry, and single-use.

### Seeding Or Re-Seeding The CSPRNG Yourself

Mixing in your own "entropy" or seeding the CSPRNG with a chosen value, which can reduce entropy rather than add it. Rely on the OS entropy source directly and do not improvise entropy estimation.

### Assuming Random Output Means Secure Output

Judging an RNG by whether its values look random, which a weak RNG also produces. Predictability to an adversary is invisible to statistical inspection; only using a CSPRNG guarantees it.

## Self-Check

- [ ] Every security-critical value (keys, nonces, IVs, tokens, salts, reset secrets, session ids) is generated with a CSPRNG (`secrets`, `crypto.randomBytes`, `SecureRandom`, `crypto/rand`, OS `/dev/urandom`/`getrandom`), and no general-purpose RNG (`Math.random()`, `rand()`, `java.util.Random`, Mersenne Twister) is used for any of them.
- [ ] The CSPRNG is confirmed seeded before high-value keys are generated, especially at startup and in VMs/containers/embedded devices; long-lived keys are generated out-of-band or delayed until entropy is confirmed rather than at fresh container start.
- [ ] The nonce/IV strategy is designed before encryption code is written: counter-based (with a durably persisted counter that never resets or wraps) or random (within the collision limit, with key rotation before the birthday bound), and the choice is enforced consistently per key.
- [ ] Nonce uniqueness is guaranteed across restarts, processes, and distributed nodes (persisted counter, or partitioned nonce space, or bounded random use with rotation), and any suspected nonce reuse is treated as a key compromise requiring rotation.
- [ ] No nonce is reused under the same key in AES-GCM or CTR; the catastrophic consequence of a single GCM nonce reuse (key and plaintext exposure) is understood, and the code path makes reuse impossible rather than merely unlikely.
- [ ] Tokens, reset secrets, and session identifiers are generated from a CSPRNG at adequate entropy (128+ bits), never derived from timestamps, user ids, counters, or hashes of predictable inputs, and are bound to purpose, expiry, and single-use.
- [ ] The CSPRNG is not self-seeded or re-seeded with chosen values, and no custom RNG is implemented; the platform OS entropy source is used directly.
- [ ] No security decision rests on the assumption that "the output looks random"; weak RNG output is indistinguishable from strong by inspection, so the source (CSPRNG) is verified rather than the output's appearance.
