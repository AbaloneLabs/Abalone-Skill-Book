---
name: spec_and_redline_quality.md
description: Use when the agent is writing or reviewing design specifications, redline annotations, spacing and typography specs, component measurements, token definitions, or the precise measurement documentation that lets engineers build a design exactly as intended.
---

# Spec And Redline Quality

A spec or redline is the precise measurement and behavior contract between design and engineering. It says exactly how tall, how far, what color, what weight, what radius, what happens on focus, and what the value becomes when the content is twice as long. When the spec is precise and complete, the engineer builds the design once and it matches. When the spec is vague, contradictory, or missing measurements, the engineer guesses, and the result is a build that looks almost right and feels wrong.

The judgment problem is that specs look finished when they are not. A redline with measurements on every element can still be incomplete if it omits the states, the relative spacing logic, the token references, or the behavior under variable content. The agent's job is to treat a spec as an engineering contract that will be implemented literally, and to make sure that literal implementation produces the intended result, including in the cases the designer did not think to draw.

Use this skill before writing or reviewing redlines, component specs, spacing systems, typography scales, token sheets, or any measurement documentation for engineering. The goal is to prevent the agent from shipping specs that are visually dense but operationally ambiguous, forcing engineers to fill gaps with guesses that drift from the design.

## Core Rules

### Specify Relationships And Systems, Not Just Absolute Numbers

A redline that labels every gap with an absolute pixel value looks thorough but is fragile. If the engineer changes one spacing, the rest may no longer relate correctly. Strong specs express the system behind the numbers.

Express systems by:

- referencing spacing tokens or a scale, not isolated pixel values;
- defining the relationship between elements, such as inner padding versus gap between items;
- showing how spacing scales across breakpoints or sizes;
- documenting the grid, baseline, and alignment logic;
- clarifying which measurements are fixed and which are derived.

When the engineer understands the system, they can implement it consistently and adapt it correctly. When they only have isolated numbers, every new case is a new guess.

### Define Typography As A Scale, Not As Isolated Styles

Typography specs often list font sizes and weights per element without revealing the scale or the relationships. This leads to inconsistent type as the product grows. Specify the type system, not just instances.

A complete type spec includes:

- the type scale and the naming for each level;
- font family, weight, size, line height, and letter spacing per level;
- the relationship between heading and body levels;
- how type scales across breakpoints;
- behavior for long text, wrapping, and truncation;
- fallback fonts and loading behavior.

Engineers should be able to map every piece of text in the design to a scale level, not to a one-off style. This keeps the product typographically coherent and maintainable.

### Reference Tokens For Color, Spacing, Radius, And Elevation

Hardcoded values in specs create drift. The same blue specified as six different hex values across a redline guarantees inconsistency in the build. Specs should reference the token system, and the tokens should be the single source of truth.

Use tokens by:

- naming colors, spacings, radii, and elevations with semantic tokens;
- mapping every spec value to a token rather than a raw value;
- documenting token aliases for light, dark, and high-contrast themes;
- confirming tokens exist in code before referencing them;
- flagging any new token that needs to be added to the system.

When a token changes, the product updates everywhere. When raw values are scattered, nothing stays in sync. Specs are the bridge that keeps design and code on the same tokens.

### Specify Every Interactive State With Measurements

A component spec that shows only the default state omits most of what the engineer must build. Hover, focus, active, selected, disabled, loading, and error states each have their own measurements, colors, and sometimes shapes.

For each component, specify:

- default, hover, focus, active, and selected states;
- disabled and read-only states;
- loading and skeleton states;
- error and validation states;
- any state-specific changes to padding, border, icon, or color;
- how focus rings or outlines affect outer dimensions.

State changes often alter size or spacing. If the spec only shows the default, the engineer will build a component that breaks or shifts when state changes. Show and measure every state.

### Document Behavior Under Variable Content

Components are designed with ideal content, but they live with real content. A button with a short label and a button with a long label may need different behavior. A card with one line and a card with five lines must still look intentional.

Specify variable-content behavior by:

