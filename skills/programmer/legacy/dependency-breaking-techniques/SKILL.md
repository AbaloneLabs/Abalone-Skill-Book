---
name: dependency_breaking_techniques.md
description: Use when the agent is trying to test or refactor legacy code whose dependencies are hard-wired, choosing a specific seam technique (extract interface, parameterize constructor, parameterize method, subclass and override method, extract implementer, instance/constructor delegation, link seams, build-time seams) to break a concrete dependency, deciding whether a dependency should be broken at the object, method, link, or preprocessor level, or judging which Michael Feathers dependency-breaking technique fits a given coupling shape (direct construction, static call, global/singleton access, hardcoded configuration, library lock-in). Also covers the failure mode of choosing the wrong seam for the coupling (e.g., subclass-and-override on a final class, link seams that break unrelated callers, over-mocking when a parameterization would suffice), introducing a seam that changes behavior, and confusing the mechanics of how to introduce a seam with the strategy of what to test. Distinct from testing-strategy skills, which cover when and what to test; this skill covers the mechanical judgment of which dependency-breaking technique to apply to a specific dependency.
---

# Dependency Breaking Techniques

Legacy code resists testing and change because its dependencies are hard-wired: a class constructs its collaborators directly, a method calls a static factory, a module reads a global singleton, a function depends on a concrete library class with no interface. To test or refactor such code you must introduce a *seam* — a place where you can alter behavior without editing the code at that point — and the judgment problem is which seam technique to use for which coupling. Michael Feathers' catalog (extract interface, parameterize constructor, parameterize method, subclass and override method, extract implementer, adapt parameter, link seams) is a toolbox, not a single recipe, and the wrong choice either fails outright (subclass-and-override on a `final`/`sealed` class; a link seam that reroutes callers you do not own) or succeeds at the cost of a fragile seam that breaks unrelated code, invites deep mocking, or quietly alters production behavior.

Agents tend to reach for one or two techniques they know — usually mock-everything or extract-one-big-interface — and force every dependency through that mold. The harm appears as seams that change behavior (a parameterization that shifts initialization order, an override that skips a side effect production relies on), as seams that over-couple tests to implementation (subclass-and-override that forces tests to know private structure), and as seams that are correct but brittle (a link seam that works on one machine and breaks CI, or that breaks a second caller who never opted into the substitution). The discipline is to read the *shape* of the coupling first — is the dependency constructed in place, passed through a static, hidden behind a global, baked into the build? — then pick the narrowest technique that opens a substitution point without disturbing production behavior or unrelated callers. The seam is a means to an end, and the smallest behavior-preserving seam is almost always the right one.

This skill is distinct from the testing-strategy skills, which address *what* to test and *when*. Here the judgment is mechanical and local: given this specific dependency, which technique introduces a safe substitution point?

## Core Rules

### Match The Technique To The Shape Of The Coupling

The right seam is determined by *how* the dependency is currently reached, not by which technique you find easiest. Read the coupling before choosing.

- **Constructed in place (`new Foo()` inside the method/constructor):** prefer *parameterize constructor* (inject the collaborator, default to the production value so existing callers are unaffected) or *parameterize method* (inject per-call where construction must stay lazy or context-dependent). Subclass-and-override is a fallback when you cannot change the signature.
- **Reached through a static/global/singleton (`Config.get()`, `Clock.now()`, `Registry.instance()`):** prefer *extract and override* — wrap the static access in a protected method, then override it in a test subclass — or *introduce a static setter* seam only as a last resort, because static setters create global mutable state that leaks across tests.
- **A concrete class with no interface, where you depend on many methods:** prefer *extract interface* so the code depends on an abstraction the production class and a test double both satisfy.
- **A concrete class where you depend on one method and the class is large/foreign:** prefer *adapt parameter* (widen the parameter to an interface covering only what you call) rather than extracting a giant interface.
- **A dependency you cannot change at the source (third-party library, framework type):** prefer *extract implementer* or a thin *wrapper/facade* you own, so your code depends on your abstraction, not the foreign type.

### Choose The Narrowest Seam That Does Not Disturb Production Behavior

Every seam is a behavior-preserving transformation, and the risk is that the transformation silently changes behavior. Prefer the narrowest seam: the one that touches the least code, opens exactly one substitution point, and leaves every existing caller's behavior identical.

- **Parameterize with a production default, not a breaking signature change.** Adding a parameter that defaults to the current value keeps all callers unchanged; rewriting every caller to pass the dependency is a larger, riskier change that should be justified separately.
- **Extract interface without renaming or moving the production class.** The production class now implements the interface; nothing else moves. Renames and moves are separate refactors with their own risk and should not be bundled into seam creation.
- **Subclass-and-override only where you control instantiation.** If the production code constructs the class itself with `new`, your test subclass is never used unless you also break the construction — so subclass-and-override pairs with a construction seam, not alone.

### Respect Language And Visibility Constraints When Choosing A Technique

The catalog is language-shaped. A technique that is clean in one language is impossible or ugly in another, and ignoring this leads to seams that compile only by accident.

