---
name: illustration_in_product_context.md
description: Use when the agent is placing illustrations inside product surfaces such as empty states, onboarding, errors, success moments, paywalls, and loading states, deciding size, placement, emotional tone, text pairing, cultural sensitivity, and whether an illustration helps or obstructs the user's task.
---

# Illustration In Product Context

An illustration that works on a marketing page can fail inside a product surface. In marketing, illustration sets mood and invites. In product, illustration appears at moments when the user is trying to do something: when a list is empty, when a payment fails, when onboarding asks for attention, when a long process finishes. At those moments, illustration is in dialogue with a task. If it is too large, it pushes the action below the fold. If it is too playful, it trivializes an error. If it is culturally insensitive, it alienates the very user it meant to reassure. The judgment problem is placing illustration so that it supports the moment without obstructing the work.

Use this skill when designing or reviewing illustrations inside functional product surfaces. The goal is to prevent the agent from dropping illustrations into states as decoration, and instead to treat each placement as a decision about emotional support, hierarchy, and task flow.

## Core Rules

### Match Illustration Tone To The State's Emotional Reality

Every product state has an emotional reality, and the illustration must respect it. Mismatched tone is the most common failure.

State-by-tone guidance:

- **Empty states**: the user has not done anything wrong; a helpful, encouraging tone with a clear next step is appropriate.
- **Error states**: the user is frustrated or blocked; tone should be calm, clear, and reassuring, never playful or dismissive.
- **Loading states**: the user is waiting; illustration should reduce anxiety without implying the wait is entertaining.
- **Success states**: the user accomplished something; celebration is earned here, but should match the magnitude of the achievement.
- **Onboarding**: the user is learning; illustration should reduce intimidation and clarify, not perform.

A cheerful cartoon on a payment-failure screen tells the user their problem is not being taken seriously. A somber illustration on a signup success makes a small win feel heavy. Calibrate tone to the moment.

### Keep Illustration Subordinate To The Action

In product surfaces, the primary content is the message and the action, not the illustration. Illustration that dominates the layout pushes the call to action below the fold, especially on mobile.

Hierarchy rules:

- illustration should support the headline and action, not compete with them;
- size should be proportional to the surface and smaller on dense or task-focused screens;
- the call to action must remain visible and reachable without scrolling on common devices;
- on mobile, prefer compact illustrations that leave room for the message and button.

When in doubt, make the illustration smaller. A small, well-placed illustration that supports the message is better than a large one that overshadows it.

### Pair Illustration With Clear, Useful Text

An illustration in a product state almost never stands alone. It works with a headline, body text, and an action. The text must do the real communicative work; the illustration reinforces emotionally.

Ensure the text:

- states what happened in plain language;
- explains why, when that helps the user;
- offers a clear next step or action;
- does not rely on the illustration to convey critical information.

If removing the illustration would leave the user unable to understand the state, the illustration is carrying too much meaning. Critical information belongs in text, because text is accessible, translatable, and reliable in ways illustration is not.

### Respect Cultural And Human Sensitivity

Illustration in product surfaces is seen by everyone, across cultures, abilities, and contexts. Characters, gestures, metaphors, and depictions of people carry meaning that varies and can harm.

Check:

- whether depicted people represent the audience with dignity and diversity;
- whether gestures, clothing, or symbols carry unintended cultural meaning;
- whether the illustration trivializes serious situations (errors involving money, health, safety);
- whether the style otherizes or stereotypes any group.

Abstract or object-based illustration reduces cultural risk when human depiction is not necessary. When people are depicted, representation must be genuine, not tokenized.

### Design For Reuse And Consistency Across States

Product illustration appears in many states across the product. Each state is part of a system, and the illustrations should share a family so that encountering them feels coherent.

Establish:

- a consistent style family across empty, error, loading, and success states;
- a consistent sizing and placement pattern so users learn where to expect illustration;
- a consistent relationship between illustration and the surrounding text and action;
- variations that distinguish state type (for example, subtle color shifts for error versus success) without breaking the family.

A product where each state has a different illustration style feels like several products stitched together.

### Consider Performance And Loading Behavior

Illustrations in functional states often appear at moments of friction — errors, loading, empty — when the user is already impatient. Heavy illustrations that load slowly at these moments make a bad moment worse.

Prefer:

- lightweight SVG or optimized assets for state illustrations;
- inline or preloaded assets so they appear instantly when the state triggers;
- consistent file handling so the same illustration performs reliably everywhere.

A loading illustration that itself loads slowly is a particularly painful failure.

### Avoid Illustration In High-Stakes Or High-Stress Contexts

Some surfaces are too serious for illustration. Critical errors, security warnings, legal notices, medical alerts, and financial confirmations usually benefit from clarity and restraint, not from character art. In these contexts, illustration can read as trivializing or manipulative.

When stakes are high, prefer clear typography, explicit language, and direct action over decorative support. Illustration should recede where gravity is required.

## Common Traps

### Playful Illustration On Error Or Failure States

Cheerful art on a screen where the user is blocked or frustrated reads as dismissive and increases irritation.

### Oversized Illustration Pushing Action Off-Screen

Large state illustrations on mobile push the call to action below the fold, defeating the purpose of the state.

### Illustration Carrying Critical Meaning

If the user cannot understand the state without the illustration, the design fails for screen reader users, low-vision users, and anyone whose context strips the image.

### Inconsistent Style Across States

Different illustration styles in empty, error, and success states make the product feel disjointed.

### Culturally Insensitive Or Stereotyped Depiction

Characters and symbols that otherize or stereotype alienate the audiences the illustration meant to include.

### Heavy Assets On Friction Surfaces

Slow-loading illustrations on error and loading states compound the user's frustration at exactly the wrong moment.

### Illustration Where Gravity Is Required

Decorative art on security, medical, legal, or high-stakes financial surfaces undermines the seriousness the moment demands.

## Self-Check

- [ ] Illustration tone matches the emotional reality of each state: encouraging for empty, calm for error, earned for success, clarifying for onboarding.
- [ ] Illustration is subordinate to the message and action, with the call to action visible without scrolling on common devices.
- [ ] Critical information is in text, not in the illustration; removing the illustration still leaves the state understandable.
- [ ] Depicted people and symbols represent the audience with dignity, diversity, and no stereotyping or unintended cultural meaning.
- [ ] State illustrations share a consistent style family, sizing, and placement pattern across the product.
- [ ] Illustrations are lightweight, optimized, and load instantly on friction surfaces.
- [ ] Illustration is withheld or minimized on high-stakes surfaces where clarity and gravity matter more than decoration.
- [ ] Each illustration was reviewed in context, at real size, on real devices, not in isolation.
