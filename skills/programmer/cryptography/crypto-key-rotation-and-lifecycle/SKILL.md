---
name: crypto_key_rotation_and_lifecycle.md
description: Use when the agent is designing cryptographic key lifecycle, planning key rotation schedules or emergency rotation on suspected compromise, versioning keys for backward-compatible decryption, migrating encrypted data between keys without breaking in-flight traffic, setting up dual-key or phased rotation, deciding key escrow or backup policy, retiring or destroying old keys, or reviewing a system for orphaned keys and unrecoverable data. Also covers time-based versus event-based rotation triggers, envelope-encryption re-wrapping, the blast radius of a single key, and the trap of rotation that silently makes historical ciphertexts undecryptable.
---

# Crypto Key Rotation And Lifecycle

A cryptographic key is a secret with a lifetime, and that lifetime is where most key-management failures actually happen. Selecting a strong algorithm and generating a good key is the easy part; the hard part is everything that follows: how long the key lives, how it is replaced, how old ciphertexts stay readable after the key changes, what happens the moment a key is suspected compromised, and how the key is finally destroyed without taking data with it. The recurring failure is not a weak key — it is a lifecycle designed for the happy path (one key, generated once, used forever) that becomes a catastrophe on the bad day: a single compromise exposes all data ever encrypted under that key, an emergency rotation breaks decryption of historical records, or a key is destroyed and the data it protected is lost forever.

Agents tend to under-invest here because "rotate the key" sounds like a single operation and the code works the moment after generation. The defects are invisible until rotation is attempted or a breach occurs: ciphertexts that carry no key identifier so the system cannot tell which key decrypted them, rotation that atomically swaps the active key and leaves in-flight messages undecryptable, old keys deleted to "clean up" that orphan the data they encrypted, or a master key with no escrow whose loss means every dependent data key is unrecoverable. The judgment problem is treating key rotation as a data-continuity problem first and a security problem second: a rotation that improves security but breaks decryption has traded one failure mode for another.

This skill is about the operational lifecycle of cryptographic keys — generation, versioning, rotation, backward compatibility, escrow, and retirement. It is distinct from the key-management overview in the encryption skill, which covers where keys live and how they are classified. Here the question is how a key changes over time without breaking the data it protects and without leaving secrets exposed longer than necessary.

## Core Rules

### Make Every Key Versioned And Addressable From Its Ciphertext

Rotation is impossible if the system cannot tell which key produced a given ciphertext. The first rule of any rotatable key is that every protected blob must carry, or be associated with, a stable key identifier and version, so decryption can locate the exact key that was used.

- **Store a key id (and version) alongside every ciphertext**, whether in a database column, a token header, an envelope, or metadata. AES-GCM does not embed a key id; you must add one at the layer above.
- **Never assume "the current key" decrypts historical data.** After rotation, old ciphertexts were produced under old keys; decryption must select the key by id, not by "the active one."
- **Version keys, not just rotate them.** A versioned keyring (active key plus a set of previous keys retained for decryption) is the foundation that makes both rotation and backward compatibility possible.
- **Bind the key id into authenticated data where it affects security.** For AEAD, include the key id in the associated data (AAD) so an attacker cannot substitute a ciphertext encrypted under one key and claim it was under another.

Without a key id on every ciphertext, the system is permanently locked to a single key, and any future rotation either breaks decryption or is impossible. Design the key identifier in from the start, even if you only ever have one key — retrofitting it onto existing ciphertexts is often infeasible.

### Separate Time-Based Rotation From Event-Based (Emergency) Rotation

Rotation has two distinct triggers, and conflating them produces both over-rotation (churn and operational risk for no benefit) and under-rotation (a compromised key kept in service because "the schedule says rotate next quarter"). Decide each deliberately.

- **Time-based (scheduled) rotation** limits the blast radius of an undetected compromise and satisfies compliance requirements. Choose intervals by sensitivity: short-lived data keys (per-object, per-session), longer-lived application keys (weeks to months), and long-lived master keys (rotated rarely because rotation is expensive and risky). The interval should reflect how much data a single key compromise would expose.
- **Event-based (emergency) rotation** is triggered by suspected or confirmed key compromise: a leaked secret, a breached host that held the key, an insider departure, a detected misuse. It must be possible immediately, out of schedule, and without a long approval chain — but it must also be safe (see phased rotation below).
- **Define the trigger conditions in advance.** "Rotate when compromised" is useless if no one has defined what counts as compromised. Write down the events that force emergency rotation and who can declare it.

