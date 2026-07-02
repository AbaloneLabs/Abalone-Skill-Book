---
name: platform_specific_constraints.md
description: Use when the agent is building or porting a mobile application across iOS and Android, handling platform-specific constraints such as background execution limits, memory and battery management, permission models and lifecycle differences, or building a native bridge between JavaScript/cross-platform code and platform APIs. Also covers the failure mode of assuming parity between platforms, hitting OS-imposed limits that do not exist on the other platform or on the web, mishandling the activity or app lifecycle so state is lost, and the subtle correctness and review-rejection risks of native bridges.
---

# Platform Specific Constraints

Building for mobile means building for two platforms (iOS and Android) that look similar from a distance and diverge sharply up close, plus the web conventions that mobile engineers often carry over and that do not apply. The judgment problem is that each OS imposes hard constraints — background execution limits, memory pressure handling, permission gating, lifecycle semantics, power budgets — that are not advisory suggestions but enforcement: the OS will kill your process, revoke your permission, or reject your build if you violate them. A background task that runs indefinitely on Android's foreground service will be killed within seconds on iOS; a permission assumed granted on Android may require a runtime prompt and a justification on iOS; a lifecycle transition that preserves state on one platform silently drops it on the other. The discipline is to learn each platform's constraints as hard limits, to design within them rather than against them, to handle the lifecycle explicitly (because the OS will reclaim your process and your state at moments you do not control), and to treat native bridges — the seams between cross-platform and platform-specific code — as the riskiest part of the system where type, threading, and memory assumptions collide.

Agents tend to assume parity because cross-platform frameworks (React Native, Flutter) abstract over it, and the abstraction holds until it doesn't. The harm appears as features that work in development and fail on real devices under real conditions (background, low memory, locked screen), as lifecycle bugs that lose user state, as app-store rejections for permission or background-use violations, and as native-bridge crashes that are hard to reproduce and hard to debug. The judgment is to test under the conditions the OS imposes (background, memory pressure, permission denial, interrupted lifecycle), to handle lifecycle transitions as first-class state transitions, to request permissions with justification and to handle denial gracefully, and to minimize and carefully design the native-bridge surface where most cross-platform bugs live. Mobile is not a smaller web; it is a different environment with different and stricter rules.

## Core Rules

### Learn Each Platform's Constraints As Hard Limits, Not Suggestions

iOS and Android impose constraints that the OS enforces by killing processes, revoking permissions, draining battery, or rejecting builds. These are not performance guidelines; they are correctness limits. Learn them per platform and design within them.

- **Background execution limits differ sharply.** iOS allows background work only for specific categories (audio, location, VoIP, finite tasks) and kills work that exceeds its budget; Android's foreground service model is more permissive but requires a persistent notification. Do not assume a background task that works on one runs at all on the other.
- **Memory and process limits are enforced.** Mobile OSes reclaim memory aggressively; a process can be killed at any time when the OS is under pressure, especially in the background. Design for termination, not for permanence.
- **Permission models differ.** Android runtime permissions and iOS permission prompts have different gating, different rationales, and different denial behaviors; a permission assumed available may require a runtime request with a justification, and denial must be handled.
- **Power budgets affect what the OS allows.** Background work, location updates, and push handling are constrained by power budgets; excessive use gets throttled or killed.

### Handle The Lifecycle Explicitly As State Transitions

The OS controls the lifecycle, not the app. The app is foregrounded, backgrounded, terminated, and relaunched at moments the developer does not choose, and any state not explicitly persisted is lost. Treat lifecycle transitions as first-class state transitions that the app must handle correctly.

- **Persist user state across termination.** When the OS kills and relaunches the app, the user should not lose their work; persist state on backgrounding and restore on launch.
- **Handle backgrounding as a transition, not a no-op.** When the app goes to the background, pause work that the OS will kill, release resources the OS will reclaim, and save state.
- **Handle relaunch after termination.** A cold start after the OS killed the process must restore the user's prior state; treating every launch as fresh loses work.
- **Test lifecycle transitions on real devices.** Lifecycle behavior in a simulator often differs from a real device under memory pressure; test backgrounding, termination, and relaunch on hardware.

### Design Permissions With Justification And Graceful Denial

Permissions are gating: the app cannot access the capability until the user grants it, the user may deny it, and the user may revoke it later. Design the permission flow with a justification before the prompt, a graceful path when denied, and handling for later revocation.

- **Explain why before prompting.** A pre-prompt explanation of why the permission is needed increases grant rates and is expected on iOS; prompting cold often earns denial.
- **Handle denial gracefully.** When the permission is denied, the app must still function (degraded if necessary) and must not crash or loop; offer a path to settings if the user wants to grant later.
- **Handle revocation and re-prompting.** The user may revoke a permission after granting it ("don't allow" later, or settings change); re-check permission state at the point of use, not only at launch.
- **Do not gate core functionality on a permission unless necessary.** If the app is unusable without a permission, that is a design problem; prefer degraded function over a dead-end.

