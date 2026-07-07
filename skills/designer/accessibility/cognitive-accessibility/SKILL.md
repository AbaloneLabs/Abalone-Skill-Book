---
name: cognitive_accessibility.md
description: Use when the agent is designing for cognitive load during real-time interaction, managing attention and decision effort in flows, reducing working memory demands in dynamic interfaces, supporting users under time pressure or cognitive fatigue, or ensuring that moment-to-moment interaction does not exceed what a user can process while actively using the product.
---

# Cognitive Accessibility

Most cognitive accessibility guidance focuses on who the user is: people with dyslexia, ADHD, low literacy, or cognitive differences. Equally important, and more often missed, is the cognitive load created by the interaction itself in the moment of use. Even a neurotypical, well-rested expert can be pushed past their processing capacity by an interface that demands too much simultaneous attention, too much working memory, or too many decisions under time pressure. Cognitive accessibility in this sense is not about a fixed user population; it is about the load the design imposes, which rises and falls with context, fatigue, interruption, and stress. The judgment problem is designing so that the moment-to-moment cost of using the product stays within what a user can actually process, rather than assuming an idealized user who is always fresh, focused, and undistracted. Agents tend to fail by designing for the calm review state rather than the stressed use state, by stacking decisions and information in ways that exceed working memory, and by treating clarity as a static property rather than a dynamic one that degrades under load.

Use this skill when designing flows, real-time interactions, multi-step processes, dashboards, or any interface used under time pressure, interruption, or cognitive strain. The goal is interaction whose cognitive cost stays bounded even when the user is tired, rushed, or distracted.

## Core Rules

### Design For The Stressed State, Not The Review State

Designers review their work calm, focused, and unhurried, and they unconsciously assume users arrive the same way. Real users arrive distracted, tired, interrupted, and often under time pressure, and an interface that is clear in review can be opaque under stress.

Account for the stressed state by:

- imagining the user interrupted mid-flow and returned; can they resume without rebuilding context?
- testing flows under realistic distraction, not in a quiet usability lab;
- assuming reduced patience, reduced attention, and reduced working memory as the default condition;
- prioritizing the path that works when the user can give only partial attention.

An interface that survives interruption and fatigue is accessible to a far broader range of users and conditions than one optimized only for the ideal case.

### Bound The Number Of Simultaneous Decisions

Working memory is strictly limited, and interfaces that force users to hold several constraints, options, or steps in mind at once exceed that limit. The result is errors, abandonment, and choices the user later regrets.

Reduce simultaneous decisions by:

- presenting one primary decision at a time, with secondary options deferred;
- showing the relevant constraint or total next to the decision it affects, not on a prior screen;
- providing sensible defaults so not every choice is required;
- grouping related decisions and separating unrelated ones;
- allowing the user to review and change before committing, so mistakes are recoverable.

If the user must hold more than a few things in mind to proceed, the design is carrying cognitive risk. Redistribute the load across time or onto the system.

### Keep Context Visible Across Steps And Interruptions

When context lives only in the user's memory, any interruption breaks the flow. Context that the interface persists and re-displays survives distraction, navigation away, and return.

Persist context by:

- showing progress, selections, and summaries at each step;
- restating requirements near the field that enforces them, not once at the start;
- preserving entered data across errors, refreshes, and session resumes;
- making the current location in a flow obvious, so the user knows where they are and what remains.

An interface that assumes perfect, uninterrupted memory fails the moment reality intrudes. Make the interface remember what the user should not have to.

### Reduce The Cost Of Recovery, Not Just The Rate Of Errors

Errors will happen under cognitive load; the question is how expensive they are. A design that prevents some errors but makes recovery punishing shifts the cost rather than reducing it.

Lower recovery cost by:

- preserving input after errors so the user does not retype;
- returning the user to the point of failure, not the start;
- explaining errors plainly and indicating exactly what to fix;
- offering undo for reversible actions rather than demanding perfect first attempts;
- avoiding punitive language or visuals that add emotional load to an already frustrating moment.