- **`final`/`sealed` classes defeat subclass-and-override.** In Java/Kotlin/C#, you cannot override a method on a final class; choose parameterize or extract-interface instead, or extract the logic into a non-final collaborator you can subclass.
- **`static` methods cannot be overridden.** Wrap the static call in an instance method (extract and override) or parameterize; do not pretend a test can override a static.
- **Link and preprocessor seams depend on the build system.** A link seam (substituting a library at link time) or a preprocessor/conditional-compilation seam is real but environment-fragile; it works only where you control the build, and it tends to break CI or other callers silently. Prefer object-level seams unless link/build seams are the only option (e.g., C/C++ legacy with no DI surface).

### Keep The Seam And The Behavior Change In Separate Steps

A seam is introduced so that you can then characterize or change behavior safely. Bundling the seam with the behavior change makes it impossible to tell whether a test failure is a regression or the intended new behavior, and it makes the seam itself unauditable.

- **Introduce the seam first, run the existing characterization tests, confirm behavior is unchanged.** The seam commit should be a no-op behaviorally.
- **Then write or update tests that use the seam.**
- **Then make the intended behavior change, with the seam-enabled tests guarding it.**

This sequencing is what makes a seam trustworthy. A seam introduced in the same diff as a behavior change is just a refactor whose safety cannot be verified.

### Prefer Object Seams Over Link And Static Seams For Testability

Object-level seams (parameterization, interface extraction, subclass-and-override) produce tests that are explicit, local, and runnable anywhere. Link seams, static-setter seams, and preprocessor seams produce tests that are implicit, global, and environment-dependent — they pass on one machine and fail on another, and they leak state across tests because the substitution is global.

- **Default to object seams.** They are more work to introduce but pay back in test reliability.
- **Use link/static seams only when object seams are impossible** (e.g., the dependency is woven through code you cannot recompile, or the system is C/C++ with no runtime DI surface), and isolate them so they cannot affect unrelated callers.

## Common Traps

### Choosing Subclass-and-Override On A Final Or Sealed Class

Picking the technique before reading the coupling, then discovering the class cannot be subclassed, and either giving up or weakening the production class's invariants to make it testable. Read visibility and finality first; choose parameterize or extract-interface when subclassing is blocked.

### Over-Extracting A Giant Interface For A Single Dependency

Extracting an interface that mirrors the entire concrete class because "we might need all of it," coupling tests to dozens of methods they never exercise and making the interface a maintenance burden. Extract only the methods you depend on (adapt parameter / interface segregation); widen later only if needed.

### Link Seams That Break Callers You Do Not Control

Using a link-time or build-time substitution that reroutes a shared library globally, breaking other callers who never opted into the substitution and whose failures appear far from the change. Link seams must be scoped; prefer object seams, and never apply a global link seam to a library with consumers outside your control.

### Bundling The Seam With A Behavior Change

Introducing the seam and changing behavior in one diff so a test failure cannot be attributed to regression versus intended change. Sequence strictly: seam first (behavior-preserving, tests green), then behavior change (tests guard it).

### Parameterization That Silently Shifts Initialization Order

Injecting a collaborator that was previously constructed lazily inside a method, moving its construction to the constructor, and thereby changing when side effects (a DB connection, a cache warm-up, a registration) occur. Verify initialization order is preserved; parameterize the method rather than the constructor when construction timing matters.

### Confusing Seam Mechanics With Testing Strategy

Treating "which technique" as the whole problem and skipping the question of whether the dependency is worth breaking at all, or whether a higher-level characterization test would give the same protection at lower cost. This skill supplies the mechanics; the testing-strategy skills supply the *whether* and *what*. Use both.

## Self-Check

- [ ] The coupling shape was read before a technique was chosen: in-place construction → parameterize constructor/method; static/global access → extract-and-override or (last resort) static setter; concrete dependency on many methods → extract interface; concrete dependency on few methods → adapt parameter; foreign/unmodifiable type → extract implementer or wrapper facade.
- [ ] The narrowest seam was chosen: parameterization uses production defaults so existing callers are unchanged, interface extraction does not bundle renames or moves, and subclass-and-override is paired with a construction seam (it is useless alone when production constructs the class with `new`).
- [ ] Language and visibility constraints were respected: no subclass-and-override on final/sealed classes, no override of static methods, and link/preprocessor seams are used only where object seams are impossible and scoped so they cannot break unrelated callers.
- [ ] The seam was introduced in a separate, behavior-preserving step (existing characterization tests green afterward), before any behavior change or test that uses the seam.
- [ ] Object seams are preferred over link and static-setter seams for testability; static/link seams are isolated and not applied globally to libraries with consumers outside the agent's control.
- [ ] No giant interface was extracted for a single-method dependency (interface segregation / adapt parameter used instead), and no seam silently shifted initialization order or side-effect timing.
- [ ] The highest-risk cases were verified — a final class where subclass-and-override was correctly rejected, a parameterization that preserved initialization order, a link seam that did not break an external caller, and a seam whose introduction left existing tests green — not only the easy extract-interface case.
- [ ] The judgment stayed mechanical and local (which technique for this dependency) and did not collapse into the testing-strategy question of what to test; both skills were applied where both apply.
