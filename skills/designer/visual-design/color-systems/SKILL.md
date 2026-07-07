---
name: color_systems.md
description: Use when the agent is building or extending a color system, defining semantic and functional color tokens, setting up themes including dark mode, managing contrast and accessibility, deciding how many colors a product needs, or ensuring color choices scale across states, surfaces, and brand expressions without becoming inconsistent or unmaintainable.
---

# Color Systems

A color system is not a palette of attractive swatches; it is a set of decisions about what each color means, where it is allowed to appear, how it changes across states and themes, and how it stays consistent as the product grows. Most color problems are not problems of taste but of structure: a product accumulates dozens of near-identical grays, semantic colors that mean different things in different places, and dark modes that are bolted on as an afterthought. The judgment problem is building color as a system of roles and tokens rather than a list of hex values, deciding how many colors the product truly needs, and ensuring the system handles contrast, states, and themes without collapsing. Agents tend to fail by picking colors visually without defining their function, by adding colors ad hoc until the set is ungovernable, and by treating dark mode as an inversion rather than a separate, designed theme.

Use this skill when creating, extending, or auditing a color system for any product: defining tokens, building themes, ensuring accessibility, or deciding color's role in hierarchy and meaning. The goal is a color system that is consistent, meaningful, accessible, and maintainable at scale.

## Core Rules

### Define Color By Role, Not By Hex Value

The foundation of a maintainable color system is that colors are chosen by what they do, not by what they look like. A button does not use "#3B82F6"; it uses the "primary action" color, which happens to resolve to a value in the default theme.

Organize color by role:

- semantic roles: primary, secondary, success, warning, error, info;
- functional roles: text, background, border, surface, overlay;
- state roles: default, hover, active, disabled, focused;
- contextual roles specific to the product, such as chart series or status indicators.

When a designer or engineer reaches for a color by its role, the system stays consistent. When they reach by hex value, the system fragments, because no one can remember which of forty similar blues is the right one.

### Build A Token Layer That Separates Meaning From Value

A robust color system has at least two layers: semantic tokens that describe meaning, and primitive tokens that hold actual values. The semantic layer references the primitive layer, and components reference only the semantic layer.

Structure tokens as:

- primitive tokens: the raw palette, such as blue-500, gray-100, with defined scales;
- semantic tokens: roles like color-text-primary or color-action-default, mapped to primitives;
- component-level tokens, optional, for cases where a component needs its own named color.

This separation lets you retheme the entire product by remapping semantic tokens to different primitives, without touching components. Dark mode, brand variants, and white-labeling all become remapping problems rather than redesign problems.

### Decide How Many Colors The Product Needs, And Stop

Color sprawl is one of the most common maintainability failures. Every new screen tempts someone to add a new shade, and soon the system contains hundreds of colors that no one can govern.

Control scope by:

- defining a fixed number of hues the product uses, typically the brand color plus a small set of semantic colors;
- giving each hue a defined scale of steps, commonly 5 to 11, rather than arbitrary single values;
- mapping every needed role onto existing tokens before creating a new one;
- requiring justification and review for any new color added to the system.

The discipline is not minimalism for its own sake; it is preventing the system from becoming ungovernable. A system of fifty well-defined tokens beats a system of five hundred ad hoc values every time.

### Design Semantic Color To Be Unambiguous

Semantic colors carry meaning, and that meaning must be consistent everywhere. If red means error in one place and deletion in another and danger in a third, the user cannot interpret it reliably.

Make semantic color unambiguous:

- assign one meaning to each semantic color and apply it everywhere;
- distinguish levels where needed, such as warning versus error versus critical, with clearly different hues;
- never reuse a semantic color for decoration, or it loses its signal;
- pair color with a secondary cue, icon or text, because color alone is not accessible.

Color is a powerful signal only when it is reliable. Reusing semantic colors for non-semantic purposes destroys their meaning.

### Ensure Contrast And Accessibility By Construction

