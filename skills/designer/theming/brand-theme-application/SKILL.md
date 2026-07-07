---
name: brand_theme_application.md
description: Use when the agent is applying a brand to a product interface, translating brand guidelines into UI tokens, balancing brand expression with usability, adapting logos and brand color to screens, or ensuring brand consistency across surfaces without sacrificing legibility and hierarchy.
---

# Brand Theme Application

Applying a brand to a product is not pasting a logo and a primary color onto every screen. It is translating brand identity, voice, and emotion into interface decisions, color, typography, spacing, motion, and imagery, while keeping the product usable, accessible, and hierarchical. A brand applied literally becomes noise; a brand applied thoughtfully becomes recognition. The failure modes are opposite but equally common: the product looks generic with no brand presence, or it looks on-brand but is hard to use.

Use this skill before applying brand guidelines to a product UI, building a branded theme from a brand book, adapting a logo or brand color to interactive surfaces, balancing brand expression with accessibility, or ensuring brand consistency across many product surfaces. The goal is to prevent the agent from either stripping the brand out for safety or forcing the brand in so hard that usability, contrast, and hierarchy suffer.

## Core Rules

### Translate The Brand Into Interface Decisions, Not Just Colors

A brand book usually contains logo, primary color, typography, and tone of voice. The interface needs more: how the brand feels in spacing, density, motion, imagery style, corner radius, and surface treatment. Decide how the brand personality maps to UI qualities:

- a precise, technical brand may favor tight grids, sharp corners, restrained motion;
- a warm, friendly brand may favor softer radius, generous spacing, gentle easing;
- a bold, energetic brand may favor strong contrast, larger type, assertive color blocks;
- a premium, minimal brand may favor whitespace, restrained palette, subtle motion.

Make these mappings explicit so the brand is expressed consistently, not only through the logo.

### Decide Where The Brand Lives And Where It Steps Back

Not every surface should carry equal brand weight. Decide the brand presence hierarchy:

- marketing and landing surfaces can carry full brand expression;
- onboarding and empty states can use brand illustration and voice;
- core product surfaces should carry brand through subtle, structural cues;
- dense operational surfaces should let content dominate and brand recede.

Forcing the same brand intensity everywhere flattens hierarchy. A dashboard screaming brand color in every cell is harder to use than one that uses brand accents sparingly.

### Protect Legibility And Hierarchy Over Brand Purity

Brand colors and brand typography are often chosen for print, marketing, or logos, not for body text on screens. A brand color may fail contrast as text. A brand display typeface may be unreadable at small sizes. The right response is not to abandon the brand, but to define where each brand element is appropriate:

- use brand display type for headings and hero moments, system or neutral type for body and UI;
- use brand color for accents, buttons, and focus, and neutral colors for text and dense data;
- use brand imagery for empty states and marketing, restrained treatment for operational surfaces.

Document the boundary so the brand stays recognizable without breaking usability.

### Handle The Logo And Brand Marks Deliberately

Logos fail in interfaces when they are treated as decoration. Decide the rules for:

- minimum clear space around the mark;
- placement on light, dark, image, and colored backgrounds;
- acceptable logo variants and when each applies;
- behavior on small screens and in dense headers;
- when the full lockup is required versus a compact mark.

Avoid stretching, recoloring, or repositioning the logo per screen. Consistency of the mark is a core part of brand recognition.

### Adapt Brand Color To Accessibility Constraints

Brand colors frequently fail contrast guidelines, especially as text on backgrounds or as status indicators. Rather than discarding the brand color, define accessible variants:

- a darker or lighter shade of the brand color for text use;
- a brand-tinted neutral for backgrounds and surfaces;
- a separate accessible accent for links and focus if the brand color cannot meet ratio.

Keep the brand recognizable while meeting contrast. It is acceptable to have a brand color for fills and a derived accessible variant for text.

### Keep Brand Voice Consistent Across Copy And States

Brand application extends to language. Error messages, empty states, tooltips, button labels, and onboarding copy all carry brand voice. Decide the voice rules:

- formality level;
- use of humor in serious contexts;
- terminology and product naming;
- how errors and failures are framed.

Inconsistent voice, especially joking in error states or cold copy in onboarding, undermines the brand more than a wrong color.

### Ensure Brand Consistency Across Surfaces And Platforms

The brand must feel the same on web, mobile, email, and embedded surfaces. Differences in spacing, radius, color, and type across platforms erode recognition. Define a core brand application that each platform adapts without losing identity. Pay attention to:

- email clients that ignore styles;
- native components that resist custom theming;
- third-party surfaces with limited brand control;
- print and PDF outputs.

### Plan For Co-Brand And White-Label Cases

If the product may be co-branded, resold, or white-labeled, the brand application must be separable from the core UI. Keep brand-specific tokens distinct from structural tokens so a partner brand can replace them without redesigning the interface.

## Common Traps

### Brand Color Everywhere

Flooding the interface with the brand color flattens hierarchy, breaks contrast, and makes status colors indistinguishable. Brand accents should be intentional, not default.

### Brand Typeface For Body Text

Display typefaces chosen for logos and headlines often fail at small sizes and long reading. Forcing them into body copy hurts legibility while gaining little brand value.

### Logo As Decoration

Treating the logo as a styling element, resizing or recoloring it per screen, weakens recognition and often violates brand guidelines.

### Ignoring Accessibility To Stay On-Brand

Refusing to adjust a brand color that fails contrast produces an interface that is on-brand but unusable for part of the audience. Accessible variants preserve both brand and usability.

### Generic UI With A Logo Stuck On

The opposite failure: a completely neutral interface with only a logo to signal brand. The product feels unbranded and forgettable.

### Inconsistent Voice Across States

Playful onboarding copy paired with cold, technical error messages breaks brand trust. Voice must be governed as carefully as color.

### No Plan For Cross-Platform Drift

When each platform interprets the brand differently, recognition collapses. A core application standard is needed before platform adaptation.

## Self-Check

- [ ] The brand is translated into spacing, motion, radius, imagery, and density decisions, not only color and logo.
- [ ] Brand presence is scaled by surface type, with full expression on marketing and restrained accents on operational screens.
- [ ] Brand colors and typefaces are used where they are legible, with accessible variants defined where the raw brand value fails contrast.
- [ ] The logo has documented rules for clear space, placement, variants, and behavior across backgrounds and screen sizes.
- [ ] Brand voice rules cover error states, empty states, onboarding, and microcopy, not only marketing headlines.
- [ ] The brand application is consistent across web, mobile, email, and embedded surfaces, with a documented core standard.
- [ ] Brand-specific tokens are separable from structural tokens so co-brand or white-label cases can swap brand without redesign.
- [ ] Status colors remain distinguishable and are not overwhelmed by brand color usage.
- [ ] Body text and dense data use legible neutral type, with brand display type reserved for headings and hero moments.
- [ ] The interface was reviewed for both recognition (does it feel on-brand?) and usability (is it still legible and hierarchical?).
