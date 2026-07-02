---
name: language_and_codebase_migration.md
description: Use when the agent is migrating a codebase between programming languages or type systems (untyped to typed, JavaScript to TypeScript, Python 2 to 3, one language to another), adding types gradually, using automated conversion tools, planning a partial migration that coexists with legacy, or deciding when a language migration is complete.
---

# Language And Codebase Migration

A language migration is the replacement of the medium in which the codebase is written, and that medium shapes everything: the tooling, the kinds of errors the compiler catches, the idioms the team uses, the libraries available, and the way the team reasons about correctness. Migrating from an untyped language to a typed one, or from one language to another, is rarely a single conversion; it is a long coexistence between the old and new, during which the codebase is partly one thing and partly another, the tooling must handle both, and the team must hold two mental models at once. The migration succeeds or fails on whether that coexistence is manageable and whether it converges to completion — because a language migration that stalls halfway leaves the codebase permanently split, with all the cost of both languages and the benefits of neither.

Agents tend to treat a language migration as a mechanical conversion: run the codemod, fix the errors, done. This works for trivial migrations and fails for real ones, because the value of the new language (types, stricter checks, different idioms) is not produced by conversion alone — it is produced by the new language's checks actually constraining the code, which requires the migration to reach a state where the checks are meaningful and enforced. A migration that converts syntax but leaves types as `any`, or that runs alongside the old code with no path to retire it, has paid the migration's cost without capturing its benefit. The judgment problem is to migrate incrementally in a way that coexists safely with the old, to add the new language's guarantees progressively rather than as decoration, to use automation for the mechanical work and human judgment for the semantic work, and to define and drive toward a completion that actually delivers the new language's value.

## Core Rules

### Migrate Incrementally With Coexistence, Not A Freeze-And-Rewrite

A language migration that freezes the codebase for a full rewrite stalls the product, diverges from the changing needs of the business, and rarely converges — the rewrite is always "almost done" while the old codebase accrues the actual feature work. The sustainable approach is incremental migration, where parts of the codebase move to the new language while the rest continues to operate and evolve, and the boundary between old and new is managed deliberately.

Incremental migration requires:

- **A coexistence mechanism** — the ability for old and new language code to call each other (interop, type declarations for untyped modules, shared build), so the codebase functions as one system throughout the migration.
- **A migration order** — which parts move first. Prefer isolated, low-coupling modules first (to validate the tooling and the interop), then higher-coupling cores, leaving the most tangled or least valuable parts for last.
- **A rule against backsliding** — once a module is migrated, new code in it uses the new language, so the migration monotonically progresses rather than oscillating.

The coexistence period is the hard part: the team maintains two toolchains, two sets of idioms, and the interop between them. Plan for this cost, minimize it by migrating steadily, and drive toward its end rather than letting it become the permanent state.

### Add Types As Real Constraints, Not Decoration

In a migration to a typed language (or a stricter type system), the value comes from the types constraining the code — catching errors at compile time, enabling confident refactoring, and documenting contracts. A migration that converts files to the new language but fills them with escape hatches (`any`, unchecked casts, disabled checks) has changed the syntax without gaining the guarantees; the code is nominally typed but practically untyped, and the migration's benefit is unrealized.

Progressive typing discipline:

- **Allow gradual typing at the boundary**, but drive toward precision. New code and migrated code should use precise types; the `any` escape hatch is for the transition, not the destination.
- **Tighten over time.** A codebase that starts the migration with loose types should ratchet toward strictness — enabling stricter checks file by file or module by module, reducing the count of escape hatches, treating each remaining `any` as debt to eliminate rather than a permanent fixture.
- **Do not let the escape hatches become load-bearing.** If large parts of the codebase permanently rely on `any` or unchecked casts, the type system is not actually checking those parts, and the migration has not delivered its safety benefit there.

The completion criterion for a typing migration is not "the files are in the typed language" but "the types are precise enough that the compiler catches the errors the migration was meant to catch." Measure the escape hatches and drive them down.

### Use Automation For The Mechanical, Human Judgment For The Semantic

