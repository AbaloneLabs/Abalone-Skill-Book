---
name: device_capability_design.md
description: Use when the agent is designing for device capabilities such as camera, microphone, GPS, Bluetooth, sensors, biometrics, NFC, AR, offline storage, push notifications, or platform integrations, deciding which capabilities to require versus optionalize, handling missing or denied capabilities, requesting permissions, designing capability-dependent features, or graceful degradation when hardware or permissions are unavailable, and must avoid assuming capabilities are present, granted, and reliable.
---

# Device Capability Design

Modern devices offer capabilities a desktop never had: cameras, microphones, location, motion sensors, biometrics, Bluetooth, NFC, augmented reality. These capabilities enable powerful features, but each one introduces a chain of conditions that can fail: the hardware may be absent, the operating system may deny access, the user may revoke permission, the capability may be unreliable, or the context may make it inappropriate. The judgment problem is not deciding which cool capability to use. It is deciding whether a capability is required or optional, how to behave when it is missing or denied, how to earn the permission rather than demand it, and how to degrade gracefully so the product remains useful when the capability is unavailable. Agents tend to fail by assuming a capability is always present and granted, by gating core features behind permissions the user has no reason to grant, by requesting capabilities upfront in a block, and by failing opaquely when a capability is denied or fails at runtime.

Use this skill when designing features that depend on device capabilities, requesting permissions, handling missing or denied hardware, or planning graceful degradation. The goal is capability-dependent features that earn access, handle its absence, and never leave the user stranded because a condition was assumed rather than designed for.

## Core Rules

### Treat Every Capability As Conditional

A capability is not a fact about the device; it is a set of conditions that may or may not hold. The camera requires hardware, operating system support, and user permission, any of which can fail. Location requires a sensor, permission, and a signal, any of which can be absent. Biometrics require enrollment and permission. Treating a capability as guaranteed leads to designs that break the moment a condition fails, which on real devices is common: users deny permissions, hardware differs, signals drop, and permissions get revoked later. Design every capability-dependent feature as if the capability might be absent, because it might.

For each capability, enumerate the conditions:

- hardware presence and version support;
- operating system and browser support;
- user permission, granted, denied, or not yet asked;
- runtime reliability, such as signal, battery, or environmental factors;
- revocation, the user may remove permission later.

A feature that only works when every condition holds is fragile; one that degrades gracefully when any fails is robust.

### Distinguish Required From Optional Capabilities

Some capabilities are essential to a product's purpose: a video call app needs camera and microphone access to function. Others are optional enhancements: a document editor might use the camera for scanning but must remain useful without it. Classify each capability as required or optional, and design accordingly. Required capabilities justify a stronger permission request and an honest "this app needs X to work" message; optional capabilities must never block core functionality when absent or denied.

Classify and act:

- required: explain the dependency clearly, request at the right moment, and provide an honest path if denied;
- optional: design the feature to work without it, request only when the user engages the relevant action, and degrade silently or with a light explanation.

Optional capabilities that block core features when denied turn a graceful enhancement into a hard failure.

### Request Permissions In Context, Never In A Block

Permissions requested upfront, in a batch, at launch are refused at high rates because the user has no reason to grant them and every reason to be suspicious. Permissions requested at the moment the user engages the feature that needs them, with a clear explanation of the benefit, are granted far more often. Each permission request is a small trust transaction; make it at the moment the user can see the value, not before.

Request in context by:

- waiting until the user engages the feature that needs the capability;
- showing an in-app rationale explaining the benefit before the system prompt;
- requesting one permission at a time, not a batch;
- never blocking the core product behind permissions that are not yet needed.

A launch-time permission block converts curious users into refusals and uninstalls.

### Provide An In-App Rationale Before The System Prompt

Most platforms only let you ask for a permission once via the system prompt; after a denial, further system prompts are suppressed and the user must visit settings. This makes the moment before the system prompt critical. Use an in-app screen or message to explain what the capability enables and why it benefits the user, so that when the system prompt appears, the user has the context to say yes. If they deny, you retain the ability to guide them to settings later, because the system prompt itself is now spent.

A good rationale states:

- what the capability enables, in concrete user benefit terms;
- that permission is about to be requested;
- what happens if denied, honestly;
- how to grant it later from settings if they decline now.

