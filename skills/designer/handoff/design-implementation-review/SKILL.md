---
name: design_implementation_review.md
description: Use when the agent is reviewing a built implementation against a design, conducting a design QA or visual regression check, comparing staging or production to the original mockups, or deciding whether a build is ready to ship from a design-quality perspective.
---

# Design Implementation Review

A design implementation review is the moment the built product is compared against the design intent and judged ready or not ready. It is not a casual glance at staging. It is a structured comparison that catches the drift that accumulates between mockup and reality: the spacing that is off by a few pixels, the state that was never built, the interaction that feels different, the edge case that breaks the layout. These gaps are normal, because implementation involves thousands of decisions the designer never made explicitly. The review is where those decisions are checked against intent.

The judgment problem is that implementation reviews are easy to do badly. A review that only checks whether the happy path looks like the mockup will miss the states, the content variability, the accessibility gaps, and the interaction feel that determine whether the product is actually good. A review that nitpicks every pixel will exhaust the team and obscure the issues that matter. The agent's job is to review implementation systematically, separating the deviations that harm the user experience from the harmless differences, and producing clear, actionable feedback that an engineer can fix without re-litigating the design.

Use this skill before reviewing staging or production builds against designs, conducting design QA, writing implementation feedback, or signing off on ship readiness. The goal is to prevent the agent from approving builds that look right but behave wrong, or from generating review feedback that is vague, overwhelming, or focused on the wrong things.

## Core Rules

### Review Against Intent And Behavior, Not Just Pixels

The most important question is not whether the build matches the mockup pixel for pixel. It is whether the build achieves the intent the design was meant to deliver. A build can match the mockup and still fail, if the states are missing or the interaction feels wrong. A build can deviate from the mockup and still succeed, if the deviation serves the intent.

Review against intent by:

- recalling the user and business goal each screen serves;
- checking whether the primary action is clear and reachable;
- verifying the hierarchy and priority read as intended;
- confirming the behavior, not just the appearance, matches;
- distinguishing deviations that harm intent from those that are harmless.

Pixel-perfect feedback on details that do not affect the user experience wastes effort. Intent-level feedback on behavior and structure catches the issues that actually matter.

### Check Every State, Not Just The Default

Implementation drift is concentrated in the states the designer drew last or the engineer built first. The default happy path usually looks right. The loading, empty, error, disabled, and long-content states are where builds diverge.

Systematically verify:

- loading and skeleton states;
- empty and first-use states;
- error, validation, and permission states;
- disabled and read-only states;
- long-content and overflow behavior;
- success and completion states;
- state transitions and their timing.

For each, ask whether the built state matches the intended behavior, not just whether something exists. An error state that exists but is unhelpful is still a defect.

### Test With Real And Edge-Case Content

Mockups use ideal content. Production uses real names, long translations, missing images, unusual numbers, and dynamic data. A build that looks perfect with sample data can break badly with real content.

Test with:

- the longest plausible text and names;
- translations and localized formats where relevant;
- missing, loading, and failed images;
- empty and single-item lists;
- very large numbers, negative values, and unusual units;
- content that changes after load.

If the layout shifts, overlaps, truncates destructively, or hides important information under real content, the implementation is not ready, even if the happy path is pixel-perfect.

### Verify Interaction Feel, Not Just Visuals

A build can look identical to the design and still feel wrong. Interaction feel, the timing, responsiveness, feedback, and flow of using the product, is invisible in a screenshot and decisive in use.

Verify interaction by:

- clicking, tapping, and keyboard-navigating through real flows;
- checking transition timing and easing against the spec;
- confirming feedback, hover, focus, and loading, is immediate and clear;
- testing error recovery and retry flows end to end;
- checking that animations do not distract or block;
- assessing whether the product feels responsive or sluggish.

Visual review catches appearance. Interaction review catches experience. Both are required, and the second is more often skipped.

### Review Accessibility In The Built Product

Accessibility specified in design is not accessibility delivered. The build must be checked for actual accessibility, because implementation choices in markup, focus, and semantics determine whether the product is usable by everyone.

Check in the built product:

