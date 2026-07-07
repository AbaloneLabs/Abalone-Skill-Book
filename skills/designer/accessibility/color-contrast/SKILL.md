---
name: color_contrast.md
description: Use when the agent is ensuring color contrast meets accessibility standards, calculating contrast ratios for text and meaningful graphics, designing for low vision and color blindness, handling contrast in dark mode and theme variants, choosing colors that pass WCAG AA and AAA, or verifying that meaning is not conveyed by color alone.
---

# Color Contrast

Color contrast is the difference in luminance between foreground and background that determines whether content is legible. It looks like a design preference, but it is really an accessibility requirement with measurable thresholds: below a certain ratio, text and meaning become invisible to users with low vision, in bright ambient light, or on poor screens. Agents tend to treat contrast as an aesthetic concern, trust their eyes in ideal conditions, or pass a contrast check on body text while ignoring the dozens of other surfaces where contrast fails. The harm is content that looks fine to the designer and is unreadable to a significant portion of users.

Use this skill before finalizing color choices, verifying contrast, or relying on color to convey meaning. The goal is to prevent the agent from shipping color combinations that fail accessibility, from checking only the obvious cases, or from assuming their perception represents all users.

## Core Rules

### Meet WCAG Contrast Thresholds For Every Text And Graphic Combination

Contrast is not judged by appearance but by a measurable ratio, and the WCAG standards define the thresholds that content must meet. Normal text requires at least 4.5:1 for AA; large text (eighteen point or fourteen point bold) requires 3:1. AAA raises these to 7:1 and 4.5:1 respectively. Meaningful graphics and interface components require 3:1 against adjacent colors. These are not suggestions; they are the baseline for legibility.

Meet the thresholds everywhere:

- target WCAG AA at minimum: 4.5:1 for normal text, 3:1 for large text;
- consider AAA (7:1 for normal text) for critical reading content like long-form articles;
- ensure meaningful graphics (icons, charts, data visualizations) meet 3:1 against their background;
- verify interface components (button borders, focus indicators, form field outlines) meet 3:1.

A design that passes body text but fails on captions, placeholders, disabled states, or chart lines is not accessible. Every combination matters, not just the most prominent.

### Check Contrast Computationally, Not By Eye

Human perception of contrast is unreliable. Designers, especially those with normal vision reviewing on calibrated screens in good lighting, consistently overestimate contrast. The only trustworthy method is computing the ratio from the actual foreground and background colors using the WCAG relative luminance formula.

Verify computationally:

- use a contrast checker that computes the exact ratio from hex or RGB values;
- check every text-on-background combination in the system, not just a few;
- account for opacity and transparency, which change the effective color;
- re-check whenever colors change, since a palette update can break previously passing combinations.

A design that "looks fine" can fail the ratio badly. Trusting the eye is the most common cause of contrast failures reaching production.

### Never Rely On Color Alone To Convey Meaning

Contrast ensures color is visible, but visibility is not enough if meaning depends on color that some users cannot perceive. Color blindness affects roughly one in twelve men and one in two hundred women; for them, red and green are indistinguishable, and meaning encoded only in color is invisible. A red error border with no icon or text fails for these users, no matter how high its contrast.

Convey meaning redundantly:

- pair color with text labels, icons, or patterns so meaning survives without color;
- in charts and data visualizations, use shape, pattern, or direct labeling in addition to color;
- in forms, do not rely on red borders alone for errors; include an icon and text message;
- test designs in grayscale to confirm meaning is still conveyed.

Color should enhance meaning, not carry it alone. Redundant encoding ensures all users understand state, category, and emphasis.

### Design For Low Vision Beyond Color Blindness

Color blindness is the most discussed contrast concern, but low vision is broader and more common with age. Users with low vision need higher contrast, larger text, and the ability to override colors. A design that meets minimum thresholds but cannot adapt to user preferences still fails these users.

Accommodate low vision:

- exceed minimum contrast where feasible, since higher ratios help users with low vision;
- support text resizing and zoom without breaking layout;
- respect user color preferences and forced-color modes (like Windows high contrast);
- avoid thin text weights and small font sizes that compound low vision difficulties.

Meeting the minimum is the floor. Designing for the range of low vision means building adaptability and margin into the contrast and type system.

### Handle Contrast In Dark Mode And Theme Variants

Dark mode is not a simple inversion of light mode, and contrast behaves differently against dark backgrounds. Colors that pass on white may fail on black, and the perception of contrast shifts: the same ratio can feel more or less legible depending on the surrounding luminance. Each theme variant must be checked independently.

