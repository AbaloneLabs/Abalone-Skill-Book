---
name: php_legacy_refactoring.md
description: Use when the agent is refactoring a legacy PHP codebase (PHP 4/5 era, untyped, include-based, no Composer, register_globals, mysql_ functions, globals and superglobals everywhere, mixed HTML and PHP, no tests), planning a modernization (adding types, strict_types, migrating to Composer autoloading, upgrading PHP versions, extracting classes from spaghetti), introducing tests before refactoring, or is diagnosing "refactoring legacy code breaks things in production", "cannot add types without changing behavior", or upgrade-path paralysis. Covers the legacy modernization strategy, characterization tests, the strangler-fig pattern, safe type and autoload introduction, version upgrade sequencing, and the pitfalls of big-bang rewrites and unsafe changes.
---

# Legacy Refactoring In PHP

Legacy PHP codebases are common and uniquely treacherous because they combine several sources of fragility: no type declarations (so the compiler will not catch a wrong argument), include-based structure with no autoloading (so dependencies are implicit and order-dependent), heavy use of globals and superglobals (`$_GET`, `$_POST`, `global $db`) so state is invisible and shared, mixed PHP-and-HTML pages so logic and presentation are entangled, deprecated/removed extensions (`mysql_*`, `ereg`, `create_function`), and almost never a test suite. Refactoring such code by intuition — "I'll just add types" or "I'll rewrite this module" — breaks production, because the existing behavior often depends on undocumented quirks (a function that silently accepts a string where an int was intended, a global set as a side effect of an include). The judgment problem is to refactor *safely*: to establish a safety net of characterization tests before changing behavior, to modernize incrementally with the strangler-fig pattern rather than a big-bang rewrite, to introduce types and autoloading without changing runtime behavior, and to sequence version upgrades so each step is verifiable.

Agents attempt big-bang rewrites ("let's rebuild this in Laravel") that stall or diverge, or they add `strict_types` and types naively and change behavior (a parameter that received `int|string` now rejects strings), or they refactor without tests and ship regressions. The remedy is characterization tests first, incremental change, and behavior-preserving transformations verified at each step.

## Core Rules

### Establish Characterization Tests Before Changing Behavior

Before refactoring legacy code, capture its *current* behavior with tests, even if that behavior is buggy — these are characterization tests, not correctness tests. They pin what the code does today so a refactor that changes it fails loudly. For untested code, write integration/feature tests around the entry points (an HTTP request, a CLI invocation, a function call with representative inputs) that assert on the observable output/state. Where you cannot test from the outside, extract the logic into a testable function first (a behavior-preserving extraction), then test it. Without this net, refactoring is guessing.

- Write characterization tests asserting current output for representative inputs (including the weird cases the code handles oddly).
- Test from the outside in (HTTP/CLI/function boundary) where possible; extract-to-testable where not.
- Treat the existing behavior as the spec, even the bugs, until you deliberately change them.

### Use The Strangler-Fig Pattern, Not A Big-Bang Rewrite

A big-bang rewrite ("rebuild the whole app in a new framework") almost always stalls, diverges from the evolving old system, and ships late or never. The strangler-fig pattern modernizes incrementally: new features are built in the new structure (Composer, a framework, typed classes), and old code is gradually wrapped or replaced, with both old and new running together until the old is gone. Route new requests to the new code; leave old routes on the legacy code; migrate one route/feature at a time, each verifiable. This keeps the system working throughout and lets you stop at any point.

- New work in the new structure; old code stays until migrated.
- Migrate incrementally (one route, one module, one feature at a time), each step shippable.
- Keep old and new running together (a front controller that routes to each) until the old is fully replaced.

### Introduce Types And strict_types Without Changing Behavior

Adding type declarations to legacy code is valuable but risky: a parameter that historically received mixed types (an `int` sometimes passed as a numeric string) will, with a declared `int` parameter under `strict_types`, reject the string and break the caller. To add types safely: first add types *without* `strict_types` (PHP coerces, preserving behavior for most cases), verify in production, then enable `strict_types` per-file after auditing call sites. Declare one file/parameter at a time, run the test suite (characterization tests catch coercion changes), and watch for `TypeError`s in production monitoring. Prefer widening types (`int|string`) where the code genuinely accepts multiple types, rather than narrowing to the "intended" type and breaking callers.

- Add types without `strict_types` first (coercion preserves behavior); enable `strict_types` later per-file after auditing.
- Widen types (`int|string`) where the code accepts multiple; do not narrow to an "intended" type that breaks callers.
- Add types incrementally (one parameter/file), verified by tests and production monitoring for `TypeError`s.

