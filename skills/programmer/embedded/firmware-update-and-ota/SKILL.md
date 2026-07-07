---
name: firmware_update_and_ota.md
description: Use when the agent is designing or implementing firmware update, over-the-air (OTA) update, bootloader, or secure boot for embedded devices, IoT products, or fielded microcontroller fleets; choosing between dual-bank, A-B partition, swap, or single-bank-with-download update schemes; ensuring rollback safety and power-fail-during-update recovery; verifying image authenticity via signatures, checksums, or hash; handling constrained or unreliable transport (LoRa, BLE, cellular, low-bandwidth mesh); managing version compatibility with hardware revisions; implementing anti-rollback; or preventing a bad update from bricking a device fleet. Covers bootloader design, atomic image commits, secure verification, and the failure mode where a single flawed update disables every device at once.
---

# Firmware Update And OTA

A firmware update mechanism is the one feature that can brick a device you can no longer reach. On a server, a bad deploy is rolled back from a console; on a fielded embedded device sold in the hundreds of thousands, a bad update pushed over the air is a fleet-wide recall. The defining property of a safe update is not that it installs the new image — it is that the device is *guaranteed recoverable* when the update fails, is interrupted by power loss, is corrupted in transit, or turns out to be broken after it boots. Agents trained on cloud deployment models routinely ship OTA that works perfectly in the lab and bricks devices in the field, because the lab never exercised the failure path: the update that was 90% written when the battery died, the image that passed its checksum but had the wrong hardware revision, the "improved" firmware with a regression that now runs on every device with no way back.

The judgment problem is deciding how the update is staged so a failure can never leave the device unbootable, how the new image is verified before it is trusted, how a broken update is rolled back automatically, and how the update scheme is constrained by the realities of unreliable links and finite flash. The harm of casual decisions is the worst outcome in embedded engineering: a defect that requires physical recovery of hardware you cannot physically reach.

## Core Rules

### Guarantee Recovery: Never Overwrite The Running Image Without A Fall-Back

The foundational rule of field firmware update is that a power loss or failure at any point must leave a bootable image. This rules out the naive scheme of erasing and rewriting the active image in place — a power failure mid-write leaves a partially-erased image and a brick. The robust patterns all preserve the known-good image until the new one is proven:

- **Dual-bank / A-B partitioning.** Flash is split into two slots. The device boots from slot A; an update is written to slot B while A keeps running. Only after B is fully written and verified does the bootloader switch the active slot. If B fails to boot or a watchdog fires, the bootloader falls back to A. This is the standard safe scheme and the basis of most production OTA.
- **Swap / B/G with scratch.** A smaller scheme uses a scratch sector to shuffle images; more complex but works with less flash. The invariant is the same: the running image is never destroyed until the replacement is committed.
- **Single-bank with download-to-RAM.** Acceptable only for very small images on devices that can hold the whole new image in RAM, verify, then write atomically; rare and risky.

The invariant across all safe schemes: there is always a known-good, committed image the bootloader can return to, and the "commit" of the new image is an atomic operation (a single flag or sector write) that cannot be torn by power loss.

### Verify The Image Cryptographically Before Trusting It

An update image arriving over any transport can be corrupted in transit or tampered with. A CRC or checksum catches corruption but not an attacker; a cryptographic signature (the firmware is signed with a private key, the device verifies with an embedded public key) is the only defense against a malicious or substituted image. Verify the signature *before* the image is allowed to boot, and verify it against the full image, not a header field an attacker could rewrite. For constrained devices, a hash (SHA-256) plus a signature (ECDSA or RSA) over the hash is the common pattern. Tie signature verification to the bootloader or secure boot, not to application code an attacker could bypass — and protect the verification keys and the boot path in a region the application cannot rewrite. An OTA scheme with no signature verification is a remote code execution vector for anyone who can reach the transport.

### Make Rollback Automatic, Not Manual

A new image that boots but misbehaves (hangs, crashes, fails its watchdog, cannot reach the network to report success) must roll back to the previous image without human intervention, because the device may be unreachable. The standard pattern: on selecting the new slot, the bootloader marks it "pending" or "trial." If the application does not, within a deadline, signal "I am healthy, commit me" (pet a commit watchdog, set a confirmed-healthy flag), the bootloader reverts to the last known-good slot on the next reset. The application must be designed to make that health signal honest — it should mean "I brought up the radio, ran self-test, and can function," not "I reached main()." Without automatic rollback, a single regression in a new image becomes a permanent brick on every device that received it.

### Handle Power-Fail At Every Step, Especially The Commit

