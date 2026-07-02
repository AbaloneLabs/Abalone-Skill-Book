---
name: drm_and_content_protection.md
description: Use when the agent is implementing digital rights management, integrating Widevine, FairPlay, or PlayReady, designing license server interactions, handling content keys and key rotation, applying forensic watermarking, enforcing geographic and device restrictions, or protecting premium video and audio from unauthorized capture and redistribution.
---

# DRM and Content Protection

Content protection is the engineering of making paid media hard to copy, and it is unlike most software problems because it pits you against an adversary, not against bugs. A feature that "works" against honest users but is trivially bypassed by a determined attacker provides no protection. Agents often approach DRM as an integration task ("call the license API, get a key, decrypt playback") and miss that the entire value depends on a chain of trust whose weakest link defines the real protection level. A single hardcoded key, a log that prints the content key, or a client that fetches licenses without entitlement checks collapses the whole system.

The judgment problem is that DRM is a tradeoff between security, platform reach, device compatibility, user experience, and cost. There is no "most secure" option that works everywhere; FairPlay is Apple-only, Widevine is Android/Chrome/some TVs, PlayReady is Windows/Edge/Xbox, and reaching all devices means multi-DRM. Stronger protection (hardware-backed L1) excludes older or cheaper devices. Forensic watermarking adds cost and complexity but is the only deterrent after a leak. The agent must reason about the threat model (casual copying vs. organized piracy vs. insider leak), the device matrix, and the operational burden of key management, not just "make it play."

## Core Rules

### Start from a threat model, not from a DRM vendor

Before choosing technology, define what you are protecting against. Casual screen-recording by a user, organized redistribution by a pirate group, and an insider leaking a master file are different threats requiring different controls. DRM (encryption plus licensed playback) addresses unauthorized playback and redistribution of the encrypted stream. Forensic watermarking addresses post-leak attribution. Session and entitlement controls address unauthorized access. Choose the combination that matches the threat; over-protecting (e.g., requiring L1 hardware for low-value content) excludes users and raises support costs for little benefit, while under-protecting premium content invites redistribution the moment it launches.

### Understand protection levels and the device matrix

Each DRM system defines protection levels that determine where decryption happens. Widevine L1 decrypts in a hardware-backed trusted execution environment; L3 is software-only. FairPlay and PlayReady have analogous hardware vs. software distinctions. The protection level you require determines which devices can play your content. Requiring the strongest level excludes older or budget devices and some browsers. Decide the minimum acceptable protection level per content tier (e.g., L1 for premium early-window content, L3 acceptable for back-catalog) and verify the device matrix supports your choice. Do not assume a device supports a level because it supports the DRM system.

### Use multi-DRM for cross-platform reach, and abstract correctly

No single DRM reaches all devices. To cover iOS/Safari (FairPlay), Android/Chrome (Widevine), and Windows/Edge/Xbox (PlayReady), you need multi-DRM. The standard pattern is CENC (Common Encryption): encrypt each track once with a key, package with the appropriate DRM-specific headers (PSSH boxes) for each system, and serve a single encrypted asset that each platform licenses through its native DRM. Use a multi-DRM service or license proxy rather than integrating each DRM separately. Abstract the DRM-specific logic behind a service so the client requests a license generically and the backend routes to the right system per device.

### Enforce entitlement before issuing a license, not just before playback

The license server is the real enforcement point. A client that can fetch a license without a valid entitlement can decrypt and redistribute. Every license request must be authenticated and authorized against the user's actual entitlements (subscription state, purchase, region, device limit, concurrent-stream limit) at the moment of the request. Do not rely on the client to self-report entitlement, and do not cache entitlement so loosely that a cancelled subscription can still license content. Tie license issuance to a short-lived session and re-check on renewal.

### Protect the content key throughout its lifecycle

The content key is the secret that protects the media, and its exposure defeats all DRM. Never log content keys, never embed them in client code, never transmit them outside the DRM's secure channel, and never store them in plaintext at rest. Rotate keys per asset (each piece of content has its own key) and consider key rotation within long assets. Use a key management service (KMS) for storage and access control. Treat any logs, crash dumps, error messages, or debug output that might contain a key as a security incident and scrub them.

### Apply output protections where the content tier demands it

