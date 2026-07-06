---
name: mobile_performance_and_resources.md
description: Use when the agent is optimizing a mobile application's startup time, memory footprint, network efficiency, or battery impact, supporting low-end devices, managing app size and asset strategy, or diagnosing mobile performance problems under real device and network conditions. Also covers the failure mode of optimizing only on flagship hardware and fast networks, memory spikes that trigger OS kills, battery-draining background work, an oversized binary that discourages installs, and the gap between lab benchmarks and the long tail of real devices.
---

# Mobile Performance And Resources

Mobile performance is constrained by resources that are far more limited, variable, and contested than on desktop or server: a small battery, limited memory shared with every other app, a metered or slow network, and hardware that ranges from flagship to low-end across the installed base. The judgment problem is that the development device is almost always the best hardware on the best network, so performance that feels fine in development fails for the users who matter most — those on low-end devices, poor networks, and tight data plans. A memory footprint that is comfortable on a flagship triggers OS kills on a low-end device; a startup that is instant on a fast phone takes seconds on a budget phone; a feature that streams high-resolution assets drains battery and data on a metered connection. The discipline is to define performance budgets for the long tail (not the flagship), to measure on real low-end hardware and real networks, to manage memory as a hard constraint (because the OS kills, it does not swap), to treat network and battery as user-visible costs, and to keep the app size small because size directly affects install and retention.

Agents tend to optimize for the device on their desk and the network in their office. The harm appears as crashes on low-end devices from memory pressure, as startups measured in seconds on budget hardware, as uninstall-inducing battery drain, as data-plan-hostile asset strategies, and as an app size that suppresses installs in markets where storage or bandwidth is scarce. The judgment is to set budgets for startup, memory, network, and size that target the long tail, to profile on real low-end devices and constrained networks, to minimize memory and release it proactively (because the OS will reclaim by killing), to batch and compress network traffic, to defer or avoid background work that drains battery, and to treat app size as a first-class metric. Mobile performance is measured at the long tail, not the median.

## Core Rules

### Set Performance Budgets For The Long Tail, Not The Flagship

Performance budgets (startup time, memory ceiling, network payload, app size) should target the low-end of the installed base and the constrained conditions, not the development device. A budget met on a flagship is not met; it is met when it holds on the worst device you intend to support.

- **Define startup budgets for low-end hardware.** Measure cold start on the slowest supported device; a startup that is instant on a flagship can be multiple seconds on budget hardware, and startup time directly affects retention.
- **Define memory budgets that respect OS reclamation.** The OS kills apps that exceed memory under pressure; the budget must leave headroom for the OS and for the app's own peaks, on low-end devices with less RAM.
- **Define network budgets for metered and slow connections.** A feature that loads megabytes of assets is hostile on a metered or slow connection; budget payload size and lazy-load where possible.
- **Define an app-size budget and enforce it.** App size affects install rate (especially where storage is scarce) and update cost; track it as a first-class metric and treat growth as a decision, not drift.

### Measure On Real Low-End Devices And Constrained Networks

Lab benchmarks on flagship hardware overstate real-world performance. Measure on the devices and networks your users actually have, especially the long tail.

- **Profile on the lowest-end supported device.** Performance problems that are invisible on a flagship (jank, slow startup, memory kills) appear immediately on budget hardware.
- **Test on constrained networks.** Use network conditioners to simulate slow, lossy, and metered connections; a feature that works on fast Wi-Fi may be unusable on EDGE.
- **Measure under realistic load.** A device with many apps installed, low storage, and background activity behaves differently from a clean development device; test under realistic conditions.

### Manage Memory As A Hard Constraint

Mobile memory is a hard constraint: the OS does not swap to disk, it kills. An app that exceeds its memory budget under pressure is terminated, which the user experiences as a crash and lost state. Manage memory proactively, not reactively.

- **Release memory proactively on backgrounding.** When the app goes to the background, release caches, images, and resources that can be rebuilt; the OS reclaims background apps first under pressure.
- **Watch for memory spikes, not only steady state.** A peak (loading a large image, building a data structure) is what triggers the kill, even if steady state is modest; profile and cap peaks.
- **Avoid retain cycles and leaks.** Leaks accumulate and eventually trigger kills; use the platform's memory profiling tools to find them.
- **Stream and page large data sets.** Loading an entire dataset into memory spikes the footprint; page and stream, releasing items as they scroll off-screen.

