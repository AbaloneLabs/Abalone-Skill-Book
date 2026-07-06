---
name: rtl_layout_and_direction.md
description: Use when the agent is establishing or auditing the base text direction and physical layout for right-to-left locales, setting direction in markup or document formats, choosing between logical and physical layout properties, configuring alignment and flow for RTL, debugging direction inheritance in nested containers, or preparing CSS, HTML, XML, desktop resource, or document-publishing assets for Arabic Hebrew Persian Urdu or other RTL targets.
---

# RTL Layout And Direction

Right-to-left (RTL) locales do not merely need their text right-aligned. The entire layout model reverses: the starting edge of a block becomes the right, inline content flows toward the left, the visual order of multi-column structures inverts, and every property that was written against a fixed left-to-right assumption must be re-expressed in direction-aware terms. Translators and localization engineers often treat direction as a per-paragraph text attribute and stop there, but the harm shows up at the layout level: sidebars appear on the wrong side, indentation and margins push content the wrong way, tables read in the wrong order, and the composition that looked balanced in the LTR source becomes visually broken or semantically scrambled in the RTL target. Direction is a layout concern as much as a text concern, and getting it right requires understanding how base direction propagates, how logical and physical properties differ, and how each output format expresses RTL.

This skill applies when you are setting up, reviewing, or repairing the direction and layout of localized RTL deliverables in HTML, CSS, XML, desktop and mobile resource files, office documents, or publishing formats. The objective is a layout whose direction is declared explicitly, inherited correctly, and expressed in direction-aware terms so that the RTL composition mirrors the LTR source in both structure and intent.

## Core Rules

### Declare Base Direction Explicitly At Every Meaningful Boundary

Base direction is the property that tells the rendering engine which way the block flows, and it should be set deliberately rather than left to inference.

Many formats infer base direction from the first strong directional character in a segment. That inference is fragile: it fails on segments that begin with a number, a URL, punctuation, whitespace, or an English brand name embedded in RTL text, and it can flip the reading order of an entire paragraph. Prefer explicit declaration. In HTML, set the `dir` attribute on the root or on each container; in XML and document formats, use the equivalent direction attribute or locale tag; in desktop and mobile resources, set the layout direction property on the window or view. Declare direction at the document level and again at any container whose content language differs from its surroundings. Explicit declaration is what makes direction predictable rather than something the reader must reverse-engineer.

### Distinguish Logical Properties From Physical Ones

The single most consequential layout decision in RTL work is whether properties are expressed in physical terms (left, right) or logical terms (start, end).

Physical properties are fixed to the viewport: `margin-left`, `padding-right`, `left: 10px`, and `text-align: left` do not change when the direction flips, so an RTL layout built with them ends up pushing content toward the wrong edge. Logical properties are direction-aware: `margin-inline-start`, `padding-inline-end`, `inset-inline-start`, and `text-align: start` resolve to the correct physical side based on the declared direction. Whenever the format supports logical equivalents, use them. Reserve physical properties for cases where the design truly must be fixed to one side regardless of direction, such as a physical diagram or a logo whose position is intentional. Audit any stylesheet or resource file inherited from the LTR source and convert directional physical properties to logical ones; leaving them physical is the most common reason an RTL layout looks half-mirrored.

### Make Direction Inherit Correctly Through Nested Containers

Direction inherits from parent to child, and nested containers with mixed content can break that inheritance in subtle ways.

When a container declares RTL and a child holds an LTR run such as a code block, a citation, or a phone number, the child often needs its own explicit LTR direction rather than relying on the inherited RTL. Conversely, an LTR document that embeds an RTL quotation must declare RTL on that quotation so it does not inherit the surrounding LTR. The trap is that inheritance is silent: a container that looks correct in isolation can produce wrong reading order because an ancestor or descendant set a direction that propagated unexpectedly. Trace direction through the containment hierarchy, declare it wherever the dominant script of the content changes, and verify that each nested level resolves to the direction its content actually needs.

### Handle Alignment As A Logical Concern

Text and element alignment must follow the logical model, not a hardcoded physical side.

