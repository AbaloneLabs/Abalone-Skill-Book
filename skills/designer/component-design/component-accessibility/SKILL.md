---
name: component_accessibility.md
description: Use when the agent is designing or reviewing a component for accessibility, including semantics, roles, ARIA, keyboard interaction, focus management, naming, state communication, screen reader behavior, and platform assistive technology support for buttons, menus, dialogs, tabs, forms, and custom widgets.
---

# Component Accessibility

Component-level accessibility is different from page-level accessibility. A page can have correct contrast and readable text and still be unusable if its components lie to assistive technology: a div that looks like a button but is not announced as one, a custom dropdown that cannot be operated by keyboard, a modal that traps focus incorrectly, or a tab list whose state is invisible to a screen reader. Components are where semantics live or die, and once a component is shared across a system, its accessibility defects multiply into every surface that uses it.

Use this skill before finalizing any interactive component, custom widget, form control, or composite pattern, and before auditing why a component behaves incorrectly with keyboard or screen readers. The goal is to prevent the agent from shipping components that look correct visually but communicate the wrong role, name, or state to assistive technology, or that cannot be operated without a mouse.

## Core Rules

### Choose Native Semantics Before Custom ARIA

The strongest accessibility foundation is the native element whose role and behavior the browser already implements correctly. A real `<button>` is focusable, announces as a button, responds to Enter and Space, and works with every assistive technology for free. Recreating it with a `<div>` plus ARIA re-implements, often incorrectly, what the platform already provides.

Prefer native elements when they fit:

- `<button>` for actions, `<a>` for navigation;
- `<input>` types for form controls;
- `<select>`, `<textarea>`, `<fieldset>`, `<legend>` for forms;
- `<dialog>`, `<details>`, `<summary>` where supported.

Reach for custom widgets and ARIA only when no native element fits, and even then, follow an established ARIA pattern rather than inventing one. Custom semantics are a maintenance burden and an assistive-tech compatibility risk.

### Provide An Accessible Name For Every Interactive Element

Every interactive element needs a name that assistive technology can announce. A button containing only an icon, a tab with only an image, or a menu item with only a symbol will be announced as "button" with no context, which is useless.

Provide names through:

- visible text content (the strongest option, helping everyone);
- `aria-label` when there is no visible text (icon-only buttons, close controls);
- `aria-labelledby` when a name exists elsewhere on the page;
- `alt` text for image-based controls;
- `title` only as a supplement, never as the sole name.

The name should describe the action or purpose ("Close dialog", "Search", "Delete row 3"), not the appearance ("X icon"). Update dynamic names when context changes, such as expanding or collapsing a section.

### Communicate State, Not Just Appearance

Visual state changes must be reflected in the semantics, or screen reader users will not perceive them. A toggle that looks pressed, a menu that looks open, a tab that looks selected, and a field that shows an error must all announce those states.

Map visual state to semantics:

- pressed toggle: `aria-pressed`;
- expanded disclosure: `aria-expanded`;
- selected tab or option: `aria-selected`;
- open menu or listbox: `aria-expanded` on the trigger;
- invalid field: `aria-invalid` plus an associated error message;
- disabled control: the native `disabled` or `aria-disabled`;
- busy or loading: `aria-busy`.

If a state is worth showing visually, it is worth exposing programmatically. Color and position alone never reach a screen reader.

### Implement Expected Keyboard Behavior For Each Pattern

Each common widget has an expected keyboard interaction that assistive technology users rely on. Deviating from it makes the component feel broken even when it is operable. Follow the established patterns.

Expected behavior by pattern:

- button: Enter and Space activate; focusable via Tab;
- link: Enter activates;
- menu or menu button: arrow keys move, Enter or Space selects, Escape closes;
- tabs: arrow keys move between tabs, Tab moves into the associated panel;
- dialog: focus moves into the dialog, Tab cycles within, Escape closes, focus returns to the trigger;
- combobox or autocomplete: arrow keys move through options, Enter selects, Escape clears;
- listbox: arrow keys move, Enter or Space selects, type-ahead supported;
- disclosure or accordion: Enter or Space toggles, optionally arrow keys move.

Inventing novel keyboard behavior forces users to learn a new model and breaks the expectations their assistive technology has taught them.

### Manage Focus Deliberately For Composite Widgets

Components that change what is on screen, dialogs, menus, tabs, single-page navigation, and dynamic inserts must manage where focus goes. Unmanaged focus leaves screen reader users stranded or sends them to the top of the page.

Focus management rules:

