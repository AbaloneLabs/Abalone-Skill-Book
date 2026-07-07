---
name: layout_composition_and_alignment.md
description: Use when the agent is composing a page or screen layout, deciding the arrangement and relationship of regions, applying alignment and grouping principles, balancing symmetry and asymmetry, establishing focal points and reading order, using contrast and scale to direct attention, or reviewing whether a composition communicates structure and priority before any visual styling is applied.
---

# Layout Composition And Alignment

Composition is the arrangement of elements into a deliberate whole, and alignment is the discipline that holds that whole together. Before any color, type, or imagery is applied, the composition already determines most of what the user will perceive: what matters, what groups with what, where to look first, and how to move through the page. A weak composition cannot be rescued by styling, because the eye reads structure before decoration, and a layout whose regions are misaligned, ungrouped, or competing for attention will feel disorganized no matter how polished its components are.

Agents tend to fail composition in predictable ways. They fill the canvas edge to edge, leaving no room for the eye to rest or for hierarchy to register. They align elements loosely, so that items which should relate appear disconnected and items which should be separate appear joined. They create multiple competing focal points, so attention scatters instead of landing. Or they treat composition as a visual concern only, forgetting that the same structure determines reading order, accessibility, and how a screen reader narrates the page.

Use this skill before finalizing any page or screen layout, when arranging regions, and when reviewing whether a composition communicates structure. The goal is a composition whose alignment, grouping, focal hierarchy, and balance communicate priority before any styling is applied, and whose structure survives across viewport sizes.

## Core Rules

### Establish A Clear Focal Hierarchy

A composition with no focal point forces the user to decide what matters, which is the designer's job. Every screen should answer the question "where do I look first" with a single dominant element, supported by secondary and tertiary elements in descending prominence.

Build focal hierarchy deliberately:

- choose one primary focal point per view, usually the element that serves the page's main intent;
- use scale, weight, contrast, and position to establish which elements dominate and which recede;
- limit the number of competing focal points, because each additional one dilutes attention;
- ensure the focal point aligns with the user's actual goal on the screen, not with what the team wants to promote.

A page with five equally weighted call-to-action buttons has no focal point; it has five competing ones, and the user will choose arbitrarily.

### Align Deliberately And Consistently

Alignment is the invisible structure that makes a composition feel intentional. Elements that share an alignment line read as related; elements that are misaligned read as accidental. The discipline is to choose alignment lines deliberately and apply them consistently, rather than nudging elements into place by eye.

Align with intent:

- establish a clear alignment scheme, such as left-aligned text, centered headers, or grid-based columns, and apply it consistently;
- align related elements to a common edge or center so their relationship is visible;
- avoid mixed alignments within a group, which fragments the composition;
- use the underlying grid or baseline as the alignment reference, not ad hoc positioning.

Loose alignment is the most common cause of layouts that feel "off" in a way the team cannot diagnose. The fix is almost always stricter alignment, not more styling.

### Group Through Proximity And Closure

What belongs together should sit together. The eye uses proximity, closure, and shared visual treatment to infer groups, and a layout that scatters related elements forces the user to reconstruct relationships that proximity could have made obvious.

Group deliberately:

- place related elements close together and separate unrelated ones with space, letting proximity carry the grouping;
- use consistent internal spacing within a group and larger spacing between groups;
- reinforce grouping with shared background, border, or alignment where proximity alone is not enough;
- avoid arbitrary spacing that breaks the perceived groups the design intends.

Inconsistent spacing is the second most common composition failure: when the gap within a group equals the gap between groups, the groups dissolve.

### Balance Symmetry And Asymmetry With Intent

Balance does not require symmetry. A symmetrical composition feels formal and stable; an asymmetrical composition feels dynamic and modern. Both are valid, but each must be balanced deliberately, because accidental asymmetry reads as carelessness.

Balance with intent:

- use symmetry where stability and formality matter, such as login screens or ceremonial pages;
- use asymmetry where dynamism and editorial expression serve the content, balancing a large element with a smaller, visually heavier one;
- ensure asymmetry is deliberate, with the imbalance serving a purpose such as directing attention;
- avoid accidental asymmetry, where elements drift off-axis without reason, which reads as sloppy.

The test is whether the imbalance is intentional. Deliberate asymmetry communicates; accidental asymmetry distracts.

