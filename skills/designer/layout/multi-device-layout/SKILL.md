---
name: multi_device_layout.md
description: Use when the agent is designing an interface that must work across phone, tablet, desktop, watch, TV, or other devices, deciding between responsive and adaptive strategies, handling input modality differences, reflowing content and navigation across breakpoints, or ensuring feature and content parity without forcing one device's patterns onto another.
---

# Multi-Device Layout

Designing for multiple devices is not the act of resizing one layout to fit several screens; it is the act of designing one product that must serve users who arrive with different intents, postures, input methods, attention spans, and environmental constraints on each device. A layout that works on desktop can fail on mobile not because the screen is smaller but because the user is standing, on a slow network, using a thumb, and interrupted every few seconds. The judgment problem is deciding what stays consistent across devices and what must adapt, choosing between responsive reflow and adaptive redesign, and avoiding the two opposite failures: forcing desktop patterns onto mobile, and treating each device as a separate product until they share nothing. Agents tend to fail by designing desktop-first and squeezing it down, by assuming input modality is uniform, and by chasing pixel parity instead of experience parity.

Use this skill when designing any interface that must span devices: responsive websites, apps with phone and tablet variants, cross-platform products, or experiences that include wearables and large screens. The goal is a coherent product whose layout serves each device's real conditions.

## Core Rules

### Design For Experience Parity, Not Pixel Parity

The goal across devices is not that the interface looks identical, which is usually impossible and often undesirable, but that the user can accomplish the same core tasks with equivalent ease. Pixel parity chases a screenshot; experience parity chases the outcome.

Pursue experience parity by:

- identifying the core jobs the product must support on every device;
- ensuring each job is fully completable on each device, even if the path differs;
- accepting that layout, navigation, and interaction will legitimately differ by device;
- preserving data and state so a user can move between devices without losing progress.

A user who starts a task on phone and finishes on desktop should find their work waiting, not restart from scratch.

### Start From The Most Constrained Device, Or From The Content

Designing desktop-first and scaling down tends to produce mobile layouts that feel like squeezed desktops, with hidden navigation, cramped tables, and interactions built for a mouse. Starting from the most constrained context forces honest prioritization.

Two effective approaches:

- mobile-first: design for the smallest screen and slowest network first, which forces ruthless prioritization of content and actions, then enhance upward;
- content-first: structure the content and tasks independently of device, then let each device present that structure appropriately.

Either approach beats starting from a rich desktop layout and deciding what to cut, because cutting is always done poorly under pressure.

### Account For Input Modality, Not Just Screen Size

Devices differ in how they are controlled as much as in how large they are. A layout that ignores input modality produces targets too small for touch, hover states unreachable on touchscreens, and keyboard shortcuts invisible to mouse users.

Design for input by:

- sizing touch targets for fingers, commonly at least 44 by 44 pixels, and spacing them to prevent mis-taps;
- never relying on hover to reveal critical information or actions, since touch devices have no hover;
- ensuring full keyboard navigability for desktop and accessibility;
- providing appropriate cursor and selection states for pointer and touch;
- considering voice, gamepad, and remote input where relevant.

The same component may need different sizing, spacing, and interaction depending on whether it is touched, clicked, or navigated by keyboard.

### Reflow Content And Navigation By Breakpoint

As screen width changes, content and navigation must reflow to remain usable. The decision at each breakpoint is what to stack, what to collapse, what to hide behind a toggle, and what to reveal.

Reflow deliberately:

- stack multi-column layouts into a single readable column on narrow screens;
- collapse complex navigation into patterns suited to the width, such as a bottom bar on mobile and a top bar on desktop;
- prioritize primary content and actions, demoting secondary content behind progressive disclosure;
- avoid simply hiding content that desktop shows, which breaks parity; restructure it instead.

Hiding everything behind a hamburger and calling it mobile design is a common failure. Restructure navigation to fit the device's usage patterns.

### Handle Data-Dense Views Across Devices

