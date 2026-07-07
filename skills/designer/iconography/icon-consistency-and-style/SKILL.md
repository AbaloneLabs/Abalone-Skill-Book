---
name: icon_consistency_and_style.md
description: Use when the agent is mixing icons from different sources, auditing an icon set for visual consistency, deciding stroke weight, corner radius, fill style, optical weight, and detail density, or unifying icons drawn by different designers and from different libraries into one coherent family.
---

# Icon Consistency And Style

Consistency is what separates a designed icon family from a borrowed collection. When icons come from different libraries, different designers, or different eras of a product, they almost always clash: one is outlined and thin, another is filled and heavy, one uses round corners, another uses sharp ones, one is detailed and illustrative, another is minimal and geometric. Even when each icon is acceptable alone, the mixture reads as unfinished, because the eye detects the absence of a shared style before it identifies any individual icon.

Use this skill when assembling, auditing, or unifying an icon set. The goal is to prevent the agent from shipping a product whose icons are technically present but stylistically incoherent, and to give the agent concrete rules for resolving clashes when icons must be combined from multiple sources.

## Core Rules

### Define The Style Attributes Before Mixing Anything

An icon family is defined by a small set of style attributes. Decide each one explicitly, then enforce it across every icon.

The attributes that most affect perceived consistency:

- **Fill style**: outline, filled, duotone, or solid. Mixing outline and filled icons in the same toolbar is the most common consistency failure.
- **Stroke weight**: the thickness of lines at the base size. Even a 0.25px difference is visible side by side.
- **Corner radius**: how rounded the corners are, both inside and outside. Sharp icons mixed with rounded icons feel like two families.
- **End caps and joins**: rounded, square, or butt ends; miter, round, or bevel joins.
- **Detail density**: how much internal detail an icon carries. A highly detailed icon next to a minimal one breaks rhythm.
- **Geometric versus organic**: whether shapes are built from strict geometry or drawn with hand-feel.
- **Optical weight**: how heavy the icon feels, determined by the ratio of ink to space.

Document these attributes as the family contract. Any icon that violates one of them is inconsistent, regardless of how good it looks alone.

### Match Optical Weight, Not Just Bounding Box

Two icons can sit in the same 24x24 box and still feel different in size because their optical weight differs. A dense, filled icon feels heavier and larger than a sparse, outlined one even when their bounding boxes match.

To unify a set:

- balance the ratio of positive to negative space across icons;
- ensure stroke weight is constant, because stroke is the main driver of perceived weight in outlined sets;
- check that filled and outlined icons are not mixed unless the mix is intentional and rule-based (for example, outline default and filled active).

Optical consistency is judged by eye on a contact sheet, not by measuring boxes. Lay all icons out at the same size and look for the ones that feel heavier or lighter.

### Keep Corner Radius And Geometry Language Uniform

Corner radius is a subtle but powerful consistency signal. A set where some icons have 2px corners and others have sharp corners looks like two systems stitched together.

Decide:

- a single outer corner radius for the family;
- a single inner corner radius (often related to the outer by a consistent rule);
- whether circular elements, badges, and containers follow the same radius logic.

The same applies to geometric construction. If most icons are built from circles and rectangles on a strict grid, an icon drawn freehand with curves will stand out. Match the construction method as well as the visible style.

### Resolve Source Clashes Deliberately

When icons must be combined from different libraries, do not simply drop them in. Choose one library or style as the dominant reference and redraw or adjust the others to match.

Options for resolving clashes:

- **Redraw the minority**: if most icons are from one family, redraw the few outsiders to match.
- **Adopt a neutral style**: if no source dominates, pick a single target style and convert all icons to it.
- **Segment by surface**: if mixing is unavoidable, confine each style to a distinct surface (for example, navigation uses one family, status uses another) so the clash is not visible in the same view.

The worst option is to mix styles within the same toolbar, card, or list, where the inconsistency is most visible.

### Audit With A Contact Sheet

Consistency problems hide when icons are viewed one at a time and reveal themselves when icons are viewed together. Build a contact sheet that places every icon in the set at the same size, in a grid, on a neutral background.

Look for:

- icons that feel heavier or lighter than their neighbors;
- stroke weight drift;
- corner radius drift;
- detail density outliers;
- icons whose optical center is off compared to the grid.

This audit should be repeated whenever icons are added, because new additions are the most common source of drift.

### Respect The Product Voice In Style Choice

Style is not only consistency; it is expression. A rounded, soft icon family supports a friendly, approachable product voice. A sharp, geometric family supports a precise, technical, or premium voice. A hand-drawn family supports a warm, human, or craft-oriented voice.

The style attributes chosen for consistency should also be the ones that reinforce the product's intended character. Consistency for its own sake is necessary but not sufficient; the unified style should be the right style.

### Plan For Filled And Outline Variants Together

Many systems need both outline and filled variants (for default and active states). Decide the relationship between them in advance rather than creating filled versions ad hoc.

Rules that help:

- the filled version should be the outline version with its interior filled, not a different drawing;
- stroke weight of the outline should map to a consistent visual weight in the filled version;
- document which icons have filled variants and which do not, so the set is predictable.

## Common Traps

### Mixing Outline And Filled Icons In The Same View

This is the single most visible consistency failure. Unless the mix follows a documented state rule, it reads as error.

### Treating Bounding-Box Match As Size Match

Icons with identical bounding boxes can differ widely in optical weight. Consistency is optical, not mathematical.

### Inheriting Inconsistency From Third-Party Libraries

Open-source icon libraries each have their own style assumptions. Combining two without unification produces a clash that users feel even if they cannot name it.

### Letting Detail Density Vary Wildly

One highly detailed icon among minimal ones draws disproportionate attention and breaks the rhythm of a row or grid.

### Ignoring Inner Versus Outer Corner Radius

Matching outer corners while leaving inner corners inconsistent is a subtle flaw that experienced reviewers catch and that reads as slightly off to everyone.

### Adding Icons Without Re-Auditing The Set

Each new icon is a chance to introduce drift. Sets that are never re-audited gradually lose consistency as they grow.

### Choosing Style For Aesthetics Over Voice Fit

A beautifully consistent set in the wrong style still fails if it contradicts the product's character. Consistency must serve expression.

## Self-Check

- [ ] Fill style, stroke weight, corner radius, caps, joins, detail density, and construction method are defined and documented for the family.
- [ ] Icons share optical weight, not just bounding-box size, verified on a contact sheet.
- [ ] Outer and inner corner radii follow a consistent rule across the set.
- [ ] Icons from different sources were redrawn or adjusted to match the dominant style, not dropped in unchanged.
- [ ] A contact-sheet audit was performed and any outliers in weight, stroke, radius, or detail were corrected.
- [ ] The chosen style reinforces the product voice rather than contradicting it.
- [ ] Filled and outline variants follow a documented relationship and are not created ad hoc.
- [ ] No view mixes inconsistent styles unless the mix follows an explicit, documented rule.
- [ ] The set is scheduled for re-audit whenever new icons are added.
