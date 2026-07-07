---
name: image_and_asset_optimization.md
description: Use when the agent is selecting, preparing, or specifying images and visual assets for a product, including format choice, resolution and density, compression, responsive image behavior, lazy loading, art direction across breakpoints, icon formats, illustration systems, and the tradeoff between visual quality and load weight.
---

# Image And Asset Optimization

Images are usually the heaviest part of a product's payload, and they are almost always specified by designers who are thinking about appearance, not weight. A hero image that looks crisp on a 5K monitor can add megabytes to a page that a user on a metered plan pays for in data and time. Asset optimization is a design responsibility because the decisions that drive weight, format, resolution, art direction, and fallback behavior, are visible at the design stage, not the engineering stage.

Use this skill when choosing, producing, or specifying any image, illustration, icon, or visual asset for a product surface. The goal is to prevent the agent from treating assets as drop-in decorations and instead make explicit decisions about format, size, density, compression, and responsive behavior that keep the product fast without sacrificing the visual quality that matters.

## Core Rules

### Choose Format By Content Type And Target Platform

Image format is not a neutral choice. Different formats compress different content types with very different results, and the right format depends on what the image contains and where it ships.

Format guidance by content:

- **Photographs and complex imagery**: prefer modern formats like WebP or AVIF for web, with JPEG as a universal fallback. These compress photographic content far better than PNG.
- **Flat graphics, logos, and icons with few colors**: prefer SVG, which is resolution-independent, tiny, and styleable.
- **Images requiring transparency**: SVG for vector, WebP or PNG for raster. Avoid JPEG, which has no alpha channel.
- **Animations**: prefer video formats or animated WebP/AVIF over animated GIF, which is enormous for its quality.

Do not default to PNG for everything. A photographic PNG can be five to ten times the weight of an equivalent WebP with no visible difference.

### Specify Resolution And Density Deliberately

A single asset at one resolution cannot serve every device well. High-density screens need more pixels; low-density screens and small displays waste bandwidth downloading pixels they cannot show.

Decide:

- the maximum display size each asset will occupy;
- the density variants needed, typically 1x, 2x, and sometimes 3x for the target platforms;
- whether to serve different resolutions via responsive image techniques rather than shipping the largest to everyone;
- whether the asset is even needed at full resolution on mobile, where the display is small.

Exporting only the highest resolution and letting the browser scale it down is the most common cause of image bloat.

### Use Responsive Images And Art Direction

Different viewports are not just different sizes; they need different compositions. A wide hero image cropped to a phone aspect ratio may lose its subject. Art direction means specifying different source images or crops for different breakpoints, not just scaling one image.

Responsive image responsibilities:

- provide multiple sources so each device gets an appropriate resolution;
- use art-directed crops for breakpoints where the composition would otherwise fail;
- specify fallback sources for format support;
- ensure the largest source is never sent to a device that cannot use it.

A single image tag with one source is a performance and composition compromise. Designers should specify the variants, not leave them to engineering.

### Apply Compression Without Visible Quality Loss

Compression is where most weight is saved, and it requires judgment. Too little compression wastes bandwidth; too much destroys detail. The right level depends on the content and the context.

Compression decisions:

- photographic content tolerates lossy compression well; tune quality until just before artifacts appear;
- flat graphics and text-heavy images degrade visibly under lossy compression; prefer lossless or vector;
- faces, skin tones, and text are where artifacts are most noticeable;
- test compression on the actual background and at the actual display size, not in isolation;
- re-evaluate compression when content changes, since different images compress differently.

Blindly exporting at maximum quality is a design failure disguised as care.

### Plan Lazy Loading And Loading Priorities

Not every image needs to load immediately. Above-the-fold images affect perceived performance; below-the-fold images can wait. Designers should indicate loading priority so engineering knows what is critical.

Specify:

- which images are critical to first paint and should load eagerly;
- which images can lazy load as they approach the viewport;
- placeholder or blur-up behavior for lazy images so the layout does not jump;
- whether a low-quality placeholder should load first and sharpen later.

Layout shift from late-loading images is a common, avoidable degradation. Reserving space or using placeholders is a design decision.

### Maintain A Single Source And Regenerable Exports

Assets should flow from a single editable source to all exported variants. When exports are edited by hand, they diverge from the source and become unmaintainable.

A healthy asset pipeline:

- keeps one source file per asset;
- regenerates all densities, formats, and crops from that source;
- names exports consistently and predictably;
- documents the export settings so anyone can reproduce them.

Hand-naming and hand-exporting dozens of variants is a pipeline that breaks under absence.

### Treat Icons And Illustrations As Systems, Not One-Offs

Icons and illustrations accumulate. Treating each as a custom asset produces inconsistency in stroke weight, grid, padding, and style. Treat them as systems with shared rules.

System rules for vector assets:

- a shared grid and keyline for icons;
- consistent stroke weight, corner radius, and endpoint style;
- defined padding so icons align consistently in containers;
- a consistent illustration style, palette, and level of detail;
- a documented process for adding new assets that conform.

A product with icons drawn ad hoc will look inconsistent and will be heavier than necessary due to duplicated or poorly optimized paths.

## Common Traps

### Defaulting To PNG For Everything

PNG is excellent for flat graphics with transparency but catastrophically heavy for photographs. Misuse is a leading cause of image bloat.

### Exporting Only The Largest Resolution

Shipping a 4000-pixel-wide image to every device, including small phones, wastes bandwidth and memory for no visual benefit.

### Ignoring Art Direction Across Breakpoints

Scaling one image to all viewports often crops out the subject on narrow screens. Art direction is a composition decision, not an engineering detail.

### Maximum Quality As Default

Exporting at 100% quality preserves imperceptible detail at a large weight cost. Compression tuned to the content is almost always better.

### Layout Shift From Late Images

Images that load without reserved space push content around, which feels broken and harms perceived performance.

### Inconsistent Icon Systems

Icons drawn one at a time without a shared grid and stroke system produce visual noise that reads as unprofessional, even when each icon alone looks fine.

### Forgetting Fallbacks For Modern Formats

AVIF and WebP are not universally supported. Specifying them without a fallback leaves some users with a broken image.

## Self-Check

- [ ] Format is chosen by content type and platform support, with appropriate fallbacks.
- [ ] Resolution and density variants are specified, and the largest variant is not sent to devices that cannot use it.
- [ ] Responsive sources and art-directed crops are specified for breakpoints where composition would fail.
- [ ] Compression is tuned to the content and reviewed at display size, not exported at maximum quality by default.
- [ ] Loading priority is indicated, with lazy loading and placeholders for non-critical images.
- [ ] Space is reserved or placeholders used so late-loading images do not cause layout shift.
- [ ] Assets flow from a single source to regenerable, consistently named exports.
- [ ] Icons and illustrations follow a shared system of grid, stroke, padding, and style.
