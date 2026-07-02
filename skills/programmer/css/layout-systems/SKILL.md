---
name: layout_systems.md
description: Use when the agent is choosing or debugging a CSS layout mechanism (flexbox versus grid versus multi-column versus absolute or fixed positioning), building responsive layouts without media queries, deciding between container queries and media queries, working with grid-template-areas, minmax, auto-fit, auto-fill, subgrid, flex-grow/shrink/basis math, gap, intrinsic sizing, clamp, or debugging collapsed margins, overflow, stacking contexts, z-index, and containing-block traps. Also covers progressive enhancement with feature queries and @supports fallbacks for layout, and the craft of selecting the right layout primitive for a given structure rather than reaching for one tool everywhere.
---

# Layout Systems

Layout is the part of CSS where "it works on my screen" hides the most subtle bugs. Unlike color or typography, a layout choice is usually invisible until it breaks on a different viewport, a different amount of content, or inside a different parent. The judgment problem is not "how do I position this box" but "which layout system makes this structure correct by construction, so that it stays correct when content, viewport, and context change?" An agent that reaches for the first layout tool it knows — flexbox for everything, or absolute positioning to nudge something into place — produces layouts that look right in the demo and collapse the moment a heading wraps or a third item is added.

Two failure patterns dominate real layout work. The first is **using the wrong primitive**: forcing flexbox to do a two-dimensional grid's job, or using absolute positioning where normal flow would have been correct, then patching the resulting breakage with magic numbers and `@media` overrides. The second is **treating layout as a per-element concern**: nudging individual properties at breakpoints instead of choosing a layout system whose intrinsic behavior already adapts. This skill is the layout-mechanics deep dive. It complements `css-architecture-and-specificity`, which covers methodology, specificity, tokens, and performance broadly; here the focus is the actual craft of flexbox, grid, container queries, and the layout debugging that consumes most real layout time.

## Core Rules

### Choose Flexbox Versus Grid By Dimensionality, Not By Familiarity

The single most important layout decision is one-dimensional versus two-dimensional. **Flexbox is one-dimensional and content-driven**: it distributes items along a main axis (row or column) and lets the content's intrinsic size influence the result. **Grid is two-dimensional and structure-driven**: you define explicit rows and columns, and items are placed into that structure. The decision criterion is whether you need to control both axes simultaneously.

- If you are laying out a row of buttons, a toolbar, a card's header with a title and an action, or a vertical stack where items size themselves to content — **flexbox wins**. The content drives the sizing, and alignment along one axis is the whole job.
- If you are laying out a page shell (header / sidebar / main / footer), a form with aligned labels and inputs, a photo gallery with fixed columns, or anything where rows and columns must align to each other — **grid wins**. You are defining structure that content fills, not the reverse.

The trap signal is "I need flexbox with `flex-wrap` and carefully matched widths so the items line up in columns." That is a grid problem solved with flexbox, and the result is fragile to content changes. Conversely, a single row of items placed with explicit `grid-column` lines when `justify-content` would do is grid doing flexbox's job with more ceremony. Pick by the geometry of the problem.

A practical tiebreaker: when you need items to align in both dimensions (the second row's items line up with the first row's items), grid is almost always correct, because flexbox's wrapping is reflow per-line and does not align across lines.

### Prefer Explicit Grid Structure With `grid-template-areas` For Readable Layouts

For page shells and component regions, `grid-template-areas` produces layouts that read as a picture of the structure and that adapt cleanly at breakpoints by re-declaring the area map. Compare a header/sidebar/main/footer shell: with `grid-template-areas` you can collapse the sidebar on narrow viewports by rewriting the area map in one media query, with no changes to the markup or to individual item placement. This is far more maintainable than assigning each region a `grid-column` / `grid-row` by index, where a structural change means renumbering.

The discipline: name your grid areas, place items by area name, and express responsive changes as a new area map rather than as per-item overrides. Reserve explicit line-based placement (`grid-column: 1 / 3`) for cases where an item genuinely spans or is positioned outside the area structure.

### Understand Explicit Versus Implicit Tracks, And Use `minmax` With `auto-fit`/`auto-fill` For Responsive Grids

A common responsive goal is "as many columns as fit, each at least N pixels wide, without media queries." This is `grid-template-columns: repeat(auto-fit, minmax(240px, 1fr))`. The mechanics matter, because the two keywords behave differently and the wrong choice produces a visible bug:

- **`auto-fit`** collapses empty tracks to zero, so a small number of items stretch to fill the row. Five items in a wide container become five wide items.
- **`auto-fill`** keeps empty tracks reserved, so the same five items keep their minimum size and leave gaps. This is usually what you want for a gallery where items should not balloon, and usually wrong for a card row where you want them to fill.

