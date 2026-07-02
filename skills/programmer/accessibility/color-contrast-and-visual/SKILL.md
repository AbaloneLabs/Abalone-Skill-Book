---
name: color_contrast_and_visual_accessibility.md
description: Use when the agent is choosing colors, contrast ratios, text sizes, focus indicators, status or error encodings, chart palettes, or visual distinctions in a UI; designing for low vision, color vision deficiency (color blindness), high-contrast or forced-color mode, glare or small-screen conditions; setting WCAG contrast targets; ensuring information is not conveyed by color alone; building visible focus states; respecting reduced motion for visual transitions; or reviewing whether text, icons, and UI components remain perceivable at the sizes and conditions real users encounter. Also covers the failure mode of a design that looks clear to a sighted designer on a calibrated monitor but excludes users with color blindness, low contrast sensitivity, or high-contrast-mode needs.
---

# Color Contrast And Visual Accessibility

Visual accessibility is the judgment of whether information encoded visually — text, icons, status colors, chart palettes, focus indicators, borders — can actually be perceived by users whose vision differs from the designer's. Designers work on calibrated monitors in good lighting with full color vision and normal contrast sensitivity. Real users do not. Roughly 1 in 12 men and 1 in 200 women have some form of color vision deficiency; many users have low vision or reduced contrast sensitivity; many encounter the interface in glare, on a dim screen, in high-contrast or forced-color mode, or at a size the designer never tested. A design that depends on a subtle color difference, a low-contrast gray-on-white, or a status indicated by color alone is invisible to a meaningful fraction of users — and the invisibility is silent, because the designer never experiences it.

Agents tend to under-invest here because the design looks clear to them, automated contrast checkers catch only the simplest text-on-solid-background cases, and the harm is invisible (the excluded users are not in the room). The judgment problem is deciding, for every piece of visually-encoded information, what second channel carries it for users who cannot perceive the first, whether the contrast meets the floor for the conditions users encounter, and whether focus and state are perceivable without color alone. This is not about avoiding color; it is about never making color the sole carrier of meaning, and about ensuring text and UI components are legible at the contrast and size users actually experience.

## Core Rules

### Meet WCAG Contrast Ratios As A Floor, And Check The Real Conditions

WCAG defines minimum contrast ratios, and they are a floor, not a target. The ratios:

- **4.5:1 for normal text** (below ~18pt / ~14pt bold). This is the minimum for body text against its background.
- **3:1 for large text** (~18pt+ / ~14pt+ bold) and for meaningful graphics and UI components (icon glyphs, input borders, focus indicators, chart elements that convey information). Large text and bold graphics are legible at lower contrast because their size compensates.
- **7:1 and 4.5:1 for the enhanced (AAA) levels**, where the audience includes users with significant low vision or where the stakes of misreading are high.

The discipline is to check contrast in the conditions users encounter, not the designer's ideal:

