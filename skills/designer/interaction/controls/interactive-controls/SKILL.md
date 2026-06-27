---
name: interactive_controls.md
description: Use when the agent is designing buttons, inputs, menus, tabs, toggles, sliders, toolbars, icon buttons, forms, or any interactive control where users must understand state, affordance, feedback, and consequences.
---

# Interactive Controls

Controls are promises. A button promises an action. A field promises input. A tab promises a view change. A toggle promises an on/off state. If a control looks unclear, behaves inconsistently, lacks feedback, or hides consequences, the user cannot act with confidence.

Use this skill before designing or reviewing forms, toolbars, settings, filters, command bars, menus, tab sets, icon buttons, upload controls, destructive actions, and multi-step interactions. The goal is to prevent the agent from drawing plausible controls without specifying state, affordance, feedback, and error behavior.

## Core Rules

### Match The Control To The Decision

Choose controls based on the kind of choice the user makes.

Use:

- buttons for clear commands;
- icon buttons for common tool actions when the icon is familiar or labeled by tooltip;
- toggles or checkboxes for binary settings;
- radio buttons or segmented controls for mutually exclusive small option sets;
- select menus for longer option sets;
- tabs for switching views at the same level;
- sliders or steppers for bounded numeric adjustment;
- text inputs for open-ended values;
- date, time, file, color, and search controls for their specific domains.

Do not use a styled rectangle with text for every interaction. Do not use a toggle for an action that triggers work immediately. Do not use tabs for filtering if the content is not actually a peer view.

### Make State Visible

Every control should have visible states where relevant:

- default;
- hover;
- focus;
- active or pressed;
- selected;
- disabled;
- loading;
- success;
- error;
- destructive or high-risk.

State should not rely on color alone. Pair color with shape, icon, label, position, or motion. Focus states are not optional; keyboard and assistive technology users need to know where they are.

### Separate Action, Selection, And Navigation

Users must be able to tell whether a control performs an action, selects an option, opens a menu, navigates to a new view, or changes the current view. Mixed signals create hesitation and mistakes.

Examples:

- "Save" is an action.
- "Billing" in a tab bar is view selection.
- "Open invoice" is navigation.
- "Monthly" in a segmented control is a selection.
- A chevron often implies expansion or navigation, depending on placement.

Use consistent visual treatment and placement so the user does not need to infer behavior from trial and error.

### Put Feedback Where The User Looks

After interaction, show feedback near the control or affected object. A loading spinner far away from the clicked button can look like nothing happened. A form error at the top of the page can be missed if the invalid field is lower down.

Feedback should answer:

- Did the system receive the action?
- Is work still in progress?
- Did it succeed?
- Did it fail?
- What should the user do next?
- Was anything changed, saved, deleted, sent, or charged?

Avoid silent state changes for important actions.

### Design Disabled States Carefully

Disabled controls can prevent invalid actions, but they can also confuse users when the reason is hidden. If a disabled state blocks an expected action, explain why through helper text, tooltip, validation message, or visible requirement.

Do not use disabled styling as the only way to communicate missing permissions, unmet prerequisites, or unavailable features. Sometimes an enabled control with an explanatory message is better because it teaches the requirement.

### Protect Destructive And Irreversible Actions

Deletion, cancellation, payment, publishing, permission changes, data export, account closure, and irreversible submission need clear consequences. Use hierarchy, spacing, confirmation, undo, review screens, or typed confirmation based on severity.

The design should make accidental activation unlikely without making routine safe actions unnecessarily slow. A destructive action should not sit visually equal to a primary positive action unless the context is specifically about removal.

### Make Forms Conversationally Clear

A form is a conversation about required information. It should reveal what is required, what format is expected, what happens next, and how errors can be fixed.

Check:

- labels remain visible;
- required and optional status is clear;
- examples or placeholders do not replace labels;
- validation timing is appropriate;
- errors are specific and near fields;
- related fields are grouped;
- keyboard order matches visual order;
- submission state prevents duplicate ambiguity.

### Account For Touch, Keyboard, And Assistive Use

Controls should be reachable and understandable across input methods. Touch targets need enough size and spacing. Keyboard users need focus order and visible focus. Screen reader users need labels and state. Drag interactions need alternatives when the action is important.

Do not design only for precise pointer use on a large screen.

## Common Traps

### Icon-Only Controls Without Meaning

Icons are efficient when conventional. They are risky when invented, overloaded, or ambiguous. If the icon is not universally clear in context, add a label or tooltip.

### Primary Button Inflation

If several buttons look primary, none is primary. Reserve the strongest treatment for the most important safe action in the current context.

### Hidden Validation

Users should not have to submit a form repeatedly to discover basic requirements. Use inline guidance where it prevents predictable errors.

### Loading Without Preservation

When a control enters loading state, preserve size and context. If the button text changes and shifts layout, or if the control disappears, users may click again or lose confidence.

### Ambiguous Toggles

A toggle should make it clear whether the label describes the current state or the action that will happen. "Email notifications" with an on/off switch is clearer than "Disable emails" with a switch.

### Controls That Move Under The Pointer

Layout shifts during hover, loading, validation, or dynamic content can cause misclicks. Stable dimensions matter for professional interfaces.

## Self-Check

- [ ] Each control type matches the user's decision: command, binary setting, exclusive choice, view switch, navigation, text entry, or numeric adjustment.
- [ ] Default, hover, focus, active, selected, disabled, loading, success, error, and destructive states are designed where relevant.
- [ ] Users can distinguish actions, selections, navigation, expansion, and view changes.
- [ ] Feedback appears near the control or affected object and explains progress, success, failure, and next steps.
- [ ] Disabled states explain why the action is unavailable when the reason is not obvious.
- [ ] Destructive or irreversible actions have appropriate separation, confirmation, undo, or review behavior.
- [ ] Form labels, requirements, examples, validation, errors, grouping, and submission state are clear.
- [ ] Touch targets, keyboard focus, screen reader labels, and non-drag alternatives were considered.
- [ ] Icon-only controls are familiar in context or supported by labels/tooltips.
- [ ] Control dimensions remain stable across hover, loading, validation, and dynamic content.
