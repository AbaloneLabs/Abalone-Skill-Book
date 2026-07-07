---
name: typography_and_text_rendering.md
description: Use when the agent is setting web typography — choosing and loading web fonts, configuring font-display and preloading to control FOUT/FOIT, sizing line-height and vertical rhythm, tuning text wrapping with text-wrap balance/pretty, controlling hyphenation, truncating text with line-clamp or ellipsis, subsetting or adopting variable fonts, or debugging layout shift and reflow caused by late-loading fonts. Also covers the trap of typographic choices that look correct in isolation but shift content, orphan headings, break on long words, or read poorly across viewports. Distinct from layout-systems (geometry) and design-tokens (token values).
---

# Typography And Text Rendering

Typography is where "it looks fine to me" hides the most user-visible defects, because text is both the primary content of most interfaces and a system of interacting measurements: font metrics, line-height, measure width, wrapping, and load timing. An agent that picks a font, sets a size, and moves on has usually shipped three invisible problems: a late-loading web font that visibly shifts the layout after first paint, a heading that orphans onto the next page, and a long unbreakable string that overflows its container. None of these error. They simply degrade reading and perceived quality, and the degradation is silent until a real user on a real viewport hits it.

The judgment problem is not "which font looks nice" but "will this text render correctly and stay correct when the font swaps, when the content is longer or shorter than expected, and when the viewport changes." Agents tend to treat typography as decoration (pick a font, set a size) rather than as a rendering pipeline with load-order dependencies, metric mismatches, and wrapping behavior that must be reasoned about. This skill is the typography-rendering deep dive. It complements `layout-systems` (which covers the geometry of flexbox/grid and overflow mechanics broadly), `design-tokens-and-theming` (which covers typography *token values* and scales), and `performance-and-critical-css` (which covers font loading for first-paint performance); here the focus is the craft of making text read and render correctly — font loading and swap behavior, variable fonts, vertical rhythm, wrapping and balance, hyphenation, truncation, and the layout-shift trap.

## Core Rules

### Control Font Loading Behavior Deliberately, Not By Default

A web font is an asynchronous resource that arrives after the browser has already started rendering, and what happens during that gap is configurable, not automatic. The `font-display` descriptor is the lever, and choosing it is a typographic decision, not merely a performance one:

- **`swap`** shows fallback text immediately and replaces it when the web font loads (FOUT — flash of unstyled text). This is usually the right default for body text and most UI, because the user can read immediately and the swap is a brief refinement. The cost is a visible reflow if the fallback and web fonts have different metrics.
- **`optional`** behaves like swap but, on slow connections, may abandon the web font entirely and keep the fallback, avoiding a disruptive late swap. Good when a late font swap would be more jarring than a slightly-less-branded fallback — but it means some users never see your chosen font, which is a deliberate tradeoff.
- **`block`** hides text for up to a few seconds waiting for the font (FOIT — flash of invisible text). This is almost always wrong for readable content: it harms Largest Contentful Paint and leaves users staring at blank space. Reserve it only for cases where the fallback is truly unusable.

The discipline: make the choice explicit per font face, default to `swap`, and preload the fonts that above-the-fold text actually uses (`<link rel="preload" as="font" crossorigin>`). Preloading without `crossorigin` silently fails for fonts. Do not preload every weight and variant — preload only what the initial viewport needs.

### Eliminate Font-Swap Layout Shift By Matching Fallback Metrics

The most common typographic defect is Cumulative Layout Shift (CLS) caused by a web font swapping in with different metrics than the fallback, so every line of text reflows. The fix is not "load the font faster" (though that helps) but to choose a fallback font whose metrics approximate the web font, and to size it to match:

- **Pick a fallback with similar x-height, width, and stroke weight** to the web font (e.g., a system sans for a humanist web sans). The closer the metrics, the smaller the shift.
- **Use `size-adjust`, `ascent-override`, and `descent-override` in an `@font-face` for the fallback** to force its metrics to match the web font, so the swap changes glyph shapes without changing line boxes. This is the most effective CLS mitigation for fonts.
- **`font-size-adjust`** (where supported) keeps the x-height constant across font swaps, which preserves reading size even when the family changes.