The trap is relying on scheduled rotation alone: a key compromised today but rotated only on schedule exposes every record encrypted between compromise and rotation. Emergency rotation must be rehearsed, not merely documented.

### Rotate In Phases (Dual-Key) To Avoid Breaking In-Flight Data

The most common rotation failure is an atomic cutover: the new key becomes active, the old key is deactivated, and any ciphertext produced or in flight under the old key becomes undecryptable or unverifiable. Correct rotation is phased, with an overlap window where both keys are valid.

- **Introduce the new key before withdrawing the old.** Publish the new key (or its public counterpart) to all participants first, so they can begin using it, while the old key remains available.
- **New encryption uses the new key; old key remains for decryption only.** During the overlap, the active encryption key is the new one, but the old key is retained on the decryption keyring so historical data stays readable.
- **Allow an overlap window long enough for all in-flight data to settle.** For long-lived data (encrypted columns, archived blobs), the old key may need to be retained for decryption for years; for short-lived data (session tokens), minutes may suffice.
- **For asymmetric keys (signing, verification), publish the new public key and let verifiers learn it before signatures stop validating under the old key.** A signature made under a key no verifier trusts is a broken signature.

Phased rotation treats rotation as a migration, not a switch. Plan the overlap by data lifetime: anything still needing decryption under the old key means the old key must be retained for decryption until that data is re-encrypted or expires.

### Re-Encrypt Historical Data Deliberately, Or Accept It Stays Under The Old Key

After rotation, historical ciphertexts are still encrypted under old keys. There are two valid strategies, and the choice must be explicit rather than accidental.

- **Retain old keys for decryption (lazy migration).** Old ciphertexts are left as-is; the old keys are kept on the decryption keyring. This is simple and correct, but it means old keys cannot be fully destroyed until the data they protect is no longer needed — which extends the key's effective lifetime to the data's lifetime, not the rotation interval.
- **Re-encrypt (active migration).** Historical data is re-encrypted under the new key, after which the old key can be retired. This is necessary when the old key is compromised (you must move data off it) or when policy requires destruction of old keys. Re-encryption must be batched, idempotent, resumable, and verified — re-encrypting a large dataset is itself a risky operation that can corrupt or lose data if interrupted.
- **For envelope encryption, re-wrap the data keys rather than re-encrypting the data.** The data key is encrypted (wrapped) under the master key; rotating the master key means re-wrapping each data key under the new master key, which is cheap and does not touch the underlying ciphertext. This is the standard reason to use envelope encryption: master-key rotation becomes a re-wrap, not a full re-encryption.

Never silently leave data under a key you believe is compromised. If the key is suspect, the data must be re-encrypted under a fresh key, and the old key treated as exposed.

### Plan Escrow And Backup So Key Loss Does Not Mean Data Loss

A key with no backup is a single point of failure: lose it, and every ciphertext it protects is unrecoverable. But a key with a sloppy backup is a leak vector. Escrow and backup must balance recoverability against exposure.

- **Back up master keys to a separate, hardened store** (a second KMS/HSM, or a quorum-protected escrow), not to the same system that uses them. A backup that lives next to the key protects against neither the failure that takes out the key.
- **Use quorum-based escrow (M-of-N) for high-value keys**, so recovering the key requires multiple trusted parties, preventing a single insider or single-site failure from compromising or losing it.
- **Test recovery.** An untested backup is assumed intact; a key-restore drill that has never run is a backup that may not work. Periodically restore from escrow in a non-emergency context.
- **Scope backups to keys, not to data, where possible.** Envelope encryption means only the master key needs escrow; per-object data keys are disposable and wrapped, so losing one loses one object, not the system.

The trap is the extreme ends: no backup (key loss = data loss) or a backup that is too accessible (a copy of the master key in source control or a shared drive, which turns a rotation into theater because the old key never really went away).

### Retire Keys Securely And Prove The Secret Is Gone

Retirement is the end of the lifecycle, and it is where "the key is gone" must actually be true. A key that is merely deactivated but still present in a backup, a log, a crash dump, or a retired server is still a live secret.

