---
name: typography_scale.md
description: Use when the agent is defining a type system or type scale, choosing font sizes and line heights for headings and body, establishing hierarchy through type, selecting type families and weights, setting line length and measure, or ensuring typography is readable, consistent, and responsive across the product.
---

# Typography Scale

Typography is the primary medium through which most products communicate, and a type scale is the system that keeps that communication legible, hierarchical, and consistent. Without a scale, every heading and label becomes a one-off decision, and the product accumulates dozens of slightly different sizes, line heights, and weights that feel chaotic even when each seems reasonable alone. The judgment problem is building type as a system of related sizes rather than a collection of independent choices, choosing type that serves reading and hierarchy, and ensuring the scale holds across devices, contexts, and content lengths. Agents tend to fail by picking sizes visually per screen, by ignoring line height and measure, and by treating type as styling rather than as the structure of reading.

Use this skill when establishing or revising a type system: defining sizes, weights, line heights, families, or responsive behavior. The goal is typography that is readable, hierarchical, consistent, and scalable.

## Core Rules

### Build A Scale, Not A List Of Sizes

A type scale is a set of sizes that relate to each other by a consistent ratio, so that stepping from one level to the next feels predictable and harmonious. A list of unrelated sizes, by contrast, feels arbitrary and makes hierarchy unclear.

Build the scale by:

- choosing a base body size grounded in readability, commonly 16 pixels for web;
- selecting a ratio, such as a minor third or perfect fourth, to generate successive heading sizes;
- defining a small set of named steps, such as display, h1, h2, h3, body, small, caption;
- using only those steps, and documenting them as the system.

A scale creates rhythm and makes hierarchy legible at a glance, because the eye recognizes the proportional steps. Arbitrary sizes cannot achieve this.

### Establish Hierarchy Through Size, Weight, And Color Together

Hierarchy is not only about making headings bigger. Over-reliance on size produces headings that are too large and a layout that feels shouty. A strong system uses size, weight, color, and spacing in concert.

Balance hierarchy levers:

- use size differences that are large enough to read as distinct levels, not so large they dominate;
- vary weight to reinforce hierarchy without always going to the heaviest weight;
- use color and contrast to differentiate, such as darker text for headings and muted text for secondary;
- let spacing and whitespace reinforce the grouping that type implies.

A heading that is only slightly larger but clearly heavier and better spaced reads as a heading. Hierarchy is the sum of several cues, not the size alone.

### Choose Type For Readability First, Personality Second

Typefaces convey personality, but their primary job is to be read, often at length, on screens, by diverse users. Choosing a typeface for its character before its readability inverts the priority.

Prioritize readability by:

- selecting body faces with open counters, clear distinctions between similar characters, and sturdy weights;
- testing at the smallest sizes and on the lowest-density screens the product supports;
- favoring typefaces designed for screens, with hinting and a full weight range;
- reserving expressive or display faces for large, short headings where readability stress is low.

A beautiful display face that strains the eye in body copy fails its purpose. Readability is the floor; personality is the ceiling.

### Set Line Height And Measure For Sustained Reading

Size is only one variable in readability. Line height (leading) and line length (measure) determine whether text can be read comfortably for more than a sentence, and they are routinely neglected.

Set leading and measure by:

- using looser line height for body text, commonly around 1.4 to 1.6 times the font size, to let the eye track lines;
- using tighter line height for large headings, where the extra space separates letters within a word awkwardly;
- constraining measure to a readable line length, commonly 45 to 75 characters, so lines are neither too short nor too long;
- adjusting line height inversely with size, larger text needs less leading.

Text that ignores leading and measure may be legible word by word but exhausting to read in paragraphs.

### Define Weights Deliberately And Sparingly

Most products need only a few weights: regular for body, medium or semibold for emphasis and subheads, and bold for strong emphasis. Loading and using many weights adds performance cost and visual noise.

Manage weights by:

- defining which weights the system uses and loading only those;
- assigning each weight a clear role, such as regular for body and semibold for labels;
- avoiding weight as the only emphasis method, which fails when bold is unavailable or overused;
- pairing weights with color or size changes for stronger, more reliable hierarchy.

A system with two or three well-chosen weights is clearer and faster than one with seven weights used unpredictably.

### Make Typography Responsive Without Breaking Hierarchy

As the viewport changes, type sizes must adapt, but the hierarchy and rhythm should hold. A common failure is shrinking everything proportionally until headings and body become indistinguishable on mobile.

Make type responsive by:

- using relative units, such as rem, so type scales with user settings;
- defining size adjustments at breakpoints that preserve the steps between levels;
- reducing display and large heading sizes more aggressively on small screens, where they otherwise dominate;
- keeping body size at or above the readability floor on every device.

Fluid type that scales smoothly with viewport can work, but it must be bounded so it never becomes too small to read or too large to be usable.

### Support Internationalization And Character Coverage

Type systems often break the moment they encounter languages or scripts they were not designed for. A product that may serve multilingual audiences must choose type with broad coverage and plan for fallbacks.

Plan for i18n by:

- selecting typefaces with coverage for the scripts and languages the product supports;
- defining fallback stacks so missing characters degrade gracefully rather than showing tofu;
- accounting for scripts that need different line heights, such as those with diacritics or tall characters;
- testing real localized content, not just Latin placeholders.

A type system that works only for English fails the moment it meets the wider world.

### Treat Type As Part Of The Design System

Typography, like color and spacing, must be systematized and governed. Ad hoc type choices per screen produce inconsistency that no amount of polish can hide.

Systematize type by:

- defining type tokens for size, weight, line height, and family;
- applying tokens uniformly so the same role always uses the same type;
- documenting the scale and its intended uses;
- reviewing new type usage against the system before adding variants.

Consistent typography is one of the strongest signals of a mature product. Inconsistent typography is one of the strongest signals of an immature one.

## Common Traps

### Arbitrary Sizes Per Screen

Picking sizes visually destroys hierarchy and rhythm. Use a ratio-based scale.

### Hierarchy Through Size Alone

Over-reliance on size produces shouty layouts. Combine size, weight, color, and spacing.

### Choosing Faces For Personality Over Readability

Expressive faces that strain the eye fail their primary job. Readability first.

### Ignoring Line Height And Measure

Legible words are not the same as readable paragraphs. Set leading and constrain measure.

### Too Many Weights

Loading and using many weights adds cost and noise. Define a few roles.

### Proportional Shrinking That Flattens Hierarchy

Scaling everything down equally erases the steps between levels. Adjust by role at breakpoints.

### Latin-Only Assumptions

Type that lacks script coverage and fallbacks breaks on localization. Plan for i18n.

### Type As Styling Rather Than System

Ad hoc type per screen produces inconsistency. Tokenize and govern typography.

## Self-Check

- [ ] A ratio-based type scale defines a small set of named sizes used consistently.
- [ ] Hierarchy is established through size, weight, color, and spacing together, not size alone.
- [ ] Body typefaces are chosen for readability at small sizes and on low-density screens before personality.
- [ ] Line height and measure are set to support sustained reading, with leading varied inversely by size.
- [ ] A small set of weights is defined, each with a clear role, and only those are loaded.
- [ ] Type is responsive, preserving hierarchy and staying above the readability floor across breakpoints.
- [ ] Type families have coverage and fallbacks for all supported scripts and languages.
- [ ] Typography is tokenized as part of the design system and applied uniformly.
- [ ] The scale and its intended uses are documented for contributors.
- [ ] Real content, including long and localized text, was used to validate the system, not just short placeholders.
