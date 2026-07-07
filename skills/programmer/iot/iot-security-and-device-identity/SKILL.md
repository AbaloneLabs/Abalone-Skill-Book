---
name: iot_security_and_device_identity.md
description: Use when the agent is designing security, identity, or authentication for IoT or embedded devices; provisioning per-device credentials or certificates at manufacturing or enrollment; rotating keys or certificates across a fleet; implementing secure boot, a hardware root of trust, TPM, secure elements, or attestation; reasoning about compromised-device blast radius and key revocation at fleet scale; validating supply-chain firmware integrity and signing; or diagnosing a fleet that shares credentials, has an unbounded blast radius, cannot revoke a stolen key, or trusts firmware without a verifiable chain. Covers the security and identity judgment problem distinct from OTA update mechanics — per-device identity, provisioning lifecycle, rotation, attestation, and blast-radius containment.
---

# IoT Security And Device Identity

An IoT device is a networked computer that an attacker can usually reach, often physically hold, and almost never patch quickly — and its identity and keys are the only thing standing between a single compromised device and the entire fleet. Agents trained on web security, where a compromised server is one incident among many and keys rotate on demand, routinely ship IoT fleets with a shared key burned into every device, with credentials that cannot be revoked, or with firmware that trusts whatever it boots. Then one device is captured, its key extracted, and the attacker speaks as the entire fleet — or one stolen signing key pushes malware to every device, and there is no way to revoke it without bricking devices you cannot reach. The judgment problem is that IoT security is identity and blast-radius design: every device needs its own unforgeable identity, credentials must be provisioned and revocable per device, firmware must be verifiable end to end, and the system must assume some devices will be captured and contain the damage to that one device.

The harm of a casual security design is the worst kind of incident: not a single broken device, but a fleet-wide compromise, because the design made every device share the same secret or trust the same unrevocable key. IoT security must be designed as if some devices will be captured — because in a deployed fleet, some will be.

## Core Rules

### Give Every Device A Unique Identity, And Provision It At Manufacturing

