---
name: signing_and_verification.md
description: Use when the agent is designing or reviewing message authentication, integrity protection, or authenticity for tokens, webhooks, API requests, software artifacts, events, or stored data; choosing between a MAC (HMAC) and an asymmetric digital signature; deciding what to bind into a signed payload; verifying signatures safely (order of operations, what to do on failure); defending against replay, tampering, key-substitution, and timing attacks; selecting signing algorithms (Ed25519, ECDSA, RSA-PSS, HMAC); managing signing and verification keys; or implementing webhook or JWT signature checks. Also covers canonicalization, algorithm confusion, nonce and timestamp binding, constant-time comparison, and the rule against verifying with attacker-influenced algorithms or keys.
---

# Signing And Verification

A signature answers two questions that encryption cannot: did this data come from who I think it did, and has it been changed since? Signing is how a system trusts a message without trusting the channel it arrived on — a webhook from a payment provider, a token issued by your own auth server, a software update, an event from a partner. Verification is the moment that trust is granted or refused. The recurring failure is not weak algorithms; it is verification that does not actually verify: a signature checked against the wrong key, an algorithm chosen by the attacker, a check that passes the happy path but accepts a forged or replayed message in the cases no one tested, or a comparison that leaks the expected value through timing.

Agents tend to under-invest here because "verify the signature" sounds like a single step, and the library call returns true or false. The defects live in the decisions around that call: which key, which algorithm, what is bound into the payload, what happens on failure, and what an attacker who controls the message can influence about the verification itself. A signature scheme that is mathematically sound can be trivially bypassed if verification accepts an attacker-chosen algorithm, looks up the key from the message, or treats a missing signature as valid. The judgment problem is treating verification as an adversarial operation — the input is supplied by someone who may be trying to forge it — rather than as a formality.

This skill is about the integrity and authenticity of messages, distinct from the encryption skill (which covers confidentiality) and the session skill (which covers authenticated session state). Here the question is: when a message arrives, how do you prove it is genuine and unmodified, and how do you refuse everything else.

## Core Rules

### Choose MAC Versus Signature By Who Must Verify

The first decision is whether the verifying party shares a secret with the signer. This determines the primitive:

- **MAC (Message Authentication Code, e.g., HMAC).** Symmetric: signer and verifier share a secret key. Fast, simple, and correct when the same party (or mutually trusting parties) both signs and verifies. The limitation: anyone who can verify can also forge, because verification and signing use the same key. Use HMAC for webhooks you send to yourself, internal service-to-service integrity, and sealed tokens issued and verified by the same authority.
- **Digital signature (asymmetric).** Signer holds a private key; anyone with the public key can verify, but only the private-key holder can sign. Slower, but solves problems HMAC cannot: non-repudiation (the verifier cannot forge), and verification by parties who must not be able to sign (a public API verifying your signed updates; many verifiers, one signer). Use signatures when the verifier is not the signer, when many parties verify, or when non-repudiation matters.

The common error is using a MAC where verifiers must not be able to forge (giving every service the shared signing key), or using asymmetric signatures where a symmetric MAC would suffice (adding key-distribution complexity for no benefit). Decide by the relationship: same trust domain and same key on both sides → MAC; different trust domains, or many verifiers → signature.

### Bind Everything That Must Not Be Substituted Into The Signed Payload

A signature protects exactly the bytes it covers. Anything not covered can be changed freely without invalidating the signature. The recurring bug is signing the payload but not the context, so an attacker takes a valid signed message and reuses it in a different context where it was not intended:

- **Bind the algorithm and key id.** Otherwise an attacker may substitute a different algorithm or key during verification (see algorithm confusion below).
- **Bind the intended audience and purpose.** A token signed for service A should not be accepted by service B; bind the audience, issuer, and purpose into the payload and check them on verification.
- **Bind the validity window.** A signed message without an expiry or issued-at timestamp is replayable forever. Bind `iat` (issued at) and `exp` (expiry), and check them.
- **Bind a nonce or request id where replay matters.** For a payment intent or a state-changing webhook, a unique nonce (stored and checked, or derived from the request) prevents the same valid message from being accepted twice.
- **Bind the recipient where relevant.** A key-exchange or session message should bind the parties so it cannot be replayed to a different one.

