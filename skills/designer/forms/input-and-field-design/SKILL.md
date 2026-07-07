---
name: input_and_field_design.md
description: Use when the agent is choosing input types, designing form fields, selecting controls such as text inputs, selects, checkboxes, radios, toggles, date pickers, and sliders, sizing touch targets, writing placeholders and helper text, or deciding field affordances so users enter the right value in the right format with minimal effort and error.
---

# Input And Field Design

Input design is the decision of which control a field uses, how it is sized and labeled, what it accepts, and how it signals what to do. It looks like picking a component, but it is really the decision of whether the user can enter the right value in the right format without guessing, retyping, or hitting an error they could not have foreseen. Agents tend to default to text inputs for everything, use dropdowns because they look tidy, or leave format expectations to the validation message that appears after the mistake. The harm is friction on every field: users who type a date three ways before one is accepted, who fight a dropdown that should have been a text field, or who never learn a field accepts multiple values because the control implied one.

Use this skill before choosing input types, sizing fields, writing placeholders and helper text, or deciding field affordances. The goal is to prevent the agent from selecting controls by appearance, from using placeholders where labels belong, or from leaving format and value expectations implicit until after the user has failed.

## Core Rules

### Match The Control To The Data And The Decision

Every field asks the user for a value, and the right control depends on the kind of value and how the user decides it. A mismatched control makes entry harder and errors more likely. The control should match the data type, the option set, and the user's decision process.

Match control to data:

- free text the system cannot predict: text input;
- a small fixed set of mutually exclusive options: radio buttons or segmented control;
- a small fixed set where multiple apply: checkboxes;
- a large option set the user must search: combobox or searchable select, not a long dropdown;
- a value within a range: slider or number input with bounds;
- a date: a date picker with manual typing supported, never format-only.

A fifty-item dropdown forces scrolling where a searchable combobox would let users type; radios for twelve options waste space where a select belongs.

### Prefer The Lowest-Friction Control That Prevents Error

The best control is the one that lets the user give the right answer with the least effort while making wrong answers hard. Lower friction and error prevention pull in the same direction when the control constrains input to valid values. Choose the control that narrows the input space without adding effort.

Apply this principle:

- use a toggle or checkbox for binary choices instead of yes/no text;
- use a segmented control for a few mutually exclusive options instead of a dropdown;
- use a select for a known list instead of free text that must be validated;
- constrain numeric inputs with min, max, and step instead of validating after entry;
- auto-format and mask inputs (phone, card, date) as the user types.

Free text plus post-hoc validation is the highest-friction, most error-prone pattern; constraining input at the control is usually better.

### Always Use A Label, Never A Placeholder Substitute

Labels are not optional, and placeholders are not labels. A label persists above the field so the user always knows what they are entering; a placeholder disappears the moment typing starts, leaving the user, and any reviewer, without context. Using a placeholder as a label is one of the most common and most damaging field design mistakes.

Use labels correctly:

- give every field a persistent, visible label;
- use placeholders only for examples or format hints, never as the field's name;
- keep labels short, specific, and in the user's vocabulary;
- associate labels with inputs programmatically for screen readers.

Once a user types in a placeholder-as-label field, they can no longer see what the field was, and reviewing or correcting entries becomes guesswork.

### Provide Format And Constraint Guidance Before Entry

Users should know what a field expects before they type, not after they fail validation. Format requirements, character limits, allowed values, and constraints should be visible as helper text or input affordances before entry. Surprising users with rules after the fact produces avoidable errors and rework.

Communicate expectations early:

- show required fields clearly and consistently;
- display format hints (for example date format, character limits) before entry;
- show allowed or example values where the format is non-obvious;
- use input masking and formatting to enforce format as the user types.

A field that rejects "2024-03-01" after entry because it wanted "03/01/2024" has failed the user; the format should have been shown or enforced during entry.

### Size Fields To Signal Expected Input

