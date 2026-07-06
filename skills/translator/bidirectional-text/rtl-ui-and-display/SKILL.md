---
name: rtl_ui_and_display.md
description: Use when the agent is localizing or reviewing user interface text and screen layouts for right-to-left locales, mirroring navigation icons and directional affordances, handling RTL form fields buttons and dialogs, deciding which icons must flip and which must stay fixed, auditing screenshots and visual assets for RTL correctness, or verifying that interactive components such as sliders carousels and timelines behave correctly in Arabic Hebrew Persian or Urdu interfaces.
---

# RTL UI And Display

A user interface localized into a right-to-left locale is not the LTR interface with its text translated and right-aligned. It is a mirrored composition in which the entire spatial grammar reverses: the starting point of a reading sequence moves to the right, primary navigation migrates to the opposite side, progress and sequence flow toward the left, and every directional affordance, an arrow, a chevron, a slider, a timeline, a swipe gesture, must be re-evaluated against what it now means to a reader whose eyes begin on the right. Translators and localization reviewers routinely underestimate this work because the linguistic translation can be flawless while the interface remains unusable: a back button that now points the wrong way, a progress bar that fills against the direction of progress, a form whose label sits on the wrong side of its field, an icon whose meaning inverts when mirrored. Worse, some elements must flip and others must not, and the rule is not uniform; it depends on whether the element encodes a physical direction or a conceptual one. RTL UI work is therefore a judgment discipline about meaning, not a mechanical flip-everything operation, and getting it wrong produces interfaces that feel foreign, confusing, or broken to the very users the localization was meant to serve.

This skill applies when you are translating, reviewing, or specifying UI strings and screen layouts for RTL locales, when you are deciding which visual and interactive elements to mirror, and when you are auditing rendered RTL interfaces for correctness. The objective is an interface whose text, layout, icons, and interaction all resolve to a coherent RTL experience that mirrors the LTR source in intent rather than merely in geometry.

## Core Rules

### Mirror Spatial Layout, Not Just Text

The foundation of RTL UI correctness is that the spatial composition reverses, and this must be planned at the layout level before any string work.

In an RTL interface, the inline starting edge is the right edge: primary navigation, logos, and leading content sit on the right; secondary content and trailing elements sit on the left; multi-column grids reverse their column order; and the visual weight that balanced the LTR screen must be rebalanced for the mirrored one. This mirroring should be driven by direction-aware layout properties so that a single direction declaration reverses the composition, rather than by manually repositioning each element. Confirm that the layout system actually mirrors when direction is set; if elements stay physically fixed, the interface will be text-RTL but structurally-LTR, which reads as broken. Plan mirroring as a layout concern first, then layer string translation on top.

### Classify Each Directional Element As Flip Or Keep

Not every directional element should be mirrored, and the decision depends on whether the element encodes a physical direction or a conceptual one.

A back arrow is the canonical example of the nuance: in many RTL interfaces, back still points left, because back is conceptually toward the previous place, and users have learned that left means back even in RTL contexts. By contrast, a play button points right in LTR because media plays forward, and in many RTL contexts it should point right for the same conceptual reason, though conventions vary and must be checked against the platform. A timeline that represents chronological progress should reverse so that earlier events sit on the right and later ones on the left, matching the reading direction. A physical-world diagram, a map, or a photograph of a real object must not flip, because flipping would misrepresent reality. For every directional element, ask what it means conceptually, consult the platform's RTL conventions, and decide flip or keep deliberately. Document the decision so it is not relitigated.

### Handle Forms, Fields, And Alignment As Logical Structures

Forms are where RTL correctness is most often visibly wrong, because labels, fields, placeholders, and alignment all carry direction.

In an RTL form, labels typically sit to the right of their fields, text inside fields is right-aligned and flows right-to-left, placeholder text aligns to the right, and required-field markers, help icons, and validation messages position themselves on the logical side. Checkbox and radio groups reverse their order so the first option is on the right. Dropdown content and menu items reverse as well. All of this should follow from direction-aware alignment and layout; where it does not, the form will mix RTL labels with LTR-aligned fields and look inconsistent. Review each form component for logical alignment, and pay special attention to validation states, because an error icon that appears on the wrong side can be missed entirely by the user.

### Re-Evaluate Interactive Components For Direction

Sliders, carousels, tabs, steppers, and pagination carry directional semantics that must be re-evaluated, not merely mirrored.