The `minmax(min, max)` is what makes this responsive: the track grows from its minimum up to `1fr` as space allows, and a new track is created once there is room for another minimum. Get the minimum wrong (too large) and you get overflow on small screens; too small and the layout looks cramped. Test the minimum against real content, not a guessed pixel value.

Implicit tracks (rows or columns created automatically when items exceed the explicit grid) default to `auto`, which is usually fine, but if you are placing items into implicit tracks, set `grid-auto-rows` or `grid-auto-columns` explicitly so the behavior is predictable rather than accidental.

### Master The Flexbox Sizing Math: `flex-grow`, `flex-shrink`, `flex-basis`

Flexbox bugs are almost always sizing-math bugs. The `flex` shorthand expands to `grow shrink basis`, and the defaults (`0 1 auto`) surprise people. The rules:

- **`flex-basis`** is the initial size before free space is distributed. `auto` means "use the content size (or the `width`/`height`)." Setting `flex-basis: 0` (as in `flex: 1`) means "ignore content size, start from zero, then grow."
- **`flex-grow`** distributes *positive* free space proportionally. `flex: 1` on all items makes them equal *only if their basis is equal*. With `flex-basis: auto`, items with larger content start larger and stay larger after growing.
- **`flex-shrink`** distributes *negative* free space (overflow) proportionally, weighted by basis. This is why items shrink unevenly: a wide item gives up more pixels than a narrow one.

The strong practice: use the `flex: <grow> <shrink> <basis>` shorthand explicitly and reason about all three. For "equal columns regardless of content," use `flex: 1 1 0` (basis zero, grow equally). For "size to content but shrink if needed," `flex: 0 1 auto` (the default). Avoid `flex: 1` without thinking — it means `1 1 0%`, which discards content size, and that is sometimes wrong (e.g., for items whose content must not be truncated).

### Beware The `min-width: auto` Overflow Trap In Flexbox

This is the most common flexbox bug: a flex item with long unbreakable content (a long word, a wide image, a non-wrapping flex child) overflows its container and cannot shrink below its content's minimum size, because flex items default to `min-width: auto`. The fix is `min-width: 0` (or `min-height: 0` for columns) on the flex item, which lets it shrink and lets inner content (like text with `overflow`/`ellipsis`, or a nested flex/grid) behave. Without this, `text-overflow: ellipsis` silently fails inside a flex child, and nested grids overflow.

Treat `min-width: 0` as a deliberate, documented choice whenever a flex item contains truncating text or a nested layout that must constrain to the item. It is not a hack to sprinkle everywhere — it changes sizing — but it is the correct answer to "my flex child won't shrink / won't truncate."

### Use `gap` For Spacing, Not Margins, In Flex And Grid

The `gap` property spaces flex and grid children uniformly without the margin-collapse and last-child concerns of margins. Prefer `gap` for internal spacing of a flex/grid container; it composes cleanly and avoids the `:last-child { margin: 0 }` dance. `gap` is now universally supported in flexbox (this was not always true — old guidance warned against it; that warning is outdated). Reserve margins for spacing between separate elements in normal flow, where `gap` does not apply.

### Make Layouts Responsive Without Media Queries Where The Structure Allows It

Many responsive needs are better served by intrinsic sizing than by breakpoints. Before reaching for `@media`, ask whether the layout can adapt by construction:

- Fluid sizing with `clamp(min, preferred, max)` for font sizes, spacings, and dimensions that should scale with viewport without snapping.
- `min()`, `max()`, and `minmax()` to let a dimension pick the smaller/larger of two constraints (e.g., `width: min(100%, 960px)` for a centered container that never overflows).
- Grid `auto-fit`/`auto-fill` with `minmax` for responsive column counts.
- Flexbox `flex-wrap` for rows that reflow.

The principle: a layout that adapts intrinsically needs no breakpoint and cannot desync from the viewport. Breakpoints are for *structural* changes (stack vs row, sidebar hidden, navigation restructured), not for nudging a single property. If you find yourself writing a media query to change one padding value, you are probably missing an intrinsic-sizing solution.

### Choose Container Queries When A Component Must Respond To Its Container, Not The Viewport

Container queries (`@container`) let a component adapt to the size of its nearest container, not the viewport. This is the correct tool for reusable components (cards, widgets) dropped into different layouts — a card in a sidebar should stack vertically when the sidebar is narrow, regardless of the viewport width. With media queries, the same card responds to the viewport and behaves identically in a narrow sidebar and a wide main column, which is wrong.

