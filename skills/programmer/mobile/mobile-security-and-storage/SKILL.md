---
name: mobile_security_and_storage.md
description: Use when the agent is handling secrets, credentials, tokens, or sensitive user data in a mobile application; choosing secure storage (keystore, keychain, secure enclave, encrypted SQLite); implementing biometric authentication (Face ID, Touch ID, fingerprint); configuring certificate pinning or TLS validation; detecting root or jailbreak; encrypting data at rest; protecting sensitive views from screenshots, screen recording, or the app switcher; or deciding where to keep API keys and third-party secrets. Also covers the failure mode of storing secrets in plaintext shared preferences, plist, or local storage, shipping API keys or signing secrets inside the app binary, relying on client-side checks that a rooted or repackaged app defeats, and assuming HTTPS alone protects against MITM.
---

# Mobile Security And Storage

A mobile app runs on a device the developer does not control, in the hands of a user who may be the attacker, on hardware that can be rooted, jailbroken, inspected, and repackaged. The judgment problem is that mobile is a hostile, untrusted environment: the binary and its resources can be extracted, the local storage can be read, the network traffic can be intercepted, and the app's own logic can be patched out. Security decisions that are safe on a server (where the attacker cannot reach the memory) are unsafe on a client (where the attacker owns the device). The discipline is to assume the device and the binary are compromised, to store nothing sensitive that you do not absolutely have to, to put what you must store into hardware-backed secure storage, to validate the server identity you talk to (because HTTPS alone does not stop a proxy with a trusted certificate), to protect sensitive UI from capture, and to keep secrets out of the binary entirely — because anything shipped in the app can and will be extracted.

Agents tend to treat mobile security as "use HTTPS and a token," which is the floor, not the ceiling. The harm appears as auth tokens and PII readable in plaintext on a backed-up device, as API keys and third-party secrets extracted from the APK/IPA and abused, as traffic intercepted through a proxy that the app trusts, as screenshots and screen recordings of banking or health screens leaking sensitive data, and as root/jailbreak bypasses that defeat client-only checks. The judgment is to treat the device as untrusted, to use hardware-backed secure storage for anything sensitive, to pin certificates against the specific backends that matter, to protect capture-sensitive views, to detect (but not rely on) rooted/jailbroken environments, and to move any secret that must remain secret to the server rather than embedding it in the client. Mobile security is defense in depth against an attacker who holds the device.

## Core Rules

### Treat The Device And Binary As Untrusted

The mobile device is an untrusted host: it can be rooted or jailbroken, its storage can be read, its backups expose app data, the binary can be unpacked and inspected, and the app's logic can be patched. Every security decision must start from the assumption that the attacker controls the device and has the binary.

- **Store nothing sensitive you do not have to.** The most secure secret is the one not on the device. Prefer ephemeral, short-lived tokens; fetch secrets from the server at runtime rather than embedding them; scope tokens to the device so a leak is contained.
- **Assume local storage is readable.** Plaintext shared preferences, plist, UserDefaults, AsyncStorage, SQLite, and files in the app sandbox are readable on a rooted/jailbroken device and often via backups. Do not store secrets, tokens, or PII there.
- **Assume the binary can be unpacked.** String literals, assets, and compiled code can be extracted from an APK or IPA; anything embedded (keys, URLs, checks) is visible to an attacker. Do not rely on obscurity inside the binary.
- **Assume the app can be repackaged.** A determined attacker can resign a modified app that skips your checks; client-only enforcement (a root check that can be patched out) is a deterrent, not a guarantee.

### Use Hardware-Backed Secure Storage For Anything Sensitive

When a secret or sensitive value must live on the device (an auth token, a refresh token, an encryption key, biometric-gated data), it must go into hardware-backed secure storage — the iOS Keychain (ideally Secure Enclave-backed, non-portable classes), the Android Keystore (hardware-backed keys that never leave the TEE/StrongBox) — not into ordinary app storage. The distinction is not cosmetic: hardware-backed keys cannot be extracted even from a rooted device, while plaintext preferences can be read in seconds.

