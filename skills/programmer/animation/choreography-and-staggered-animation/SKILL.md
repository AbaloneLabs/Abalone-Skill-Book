---
name: choreography_and_staggered_animation.md
description: Use when the agent is orchestrating motion across multiple elements at once — staggered list entrances, sequenced card or panel reveals, coordinating related elements that should move together, spatial choreography for page or view transitions, deciding whether to stagger items or animate them as a group, setting per-item delays and rhythm across a list, building directional reveals, or reviewing a UI for visual noise caused by everything animating simultaneously. Also covers the trap of animating all elements in parallel causing a chaotic flash, choreography that fails to guide attention, stagger timing that is too long for large lists, choreography that breaks under reduced-motion or re-render, and the failure mode of motion that looks impressive in a demo but disorients the repeat user.
---

# Choreography And Staggered Animation

Choreography is the judgment of how multiple elements animate in relation to each other over time. When a single element moves, the question is how *it* should feel; when many move, the question is how the *relationship* between them should read — what enters first, what follows, what moves together, and where the eye should land. Done well, choreography guides attention: the user perceives a hierarchy of motion that tells them what is primary. Done poorly, it produces a wall of simultaneous motion — a flash, or a long cascade the user must wait through — that communicates nothing and fatigues on repeat.

Agents fail in two opposite directions. The first is animating everything at once: every child fades or slides in parallel, producing an undifferentiated burst that reads as a flicker. The second is over-staggering: every item gets its own delay, so a ten-item list takes a full second to reveal and the user waits for content they could already read. Both miss the point — choreography is an attention-design problem, not a timing trick. The goal is to use sequence, grouping, and direction to tell the user what matters, within a total duration that respects the repeat user.

## Core Rules

### Decide Whether Elements Animate Together, In Sequence, Or By Group

The first decision is the relationship between elements. Three legitimate patterns exist, chosen by what the motion must communicate:

- **Animate together (parallel) when elements are a single unit.** A card's image, title, and body form one object and should enter as one object, sharing a single transition. Animating each part of a single card in sequence fragments a unit and reads as fussy.
- **Animate in sequence (staggered) when elements are a list of peers and order carries meaning.** Search results, a feed, a list of notifications. A short stagger helps the eye track the hierarchy (the first result leads).
- **Animate by group when there are natural clusters.** A header, a list, and a footer are three groups; within each, elements move together, and the groups sequence. Grouping keeps motion legible without per-item delay sprawl.

The common failure is animating every individual element with its own delay regardless of whether it is a unit, a peer, or part of a group. Ask, for each element: is this a single object with the others, one of many peers, or a member of a cluster? The answer determines the pattern.

### Keep Total Duration Short And Optimize For The Repeat User

Optimize for the user who sees it repeatedly, not the viewer who sees it once. A staggered reveal that delights on first load and annoys on the fiftieth page view is too long.

- **Total time for a staggered list to resolve should stay within the transition range (200–400ms), up to ~500ms for large structural reveals.** A ten-item list with a 100ms stagger per item takes a full second — too long for content the user wants to read now.
- **Use a short, fixed per-item increment (often 30–60ms).** For long lists, cap the staggered items (animate the first N, reveal the rest immediately) so the tail does not drag.
- **Question any choreography whose total duration would annoy a user who triggers it many times a day.** If it would, shorten the delay, reduce the staggered items, or animate the group as a unit.

The "demo effect" — a cascading reveal that looks stunning in a portfolio — is the most common duration failure. Ship shorter than feels impressive.

### Use Choreography To Guide Attention, Not To Show Everything Moved

Choreography is an attention-design tool. A strong choreography makes the primary element lead and the secondary follow; a weak one animates everything equally, so nothing leads and the eye has nowhere to settle.

- **Lead with the element the user most needs to see.** In a modal, the title or primary action can enter a beat before secondary content; in a results view, the first result leads the list.
- **Let the most important element have the most legible motion.** A primary CTA can have a clearer entrance; supporting elements can be subtler (a fade rather than a slide). Motion intensity encodes importance.
- **Use direction to convey structure.** Elements entering from the side they relate to (a detail panel from the right, a dropdown from its trigger) reinforce the spatial model.

If, after the choreography plays, the user does not know where to look, the choreography failed — it only showed that things moved.

### Make Choreography Robust To Interruption, Re-Render, And Reduced Motion

