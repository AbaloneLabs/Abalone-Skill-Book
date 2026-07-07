---
name: design_for_performance.md
description: Use when the agent is making design decisions that affect performance, including perceived load time, skeleton states, progressive loading, content prioritization, deferring non-critical UI, choosing interaction patterns by their cost, and trading visual richness against speed on low-end devices and networks.
---

# Design For Performance

Performance is a design property, not an engineering afterthought. The choices a designer makes, how much content loads at once, how many images a hero contains, whether a transition animates a list or rebuilds it, whether the first screen waits for all data or shows partial state, determine whether the product feels fast or slow long before any code is written. A design that assumes a fast machine and a fast network will fail for most of its real users.

Use this skill when designing flows, screens, and interactions where speed and responsiveness matter. The goal is to prevent the agent from producing designs that look beautiful in a prototype on a fast laptop but degrade into slow, janky, or unusable experiences on the devices, networks, and conditions where the product actually lives.

## Core Rules

### Design The Perceived Experience, Not The Technical One

Users do not measure milliseconds; they measure waiting, uncertainty, and interruption. Two implementations with the same load time can feel completely different depending on what the user sees while waiting. Design for perceived performance first.

Perceived performance techniques:

- show structure immediately with skeleton or placeholder states rather than blank screens;
- render critical content first and defer secondary content;
- use progressive loading so something is always happening;
- avoid blocking the user from acting while non-critical work continues;
- communicate progress honestly when work is genuinely long.

A blank screen for 800ms feels broken. A skeleton for the same 800ms feels responsive. The difference is design.

### Prioritize The Critical Path And Defer Everything Else

Not everything on a screen is equally important. The first frame should contain what the user came for; everything else can follow. Decide explicitly what is critical, what is secondary, and what is optional.

When prioritizing:

- identify the primary task or content the user needs immediately;
- defer below-the-fold content, secondary navigation, recommendations, and personalization;
- defer analytics, non-essential scripts, and decoration;
- sequence data so the critical request completes first;
- avoid making the first paint wait on data that is not needed for the first interaction.

A design that treats every section as equally urgent forces the implementation to load everything before showing anything.

### Account For The Real Device And Network Distribution

A design reviewed only on a fast laptop over fast wifi will be optimized for the designer, not the user. Real users are on mid-range phones, slow or intermittent networks, shared connections, and older hardware. Performance budgets must reflect the low end of the distribution, not the median of the office.

Consider:

- entry-level Android devices with limited CPU and memory;
- slow 3G or 4G connections and high latency;
- data-sensitive users on metered plans;
- devices with small batteries where background work drains power;
- environments with poor signal where requests fail and retry.

Set performance budgets against the slowest devices you intend to support, then design within them.

### Choose Interaction Patterns By Their Cost

Interactions have different performance costs, and designers often pick a pattern for its feel without considering its implementation cost. A pattern that animates a long list, blurs a background, or parallax-scrolls multiple layers can be expensive on low-end devices.

Evaluate interaction cost:

- does it animate many elements simultaneously?
- does it trigger layout reflow or repaint on every frame?
- does it depend on effects like blur, shadow, or filter that are GPU-expensive?
- does it require holding large data sets in memory?
- does it cause the main thread to block interaction?

Prefer patterns that are cheap to render and reserve expensive effects for moments where the cost is justified and the device can handle it.

### Design Loading, Empty, And Error States As First-Class

Performance-sensitive surfaces spend significant time in transitional states. Treating loading, empty, and error states as afterthoughts guarantees a poor experience exactly when performance pressure is highest.

Each state should:

- loading: show structure, set expectations, avoid fake progress that jumps;
- empty: explain the state and offer a next action, do not show a perpetual spinner;
- error: explain what happened, preserve any entered data, offer recovery;
- partial: show what succeeded while indicating what is still pending or failed.

A design that only specifies the happy, fully-loaded state leaves the most common real states undefined.

### Respect Data And Battery Constraints

Performance is not only speed. On metered data plans and battery-constrained devices, a design that loads heavy images, autoplay video, or constant background refresh is hostile. Design with resource respect as a value.

Practices:

- avoid autoplaying heavy media by default;
- prefer lightweight formats and lazy loading for images and video;
- avoid constant polling or background sync unless necessary;
- let users opt into richer experiences rather than forcing them;
- consider a data-saver or lite mode for sensitive contexts.

### Set And Track Performance Budgets

A design without a performance budget will accumulate weight until it is slow, because every addition feels small in isolation. Budgets make the cumulative cost visible.

Useful budgets:

- maximum weight of the first load;
- time to interactive for the critical path;
- number of network requests for first paint;
- animation frame rate targets on reference low-end devices;
- image and asset size ceilings per surface.

When a design exceeds budget, the answer is to revise the design, not to hope engineering optimizes around it.

## Common Traps

### Designing Only The Fully-Loaded State

Specifying only the complete screen leaves loading, partial, empty, and error states to chance, which is where perceived performance lives or dies.

### Optimizing For The Designer's Machine

Reviewing performance on a fast laptop hides the experience of the majority of users on weaker hardware and networks.

### Treating Animation As Free

Animations that look smooth in a prototype can drop frames on real devices, especially when many elements move or effects like blur are involved.

### Loading Everything Before Showing Anything

Waiting for all data and assets before first paint feels slower than showing partial content immediately, even when total time is identical.

### Ignoring Retry And Failure Costs

On flaky networks, failed requests retry, multiplying cost. Designs that assume requests succeed once underestimate real load.

### Confusing Skeletons With Progress

A skeleton that sits static with no sense of progress can feel longer than an honest loading indicator. Skeletons should imply movement or be paired with honest feedback.

## Self-Check

- [ ] Loading, skeleton, empty, partial, and error states are designed, not left to implementation.
- [ ] The critical path is identified, and secondary and optional content is deferred.
- [ ] Performance budgets are set against the low end of the intended device and network distribution.
- [ ] Interaction patterns were evaluated for render and main-thread cost, not only visual feel.
- [ ] Heavy media, autoplay, and background refresh respect data and battery constraints.
- [ ] The first frame shows useful structure or content rather than a blank screen.
- [ ] Animations were considered against frame-rate targets on reference low-end devices.
- [ ] The design was reviewed under slow network and low-end device conditions, not only on a fast laptop.