- **Prefer hardware-backed, non-extractable keys.** Generate keys inside the secure enclave / Keystore so the private material never enters app memory and cannot be dumped; mark keys as requiring user authentication (biometric or device passcode) where appropriate.
- **Store tokens and credentials in the Keychain / Keystore-backed store, not in shared preferences, plist, or plain SQLite.** If you need a structured store, encrypt it with a key held in the Keystore rather than storing the data and the key together in plaintext.
- **Choose storage protection classes deliberately.** On iOS, choose the data-protection class (`NSFileProtectionComplete`, `completeUnlessOpen`, etc.) that matches when the data should be decryptable; `complete` requires the device to be unlocked. On Android, use `EncryptedSharedPreferences` / encrypted files with Keystore-backed keys, and set lock-based authentication where it matters.
- **Handle biometric as a key-release gate, not as authentication to your server.** Biometrics unlock a local key; they do not prove identity to the server. Use the biometric-unlocked key to decrypt a token or sign a challenge, and let the server validate the result.

### Validate The Server You Talk To (Certificate Pinning)

HTTPS encrypts the channel, but it does not prove the server is yours: a device with a user-installed (or enterprise-installed, or maliciously-installed) trusted certificate can MITM the connection, and the app will trust the proxy. For backends carrying sensitive data, validate that the certificate is specifically yours, not merely that a CA somewhere vouched for it.

- **Pin the certificates or public keys of your own backends.** Pinning the SPKI (public key) is more robust to certificate rotation than pinning the leaf certificate; pin a backup key so a rotation does not lock the app out.
- **Pin only backends you control.** Pinning third-party APIs breaks when they rotate certificates; pin your own auth/API endpoints where the risk of MITM is yours to bear.
- **Handle pin failure explicitly and safely.** A pin failure means "do not trust this connection"; fail closed (block the request) and report, rather than falling back to ordinary TLS validation.
- **Have a rotation and bypass strategy before you ship.** Pinning without a plan for key rotation or emergency bypass strands users on a broken app; ship a backup pin and a versioned config you can update.

### Protect Sensitive Views And Data From Capture

Screens that display sensitive content (financial details, health records, credentials, one-time codes) can be captured by screenshots, screen recording, the app-switcher preview, or external displays. Protect these views deliberately rather than assuming the OS will not expose them.

- **Flag capture-sensitive screens.** On iOS, set `isHidden`/the secure screenshot flag on the window or view so the app-switcher and screen recording show a blank; on Android, set `FLAG_SECURE` on the window to block screenshots, screen recording, and the recents preview.
- **Consider whether to block or to warn.** Blocking capture protects data but can frustrate legitimate users; for regulated content, block; for convenience-sensitive content, consider warning. Make the choice deliberately.
- **Clear sensitive data from the UI on backgrounding.** When the app goes to the background, obscure or clear sensitive fields so the app-switcher preview does not show them.
- **Do not log or cache sensitive data.** Crash logs, analytics, and debug screenshots are a common leak vector; redact tokens, credentials, and PII before any logging or telemetry.

### Keep Secrets Out Of The Binary And Detect Hostile Environments

Anything embedded in the APK or IPA — API keys, third-party SDK keys, signing secrets, hardcoded credentials — can and will be extracted. The robust answer is to not ship secrets at all; the partial answer is to detect and deter hostile environments.

- **Do not ship secrets in the binary.** API keys, signing keys, and service credentials belong on the server; the app should fetch scoped, short-lived credentials at runtime. If a third-party SDK requires an embedded key, proxy it through your server so the real key is not in the client.
- **Scope and rate-limit any client-held credential.** If a key must live in the client, scope it narrowly (per-device, per-user, read-only) and rate-limit it so extraction damage is bounded.
- **Detect root/jailbreak as a signal, not a guarantee.** Rooted/jailbroken detection (SafetyNet/Play Integrity, jailbreak indicators) raises the bar and can gate high-risk features, but it can be bypassed; treat a failure as a risk signal (degrade, warn, require step-up auth) rather than assuming a clean check means a clean device.
- **Use app-attestation where the stakes justify it.** For high-value flows, have the server verify the app's integrity (Play Integrity / DeviceCheck / App Attest) before trusting a request, so a repackaged or fake client is rejected server-side.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what threat model was assumed, what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A security decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Storing Secrets In Plaintext App Storage