Decryption in a trusted environment is not the end of protection; the decrypted frames must also be protected on their way to the display. HDCP (for HDMI/digital outputs) prevents capture of the decrypted stream over the output path. Require HDCP for premium content and fail closed (block playback) when HDCP is unavailable, rather than downgrading silently. Be aware that HDCP has versions (1.4, 2.x) and that older displays and capture setups may not support newer versions; this is another device-matrix decision.

### Use forensic watermarking for attribution, not prevention

DRM prevents playback of copied encrypted streams, but a determined attacker with a legitimate session can still capture the decrypted output (e.g., via an HDCP stripper or an insider with access to decrypted frames). Forensic watermarking embeds an imperceptible, per-session identifier in the video so that a leaked copy can be traced back to the session that leaked it. Watermarking does not prevent copying; it enables attribution after the fact, which deters insiders and enables takedowns. It is computationally expensive and operationally complex (embedding at packager or client, extraction infrastructure, legal process for acting on findings). Use it for high-value content where post-leak attribution matters, and decide whether client-side or A/B-variant server-side watermarking fits your architecture.

### Fail closed, and make failures observable

When DRM cannot establish a trusted path (no supported DRM system, license fetch fails, HDCP unavailable, key rotation mismatch), the default must be to block playback, not to fall back to unprotected content. A "fail open" fallback that plays unencrypted content when DRM fails is worse than no DRM because it provides a bypass an attacker can trigger. Make all DRM failures observable (which step failed, on which device, for which asset) so you can distinguish a legitimate support issue from an attack pattern.

## Common Traps

### Logging or printing the content key during debugging

A debug log line that prints the content key, even briefly during development, is a permanent exposure if that log is retained or shipped. Attackers and insiders scan logs for secrets. Never log keys, and audit logging paths to ensure keys cannot leak via error messages or crash dumps.

### Issuing licenses based on client-reported entitlement

If the client tells the license server "I am entitled" and the server believes it, the DRM is bypassable by any client that lies. The license server must independently verify entitlement against authoritative state. Trusting the client is the single most common DRM implementation flaw.

### Assuming a device supports the required protection level

A device may support a DRM system (e.g., Widevine) but only at a software level (L3), not the hardware level (L1) your premium content requires. Shipping with an untested device matrix leads to "content won't play" support tickets from legitimate users on excluded devices. Test the matrix explicitly.

### Reusing one content key across many assets

A single key for all content means compromising one asset compromises all. Use per-asset keys (and consider per-segment or periodic rotation for long or high-value assets) so a key exposure is bounded.

### Failing open to avoid support tickets

When DRM fails, the pressure to "just let it play" to avoid a support ticket is strong. A fail-open fallback is a bypass. Block playback and make the failure observable and actionable instead.

### Treating watermarking as a replacement for DRM

Watermarking enables attribution after a leak; it does not prevent the leak. A watermarked-but-unencrypted stream can still be copied freely. Watermarking complements DRM for high-value content; it does not substitute for it.

### Ignoring region and device restrictions at the license server

Geo-restrictions and device limits enforced only at the app level are bypassable. The license server is the enforcement point; apply region checks (via the request source, not client-reported location) and device/concurrency limits there.

## Self-Check

- Have you defined a threat model (casual copying, organized piracy, insider leak) and chosen DRM, watermarking, and session controls to match, rather than defaulting to the strongest available protection everywhere?
- Do you know which protection level (hardware vs. software) each content tier requires, and have you verified the device matrix supports that level rather than assuming support from DRM-system presence?
- For cross-platform reach, are you using CENC with multi-DRM packaging and a license proxy, rather than integrating each DRM separately?
- Does the license server independently verify entitlement (subscription, purchase, region, device limit, concurrency) on every request, rather than trusting client-reported state?
- Are content keys never logged, never embedded in client code, never transmitted outside the DRM secure channel, rotated per asset, and stored in a KMS?
- For premium content, is HDCP (or equivalent output protection) required, with fail-closed behavior when it is unavailable?
- If watermarking is used, is it understood as an attribution tool for high-value content, with a clear plan for embedding and extraction, rather than as a substitute for DRM?
- Does the system fail closed (block playback) when DRM cannot establish a trusted path, with no fail-open fallback to unencrypted content?
- Are DRM failures (which step, which device, which asset) observable and distinguishable between legitimate support issues and attack patterns?
- Are region and device/concurrency restrictions enforced at the license server, not only at the application level?
