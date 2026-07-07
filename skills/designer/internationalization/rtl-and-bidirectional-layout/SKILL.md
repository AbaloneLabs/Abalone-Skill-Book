---
name: rtl_and_bidirectional_layout.md
description: Use when the agent is designing or reviewing layouts, navigation, icons, spacing, alignment, motion, or components for right-to-left languages such as Arabic, Hebrew, Persian, or Urdu, or when handling bidirectional text, mirroring interfaces, and logical rather than physical positioning.
---

# RTL And Bidirectional Layout

Right-to-left layout is not a mirror flip of a left-to-right design. It is a re-derivation of the interface against a different reading direction, where some elements must flip, some must not, and the rules for which is which are not obvious. A naive mirror produces backward icons, broken charts, misplaced punctuation, and a layout that feels wrong even to users who cannot articulate why. Worse, many teams discover RTL only after launch, when the cost of fixing it is high and the harm to affected users is already done.

Use this skill before finalizing layouts, navigation, icons, directional affordances, data tables, charts, forms, or any component that will ship in RTL languages. The goal is to prevent the agent from treating mirroring as a CSS property to toggle, flipping things that must stay fixed, or leaving physical assumptions embedded in spacing, alignment, and motion.

## Core Rules

### Use Logical, Not Physical, Positioning

The core habit that makes RTL tractable is thinking in logical directions, start and end, rather than physical ones, left and right. A "back" button goes to the start, not to the left; a primary action sits at the end of its row, not at the right. When positioning is expressed logically, the layout flips correctly when the direction changes. When it is expressed physically, every value must be hand-corrected.

Build layouts using logical properties: inline-start and inline-end, block-start and block-end, margin-start and margin-end. Reserve physical left and right only for cases where direction genuinely should not change.

### Mirror What Conveys Direction, Keep What Conveys Identity

Not everything flips. The rule is to mirror elements whose meaning depends on reading or progress direction, and to leave unchanged elements whose meaning is independent of direction or tied to real-world convention.

Mirror these in RTL:

- navigation flow, such as back and forward arrows;
- progress indicators, steppers, and timelines;
- text alignment and reading order;
- indentation, bullets, and list markers;
- sliders, toggles, and directional controls;
- iconography that implies motion or sequence.

Keep these unchanged:

- brand logos and wordmarks;
- phone numbers, currencies, and math notation;
- photographs and illustrations of real scenes;
- media playback controls, which follow hardware convention;
- icons whose meaning is not directional, such as a trash can or magnifier.

When unsure, ask whether the element's meaning changes if it faces the other way. If yes, mirror it; if no, leave it.

### Handle Bidirectional Text Within A Single Line

Real interfaces mix directions within one line: an English product name inside an Arabic sentence, a number inside Hebrew text, parentheses and punctuation that belong to different directions. Bidirectional text has subtle rules about where punctuation attaches and how neutral characters resolve.

Common failures include:

- parentheses appearing on the wrong side;
- punctuation migrating to the opposite end of a phrase;
- mixed-language phrases rendering in the wrong order;
- cursor movement behaving unexpectedly through direction switches.

Use the platform's bidirectional algorithm rather than trying to fix order manually, and test with realistic mixed content rather than pure RTL strings.

### Re-Derive Spacing, Alignment, And Visual Balance

Mirroring a layout does not automatically mirror the visual logic. Icon spacing inside a button, the gap between a label and its field, the position of a chevron next to menu text, and the alignment of badges all need to follow the new reading direction. A mirrored layout that still has left-aligned internal spacing feels subtly broken.

Check each composite element:

- icons inside buttons sit on the correct side relative to text;
- chevrons and arrows point along the reading direction;
- avatar and text pairs flip their internal arrangement;
- badges, counts, and metadata move to the logical end.

### Rethink Charts, Timelines, And Data Direction

Data visualizations carry an implicit direction. A line chart usually reads left-to-right as time progresses; a timeline moves left-to-right; a progress bar fills left-to-right. In RTL, the question is whether the data's direction is a convention, in which case it may stay, or a reading-direction cue, in which case it should flip.

Decide deliberately for each:

- time-based axes may flip so the earliest point is on the right;
- progress bars fill from the start, which is now the right;
- numbered steps progress along the reading direction;
- legends and labels follow the new alignment.

Inconsistency is worse than either choice, so apply one rule across the product.

### Account For Fonts And Vertical Metrics

RTL scripts, especially Arabic, need fonts designed for them, with correct shaping, ligatures, and vertical metrics. A Latin font applied to Arabic produces disconnected, misshapen, or vertically misaligned glyphs. Line height that works for Latin may clip Arabic descenders or diacritics.

Provide font stacks that fall back to quality RTL fonts, set line heights that accommodate taller scripts, and avoid forcing a single typeface across all languages.

### Test With Real RTL Content Early

RTL problems are invisible until real content is loaded. Placeholder Latin text mirrored at the CSS level hides bidirectional issues, punctuation bugs, and metric failures. Test with realistic Arabic or Hebrew content, including mixed-direction phrases, before considering the layout done.

## Common Traps

### Treating Mirroring As A Single CSS Toggle

Flipping the whole interface with one property mirrors things that should not flip, such as logos and media controls, and breaks composite spacing.

### Flipping Things That Should Stay Fixed

Logos, phone numbers, photographs, and hardware-conventional controls become confusing or wrong when mirrored.

### Leaving Directional Icons Unmirrored

A back arrow still pointing left in an RTL interface sends users the wrong way, because the reading direction, and thus the sense of "back," has reversed.

### Physical Left And Right Hardcoded Everywhere

Spacing, alignment, and positioning expressed as left and right must each be hand-corrected, and any missed value produces a subtly broken layout.

### Ignoring Bidirectional Mixed Text

Pure RTL test strings hide the punctuation, parenthesis, and ordering failures that appear only when directions mix within a line.

### Using Latin Fonts For RTL Scripts

Applying a Latin typeface to Arabic or Hebrew produces broken shaping and poor vertical alignment, signaling that RTL was an afterthought.

### Discovering RTL After Launch

Treating RTL as a future task means the architecture, components, and assets were all built on LTR assumptions that are expensive to undo.

## Self-Check

- [ ] Layouts use logical, start and end, positioning rather than hardcoded left and right.
- [ ] Directional elements such as back arrows, progress, steppers, and chevrons were mirrored, while logos, media controls, and non-directional icons were not.
- [ ] Composite spacing inside buttons, fields, list items, and navigation flips correctly, not just the outer layout.
- [ ] Bidirectional mixed text was tested with realistic content, including parentheses and punctuation across direction switches.
- [ ] Charts, timelines, and progress indicators have a deliberate, consistent direction rule for RTL.
- [ ] Fonts fall back to quality RTL typefaces, and line heights accommodate taller scripts and diacritics.
- [ ] No element was mirrored or left fixed by accident; each was decided based on whether its meaning depends on direction.
- [ ] The layout was tested with real Arabic or Hebrew content, not only mirrored Latin placeholders.
- [ ] Alignment of icons, badges, avatars, and metadata follows the new reading direction within each component.
- [ ] RTL was considered during architecture and component design, not deferred to a post-launch fix.