A slider that increases as it moves right in LTR should increase as it moves left in some RTL conventions, or stay rightward in others, depending on platform guidance. A carousel's next and previous controls must be consistent with the reading direction. Tab order and focus movement should follow the RTL sequence so that keyboard navigation feels natural. Pagination, breadcrumbs, and step indicators reverse their visual order. For each interactive component, determine its directional semantics, apply the platform's RTL convention, and test the interaction, because a mirrored appearance with unmirrored behavior is worse than no mirroring at all.

### Audit Visual Assets And Screenshots

Icons, illustrations, and screenshots are often delivered as fixed images, and they must be reviewed for RTL correctness independently of the text.

Flat icons that encode direction may need mirrored variants for RTL. Illustrations containing text, signage, or directional cues may need localized or mirrored versions. Screenshots embedded in help documentation must be recaptured from the RTL build, not reused from the LTR build, because reusing an LTR screenshot in an RTL guide immediately signals incomplete localization. Maintain a manifest of visual assets, mark which ones require RTL variants, and ensure the correct variant is referenced when direction is RTL. An interface can be perfectly mirrored in code yet still present LTR screenshots to RTL users.

### Preserve Brand And Identity Elements Intentionally

Logos, brand marks, and certain identity elements are often deliberately kept in their original form, and this must be a conscious decision rather than an accident of the pipeline.

Some brands mirror their logo for RTL markets; others keep it identical worldwide because the mark is a fixed asset. Neither choice is wrong, but it must be deliberate and consistent. The same applies to product names rendered in Latin script within RTL text: they run left-to-right and are not mirrored, and their positioning must be handled as embedded LTR content. Record the brand's RTL treatment as a documented decision so that every screen applies it consistently rather than each translator guessing.

### Verify The Complete Rendered Interface

RTL UI defects are visible only in the running interface, and they must be verified in context, screen by screen.

A string-level review cannot catch a mirrored layout that forgot to flip an icon, a form whose validation appears on the wrong side, or a carousel whose controls contradict the reading direction. Walk through the full RTL build, exercise every interactive component, and check every screen against the LTR source to confirm that mirroring is complete and consistent. Treat the rendered interface as the only authoritative artifact; the source strings and the layout declarations are means to that end, not the end itself.

## Common Traps

### Right-Aligning Text And Calling It Done

Mirroring is a layout and interaction concern, not a text-alignment task; an interface that is only right-aligned remains structurally LTR and feels broken.

### Flipping Everything Uniformly

Some elements must flip and others must not; flipping logos, maps, or back arrows uniformly produces nonsense. Classify each directional element by meaning.

### Keeping Back Or Next Controls Inconsistent With Convention

Back arrows, play buttons, and carousel controls have platform-specific RTL conventions; guessing instead of checking produces controls that contradict user expectation.

### Forgetting Form Components

Labels, placeholders, validation markers, and option order in forms are frequent sources of visible RTL defects; review each form component for logical alignment.

### Reusing LTR Screenshots In RTL Help

Embedding LTR screenshots in RTL documentation signals incomplete localization; recapture visuals from the RTL build.

### Mirroring Appearance But Not Behavior

A component that looks mirrored but still increases rightward or tabs in LTR order is worse than no mirroring; re-evaluate interaction semantics, not just visuals.

### Skipping The Full Interface Walkthrough

String-level review cannot catch layout and interaction defects; verify the complete rendered RTL interface screen by screen.

## Self-Check

Before delivering or approving an RTL user interface, verify:

- Spatial layout is mirrored through direction-aware properties so that navigation, columns, and visual weight resolve to a coherent RTL composition.
- Every directional element has been classified as flip or keep based on its conceptual meaning and the platform's RTL convention, with decisions documented.
- Form components use logical alignment for labels, fields, placeholders, validation markers, and option order, with no RTL label paired to an LTR-aligned field.
- Interactive components such as sliders, carousels, tabs, and pagination have had their directional semantics re-evaluated and tested, not merely mirrored visually.
- Visual assets and screenshots have been audited, with RTL variants created where needed and LTR screenshots replaced by recaptured RTL ones in documentation.
- Brand and identity elements have a deliberate, documented RTL treatment applied consistently across every screen.
- The complete rendered RTL interface has been walked through screen by screen, exercising interactive components and comparing against the LTR source.
- No element that should flip has been left fixed, and no element that should stay fixed has been flipped, confirming the interface mirrors the source in intent rather than only in geometry.