- **Check text over images and gradients, not only solid backgrounds.** A white headline over a photo may meet 4.5:1 in one area and fail where a light part of the image sits behind it. Add a scrim, overlay, or solid background panel to guarantee contrast everywhere the text appears, or check the worst-case pixel.
- **Check at the actual size and weight.** A color pair that passes at 18px may fail at 12px; a thin font needs more contrast than a bold one because the strokes are thinner. Test the smallest and lightest text actually used.
- **Check disabled and placeholder text.** These are often low-contrast by design, but if they must be read (a disabled button's label, a placeholder that conveys format), they still need to be legible — disabled state should be conveyed by more than contrast alone.
- **Account for the user's environment.** Glare, dim screens, and dirty or smudged displays reduce effective contrast. Designing to the floor in ideal conditions leaves no margin for real conditions.

Use a contrast checker (browser devtools, standalone tools) on the actual rendered colors, including over images. A color pair that "looks fine" may measure below the floor.

### Never Convey Information By Color Alone

Color is one encoding channel. Any state, category, or distinction shown only by a color change is invisible to users with color vision deficiency, users in high-contrast/forced-color mode, users on monochrome displays, and screen-reader users (who perceive no color at all). For every color-encoded meaning, provide a second channel:

- **Form errors.** A red border plus an icon, plus text ("Email is required"), plus `aria-invalid` and `aria-describedby` linking the message to the field. Never a red border alone.
- **Status indicators.** A green dot plus a checkmark icon, or "Active" text, not a green dot alone. A red dot plus an "Error" label, not red alone.
- **Links in text.** Underline or another differentiator, not color alone — many users cannot distinguish the link color from body text color, and underlining is the conventional signal.
- **Charts and data visualization.** Color plus shape, pattern, texture, or direct labels on each series. Do not rely on a color legend alone, because a color-blind user cannot map the legend swatches to the series if the colors are indistinguishable to them.
- **Required fields.** Text ("Required") or an icon with a text alternative, not only a red asterisk whose meaning must be inferred.

The test: imagine the interface in grayscale. If any distinction disappears, that distinction was color-only and needs a second channel. This is one of the most common and most easily fixed accessibility failures.

### Provide A Visible Focus Indicator That Survives Every State

Keyboard users (and sighted screen-reader users) navigate by focus, and they must be able to see where focus is at all times. The browser's default focus outline is usually visible; the common failure is removing it (`outline: none` or `outline: 0`) for aesthetics without providing an equally visible replacement, leaving keyboard users with no indication of their location.

- **Never remove focus indication without a deliberate, equally visible alternative.** If the default outline is removed, provide a `:focus-visible` style (a ring, a border, a background change) that is at least as visible. "Equally visible" means comparable contrast and size to the default outline, not a faint 1px shift.
- **Use `:focus-visible` over `:focus` for the indicator** so mouse users do not see a ring on every click while keyboard users always do. `:focus-visible` targets focus that comes from keyboard or non-pointing input.
- **Ensure the indicator has sufficient contrast against the background** (3:1 against adjacent colors per WCAG 2.4.13 Focus Appearance). A focus ring that is the same color as the button background is invisible.
- **Do not rely on a cursor change or a subtle shadow.** These are often imperceptible; use a clear outline, ring, or border change.

Focus indication is the keyboard user's equivalent of the mouse cursor. Removing it is the equivalent of hiding the cursor for mouse users.

### Design Charts And Data Visualization For Color-Blind Users

Data visualization is where color-only encoding is most common and most damaging, because charts rely on color to distinguish series, categories, and states. A color-blind user who cannot distinguish the red and green series in a line chart cannot read the chart at all.

- **Use a colorblind-safe palette.** Designed palettes (e.g., Okabe-Ito, ColorBrewer's colorblind-safe sets) choose colors distinguishable across the common color vision deficiencies. Avoid red-green as the sole distinction, since red-green color blindness is the most common.
- **Add a second channel per series: shape (different markers per line), pattern (hatching), or direct labels.** A line chart where each line also has a distinct marker shape (circle, square, triangle) is readable without color.
- **Label data directly on the chart** rather than relying solely on a color legend. Direct labels attach the identity to the data, removing the color-mapping step entirely.
- **Check the chart in a colorblind simulator** (browser devtools, standalone tools) for deuteranopia, protanopia, and tritanopia. If any series becomes indistinguishable, the encoding fails.

A chart that is unreadable by 8% of male users is not an accessible chart. The fix is usually a palette change plus a second channel, which costs little and removes the barrier.

### Support High-Contrast And Forced-Color Modes

Some users (notably Windows users with visual impairments) run their OS in high-contrast or forced-color mode, which remaps the page's colors to a limited system palette. Interfaces that rely on background colors, borders, or icons defined only by color can break — borders disappear, icons vanish, text becomes unreadable — because the forced-color mode overrides the authored colors.

- **Use semantic colors via CSS system color keywords** (`ButtonText`, `Canvas`, `Highlight`) where the design must adapt to forced-color mode, so the OS palette applies correctly.
- **Do not rely on background-color alone to convey boundaries.** A card distinguished only by a background tint loses its boundary when the tint is overridden; add a border or use a semantic container.
- **Ensure icons have sufficient weight and contrast** under forced-color mode, where thin or color-only icons may disappear. Provide a fallback (a border, a text label).
- **Test in the OS high-contrast modes** (Windows high contrast themes) to catch what breaks when the authored palette is replaced.

Forced-color mode is less common than color blindness but is essential for the users who depend on it, and it exposes designs that rely on color as a structural element rather than a semantic one.

### Size Text And Spacing For Legibility, And Support Resizing

Text must be legible at the sizes users encounter, and users must be able to resize it. The related failures: fixed tiny font sizes that cannot be enlarged, layouts that break when text is zoomed, and pinch-zoom disabled on mobile.

- **Use relative units (`rem`) for font sizing** so the text respects the user's root font-size preference and browser zoom. Fixed `px` sizes can block users who need larger text.
- **Do not disable pinch-zoom on mobile** (`user-scalable=no` or `maximum-scale=1` in the viewport meta). Pinch-zoom is an essential accessibility tool for low-vision users; disabling it is an accessibility failure with little real benefit.
- **Verify the layout survives 200% text zoom** (WCAG 1.4.10 Reflow and 1.4.4 Resize Text). Content must not be clipped, overlapping, or lost at 200%; horizontal scrolling at 320px width must not be required.
- **Choose base font sizes with a floor.** Body text below ~16px is hard to read for many users; use larger defaults and let users scale up.

Text that cannot be resized or that breaks the layout when zoomed excludes low-vision users who depend on larger text.

### Respect Reduced Motion For Visual Transitions

Motion is a visual encoding, and some users cannot or should not perceive it: vestibular disorders, motion sensitivity, migraines, and photo-sensitivity make certain motion physically harmful. This is covered in depth by the motion accessibility guidance, but the visual-accessibility dimension is that motion must never be the sole carrier of information, and non-essential motion must respect `prefers-reduced-motion`.

- **Pair every motion-conveyed state with a non-motion channel** (text, icon, announcement). An error indicated only by a shake is invisible to users who disable motion.
- **Provide a reduced-motion path** that removes or replaces non-essential animation (a fade becomes a cut, a slide becomes an appearance).
- **Never autoplay large-scale, parallax, or flashing motion.** These are the highest-risk motion for vestibular harm; make them opt-in or omit them.

## Common Traps

### Gray-On-White Text That "Looks Elegant"

Using low-contrast gray text on white for a minimalist aesthetic, measuring below 4.5:1. It looks clean to the designer and is illegible to users with reduced contrast sensitivity, in glare, or on dim screens. Meet the contrast floor; use higher contrast for body text.

### Red-Green Status With No Second Channel

"Active" shown as a green dot and "Error" as a red dot, with no icon or text. Red-green color-blind users cannot distinguish them. Always pair status color with an icon, shape, or text label.

### Removing The Focus Outline For Aesthetics

Setting `outline: none` globally or on `:focus` to make the design "clean," with no replacement. Keyboard users lose their only location indicator. Provide an equally visible `:focus-visible` alternative; never remove focus indication without one.

### Chart With Color-Only Series And A Color Legend

A line chart with red and green lines distinguished only by a color legend. Color-blind users cannot map the legend to the lines. Use a colorblind-safe palette plus distinct markers or direct labels.

### Text Over An Image Without A Scrim

A white headline over a photo, where part of the photo is light and the text disappears against it. Check contrast across the whole image area; add a scrim, overlay, or solid background panel to guarantee contrast everywhere.

### Disabled State Conveyed Only By Lower Contrast

A disabled button distinguished only by lighter (lower-contrast) text, which is then illegible — and the disabled state is invisible to users who cannot perceive the contrast difference. Convey disabled state via text, an icon, or `aria-disabled`, not contrast alone.

### Fixed Pixel Font Sizes And Disabled Zoom

Body text in `px` that ignores user font-size preferences, plus `user-scalable=no` on mobile blocking pinch-zoom. Low-vision users cannot enlarge the text. Use `rem` and allow zoom; verify the layout survives 200%.

### Ignoring Forced-Color / High-Contrast Mode

A design whose structure depends on background tints and color-only borders, which disappear under forced-color mode. Test in Windows high-contrast themes; use semantic colors and borders that survive the palette override.

### Motion As The Only Signal For A State

An error shown only by a shake animation, or "saved" shown only by a brief glow. Users with reduced motion (and screen-reader users) miss it entirely. Pair motion with text, icon, or announcement.

## Self-Check

- [ ] Text meets WCAG contrast ratios (4.5:1 normal, 3:1 large/UI components, higher where the audience needs it), checked at the actual sizes and weights used, including over images and gradients (with scrims/overlays where needed) and not only on solid backgrounds.
- [ ] No information is conveyed by color alone — errors, statuses, links, required fields, and chart series all have a second channel (text, icon, shape, pattern, underline, direct label) — verified by imagining the interface in grayscale.
- [ ] Focus indicators are visible at all times: no `:focus`/`:focus-visible` outline removed without an equally visible replacement, the indicator has sufficient contrast (3:1 against adjacent colors), and `:focus-visible` is used so keyboard users always see focus while mouse users are not distracted.
- [ ] Charts and data visualizations use a colorblind-safe palette plus a second channel per series (distinct markers, patterns, or direct labels), verified in a colorblind simulator for deuteranopia/protanopia/tritanopia.
- [ ] The design survives high-contrast / forced-color mode: semantic colors or borders are used so boundaries and icons do not disappear when the authored palette is overridden, verified in Windows high-contrast themes.
- [ ] Text uses relative units (`rem`), pinch-zoom is not disabled on mobile, and the layout survives 200% zoom and 320px reflow without clipping, overlap, or required horizontal scrolling.
- [ ] Motion is never the sole carrier of information, non-essential motion respects `prefers-reduced-motion`, and no autoplay parallax or flashing motion is present.
- [ ] Disabled and placeholder states are conveyed by more than contrast alone (text, icon, `aria-disabled`) so the state is perceivable to users who cannot detect the contrast difference.
- [ ] Verification went beyond automated contrast checkers (which catch only simple text-on-solid cases) to include worst-case conditions: text over images, small/light text, high-contrast mode, grayscale, and real-screen glare.