Right-aligning body text is the visible symptom people associate with RTL, but alignment is downstream of direction: when direction is set correctly and alignment uses logical values, the text aligns itself correctly. Hardcoding `text-align: right` produces text that aligns correctly for RTL body copy but breaks for any LTR run inside it, and it does nothing for block-level layout. Use `text-align: start` so that body copy aligns to the right in RTL and to the left in LTR, and so embedded runs of the opposite direction still behave. Check headings, captions, table cells, and form labels specifically, because these are frequently overlooked and end up aligned to the wrong edge.

### Configure Flow Direction For Structural Layout

Beyond text, structural layout elements such as columns, flex containers, grids, and tables must have their flow direction configured for RTL.

In flexbox and grid, the main axis and the placement order follow the inline direction, so an RTL container reverses the visual order of its items automatically when direction is set. If the source CSS used physical ordering or explicit item ordering, that ordering can fight the direction and produce a layout that is text-RTL but structurally-LTR. Tables in RTL reverse column order, which is usually correct but can be wrong when the table encodes a fixed physical sequence such as a timeline. Walk through every structural container, confirm that its flow resolves under RTL, and override only where the physical order is semantically meaningful.

### Preserve The Distinction Between Reading Order And Visual Order

RTL layout reverses visual order, but the underlying logical or source order should remain meaningful and should not be manually scrambled.

A frequent mistake is to physically reorder elements in the source to force a visual mirror, for example swapping two columns in the markup instead of letting direction handle it. Manual reordering couples the source to one direction, breaks accessibility and screen-reader order, and makes the layout impossible to maintain bilingually. Keep the logical order consistent with meaning and let direction properties produce the visual mirror. When you must reorder, do it with direction-aware mechanisms, not by editing content sequence.

### Set Direction In The Source Of Truth, Not Only In Output

Direction should be authored in the source assets and preserved through the pipeline, not patched in at publication.

If direction lives only in a post-processing step, every regeneration or re-export risks losing it, and downstream consumers such as search indexers, accessibility tools, and PDF converters may never see it. Encode direction in the source files, the resource bundles, the templates, and the content model, and confirm that the CAT and publishing pipeline preserves those declarations end to end. A direction that depends on a manual final step is a direction that will eventually be dropped.

## Common Traps

### Inferring Direction From The First Strong Character

Inference flips on numbers, URLs, punctuation, and embedded Latin terms; declare direction explicitly instead.

### Using Physical Properties Throughout

`left`, `right`, `margin-left`, and `text-align: right` are fixed to the viewport and produce half-mirrored layouts; convert to logical properties.

### Right-Aligning Instead Of Setting Direction

Hardcoded right alignment fixes body copy but breaks embedded runs and structural layout; alignment should follow direction, not replace it.

### Letting Inheritance Drift In Nested Containers

Mixed-direction children and embedded quotations inherit silently; declare direction wherever the dominant script changes.

### Manually Reordering Elements To Mirror Visually

Swapping source order couples the file to one direction and breaks accessibility and screen-reader sequence; let direction properties mirror.

### Assuming Structural Layout Mirrors Automatically

Flex, grid, and tables follow direction only if their ordering was not overridden physically; verify each structural container.

### Patching Direction Only At Publication

Direction that exists only in a post-processing step is lost on re-export and invisible to downstream tools; encode it in source assets.

## Self-Check

Before delivering or approving an RTL layout, verify:

- Base direction is declared explicitly at the document level and at every container whose dominant script differs from its surroundings, not left to first-strong-character inference.
- Layout properties use logical equivalents such as inline-start and inline-end wherever the format supports them, and physical left/right properties remain only where a fixed physical position is intentional.
- Direction inheritance has been traced through nested containers, and each level resolves to the direction its content requires.
- Text alignment uses logical values so body copy, headings, captions, table cells, and form labels align correctly without hardcoded right alignment.
- Structural containers such as flex, grid, columns, and tables have their flow direction configured, and no physical ordering fights the declared direction.
- Logical or source order has not been manually scrambled to force a visual mirror; visual reversal comes from direction properties.
- Direction is encoded in source assets and preserved through the CAT and publishing pipeline, not applied only as a final publication step.
- The layout has been reviewed in the rendered RTL target, not only in the source view, confirming that containers, margins, and alignment resolve as intended.
