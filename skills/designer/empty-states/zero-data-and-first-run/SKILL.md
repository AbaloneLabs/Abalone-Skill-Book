---
name: zero_data_and_first_run.md
description: Use when the agent is designing first-run experiences, onboarding for empty products, zero-data states, initial setup flows, sample data decisions, or the moment a new user encounters a product before any content exists.
---

# Zero Data And First Run

The first-run state is the most decisive moment in a product's relationship with a user. A user who arrives to an empty screen with no guidance concludes the product is broken, useless, or not for them, and most never return. First-run design is not a welcome screen; it is the bridge from "I signed up" to "I see value," and it must be designed with the same care as the core product.

Use this skill before designing or reviewing first-run experiences, onboarding flows, initial empty states, setup wizards, sample-data strategies, and any surface a new user meets before they have created or connected content. The goal is to prevent the agent from leaving new users staring at an empty product, or from overwhelming them with setup before they understand why it matters.

## Core Rules

### Get The User To Value Before Asking For Effort

The central principle of first run is to reach a moment of perceived value as fast as possible, before demanding configuration, data entry, or learning. Users who see value early are far more likely to complete setup.

Sequence for value:

- show what the product does before asking the user to build anything;
- use sample or demo data to populate the interface immediately;
- defer optional configuration until after the user understands the benefit;
- break setup into the smallest set of steps that produces a useful result;
- let the user act, even partially, before requiring a complete profile.

A first run that front-loads forms, permissions, and settings loses users before they ever see the payoff.

### Decide Between Sample Data, Guided Creation, And Empty Guidance

There are three valid approaches to a zero-data first run, and the choice depends on the product. Each has tradeoffs.

Approaches:

- sample or demo data: populate the interface with realistic content so the user can explore safely; best when the product's value is visible only with data;
- guided creation: walk the user through creating their first item with help; best when the act of creating teaches the product;
- empty with strong guidance: show a clear empty state with a prominent primary action; best when content is quick to add and exploration is intuitive.

The wrong choice, such as a blank screen for a product that needs data to make sense, guarantees abandonment.

### Make Sample Data Realistic And Safe

If you use sample data, it must be good enough to represent the real product without misleading the user or causing confusion when they add their own.

Design sample data to:

- look realistic, not obviously fake or lorem-ipsum;
- represent the common use case, not an edge case;
- be clearly marked as sample so the user knows it is not theirs;
- be easy to clear or replace without affecting real data;
- not trigger real side effects, such as sending emails or charging payment;
- demonstrate the features the user will actually use.

Sample data that looks like a toy confirms the user's fear that the product is not serious.

### Guide The First Action Explicitly

In a first run, the user does not yet know what to do. The primary action should be unambiguous, prominent, and accompanied by just enough context to motivate it.

Make the first action clear by:

- a single prominent call to action, not a menu of equal options;
- a short reason why this action matters and what it produces;
- a path that requires minimal decisions before the first result;
- inline help or a guided tour tied to the actual first step;
- visible progress so the user knows they are advancing toward value.

Multiple competing first actions paralyze new users. Lead with one.

### Minimize Friction In Initial Setup

Every field, permission, integration, and confirmation in first run is friction. Each one costs users. Setup should include only what is necessary to reach the first moment of value.

Reduce friction by:

- requesting only essential information, deferring the rest;
- using sensible defaults instead of forcing choices;
- explaining why a permission or integration is needed before asking;
- allowing skip where a step is optional;
- preserving progress so the user can leave and return without restarting;
- avoiding account or payment walls before value is visible.

Friction that is unavoidable should be justified, sequenced after value, and as light as possible.

### Teach Through Doing, Not Through Reading

New users do not read manuals; they act. First-run teaching should be embedded in the task the user is already performing, not separated into documentation.

Teach by:

- contextual hints that appear as the user reaches each step;
- inline guidance tied to the actual controls, not abstract descriptions;
- letting the user perform the real action with support, not a simulated demo;
- confirming success at each milestone so learning is reinforced;
- revealing advanced features only after basics are mastered.

A wall of onboarding text is skipped; a hint at the moment of action is read.

### Plan For Returning And Interrupted First Runs

First runs are frequently interrupted: the user closes the app, switches devices, or returns days later. The experience must resume gracefully.

Design for interruption by:

- persisting setup progress across sessions;
- recognizing returning users who never completed setup;
- not restarting onboarding from the beginning on each return;
- allowing the user to skip and explore, then return to setup;
- preserving any partial data the user already entered.

A first run that resets on every visit punishes the user for leaving.

### Connect First Run To The Real Workflow

The first run should not feel like a separate toy; it should lead directly into the real product experience. The transition from onboarding to normal use should be seamless.

Ensure continuity by:

- carrying the user's first creation into the real interface;
- not discarding sample data without warning or offering to keep it;
- maintaining consistent terminology and layout between onboarding and the product;
- marking onboarding-complete so the user is not re-prompted endlessly;
- providing a path back to help without trapping the user in a tour.

## Common Traps

### Blank Screen On First Open

A product that opens to an empty interface with no guidance tells the user nothing and most will leave. Zero data needs explicit design.

### Setup Wall Before Value

Forcing account creation, payment, profile completion, and integrations before the user sees any benefit filters out users who would otherwise stay.

### Lorem-Ipsum Sample Data

Obviously fake sample data makes the product look like a prototype and undermines confidence. Sample data should look real.

### Competing First Actions

Offering five equally weighted buttons on the first screen paralyzes new users. Lead with one clear primary action.

### Onboarding Text No One Reads

A multi-screen tutorial of pure text is skipped by most users. Teach in context, at the moment of action.

### Resetting Onboarding On Every Return

A first run that restarts from the beginning each time the user reopens the app punishes interruption and drives users away.

### Disconnected Demo And Product

A polished onboarding that leads into a bare, confusing product creates a jarring gap. The transition must be seamless.

## Self-Check

- [ ] The first run reaches a moment of perceived value before demanding significant setup, configuration, or data entry.
- [ ] A deliberate choice was made between sample data, guided creation, and empty-with-guidance, matched to the product's nature.
- [ ] Sample data, where used, is realistic, clearly marked as sample, safe from real side effects, and easy to replace.
- [ ] The first action is unambiguous, prominent, and accompanied by a short reason why it matters.
- [ ] Setup includes only essential steps, with sensible defaults, justified permissions, and skip where optional.
- [ ] Teaching happens in context through doing, not through a separate wall of onboarding text.
- [ ] Progress persists across sessions, and returning users who never completed setup are recognized and resumed.
- [ ] The first run connects seamlessly to the real product workflow without a jarring transition.
- [ ] Advanced features and optional configuration are deferred until after the user understands the basics.
- [ ] The first run was designed for the user who is impatient, interrupted, and uncertain, not only the ideal guided path.