If your font swap visibly moves content, the fallback metrics are wrong — this is a fixable configuration, not an inherent cost of web fonts.

### Use Variable Fonts Deliberately, Not As A Default

A variable font packs multiple weights (and sometimes widths, optical sizes, or slants) into a single file by exposing continuous axes. The tradeoff is real: one variable font can replace four or five static weights and reduce total bytes *when you actually need several weights*, but a single variable font file is larger than one static weight, so it is wasteful when you only need one. Decide by usage:

- **Adopt variable fonts when the design uses multiple weights or an axis you intend to animate** (e.g., weight transitions on hover, responsive optical sizing). The single file then serves many states efficiently.
- **Prefer a single static weight when only one is used.** Loading a variable font to use one weight is slower than loading that weight as a static file.
- **Subset variable fonts to the needed character range**, the same as static fonts — a variable font with thousands of unused glyphs is even more wasteful per byte.

### Establish Line-Height And Vertical Rhythm As A System, Not Per-Element

Line-height (leading) governs readability, and inconsistent leading makes a page feel broken even when nothing is technically wrong. The discipline is to set line-height relative to the text's size and role, consistently:

- **Body text** generally reads well at `1.4`–`1.6` line-height; longer measures (wide columns) need more leading, short measures need less. Fixed unitless values (`line-height: 1.5`) scale with font-size, which is correct; fixed `px`/`em` line-heights can break at different sizes.
- **Headings** need tighter leading (`1.1`–`1.3`) because large text has generous built-in spacing; loose heading leading looks airy and disconnected.
- **Vertical rhythm** — spacing headings, paragraphs, and components on a consistent baseline — produces visual coherence. Implement it via a spacing scale (see `design-tokens-and-theming`) rather than ad-hoc margins, so that text blocks align to a shared rhythm.

The trap is setting one global line-height and inheriting it everywhere, so headings inherit body leading and look wrong. Set line-height per typographic role.

### Control Wrapping With `text-wrap` Rather Than Manual Breaks

Historically, agents controlled line breaks with `<br>`, non-breaking spaces, or by accepting whatever the browser produced. Modern `text-wrap` properties give real control without fragile manual markup:

- **`text-wrap: balance`** on headings and short blocks balances the lines so a two-line heading does not leave one word dangling on the second line. It prevents the "orphan word" that looks careless.
- **`text-wrap: pretty`** on paragraphs widows the last line intelligently, avoiding a single word stranded at the end. Use it for body copy where balanced wrapping of every line is unnecessary.
- **Avoid `<br>` for layout.** A manual break that looks right at one viewport breaks awkwardly at another; `text-wrap` re-evaluates per viewport.

These properties have growing support; provide them as progressive enhancement (browsers that ignore them still render readable text) rather than depending on them for correctness.

### Handle Hyphenation And Overflow Explicitly

Long words and narrow containers collide, and the default behavior is usually wrong. Reason about each:

- **Hyphenation** (`hyphens: auto`, with the correct `lang` attribute) lets the browser break long words across lines with hyphens. It is appropriate for justified or narrow-column text and harmful for headings or UI labels (you do not want a button label hyphenated). Set `hyphens` deliberately per context, and remember it requires `lang` to work correctly.
- **Overflow handling.** When text must be constrained to a fixed height or width, use the right tool: `text-overflow: ellipsis` with `white-space: nowrap` and `overflow: hidden` for single-line truncation; `-webkit-line-clamp` (now standard `line-clamp`) with `display: -webkit-box` for multi-line truncation. Both require the container to constrain the text (a width for ellipsis, a defined box for line-clamp), and inside flex children they require `min-width: 0` to shrink (see `layout-systems`).
- **`overflow-wrap: break-word` / `anywhere`** breaks an unbreakable string (a long URL, a code token) rather than letting it overflow. Use `break-word` for content that may contain long tokens; reserve `anywhere` (which affects min-content sizing) for cases where you need the break to shrink the container.