### Migrate From include-Based To Composer Autoloading Safely

Legacy projects use `require`/`include` chains with implicit, order-dependent side effects (an include that sets a global or registers a class). Migrating to PSR-4 autoloading requires extracting classes into files named and namespaced per PSR-4, removing the manual includes, and verifying the side effects (a global set in an include must be set somewhere explicit after the move). Do this incrementally: introduce Composer and PSR-4 for new classes, move existing classes one at a time (each move verified by tests), and replace includes with autoload as classes are moved. Beware includes that run code at include time (side effects) — move that code to an explicit initialization point.

- Introduce Composer + PSR-4 for new classes first; move existing classes one at a time.
- Replace side-effecting includes with explicit initialization (do not lose a global set in an include).
- Verify each move with tests; do not bulk-rename and hope.

### Sequence PHP Version Upgrades With Deprecations As A Guide

Upgrading PHP versions (5.6 → 7.x → 8.x) is itself a refactoring, because each version removes deprecated features (`mysql_*` in 7.0, `create_function` in 7.2, the `${}` string interpolation in 8.2, various in 8.x). Use the upgrade guide and deprecation notices (run on the current version with deprecation warnings enabled) to find the breakages before upgrading. Fix deprecations on the current version first (so the code is forward-compatible), then upgrade one minor version at a time, verifying at each step. Tools (PHP CodeSniffer with PHPCompatibility, Rector) automate finding and fixing version-specific issues.

- Enable deprecation warnings on the current version; fix them before upgrading.
- Upgrade one minor version at a time, verifying (tests, production monitoring) at each step.
- Use PHPCompatibility/Rector to find and fix version-specific breakages.

### Untangle Logic From Presentation Incrementally

Mixed PHP/HTML pages (`<table><?php while ($row = ...) { ?><tr>...` ) entangle logic and presentation so neither is testable or changeable. Separate incrementally: extract the data-fetching into a function/class that returns data, leave the presentation in a template that consumes it, route through a front controller. Do not attempt to convert a whole page to a framework in one step; extract the logic first (testable), then introduce a template, then route via a controller. Each step is behavior-preserving and verifiable.

- Extract data logic into a function returning plain data; test it.
- Introduce a template consuming the data; route via a controller.
- One page/section at a time, each step shippable.

## Common Traps

### Big-Bang Rewrite Stalling Or Diverging

"Rebuild in Laravel" takes months, the old system keeps changing, the new diverges. Use strangler-fig; modernize incrementally.

### Adding strict_types And Changing Behavior

A parameter that received numeric strings now rejects them under `strict_types`, breaking callers. Add types without `strict_types` first; widen where multiple types are accepted.

### Refactoring Without Tests

Changing untested legacy code ships regressions. Write characterization tests first.

### include Side Effects Lost On Autoload Move

An include that set a global or registered a class; moving to autoload drops the side effect. Move side effects to explicit initialization.

### Upgrading PHP Versions In One Jump

Skipping versions hits removed features en masse. Upgrade one minor at a time, fixing deprecations first.

### Narrowing Types To "Intended" And Breaking Callers

Declaring `int` where callers pass strings breaks them. Widen to `int|string` or coerce deliberately.

### Untangling A Whole Page At Once

Converting a mixed PHP/HTML page to a framework in one step breaks behavior. Extract logic, then template, then controller, incrementally.

### Trusting "Obvious" Refactors In Untyped Code

Without types, a rename or signature change can break callers invisibly. Verify with characterization tests and static analysis (Psalm/PHPStan).

## Self-Check

- [ ] Characterization tests capture the legacy code's current behavior (including quirks) before any refactor, so behavior changes fail loudly.
- [ ] Modernization uses the strangler-fig pattern (new work in the new structure, old migrated one route/module/feature at a time), not a big-bang rewrite.
- [ ] Types are introduced without `strict_types` first (coercion preserves behavior), widened where multiple types are accepted, and `strict_types` enabled per-file only after auditing call sites and watching for `TypeError`s.
- [ ] Composer/PSR-4 autoloading is introduced for new classes and existing classes moved one at a time, with include side effects relocated to explicit initialization.
- [ ] PHP version upgrades proceed one minor version at a time, deprecations fixed on the current version first, with PHPCompatibility/Rector used to find breakages.
- [ ] Logic is untangled from presentation incrementally (extract data logic → template → controller), one page/section at a time, each step behavior-preserving.
- [ ] Static analysis (Psalm/PHPStan at a low level initially) runs alongside the test suite to catch changes untyped code hides.
- [ ] The refactoring has been considered under no-test-start, mixed types, side-effecting includes, version removals, and rollback, and remains safe and incremental.