### Use Scale And Contrast To Direct Attention

Attention follows scale and contrast. The largest, darkest, or most saturated element will be seen first, regardless of where the team intends the user to look. Composition uses scale and contrast as tools to direct the eye along the intended path.

Direct attention through scale and contrast:

- reserve the strongest scale and contrast for the primary focal point;
- step scale and contrast down through the hierarchy so secondary elements support rather than compete;
- avoid uniform scale across elements, which flattens the hierarchy and forces the user to scan everything equally;
- use contrast sparingly, because a page where everything is high-contrast has no contrast.

### Compose For Reading Order And Flow

Composition is not only visual; it determines the order in which the user encounters content, which is also the order a screen reader will announce it. A composition that looks right visually but reads in the wrong order fails both sighted and assistive technology users.

Compose for flow:

- arrange content so the visual reading order matches the intended logical order, left-to-right and top-to-bottom in left-to-right languages;
- ensure the dominant focal point is also the logical starting point, so visual and structural priority agree;
- avoid layouts where the visual order diverges from the DOM order, which breaks screen reader narration;
- test the composition's reading order by narrating it aloud, which reveals flow problems the eye glosses over.

### Compose For The Viewport, Not Just The Canvas

A composition built at one size rarely survives other viewports. Regions that balance on desktop collapse or reflow on mobile, and a focal hierarchy established at one width can invert at another. Composition must be designed as a responsive system, not a fixed canvas.

Compose responsively:

- define how regions reflow and how the focal hierarchy adapts at each breakpoint;
- preserve the reading order and grouping as the layout reflows;
- decide which elements are fixed, fluid, or hidden at each size, rather than letting the implementation decide;
- test the composition at the extremes, the smallest and largest viewports, not only the comfortable middle.

### Let Structure Carry Meaning Before Styling

The test of a strong composition is whether it communicates structure before any visual styling is applied. A grayscale wireframe of the layout should already convey hierarchy, grouping, and flow. If it does not, no amount of color or type will fix it.

Validate structure first:

- review the composition in grayscale or wireframe form before applying visual styling;
- confirm that hierarchy, grouping, alignment, and flow are legible without decoration;
- treat styling as reinforcement of an already-clear structure, not as compensation for a weak one.

## Common Traps

### No Clear Focal Point

A page with several equally weighted elements forces the user to decide what matters, which is the designer's responsibility.

### Loose Or Inconsistent Alignment

Elements nudged into place by eye produce a composition that feels off in a way the team cannot diagnose. Stricter alignment is usually the fix.

### Inconsistent Spacing That Breaks Groups

When the gap within a group equals the gap between groups, the groups dissolve and the structure is lost.

### Accidental Asymmetry

Imbalance without intent reads as carelessness. Deliberate asymmetry communicates; accidental asymmetry distracts.

### Uniform Scale And Contrast

A page where everything is large and high-contrast has no hierarchy, because attention follows relative difference.

### Visual Order Diverging From Logical Order

A composition that looks right but reads in the wrong order fails screen reader users and sighted users alike.

### A Single-Viewport Composition

A layout built at one width breaks at others. Compose as a responsive system, tested at the extremes.

### Styling Compensating For Weak Structure

If the grayscale wireframe does not communicate hierarchy, color and type will not rescue it.

## Self-Check

- [ ] The composition has a single clear primary focal point that aligns with the user's goal on the screen.
- [ ] Alignment is deliberate and consistent, with related elements sharing alignment lines and the underlying grid as reference.
- [ ] Grouping is carried by proximity and consistent spacing, with larger gaps between groups than within them.
- [ ] Symmetry or asymmetry is deliberate and balanced, serving communication rather than appearing accidental.
- [ ] Scale and contrast direct attention along the intended hierarchy, with the strongest treatment reserved for the focal point.
- [ ] The visual reading order matches the logical and DOM order, so sighted and screen reader users encounter content in the same sequence.
- [ ] The composition is designed as a responsive system, with defined reflow, preserved order, and tested extremes.
- [ ] The grayscale or wireframe version of the layout communicates hierarchy, grouping, and flow before any styling is applied.
- [ ] No more focal points compete than the hierarchy can support, and secondary elements support rather than rival the primary.
- [ ] Spacing is consistent within and between groups, so structure is legible at a glance.
