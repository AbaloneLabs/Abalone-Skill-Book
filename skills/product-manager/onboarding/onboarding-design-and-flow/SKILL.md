---
name: onboarding-design-and-flow.md
description: Use when the agent is designing onboarding flows, deciding what to show new users, structuring first-run experience, choosing between guided tours and progressive disclosure, removing friction from sign-up and setup, or shaping the path from install to first meaningful use of a product.
---

# Onboarding Design And Flow

Onboarding is where most products lose the users they spent the most effort to acquire. A user who signs up and never reaches value is a user who churns silently, and the loss is invisible in aggregate metrics until it becomes a crisis. The product manager's job in onboarding is to design the path from "I just signed up" to "I got value from this," removing everything that does not serve that path and keeping only what genuinely helps a new user succeed. Onboarding is not a feature; it is the critical first segment of the user journey, and it deserves the same rigor as any core feature.

This skill covers the judgment needed to design onboarding flows: what to include, what to cut, how to sequence, and how to avoid the common failures that turn onboarding into an obstacle course.

## Core Rules

### Design onboarding around the first moment of real value, not around feature exposure

The goal of onboarding is to get the user to value, not to teach them about the product. These are different goals, and confusing them produces onboarding that walks users through features they do not yet care about, losing them before they reach the thing that would have made them stay.

- Identify the single moment or action that first delivers real value for a new user. This is the destination onboarding must reach as directly as possible.
- Cut everything that does not serve reaching that moment. Setup steps, profile completion, feature tours, and education that delay the value moment are candidates for removal or deferral.
- Sequence ruthlessly: the fastest safe path to value first, everything else later. A user who reaches value will return to learn more; a user who abandons before value never will.

The test of every onboarding step is whether it moves the user closer to value or merely informs them. Informing without progressing is friction.

### Distinguish necessary friction from unnecessary friction

Some friction in onboarding is necessary: an account must be created, certain information must be provided, certain setup must occur for the product to function. Much friction, however, is unnecessary: it exists because of internal assumptions, legacy requirements, or a desire to collect data that benefits the company, not the user.

- Audit every field, step, and decision the user is asked to make. For each, ask: is this necessary for the user to reach value, or is it necessary for the company? Company-necessary friction should be minimized and deferred, not placed in the critical path.
- Defer anything that can be deferred. Profile details, preferences, integrations, and configuration can often wait until the user is invested and the context makes the request meaningful.
- Reduce, do not just relocate. Moving a step later helps, but eliminating it helps more. Question whether each step needs to exist at all.

The highest-leverage onboarding work is usually removing steps, not adding guidance.

### Match the onboarding depth to product complexity and user type

A simple consumer app and a complex B2B platform require fundamentally different onboarding, and applying one approach to both fails. Match the depth and structure to the product and the user.

- **Simple products** with a single core action need minimal onboarding: get the user to the action fast, with light guidance. Over-onboarding a simple product feels condescending and slows value.
- **Complex products** with multiple capabilities, configuration, or conceptual learning need structured onboarding, but the structure must still serve value, not coverage. Progressive disclosure, contextual help, and just-in-time guidance beat front-loaded tours.
- **Different user types** within the same product may need different paths. An expert user resists hand-holding that a novice needs. Consider role- or intent-based onboarding that adapts.

A single onboarding flow for all users and all products is almost always wrong. Design for the specific context.

### Prefer progressive and contextual guidance over front-loaded tours

The traditional onboarding tour — a sequence of tooltips shown all at once at first run — is one of the least effective and most common patterns. Users do not retain information presented out of context, before they need it, and the tour becomes an obstacle to skip past.

- **Progressive disclosure** reveals features and complexity as the user needs them, keeping the initial experience focused and adding depth over time.
- **Contextual guidance** appears at the moment of relevance: when the user encounters a feature, when they hesitate, when they reach a step that needs explanation. This is retained far better than front-loaded education.
- **Empty states and prompts** teach through the interface itself, guiding the user to the next action without a separate onboarding layer.

Reserve front-loaded tours for products where upfront conceptual orientation is genuinely necessary (complex B2B, multi-step setup). For most products, contextual and progressive approaches outperform.

### Remove account and setup friction aggressively