Design contrast per theme:

- define and verify contrast for every combination in both light and dark themes;
- adjust colors for dark mode rather than inverting, since inversion often produces muddy or glowing results;
- reduce saturation in dark mode, where colors appear more vivid and can cause perceived vibration;
- test that semantic colors (success, error, warning) remain distinguishable and meet contrast in both themes.

A palette that passes in light mode but fails in dark mode ships an inaccessible dark theme. Theme variants are separate contrast problems that must each be solved.

### Account For Contrast On Images And Variable Backgrounds

Text on solid colors is straightforward to check; text on images, gradients, or variable backgrounds is not. When text overlays an image, the background luminance varies across the text, and a ratio computed against one point may fail at another. This is a common source of contrast failures that automated tools miss.

Handle variable backgrounds:

- avoid placing text directly on complex images without a treatment that ensures contrast;
- use overlays (scrim, gradient, solid panel) behind text to create a consistent, checkable background;
- where overlays are not possible, add text shadows or outlines to maintain legibility;
- check contrast at multiple points across the text, not just one.

Text on an untreated image is a contrast gamble. The safe approach is to create a defined background whose contrast can be verified.

### Verify Focus Indicators Have Sufficient Contrast

Focus indicators are how keyboard and assistive technology users know where they are, and they are frequently too faint to perceive. A focus outline that is the same color as its background, or too thin, effectively disappears, leaving keyboard users unable to navigate. Focus indicator contrast has its own requirement and is often neglected.

Ensure focus visibility:

- verify focus indicators meet at least 3:1 contrast against adjacent colors;
- make focus indicators thick enough to be noticeable (typically at least two pixels);
- ensure the focus indicator contrasts with the element it outlines, not just the page background;
- never remove focus outlines without providing an equally visible custom alternative.

Removing the default focus outline without a replacement is a common, severe accessibility failure. Focus must always be visible to those who depend on it.

### Test Under Real Conditions

Contrast that passes in a design tool can fail in the real world: bright sunlight washes out screens, low-quality displays render colors differently, and screen brightness settings vary. Testing under ideal conditions hides failures that users experience daily.

Test realistically:

- view the design on actual target devices, including low-end screens;
- test in bright ambient light where contrast perception drops;
- test at the screen brightness users actually use, not maximum;
- consider anti-glare coatings and screen films that reduce contrast.

A contrast ratio that passes on paper can still be unreadable on a dim phone in sunlight. Real conditions are the true test of legibility.

## Common Traps

### Judging Contrast By Eye

Designers consistently overestimate contrast on calibrated screens; compute the exact ratio for every combination.

### Checking Only Body Text

Passing the main text while failing captions, placeholders, disabled states, and chart lines leaves the design inaccessible; check every combination.

### Meaning Conveyed By Color Alone

Red error borders or color-only chart legends are invisible to color-blind users; pair color with text, icon, or pattern.

### Ignoring Low Vision Beyond Color Blindness

Meeting minimums without supporting resize, zoom, and forced-color modes fails users with low vision; build adaptability and margin.

### Treating Dark Mode As Inversion

Colors that pass in light mode often fail in dark mode; define and verify contrast per theme, adjusting rather than inverting.

### Text On Untreated Images

Variable backgrounds make contrast uncheckable and often insufficient; use overlays or treatments to create a consistent background.

### Invisible Focus Indicators

Faint or removed focus outlines leave keyboard users unable to navigate; ensure focus meets 3:1 and is thick enough to notice.

### Testing Only In Ideal Conditions

Contrast that passes in a design tool fails in sunlight, on low-end screens, and at real brightness; test real conditions.

## Self-Check

- [ ] Every text-on-background combination meets WCAG AA (4.5:1 normal, 3:1 large), with AAA considered for critical reading.
- [ ] Contrast ratios are computed from actual color values, not judged by eye, including opacity and transparency effects.
- [ ] Meaning is never conveyed by color alone; color is paired with text, icon, pattern, or direct labeling.
- [ ] The design supports low vision through higher-than-minimum contrast, text resizing, zoom, and forced-color modes.
- [ ] Light and dark theme variants each have verified contrast, with colors adjusted rather than inverted for dark mode.
- [ ] Text on images and variable backgrounds uses overlays or treatments to ensure checkable, sufficient contrast.
- [ ] Focus indicators meet at least 3:1 contrast against adjacent colors, are thick enough to notice, and are never removed without a visible replacement.
- [ ] Contrast was tested on target devices, in bright ambient light, and at real screen brightness, not only in the design tool.