- defining minimum and maximum widths and heights;
- specifying wrapping, truncation, and ellipsis behavior;
- showing alignment when content is shorter or longer than the mockup;
- documenting icon-only and icon-plus-text variants;
- handling dynamic content that appears or disappears;
- reserving space for optional elements so layout does not jump.

An engineer who only sees the ideal case will build something that collapses or overflows on real data. Show the range, not just the center.

### Resolve Contradictions And Ambiguities Before Handoff

Specs often contain contradictions: two measurements that cannot both be true, a token reference that does not exist, a spacing that breaks the grid. These seem minor but force the engineer to choose, and their choice may not match intent.

Before finalizing a spec, check for:

- measurements that conflict within or across frames;
- token names that do not exist in the system;
- values that break the stated grid or baseline;
- states that imply impossible transitions;
- annotations that contradict the visual.

Resolve contradictions explicitly, even if the resolution is "this value wins." A spec with a known, documented choice is buildable; a spec with hidden contradictions is a source of bugs.

### Make The Spec Legible To Its Reader

A spec is read under time pressure by someone who was not in the design conversations. Its legibility determines whether it is followed. Dense, overlapping redlines with tiny labels are read slowly and inaccurately.

Improve legibility by:

- using clear, consistent annotation styles;
- placing labels close to what they describe without overlapping;
- grouping related measurements visually;
- using callouts for behavior that cannot be shown spatially;
- keeping a hierarchy so the most important specs stand out;
- providing a written summary for complex components.

The spec is a communication artifact. If it is hard to read, it will be misread.

### Include Accessibility Measurements Explicitly

Accessibility requirements, focus targets, contrast, and touch sizes, are part of the spec, not an afterthought. If they are not measured and stated, they will be missed or implemented inconsistently.

Include explicitly:

- focus ring or outline specifications;
- minimum touch or click target sizes;
- contrast ratios for text and meaningful graphics;
- semantic roles and names where relevant;
- reading and focus order;
- behavior under zoom and large-text settings.

Accessibility built into the spec is accessibility that ships. Accessibility added later is accessibility that is usually incomplete.

## Common Traps

### Absolute Numbers Without A System

Labeling every gap with pixels hides the spacing logic and causes drift. Reference tokens and express the system.

### Default State Only

Omitting hover, focus, error, disabled, and loading states leaves engineers to invent them. Specify and measure every state.

### Hardcoded Colors And Values

Raw hex values and one-off measurements guarantee inconsistency. Reference semantic tokens that exist in code.

### Ideal Content Only

Specs drawn with short, balanced content break on real data. Show minimum, maximum, wrapping, and truncation behavior.

### Hidden Contradictions

Conflicting measurements and nonexistent tokens force engineers to guess. Resolve contradictions before handoff.

### Illegible Redlines

Overlapping, tiny, or inconsistent annotations are misread under pressure. Prioritize clarity and hierarchy in the spec.

### Accessibility As An Afterthought

Focus, target size, contrast, and semantics omitted from the spec are usually omitted from the build. Include them explicitly.

## Self-Check

- [ ] Spacing, grid, and alignment are expressed as systems and tokens, not only as isolated absolute numbers.
- [ ] Typography is specified as a scale with family, weight, size, line height, letter spacing, scaling, and long-text behavior.
- [ ] Every color, spacing, radius, and elevation value references an existing semantic token, with theme aliases documented.
- [ ] All interactive states, default, hover, focus, active, selected, disabled, loading, and error, are shown and measured.
- [ ] Variable-content behavior, including min/max dimensions, wrapping, truncation, alignment, and dynamic elements, is documented.
- [ ] Contradictions, conflicting measurements, nonexistent tokens, and grid-breaking values are resolved before handoff.
- [ ] The spec is legible, consistently annotated, non-overlapping, grouped, and summarized for fast accurate reading.
- [ ] Accessibility measurements, focus, target size, contrast, semantics, order, and zoom behavior, are included explicitly.
- [ ] Derived and fixed measurements are distinguished, and the system can adapt without breaking relationships.
- [ ] The spec has been read as if by an engineer who was not in the design conversations, and every gap has been closed.