A forgiving interface keeps cognitive load bounded even when things go wrong, because the user does not spiral into rework.

### Match Pace To The User, Not To The System

Some interactions impose a pace the user cannot control: auto-advancing carousels, expiring sessions, timeouts that discard work, or animations that lock input. These are cognitive accessibility failures, because they assume the user can always keep up.

Give the user control of pace by:

- avoiding auto-advancing content that the user cannot pause;
- setting timeouts generously and warning before they expire, with a way to extend;
- never discarding entered work due to inactivity;
- letting users control animation and motion, including a reduced-motion path;
- allowing users to dwell on complex information rather than rushing them forward.

Pace imposed by the system is a hidden tax on users who process more slowly, for any reason, in that moment.

### Separate Essential From Secondary Information

Cognitive load rises with the total information presented, not just the complexity of any single piece. An interface that shows everything at once forces the user to filter, which costs attention.

Manage information volume by:

- leading with the essential information and action;
- deferring detail behind progressive disclosure, available when needed;
- visually distinguishing primary from secondary content so the eye knows where to go;
- removing or relocating anything that does not serve the current task.

Clarity under load is not about dumbing down; it is about sequencing and emphasis, so the user processes what matters first and detail only when they choose.

### Support Multiple Paths To The Same Understanding

People process information differently, and a single presentation that works for one cognitive style will fail another. Offering more than one path reduces the chance that any single user is locked out.

Offer multiple paths by:

- pairing text with icons or illustrations for key concepts;
- providing an example alongside a rule;
- offering both a summary and expandable detail;
- supporting both search and browse navigation to the same content.

Redundancy here is accessibility, not waste. The user who cannot decode the text may grasp the icon, and vice versa.

### Keep The Mental Model Stable And Predictable

Cognitive load spikes when the rules change: a control that means one thing here and another there, navigation that moves, terminology that shifts. Users rely on a stable mental model to act without rethinking.

Preserve stability by:

- using the same control for the same action everywhere;
- keeping terminology consistent across screens;
- placing primary navigation and key actions in predictable locations;
- not changing the meaning of colors, icons, or labels between contexts.

Predictability lets the user act on habit, which is far cheaper cognitively than re-decoding each new screen.

## Common Traps

### Designing For The Calm Review State

An interface that is clear when reviewed unhurried can fail under stress. Design for distraction and fatigue.

### Stacking Decisions Beyond Working Memory

Forcing users to hold many constraints or steps in mind causes errors and abandonment. Bound simultaneous decisions.

### Relying On User Memory Across Steps

Context that lives only in memory breaks on interruption. Persist and re-display context.

### Punishing Recovery Instead Of Reducing Errors

Preventing some errors while making recovery harsh shifts cost rather than reducing it. Make recovery cheap.

### Imposing Pace The User Cannot Control

Auto-advance, tight timeouts, and locked animations assume the user always keeps up. Give control of pace.

### Showing Everything At Once

Total information volume drives load. Separate essential from secondary.

### Single-Path Presentation

Relying on one mode of presentation locks out users with different cognitive styles. Offer multiple paths.

### Unstable Mental Models

Changing rules, controls, and terminology force re-decoding and spike load. Keep the model predictable.

## Self-Check

- [ ] The design was considered under stress, interruption, fatigue, and partial attention, not only calm review.
- [ ] No flow forces the user to hold more than a few items in working memory at once.
- [ ] Context, progress, selections, and entered data persist across steps, errors, and interruptions.
- [ ] Recovery from errors preserves input and returns the user to the point of failure without punishment.
- [ ] The user controls pace: no uncontrollable auto-advance, generous timeouts, and no discarded work.
- [ ] Essential information leads and secondary detail is deferred through progressive disclosure.
- [ ] Key concepts offer multiple paths to understanding, such as text plus icon or summary plus detail.
- [ ] Controls, terminology, navigation, and meaning are stable and predictable across the product.
- [ ] Time pressure and decision effort were explicitly considered, not assumed away.
- [ ] The interaction's cognitive cost stays bounded even when the user is tired, rushed, or distracted.