Skipping the rationale wastes the single system prompt and leaves the user without context to decide.

### Design For The Denied And Revoked States

Permission denial is not an edge case; it is a common state that must be designed for, not just handled with an error. Some users deny by default, some deny by accident, and some revoke permissions later from settings. The product must remain useful, or at least functional, in the denied state, with clear guidance on what is unavailable and how to enable it. A feature that becomes a dead end when permission is denied strands the user.

Design denied states by:

- keeping core functionality working without the capability where possible;
- explaining clearly what is unavailable and why;
- offering a path to settings to grant the permission, with deep links where supported;
- re-requesting contextually if the user later engages the feature again;
- never repeating the system prompt after a denial, which is suppressed anyway.

### Handle Runtime Capability Failure

Even when a capability is present and permitted, it can fail at runtime: the camera is busy, the location signal is lost, the sensor returns garbage, the Bluetooth device disconnects. A design that assumes runtime success breaks the moment reality intrudes. Handle runtime failure with clear feedback, retry, and fallback, so that a transient failure does not become a permanent stuck state.

Handle runtime failure by:

- detecting failure and informing the user clearly, not silently;
- offering retry where the failure may be transient;
- providing a fallback path, such as manual entry when a scan fails;
- preserving user progress so a capability failure does not lose work;
- avoiding repeated automatic retries that drain battery or frustrate.

### Respect Privacy, Safety, And Appropriateness

Capabilities like camera, microphone, and location are sensitive, and their use carries privacy and safety weight. Users are rightly cautious about them, and misuse damages trust permanently. Design capability use with the user's awareness and consent: indicate when the camera or microphone is active, explain why location is needed, avoid continuous tracking when intermittent suffices, and never use a capability silently in the background without a clear user benefit. Appropriateness also matters: a feature that uses the camera in a way that feels intrusive, or location in a way that feels surveillant, will be denied and distrusted regardless of technical correctness.

### Degrade Gracefully Toward The Core Experience

The overarching principle is that capability-dependent features should layer on top of a core experience that works without them. The product must remain valuable when capabilities are absent, denied, or failing, with the capability-based features enhancing rather than enabling. This graceful degradation toward the core is what separates robust capability design from fragile design that collapses when conditions are not ideal.

## Common Traps

### Assuming Capabilities Are Always Present And Granted

Hardware, support, permission, and signal can each fail. Treat every capability as conditional and design for absence.

### Blocking Core Features Behind Optional Capabilities

An optional capability that gates core functionality turns an enhancement into a hard failure. Keep core features working without it.

### Batch Permission Requests At Launch

Upfront permission blocks are refused at high rates. Request in context, one at a time, with a rationale.

### Wasting The Single System Prompt

Asking via the system prompt without an in-app rationale leaves the user without context. Explain the benefit first.

### Dead Ends On Denial

A feature that strands the user when permission is denied, with no path to settings or fallback, abandons them. Design the denied state.

### Ignoring Runtime Failure

Capabilities fail at runtime even when permitted. Detect, inform, retry, and fall back rather than silently breaking.

### Silent Or Intrusive Capability Use

Using camera, microphone, or location without clear user awareness damages trust. Indicate active use and respect privacy.

### No Graceful Degradation To Core

A product that is useless without its capability-based features collapses under real conditions. Layer capabilities on a working core.

## Self-Check

- [ ] Every capability-dependent feature treats the capability as conditional, with conditions enumerated and handled.
- [ ] Capabilities are classified as required or optional, and optional ones never block core functionality.
- [ ] Permissions are requested in context, one at a time, at the moment the user engages the relevant feature.
- [ ] An in-app rationale explains the benefit before each system permission prompt.
- [ ] Denied and revoked states are designed, with clear guidance and a path to settings, not dead ends.
- [ ] Runtime capability failure is detected and handled with feedback, retry, and fallback.
- [ ] Sensitive capabilities such as camera, microphone, and location are used with user awareness and privacy respect.
- [ ] The product degrades gracefully toward a core experience that works without the capability.
- [ ] No capability is assumed present, granted, and reliable without evidence.
- [ ] The user is never stranded because a condition was assumed rather than designed for.