Field width is a cue. A field sized for a five-digit zip code tells the user to enter five digits; a field stretched wide for a two-letter state code invites overthinking. Field length should hint at the expected answer length, within the limits of the layout.

Size to signal:

- match input width roughly to expected content length where layout allows;
- use shorter fields for codes, zip, and year; wider for names, emails, addresses;
- avoid stretching all fields to a uniform width, which removes the length cue;
- keep touch targets large enough regardless of visual width.

Uniform full-width fields look tidy but discard a useful signal; varied widths communicate expected input.

### Ensure Touch Targets And Spacing Are Usable

Fields must be usable by touch and by users with motor difficulty, not just by mouse. Touch targets that are too small or too close together cause mis-taps, especially on mobile and in dense forms. Spacing between fields prevents accidental activation of the wrong control.

Meet target and spacing needs:

- keep touch targets at least the platform minimum (commonly 44x44 or 48x48 dp);
- maintain adequate spacing between adjacent interactive elements;
- ensure fields are reachable and usable at the smallest supported viewport;
- account for the virtual keyboard covering lower fields.

### Design For Defaults, Autocomplete, And Pre-Fill

Users should not type what the system already knows or can reasonably infer. Defaults, autocomplete, browser autofill, and pre-fill from context reduce effort and errors. Fields should accept and leverage these wherever the data is available or predictable.

Reduce typing by:

- pre-filling fields from account, session, or prior data where appropriate;
- supporting browser autofill with correct field semantics (autocomplete attributes);
- providing sensible defaults that most users will accept;
- offering autocomplete from a known list for repeated values.

### Handle Sensitive And High-Stakes Fields Deliberately

Some fields carry extra risk: passwords, payment details, identification numbers, and legal agreements. These need deliberate treatment beyond ordinary fields, including clear security signals, masking where appropriate, and explicit consent.

Treat sensitive fields by:

- masking values that should not be shoulder-surfed, with a reveal option;
- showing security and privacy context for sensitive data collection;
- making agreements and consent explicit, not buried in a pre-checked box;
- avoiding unnecessary collection of sensitive data at all.

## Common Traps

### Defaulting To Text Inputs For Everything

Using free text where a constrained control would prevent error forces validation and rework that a select, toggle, or slider would have avoided.

### Dropdowns For Large Or Searchable Option Sets

Long dropdowns force scrolling and hunting where a searchable combobox would let users type to find.

### Placeholders Used As Labels

Placeholders that disappear on typing leave users and reviewers without context, turning correction and review into guesswork.

### Format Rules Revealed Only After Validation

Fields that reject entries for format reasons the user could not have known produce avoidable errors and frustration.

### Uniform Full-Width Fields

Stretching all fields to one width discards the length cue that helps users gauge expected input.

### Touch Targets Too Small Or Too Close

Undersized or tightly spaced fields cause mis-taps on mobile and exclude users with motor difficulty.

### Forcing Retyping Of Known Data

Fields that ignore autofill, defaults, and pre-fill make users re-enter what the system already knows, adding effort and error risk.

### Pre-Checked Consent Or Buried Agreements

Sensitive data collection or legal consent hidden in pre-checked boxes or fine print is both a poor pattern and a compliance risk.

## Self-Check

- [ ] Each field's control matches its data type, option set, and the user's decision process, not chosen by appearance.
- [ ] The lowest-friction control that constrains input to valid values was chosen over free text plus post-hoc validation.
- [ ] Every field has a persistent visible label; placeholders are used only for examples or format hints, never as labels.
- [ ] Format, constraints, required state, and allowed values are visible before entry, with masking and formatting applied during typing.
- [ ] Field widths roughly signal expected input length, and touch targets meet platform minimums with adequate spacing.
- [ ] Defaults, autocomplete, browser autofill, and pre-fill are used to reduce typing wherever data is known or predictable.
- [ ] Sensitive fields are masked with reveal options, show security context, and require explicit consent rather than pre-checked boxes.
- [ ] Fields were tested on the smallest supported viewport with the virtual keyboard, and with assistive technology for label association.
