---
name: mobile_accessibility_and_inclusive_design.md
description: Use when the agent is building or reviewing a mobile interface for accessibility and inclusive use; supporting dynamic type and font scaling; implementing VoiceOver, TalkBack, or switch-control screen reader support; setting accessibility labels, hints, traits, and roles; ensuring color contrast and not conveying meaning by color alone; sizing touch targets for motor accessibility; respecting reduce-motion, reduce transparency, and bold-text preferences; building accessible custom controls; or deciding whether an interface is genuinely usable by people with visual, motor, cognitive, or hearing differences. Also covers the failure mode of fixed font sizes that clip or overlap at large text, custom controls with no accessibility semantics, animations that ignore reduce-motion, low-contrast text, and gestures or interactions that exclude users of assistive technology.
---

# Mobile Accessibility And Inclusive Design

Accessibility on mobile is the judgment of whether a person who does not see, hear, move, or process the interface the way the designer does can still perceive, operate, and understand it. It is not a checklist added at the end; it is a property of the design that is determined early by the choices of text sizing, control semantics, color, motion, and interaction model. The judgment problem is that the default design is built for the designer's own senses and motor abilities — good vision, precise touch, tolerance for motion — and every default that encodes that assumption excludes someone else: a fixed font size excludes a low-vision user who scales text; a custom control with no accessibility label is invisible to a screen reader; a low-contrast button is unusable in sunlight or with color blindness; a parallax animation triggers vestibular discomfort for someone with motion sensitivity. The discipline is to treat accessibility as a design constraint from the start, to use the platform's semantic accessibility APIs (not visual hacks), to let the user's system preferences (text size, reduce motion, contrast) actually change the interface, and to test with the assistive technologies real users depend on.

Agents tend to treat accessibility as a cosmetic compliance step because the interface looks fine to them and the automated scanner reports few issues. The harm appears as text that clips or overlaps when the user sets a large dynamic-type size, as custom controls that screen readers announce as empty or wrong, as gestures that cannot be performed without two working hands or precise touch, as meaning conveyed only by color that a color-blind user cannot perceive, and as animations that make motion-sensitive users physically ill. The judgment is to design for the range of human ability, to expose correct semantics to assistive tech, to honor the user's accessibility settings as first-class inputs, and to verify with the tools and settings real users use — not with a sighted, motor-typical developer's eyeball check. Accessibility is whether the interface works for the person who is not you.

## Core Rules

### Let The User's Text Size Actually Change The Interface

Dynamic type / font scaling is one of the most impactful and most broken accessibility features: users with low vision set a large system text size and expect the entire interface to scale. The failure is not that the text fails to grow, but that the layout around it was built for a fixed size and breaks — text clips, overlaps, truncates to nothing, or pushes the primary action off-screen. Designing for dynamic type means designing layouts that reflow gracefully across the full range of sizes, not picking a font and hoping.

- **Use semantic, scalable text styles.** On iOS use the text styles (Body, Headline, etc.) that scale with dynamic type; on Android use scale-independent pixels (sp) and the theme text appearances. Do not hardcode point sizes or pixels for text.
- **Design and test across the full size range, not the default.** Lay out and verify at the largest accessibility size (iOS accessibility extra-extra-large, Android largest), where clipping and overlap appear; if it works at the extremes, it works in the middle.
- **Let layouts reflow, not clip.** Use wrapping, flexible containers, and scroll where needed; avoid fixed heights and single-line truncation that hides text at large sizes. A label that becomes "…" at large text is a broken label.
- **Do not cap or override the user's size.** Capping font scaling (a maximum text size) removes the feature for the users who need it most; if the layout breaks, fix the layout, not the font.

### Expose Correct Semantics To Screen Readers And Assistive Tech

VoiceOver (iOS) and TalkBack (Android) do not read pixels; they read the accessibility tree — the labels, roles, traits, and states the app exposes. A control that looks like a button but is exposed as plain text is invisible or meaningless to a screen reader; a decorative image with no label is read as "image"; a custom switch with no role is announced wrong. The judgment is to build with semantic components and to set accessibility metadata deliberately, so the assistive tech perceives what the sighted user perceives.

