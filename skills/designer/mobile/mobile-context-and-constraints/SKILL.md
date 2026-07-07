---
name: mobile_context_and_constraints.md
description: Use when the agent is designing for mobile context, accounting for connectivity, battery, data limits, intermittent sessions, poor lighting, one-handed use, motion, background location, notifications, offline behavior, or any of the physical, environmental, and situational constraints that shape how mobile interfaces are actually used, and must decide which constraints to design for, how to degrade gracefully, and how to avoid assuming desktop conditions on a mobile device.
---

# Mobile Context And Constraints

A mobile device is not a small desktop. It is carried into environments and situations a laptop never sees: bright sunlight, crowded trains, slow networks, low batteries, one hand, a few seconds between tasks. The judgment problem is not adding mobile features. It is recognizing that the conditions under which a mobile interface is used are hostile to the assumptions desktop design quietly relies on: stable connectivity, ample power, a seated and attentive user, and a predictable environment. Agents tend to fail by designing for the ideal mobile condition, fast wifi, full battery, two hands, quiet room, and shipping an experience that collapses the moment any of those conditions degrade, which on mobile is most of the time.

Use this skill when designing for mobile and accounting for the real-world conditions of mobile use: connectivity, power, data, environment, attention, and situational constraints. The goal is designs that degrade gracefully under realistic mobile conditions rather than breaking when the ideal is absent.

## Core Rules

### Design For Intermittent And Degraded Connectivity

Mobile connectivity is not stable. Users move between wifi and cellular, enter tunnels and elevators, sit on slow or congested networks, and hit data caps that throttle speed. A design that assumes a fast, persistent connection fails the moment the signal drops, which is routine. Design for connectivity as a spectrum, from fast to absent, and ensure the experience degrades gracefully at each level rather than simply erroring.

Handle connectivity as a gradient:

- on fast connections, load full content eagerly;
- on slow connections, prioritize essential content and defer the rest;
- on flaky connections, retry transparently and preserve partial results;
- on no connection, show cached or last-known state with clear offline indication, not a blank error.

A blank error screen on a dropped signal is a design failure; a useful offline state with queued actions is design that respects reality.

### Account For Battery And Power Constraints

Mobile devices run on batteries that deplete, and users are acutely aware of what drains them. Features that constantly track location, poll the network, run heavy animations, or keep the screen bright consume power and earn the user's resentment. Battery anxiety shapes behavior: users close apps, disable features, and avoid actions they believe are power-hungry. Design with power in mind, not as an afterthought.

Reduce power cost by:

- minimizing background location tracking and polling frequency;
- deferring non-essential network requests and sync;
- avoiding continuous heavy animation and video;
- allowing the user to opt into power-intensive features rather than defaulting them on;
- supporting dark or low-power modes where they reduce consumption.

A feature that works beautifully but drains the battery will be disabled by the user, defeating the design.

### Respect Data Limits And Cost

Many mobile users pay for data or operate under caps, and background syncs, auto-playing video, and large image loads can consume allowance silently. A design that treats data as free because it is free on the developer's office wifi shifts real cost onto the user. Be deliberate about what is loaded, when, and whether the user consented to the data use.

Consider data cost:

- avoid auto-playing video and large media by default;
- lazy-load images and defer non-critical assets;
- allow the user to control sync frequency and background data;
- offer a low-data mode for capped or roaming users;
- show data-relevant choices, such as download size, before they commit.

Silent data consumption erodes trust, especially for users on metered plans.

### Design For Brief, Interrupted Sessions

Mobile sessions are short and frequently interrupted: a notification, a stop on the train, a companion speaking. A design that assumes sustained, linear attention, like a long form with no save, a multi-step flow with no resume, or content that cannot be scanned quickly, fights the reality of how mobile is used. Build for brevity and interruption.

Build for interruption:

- save progress automatically and allow resume at the same point;
- make content scannable in seconds, with the key information first;
- allow tasks to be completed in short bursts rather than one sitting;
- preserve state across app switches and returns;
- never punish the user for leaving and coming back.

Sessions measured in seconds are the norm, not the exception.

### Account For Environmental Conditions