- when a dialog opens, move focus to the dialog (typically the first focusable element or the heading);
- when a dialog closes, return focus to the element that opened it;
- when content is inserted or removed, move focus to the new content if it is the primary result of an action;
- when a menu opens, move focus to the first item or the trigger as appropriate;
- never move focus unexpectedly as a side effect of scrolling, hovering, or loading;
- preserve focus within a modal until it is dismissed (focus trap).

Focus movement should be intentional and tied to user action, not to render cycles or data arrival.

### Keep The Accessibility Tree And The Visual Tree Aligned

A common defect is a mismatch between what sighted users see and what the accessibility tree exposes: a visually hidden element that is still focusable, a removed element that is still in the tree, or a decorative element that is announced. These mismatches confuse assistive technology users.

Keep aligned:

- hide decorative elements with `aria-hidden` or empty `alt`;
- remove disabled or non-existent options from the focus order, or handle them deliberately;
- update the tree when content changes dynamically, especially in virtualized lists;
- avoid announcing transient or redundant information;
- ensure live regions announce changes that matter and stay silent about changes that do not.

### Design Forms So Labels, Errors, And Help Are Programmatically Associated

Form accessibility fails most often through association, not contrast. A visible label that is not programmatically linked to its field, an error message that is not associated, or help text that is not connected leaves screen reader users guessing.

Ensure:

- every field has a programmatically associated label (`<label>`, `aria-labelledby`, or `aria-label`);
- required fields are indicated with `required` or `aria-required`, not only an asterisk;
- error messages are linked with `aria-describedby` and the field marked `aria-invalid`;
- help and constraint text is associated so it is announced with the field;
- instructions that apply to the whole form are present and discoverable.

Placeholder text is not a label and disappears on input; it cannot carry the essential association.

### Test With Keyboard And Screen Reader, Not Only Visual Review

Visual review cannot catch most component accessibility defects. A component that looks correct can still be unusable by keyboard or incomprehensible to a screen reader. Testing must include the modalities the component claims to support.

Test with:

- keyboard only, including Tab, Shift+Tab, arrows, Enter, Space, and Escape;
- a screen reader (NVDA, VoiceOver, TalkBack) listening to what is actually announced;
- zoom and high-contrast modes;
- the component in isolation and in realistic page context.

## Common Traps

### Divs And Spans Acting As Buttons

A clickable div lacks the button role, keyboard activation, and focusability, and it will not be announced or operated correctly by assistive technology.

### Icon-Only Controls Without Names

Buttons and links containing only icons are announced without context; without an accessible name, screen reader users hear only "button".

### Visual State With No Semantic State

A toggle that looks pressed or a tab that looks selected but exposes no `aria-pressed` or `aria-selected` is invisible to assistive technology users.

### Novel Keyboard Behavior

Inventing arrow-key or Enter behavior that differs from the established pattern forces users to relearn and breaks assistive-tech expectations.

### Unmanaged Focus On Dynamic Changes

Opening a dialog, menu, or inserted panel without moving focus leaves screen reader users stranded on the previous context.

### Labels And Errors Not Programmatically Associated

Visible labels and error text that are not linked to their fields leave screen reader users without the information they need to complete the form.

### Decorative Or Removed Elements Still In The Tree

Elements that are visually hidden or removed but still focusable or announced create phantom controls and confusing announcements.

### Visual-Only Accessibility Review

Reviewing only the appearance cannot catch keyboard, focus, or screen-reader defects; components must be tested in the modalities they claim to support.

## Self-Check

- [ ] Native semantic elements are used wherever they fit, and custom ARIA is reserved for cases no native element covers, following established patterns.
- [ ] Every interactive element has an accessible name, via visible text, `aria-label`, `aria-labelledby`, or `alt`, describing its action rather than its appearance.
- [ ] Visual state changes (pressed, expanded, selected, invalid, disabled, busy) are reflected in semantics such as `aria-pressed`, `aria-expanded`, `aria-selected`, `aria-invalid`.
- [ ] Keyboard behavior matches the established pattern for each widget type (button, menu, tabs, dialog, combobox, listbox, disclosure).
- [ ] Focus is managed deliberately when dialogs, menus, tabs, or dynamic content open and close, including focus return and focus trapping in modals.
- [ ] The accessibility tree and the visual tree are aligned: decorative elements are hidden, removed options are handled, and live regions announce only meaningful changes.
- [ ] Form fields have programmatically associated labels, required indicators, error messages, and help text, and placeholders are not used as labels.
- [ ] The component was tested with keyboard only, with a screen reader, in zoom and high-contrast modes, and in realistic page context, not only by visual review.
- [ ] Dynamic name and state updates keep pace with content changes so announcements remain accurate over the component's lifetime.