Power can fail at any instant: during the download, during the erase, during the write, during verification, during the commit flag write itself. The design must be idempotent and resumable at each step. Erases and writes to the inactive slot are safe to repeat (the slot is not bootable until committed). The commit itself — the single operation that makes the new slot active — must be atomic at the storage level: use a single write to a known location, and design the bootloader to interpret a partially-written commit deterministically (treat torn commit as "do not switch," i.e., stay on the known-good slot). Flash erase granularity matters: a commit flag stored in flash cannot be written without erasing its sector, so use a counter, a rotating marker, or a dedicated small sector so the commit does not require erasing part of an image. The failure to reason about "what does the bootloader see if power dies exactly here" is the source of most field bricking.

### Constrain The Transport And Respect Hardware Revision And Anti-Rollback

Two constraints that agents routinely omit:

- **Transport reliability and resumability.** Over BLE, LoRa, cellular, or a lossy mesh, an image will arrive in fragments with drops, retransmits, and outages. The transfer must be resumable (a partial image survives a dropped connection and resumes), the written fragments must be verified as they arrive or on completion, and the scheme must not assume a clean, fast link. A protocol that restarts from zero on every drop will never finish a large image on a constrained link.
- **Hardware-revision and anti-rollback compatibility.** An image must refuse to run on hardware it was not built for (a pin mapping change, a different sensor, a new silicon stepping) — encode the target hardware revision in the image and have the bootloader reject mismatches. Anti-rollback is the inverse: a security fix should not be overwritten by an older, vulnerable image, so the bootloader tracks a minimum acceptable version and refuses to boot images below it. Without anti-rollback, an attacker (or an operator error) can "downgrade" a patched device to a vulnerable version.

## Common Traps

### Overwriting The Running Image In Place

Erasing and rewriting the active firmware, then losing power partway and bricking the device. Always write to an inactive slot and switch atomically; the running image is sacred until the replacement is committed.

### Trusting A Checksum As "Verification"

Using a CRC to catch transit corruption and calling the image verified, when a CRC provides no defense against a substituted or malicious image. Use a cryptographic signature verified by the bootloader; checksums catch noise, not attackers.

### No Automatic Rollback Or A Commit That Cannot Survive Power Loss

Switching to the new image permanently on first boot, so an image that crashes after boot bricks the device; or writing the commit marker and assuming power cannot fail mid-write, leaving the bootloader with an ambiguous active slot. Implement a trial-boot with a health-confirmation deadline that reverts to the known-good image, and make the commit a single atomic storage operation whose every torn state the bootloader resolves to "stay on known-good."

### Pushing One Image For All Hardware Revisions, With No Anti-Rollback

Shipping a single OTA image across a fleet that spans hardware revisions or silicon steppings (bricking devices where the pin map or peripheral behavior differs), and permitting an older vulnerable image to be installed over a patched one (reopening a fixed vulnerability). Encode the target hardware revision in the image and reject mismatches in the bootloader, and track a minimum acceptable firmware version to refuse vulnerable downgrades.

### Ignoring Link Reliability On Constrained Transports

Designing the OTA transfer for a clean fast link, then failing forever on BLE/LoRa/cellular because a single dropped packet restarts the whole image. Make the transfer resumable and fragment-verified; constrained links will drop.

## Self-Check

- [ ] The update scheme never overwrites the running image; it writes to an inactive slot (dual-bank/A-B) and switches active slot via a single atomic commit, so power loss at any step leaves a known-good bootable image.
- [ ] The bootloader's behavior is defined for every possible torn/partial state (partial write, partial commit, failed verification) and resolves each to "stay on or revert to the known-good image."
- [ ] New images are verified with a cryptographic signature (not just a CRC/checksum) checked by the bootloader or secure boot before the image is allowed to boot.
- [ ] Automatic rollback is implemented: a new image boots in a trial state and reverts to the previous known-good image if it fails to confirm health within a deadline (watchdog/commit-pet pattern).
- [ ] The transfer is resumable and fragment-tolerant for the target transport; a dropped connection on BLE/LoRa/cellular resumes rather than restarting, and fragments are verified as they arrive.
- [ ] The image encodes its target hardware revision (and silicon stepping where relevant), and the bootloader rejects images built for incompatible hardware.
- [ ] Anti-rollback is enforced: the bootloader tracks a minimum acceptable version and refuses to boot images older than the current security baseline.
- [ ] A power-fail test was performed at every step of the update (during download, during erase, during write, during verify, during commit) and the device recovered to a bootable state in every case.
