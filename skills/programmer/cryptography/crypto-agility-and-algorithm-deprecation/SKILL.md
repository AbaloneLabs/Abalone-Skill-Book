---
name: crypto_agility_and_algorithm_deprecation.md
description: Use when the agent is designing cryptographic agility, planning the deprecation or migration of a broken algorithm (MD5, SHA-1, RC4, weak DH, legacy RSA padding, deprecated curves), swapping algorithms without a flag-day cutover, adopting hybrid classical-plus-post-quantum schemes during transition, hardcoding algorithm identifiers, versioning crypto formats, or reviewing a system for algorithms that can no longer be replaced. Also covers algorithm negotiation, pinning allowed algorithms server-side, tracking algorithm lifetimes and end-of-life dates, and the trap of baking one algorithm so deeply into a format or protocol that a future break is unfixable without redesign.
---

# Crypto Agility And Algorithm Deprecation

Cryptographic algorithms have lifetimes. An algorithm that was the right choice when it was chosen can become the wrong choice years later, because attacks improve, hardware changes, and mathematical breakthroughs happen — and because a sufficiently large adversary (or a future quantum computer) may break assumptions that held at design time. Cryptographic agility is the property that lets a system replace an algorithm when that day comes, and it is almost always undervalued until it is needed. The recurring failure is not picking a weak algorithm today; it is designing a system where the algorithm is impossible to replace tomorrow — hardcoded into a binary format, burned into a signed token's structure, or negotiated in a way that lets an attacker force the weakest option. When the break arrives, the system cannot be fixed without a redesign, a flag day, or breaking every existing client.

Agents tend to under-invest here because the current algorithm works and "we can always change it later" feels true. The defects are invisible until deprecation is forced: a token format that has no algorithm field, so old tokens cannot be distinguished from new; a database of hashes with no algorithm identifier, so a migration cannot tell which records need re-hashing; a protocol that lets the peer choose any algorithm, so an attacker downgrades to a broken one; or a binary format where the hash length is fixed at 20 bytes (SHA-1), so a stronger hash cannot fit. The judgment problem is designing for change: making the algorithm a named, versioned, replaceable component rather than a fixed property of the system, and deprecating weak algorithms deliberately rather than leaving them enabled "for compatibility."

This skill is about cryptographic agility and the lifecycle of algorithms — how to design systems so algorithms can be swapped, how to migrate without a flag day, and how to deprecate broken constructions. It complements the encryption skill (which covers choosing the right primitive) and the signing skill (which covers algorithm confusion in verification). Here the question is how the choice of algorithm changes over the system's lifetime without becoming a trap.

## Core Rules

### Make The Algorithm A Named, Versioned, Replaceable Component

The foundation of agility is that the algorithm is not implicit. Every cryptographic output — ciphertext, hash, signature, token, key — must record which algorithm and parameters produced it, so a future reader can process it correctly and a future migration knows what to change.

- **Embed an algorithm identifier (and version) in every crypto artifact.** A password hash record, a JWT header, a KDF output, an encrypted blob, a signed manifest should all carry an explicit `alg`/`version`/`method` field. The modular crypt format (`$argon2id$v=19$...`) exists precisely for this reason.
- **Never fix cryptographic lengths in a format.** A format that hardcodes a 20-byte hash (SHA-1) or a 16-byte tag cannot accommodate a stronger algorithm without a format break. Use length-prefixed or variable-length fields for hashes, tags, signatures, and keys.
- **Version the overall crypto format, not just the algorithm.** When the format itself must change (new fields, new encoding), a format version lets old and new coexist. A format with no version number cannot evolve safely.
- **Keep a registry of supported algorithms with their status** (current, deprecated, disabled), so the set of acceptable algorithms is explicit and reviewable rather than scattered across code.

The test of agility: if the strongest algorithm in use today were broken tonight, could the system move to a different one by changing configuration, or does it require changing a data format, a protocol, or every stored record? If the latter, the system is not agile.

### Deprecate Broken Algorithms Deliberately, On A Schedule

Algorithms do not fail all at once; they erode. MD5 became collision-vulnerable for some uses long before it was catastrophic for others; SHA-1 moved from "avoid for signatures" to "actively broken." Deprecation must be a managed process with a timeline, not a panic reaction.