- keyboard navigation and focus order;
- visible and sufficient focus indicators;
- screen-reader labels, roles, and announcements;
- contrast in real rendering, not just in the design tool;
- touch target sizes as actually implemented;
- behavior under zoom and large-text settings;
- motion and reduced-motion behavior.

An accessible design implemented without accessibility is not accessible. The review is where this gap is caught before ship.

### Separate Blocking Issues From Polish

Not every deviation matters equally. A review that treats every pixel difference as critical overwhelms the team and delays ship for trivial reasons. A review that treats everything as polish ships real defects. Triage deliberately.

Categorize findings by:

- blocking: breaks core flows, loses data, harms users, or violates accessibility;
- important: deviates meaningfully from intent or harms experience in common cases;
- polish: minor visual or interaction refinements with low user impact.

Blocking issues must be fixed before ship. Important issues should be fixed before ship when feasible. Polish can be queued. Communicating the category with each finding helps the team prioritize without re-debating severity.

### Provide Actionable, Specific Feedback

Vague feedback, "this doesn't look right", "the spacing feels off", generates churn without progress. Engineers need to know exactly what is wrong, where, what it should be, and why.

Make feedback actionable by:

- referencing the specific screen, component, and state;
- stating the expected behavior or measurement with the token or spec source;
- showing the issue with a screenshot, recording, or steps to reproduce;
- explaining the user impact, not just the deviation;
- proposing a fix or pointing to the relevant spec.

Feedback that an engineer can act on in one pass is worth ten rounds of clarification. Treat the review feedback itself as a designed deliverable.

### Review Across The Real Range Of Devices And Contexts

A build that works on the reviewer's laptop may fail on a phone, a slow network, a right-to-left locale, or an older browser. Implementation review must cover the range of contexts the product actually supports.

Review across:

- the key breakpoints and device classes;
- slow networks and offline or degraded states;
- different browsers and operating systems where supported;
- light and dark themes;
- localized and right-to-left layouts where relevant;
- assisted-technology use.

A build that passes review only in one ideal environment is not verified for the users who will actually use it.

## Common Traps

### Reviewing Only The Happy Path

Checking the default state and approving misses the loading, empty, error, and edge states where drift concentrates. Review every state systematically.

### Pixel Nitpicking Over Intent

Exhaustive pixel-level feedback on harmless differences wastes effort and obscures the issues that affect users. Focus on intent and behavior.

### Ignoring Interaction Feel

A screenshot-perfect build can feel sluggish, unresponsive, or jarring. Use the product, do not just look at it.

### Skipping Accessibility In The Build

Accessible design is not accessible implementation. Verify focus, semantics, contrast, target size, and zoom in the real product.

### Treating All Issues As Equal

Without severity categories, the team cannot prioritize. Distinguish blocking, important, and polish in every review.

### Vague Feedback

"This looks off" generates clarification cycles. Provide specific, referenced, reproducible, fix-oriented feedback.

### Single-Environment Review

A build that works on one device may fail on others. Review across the real range of devices, networks, themes, and locales.

## Self-Check

- [ ] The review judges the build against design intent and behavior, not only against pixel-level mockup matching.
- [ ] Every state, loading, empty, error, disabled, long-content, and success, has been verified against intended behavior.
- [ ] The build has been tested with real and edge-case content, including long text, translations, missing images, and dynamic data.
- [ ] Interaction feel, timing, responsiveness, feedback, and error recovery, has been assessed through real use, not only screenshots.
- [ ] Accessibility has been verified in the built product: keyboard, focus, semantics, contrast, target size, zoom, and motion.
- [ ] Findings are categorized as blocking, important, or polish, so the team can prioritize without re-debating severity.
- [ ] Feedback is specific, referenced, reproducible, impact-explained, and fix-oriented, pointing to the relevant spec or token.
- [ ] The review covers the real range of devices, networks, browsers, themes, locales, and assisted-technology use the product supports.
- [ ] Deviations that harm intent are distinguished from harmless differences, and only the former are treated as defects.
- [ ] Ship readiness is judged on whether the built product achieves the design intent for real users, not on whether it matches the mockup.