### Treat Network And Battery As User-Visible Costs

On mobile, network traffic and background work cost the user money (data plan) and battery life. Treat these as user-visible costs, not invisible engineering concerns.

- **Batch and compress network traffic.** Many small requests cost more (radio time, battery) than one batched request; compress payloads; avoid polling.
- **Lazy-load and right-size assets.** Load images and assets at the resolution needed, not the maximum; defer non-critical loads; cache to avoid re-fetching.
- **Minimize background work.** Background work drains battery and may be killed by the OS; do the minimum, batch it, and respect the OS's power budgets.
- **Prefer push over polling.** Polling keeps the radio on and drains battery; push (when the OS allows) is more efficient.

### Keep App Size Small And Justify Growth

App size affects whether users install (especially where storage is scarce) and the cost of updates. Treat size as a first-class metric and justify every increase.

- **Track app size as a metric with a budget.** Size growth is a decision, not drift; review it at release.
- **Strip unused code and assets.** Dead code, unused libraries, and unneeded asset densities inflate the binary; strip them.
- **Use app bundles and on-demand delivery.** Deliver only the code and assets needed for a device (architecture, density, language) and defer large optional features via on-demand modules.
- **Right-size assets.** Include only the image densities needed; avoid shipping max-resolution assets that get downscaled.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Optimizing Only On Flagships And Fast Networks

Measuring performance on the development device and office network, missing failures on low-end devices and constrained connections. Set budgets for the long tail; profile on the lowest-end supported hardware and constrained networks.

### Memory Spikes Triggering OS Kills

A memory footprint comfortable on a flagship that triggers OS kills on low-end devices, experienced by users as crashes. Manage memory as a hard constraint; release proactively on backgrounding; cap peaks; stream large datasets.

### Battery-Draining Background Work

Background work (polling, frequent location, excessive sync) that drains battery and gets the app killed or uninstalled. Minimize background work, batch it, prefer push over polling, and respect OS power budgets.

### Oversized Binary Suppressing Installs

An app binary bloated by unused code, libraries, and max-resolution assets, suppressing installs where storage is scarce. Track size as a metric with a budget; strip unused code and assets; use app bundles and on-demand delivery.

### Full-Resolution Asset Loading

Loading maximum-resolution images and assets that are then downscaled, wasting memory, network, and battery. Right-size assets; lazy-load; cache; include only needed densities.

### Polling Instead Of Push

Polling to detect changes, keeping the radio on and draining battery, when push would be more efficient. Prefer push over polling where the OS allows.

### Steady-State Memory Hiding Leak-Driven Kills

Steady-state memory that looks fine while leaks accumulate, eventually triggering kills after extended use. Profile for leaks and retain cycles; test extended sessions.

## Self-Check

- [ ] Performance budgets are defined for the long tail (startup, memory ceiling, network payload, app size) targeting the lowest-end supported device and constrained conditions, and a budget is considered met only when it holds on the worst supported hardware, not on the development flagship.
- [ ] Measurement happens on real low-end devices and constrained networks (slow, lossy, metered via network conditioners) under realistic load, not only on flagship hardware over fast Wi-Fi.
- [ ] Memory is managed as a hard constraint: released proactively on backgrounding, peaks profiled and capped, leaks and retain cycles found via platform tools, and large datasets streamed/paged rather than loaded whole.
- [ ] Network and battery are treated as user-visible costs: traffic is batched and compressed, assets are lazy-loaded and right-sized, background work is minimized/batched and respects OS power budgets, and push is preferred over polling.
- [ ] App size is tracked as a first-class metric with a budget, growth is a reviewed decision, unused code/assets are stripped, app bundles and on-demand delivery ship only what each device needs, and assets are right-sized to needed densities.
- [ ] The app is profiled for leaks and retain cycles and tested over extended sessions, so steady-state memory does not hide leak-driven kills.
- [ ] The highest-risk cases were verified — startup on the lowest-end device, memory behavior under OS pressure, battery drain from background work, and app size on a storage-constrained market — not only the clean flagship-on-fast-Wi-Fi path.
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