### Minimize And Carefully Design The Native Bridge

The native bridge — the seam between cross-platform code (JS/Dart) and platform-specific code (Swift/Kotlin) — is where type systems, threading models, and memory assumptions collide. It is the riskiest part of a cross-platform app, where serialization costs mount, where threading violations crash, and where the abstraction leaks. Minimize the bridge surface and design it carefully.

- **Minimize bridge crossings.** Each crossing serializes data and switches threads; chatty bridges destroy performance. Batch calls, keep heavy work on one side, and cross only when necessary.
- **Be explicit about threading.** UI work must run on the main thread (iOS) / UI thread (Android); background work must not. The bridge must marshal work to the correct thread, or it crashes.
- **Be explicit about types and ownership.** The bridge converts between type systems; be explicit about what crosses, who owns memory, and how large payloads are handled. Implicit conversions hide bugs.
- **Treat the bridge as a versioned API.** The cross-platform side and the native side evolve independently; version the bridge interface so a mismatch fails loudly rather than silently corrupting.

### Test Under The Conditions The OS Imposes

Mobile bugs appear under the conditions the OS imposes — background, memory pressure, permission denial, interrupted lifecycle, poor network — none of which the development environment reproduces by default. Test under these conditions deliberately.

- **Test backgrounding and termination on real devices.** Verify state is preserved across termination and that background work respects the OS budget.
- **Test under memory pressure.** Use the OS's memory pressure tools to verify the app releases resources and survives.
- **Test permission denial and revocation.** Verify the app functions when each permission is denied and when a previously granted permission is revoked.
- **Test on low-end devices and poor networks.** A flagship on fast Wi-Fi hides problems that surface on a low-end device on a flaky cellular connection; test the long tail.

## Common Traps

### Assuming Parity Between iOS And Android

Treating the platforms as identical because a cross-platform framework abstracts them, hitting a hard limit on one that does not exist on the other (background execution, permission gating, lifecycle semantics). Learn each platform's constraints as hard limits; design within them.

### Ignoring The Lifecycle, Losing User State

Not persisting state on backgrounding or handling relaunch after termination, so the OS killing the process loses the user's work. Handle lifecycle transitions as first-class state transitions; persist and restore.

### Background Work That Exceeds The OS Budget

Background tasks that run indefinitely, getting killed by the OS within seconds (iOS) or draining battery and triggering throttling. Respect each platform's background execution model; design for finite, budgeted work.

### Permissions Prompted Cold Or Unhandled Denial

Prompting for a permission without justification (earning denial) or crashing/dead-ending when denied or revoked. Explain before prompting, handle denial gracefully, and re-check at the point of use.

### Chatty Native Bridge Destroying Performance

Crossing the bridge frequently with small payloads, serializing and switching threads on every call, destroying performance. Minimize bridge crossings, batch work, and keep heavy work on one side.

### Threading Violations At The Bridge

UI work off the main thread or background work on it, crashing because the bridge did not marshal correctly. Be explicit about threading at the bridge; marshal to the correct thread.

### Testing Only On Flagships And Fast Networks

Validating only on a flagship on fast Wi-Fi, missing problems that surface on low-end devices and flaky networks. Test the long tail: low-end hardware, poor networks, memory pressure, interrupted lifecycle.

## Self-Check

- [ ] Each platform's constraints (background execution, memory/process reclamation, permission gating, power budgets) are learned as hard OS-enforced limits and designed within, not treated as advisory suggestions or assumed to match the other platform.
- [ ] The lifecycle is handled explicitly as state transitions: user state is persisted on backgrounding and restored on relaunch after termination, backgrounding releases resources and pauses killable work, and lifecycle transitions are tested on real devices under memory pressure.
- [ ] Permissions are designed with a pre-prompt justification, graceful handling of denial (degraded function, no crash or dead-end), and re-checking at the point of use to handle later revocation; core functionality is not gated on a permission unless necessary.
- [ ] The native bridge is minimized and carefully designed: crossings are batched to avoid serialization/thread-switch overhead, threading is marshaled explicitly (UI work on the main/UI thread, background work off it), types and ownership are explicit, and the bridge is a versioned interface that fails loudly on mismatch.
- [ ] The app is tested under the conditions the OS imposes: backgrounding and termination (state preserved), memory pressure (resources released, survives), permission denial and revocation (functions degraded), and low-end devices on poor networks (the long tail).
- [ ] App-store review risks are anticipated for permission use, background operation, and policy compliance, so the build is not rejected for a violation discovered at submission.
- [ ] The highest-risk cases were verified — a background task killed by the OS budget, user state lost on termination then restored, a permission denied at runtime handled gracefully, and a bridge crash under threading mismatch — not only the clean foreground-on-a-flagship path.