- **Prefer standard, semantic components.** Native buttons, switches, and text fields carry correct roles and traits for free; custom-drawn controls do not, and must have their accessibility semantics set manually and correctly.
- **Set concise, meaningful accessibility labels.** Each interactive element needs a label that conveys its purpose without the surrounding visual context ("Delete," not "trash icon"); avoid redundant prefixes ("button") the trait already announces.
- **Group and order deliberately.** Use accessibility containers/elements to group related controls so the reader navigates logically, and ensure the reading order matches the visual/logical order; a tab order that jumps randomly is unusable.
- **Convey state, not just identity.** A toggle's on/off state, a button's selected/disabled state, and a heading's level must be exposed via traits and state so the user understands the current state, not just what the control is.
- **Hide purely decorative content from the reader.** Decorative images and dividers should be marked not important for accessibility so they are skipped, reducing noise.

### Respect Reduce-Motion, Contrast, And Other Accessibility Preferences

The user's accessibility settings are inputs the interface should respond to, not ignore. Reduce Motion, Reduce Transparency, Increase Contrast, Bold Text, and color-inversion each signal that the default presentation harms this user, and the interface should adapt.

- **Honor Reduce Motion.** When the user enables reduce motion, replace parallax, screen transitions, zooming, and auto-playing animation with fades or no motion; do not simply reduce the duration of a vestibular-triggering effect.
- **Honor contrast and transparency preferences.** When Increase Contrast or Reduce Transparency is on, increase border and text contrast and remove frosted/blur effects that reduce legibility; do not render a low-contrast overlay the user has explicitly asked to avoid.
- **Do not convey meaning by color alone.** Status, errors, and required fields indicated only by color are invisible to color-blind users and to high-contrast mode; pair color with text, icons, or shape.
- **Support bold text and smart inversion.** Ensure the layout survives bold text (heavier glyphs need room) and that smart color inversion does not break images, icons, or branded colors.

### Ensure Color Contrast And Legible Text

Insufficient contrast is one of the most common and most consequential accessibility failures: text that is readable to the designer is unreadable in sunlight, on a dim screen, or to a low-vision user. Contrast is measurable, not a matter of taste.

- **Meet contrast thresholds for text.** Aim for the established ratios (roughly 4.5:1 for normal text, 3:1 for large text) as a floor, and verify with a contrast checker against the actual background, including over images and gradients.
- **Account for real conditions.** Contrast that passes in a dark theme on a calibrated monitor can fail in bright sunlight on a dim auto-brightness screen; design for the worst realistic viewing condition.
- **Make non-text contrast sufficient too.** Icons, chart elements, focus indicators, and the borders of inputs need adequate contrast against their background to be perceivable; a 1px gray border on white is not enough.

### Size And Space Interactions For Motor Accessibility

Not every user can perform a precise tap, a multi-finger gesture, or a small swipe. Motor accessibility means targets are large enough, spaced enough, and reachable, and that gestures have accessible alternatives.

- **Provide adequate touch-target size and spacing.** Use the platform minimums (roughly 44×44pt iOS, 48×48dp Android) with spacing between adjacent targets, so a user with tremor or imprecise touch can hit the right one.
- **Offer alternatives to complex gestures.** Multi-finger, long-press, or precision-swipe gestures should have a tap or switch-control equivalent; a feature reachable only by a custom gesture excludes users of switch control and assistive touch.
- **Do not require two hands or timing.** Interactions that require simultaneous touches, fast double-taps, or holding-and-dragging exclude users with motor limitations; provide simpler alternatives.

### Verify With Real Assistive Technology, Not A Scanner

Automated accessibility scanners catch a fraction of issues and miss the ones that matter most — a screen reader announcing the wrong thing, a layout that breaks at large text, a gesture with no alternative. Verification must use the actual tools users use.

