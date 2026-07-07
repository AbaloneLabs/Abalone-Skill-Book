---
name: visual_consistency.md
description: Use when the agent is establishing or reviewing visual consistency across a product, defining and applying a shared visual language of type, color, spacing, and component styling, deciding when consistency matters and when variation is justified, managing consistency across teams and surfaces, diagnosing inconsistency, or ensuring that visual consistency serves usability and brand coherence rather than uniformity for its own sake.
---

# Visual Consistency

Visual consistency is the property by which a product looks and behaves as if a single hand designed it, and it is the quiet foundation of both usability and brand trust. When a product is visually consistent, users learn its patterns once and apply them everywhere: a primary button looks like a primary button, a danger action is colored like every other danger action, and a heading is sized like every heading at its level. When consistency breaks, users must relearn the interface in every new screen, trust erodes, and the product starts to feel assembled from parts rather than designed as a whole. The harm is rarely a single jarring moment; it is the accumulated friction of a thousand small inconsistencies.

Agents tend to fail visual consistency in predictable ways. They optimize each screen in isolation, so locally perfect decisions produce a globally incoherent product. They enforce consistency so rigidly that legitimate variation, such as a distinct treatment for a genuinely different context, becomes impossible. They confuse consistency with uniformity, applying the same treatment everywhere even where the content or task differs. Or they fail to define the shared visual language precisely enough that two designers implementing the same intent produce visibly different results.

Use this skill before establishing a visual language, when applying it across surfaces, and when reviewing a product for consistency. The goal is a shared visual language that is precise enough to produce coherence, flexible enough to allow justified variation, and governed so that it stays consistent as the product grows.

## Core Rules

### Define The Visual Language Precisely

Consistency is impossible without a shared definition. If the visual language, the type scale, color tokens, spacing values, component styling, and motion rules, lives only in designers' heads, every implementation diverges. A precise, shared definition is the prerequisite for any consistent product.

Define the language precisely:

- codify the type scale, color tokens, spacing scale, elevation, radius, and motion as named, documented values;
- express the language as tokens and components that design and code consume together, so there is one source of truth;
- document the rules for applying the language, not just the values, so intent travels with the tokens;
- version the language so that changes are deliberate and their effects traceable.

A visual language that is not tokenized and documented fragments the moment more than one person works on the product.

### Distinguish Consistency From Uniformity

Consistency means applying the same solution to the same problem; uniformity means applying the same solution to every problem. The two are different, and confusing them produces products that are rigid where they should be flexible and varied where they should be consistent. Legitimate variation, where a genuinely different context warrants a different treatment, is not inconsistency; it is appropriate design.

Apply consistency with judgment:

- be consistent where the problem is the same, so a primary action always looks like a primary action;
- allow variation where the context genuinely differs, such as a distinct treatment for a marketing surface versus an application surface;
- document the conditions for variation, so that differences are intentional and governed rather than ad hoc;
- resist uniformity that ignores context, because forcing one treatment onto every situation produces awkward fits.

### Apply The Language Across All Surfaces

A product's surfaces are often owned by different teams, and consistency breaks most often at the seams between them. The marketing site, the application, the settings, the emails, and the help center each drift toward their own visual language, and the user experiences the product as a patchwork. Consistency must be enforced across surfaces, not only within each one.

Govern across surfaces:

- extend the visual language to every surface the user encounters, including email, marketing, and support;
- define where surfaces may legitimately differ, such as a more expressive marketing site, and where they must align;
- assign ownership of the language so that cross-surface drift has someone responsible for correcting it;
- audit across surfaces periodically, because drift accumulates silently.

### Optimize For The Whole Product, Not The Single Screen

The most common consistency failure is local optimization: a decision that makes one screen better while making the product less coherent. A designer improves a button in isolation, a team refines a card for their feature, and each improvement, taken alone, is defensible, but the product as a whole loses its shared language.

Optimize globally:

- evaluate each visual decision against the whole product, not only the screen it appears on;
- when a local improvement would break consistency, either generalize it to the language or reject it;
- prefer a slightly weaker global solution to a slightly stronger local one, because coherence compounds;
- make changes to the shared language deliberately, so that improvements propagate rather than fragment.

### Make Consistency Serve Usability, Not Just Aesthetics

