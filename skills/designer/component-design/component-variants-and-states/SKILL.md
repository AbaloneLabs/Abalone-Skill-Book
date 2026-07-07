---
name: component_variants_and_states.md
description: Use when the agent is designing a component's visual and interactive variants, sizes, tones, emphasis levels, and all interactive states including default, hover, focus, active, disabled, selected, loading, error, and readonly, or building a variant matrix for a design system.
---

# Component Variants And States

A component is never a single drawing. It is a matrix: variants crossed with sizes crossed with tones, each combination existing across a set of interactive states. A button is not one thing but dozens, and the quality of a design system shows in whether every cell of that matrix is intentional and coherent, or whether the default and hover states were designed and the rest were guessed at by engineers under deadline. Variants and states are where systems feel polished or fragile, and where the gaps, usually invisible in a single screenshot, surface in real use.

Use this skill before finalizing any component that has more than one appearance or that responds to interaction, and before building or auditing a variant matrix. The goal is to prevent the agent from designing only the default state, from leaving states undefined so they are implemented inconsistently, or from creating variant combinations that conflict or that no one can maintain.

## Core Rules

### Enumerate The Full State Set Before Designing Any Single State

Every interactive component has more states than designers remember to draw. The default appearance is one; the rest are where users actually spend time and where bugs cluster. Before designing, list the full set.

Typical interactive states:

- default (resting);
- hover (pointer over);
- focus (keyboard or programmatic focus);
- focus-visible (distinguishing keyboard from mouse focus);
- active or pressed (during activation);
- disabled (not interactive);
- selected or checked (for toggle and selection components);
- loading or pending (async action in progress);
- error or invalid (validation or failure);
- readonly (value shown, not editable);
- empty (no content yet).

Each state must be designed, not implied. A component with a designed default and an undefined focus state will end up with a focus state invented by whoever implements it, and it will be wrong.

### Define Variants As Small, Closed, Orthogonal Sets

Variants should be a small number of clearly distinct options, not a continuum. The goal is enough variation to serve real needs without exploding into untestable combinations.

Design variants as closed enums along orthogonal axes:

- emphasis or hierarchy: primary, secondary, tertiary, ghost;
- tone or meaning: neutral, brand, success, warning, destructive, info;
- size: small, medium, large (rarely more than three).

Keep axes orthogonal so they combine cleanly: any emphasis in any tone in any size. When axes are not orthogonal (a tone that only exists at one size, or an emphasis that implies a tone), the matrix has holes that confuse callers and break consistency.

### Make States Visible Across Every Variant And Theme

A state that is visible on the primary button but invisible on the ghost button is a defect. State changes must be perceivable in every variant, every tone, and every theme, including dark mode and high-contrast mode.

Check state visibility for:

- hover: a change large enough to notice but not jarring;
- focus and focus-visible: an indicator that does not rely on color alone and survives in dark and high-contrast themes;
- active or pressed: feedback that the activation registered;
- disabled: clearly non-interactive without looking like a loading or error state;
- selected: a persistent indicator that does not disappear when focus moves away;
- loading: a state that explains the wait without implying success or failure.

The most common failure is a hover or focus treatment that works on a filled button and vanishes on an outline or ghost variant.

### Distinguish Focus From Hover, And Keyboard From Pointer

Focus and hover are different signals for different users, and conflating them excludes keyboard users. Focus indicates where keyboard input will go; hover indicates pointer interest. A component that only styles hover and reuses it for focus leaves keyboard users with no visible focus indicator.

Design so that:

- focus has its own, persistent, visible treatment;
- focus-visible distinguishes keyboard focus (which needs a strong indicator) from mouse focus (which does not);
- hover does not substitute for focus;
- focus is not removed when hover appears, and vice versa.

Removing focus outlines to match a clean hover aesthetic is one of the most common and most damaging accessibility failures.

### Define Disabled, Loading, And Pending States Deliberately

Disabled, loading, and pending states are frequently improvised, and they carry distinct meaning that users rely on.

- Disabled: the control exists but cannot be used now. Should look non-interactive, should not be focusable if truly disabled (or should be focusable but non-operable, depending on the accessibility model), and should explain why if possible.
- Loading: an async operation is in progress. Should show progress, prevent duplicate submission, and not imply success.
- Pending: a longer-running state. Should set expectations about duration and allow cancellation where relevant.