Tables, dashboards, and data grids are the hardest layouts to move across devices, because their value is dense comparison and that density does not survive small screens. Do not simply scale a wide table until it requires horizontal scrolling, which is a poor mobile experience.

Adapt dense views by:

- transforming wide tables into card lists on narrow screens, preserving the key fields;
- letting users choose which columns matter and reordering for priority;
- providing filter, sort, and search so users can reach specific rows without scanning all;
- using progressive disclosure to show summary first and detail on demand.

The goal is to preserve the task, comparison or lookup, not the table format. Sometimes the right mobile answer is not a table at all.

### Respect Device Posture, Attention, And Environment

Devices are used in different physical and social contexts, and layout should reflect that. A phone is used standing, in transit, in short bursts, often one-handed; a desktop is used seated, focused, for longer sessions; a TV is used at distance, with a remote, often socially.

Design for context:

- mobile: large targets, short flows, resilient to interruption, usable one-handed;
- tablet: supports both touch and longer sessions, often split attention;
- desktop: supports density, multi-tasking, keyboard efficiency, and precision;
- TV and wearable: extreme constraints on input and attention that demand focused, minimal layouts.

A layout that ignores posture produces interactions that feel wrong even when they are technically possible.

### Maintain Consistency Where It Aids The User

While layout must adapt, some consistency aids the user across devices: terminology, visual identity, core mental models, and the location of primary navigation. Preserve these so that learning transfers.

Keep consistent:

- terminology and naming, so the same concept has the same name everywhere;
- visual identity, color, type, and iconography, so the product is recognizable;
- the core mental model of how the product is organized;
- the relative location of primary navigation, even if the exact pattern differs.

Consistency here reduces relearning. Inconsistency in layout is acceptable; inconsistency in concept is not.

### Test On Real Devices, Not Just Resized Browser Windows

A layout that looks correct in a responsive preview can still fail on a real device, because real devices introduce touch latency, network variability, one-handed reach, glare, and interruption. Testing only in a browser hides these conditions.

Test by:

- running the design on actual target devices, not just emulators;
- testing on representative networks, including slow and intermittent connections;
- observing real postures, one-handed use, walking, interrupted sessions;
- checking that performance and load times are acceptable on the slowest supported device.

The browser preview is a starting point, not a validation. Real devices reveal the failures that matter.

## Common Traps

### Desktop-First And Squeeze Down

Starting rich and cutting produces cramped, hidden, inferior mobile experiences. Start constrained.

### Chasing Pixel Parity

Identical layouts across devices are often impossible and undesirable. Pursue experience and task parity.

### Ignoring Input Modality

Layouts built for a mouse fail for touch, and vice versa. Design targets, hover, and navigation for each input.

### Hiding Content Behind A Hamburger And Calling It Done

Collapsing navigation without restructuring breaks parity. Reorganize for the device's usage.

### Scaling Tables Until They Scroll Horizontally

Horizontal scrolling on mobile is a poor experience. Transform dense views to preserve the task.

### Assuming One Posture Fits All

Phone, tablet, desktop, and TV are used differently. Match layout to posture and attention.

### Testing Only In A Browser Preview

Real devices reveal touch, network, and environmental failures that previews hide. Validate on hardware.

## Self-Check

- [ ] Core tasks are completable on every supported device, even if the layout and path differ.
- [ ] The design starts from the most constrained device or from content, not from a desktop layout scaled down.
- [ ] Touch targets, hover independence, and keyboard navigability match each device's input modality.
- [ ] Content and navigation reflow deliberately at each breakpoint rather than being hidden or squeezed.
- [ ] Data-dense views are transformed for small screens to preserve the task, not horizontally scrolled.
- [ ] Layout reflects device posture, attention span, and environment, not just screen dimensions.
- [ ] Terminology, visual identity, and core mental model stay consistent to aid learning transfer.
- [ ] The design was tested on real target devices and networks, not only browser previews.
- [ ] State and data persist so users can move between devices without losing progress.
- [ ] No device is treated as a degraded version of another; each serves its real conditions.