- **Distinguish deactivation from destruction.** Deactivation stops new use but retains the key for decryption of historical data; destruction removes the key entirely and makes its ciphertexts unrecoverable. Decide which applies.
- **Destroy the key in every place it lived**, including KMS, backups, escrow copies, application memory/config, CI/CD secrets, and any server that held it. A key destroyed in the primary store but surviving in a backup is not destroyed.
- **For envelope encryption, retire data keys by re-wrapping or re-encrypting**, then destroying the old wrapped form; the master key retirement is the high-stakes one.
- **Retire only when no data depends on the key, or accept the data is lost.** Destroying a key that still protects needed data is an unrecoverable data-loss event, not a security improvement.

## Common Traps

### Rotating Without A Key Id On Existing Ciphertexts

Switching to a new key when historical ciphertexts carry no key identifier, so the system cannot tell which key to use for decryption and either fails or tries the wrong key. Every ciphertext must record its key id from the first record ever written, because retrofitting ids onto existing data is often impossible.

### Atomic Cutover That Breaks In-Flight Data

Making the new key active and the old key unavailable in a single step, so messages encrypted just before the cutover (or cached, queued, or in transit) cannot be decrypted. Rotation must be phased with an overlap window sized to the data's lifetime.

### Deleting Old Keys To "Clean Up" And Orphaning Data

Removing previous keys from the keyring because they are "no longer active," which makes every ciphertext still encrypted under them permanently undecryptable. Old keys must be retained for decryption until their data is re-encrypted or no longer needed; deactivation is not destruction.

### Relying Only On Scheduled Rotation

Rotating on a fixed calendar interval while a key compromised between rotations stays in service, exposing all data encrypted in the gap. Emergency (event-based) rotation must be possible immediately and rehearsed, with defined trigger conditions.

### Re-Encrypting Without Verification Or Resumability

Running a batch re-encryption of historical data that, if interrupted, leaves some records encrypted under the old (possibly compromised) key and some under the new, with no record of which. Re-encryption must be idempotent, resumable, and verified per record.

### Master Key With No Escrow

A single master key with no backup or escrow, so its loss (hardware failure, deletion, insider action) makes every dependent data key and all encrypted data unrecoverable. Back up master keys to a separate, quorum-protected store and test recovery.

### Escrow Or Backup That Is Too Accessible

A copy of the master key committed to source control, stored in a shared drive, or printed in a runbook, so rotation provides no real protection because the old key persists in an accessible location. Escrow must be hardened and quorum-based; treat any key copy as a live secret.

### Treating Deactivation As Destruction

Marking a key inactive but leaving it in backups, logs, crash dumps, or retired servers, then believing the secret is gone. Destruction must remove the key from every location it occupied, or the key remains a live attack surface.

## Self-Check

- [ ] Every ciphertext, token, or protected blob carries a stable key id and version so decryption can select the exact key used, and (for AEAD) the key id is bound into the associated data to prevent substitution.
- [ ] The system uses a versioned keyring (one active encryption key plus retained previous keys for decryption), and historical data is decryptable after rotation because the old keys are still held.
- [ ] Rotation is phased with an overlap window: the new key becomes the active encryption key while the old key remains valid for decryption long enough for all in-flight and long-lived data to settle, sized to the data's actual lifetime.
- [ ] Time-based (scheduled) and event-based (emergency) rotation are distinguished, with the trigger conditions for emergency rotation written down, and emergency rotation is rehearsed rather than only documented.
- [ ] The strategy for historical data after rotation is explicit: either old keys are retained for decryption for the data's full lifetime, or data is re-encrypted (batched, idempotent, resumable, verified) — and data under a suspected-compromised key is re-encrypted, not left in place.
- [ ] Envelope encryption is used so master-key rotation is a re-wrap of data keys rather than a full re-encryption of all data, and master keys are backed up to a separate, quorum-protected escrow with a tested recovery drill.
- [ ] Retirement distinguishes deactivation from destruction, destroys the key in every location it lived (KMS, backups, escrow, config, CI, retired hosts), and destroys a key only when no needed data still depends on it.
- [ ] No old key has been deleted as "cleanup" while ciphertexts still depend on it, and no master key exists as a single unbacked copy whose loss would mean total data loss.
