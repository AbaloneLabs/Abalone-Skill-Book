---
name: accessibility_review.md
description: Use when the agent is designing or reviewing an interface for accessibility, including color contrast, focus order, keyboard use, target size, readable text, error messaging, forms, motion, semantics, and inclusive interaction.
---

# Accessibility Review

Accessibility is not a finishing pass after visual design. It is part of whether the interface can be perceived, understood, navigated, and operated by people with different vision, motor ability, cognition, devices, environments, and assistive technologies.

Use this skill before finishing interface designs, design systems, forms, dashboards, navigation, content-heavy pages, interactive controls, data visualizations, or mobile layouts. The goal is to prevent the agent from producing a design that looks polished but excludes users through low contrast, missing focus states, tiny targets, unclear labels, color-only meaning, motion dependence, or inaccessible error handling.

## Core Rules

### Design For Multiple Ways Of Perceiving

Important information should not depend on a single sensory cue. Color, shape, position, text, icon, and pattern can reinforce each other.

Avoid relying only on:

- red versus green;
- subtle shade differences;
- hover-only reveal;
- image-only labels;
- sound-only feedback;
- motion-only explanation.

For status, pair color with text or icon. For charts, use labels, patterns, legends, or direct annotation. For required fields, use visible text or programmatic indication, not only color.

### Maintain Readable Contrast And Typography

Text and important UI boundaries need enough contrast against their background. Low-contrast gray text, disabled-looking active controls, image overlays, tinted buttons, and subtle borders often fail in real environments.

Review:

- body text;
- small labels;
- placeholder text;
- button text;
- links;
- focus indicators;
- error messages;
- chart labels;
- icons that carry meaning;
- text over images.

Typography should remain readable under zoom, larger text settings, and localization. Do not use tiny text as a way to make dense layouts appear clean.

### Make Keyboard And Focus Behavior Visible

A user should be able to navigate interactive elements by keyboard and always know where focus is. Focus order should follow visual and task order. Focus indicators should be visible enough to locate quickly.

Check:

- all controls are reachable;
- focus does not disappear;
- modal dialogs trap and restore focus correctly;
- menus, tabs, accordions, and custom widgets have expected keyboard behavior;
- disabled elements are not focus traps;
- skip or landmark navigation exists for complex pages;
- focus is not hidden under sticky headers.

Focus design is a visual design responsibility, not only an engineering detail.

### Give Controls Adequate Target Size And Spacing

Touch and pointer targets need enough size and separation to reduce accidental activation. Small icon buttons, dense table actions, close buttons, checkboxes, slider handles, and map controls are common failures.

If the visual affordance is small, the hit area can still be larger. Leave enough spacing between destructive and safe actions. Consider users with tremor, one-handed use, small devices, gloves, or imprecise pointers.

### Label Forms And Errors Clearly

Forms should be understandable without guessing. Labels should remain visible when the field has content. Placeholder text is not a label. Required fields, expected formats, constraints, and examples should be available before errors occur when possible.

Error messages should:

- identify the field;
- explain the problem;
- describe how to fix it;
- appear near the field and in any summary;
- remain available to assistive technology;
- not rely on color alone.

For multi-step forms, preserve progress, allow review, and avoid losing entered data when errors occur.

### Avoid Motion Dependence And Provide Calm States

Motion can clarify transitions, but it can also distract, trigger discomfort, or hide state changes. Do not require users to track movement to understand content. Avoid excessive parallax, flashing, auto-advancing carousels, and animated backgrounds behind text.

Provide reduced-motion alternatives where relevant. Keep loading and transition states understandable without animation.

### Design Accessible Data And Visualizations

Charts, maps, dashboards, and metrics need accessible interpretation, not just visual appeal.

Check:

- chart title states the point;
- axes and units are labeled;
- color palettes are distinguishable;
- values are available in text or table form when needed;
- legends are close to the data;
- interaction does not depend only on hover;
- outliers and uncertainty are explained;
- screen reader users can access the key information.

### Keep Language And Cognitive Load Clear

Accessibility includes comprehension. Use plain labels, predictable placement, consistent terminology, and clear next steps. Avoid forcing users to remember information across screens when the interface can keep context visible.

Break complex tasks into understandable groups. Make destructive consequences explicit. Avoid clever labels for critical actions.

## Common Traps

### Treating Accessibility As A Checklist At The End

Late accessibility fixes often become compromises. Contrast, focus, target size, layout order, and semantics affect core design decisions.

### Color-Only Status

Dashboards and forms often show red, yellow, green, or blue states without text. Users with color vision differences, low-quality displays, or grayscale printing may miss the meaning.

### Invisible Focus Because It Looks Cleaner

Removing focus outlines may make screenshots cleaner but makes the product harder or impossible to use by keyboard.

### Tiny Icon Buttons In Dense UIs

Operational tools often compress actions until only precise pointer users can operate them. Dense does not have to mean tiny.

### Placeholder-Only Inputs

Once the user types, the instruction disappears. This hurts memory, review, error correction, and assistive use.

### Text On Unpredictable Images

Text over user-uploaded or variable images can fail contrast and readability. Use stable overlays, separate text areas, or image treatments that guarantee legibility.

### Hover-Only Information

Hover does not exist for all touch users and may be hard for keyboard users. Important information should have another path.

## Self-Check

- [ ] Important information is not conveyed by color, motion, sound, position, or image alone.
- [ ] Text, icons with meaning, focus indicators, controls, errors, and text over images have sufficient contrast.
- [ ] Keyboard focus order follows task order, all controls are reachable, and focus is always visible.
- [ ] Touch and pointer targets have adequate size and spacing, especially for icon buttons and destructive actions.
- [ ] Forms use persistent labels, clear requirements, useful examples, and field-level error recovery.
- [ ] Motion is not required to understand the interface, and reduced-motion needs were considered.
- [ ] Charts, maps, dashboards, and visualizations provide labels, units, textual meaning, and non-hover access.
- [ ] Language, terminology, grouping, and instructions reduce cognitive load rather than adding clever ambiguity.
- [ ] Modals, menus, tabs, accordions, and custom controls have expected focus and keyboard behavior.
- [ ] The design was reviewed under realistic conditions: mobile, zoom, large text, low vision, keyboard use, and assistive technology needs.