- **Track the status of every algorithm you use against current guidance** (NIST, IETF, BSI) and recorded end-of-life dates. An algorithm past its deprecation date that is still accepted is a latent vulnerability.
- **Remove broken algorithms entirely, do not merely rank them low.** RC4, MD5 for signatures, SHA-1 for signatures, static RSA key exchange, weak DH groups, CBC without mitigations — leaving these "at the bottom of the list" allows a forced downgrade. The only safe position is removed.
- **Plan a deprecation timeline: announce, restrict, disable, remove.** Give consumers notice, then gate the algorithm behind explicit opt-in, then disable it by default, then delete the code. Each stage has a date.
- **Distinguish algorithm weaknesses by use.** MD5 is broken for collision resistance (signatures, certificates) but not for HMAC in some contexts; SHA-1 is broken for signatures but HMAC-SHA1 is less immediately threatened. Deprecate per use case based on the property that use depends on, but trend toward removing the primitive entirely to avoid misuse.

A list of algorithms that "used to be fine" and are still enabled is technical debt with a security cost. Audit it against current guidance on a schedule, not when an incident forces it.

### Migrate Without A Flag Day: Support Old And New Simultaneously

A flag day — a single moment when the system switches algorithms and everything must change at once — is operationally catastrophic and usually impossible for systems with external clients, stored data, or asynchronous participants. Migrations must overlap.

- **Make the new algorithm the default for new output while the old remains readable/verifiable.** New hashes use the new function; new signatures use the new algorithm; but old artifacts are still processed under their recorded algorithm. This requires the algorithm identifier from the first rule.
- **Migrate stored data lazily where possible.** Re-hash passwords on next login, re-sign manifests on next publish, re-encrypt on next write — so the data moves to the new algorithm over time as it is touched, without a bulk cutover.
- **Migrate stored data actively where the old algorithm is broken.** If the old algorithm is compromised (not merely aging), lazy migration is not enough; the data must be re-processed under the new algorithm on a schedule, with verification.
- **For protocols with negotiation, roll out the new algorithm to both ends before requiring it.** Clients and servers must both support the new algorithm before it can be made mandatory; otherwise the migration breaks connectivity.

The pattern is the same as key rotation: introduce the new, default to it for new work, retain the old for compatibility, and retire the old only when nothing depends on it. The difference is that algorithm migration often touches data formats and external clients, making the overlap window longer and the planning more deliberate.

### Use Hybrid Schemes During High-Risk Transitions

When the transition is driven by a threat that may materialize before confidence in the new algorithm is complete — most notably the transition to post-quantum cryptography — a hybrid approach combines a classical and a new algorithm so that the system is secure if either holds.

- **Hybrid encryption/signing combines classical and post-quantum algorithms**, so a break of the classical algorithm (by a quantum computer) or an unforeseen weakness in the new post-quantum algorithm does not compromise the system alone. Both must fail for security to break.
- **Design hybrids so the two algorithms are independent**, not sharing keys, randomness, or parsing that would let a flaw in one compromise the other. The classical and post-quantum keys must be generated and used separately.
- **Plan the hybrid as a phase, not a destination, when appropriate.** Hybrid is the bridge during the period when the new algorithm is not yet fully trusted; once confidence is high, the classical component may be dropped — but only after the migration is complete and the classical algorithm's continued presence is the liability.
- **Track standardization status.** Post-quantum algorithms are still being standardized and profiled (NIST PQC); deploying a not-yet-finalized algorithm in production carries risk. Prefer standardized, reviewed constructions and follow the standards bodies' guidance on parameters.

Hybrid schemes are more complex and slower, so they are justified by transition risk, not as a permanent default unless the threat model demands defense against both classical and quantum adversaries indefinitely.

### Pin Allowed Algorithms Server-Side; Do Not Let The Peer Choose Freely

Algorithm negotiation (as in TLS, JWT, JOSE) is powerful for agility but dangerous if the peer can force a weak choice. The agility must be controlled by the trusted party, not by the message or the attacker.