Language migrations have a large mechanical component (syntax conversion, type inference, import rewriting) that automation handles well, and a semantic component (choosing the right types, capturing invariants, redesigning APIs that fit the new language's idioms) that automation handles poorly. Conflating the two — either doing everything by hand, or trusting automation for things it cannot judge — wastes effort or introduces subtle wrongness.

Use automation appropriately:

- **Codemods and compilers for syntax and inference** — converting syntax, inferring obvious types, renaming, and rewriting imports are mechanical and should be automated. This is the bulk of the line-by-line work.
- **Human judgment for types and design** — choosing types that capture real invariants, deciding where nullability belongs, modeling domain concepts, and redesigning APIs that were shaped by the old language's limitations require understanding the code's intent, which automation lacks.
- **Review the automation's output**, especially inferred types and converted edge cases. Automation produces plausible-but-wrong results at exactly the boundaries where bugs live; treat its output as a draft to verify, not a result to trust.

A migration that automates the mechanical and invests human effort in the semantic progresses faster and produces a better result than one that does either all by hand or all by tool.

### Plan For Partial Migration That Can Coexist Indefinitely (If Needed)

Not every language migration reaches 100%. Some stall at a partial state because the remaining code is too tangled, too low-value, or too risky to migrate, and the business will not prioritize finishing. A good migration plan acknowledges this possibility and ensures that a partial migration is still a defensible steady state — that the migrated and unmigrated parts coexist cleanly, the tooling handles both indefinitely, and the team is not trapped in a broken half-state.

This means:

- **The interop must be robust for the long term**, not a temporary bridge assumed to disappear. If 20% of the code will remain in the old language, that boundary must be maintainable.
- **The new language's guarantees must hold where it is applied**, even if other parts remain untyped or old-language. A partial typing migration is valuable if the typed parts are genuinely checked; it is worthless if the untyped parts undermine the typed parts' guarantees.
- **The completion criterion may be "the valuable parts are migrated"** rather than "every file is migrated," and that should be a conscious decision, not an accidental stall.

The danger is the accidental stall: a migration that intended to complete but drifted into a permanent partial state without anyone deciding that was acceptable. Distinguish a deliberate partial end state from an unfinished one, and if the goal is completion, keep driving.

### Build Team Capability In The New Language Before Relying On It

A language migration is also a migration of team skill. The new language has different idioms, error patterns, debugging techniques, and performance characteristics, and code written in it by a team still thinking in the old language's idioms will be stilted and error-prone — "old language with new syntax," which captures none of the new language's benefits. The team must learn to think in the new language, and that learning happens through the migration itself.

Support the capability build:

- **Migrate the early modules as a learning exercise**, with review by someone fluent in the new language, so idioms are established before they propagate.
- **Document the new language's conventions** as they are discovered, so later migrations follow consistent patterns rather than each author reinventing them.
- **Do not assume fluency transfers instantly.** A team expert in the old language will write novice-quality code in the new language at first; plan for the ramp, and protect correctness through review and testing during the learning period.

### Define Completion By The New Language's Value Being Realized

The end state of a language migration is not "the files are converted" but "the new language's guarantees and idioms are actually in effect and delivering their benefit." Define completion in terms of that value: the type system catches the errors it was meant to catch (measured by the absence of escape hatches and the strictness of checks), the old language is retired where the goal was full migration, the tooling is unified, and the team operates fluently in the new language.

A migration declared complete while the old language still runs in critical paths, while types are dominated by `any`, or while the team still thinks in the old language's idioms has met a syntactic criterion but not the substantive one. Hold completion to the substantive standard, or the migration's cost will have been paid without its benefit.

## Common Traps

### The Freeze-And-Rewrite That Never Converges

Halting product work to rewrite the whole codebase in the new language diverges from business needs and rarely finishes. Migrate incrementally with coexistence so the product evolves throughout.

### Types As Decoration

Converting files to a typed language but filling them with `any` and unchecked casts changes syntax without gaining guarantees. Drive toward precise types and ratchet strictness, measuring and reducing escape hatches.

### Trusting Automation For Semantic Decisions

Codemods and inference handle syntax and obvious types well but produce plausible-but-wrong results at boundaries and for invariants. Automate the mechanical, apply human judgment to types and design, and review automation output rather than trusting it.

### The Accidental Permanent Partial Migration

A migration that intended to complete but stalled into a partial state without a decision leaves the codebase split with the costs of both languages. Distinguish a deliberate partial end state from an unfinished migration, and if completion is the goal, keep driving.

### Old-Language Idioms In New-Language Syntax

Code written by a team still thinking in the old language captures none of the new language's benefits and is error-prone. Build team capability through early reviewed migrations and documented conventions.

### Declaring Completion At Conversion Rather Than Value Realized

Calling the migration done when files are converted, while the old language still runs, types are loose, and the team is not fluent, meets a syntactic bar but not the substantive one. Define completion by the new language's guarantees being in effect.

### Interop Assumed To Be Temporary That Becomes Permanent

A coexistence bridge built as a short-term expedient, never improved, becomes a fragile permanent boundary. If coexistence will be long-term, make the interop robust; if it is temporary, retire it on schedule.

## Self-Check

- [ ] The migration proceeds incrementally with a robust coexistence mechanism (interop, declarations, shared build), a deliberate migration order (isolated modules first, tangled cores later), and a rule against backsliding — not a freeze-and-rewrite.
- [ ] Types are added as real constraints: new and migrated code uses precise types, escape hatches (`any`, unchecked casts) are tracked and driven down, and strictness is ratcheted over time rather than left loose.
- [ ] Automation handles the mechanical conversion (syntax, inference, imports) while human judgment handles types, invariants, and API design, and automation output is reviewed rather than trusted.
- [ ] If the migration may remain partial, the interop is robust for the long term, the new language's guarantees hold where applied, and the partial state is a deliberate decision rather than an accidental stall.
- [ ] Team capability in the new language was built through early reviewed migrations, documented conventions, and a planned learning ramp, rather than assuming old-language fluency transfers.
- [ ] Completion is defined by the new language's value being realized — checks are strict, escape hatches are minimal, the old language is retired where the goal was full migration, tooling is unified, and the team operates fluently — not merely by files being converted.
- [ ] The coexistence period's doubled toolchain and cognitive cost is planned for and minimized by steady progress, not accepted as permanent.
- [ ] No part of the codebase declared migrated is practically still operating under the old language's semantics (loose types, old idioms) in a way that undermines the migration's benefit.