A disabled button that looks identical to an enabled one, or a loading state that looks like success, causes confusion and duplicate actions.

### Make Selection And Toggle States Persistent And Clear

Components that can be selected, checked, or toggled must make the selected state unmistakable and persistent. A selection that only shows on hover or focus is invisible to anyone not currently pointing at it.

For selection states:

- show the selected state persistently, not only on interaction;
- use more than color to indicate selection (icon, fill, border, position);
- make group selection (radio, tabs, segmented controls) show which item is active at all times;
- preserve selection across scroll, filter, and pagination where appropriate.

### Handle Conflict And Impossibility Explicitly

Some state combinations are impossible or meaningless: a disabled button cannot be loading; a readonly field cannot show a validation error in the same way an editable one does. The system must define what happens, rather than letting implementations disagree.

Define precedence for conflicting states:

- disabled overrides hover, focus, and active (a disabled control shows no interactive feedback);
- loading may override disabled (a control mid-action is neither enabled nor disabled in the usual sense);
- error and readonly may need coexistence rules for forms;
- selected and disabled need defined behavior.

Undefined precedence produces inconsistent implementations across the system.

### Keep The Variant Matrix Maintainable

A matrix of three emphases, five tones, and three sizes, across eleven states, is hundreds of combinations. No one will draw or test all of them individually, so the system must be designed so combinations are predictable rather than bespoke.

Maintainability practices:

- derive states from tokens so they update together;
- define state deltas (what changes from default) rather than full designs per cell;
- test representative combinations rather than every cell, but document the rules so untested combinations are still predictable;
- resist adding variants that only one consumer needs;
- review the matrix regularly and retire unused variants.

A matrix that requires hand-designing every cell will fall out of sync the moment someone is in a hurry.

## Common Traps

### Designing Only The Default State

Components drawn only at rest end up with hover, focus, and disabled states improvised by implementers, and those states are inconsistent across the system.

### Variants That Are Not Orthogonal

Tones that imply an emphasis, or sizes that only exist for some variants, create holes in the matrix that confuse callers and break consistency.

### Focus That Vanishes On Some Variants

A focus or hover treatment designed for filled buttons often disappears on outline or ghost variants, leaving those variants without visible state.

### Hover Replacing Focus

Styling only hover and applying it to focus leaves keyboard users with no perceivable focus indicator, a serious accessibility failure.

### Disabled That Looks Enabled

A disabled state too similar to the default causes users to click repeatedly, believe the product is broken, and submit duplicate actions.

### Loading That Looks Like Success

A loading indicator that resembles a success checkmark leads users to navigate away before the action completes, losing their work.

### Selection That Only Shows On Hover

A selected state visible only while pointing is invisible to keyboard, touch, and review contexts and defeats the purpose of selection.

### Undefined State Precedence

When disabled, loading, error, and selected states can co-occur and the system does not define precedence, each implementation resolves it differently.

### Bespoke Cells In A Large Matrix

Hand-designing every variant-state combination guarantees the matrix falls out of sync; the system must derive combinations from rules and tokens.

## Self-Check

- [ ] The full interactive state set (default, hover, focus, focus-visible, active, disabled, selected, loading, error, readonly, empty) is enumerated and each state is designed, not implied.
- [ ] Variants are small, closed, orthogonal sets along emphasis, tone, and size axes, with no holes where combinations do not exist.
- [ ] State changes are visible across every variant, tone, and theme, including dark and high-contrast modes, and do not rely on color alone.
- [ ] Focus has its own persistent visible treatment distinct from hover, and focus-visible distinguishes keyboard from pointer focus.
- [ ] Disabled, loading, and pending states are distinct, non-misleading, and communicate why the control is in that state where possible.
- [ ] Selection and toggle states are persistent, use more than color, and remain visible when focus moves away.
- [ ] Precedence for conflicting or impossible state combinations (disabled versus loading, error versus readonly, selected versus disabled) is explicitly defined.
- [ ] The variant matrix is maintainable through token-driven state deltas and documented rules, not bespoke per-cell design.
- [ ] Representative combinations were tested, and unused variants are retired to keep the matrix coherent.
- [ ] The component was reviewed in dark mode, high-contrast mode, and keyboard-only interaction, not only the default screenshot.