The mechanics: declare a containment context with `container-type: inline-size` (usually) on the parent, then query it with `@container (min-width: ...)`. The discipline: name containers with `container-name` when nested, because the nearest container is queried by default and that may not be the one you intend. Container queries do not replace media queries — use media queries for page-level/viewport-driven changes (overall layout, navigation) and container queries for component-level adaptation. Mixing them deliberately is normal.

### Debug Layout With The Right Mental Model: Margins, Overflow, Stacking, Containing Block

Most "my layout is broken" bugs fall into four categories, and the debugging approach differs for each:

- **Collapsed margins.** Adjacent vertical margins between block elements collapse to the larger one; parent/child margins collapse when no border, padding, or `overflow` separates them. If spacing is smaller than expected or a child's margin pushes the parent instead of the child, suspect margin collapse. Fixes are structural (add padding/border, use `display: flow-root` to establish a block formatting context, or switch to flex/grid where margins do not collapse).
- **Overflow.** Content escaping its container is usually `min-width: auto` in flex (above), a fixed width larger than the available space, or an unbreakable string. Diagnose with the browser's overflow highlight; fix the sizing constraint, do not paper over it with `overflow: hidden` unless hiding is genuinely intended.
- **Stacking contexts and `z-index`.** `z-index` only works on positioned (or flex/grid/flex-item) elements, and only relative to siblings *within the same stacking context*. A child with `z-index: 9999` will not appear above a sibling of its parent that creates a stacking context with a higher z-index — because stacking contexts are nested. Properties that create stacking contexts (`position` with z-index, `opacity < 1`, `transform`, `filter`, `will-change`, `contain: layout`) are the reason "I set z-index to a million and it's still behind." Diagnose by walking up the tree looking for stacking-context creators, not by raising the number.
- **Containing block traps.** Absolutely positioned elements size and position relative to their nearest *positioned* ancestor, not their DOM parent. A `position: absolute` element with no positioned ancestor positions relative to the initial containing block (the viewport-ish), which is why it "jumps to the page." Fixed elements position relative to the viewport *unless* an ancestor has a `transform`, `filter`, or `perspective`, which becomes the containing block and breaks "fixed" behavior. Know which ancestor is the containing block before debugging position.

### Use Feature Queries And Fallbacks For Layout, And Accept Progressive Enhancement

Layout features vary in support (subgrid historically, container queries recently, `gap` in flexbox in older browsers). Use `@supports` to provide a baseline layout and enhance where supported, rather than assuming universal support or abandoning a feature outright. The pattern: write the fallback (often a simpler flex or block layout), then `@supports (property: value) { /* enhanced layout */ }`.

For subgrid specifically: it is the correct tool for aligning a child grid's tracks to its parent grid's tracks, but historically had uneven support. Decide deliberately whether to use it (with a `@supports` fallback) or to achieve the alignment another way (e.g., flattening the structure, or using the parent grid directly). Do not silently depend on it without a fallback if the layout must work everywhere.

### Keep Layout Performant: Avoid Layout-Triggering Properties In Hot Paths

Layout performance is covered broadly in `css-architecture-and-specificity`; the layout-specific point is which properties trigger which work. Animating or repeatedly changing `width`, `height`, `top`, `left`, `margin`, or `padding` triggers layout recalculation across the affected subtree (and sometimes the whole document), which is far costlier than compositing-only changes (`transform`, `opacity`). For movement and resize effects, animate `transform` (translate, scale) rather than the layout properties; for visibility, animate `opacity` rather than `display`/`visibility` toggles in a hot path. Also avoid reading layout properties (`offsetWidth`, `getBoundingClientRect`) interleaved with layout writes in a loop — that forces synchronous reflow. Batch reads before writes.

## Common Traps

### Using Flexbox For Two-Dimensional Alignment And Patching It With Fixed Widths

Forcing a flex container with `flex-wrap` and matched widths to behave like a grid. It works until content of different lengths arrives, then rows misalign. If items must align in two dimensions, use grid; flexbox wrapping does not align across lines.

### `flex: 1` Assumed To Mean "Equal Width"

`flex: 1` means `1 1 0%` — equal *growth from zero*, which is equal only when content is similar. With `flex-basis: auto` (the default in other shorthands), larger content stays larger. Reason about grow, shrink, and basis together; do not assume `flex: 1` equalizes.

### Forgetting `min-width: 0` On A Flex Child, Then Truncation Or Nested Layout Breaks

A flex item cannot shrink below its content's min size by default, so `text-overflow: ellipsis` and nested grids/overflow silently fail. Set `min-width: 0` (or `min-height: 0` in columns) deliberately on flex children that must constrain.

