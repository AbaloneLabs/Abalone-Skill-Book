---
name: ota_update_and_device_management.md
description: Use when the agent is designing over-the-air (OTA) firmware update for embedded or IoT devices; implementing A/B partition schemes, rollback, delta or partial updates, or anti-bricking safeguards; planning versioning, device authentication, signing, and key management; designing large-scale rollout strategies (canary, staged, ring-based); managing device fleets, provisioning, or decommissioning; or diagnosing update failures, bricked devices, rollback loops, or fleet-wide regressions. Covers secure boot and image signing, A/B vs single-bank updates, power-fail safety, rollback triggers, and the operational risks of updating devices that cannot be physically reached.
---

# OTA Update And Device Management

An OTA update is the most dangerous operation a deployed device routinely performs, because it replaces the firmware the device needs to boot, on devices you cannot physically reach, often in environments where power can fail mid-write. Get it wrong and the device does not recover on reboot — it is bricked, requiring physical return or field service across a fleet that may number in the millions. Agents trained on software update (where a failed update is rolled back by redeploying) routinely underestimate OTA: they treat it as "copy the new binary and reboot," then ship an update that bricks devices when power drops during the write, that fails to boot and has no rollback, or that rolls out a regression to the entire fleet in minutes because there was no staged rollout. The judgment problem is designing an update path that is power-fail-safe, that can always roll back to a known-good image, that authenticates and verifies images so an attacker cannot push malware, and that rolls out gradually so a bad update is caught before it reaches the fleet.

The harm of a casual OTA design is the worst kind of field failure: not a single broken device, but a coordinated brick or regression across the fleet, recoverable only by physical access to devices scattered across the world. OTA must be designed as if every update could fail to boot — because eventually one will.

## Core Rules

### Design For Power-Fail Safety At Every Step

The defining risk of OTA is that power can fail at any point — during download, during flash write, during the reboot — and the device must still boot afterward. A single-bank update that overwrites the running image in place is not power-fail-safe: an interruption mid-write leaves a corrupt image and a brick. The robust pattern is A/B (dual-bank) partitions: the running system writes the new image to the inactive bank without touching the active one, verifies it, and only then swaps the boot pointer. At no point is the bootable image destroyed before a verified replacement exists. If power fails during download or write, the active bank is untouched and the device boots normally. This is the single most important OTA design decision; a single-bank scheme with "we'll be careful" is a brick waiting for the first power glitch.

### Guarantee A Working Rollback Path Before You Need It

An update that boots but misbehaves (crashes, fails to connect, breaks a function) must be able to return to the previous known-good image automatically, without human intervention. The mechanisms: a bootloader that watches a "marked good" flag — the new image sets it only after proving it can run (e.g., reached a healthy state within a timeout); if the flag is not set, the bootloader rolls back to the previous bank on the next reboot. Design the rollback trigger deliberately: a watchdog reset loop, a failure to check in, a health-check failure. The principle is that a bad update must be self-healing: the device detects it cannot function and reverts, rather than sitting in a broken state. The recurring failure is an update that can be applied but has no automatic rollback, so a boot-looping new image bricks the device until someone intervenes.

### Authenticate And Verify Images End-To-End

A device that accepts any firmware over the air is a remote-code-execution vector for anyone who can reach the update channel. Every image must be cryptographically signed, and the device must verify the signature against a trusted key before it will even consider writing the image — and certainly before booting it. The signing keys are now among the most sensitive secrets in the system: compromise them and an attacker can push malware to the entire fleet. Manage keys with rotation and revocation in mind, use a hardware root of trust or secure boot where the platform supports it, and ensure the verification cannot be bypassed (a "recovery mode" that accepts unsigned images is a common bypass exploited in the field). The chain — build signs image, device verifies signature, bootloader enforces secure boot — must be unbroken end to end. A signed update with a bypassable recovery mode is unsigned in practice.

### Version Deliberately And Make Updates Idempotent

Every image and every device needs a coherent version story. Images carry a version; devices report their current version; the update system targets versions explicitly (not "update everyone to latest," but "move devices on 1.3 to 1.4"). Updates must be idempotent and resumable: a device that downloads half an image, loses power, and resumes must end up with the correct full image, not a half-written corrupt one. Use checksums per chunk and verify the complete image before swapping the boot pointer. For configuration and state that persists across updates, define migration paths so an update from version N to N+2 does not break when it encounters state from N. The discipline: treat the update as a transaction that either completes to a verified, bootable, healthy state or leaves the device on its previous known-good state — never in between.

### Roll Out Gradually, Never To The Whole Fleet At Once