Visual consistency is a usability property, not only an aesthetic one. Consistent styling lets users transfer learning from one part of the product to another, reduces cognitive load, and builds trust that the interface behaves predictably. Consistency pursued only for visual elegance misses its functional value.

Connect consistency to usability:

- keep interactive elements visually consistent so users can recognize what is clickable, editable, or dangerous everywhere;
- maintain consistent meaning for color, icon, and styling, so a red treatment always signals the same kind of action;
- preserve consistent layout patterns, so users find primary actions and navigation in expected places;
- treat consistency as a way to reduce the user's learning burden, not only to make the product look unified.

### Diagnose And Correct Inconsistency Systematically

Inconsistency is rarely noticed through individual screens; it is noticed when the product is reviewed as a whole. Systematic diagnosis, through audits and reviews that compare treatments across the product, is how drift is caught before it becomes entrenched.

Diagnose systematically:

- conduct visual audits that compare type, color, spacing, and component treatments across screens;
- look for the same intent rendered differently, which is the signature of inconsistency;
- trace each inconsistency to its cause, whether a missing token, an undocumented rule, or an ungoverned team;
- correct at the language level where possible, so a single fix resolves many instances.

### Govern The Language As It Evolves

A visual language that does not evolve becomes a straitjacket, and one that evolves without governance becomes noise. Governance is the practice of changing the language deliberately, evaluating proposed additions and modifications against the whole, so that evolution strengthens coherence rather than eroding it.

Govern evolution:

- review proposed changes to the language for their effect on the whole product, not only the requesting feature;
- document and communicate changes so that all teams update together;
- deprecate outdated treatments explicitly, with a migration path, rather than letting them linger;
- assign clear ownership so that the language has stewards accountable for its coherence.

### Balance Consistency With Innovation

A product that never deviates from its language can feel stagnant, and legitimate innovation sometimes requires breaking a convention. The skill is allowing innovation while containing its effects, so that a justified departure becomes a controlled evolution rather than uncontrolled drift.

Balance the two:

- treat deviations as proposals to the language, to be either generalized or rejected, rather than as one-off exceptions;
- contain experimental treatments to clearly bounded contexts, so they do not silently propagate;
- review whether a deviation should become the new standard, raising consistency for everyone, or remain a special case;
- document the rationale for allowed deviations, so future teams understand why the exception exists.

## Common Traps

### Optimizing Each Screen In Isolation

Locally perfect decisions produce a globally incoherent product when each screen is improved without reference to the whole.

### Confusing Consistency With Uniformity

Forcing one treatment onto every context ignores legitimate variation and produces awkward fits.

### An Undefined Or Implicit Visual Language

A language that lives only in designers' heads fragments the moment more than one person implements it.

### Drift Across Surfaces And Teams

Different teams owning different surfaces drift toward separate languages, producing a patchwork product.

### Inconsistent Meaning For The Same Styling

When the same color or treatment means different things in different places, users cannot trust their learning.

### Ungoverned Evolution

A language that changes without review becomes noise; one that never changes becomes a straitjacket.

### One-Off Exceptions That Propagate

Ungoverned deviations silently spread, turning a single exception into a new inconsistency.

### Consistency Pursued Only For Aesthetics

Treating consistency as a visual concern misses its function in reducing cognitive load and building predictable interaction.

## Self-Check

- [ ] The visual language is defined precisely as tokenized, documented, versioned values shared by design and code.
- [ ] Consistency is distinguished from uniformity, with documented conditions for legitimate variation.
- [ ] The language extends across all user-facing surfaces, including marketing, application, email, and support.
- [ ] Visual decisions are evaluated against the whole product, not only the screen they appear on.
- [ ] Consistency serves usability, with interactive elements, color meaning, and layout patterns consistent enough to transfer learning.
- [ ] Inconsistency is diagnosed systematically through visual audits that compare treatments across screens.
- [ ] The language is governed as it evolves, with proposed changes reviewed for whole-product effect and communicated to all teams.
- [ ] Deviations are treated as proposals to the language, contained, documented, and either generalized or rejected.
- [ ] Outdated treatments are deprecated explicitly with a migration path, rather than left to linger.
- [ ] Ownership of the visual language is clear, so that drift has stewards accountable for correcting it.