Enumerate what an attacker could change or reuse if it were not signed, and bind exactly that. A signature over the data but not the context protects the data and leaves the context unprotected.

### Verify Before You Trust, And Fail Closed

Verification must happen before any side effect or trust decision, and failure must refuse the message, not degrade to accepting it. The discipline:

- **Verify first, act second.** Do not process the payload, persist it, trigger work, or return its contents before the signature is confirmed. An unsigned or forged message should produce no effect beyond the refusal.
- **Fail closed on every failure mode.** Missing signature, malformed signature, wrong key, expired token, unknown algorithm, replayed nonce — each must refuse, not default to accepted. A verification routine with a code path that returns "valid" when an exception occurs is a bypass.
- **Treat "no signature" as "invalid," not as "no verification needed."** Endpoints that accept signed messages must require the signature; an unsigned request must not silently bypass the check.

Audit the verification path for any branch that grants trust without confirming the signature. The bypass is rarely the cryptographic check; it is the error handling around it.

### Do Not Let The Attacker Choose The Algorithm Or The Key

The most dangerous class of signature-verification bug is letting the untrusted message influence how it is verified. Two well-known variants:

- **Algorithm confusion (key substitution).** The message declares its algorithm (e.g., `alg: none`, or `RS256` vs `HS256`). If verification trusts the declared algorithm, an attacker can force a weak or wrong mode — the classic JWT `alg: none` bypass, or forcing RSA-public-key verification to be interpreted as an HMAC key, forging a valid signature with the public key. The defense: pin the expected algorithm(s) server-side and reject anything else; never accept `none`; never let the message select between asymmetric and symmetric modes.
- **Key lookup from the message.** If the verifier looks up the signing key based on a `kid` (key id) or issuer field supplied by the message, an attacker may point verification at a key they control (including a public key exposed in JWKS, used as an HMAC secret). Bind the key to a trusted mapping; do not let the message name which key verifies it without confirming that key is the right one for this issuer and context.

The principle: everything that determines how verification proceeds (algorithm, key, mode) must come from trusted configuration, not from the message being verified. The message may identify which trusted key applies, but the verifier must confirm that mapping, not trust it.

### Compare In Constant Time And Avoid Oracles

Signature and MAC comparison must not leak information about the expected value through timing, and the verification routine must not become an oracle that tells the attacker how close their forgery is:

- **Use constant-time comparison** for MAC tags and any direct equality check on a signature or token. A non-constant-time comparison exits early on the first mismatched byte, leaking the expected prefix one byte at a time. Libraries provide `constant_time_compare` / `CryptographicOperations.FixedTimeEquals`; use them.
- **Do not distinguish "invalid signature" from "valid signature, wrong context" in the response** where that distinction helps an attacker. A response that reveals whether the signature was right but the token expired, or right but the audience was wrong, can be used to iterate toward a valid forgery. Return a generic failure.
- **Beware verification oracles in error messages and logs.** Detailed verification-failure reasons in a response or a log readable to the attacker become a side channel.

Timing and oracle attacks are subtle because the function "works" — it returns the right answer — while leaking enough to eventually forge. Default to constant-time comparison and generic failures.

### Canonicalize Before Signing If The Bytes Can Vary

A signature is over bytes, not over meaning. If the same logical message can be represented by different bytes (JSON key ordering, whitespace, Unicode normalization, float formatting, number precision), then signing and verifying parties must agree on a canonical byte representation, or verification will fail on a valid message — or worse, an attacker will find a second representation that verifies under a different interpretation.

- **Canonicalize explicitly before signing and before verifying**, using the same rules. For JSON, this means stable key ordering, defined number handling, and defined Unicode normalization.
- **Do not re-serialize and sign the re-serialization** if the original bytes are what must be protected; sign the original bytes, or define and enforce the canonical form.
- **Beware canonicalization attacks** where an attacker constructs a payload that canonicalizes differently for the signer and the verifier (e.g., parser quirks, duplicate keys, comments). Use a single, well-defined canonicalization and reject inputs that do not round-trip.