The steps between intent (signing up) and first use are where the most users are lost, and much of that loss is preventable. Common friction points include account creation requirements, email verification gates, payment collection, configuration, and integration setup.

- Reduce account creation to the minimum: often email or SSO, with progressive profiling later.
- Avoid gating first value behind email verification or payment unless legally or functionally required. Each gate loses users who would otherwise have reached value.
- Pre-configure or auto-detect settings where possible, rather than asking the user to set them. Smart defaults beat setup wizards.
- For products requiring integration or data import, offer sample data or a demo state so the user can experience value before doing the work of setup.

Every gate has a conversion cost. Include only those whose value exceeds their cost, and measure the cost rather than assuming it.

### Measure where users drop off and fix the biggest leaks

Onboarding optimization requires knowing where users are lost. Instrument the onboarding funnel so that drop-off at each step is visible, and prioritize fixing the biggest leaks rather than the most visible ones.

- Track completion of each step: sign-up, verification, first key action, value moment.
- Identify the step with the largest drop-off and investigate why. The biggest leak is rarely where intuition expects.
- Test changes and measure their effect on the funnel, not just on the step being changed. A change that improves one step's completion but harms the next is not an improvement.

Onboarding is one of the most measurable parts of the product, and the data should drive the optimization, not assumptions about what users want.

### Onboard for retention, not just for activation

A narrow focus on the activation metric (did the user perform the key action once) can produce onboarding that drives a single action at the expense of the habits that produce retention. The user who is pushed to one action and then abandoned churns slightly later than the user who never activated.

- Design onboarding to establish the habit or recurring value, not just the first action. What brings the user back the second time, the third time?
- Ensure the first value moment is representative of ongoing value, not a special case that does not recur. A dramatic first-run demo that the product cannot sustain in normal use sets up disappointment.
- Connect onboarding to the broader lifecycle. The end of onboarding is the beginning of the regular experience, and the transition should be smooth, not a cliff.

## Common Traps

### Onboarding as feature tour

Walking new users through features they do not yet care about loses them before value. Design onboarding around reaching value, and let feature education come later, in context.

### Company-necessary friction in the critical path

Signup forms full of fields that benefit the company (segmentation, marketing preferences, detailed profiles) placed before the user reaches value cause drop-off that exceeds the value of the data collected. Defer or eliminate company-necessary friction.

### Front-loaded tours that users skip

Information presented all at once, out of context, before it is needed, is not retained and is skipped past. Use progressive, contextual guidance instead.

### Over-onboarding simple products

A simple product with heavy onboarding feels condescending and slows the user from the value they could reach immediately. Match onboarding depth to product complexity.

### Optimizing one step at the expense of the whole

A change that improves completion of one step but harms subsequent steps is not an improvement. Measure the whole funnel, not just the step being changed.

### Driving activation at the expense of retention

Pushing users to a single activation action, then abandoning them, produces a one-time spike that does not sustain. Design onboarding for the habit and recurring value that drives retention.

### First-run demo that normal use cannot sustain

A dramatic first-run experience that sets expectations the product cannot meet in normal use produces disappointment and churn. The first value moment should be representative of ongoing value.

### Assuming where the drop-off is

Intuition about where users are lost in onboarding is consistently wrong. Instrument the funnel and let the data identify the biggest leaks, then fix those.

## Self-Check

- Is onboarding designed around reaching the first moment of real value, with every step evaluated against whether it progresses the user toward that moment?
- Have I distinguished necessary friction from company-necessary friction, and deferred or eliminated the latter from the critical path?
- Does the onboarding depth match the product complexity and the user type, rather than applying a one-size-fits-all flow?
- Am I using progressive, contextual guidance rather than front-loaded tours that users skip?
- Have I aggressively reduced account and setup friction, removing gates whose cost exceeds their value?
- Is the onboarding funnel instrumented so I can see where users drop off, and am I fixing the biggest measured leaks rather than assumed ones?
- Does onboarding establish the habit and recurring value that drives retention, not just a one-time activation?
- Is the first value moment representative of ongoing value, or does it set expectations normal use cannot sustain?
- Did I measure the effect of changes on the whole funnel, not just the step being changed?
- If a new user signed up today with no help, would they reach value, and if not, which step is responsible?