Mobile use happens outdoors, in motion, in bright light and in dark rooms, with one hand, while walking or talking. A design that looks great in a quiet studio may be unusable in direct sunlight where contrast collapses, illegible on a bumpy train, or unreachable while walking. Consider the environment when choosing contrast, text size, motion, and target size.

Account for environment:

- ensure sufficient contrast for bright outdoor light, not just dim studios;
- use text sizes that remain legible during motion and vibration;
- avoid relying on precise gestures that are hard while moving;
- support both very bright and very dark environments;
- do not rely on audio that is inaudible or inappropriate in public.

The environment is part of the interface; designing for the studio alone ignores where the device actually goes.

### Handle Notifications And Interruptions Deliberately

Mobile devices interrupt constantly, and the design's notifications compete with every other app for attention. Over-notifying trains the user to ignore or disable all notifications; under-notifying misses genuine value. Treat notifications as a scarce, consent-based channel, earned by relevance, not assumed as a right.

For notifications:

- request permission in context, with a clear benefit, never as a launch-time block;
- send only genuinely time-sensitive or user-requested alerts;
- allow granular control over what triggers a notification;
- group or digest non-urgent updates rather than sending them individually;
- make every notification valuable enough that the user is glad it appeared.

Notification abuse is the fastest way to get silenced or uninstalled.

### Respect The User's Situation And Attention State

Mobile users are often doing something else at the same time: walking, talking, watching, working. The interface competes for partial attention, not full attention. A design that demands focused, sequential interaction in a context where the user can only give partial attention will be abandoned. Match the required attention to the likely situation, and offer quick, low-attention paths for the common cases.

### Degrade Gracefully Rather Than Failing Opaquely

Across all these constraints, the principle is the same: when a condition degrades, the experience should degrade visibly and usefully, not fail opaquely. A slow network should show essential content with a loading indicator, not a spinner over nothing. A low battery should reduce optional work, not disable the core function. An offline state should show what is available and queue what is not. Graceful degradation keeps the user functional under real conditions; opaque failure abandons them.

## Common Traps

### Assuming Stable, Fast Connectivity

Designing for persistent broadband ignores that mobile networks drop and throttle constantly. Design for a connectivity gradient with graceful offline states.

### Ignoring Battery Cost

Power-hungry features get disabled by users, defeating the design. Minimize background work and let users opt into intensive features.

### Treating Data As Free

Auto-playing media and silent syncs shift real cost onto metered users. Make data use deliberate and user-controlled.

### Designing For Sustained Attention

Long, unsaved flows fight the reality of brief, interrupted mobile sessions. Save progress and enable short bursts of use.

### Studio Conditions Assumed

Contrast and sizing that work in a quiet studio fail in sunlight or motion. Design for the real environment.

### Notification Overuse

Sending too many notifications trains users to silence the app. Treat notifications as scarce and consent-based.

### Demanding Full Attention In Partial-Attention Contexts

Interfaces that require focused, sequential use are abandoned when the user can only give partial attention. Offer quick, low-attention paths.

### Opaque Failure On Degraded Conditions

A spinner over nothing or a hard error abandons the user. Degrade visibly and usefully when conditions worsen.

## Self-Check

- [ ] The experience degrades gracefully across a connectivity gradient, with useful offline states rather than blank errors.
- [ ] Battery-intensive behaviors are minimized, opt-in, and respect the user's power anxiety.
- [ ] Data use is deliberate and user-controlled, with no silent heavy consumption on metered plans.
- [ ] Sessions support brevity and interruption: progress is saved, content is scannable, and tasks resume where they left off.
- [ ] Contrast, text size, and gesture precision account for outdoor light, motion, and one-handed use, not only studio conditions.
- [ ] Notifications are consent-based, contextually requested, and limited to genuinely valuable or time-sensitive alerts.
- [ ] The interface offers low-attention paths for common tasks in partial-attention situations.
- [ ] Degraded conditions produce visible, useful fallbacks rather than opaque failures.
- [ ] The design assumes mobile conditions, not desktop conditions, throughout.
- [ ] Real-world constraints such as connectivity, power, data, environment, and attention were each explicitly considered rather than assumed away.