- **Maintain an explicit allowlist of acceptable algorithms** in trusted configuration, and reject everything else. An unknown or deprecated algorithm offered by a peer must be refused, not negotiated down to.
- **Reject `none` and disabled algorithms unconditionally.** A token with `alg: none`, or a connection offering a removed cipher, must fail, not fall back.
- **Do not let a message select between fundamentally different modes** (symmetric vs asymmetric) based on a field it controls — this is the classic JWT algorithm-confusion attack. The verifier's expected algorithm is fixed by context.
- **Negotiate down to the strongest mutually supported algorithm, with a hard floor.** The floor (minimum acceptable strength) is non-negotiable; if the peer cannot meet it, the interaction fails rather than downgrading.

Agility means the trusted operator can change the algorithm set over time; it does not mean the untrusted peer can choose any algorithm at will. Those are opposite properties, and confusing them creates downgrade vulnerabilities.

## Common Traps

### Hardcoding One Algorithm Into A Format Or Protocol

Baking SHA-1, a fixed hash length, or an implicit algorithm into a binary format, token structure, or wire protocol so that replacing it requires a format break or redesign. Make the algorithm a named, versioned field with variable-length outputs from the start.

### Leaving Broken Algorithms Enabled "For Compatibility"

Keeping RC4, MD5, SHA-1, static RSA, or weak DH in the allowed list because an old client might need them, which allows a forced downgrade against modern clients. Remove broken algorithms entirely; provide a hardened upgrade path instead.

### No Algorithm Identifier On Stored Crypto Artifacts

A database of hashes or ciphertexts with no `alg`/`version` column, so a migration cannot tell which records use the old algorithm and must re-process everything blindly or cannot migrate at all. Record the algorithm with every artifact.

### Flag-Day Cutover That Breaks Clients Or Data

Switching the algorithm for all output at a single moment without an overlap window, so clients that have not upgraded and data produced before the cutover become unprocessable. Migrate with overlap: new default, old retained, old retired only when unused.

### Negotiation That Lets The Attacker Force The Weakest Algorithm

Accepting the algorithm declared by the peer or message without a server-side allowlist and hard floor, enabling downgrade attacks. Pin allowed algorithms in trusted config and reject anything below the floor.

### Treating Post-Quantum Migration As A Future Problem With No Preparation

Assuming quantum resistance can be added later, while the system's format, key sizes, and protocol have no room for the larger post-quantum keys and signatures. Plan format agility and hybrid support now; retrofitting is often infeasible.

### Confusing Agility (Operator-Controlled) With Peer-Controlled Choice

Believing that "supporting many algorithms" provides security, when in fact letting the untrusted peer pick the algorithm creates downgrade risk. Agility is the trusted operator's ability to change the set, not the peer's freedom to choose.

### Never Auditing The Algorithm List Against Current Guidance

A set of algorithms chosen years ago and never reviewed, so deprecated constructions remain accepted long after they were broken. Audit the algorithm registry against NIST/IETF guidance on a schedule with recorded end-of-life dates.

## Self-Check

- [ ] Every cryptographic artifact (hash, ciphertext, signature, token, key) carries an explicit algorithm identifier and version, and formats use variable-length fields so a stronger algorithm fits without a format break.
- [ ] The overall crypto format is versioned, and a registry of supported algorithms with status (current/deprecated/disabled) and end-of-life dates is maintained and audited against current guidance on a schedule.
- [ ] Broken algorithms (RC4, MD5/SHA-1 for signatures, static RSA, weak DH) are removed entirely rather than ranked low, and deprecation follows an announce-restrict-disable-remove timeline.
- [ ] Migrations use an overlap window: the new algorithm is the default for new output while the old remains processable, stored data migrates lazily (re-hash/re-sign on touch) or actively when the old algorithm is broken, and there is no flag-day cutover.
- [ ] High-risk transitions (post-quantum) use hybrid schemes where the classical and new algorithms are independent, and the system's formats and protocols have room for larger post-quantum keys and signatures.
- [ ] Allowed algorithms are pinned in trusted server-side configuration with a hard minimum strength floor; the peer or message cannot force a weak, unknown, or `none` algorithm, and cannot switch between symmetric and asymmetric modes.
- [ ] No algorithm is hardcoded so deeply (fixed length, implicit choice, burned into a protocol) that replacing it would require a redesign; the system could move to a different algorithm by changing configuration and data, not by changing formats.
- [ ] The algorithm allowlist was reviewed against current standards guidance (NIST/IETF) recently, and any algorithm past its deprecation date is on a removal timeline rather than still accepted.