Choreography is more fragile than single-element animation because it depends on timing across many elements, and that timing breaks under real conditions:

- **Interruption:** if the user re-triggers mid-choreography, in-flight elements must cancel or complete cleanly, not jump or leave elements stuck mid-entrance. A re-render should not replay the whole cascade each time.
- **Re-render and virtualization:** lists that recycle elements must not re-run the entrance animation on every recycle, or scrolling re-triggers the cascade endlessly. Gate entrance animation to genuine first-mount, not every DOM insertion.
- **Reduced motion:** under `prefers-reduced-motion`, replace the staggered cascade with a single instant appearance of the group. A long staggered sequence is exactly the motion reduced-motion users want removed; do not merely speed it up.
- **Layout shift:** if choreographed elements affect layout as they enter, reserve layout space before the animation so neighbors do not jump.

### Coordinate Related Elements So Shared State Animates As One

When elements are related by shared state — a selected item and its detail panel, a parent and its expanding children, tabs and their content — their motion should be coordinated, not independent. A selection that highlights the item instantly but slides the detail panel 300ms later feels disconnected. Animate the relationship, not just the parts: a highlight that travels from the selected item to the detail panel ties them together. Sequence related elements so the cause precedes the effect — the selection highlights first, then the detail panel reveals — because reversing this inverts the causal model.

## Common Traps

### Animating Everything At Once Produces A Flash

Setting every child of a container to fade or slide in with no stagger, so the whole group bursts in as one undifferentiated motion that reads as a flicker rather than a structured reveal. A single unit may animate together, but a peer list needs a short stagger to be legible.

### Over-Staggering A Long List So The User Waits

Applying a per-item delay to every item in a long list, so a twenty-item feed takes over a second to reveal and the user waits for content they could already read. Cap the staggered items so the tail reveals immediately.

### The Demo-Effect Cascade

A long, elaborate cascading reveal that looks stunning in a portfolio and feels agonizing at the hundredth page load. If the choreography would annoy someone who triggers it many times a day, it is too long or too elaborate.

### Re-Triggering Entrance Animation On Every Re-Render Or Virtualization

Wiring the entrance animation to mount such that a virtualized list re-runs the stagger every time an element recycles into view, so scrolling produces an endless flickering cascade. Gate entrance animation to genuine first-mount, not every DOM insertion.

### Choreography That Does Not Guide Attention

Animating every element with equal intensity and no clear lead, so after the motion the user does not know where to look. Lead with the primary element and let motion intensity encode importance.

### Stagger That Continues Under Reduced Motion

Leaving the staggered cascade running (perhaps "sped up") under `prefers-reduced-motion`, when reduced-motion users want it removed entirely. Replace the cascade with a single instant appearance of the group under reduced motion.

### Layout Shift During Choreographed Entrance

Elements that animate height or position during entrance without reserving layout space, so neighboring content jumps as each element enters. Reserve the final layout before animating, or animate compositor-friendly properties (transform, opacity) so layout is stable.

## Self-Check

- [ ] The relationship between elements was decided deliberately — single units animate together, peer lists use a short stagger, clusters animate by group — rather than every element getting its own delay regardless of role.
- [ ] Total choreography duration stays within the transition range (~200–400ms, up to ~500ms for large structural reveals), with a short per-item increment (often 30–60ms) and staggered items capped so long lists do not drag.
- [ ] Choreography guides attention: the primary element leads, motion intensity encodes importance, and direction conveys spatial structure — verified by confirming the user knows where to look after the motion settles.
- [ ] The choreography is robust to interruption: re-triggering mid-cascade cancels or completes cleanly without stuck or jumping elements, and rapid interaction does not replay the whole cascade each time.
- [ ] Entrance animation is gated to genuine first-mount, not re-run on every re-render or virtualized recycle, so scrolling a long list does not produce an endless flickering cascade.
- [ ] Under `prefers-reduced-motion`, the staggered cascade is replaced with a single instant appearance of the group (not merely sped up), and no per-item stagger remains.
- [ ] Layout is reserved before the choreographed entrance so neighboring content does not shift, and motion runs on compositor-friendly properties (transform, opacity) where possible.
- [ ] The highest-risk cases were verified — a long list over-staggered, entrance animation re-triggering on virtualization, the cascade continuing under reduced motion, and layout shift during entrance — not only the smooth short-list demo path.
