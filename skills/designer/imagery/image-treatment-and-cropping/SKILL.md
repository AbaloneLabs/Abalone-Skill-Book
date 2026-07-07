---
name: image_treatment_and_cropping.md
description: Use when the agent is cropping, resizing, retouching, applying overlays or filters, preparing images for cards, heroes, thumbnails, avatars, and responsive containers, ensuring focal subjects survive aspect ratios, and protecting legibility of text over imagery.
---

# Image Treatment And Cropping

An image rarely arrives in the exact shape and condition a product needs. It must be cropped to fit a card, resized across aspect ratios, retouched to remove distractions, graded to match a direction, and often overlaid with text. Each of these operations is a design decision with consequences. A crop can decapitate a subject, a resize can push the focal point off-screen, a filter can flatten a vibrant image, and an overlay can render text unreadable. Treatment and cropping are where good source imagery is either preserved or quietly destroyed.

Use this skill whenever an image must be prepared for a product surface: hero blocks, cards, thumbnails, avatars, galleries, backgrounds, and responsive containers. The goal is to prevent the agent from treating crop, scale, and treatment as mechanical steps, and instead to treat them as decisions that protect the subject, the meaning, and the legibility of every surface.

## Core Rules

### Protect The Focal Subject Across Every Crop

The focal subject is what the image is about. Cropping must preserve it, not just fill a container. When the same image is used across multiple aspect ratios, the subject must survive each one.

Techniques to protect the subject:

- use focal points or object-position so that resizing keeps the subject in frame;
- prefer aspect-ratio-specific crops over naive center crops for critical images;
- avoid crops that cut through faces, hands, or key objects at awkward points;
- when a subject is off-center, re-crop deliberately rather than letting the container decide.

A center crop assumes the subject is centered, which it often is not. Defaulting to center crop is the most common way products accidentally behead portraits or lose the most important part of a scene.

### Match Crop Intent To The Surface Role

Different surfaces need different crops. A hero image needs negative space for text and a clear focal area. A thumbnail needs the subject to read instantly at a small size. An avatar needs a centered face that survives a circle. A card image needs to establish context without competing with the title.

Decide crop intent per surface:

- **Hero**: wide, with safe space for headlines and calls to action.
- **Card**: consistent aspect ratio across the set, subject clear and uncropped at edges.
- **Thumbnail**: tight on the subject, minimal background, legible at small size.
- **Avatar**: face-centered, headroom controlled, works inside a circle.
- **Background**: enough texture or darkness to support overlaid content.

Using one crop strategy for all surfaces guarantees that some surfaces will fail.

### Preserve Aspect Ratio Consistency Within Sets

When images appear together as a set — a grid of cards, a gallery, a list of avatars — inconsistent aspect ratios break the rhythm and make the layout look unfinished. Define the aspect ratio for each set and enforce it.

If source images have different native ratios, decide how to reconcile them:

- crop all to the target ratio, accepting some loss;
- use a consistent container with object-fit so images fill without distortion;
- separate images that cannot conform into a different surface.

Never stretch or squeeze images to fit a container. Distortion is instantly visible and signals carelessness.

### Guarantee Text Legibility Over Imagery

When text sits over an image, legibility is not guaranteed by default. Variable photographs, bright spots, and busy backgrounds all undermine contrast.

Protect legibility by:

- placing text over the darkest or most uniform region of the image;
- applying a scrim, gradient, or solid overlay tuned to the image;
- choosing images with built-in negative space for text;
- adding text shadows or backdrops for variable imagery;
- testing with the actual images that will ship, not idealized samples.

A common failure is designing over a carefully chosen image and then shipping the same layout over user-uploaded images that have no safe text region. Build the treatment to survive the worst realistic image, not just the best one.

### Apply Treatment As A System, Not Per Image

Color grade, contrast, warmth, and finish should be consistent across an image set. Per-image ad hoc treatment produces a library that drifts.

Define a treatment system:

- a baseline grade applied to all images in a direction;
- allowed variations for specific contexts (for example, darker grade for moody sections);
- a retouching philosophy (natural versus polished);
- rules for overlays, borders, and corner radius applied to image containers.

Document the treatment so that new images can be prepared to match without re-inventing the look each time.

### Handle Variable And User-Generated Content Defensively

Not all imagery is under the designer's control. User avatars, uploaded photos, and dynamic content arrive in arbitrary condition. Treatment must assume the worst.

Defensive patterns:

- containers with fixed aspect ratios and object-fit so any image fills cleanly;
- fallback images or colored backgrounds when no image is provided;
- overlays that guarantee text legibility regardless of image content;
- avatar treatments that work for faces, logos, and abstract images alike.

Designing only for ideal source images guarantees breakage in production.

### Respect The Integrity Of The Original

Retouching and treatment can cross into misrepresentation. Removing a person to clean a scene, altering a product's appearance, or editing a real location to look different from reality can create ethical and trust problems, especially in journalism, commerce, and documentation.

Decide what level of alteration is acceptable for each context, and document it. Product marketing may allow idealization; documentation and editorial content usually do not.

## Common Traps

### Defaulting To Center Crop

Center crop assumes a centered subject. For portraits, landscapes, and off-center compositions, it routinely destroys the focal point.

### Stretching Or Squeezing To Fit

Distorting an image to fill a container is always visible and always reads as an error. Use object-fit, cropping, or ratio-specific masters instead.

### Inconsistent Aspect Ratios In A Set

Mixed ratios in a card grid or gallery break visual rhythm and make the layout feel unfinished.

### Designing Text Overlays Only For The Best Image

A layout that works over a curated hero often fails over user-generated or variable images. Legibility treatment must survive realistic inputs.

### Per-Image Ad Hoc Grading

When each image is graded individually, the library slowly loses coherence. Treatment must be systematized.

### Cutting Subjects At Awkward Points

Crops through eyes, joints, or hands feel uncomfortable even when viewers cannot articulate why. Crop at natural boundaries or widen to include the whole subject.

### Over-Retouching Into Misrepresentation

Heavy retouching that changes what is real erodes trust and can create legal or ethical exposure in commercial and editorial contexts.

## Self-Check

- [ ] The focal subject survives every crop and aspect ratio, using focal points or ratio-specific crops rather than naive center crops.
- [ ] Crop intent matches each surface role: hero, card, thumbnail, avatar, and background each have a defined strategy.
- [ ] Aspect ratios are consistent within each image set, and distortion is never used to fit a container.
- [ ] Text legibility over imagery is guaranteed by scrim, gradient, placement, or image selection, and survives realistic variable images.
- [ ] Color grade, contrast, and finish are applied as a documented system, not reinvented per image.
- [ ] Variable and user-generated content is handled defensively with fixed containers, fallbacks, and robust overlays.
- [ ] Retouching stays within the ethical limits appropriate to the context and does not misrepresent reality.
- [ ] The image set was reviewed together at final size, not approved one image at a time.
