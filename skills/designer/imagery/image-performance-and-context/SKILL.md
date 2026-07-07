---
name: image_performance_and_context.md
description: Use when the agent is placing imagery in a product and must weigh file weight, loading behavior, responsive sources, lazy loading, placeholders, alt text, decorative versus informational roles, dark mode adaptation, and whether an image earns its place in the experience.
---

# Image Performance And Context

An image in a product is never just a visual decision. It is also a performance decision, an accessibility decision, a contextual decision, and often a content decision. A beautiful hero photograph that weighs three megabytes, loads last, has no text alternative, and disappears in dark mode is a failure even though it looks correct in a static mockup. Imagery exists inside a running system, and that system has bandwidth, devices, assistive technology, themes, and real users with real conditions. The judgment problem is deciding not only what an image shows but how it behaves, what it costs, and whether it belongs at all.

Use this skill whenever imagery is placed in a shipped product: heroes, cards, avatars, backgrounds, illustrations, and decorative elements. The goal is to prevent the agent from treating images as static assets dropped into a layout, and instead to treat them as performance-critical, context-sensitive, accessibility-bound parts of the experience.

## Core Rules

### Justify Every Image Against Its Value

Not every surface needs an image. Imagery adds weight, complexity, and maintenance. Before placing an image, ask whether it earns its position.

An image earns its place when it:

- communicates something the text cannot do as well;
- sets emotional tone that shapes understanding;
- identifies a person, place, or product that text alone cannot;
- breaks up dense content to aid scanning and comprehension.

An image does not earn its place when it is decorative filler, repeats information already in the text, or exists only because the layout has an empty region. Removing a low-value image often improves a page more than adding one.

### Budget File Weight Against The Surface's Importance

Images are usually the heaviest assets on a page. Their cost must be proportional to their importance.

Weight guidance:

- critical above-the-fold heroes justify more weight, but should still be optimized aggressively;
- decorative and background images should be as light as possible, since they carry less meaning;
- thumbnails and avatars should be tiny, because they appear in large numbers;
- images far down the page can be larger only if they are unlikely to load for most users.

Set explicit budgets for common image types and enforce them. A library of unoptimized images silently degrades performance across the whole product.

### Choose Formats And Responsive Sources Deliberately

Modern image delivery is not a single file. It is a set of format and size decisions that adapt to the device.

Decide:

- the format: modern formats such as WebP or AVIF for smaller weight, with JPEG or PNG fallbacks where needed;
- responsive sources: multiple resolutions via `srcset` so devices get appropriately sized images;
- art direction via `picture` when different crops are needed at different widths;
- whether vector (SVG) is more appropriate than raster for icons, logos, and simple graphics.

Serving a 4000-pixel-wide image to a phone wastes bandwidth and slows the largest class of users. Responsive sources are not optional for any image-heavy product.

### Design Loading States And Placeholders

Images do not appear instantly. What users see while they wait is a design decision. A blank gap, a layout shift, or a jarring pop-in all feel broken.

Plan loading behavior:

- reserve space with aspect-ratio containers to prevent layout shift;
- use lightweight placeholders, blur-up previews, or solid color fills;
- lazy-load images below the fold so they do not compete with critical content;
- consider skeleton states for image-heavy grids and galleries.

Layout shift caused by late-loading images is one of the most common sources of a product feeling janky. Reserving dimensions is a simple, high-impact fix.

### Assign Accessibility Roles Explicitly

Every image has an accessibility role: it is either informational or decorative. The role determines what assistive technology should do with it.

For informational images:

- provide concise, meaningful alt text that conveys the image's purpose;
- avoid redundant alt text that repeats adjacent text;
- describe the meaning, not the visual mechanics, unless the visual is the point.

For decorative images:

- mark them as decorative so screen readers skip them;
- do not add empty or placeholder alt text that adds noise.

Charts, diagrams, and images of text need special care: they often require longer descriptions, captions, or accessible alternatives, because their content cannot be inferred from a short alt string.

### Adapt Imagery To Theme And Context

Images that look correct in one theme can fail in another. A bright photograph optimized for light mode can glare in dark mode; a low-contrast background can disappear; an image with baked-in white background creates a visible box on dark surfaces.

Plan for context:

- provide dark-mode-aware variants where images have baked-in backgrounds;
- avoid imagery whose meaning depends on a specific theme;
- test illustrations and icons in both light and dark modes;
- consider reduced-data and low-end-device contexts where heavy imagery should degrade gracefully.

### Respect Privacy And Provenance

Imagery involving real people, real locations, and sensitive content carries obligations. A product that displays user photos, surveillance imagery, medical imagery, or images of minors has responsibilities that decorative stock does not.

Check:

- consent and releases for recognizable people;
- handling of sensitive, explicit, or distressing imagery with appropriate warnings;
- provenance and licensing for every image;
- whether user-uploaded imagery needs moderation before display.

Performance and aesthetics never override these obligations.

## Common Traps

### Adding Images As Filler

Decorative images placed to fill space add weight without value and often distract from the content that matters.

### Serving One Huge Image To All Devices

A single high-resolution file wastes bandwidth on mobile and slows the experience for the users least able to afford it.

### Ignoring Layout Shift

Images without reserved dimensions push content around as they load, making the page feel unstable and causing accidental clicks.

### Treating All Images As Informational

Adding alt text to decorative images floods screen reader users with noise. Decorative images should be marked decorative.

### Forgetting Dark Mode And Theme Adaptation

Images with baked-in backgrounds or theme-dependent contrast break when the theme changes.

### Optimizing Only The Hero

Teams often optimize the visible hero and leave dozens of thumbnails and avatars unoptimized, where their cumulative weight dominates the page.

### Displaying Sensitive Imagery Without Handling

Showing real people, minors, or distressing content without consent, moderation, or content warnings creates real harm and legal exposure.

## Self-Check

- [ ] Each image justifies its place by adding meaning, emotion, or identification that text alone cannot provide.
- [ ] File weight is budgeted by surface importance, and decorative or low-value images are aggressively optimized or removed.
- [ ] Modern formats, responsive sources via `srcset`, and art direction via `picture` are used where appropriate.
- [ ] Loading states, reserved dimensions, placeholders, and lazy loading prevent layout shift and jarring pop-in.
- [ ] Every image has an explicit accessibility role: meaningful alt text for informational images, decorative marking for the rest.
- [ ] Imagery adapts to theme and context, including dark mode, reduced-data modes, and low-end devices.
- [ ] Consent, releases, licensing, moderation, and content warnings are handled for sensitive and user-generated imagery.
- [ ] The full page was audited for cumulative image weight, not just the hero, and the heaviest images are the most important ones.