### Confusing `auto-fit` And `auto-fill`

`auto-fit` stretches the few items to fill the row; `auto-fill` keeps empty tracks and leaves gaps. Picking the wrong one gives either ballooning cards or unexpected whitespace. Choose by whether items should grow to fill (`auto-fit`) or hold their size (`auto-fill`).

### Raising `z-index` To Fix A Stacking-Context Problem

Setting `z-index: 99999` when the real issue is that an ancestor created a stacking context. The number cannot beat a parent's stacking context. Walk up the tree to find the context creator; fix the structure (often by not creating an unnecessary context, or by restructuring), not the number.

### Absolute Positioning To "Just Put It There"

Using `position: absolute` with pixel offsets to place something that normal flow, flex, or grid would handle. It removes the element from flow, so it no longer adapts to content or siblings, and breaks on the next viewport. Reserve absolute positioning for genuine overlays (tooltips, badges) relative to a positioned ancestor.

### Assuming `position: fixed` Is Always Relative To The Viewport

A `transform`, `filter`, or `perspective` on an ancestor becomes the containing block for a fixed element, so "fixed" suddenly scrolls with that ancestor. If a fixed header is stuck inside a transformed modal, an ancestor's transform is why.

### Margin Collapse Misdiagnosed As A Spacing Bug

Adding more margin to fix spacing that is smaller than expected, when the real cause is margin collapse between adjacent blocks or between parent and child. Use `display: flow-root`, padding, or a flex/grid container (where margins do not collapse) rather than inflating values.

### Media Queries For Single-Property Tweaks Instead Of Intrinsic Sizing

Writing a breakpoint to change one `padding` or `font-size` when `clamp()` or `min()`/`max()` would adapt fluidly. Intrinsic sizing cannot desync from the viewport; per-property breakpoints accumulate into an unmaintainable matrix.

### Using Container Queries For Viewport-Level Layout

Querying the container for page-shell or navigation changes that depend on the viewport. Container queries are for component adaptation; page-level responsive behavior belongs in media queries. Mixing them without intent produces components that behave differently depending on where they are placed, which is sometimes a bug, not a feature.

### Silently Depending On Subgrid Or Container Queries Without A Fallback

Using a recent feature as if it were universal, so the layout collapses in unsupported browsers. Wrap in `@supports` with a baseline layout, or choose a universally-supported approach when the feature is not load-bearing.

## Self-Check

- [ ] The layout primitive (flexbox vs grid vs multi-column vs positioned) was chosen by the dimensionality and geometry of the structure — flexbox for one-dimensional content-driven distribution, grid for two-dimensional explicit structure — not by familiarity with one tool.
- [ ] Page shells and multi-region layouts use `grid-template-areas` with items placed by area name, and responsive changes re-declare the area map rather than renumbering `grid-column`/`grid-row` per item.
- [ ] Responsive grids use `minmax` with `auto-fit` or `auto-fill` chosen deliberately (fill to stretch items, auto-fill to reserve tracks), and the minimum track size was tested against real content rather than guessed.
- [ ] Flex sizing uses an explicit `flex: grow shrink basis` reasoned as a triple; "equal columns" uses `flex: 1 1 0` deliberately, and `flex: 1` was not assumed to equalize content of different sizes.
- [ ] Flex children that contain truncating text or nested layouts have `min-width: 0` (or `min-height: 0`) so they can shrink and so `text-overflow: ellipsis` and inner grids work.
- [ ] Internal spacing in flex/grid containers uses `gap`, not per-child margins with `:last-child` removal.
- [ ] Fluid dimensions use `clamp`/`min`/`max` and intrinsic sizing where the structure allows, and media queries are reserved for genuine structural changes (stack vs row, sidebar visibility), not single-property nudges.
- [ ] Container queries are used for component-level adaptation to the component's container, with containers named where nested, and media queries retained for viewport-level page changes — the two are not confused.
- [ ] Layout bugs were diagnosed with the right model: margin collapse (use `flow-root`/padding/flex), overflow (fix the sizing constraint, not `overflow: hidden` as a patch), stacking contexts (walked up the tree rather than raising `z-index`), and containing block (the nearest positioned ancestor for absolute; transformed ancestors breaking `fixed`).
- [ ] Movement and resize effects animate `transform`/`opacity` rather than `width`/`height`/`top`/`left`, and layout reads are batched before writes to avoid forced reflow.
- [ ] Layout features with uneven support (subgrid, container queries) are wrapped in `@supports` with a baseline fallback, or a universally-supported approach was chosen when the feature is load-bearing.
