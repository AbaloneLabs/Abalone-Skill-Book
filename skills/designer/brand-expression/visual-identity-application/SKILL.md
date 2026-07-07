---
name: visual_identity_application.md
description: Use when the agent is applying a visual identity across product surfaces, defining color, typography, spacing, logo usage, component styling, theming, dark mode, and responsive behavior, and ensuring identity is applied as a coherent system rather than ad hoc per screen.
---

# Visual Identity Application

A visual identity exists as a set of tokens, logos, and guidelines, but it lives only when applied. Application is where identity succeeds or fails, because it is where abstract rules meet concrete constraints: a button that must work in a dense toolbar, a logo that must survive a tiny favicon, a color that must remain accessible on five different backgrounds, a type scale that must hold across mobile and desktop. The judgment problem is applying identity as a system — consistent, scalable, and governed — rather than re-interpreting it fresh on every screen until the product looks like many products.

Use this skill when applying, extending, or auditing a visual identity across product surfaces, components, and themes. The goal is to prevent the agent from applying identity ad hoc, producing drift between screens, and from treating tokens as suggestions rather than as the contract that holds the product together.

## Core Rules

### Apply Identity Through Tokens, Not Hardcoded Values

The foundation of coherent identity application is the token layer: named references for color, spacing, typography, radius, elevation, and motion that sit between the brand definition and the implementation. Components and screens consume tokens, not raw values.

A token system typically includes:

- **Primitive tokens**: raw palette and scale values (for example, blue-500, space-4).
- **Semantic tokens**: role-based names that map primitives to meaning (for example, color-action-default, space-component-padding).
- **Component tokens**: names tied to specific components (for example, button-background-primary).

Semantic tokens are what make theming, dark mode, and rebrand possible without rewriting every screen. When a screen uses a hardcoded hex value or a raw pixel measurement, the identity has been broken at that point, and every such break is a future maintenance cost.

### Define Color By Role And Validate Contrast

Color in identity is rarely one value; it is a set of roles with relationships. Define each color by what it does, not only by what it looks like.

Roles to define:

- primary action and brand accent;
- surface, background, and elevated surface;
- text, secondary text, and disabled text;
- success, warning, error, and information states;
- borders, dividers, and focus indicators.

For every role, validate contrast against the backgrounds where it appears. A brand color that passes contrast on white may fail on a tinted surface, in dark mode, or behind an image. Identity application is incomplete until contrast is checked in every real context, not just on a neutral canvas.

### Build A Type Scale With Clear Roles

Typography application fails when every screen picks its own sizes. Define a type scale with named roles and apply them consistently.

A robust type system specifies:

- a limited scale of sizes with consistent ratio;
- named roles (display, heading, title, body, caption, label);
- weight, line height, and letter spacing per role;
- responsive behavior, so roles scale predictably across breakpoints;
- fallback and web font loading strategy so typography is stable, not jarring.

Resist adding one-off sizes for special cases. Each exception weakens the scale and makes the next screen harder to design consistently.

### Govern Logo Usage And Clear Space

The logo is the most literal identity element and the most often misapplied. Define how it may and may not be used.

Logo rules to specify:

- minimum sizes for full and simplified versions;
- clear space around the logo that no element may invade;
- approved background colors and treatments;
- variants for light, dark, and low-contrast contexts;
- prohibited modifications (stretching, recoloring, rotating, adding effects);
- favicon and app icon derivatives that preserve recognition at tiny sizes.

Logo misuse is highly visible because the logo is the identity's signature. A stretched, low-contrast, or cramped logo signals carelessness about the brand more loudly than almost any other error.

### Design Theming And Dark Mode As First-Class

Identity application must account for theme from the start, not as a late inversion. Dark mode is not white text on black; it requires a remapped palette, adjusted elevation, reduced contrast for large areas, and reconsidered imagery.

Theming rules:

- every semantic color needs light and dark variants;
- elevation and shadows behave differently on dark surfaces;
- saturated brand colors often need desaturation in dark mode to avoid vibrating;
- borders and dividers need distinct treatment to remain visible without glaring;
- images and illustrations need theme-aware variants where they have baked-in backgrounds.

If theming is bolted on after components are built, it produces inconsistencies that are expensive to fix. Build tokens theme-aware from the first component.

### Maintain Consistency Across Responsive Breakpoints

Identity must hold as the viewport changes. Spacing, type, and component proportions should scale on a defined rhythm, not be re-decided at each breakpoint.

Define:

- how spacing scales from mobile to desktop (often a multiplier or a distinct scale);
- which type roles change size across breakpoints and which stay fixed;
- how component padding and density adjust for touch versus pointer;
- how layout grids and margins shift while preserving alignment relationships.

A product that looks carefully designed on desktop and cramped or loose on mobile has applied identity inconsistently across breakpoints.

### Document Application Rules And Audit Against Them

Identity application is only as strong as its governance. Document the token system, component styling rules, logo usage, theming approach, and responsive behavior, then audit real screens against the documentation.

Audit for:

- hardcoded values that bypass tokens;
- contrast failures in real contexts;
- off-scale typography;
- logo misuse;
- theme inconsistencies;
- responsive drift.

Drift is inevitable as teams grow; only disciplined auditing keeps the applied identity coherent over time.

## Common Traps

### Hardcoding Values Instead Of Using Tokens

Every hardcoded color, size, or spacing is a point where the identity cannot be themed, rebranded, or maintained without manual hunting.

### Validating Contrast Only On Neutral Backgrounds

A brand color that passes on white can fail on tinted surfaces, in dark mode, or over imagery. Contrast must be checked in real contexts.

### One-Off Typography Sizes

Each ad hoc size weakens the scale and multiplies inconsistency across screens.

### Logo Misuse

Stretching, crowding, or placing the logo on unapproved backgrounds is the most visible sign of identity carelessness.

### Bolted-On Dark Mode

Treating dark mode as inverted light mode produces vibrating colors, lost elevation, and broken imagery.

### Re-Deciding Spacing And Type At Every Breakpoint

Without a responsive rhythm, the product looks like different designs on different devices.

### No Auditing, So Drift Accumulates

Without periodic audits against documented rules, the applied identity fragments as the team and product grow.

## Self-Check

- [ ] Color, spacing, typography, radius, elevation, and motion are applied through a token layer with primitive, semantic, and component levels.
- [ ] Every color role has defined variants, and contrast was validated in every real context including dark mode and over imagery.
- [ ] A limited type scale with named roles governs all text, with responsive behavior and stable font loading.
- [ ] Logo usage rules cover minimum size, clear space, backgrounds, variants, and prohibited modifications, including favicon and app icon derivatives.
- [ ] Theming and dark mode are designed as first-class, with theme-aware tokens, adjusted elevation, and theme-aware imagery.
- [ ] Spacing, type, and component proportions follow a defined responsive rhythm across breakpoints.
- [ ] Application rules are documented, and real screens are audited periodically for hardcoded values, contrast, scale, logo, theme, and responsive drift.
- [ ] The identity was reviewed as an applied system across multiple surfaces, not approved one screen at a time.