Putting tokens, credentials, or PII into shared preferences, plist, UserDefaults, AsyncStorage, or plain SQLite, readable on a rooted device or via a backup. Use hardware-backed Keychain / Keystore storage; encrypt structured stores with a Keystore-held key; store nothing sensitive you do not have to.

### Shipping API Keys Or Secrets In The Binary

Embedding API keys, third-party SDK keys, or signing secrets as string literals or in build config, assuming they are hidden. They are not: the APK/IPA can be unpacked and strings extracted. Keep secrets on the server; fetch scoped, short-lived credentials at runtime; proxy third-party keys through your backend.

### Trusting HTTPS Without Pinning

Assuming HTTPS protects against MITM, when a user-installed or enterprise trusted certificate lets a proxy intercept traffic. Pin your own backends' public keys, fail closed on mismatch, and plan for rotation with a backup pin.

### Relying On Client-Only Checks That Rooting Defeats

A root/jailbreak check, a "isDebuggable" guard, or a feature gate that exists only in the client and can be patched out of a repackaged app. Treat these as deterrents and risk signals; enforce what matters server-side with app attestation; degrade rather than trust a clean client check.

### Leaking Sensitive Data Through Screenshots, Recording, Or Logs

Banking or health screens that can be screenshotted or recorded, app-switcher previews that show sensitive fields, and crash/analytics logs that capture tokens or PII. Flag capture-sensitive windows, clear sensitive UI on backgrounding, and redact before any logging or telemetry.

### Biometric Treated As Server Authentication

Treating a successful Face ID / fingerprint prompt as proof of identity to the server. Biometrics unlock a local key; use that key to decrypt a token or sign a challenge, and let the server validate. Do not send "biometric passed" as an auth assertion.

## Self-Check

- [ ] The device and binary are treated as untrusted: nothing sensitive is stored that does not need to be, ephemeral and scoped tokens are preferred over long-lived secrets, and it is assumed that local storage, the binary, and app logic can all be inspected or patched.
- [ ] Secrets, tokens, and credentials are stored in hardware-backed secure storage (iOS Keychain / Secure Enclave, Android Keystore-backed EncryptedSharedPreferences or encrypted files), with non-extractable keys, deliberate data-protection classes, and biometric/device-passcode gating where appropriate — not in shared preferences, plist, AsyncStorage, or plain SQLite.
- [ ] Sensitive backends use certificate / public-key pinning (SPKI pins with a backup for rotation), pin failures fail closed, and only first-party endpoints are pinned; the rotation and emergency-bypass strategy was designed before shipping.
- [ ] Capture-sensitive screens are protected: screenshots, screen recording, and the app-switcher preview are blocked or blanked (FLAG_SECURE / secure screenshot flag), sensitive UI is cleared on backgrounding, and tokens/PII are redacted from logs and analytics.
- [ ] No secrets are shipped in the binary: API keys, third-party SDK keys, and signing credentials live on the server, the app fetches scoped short-lived credentials at runtime, and any client-held key is scoped and rate-limited so extraction damage is bounded.
- [ ] Root/jailbreak detection and app integrity (Play Integrity / DeviceCheck / App Attest) are used as risk signals enforced server-side where the stakes justify it — not as client-only guarantees — and high-value flows verify app integrity before trusting a request.
- [ ] Biometric authentication is implemented as a key-release gate that decrypts a token or signs a challenge for the server to validate, not as an authentication assertion sent to the server.
- [ ] The highest-risk cases were verified — a secret extracted from storage on a rooted device, a MITM attempted with a trusted proxy, a screenshot/recording of a sensitive screen, and a repackaged app bypassing a client check — not only the clean path.
