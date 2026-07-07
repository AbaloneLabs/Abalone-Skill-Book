---
name: color_contrast_and_perception.md
description: Use when the agent is choosing or reviewing color contrast for text, icons, borders, focus indicators, data visualizations, or text over images, accounting for color vision deficiencies, low vision, ambient lighting, display variation, perceived versus measured contrast, and accessibility thresholds.
---

# Color Contrast And Perception

Measured contrast is not the same as perceived contrast. A pair of colors that passes a contrast formula can still be unreadable to a user with low vision, indistinguishable to a user with a color vision deficiency, washed out on a glossy screen in sunlight, or defeated by a subtle gradient behind the text. Designers often treat contrast as a number to clear and then move on, when the real question is whether the right users can read the content in the conditions where they actually use it.

Use this skill before finalizing text colors, icon colors, borders, focus indicators, data visualization palettes, text over images, or any color pair that carries meaning. The goal is to prevent the agent from passing a contrast checker while shipping unreadable content, relying on color distinctions that vanish for color-blind users, or testing only under ideal conditions that real users never experience.

## Core Rules

### Target Contrast For The Content That Actually Carries Meaning

Contrast requirements scale with the importance and size of the content. Body text, which users read at length, needs the strongest contrast. But the riskiest elements are often the small, operational ones: labels, captions, timestamps, placeholder text, metadata, helper text, and the subtle borders that define interactive regions. These carry the information users act on, and they are the most common to fail.

Check contrast on:

- body text and headings;
- small labels, captions, and metadata;
- placeholder and helper text;
- disabled-looking but active controls;
- icons that carry meaning;
- borders that define interactive regions;
- focus indicators;
- text over images, gradients, and branded backgrounds.

A design that clears body text contrast while failing labels has solved the easy case and missed the important one.

### Distinguish Measured Contrast From Perceived Contrast

Contrast formulas measure luminance difference, but perception depends on more: font size, font weight, text length, surrounding color, screen brightness, and ambient light. Thin, small, light-weight text needs more contrast than the formula suggests to read comfortably; bold, large text can read well slightly below a strict threshold. A pair that measures adequately can feel strained in practice, and a pair that measures borderline can be fine for short, bold content.

Use measured contrast as a floor, not a finish line. Then evaluate perceived readability with real content at real sizes, especially for long-form text where eye strain accumulates.

### Design For Color Vision Deficiencies, Not Only Luminance

Color-blindness is not rare, and it is not the only perceptual difference. Roughly one in twelve men and one in two hundred women have some form of color vision deficiency. The most common failures are distinctions that rely on red versus green, or on subtle hue differences, which collapse for these users. A chart whose categories are red and green, a form whose errors are marked only by a red border, or a status indicator using only color all become meaningless.

For any color-coded distinction:

- verify the pair is distinguishable under deuteranopia, protanopia, and tritanopia simulation;
- add a second cue such as pattern, shape, label, or icon;
- avoid relying on red-green distinctions for critical meaning;
- prefer lightness differences, which survive most color vision deficiencies, over hue differences alone.

### Account For Ambient Light And Display Variation

Contrast that works in a dim studio fails in sunlight, and contrast that works on a calibrated monitor fails on a cheap laptop or a phone at low brightness. Glossy screens reflect ambient light, effectively raising the black point and reducing contrast. OLED displays render dark backgrounds very dark, which can cause halation around light text. Users frequently dim screens to save battery or reduce glare, further cutting contrast.

Stress-test critical text under:

- bright ambient light and screen reflections;
- low screen brightness settings;
- older or lower-quality displays;
- night-shift or blue-light filtering, which warms colors and shifts perceived contrast.

### Treat Text Over Images As A Special Case

Text over photography, gradients, or branded backgrounds is one of the most common contrast failures, because the background varies unpredictably. A overlay that works over a light area of an image fails over a dark area, and vice versa. Designing for the average part of the image leaves text unreadable over the extremes.

Use stable techniques:

- a consistent scrim or overlay behind the text, dark enough to guarantee contrast regardless of the image;
- a solid text container rather than text directly on the image;
- text shadows only as reinforcement, never as the sole guarantee;
- testing against the lightest and darkest realistic images, not a favorable example.

### Give Focus Indicators Real Contrast

Focus indicators are critical for keyboard users and are frequently under-designed. A thin outline in a color close to its background is invisible precisely when it needs to be seen. Focus indicators need contrast against both the element they outline and the background behind it, and they need enough thickness to be located quickly.

Do not remove focus outlines for cleanliness. If the default outline is visually heavy, design a custom indicator that is clearly visible, not a faint one that pretends to solve the problem.

### Verify Data Visualization Palettes For Distinguishability

Charts and maps rely on color to distinguish categories, and these palettes fail more often than text contrast. Sequential palettes can collapse for color-blind users; categorical palettes with too many similar hues become indistinguishable; red-green pairings vanish for a large share of the audience.

Build data palettes that:

- combine hue and lightness variation so categories differ in both;
- keep categorical counts low, since beyond a handful, color cannot carry the distinction;
- offer patterns, direct labels, or legends close to the data;
- are checked under color vision deficiency simulation, not only in full color.

## Common Traps

### Clearing Body Text Contrast And Stopping

The labels, captions, and helper text that carry operational meaning are left unverified and often fail.

### Trusting The Number Over The Experience

A pair that passes a formula can still strain the eyes in long reading, especially when thin or small.

### Red-Green Distinctions For Critical Meaning

The most common color-blindness makes red and green indistinguishable, collapsing charts, statuses, and errors that rely on the pair.

### Text Directly On Variable Images

Without a stable scrim or container, text becomes unreadable over part of the image, usually the part the designer did not test.

### Removing Focus Outlines For Aesthetics

A cleaner screenshot is traded for an interface that keyboard users cannot operate, because focus disappears.

### Designing Only For The Calibrated Monitor

Contrast that passes in the studio fails in sunlight, on dimmed screens, and on cheaper displays where users actually read.

### Too Many Similar Categories In A Chart

Categorical color palettes with many close hues become indistinguishable, defeating the purpose of color-coding.

## Self-Check

- [ ] Contrast was checked on labels, captions, metadata, placeholders, borders, icons, and focus indicators, not only body text.
- [ ] Measured contrast was treated as a floor, and perceived readability was verified with real content at real sizes.
- [ ] Color-coded distinctions were verified under deuteranopia, protanopia, and tritanopia simulation, with a second non-color cue added.
- [ ] No critical meaning relies on a red-green distinction alone.
- [ ] Critical text was stress-tested under bright ambient light, low brightness, older displays, and blue-light filtering.
- [ ] Text over images uses a stable scrim, overlay, or container that guarantees contrast over the lightest and darkest realistic images.
- [ ] Focus indicators have strong contrast against both the element and the background, and were not removed for visual cleanliness.
- [ ] Data visualization palettes combine hue and lightness variation, keep categorical counts low, and were checked for color-blind safety.
- [ ] Long-form text was evaluated for reading strain, not only for passing a threshold.
- [ ] The design was reviewed in real conditions, not only on a calibrated monitor in controlled lighting.