The trap is choosing truncation when wrapping would be correct, or letting a long URL overflow because no overflow rule was set. Decide whether the text should wrap, hyphenate, truncate, or break — these are different intents with different properties.

## Common Traps

### `font-display: block` To "Avoid" The Unstyled-Text Flash

Setting `block` to prevent a flash of fallback text, which instead hides readable content for seconds and harms LCP. The flash is less harmful than invisible text; prefer `swap` and eliminate the shift with metric-matched fallbacks.

### Visible Layout Shift From A Metric-Mismatched Fallback

Loading a web font whose fallback has a different x-height or width, so every line reflows on swap, producing CLS and a jarring jump. Match fallback metrics with `size-adjust`/`ascent-override`/`descent-override` or `font-size-adjust`.

### Inheriting One Line-Height Everywhere

Setting a global `line-height: 1.5` and letting headings inherit it, so large headings sit in loose, disconnected lines. Set line-height per typographic role — tighter for headings, looser for body.

### Forgetting `min-width: 0` So Ellipsis Silently Fails

Putting `text-overflow: ellipsis` on a flex child without `min-width: 0`, so the child cannot shrink below its content and the ellipsis never appears (the text overflows instead). This is the layout-systems overflow trap, applied to text.

### Hyphenating UI Labels Or Headings

Applying `hyphens: auto` globally so button labels, nav items, and headings break with hyphens mid-word. Hyphenation is for reading prose in narrow columns; turn it off for short labels and headings.

### Loading A Variable Font To Use One Weight

Shipping a large variable font file but only ever using the 400 weight, paying the byte cost of axes never exercised. Use a static weight when one suffices; adopt variable fonts when multiple weights or axes are genuinely needed.

### Manual `<br>` Breaks That Break At Other Viewports

Inserting `<br>` to force a two-line heading that looks right on desktop, then becomes an awkward break on mobile. Use `text-wrap: balance` and let the browser re-evaluate per viewport.

### Truncating Text That Users Need To Read In Full

Applying `line-clamp` or ellipsis to content whose full text matters (an error message, a critical value) without providing a way to expand or access it. Truncation is for previews and constrained UI; ensure truncated-but-important text has a tooltip, expand interaction, or full view.

## Self-Check

- [ ] `font-display` is set explicitly per font face (defaulting to `swap` for readable text, `optional` where a late swap is worse than the fallback, and `block` avoided for content), and above-the-fold fonts are preloaded with `crossorigin` — not left at the browser default.
- [ ] Font-swap layout shift is mitigated: the fallback font's metrics approximate the web font (via a matched fallback family, or `size-adjust`/`ascent-override`/`descent-override`/`font-size-adjust`), so swapping the font changes glyphs without reflowing lines and does not produce CLS.
- [ ] Variable fonts are used only where multiple weights or axes are genuinely exercised; a single needed weight ships as a static file, and variable fonts are subset to the used character range.
- [ ] Line-height is set per typographic role (tighter for headings, ~1.4–1.6 for body) rather than inherited globally, and vertical rhythm uses a consistent spacing scale rather than ad-hoc margins.
- [ ] Wrapping uses `text-wrap: balance` (headings/short blocks) and `text-wrap: pretty` (paragraphs) as progressive enhancement, and no manual `<br>` is used to force breaks that would break at other viewports.
- [ ] Hyphenation is applied deliberately (`hyphens: auto` with correct `lang` for narrow-column prose) and turned off for headings, button labels, and UI text where mid-word breaks are wrong.
- [ ] Overflow handling matches intent: single-line truncation uses `text-overflow: ellipsis` with `white-space: nowrap` and a constrained width; multi-line uses `line-clamp`; long unbreakable strings use `overflow-wrap: break-word`; flex children that truncate have `min-width: 0`.
- [ ] Truncated text that users need in full has a way to be accessed (tooltip, expand, full view), so truncation is not hiding critical content like error messages or important values.
- [ ] Typographic choices were verified across viewports and with the font both unloaded and loaded — confirming no orphaned headings, no overflow from long words, and no shift on font swap — not only in the designer's ideal state.