A bad update pushed to the entire fleet simultaneously is a fleet-wide outage, recoverable only if rollback works everywhere — which you have not yet proven at scale. The standard defense is staged rollout: update a small canary group first, monitor health metrics (crash rate, connectivity, battery, function-specific telemetry), and only proceed to larger rings if the canary is healthy. Define the health gates that must hold before expanding (e.g., crash rate below threshold for 24 hours), define the abort and rollback procedure if a gate fails, and make rollout abortable at every stage. The recurring catastrophe is an update pushed to everyone at once that contained a regression caught only after thousands of devices were affected. Stage every update as if it might be bad — because eventually one will be.

### Plan The Operational Lifecycle, Not Just The Update

OTA is one operation in a device lifecycle that also includes provisioning (enrolling a new device with identity and credentials), monitoring (knowing what version each device runs and whether it is healthy), decommissioning (revoking a device's credentials and removing it from the fleet), and key rotation. Each must be designed for, because a fleet is not just "devices that update" — it is a population whose state you must know and control over years. Provisioning that shares credentials across devices, monitoring that cannot tell you how many devices are on a vulnerable version, or decommissioning that leaves credentials active are each a security or operational incident waiting to happen. Design the management plane alongside the update plane; an OTA system without fleet visibility is updating blind.

### Account For Bandwidth, Storage, And Device Diversity

Updates can be large, devices can be on constrained or metered connections, and the fleet may have diverse hardware revisions requiring different images. Delta updates (transmit only the changed portions) cut bandwidth dramatically for small changes but add complexity in applying them safely. Partial updates (update one component, not the whole image) reduce size but complicate version compatibility. Different hardware revisions may need different builds, and the update system must target the right image to the right device — a wrong-hardware image that boots can brick as surely as a corrupt one. Reason about the bandwidth budget for the fleet (updating a million devices on cellular is a real cost), the storage budget on device (holding two banks doubles the image footprint), and the targeting logic that ensures each device gets a compatible image.

## Common Traps

### Single-Bank Update Without Power-Fail Safety

Overwriting the running image in place, so a power loss mid-write bricks the device. Use A/B partitions so the active image is never destroyed before a verified replacement exists.

### No Automatic Rollback

An update that can be applied but has no mechanism to revert if the new image misbehaves, leaving boot-looping or broken devices to be fixed manually. Implement bootloader-level rollback with a "marked good" health check.

### Bypassable Image Verification

A signed-image scheme with a recovery or debug mode that accepts unsigned firmware, which an attacker exploits to bypass the whole chain. Verification must be unbypassable; secure boot enforces it in hardware.

### Pushing To The Whole Fleet At Once

Releasing an update to every device simultaneously, so a regression becomes a fleet-wide outage before it is caught. Stage rollout with canaries, health gates, and abort capability.

### Shared Provisioning Credentials

Enrolling devices with a shared key or certificate, so one compromised device exposes the fleet. Use per-device identity and credentials.

### No Visibility Into Fleet Version Or Health

An update system that pushes images but cannot report how many devices are on which version or whether they are healthy, so regressions are detected by customer complaints, not monitoring. Build the management plane with the update plane.

### Wrong Image To Wrong Hardware

Targeting an update without accounting for hardware revisions, so a device receives an incompatible image that bricks it. Target by hardware revision and verify compatibility before applying.

### Update That Breaks Persisted State

An update whose new firmware cannot read or migrate configuration/state written by the old version, so "successful" updates lose device configuration. Define and test state migration across version paths.

## Self-Check

- [ ] The update scheme is power-fail-safe at every step: A/B (dual-bank) partitions ensure the active bootable image is never destroyed before a verified replacement exists; an interrupted update leaves the device booting its previous good image.
- [ ] An automatic rollback path exists and is tested: a bootloader-level "marked good" health check reverts to the previous bank if the new image fails to reach a healthy state, without human intervention.
- [ ] Every image is cryptographically signed and verified on device against a trusted key before write and before boot; verification is unbypassable (no unsigned recovery mode), and secure boot anchors the chain in hardware where supported.
- [ ] Signing keys are managed with rotation and revocation in mind; per-device provisioning credentials are used, not shared fleet keys.
- [ ] Updates are idempotent and resumable: chunk checksums, full-image verification before boot swap, and defined state migration across version paths so persisted configuration survives updates.
- [ ] Rollout is staged (canary then rings) with explicit health gates (crash rate, connectivity, function telemetry), an abort procedure at every stage, and rollback-to-previous verified at each scale before expanding.
- [ ] The management plane exists alongside the update plane: provisioning, fleet version and health visibility, decommissioning with credential revocation, and key rotation are all designed, not omitted.
- [ ] Bandwidth and storage budgets were considered (delta/partial updates where justified, the doubled storage of A/B accounted for), and update targeting accounts for hardware revisions so devices receive compatible images.
- [ ] A full-cycle test was run: applying a deliberately bad update confirmed automatic rollback; a power-fail during write confirmed the device boots its previous image; and a staged rollout confirmed a regression is caught and contained at the canary stage.
