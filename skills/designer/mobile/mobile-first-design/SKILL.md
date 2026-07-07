---
name: mobile_first_design.md
description: Use when the agent is designing mobile-first, starting from the smallest screen, prioritizing content and actions for constrained viewports, deciding what is essential versus secondary on mobile, scaling up to larger screens, or restructuring a desktop design for mobile, and must decide content priority, progressive disclosure, navigation strategy, and how to avoid treating mobile as a lesser version of desktop.
---

# Mobile First Design

Mobile-first is not a breakpoint; it is a prioritization discipline. Starting from the smallest screen forces the hardest decision early: given almost no space, what actually matters? The judgment problem is not how to make a desktop design fit a phone. It is how to identify the essential content and actions when there is room for almost nothing, how to sequence what the user sees and does, and how to scale up to larger screens by adding capability rather than subtracting clutter. Agents tend to fail by treating mobile as a constraint to endure rather than a lens that clarifies priority, by cramming desktop structure into a narrow column, by hiding primary actions behind menus to save space, and by scaling up mobile by simply stretching it, which wastes the affordances larger screens offer.

Use this skill when designing for mobile-first, restructuring desktop designs for small screens, deciding content and action priority under constraint, or scaling a design from phone to tablet to desktop. The goal is a mobile experience that is intentionally designed, not a demoted desktop, and a scaling strategy that adds value on larger screens rather than inflating them.

## Core Rules

### Use Constraint As A Prioritization Tool, Not A Punishment

The scarcity of mobile space is its greatest analytical value. When you must fit a design into a narrow viewport, you are forced to rank: which content is essential, which is supporting, which is optional, which can be deferred. This ranking is the core intellectual work of the design, and it is often avoided on desktop where everything can fit and therefore everything is shown. Do the ranking on mobile first, then carry that hierarchy to larger screens rather than letting desktop sprawl set the priority.

Rank by asking:

- what does the user need to see first to understand where they are;
- what single action is primary on this screen;
- what supports the primary action and must be visible;
- what is secondary and can be one step away;
- what is optional and can be deferred or removed.

If everything feels essential, the prioritization has not actually been done.

### Design Mobile As A Complete Experience, Not A Subset

Mobile users are not partial users doing a reduced version of a desktop task. They are full users with full goals who happen to be on a small, context-rich device. Designing mobile as "the features that fit" produces a frustrating experience where the one thing the user needs is always the thing that was cut. Identify the core tasks users perform on mobile, which may differ from desktop, and ensure each is fully completable, not just viewable.

Distinguish:

- tasks users genuinely do on mobile, which must be fully supported end to end;
- tasks users prefer on desktop, which may be viewable or lightly editable on mobile;
- tasks that are mobile-specific, such as capture, location, or quick check-ins.

A mobile experience that lets a user see a problem but not solve it is worse than one that never showed the problem.

### Choose Navigation For Mobile Realities, Not Desktop Habits

Desktop navigation, with persistent sidebars and top menus, does not translate to mobile where screen real estate is precious and the thumb governs reach. Choose mobile navigation based on the structure and frequency of destinations, not by porting the desktop nav. A small number of top-level destinations suits a bottom tab bar; a larger, deeper structure suits a drawer or hub-and-spoke model; a linear task flow may need minimal chrome at all.

Match navigation to structure:

- few peer destinations, frequent switching: bottom tab bar;
- many destinations, hierarchical browsing: drawer or menu;
- single primary task per session: minimal navigation, focus on the task;
- deep content trees: clear back path and breadcrumb-like orientation.

Hiding primary navigation to save space trades short-term tidiness for long-term discoverability problems.

### Make The Primary Action Reachable And Obvious

On a small screen with limited attention, the primary action must be both visible and reachable. Burying the main action in a menu, placing it in the hard-to-reach top corner, or surrounding it with competing options ensures many users never complete the core task. Decide the single primary action per screen and give it prominence: clear visual weight, a reachable position, and separation from secondary actions.