- **Navigate the entire flow with VoiceOver / TalkBack.** Turn the screen reader on and complete the core task by audio alone; this exposes unlabeled controls, wrong reading order, and trapped focus that a scanner cannot detect.
- **Test at the largest text size and in high contrast.** Verify layouts reflow without clipping and that contrast holds; these settings surface the failures invisible at defaults.
- **Test with reduce motion and switch control.** Confirm animations adapt and that every gesture has an accessible path; a flow that is unreachable by switch control is not accessible.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: which disabilities and settings were designed for, what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Fixed Font Sizes That Clip Or Overlap At Large Text

Hardcoding text in points or pixels, or building fixed-height containers, so that when the user sets a large dynamic-type size the text clips, overlaps, truncates to nothing, or pushes the primary action off-screen. Use semantic scalable text styles; design and test across the full size range; let layouts reflow.

### Custom Controls With No Accessibility Semantics

A custom-drawn button, switch, or chart exposed as a plain view with no label, role, or trait, so a screen reader announces it as empty or wrong. Prefer standard semantic components; set accessibility labels, traits, and state explicitly on custom controls.

### Meaning Conveyed By Color Alone

Errors shown only in red, status indicated only by a colored dot, or required fields marked only by hue, invisible to color-blind users and in high-contrast mode. Pair color with text, icons, or shape so meaning survives without color.

### Animations That Ignore Reduce Motion

Parallax, zooming transitions, and auto-playing motion that the user has explicitly asked to reduce, triggering vestibular discomfort or nausea. Honor the reduce-motion preference by replacing motion with fades or static transitions, not by merely shortening it.

### Low-Contrast Text That Passes The Designer's Eye

Gray-on-white text or a thin border that looks fine to the designer but fails contrast ratios and is unreadable in sunlight or to a low-vision user. Meet contrast thresholds as a floor; verify with a checker against the real background, including over images.

### Gestures And Interactions With No Accessible Alternative

A feature reachable only by a multi-finger gesture, a precise swipe, or a timed double-tap, excluding users of switch control and assistive touch. Provide tap-based or switch-accessible alternatives for every gesture-dependent action.

### Trusting The Automated Scanner Over Real Verification

Concluding the interface is accessible because a scanner reports few issues, missing the failures a scanner cannot detect — wrong screen-reader announcements, broken large-text layouts, motion that ignores preferences. Verify by completing the flow with VoiceOver/TalkBack, at the largest text size, in high contrast, and with reduce motion and switch control.

## Self-Check

- [ ] Text uses semantic, scalable styles (iOS text styles / Android sp with theme appearances) rather than hardcoded points or pixels, and the layout was designed and verified across the full dynamic-type range — including the largest accessibility size — without clipping, overlap, truncation-to-nothing, or capping the user's chosen size.
- [ ] Every interactive element exposes correct semantics to assistive tech: standard components are preferred, custom controls have explicit accessibility labels (concise, no redundant "button"), traits/roles, and state, reading order matches the logical/visual order, and decorative content is hidden from the reader.
- [ ] The interface responds to accessibility preferences: Reduce Motion replaces parallax/zoom/auto-animation with fades or static transitions (not just shorter duration), Increase Contrast / Reduce Transparency raise contrast and remove blur, and Bold Text and smart inversion do not break the layout.
- [ ] Color contrast meets established ratios (roughly 4.5:1 normal text, 3:1 large text) verified against the real background including over images and gradients, non-text elements (icons, chart series, focus indicators, input borders) have adequate contrast, and meaning is never conveyed by color alone.
- [ ] Touch targets meet platform minimums (~44×44pt / 48×48dp) with spacing between adjacent targets, and every gesture-dependent action (multi-finger, long-press, timed, precision swipe) has a tap-based or switch-accessible alternative.
- [ ] Verification used real assistive technology — the core flow was completed by audio with VoiceOver/TalkBack, at the largest text size, in high-contrast mode, and with reduce motion and switch control — not only an automated scanner.
- [ ] The interface was judged by whether a user with visual, motor, cognitive, or hearing differences can perceive, operate, and understand it — not by whether it looks fine to a sighted, motor-typical developer.