Accessibility is not a final check; it is a constraint on how the color system is built. If contrast is verified only at the end, the system will routinely fail, because attractive color combinations often do not meet contrast requirements.

Build accessibility in by:

- defining which token pairings are approved for text, and verifying their contrast at definition time;
- providing text and non-text contrast variants where needed;
- documenting the minimum contrast each role must maintain;
- testing the system, not just individual screens, against WCAG requirements;
- never relying on color alone to convey information.

A color system whose approved pairings all pass contrast makes accessible design the default rather than a retrofit.

### Design Dark Mode As A Separate Theme, Not An Inversion

Dark mode is not light mode with colors inverted. Inverting often produces poor contrast, broken semantic meaning, and visual discomfort, because colors that work on white behave differently on black.

Design dark mode deliberately by:

- choosing dark-theme primitives that maintain contrast and hierarchy, not inverting light values;
- adjusting semantic colors so they remain readable and meaningful on dark surfaces;
- reducing saturation and lightness for large color areas, which appear more vivid on dark backgrounds;
- reserving pure black carefully, as it can cause halation and lose elevation cues;
- testing every state and surface in dark mode, not just the default screen.

A dark theme designed as its own system feels intentional. An inverted theme feels broken.

### Manage Color Across States And Surfaces

A single brand color is not enough, because that color must survive on different backgrounds, in different states, and at different sizes. Each role needs a small family of related values.

Provide for variation:

- hover, active, and disabled variants for interactive colors;
- on-light and on-dark variants for colors used across themes;
- muted and emphasis variants for backgrounds versus text versus fills;
- sufficient steps in each hue scale to support all needed surfaces.

A color system with only one value per role forces compromises that hurt consistency. Anticipate the states and surfaces, and define the variants up front.

### Document Governance And Contribution

A color system without governance rots. Without clear rules for what exists, why, and how to change it, contributors add colors freely and the system drifts.

Govern by:

- documenting every token, its role, its allowed uses, and its contrast properties;
- defining a process for proposing, reviewing, and adding or retiring colors;
- regularly auditing for duplicate, unused, or off-system colors and removing them;
- keeping the source of truth in code or design tooling, not in scattered documents.

Governance is what keeps a color system usable in year three. Without it, the system becomes the problem it was meant to solve.

## Common Traps

### Choosing Colors By Hex Instead Of Role

Reaching for values rather than roles fragments the system. Always reference semantic roles.

### Color Sprawl From Ad Hoc Additions

Adding a new shade per screen produces an ungovernable system. Define a fixed set and require justification for additions.

### Reusing Semantic Color For Decoration

Using error-red for branding destroys its meaning. Keep semantic colors semantic.

### Treating Accessibility As A Final Check

Verifying contrast at the end guarantees routine failures. Build approved pairings into the system.

### Inverting Light Mode For Dark Mode

Inversion produces poor contrast and broken semantics. Design dark mode as its own theme.

### One Value Per Role

A single color cannot survive all states and surfaces. Define the needed variants up front.

### No Governance Or Source Of Truth

Without documentation and review, the system drifts into chaos. Govern tokens and audit regularly.

## Self-Check

- [ ] Colors are defined and referenced by semantic role, not by raw hex value.
- [ ] A token layer separates meaning from value, enabling retheming without touching components.
- [ ] The number of hues and shades is bounded, with justification required for additions.
- [ ] Each semantic color has one consistent meaning and is not reused for decoration.
- [ ] Approved text and non-text pairings meet contrast requirements by construction, not by late checking.
- [ ] Dark mode is designed as a separate theme with its own primitives, not an inversion.
- [ ] Each role has variants for hover, active, disabled, and cross-theme use.
- [ ] Information is never conveyed by color alone; secondary cues accompany every semantic color.
- [ ] Tokens are documented with their roles, allowed uses, and contrast properties.
- [ ] A governance process exists for adding, changing, and retiring colors, with regular audits.