Consider:

- thumb reach for one-handed use, especially on large phones;
- whether a sticky primary action is warranted for task screens;
- separation of the primary action from destructive or secondary neighbors;
- whether the primary action stays visible when the keyboard or other chrome appears.

A primary action the user cannot see or reach comfortably is functionally absent.

### Scale Up By Adding Capability, Not By Stretching

The failure mode of scaling mobile up is taking the narrow column and simply widening it, leaving a sparse, under-used desktop screen. Larger screens offer affordances mobile does not: side-by-side comparison, persistent navigation, multi-column layouts, hover states, dense data views. Scaling up should add these capabilities intentionally, using the mobile priority as the spine and enriching the experience where the larger screen allows.

When scaling up:

- add columns or panels that mobile had to stack;
- reveal secondary actions and navigation that mobile had to defer;
- use hover and pointer interactions where they add value;
- increase information density where comparison or overview helps;
- preserve the mobile priority hierarchy rather than abandoning it.

The desktop experience should feel richer, not just bigger.

### Preserve The Priority Hierarchy Across Breakpoints

The prioritization done on mobile should not be discarded on larger screens. The same content and actions that were essential on mobile remain essential on desktop; larger screens simply allow more supporting and optional elements to appear. Reintroducing everything that was cut, in equal weight, recreates the clutter the mobile-first exercise was meant to prevent. Carry the hierarchy forward and let larger screens express more of it without flattening the ranking.

### Account For Mobile Context And Interruption

Mobile use happens in motion, in fragments, under distraction, and often one-handed. A mobile design that assumes sustained, seated, two-handed attention is designed for a context that rarely exists. Build for interruption: save progress, allow quick resume, make sessions short and resumable, and ensure the user can reorient quickly after being pulled away. Context is covered more fully in dedicated skills, but mobile-first design must at minimum not assume desktop-style attention.

## Common Traps

### Treating Mobile As A Lesser Desktop

Cutting features until the design fits produces a mobile experience that always lacks the one thing the user needs. Design mobile as a complete experience for mobile tasks.

### Cramming Desktop Structure Into A Column

Porting desktop sidebars and menus into a stacked mobile layout buries primary actions and creates endless scrolling. Restructure for mobile realities.

### Hiding Primary Actions To Save Space

Tucking the main action in a menu to keep the screen tidy destroys discoverability. The primary action must be visible and reachable.

### Skipping Prioritization Because Everything Feels Essential

If nothing is cut, the ranking was not done. Constraint is the tool that forces real priority decisions.

### Stretching Mobile To Fill Desktop

Widening a narrow column produces a sparse, under-used desktop screen. Scale up by adding capability, not by inflating.

### Abandoning Mobile Priority On Larger Screens

Reintroducing all cut content at equal weight recreates clutter. Preserve the hierarchy and let larger screens express more of it.

### Assuming Sustained, Seated Attention

Mobile use is interrupted and distracted. Design for short, resumable sessions and quick reorientation.

### Placing Primary Actions Out Of Thumb Reach

A main action in the top corner of a large phone is unreachable one-handed. Position primary actions within comfortable reach.

## Self-Check

- [ ] Content and actions were ranked by priority under mobile constraint, and that hierarchy drives the design.
- [ ] Core mobile tasks are fully completable end to end, not merely viewable.
- [ ] Navigation matches the mobile structure and frequency of destinations, not a ported desktop nav.
- [ ] The single primary action per screen is visible, reachable within thumb zone, and separated from competing options.
- [ ] Scaling up adds capability such as panels, comparison, and hover rather than simply stretching the mobile layout.
- [ ] The priority hierarchy established on mobile is preserved on larger screens, not flattened by reintroducing everything.
- [ ] The design accounts for interrupted, distracted, often one-handed mobile use with resumable sessions.
- [ ] Nothing essential was cut merely because it did not fit; cuts were deliberate priority decisions.
- [ ] The mobile experience feels intentionally designed, not like a demoted version of desktop.
