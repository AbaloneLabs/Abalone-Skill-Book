---
name: semantic_and_functional_color.md
description: Use when the agent is assigning meaning to color in an interface, defining success, warning, error, and information states, mapping colors to interactive versus static elements, building semantic color tokens, separating brand from functional color, or ensuring color meaning stays consistent and unambiguous across components and themes.
---

# Semantic And Functional Color

Color in an interface does work. Beyond decoration, it signals state, priority, interactivity, and outcome. When that signaling is consistent, users learn it quickly and navigate by glance. When it is inconsistent, the same color means success in one place and danger in another, and users stop trusting color as a signal at all. The deepest problem is not choosing the right hue for "error"; it is building a system where color meaning is defined once, applied everywhere, and survives across components, states, and themes without contradiction.

Use this skill before assigning colors to states, defining semantic tokens, mapping interactive affordances, separating brand from functional color, or auditing an interface for color-meaning consistency. The goal is to prevent the agent from using colors decoratively where they carry meaning, reusing a brand color as a status, or letting each component invent its own color logic until the system contradicts itself.

## Core Rules

### Define Semantic Meaning Once, Apply Everywhere

Semantic color is a contract. Once a color means "error," it must mean error everywhere: in forms, toasts, banners, icons, text, and borders. The moment the same red appears on a decorative illustration, a brand accent, and a destructive button, the contract breaks and users can no longer rely on the signal.

Establish semantic categories first, success, warning, error, information, and optionally neutral emphasis, and assign each a dedicated color used for nothing else. Document the meaning, then enforce it across every component. If a design needs a color that is not semantic, draw from the brand or neutral palette, never from the semantic set.

### Separate Brand Color From Functional Color

Brand colors express identity; functional colors express state. These goals conflict. A brand red used proudly on a primary button becomes ambiguous the moment red also means error, because users cannot tell whether the red button is a brand action or a destructive one. The cleanest systems keep brand and functional palettes distinct, even when they share a hue family.

Decide deliberately:

- which brand colors appear as interactive fills, and whether they collide with any semantic meaning;
- whether a brand color that resembles a status needs a functional variant to remove ambiguity;
- where brand expression is decorative and can use the full palette, versus where it is structural and must defer to function.

### Make Interactivity Legible Through Color And More

Color signals whether an element is interactive, but color alone is fragile. Links, buttons, and selectable items should combine color with shape, weight, underline, cursor, or affordance so that interactivity remains clear when color is missed, such as by a color-blind user or on a low-quality display.

Reserve the strongest interactive color, often the primary brand, for the single most important action on a screen. Spreading the primary color across many equally weighted elements destroys the signal of which action matters most.

### Build Semantic Tokens, Not Raw Values

Hardcoding semantic colors per component guarantees drift. A "success" green typed into twelve components will, over time, become twelve slightly different greens as each is tweaked independently. Semantic tokens, such as color-status-success, centralize the meaning so that updating the value updates every instance, and so that the meaning is explicit in the code rather than implied by a hex value.

Tokens should encode meaning, not appearance. Name them by role, color-status-error, not by hue, color-red-500, so that the value can change, including across themes, without breaking the semantic contract.

### Preserve Semantic Meaning Across Themes And States

Semantic colors must keep their meaning in every theme and state. The success green that reads clearly on a light background can become muddy or decorative on a dark one; the error red that is obvious at rest can disappear on a hovered or disabled element. Each semantic color needs variants for hover, active, disabled, focus, and for each theme, and the meaning must survive all of them.

For every semantic color, verify:

- it remains distinguishable from every other semantic color in the same theme;
- it survives hover, active, disabled, and focus states without losing meaning;
- it carries enough contrast for text and icons, not only large fills;
- it is reinforced by icon or text so meaning never depends on color alone.

### Reinforce Meaning Beyond Color

Because color perception varies, semantic meaning should never depend on color alone. Pair every status color with an icon, a text label, or a shape so that users who cannot distinguish the color still understand the state. This is not only an accessibility requirement; it is a robustness requirement, because color signals fail on bad displays, in print, in screenshots, and under unusual lighting.

### Avoid Overloading A Single Hue With Multiple Meanings

A common failure is assigning one hue to several unrelated roles. Blue becomes link, information, primary brand, and selected state, until users cannot tell what any given blue element means. Where a hue must serve multiple roles, separate them by context and reinforcement, and prefer giving each distinct meaning its own color where the palette allows.

## Common Traps

### Reusing A Brand Color As A Status

A brand red used for both identity and destructive actions makes every red element ambiguous, eroding the meaning of both.

### Semantic Color Used Decoratively

Drawing decorative illustrations or accents from the semantic palette pollutes the signal, teaching users that the color no longer reliably means what it claims.

### Color Meaning Invented Per Component

When each component defines its own status colors, the values drift and the meanings contradict across the product.

### Tokens Named By Hue Instead Of Role

Naming a token color-red-500 hides its meaning and forces every theme change to re-derive semantics, breaking the contract.

### Interactive Color Spread Equally

Using the primary color on many equally weighted actions destroys the signal of which action is most important.

### Status Color That Loses Meaning In Dark Or Disabled States

A semantic color tuned only for the resting light state becomes muddy, invisible, or misleading in other themes and states.

### Relying On Color Alone For Meaning

Status conveyed only by color fails for color-blind users, low-quality displays, screenshots, and print, where the signal vanishes.

## Self-Check

- [ ] Each semantic category, success, warning, error, information, has a dedicated color used for that meaning and nothing else.
- [ ] Brand colors and functional colors are separated where their goals conflict, with deliberate variants to remove ambiguity.
- [ ] Semantic colors are defined as role-named tokens, not raw hex values or hue-named tokens.
- [ ] The strongest interactive color is reserved for the single most important action on each screen.
- [ ] Every semantic color was verified across hover, active, disabled, focus, and each theme, and keeps its meaning in all.
- [ ] Status meaning is reinforced by icon, text label, or shape, so it never depends on color alone.
- [ ] No single hue is overloaded with unrelated meanings without clear contextual separation.
- [ ] Semantic colors are distinguishable from each other and from brand and accent colors under the palette's relationships.
- [ ] The semantic contract is documented and enforced across all components, not left to per-component invention.
- [ ] Updating a semantic value updates every instance through tokens, without requiring manual correction across the product.