---
name: spacing_and_scale.md
description: Use when the agent is defining a spacing and measurement scale, choosing a base unit and modular ratio, building a type scale that relates to the spacing scale, controlling density and information pacing, applying spatial systems as design tokens, or ensuring spacing communicates relationship and hierarchy consistently across a product.
---

# Spacing And Scale

Spacing and scale form the measurement system that every layout inherits. It looks like a set of numbers, but it is really a set of relationships: how a base unit generates a scale, how spacing and type scales relate, how density is controlled, and how gaps communicate grouping and hierarchy. Agents tend to invent spacing values per screen, build type and spacing scales independently until they conflict, or treat spacing as separation rather than information. The harm is layouts that drift, density that feels inconsistent, and spacing that carries no meaning because it was never systematized.

Use this skill before defining a spacing scale, a type scale, or density rules. The goal is to prevent the agent from choosing measurements by eye, from building scales that do not compose, or from using spacing as mere separation when it should encode structure.

## Core Rules

### Derive The Scale From A Single Base Unit

A measurement system begins with a base unit from which all other values derive. Without a base unit, spacing values are arbitrary and do not compose: two small gaps do not equal one medium gap, and the scale cannot be extended predictably. A base unit (commonly four or eight pixels) multiplied into a sequence gives spacing that composes and scales.

Establish the base unit:

- choose one base unit and forbid off-grid values in production;
- derive the scale as multiples of the base, not as independent arbitrary numbers;
- ensure adjacent steps are distinguishable but the set remains usable;
- document the base unit so all teams derive from the same foundation.

A scale without a base unit is a list of numbers, not a system. The base unit is what makes the scale generative and consistent.

### Choose A Modular Ratio That Produces A Usable Scale

How the scale grows matters as much as its base. A modular ratio (such as a major second, minor third, or perfect fourth) determines the relationship between steps. Too small a ratio produces steps too similar to distinguish; too large a ratio produces gaps too big to fill. The ratio should produce a scale with enough steps to cover the needs without redundancy.

Select the ratio deliberately:

- smaller ratios (like 1.125) produce many fine steps, suited to type scales needing granularity;
- larger ratios (like 1.333 or 1.5) produce fewer, more distinct steps, suited to spacing scales;
- limit the scale to the steps actually needed, typically six to ten, to avoid decision fatigue;
- name the steps semantically (xs, sm, md, lg, xl) so usage is by intent, not raw numbers.

A scale with too many steps gives the illusion of precision but in practice means designers pick the closest value and the rhythm drifts. A well-chosen ratio produces a scale that is both expressive and disciplined.

### Align The Type Scale And Spacing Scale To The Same Base

Type and spacing are often designed independently, then conflict when combined: line heights do not match spacing steps, and text does not align to the layout grid. Aligning both scales to the same base unit ensures they compose: a line of text fits within spacing steps, and vertical rhythm holds across text and components.

Unify the scales:

- derive both the type scale and the spacing scale from the same base unit;
- ensure line heights relate to spacing steps so text blocks fit the vertical rhythm;
- check that headings, body, and components align to shared horizontal and vertical lines;
- reconcile the two scales where they diverge, rather than letting them drift.

When type and spacing scales conflict, layouts feel subtly off: text that does not quite align, gaps that do not match line heights. Unifying the base makes the whole layout coherent.

### Use Spacing To Encode Relationship And Hierarchy

Spacing is information, not just separation. Elements close together read as related; elements far apart read as separate. When spacing is chosen by habit, it stops carrying meaning and the layout becomes ambiguous. A disciplined spacing system uses each gap to communicate grouping, hierarchy, and structure.

Encode meaning in spacing:

- tight spacing (intra-component) groups fields, labels, and controls that belong together;
- medium spacing separates sibling components that are peers;
- large spacing separates sections and major regions;
- extra whitespace around a primary element signals its importance.

If two unrelated things sit as close as two related things, the user cannot tell the grouping. If every gap is the same, nothing reads as grouped. Spacing should be the most consistent signal of structure.

### Control Density Deliberately

Density is how tightly information is packed, and it shapes the feel and usability of a layout. A dense layout (like a data table) packs much information but risks overwhelming; a sparse layout (like a marketing page) breathes but risks feeling empty. Density is a design decision that should match the content and the user's task, not an accident of how much was packed in.

