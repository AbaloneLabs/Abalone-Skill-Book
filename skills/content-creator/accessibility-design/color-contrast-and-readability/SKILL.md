---
name: color_contrast_and_readability.md
description: Use when the agent is choosing text and background colors, checking contrast for accessibility, designing readable layouts for low-vision users, ensuring legibility on small screens and bright displays, or deciding whether color choices exclude part of the audience.
---

# Color Contrast And Readability

Color contrast is one of the most consequential and most neglected accessibility decisions in content creation. It determines whether text can be read by people with low vision, on dim or bright screens, in glare, and at small sizes. The judgment problem is that contrast is easy to get wrong because the human eye adapts and the design tool's preview often looks more legible than the real-world result. Creators fail by choosing colors for aesthetics alone, by trusting their eyes over measured contrast, by using color as the only carrier of meaning, and by designing only for ideal viewing conditions. The result is content that quietly excludes a meaningful share of the audience and often fails accessibility standards.

The harm is silent exclusion. Unlike a broken link, low contrast does not error; it simply makes content unusable for those affected. This skill helps the agent choose and verify color combinations that are genuinely readable for the whole audience.

## Core Rules

### Measure Contrast Rather Than Trust The Eye

The eye adapts to make low-contrast text feel readable in the design tool, then the same text fails on a phone in sunlight or for a low-vision reader. Contrast must be measured against an objective threshold, not judged by feel.

Measure every text-and-background pairing:

- use a contrast checker to compute the ratio;
- compare against the relevant standard threshold for normal and large text;
- check both light-on-dark and dark-on-light pairings;
- re-check after any color adjustment.

If a pairing does not meet threshold, it does not ship, regardless of how it looks on the creator's calibrated screen.

### Meet The Relevant Contrast Thresholds

Accessibility standards define minimum contrast ratios, and these vary by text size and weight because large or bold text is legible at lower contrast than small thin text. Designing to the stricter threshold is safer because it covers more situations.

Aim for robust contrast:

- at least the standard ratio for normal body text;
- the lower large-text threshold only for genuinely large or bold text;
- higher contrast where content is critical or viewed in poor conditions;
- no reliance on the minimum alone when a comfortable margin is achievable.

Designing to the bare minimum leaves no room for real-world degradation. A margin above threshold is the practical target.

### Do Not Rely On Color Alone To Convey Meaning

Color blindness and low vision mean some users cannot perceive certain colors or distinctions. If meaning depends only on color, those users lose the information entirely.

Pair color with redundant cues:

- add labels, icons, or text alongside color coding;
- use patterns, shapes, or underlines in addition to hue;
- ensure error and success states are distinguishable without color;
- test the design in grayscale to confirm meaning survives.

A chart, form, or status indicator that only works in full color is inaccessible by construction.

### Account For Real-World Viewing Conditions

Content is not viewed in the design tool. It is viewed on phones in sunlight, on dim screens at night, on aging displays, and by tired eyes. Contrast that barely passes in ideal conditions fails in reality.

Build in real-world margin:

- prefer higher contrast than the minimum for content viewed outdoors or at small size;
- test on actual devices, not only the authoring screen;
- consider dark-mode and light-mode variants separately;
- account for anti-glare coatings and screen brightness settings.

A contrast ratio that works on a calibrated monitor may be unreadable on a cheap phone in sunlight.

### Check Small Text And UI Elements Explicitly

Small text, captions, footnotes, and UI labels are the most common contrast failures because their size already challenges legibility. These elements need the strictest contrast, not the loosest.

Scrutinize small elements:

- captions, disclaimers, and footnotes meet full contrast for body text;
- buttons and links are distinguishable from surrounding text;
- placeholders and disabled states remain readable despite lower emphasis;
- chart axis labels and legends survive reduction.

Small text is where contrast failures cluster. Audit it specifically.

### Design Readable Layout, Not Just Readable Color

Readability is not only contrast. Line length, line height, font choice, and text size all interact with contrast to determine whether text is actually readable. Strong contrast cannot rescue a layout that is otherwise hard to read.

Support readability holistically:

- keep body text at a comfortable size for the surface;
- set line length and leading so the eye can track lines;
- choose fonts with open, distinct letterforms for small sizes;
- avoid all-caps for long runs, which reduces word-shape cues.

Contrast is necessary but not sufficient. Layout completes the readability picture.

### Provide Dark And Light Mode Pairings Deliberately

When content appears in both light and dark modes, each mode needs its own verified color pairings. A color that passes on white may fail on dark, and inverted text often needs different sizing or weight to stay readable.

Handle both modes:

- define and test separate palettes for light and dark;
- avoid simply inverting colors, which often breaks contrast or aesthetics;
- ensure images and brand colors remain legible in both modes;
- verify that links, borders, and dividers are visible in each mode.

Inversion is not a design strategy. Each mode deserves its own contrast-checked system.

### Test With Tools And Real Users

Automated contrast checkers catch numeric failures but miss context, such as text over images or busy backgrounds. Real users, especially those with low vision or color vision deficiencies, reveal problems tools cannot.

Combine verification methods:

- run automated contrast checks on all pairings;
- inspect text overlaid on images or gradients manually;
- view the design in grayscale to test color-only meaning;
- seek feedback from users with relevant visual conditions where possible.

Tools are the floor, not the ceiling. Real-world and real-user testing catches what numbers miss.

## Common Traps

### Trusting The Eye Over Measured Contrast

The eye adapts and the design screen lies. Measure contrast against threshold.

### Designing To The Bare Minimum

Minimum contrast leaves no margin for real-world degradation. Aim above it.

### Color As The Sole Carrier Of Meaning

Color-only distinctions exclude colorblind users. Pair color with labels, icons, or patterns.

### Ignoring Small Text And UI Elements

Small text is where contrast fails most. Audit captions, labels, and legends explicitly.

### Only Testing In Ideal Conditions

Contrast that passes on a calibrated screen fails in sunlight or on cheap devices. Test realistically.

### Inverting Colors For Dark Mode

Simple inversion breaks contrast and aesthetics. Design and test each mode separately.

### Skipping Real-User Feedback

Tools miss context and lived experience. Combine automated checks with real users.

## Self-Check

Before approving color and readability choices, verify:

- Every text-and-background pairing was measured with a contrast checker, not judged by eye.
- All pairings meet the relevant threshold for their text size and weight.
- Critical and small text exceeds the minimum with a comfortable margin.
- No meaning depends on color alone; grayscale viewing preserves all information.
- Error, success, and status states are distinguishable without perceiving hue.
- Small text, captions, footnotes, and UI labels were checked explicitly.
- The design was tested on real devices and in realistic viewing conditions, not only the authoring screen.
- Body text size, line length, leading, and font choice support readability alongside contrast.
- Light and dark mode each have separately designed and tested color pairings.
- Where possible, feedback was sought from users with low vision or color vision deficiencies.