For formats with a defined serialization (a fixed binary format, a canonical JSON scheme, the JOSE/JWT compact serialization), use it and do not improvise.

### Separate Keys By Purpose

A key used for signing should not be the key used for encryption, for a different service, or for a different algorithm. Key separation prevents a weakness or misuse in one context from compromising another:

- **One key per purpose.** Distinct keys for signing tokens, signing webhooks, encrypting data, deriving sub-keys. If one is compromised or misused, the others remain intact.
- **Derive separated keys from a master with a KDF** when you want the operational simplicity of one root secret: HKDF with a distinct info/context label per purpose produces independent keys.
- **Do not reuse an asymmetric key across algorithms.** An RSA key for signing must not be reused as an RSA encryption key; an Ed25519 key is for signing only.

A single key reused across purposes is a single point of failure and a frequent source of subtle cross-protocol attacks.

## Common Traps

### Letting The Message Choose The Algorithm (`alg: none`, RS/HK Confusion)

Accepting the algorithm declared in the signed message, so an attacker sets `alg: none` or switches between RSA and HMAC to forge a signature. Pin the expected algorithm server-side; reject unknown algorithms and `none`; never let the message select symmetric-vs-asymmetric mode.

### Verifying The Data But Not The Context

Signing the payload but not the audience, issuer, expiry, or purpose, so a valid signed message is replayed to a different service or accepted past its intended lifetime. Bind everything that must not be substituted, and check it on verification.

### Looking Up The Verification Key From The Message

Resolving the signing key from a `kid` or issuer field supplied by the attacker, who points verification at a key they control. The message may identify the key, but the verifier must confirm that mapping against a trusted source.

### Non-Constant-Time Comparison Of MAC Tags

Comparing a submitted tag to the expected tag with a normal equality operator, which exits early and leaks the expected tag byte by byte through timing. Always use constant-time comparison for cryptographic tags and signatures.

### Acting On The Payload Before Verifying

Processing, storing, or acting on the message contents before the signature check completes, so a forged or unsigned message has effects even though verification later fails. Verify first; act only after success.

### Replaying A Valid Signed Message

Accepting a correctly signed message more than once because there is no nonce, timestamp, or replay cache. For state-changing or payment messages, bind a unique nonce or a short validity window and enforce single-use.

### Signing Re-Serialized Data That Differs From What Was Verified

Signing a canonicalized form but verifying the raw form (or vice versa), or using different JSON parsers with different canonicalization, so valid messages fail verification or an attacker finds a divergent representation. Define one canonical form and use it on both sides.

### Reusing One Key Across Purposes

Using the same key to sign tokens, sign webhooks, and encrypt data, so a flaw or compromise in one use compromises all of them. Separate keys per purpose, or derive separated keys from a master with a KDF.

## Self-Check

- [ ] The primitive matches the trust model: HMAC where signer and verifier share a secret and the verifier need not forge; asymmetric signatures where verifiers must not be able to sign, many parties verify, or non-repudiation is required.
- [ ] The signed payload binds everything that must not be substituted or replayed — algorithm, key id, audience, issuer, purpose, issued-at, expiry, and a nonce where replay matters — and each is checked on verification.
- [ ] Verification happens before any trust decision or side effect, and every failure mode (missing, malformed, wrong key, expired, unknown algorithm, replayed nonce) fails closed with no path that grants trust without a confirmed signature.
- [ ] The algorithm and verification key are determined by trusted configuration, not by the message; the verifier rejects `none`, unknown algorithms, and any message-driven switch between symmetric and asymmetric modes; key lookups from the message are confirmed against a trusted mapping.
- [ ] MAC tags and signatures are compared in constant time, and verification failures return generic errors that do not reveal how close a forgery was.
- [ ] A single canonical byte representation is defined and used for both signing and verification; inputs that do not round-trip are rejected.
- [ ] Keys are separated by purpose (distinct keys, or KDF-derived with distinct context labels) and not reused across algorithms or services.