Control density by context:

- high density suits expert users and data-heavy tasks where scanning many items is the goal;
- low density suits novice users, marketing, and focused tasks where clarity matters most;
- offer density options (comfortable versus compact) where the same product serves different needs;
- ensure density choices do not break spacing relationships or accessibility (touch targets, line height).

A layout whose density was never decided feels either cramped or empty depending on the content that happened to fill it. Density should be a deliberate match to the task.

### Define Spacing Roles For Component, Layout, And Section Levels

Spacing serves different purposes at different scales, and confusing them produces inconsistent layouts. Component-level spacing (padding within a card) is different from layout-level spacing (gaps between cards), which is different from section-level spacing (between major page regions). Each level needs its own defined range from the scale.

Define spacing by level:

- intra-component spacing: tight steps for padding, field gaps, and internal structure;
- inter-component spacing: medium steps for gaps between sibling components;
- section spacing: large steps for separating major page regions;
- page margins: the outer frame, often the largest spacing values.

Using the same spacing step for internal padding and section separation flattens the hierarchy spatially. Distinct ranges per level make the structure legible through spacing alone.

### Make The Scale Token-Driven And Closed

A spacing scale that lives in a designer's head or in one file will not survive implementation. The values must become tokens that design and code share, and the scale must be closed: off-scale values are detectable and discouraged, so the system does not fragment through ad-hoc additions.

Tokenize and close the scale:

- express every spacing step as a design token consumed by both design tools and code;
- forbid off-scale values in production, with linting or review to catch them;
- evolve the scale deliberately through a defined process, not through individual additions;
- version the scale so changes are intentional and traceable.

An open scale where anyone adds values becomes a drawer of numbers with no system. A closed, tokenized scale is what keeps spacing consistent across a growing product.

### Test The Scale Against Real Content Variety

A spacing scale designed against ideal content often fails with real data: long text, dense tables, translated strings, empty states. The scale must be tested against the full variety of content the product will hold, not just the cleanest examples, or it will break where the content gets messy.

Test broadly:

- test against sparse content (empty states, short lists) to ensure the layout does not feel barren;
- test against dense content (long tables, many fields) to ensure spacing does not collapse;
- test against localized content with text expansion, which changes spacing needs;
- test against edge cases (very long names, single-item lists) that stress the scale.

A scale that only works with perfect content is fragile. Real content is the test that matters.

## Common Traps

### Arbitrary Spacing Values Without A Base Unit

Values chosen per screen do not compose and cause the rhythm to drift; derive the scale from a single base unit.

### A Ratio That Produces Too Many Or Too Few Steps

Too fine a ratio creates indistinguishable steps; too coarse creates unusable gaps; choose a modular ratio that yields a disciplined set.

### Independent Type And Spacing Scales

When type and spacing derive from different bases, text does not align to the layout grid; unify both to the same base unit.

### Spacing As Mere Separation

When gaps carry no meaning, grouping becomes ambiguous; use spacing to encode relationship and hierarchy.

### Density Left To Accident

Layouts whose density was never decided feel cramped or empty depending on content; match density deliberately to the user and task.

### One Spacing Step For All Levels

Using the same value for component padding and section separation flattens hierarchy; define distinct ranges per level.

### An Open Scale With Ad-Hoc Values

A scale anyone can add to fragments into a drawer of numbers; tokenize, close, and version the scale.

### Testing Only Against Ideal Content

Scales designed for clean examples break with dense, sparse, or localized real content; test the full variety.

## Self-Check

- [ ] The spacing scale is derived from a single base unit, with off-grid values forbidden in production.
- [ ] A modular ratio produces a usable set of steps (typically six to ten), named semantically by intent.
- [ ] The type scale and spacing scale derive from the same base unit, so text aligns to the layout rhythm.
- [ ] Spacing encodes relationship: tight gaps group related elements, medium gaps separate peers, large gaps separate sections.
- [ ] Density is controlled deliberately to match the user and task, with options where the product serves varied needs.
- [ ] Distinct spacing ranges are defined for intra-component, inter-component, section, and page-margin levels.
- [ ] The scale is tokenized, closed against ad-hoc values, and versioned so changes are deliberate.
- [ ] The scale was tested against sparse, dense, localized, and edge-case content, not only ideal examples.