The foundational rule is that no two devices share a secret. A shared fleet key means one captured device compromises every device — the attacker extracts the key and impersonates the fleet, and you cannot revoke it without re-keying every device, which you often cannot reach. Every device must have its own unique credential (a per-device certificate, key pair, or token), generated with strong entropy such that one device's secret cannot be derived from another's, and ideally stored in hardware (a secure element, TPM, or the chip's key store) so it cannot be extracted even by physical capture. Establish that identity before the device ever touches your network: provision credentials at manufacturing or a secure enrollment step, register the public part to your identity service (device registry, CA, cloud IoT service), and avoid generating keys on-device with a weak entropy source or distributing a shared secret at first boot. Plan the full enrollment lifecycle: how a new device is enrolled, how it is verified as legitimately yours (not a clone), how an RMA'd or resold device is transferred, and how a decommissioned device is revoked. The blast radius of a compromised device must be exactly one device. A fleet with a shared key has a blast radius of "everyone," which is not a security design; it is an incident waiting to happen.

### Anchor Trust In Hardware: Secure Boot, Root Of Trust, And Attestation

Software-only security on a device an attacker can hold is fragile: firmware can be replaced, keys in flash can be read, and a "verified" boot that runs in software can be bypassed by modifying the software. The robust anchor is a hardware root of trust: an immutable boot ROM (or secure element) that verifies the first-stage bootloader's signature before executing it, and a chain where each verified stage verifies the next — secure boot, end to end, so only signed firmware the device trusts can run. Where the platform supports it, use a TPM or secure element to hold keys and perform crypto so the private key never exists in extractable memory. Attestation extends this: a device can cryptographically prove to a verifier (your cloud, a gateway) what firmware version and state it is running, so you can refuse to trust a device running tampered or outdated firmware. The recurring error is a signed-firmware scheme with no hardware enforcement, or a debug/recovery mode that accepts unsigned images — both of which let an attacker bypass the entire chain on a captured device. Anchor trust in hardware that cannot be bypassed, and make attestation part of how the cloud decides what to trust.

### Design Key Rotation Before You Need It

Credentials that live for the lifetime of the device are credentials that cannot respond to compromise. A certificate that expires in 20 years cannot be rotated when its algorithm is broken or its CA is distrusted; a key that is "permanent" is permanent for the attacker too. Design rotation from the start: certificates and keys have defined lifetimes, the device can receive and activate a new credential while running on the old one (dual-key overlap), and the identity service tracks which credential is current per device. Rotation at fleet scale is an operational problem: you must rotate without bricking devices that are offline during the rotation window, without a flag day, and with a way to handle devices that miss the rotation. The recurring failure is a design where rotation is "theoretically possible" but never operationalized, so when a key must be rotated (compromise, algorithm deprecation) the team discovers it cannot actually be done without losing contact with part of the fleet. Build and rehearse rotation as a routine operation, not an emergency capability.

### Make Revocation Work At Fleet Scale, Without The Compromised Device's Cooperation

Revocation is the test of whether your identity design is real: when a device is captured, can you revoke its credential such that the attacker is immediately locked out, without affecting the rest of the fleet, and without needing to reach the captured device? Short-lived credentials (certificates or tokens that expire frequently and must be renewed) make revocation cheap — a revoked device simply cannot renew. Long-lived credentials require a revocation list that every verifier checks, which must propagate to every broker, gateway, and service that authenticates devices, and which must be checked even when the device is offline and reconnects later. Decide the revocation model per the threat: for high-value targets, short-lived credentials with online verification; for lower-stakes fleets, revocation lists with defined propagation. The failure mode is a design where revocation is "supported" but the verifier does not actually check the list, or where revoking a device requires reaching it — which is impossible for a captured device in an attacker's hands. Revocation must work without the cooperation of the compromised device; if it does not, you cannot contain a capture.

### Verify Supply-Chain Firmware Integrity End To End

The firmware on a device can be compromised long before it reaches the device — in the build, in transit, or via a tampered toolchain — and a device that boots "signed" firmware that was signed by a compromised build key is fully owned. The signing keys are among the most sensitive secrets in the system: protect them in an HSM or offline signing infrastructure, limit who can sign, and audit every signature. Verify integrity end to end: the build is reproducible and signed, the artifact is verified at every handoff (CI to artifact store, store to device, device to boot), and the device verifies the full chain before executing. This is distinct from the OTA mechanics of delivering an update (see the OTA skill) — the security judgment here is that the signature must be trustworthy, which requires protecting the signing key and verifying the chain at every boundary, not just at the device. A signed firmware whose signing key was leaked or whose build was tampered is unsigned in practice; the signature proves only that someone with the key signed something, not that the something is legitimate.

## Common Traps

### Shared Fleet Key Or Certificate

Burning one key or certificate into every device, so one captured device compromises the entire fleet and the key cannot be revoked without reaching every device. Use per-device credentials with a blast radius of one.

### Software-Only Security On A Capturable Device

Relying on signed firmware or keys-in-flash with no hardware enforcement, so an attacker who captures a device can bypass verification or extract keys. Anchor trust in a hardware root of trust and secure element.

### Credentials That Cannot Be Rotated, Or Revocation That Needs The Device

Permanent keys or 20-year certificates with no rotation path; or a revocation scheme that only works by pushing to the device, which is impossible for a captured device in an attacker's hands. Design rotation with dual-key overlap and revocation that works without the device's cooperation.

### Debug Or Recovery Mode That Accepts Unsigned Firmware

A secure-boot chain with a debug, recovery, or factory mode that accepts unsigned images, which attackers routinely exploit to bypass the entire chain. Verification must be unbypassable.

### Signed Firmware With An Unprotected Signing Key

A signing scheme whose key is accessible in CI or on a developer laptop, so a build-system compromise lets an attacker sign malware as legitimate — and no way to tell a cloned device (same credential) from a real one. Protect signing keys in an HSM and bind identity to hardware with attestation.

## Self-Check

- [ ] Every device has a unique credential (per-device certificate, key pair, or token); no two devices share a secret, and the blast radius of a single compromised device is exactly one device.
- [ ] Credentials are generated with strong entropy and stored in hardware (secure element, TPM, or chip key store) where the platform supports it, so they cannot be extracted from a captured device.
- [ ] Identity provisioning is a manufacturing-integrated, auditable lifecycle: enrollment, verification against clones, transfer (RMA/resale), and decommission/revocation are all designed, not ad-hoc.
- [ ] Trust is anchored in a hardware root of trust: secure boot verifies the chain from immutable boot ROM through each stage, there is no debug/recovery mode that accepts unsigned firmware, and attestation is used where supported so the cloud or gateway can cryptographically verify firmware version and state and refuse to trust tampered or outdated devices.
- [ ] Key and certificate rotation is designed and rehearsed: credentials have defined lifetimes, dual-key overlap allows rotation without a flag day, and offline devices are handled without being bricked.
- [ ] Revocation works at fleet scale and without the compromised device's cooperation: short-lived credentials that cannot renew, or revocation lists that every verifier checks on reconnect.
- [ ] Supply-chain firmware integrity is verified end to end: signing keys are protected in an HSM or offline infrastructure, signatures are audited, and the device verifies the full chain before executing — not just that "something was signed."
- [ ] A capture simulation was run: a device was deliberately compromised (key extracted, firmware tampered), and the design correctly contained the blast radius to that device, revoked it without fleet impact, and refused to trust tampered firmware via attestation.
