---
name: state_and_affordance_design.md
description: Use when the agent is designing how controls and elements communicate their current state and what they allow, including hover, focus, active, selected, disabled, loading, read-only, error, and drag states, and how affordances signal what is interactive, draggable, editable, or expandable.
---

# State And Affordance Design

A control that looks the same whether it is idle, hovered, focused, pressed, selected, disabled, or loading is a control the user cannot trust. State and affordance design is the discipline of making every element tell the user what it is right now, what it can become, and what will happen if they act on it. When state is invisible or affordance is ambiguous, users click things that do nothing, miss things that are interactive, and make mistakes they cannot trace.

Use this skill before finalizing buttons, inputs, list items, cards, toggles, tabs, menus, drag handles, and any element whose meaning changes with context. The goal is to prevent the agent from shipping a static-looking interface where state is implied by color alone, where interactive and non-interactive elements look identical, or where the user must guess whether an action is available.

## Core Rules

### Design Every Relevant State Explicitly

Do not leave states to chance or to the developer's default. For each interactive element, decide which states apply and specify each one. A complete state model usually includes default, hover, focus, active or pressed, selected, disabled, loading, and error. Some elements also need read-only, dragging, drop-target, expanded, or visited states.

A common failure is designing only the default appearance. The moment the user hovers, focuses, or waits, the element looks broken or indistinguishable from its surroundings. Specify each state as deliberately as the default, and ensure transitions between states are perceptible without being jarring.

### Make Affordance Visible Before Interaction

Affordance is the signal that something can be acted on. Users should not have to move the pointer over the entire screen to find what is clickable, draggable, or editable. Visual cues such as elevation, borders, underlines, cursor changes, handles, chevrons, and consistent component styling communicate affordance before any interaction occurs.

Distinguish clearly between:

- actionable elements, which should look pressable;
- informational elements, which should look static;
- editable elements, which should look open to input;
- draggable elements, which should reveal a handle or cursor change;
- navigational elements, which should read as links or destinations.

When an interactive element looks like decoration, users ignore it. When a static element looks interactive, users waste effort trying to act on it.

### Never Rely On Color Alone

Color is one signal, not the whole signal. Roughly one in twelve men and many users under poor lighting, on dim screens, or with high-contrast needs cannot distinguish some color differences. A selected item that differs from an unselected one only by background tint is invisible to those users and fragile in screenshot reviews.

Pair color with at least one additional cue: a checkmark, a border, a weight change, an icon, an underline, a position shift, or a label. This applies to selection, error, success, warning, active tabs, required fields, and any state where color carries the meaning.

### Make Focus A First-Class State

Focus tells keyboard and assistive technology users where they are. It is not optional and it must be visible. Removing the default focus outline without providing a stronger alternative is one of the most common and damaging accessibility failures.

Focus should be:

- clearly visible against any background, including images and colored surfaces;
- proportional to the element, not a thin line on a large control;
- present on every interactive element, including links, inputs, buttons, and custom widgets;
- maintained through state changes, so focus does not disappear during loading or after an action.

If a design removes the browser default, it must replace it with something more visible, not less.

### Communicate Disabled And Read-Only Distinctly

Disabled and read-only are different states with different meanings, and users often cannot tell them apart. Disabled means the action exists but is currently unavailable. Read-only means the value is visible but not meant to be changed. Both can frustrate users if the reason is hidden.

For disabled controls, explain why they are unavailable when the reason is not obvious, using helper text, a tooltip, or a visible requirement. For read-only values, style them so they do not invite editing, and make clear they are informational. Never use disabled styling as the only signal that a user lacks permission; an enabled control with an explanatory message often teaches more.

### Preserve Layout Stability Across States

When an element changes state, its dimensions should not shift the surrounding layout. A button that grows when loading, an input that expands when showing an error, or a card that jumps when selected causes misclicks, lost place, and a feeling of instability.

Reserve space for anticipated state changes. If an error message appears, its slot should exist from the start. If a loading spinner replaces text, the button width should stay constant. If a selection adds a border, use an inset or outline that does not change measured size.

### Show Selection And Multi-Selection Clearly

In lists, tables, grids, and boards, the user must always know what is currently selected, how many items are selected, and what the selection affects. Selection state should persist visibly, not disappear on scroll or after a delay.

For multi-selection, show a count, expose bulk actions that apply to the selection, and make it obvious how to add to or clear the selection. A selection that silently clears, or bulk actions that apply to hidden selections, causes destructive mistakes.

### Signal What Will Happen, Not Just What Is

Good affordance previews the consequence. A delete affordance should hint at removal. A drag handle should hint at movement. A link should hint at navigation. An expandable region should hint at disclosure. When affordance matches consequence, users act with confidence; when it does not, they hesitate or err.

Use icons, labels, cursor changes, and microcopy that align with the actual outcome. A chevron implies expansion; a trash icon implies deletion; an external-link icon implies leaving the current context. Mixing these signals breeds confusion.

## Common Traps

### Designing Only The Default State

Shipping the resting appearance of a control and leaving hover, focus, pressed, and error states to chance produces an interface that feels unfinished the moment the user touches it.

### Color-Only State Signals

Tinting a selected row or marking an error only with red hides the state from users who cannot perceive the color difference and from anyone reviewing a static screenshot.

### Removing Focus Outlines

Stripping the default focus ring for aesthetics, without a stronger replacement, removes the only navigation cue for keyboard and screen reader users.

### Ambiguous Disabled States

A grayed-out control with no explanation leaves users guessing whether they lack permission, missed a prerequisite, or hit a bug.

### Layout Shift On State Change

Borders, spinners, or error text that change element size cause the interface to jump, leading to misclicks and lost confidence.

### Interactive Elements That Look Static

Links styled as plain text, buttons styled as labels, or drag targets with no handle result in functionality users never discover.

### Static Elements That Look Interactive

Decorative cards, badges, or headings that resemble buttons invite pointless clicks and erode trust in real affordances.

## Self-Check

- [ ] Every interactive element has explicit designs for default, hover, focus, active, selected, disabled, loading, and error states where relevant.
- [ ] Affordance is visible before interaction, so users can tell what is clickable, editable, draggable, or navigational without trial and error.
- [ ] No state relies on color alone; each color-coded state has a secondary cue such as icon, border, weight, or label.
- [ ] Focus is clearly visible on every interactive element and survives state changes, loading, and dynamic updates.
- [ ] Disabled and read-only states are visually distinct and, where the reason is not obvious, explained.
- [ ] Element dimensions remain stable across hover, focus, loading, error, and selection so the layout does not shift.
- [ ] Selection and multi-selection are always visible, with a count and clear bulk actions where applicable.
- [ ] Affordance cues match the actual consequence of the action.
- [ ] Interactive and non-interactive elements are visually distinguishable in every state.
