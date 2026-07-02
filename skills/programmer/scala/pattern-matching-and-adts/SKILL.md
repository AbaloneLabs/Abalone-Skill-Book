---
name: pattern_matching_and_adts.md
description: Use when the agent is writing Scala pattern matching, sealed traits, case classes, enums, partial functions, or exhaustive matches; modeling algebraic data types; refactoring deep if/else chains into matches; debugging MatchError at runtime, non-exhaustive warnings, sealed trait leakage across files, or unexpected performance from pattern matching on large data; or designing domain models where sum types, product types, and exhaustiveness checking must enforce invariants. Covers exhaustiveness, partial vs total functions, pattern guards, unapply extractors, immutability, and the performance and correctness tradeoffs of ADT design.
---

# Pattern Matching And Algebraic Data Types

Pattern matching over algebraic data types (ADTs) is one of Scala's most powerful features, but it is also where correct-looking code hides runtime `MatchError`s, silently breaks exhaustiveness checking, and turns a clean domain model into a fragile pile of catch-all cases. The feature is deceptive because the happy path is so easy: write a `sealed trait`, a few `case class`es, a `match`, and the compiler tells you when you forgot a case. The difficulty is everything around that happy path — unsealed hierarchies that defeat the check, partial functions that throw on unexpected input, catch-all `_` cases that swallow newly-added variants, pattern guards that make a "total" match actually partial, and ADT designs that look immutable but leak mutable state through collection fields.

Agents tend to reach for pattern matching as a syntax preference and treat exhaustiveness as a compiler checkbox rather than a design property. The judgment problem is deciding when a domain concept should be modeled as a closed sum type (where the compiler enforces totality) versus an open hierarchy (where new variants can appear and the match must be defensive), how to keep `case` clauses honest about what they assume, and how to structure ADTs so that adding a variant surfaces the right errors in the right places. Getting this wrong produces code that compiles, passes unit tests on the known inputs, and throws `MatchError` the day a new variant reaches a match that nobody updated.

## Core Rules

### Decide Closed (Sealed) vs Open Based On Whether New Variants Are A Feature Or A Bug

Exhaustiveness checking only works on closed hierarchies. A `sealed trait` (or Scala 3 `enum`) declared in a file forces the compiler to know every subtype, so a `match` that misses a case becomes a compile error rather than a runtime `MatchError`. An open `trait` gives no such guarantee — a match over it is effectively partial forever, and the compiler will not warn when a new implementation appears.

The choice is a domain decision, not a style decision.

- **Closed (sealed) when the set of variants is part of the contract.** States of a state machine, event types in a persisted event log, result types (`Success`/`Failure`), AST node kinds, payment status. Adding a variant is a deliberate change that should force every consumer to decide how to handle it. Seal it, and let the compiler do the work.
- **Open when extensibility by third parties is the point.** A plugin interface, a strategy that users implement, a `trait` meant to be mixed in elsewhere. Here exhaustiveness is impossible by design; matches must include a deliberate fallback, and that fallback must be a real decision (error, default, log) rather than `_ => ()`.

Strong choice: a payment status enum is sealed, so a new status forces every reporting query to be updated. Weak choice: leaving it unsealed "for flexibility," then shipping a `MatchError` when a new status reaches the billing path. Name whether the set of cases is fixed by the domain before writing the trait.

### Treat Exhaustiveness As A Design Property, Not A Compiler Checkbox

The compiler's "match may not be exhaustive" warning is the most valuable signal in the language, and agents routinely neutralize it. The warning fires only when (a) the scrutinee's type is a sealed hierarchy, (b) no catch-all `_` or typed wildcard is present, and (c) no pattern guard makes a case conditional. Once you write `_ => ...` or `case _ => default`, the warning disappears — but so does the protection. The next variant added to the sealed trait silently falls into the catch-all, and the bug is invisible until production.

Prefer to remove catch-alls and handle each variant explicitly. If a catch-all is genuinely needed (open hierarchy, or a deliberately permissive parser), make it loud: log the unexpected value, return an explicit error, or assert. A silent `_ => unit` is the single most common source of "why did this case do nothing" bugs in mature Scala codebases. Configure the build to treat non-exhaustive matches as errors (`-Xfatal-warnings` with the exhaustivity checker), so the protection cannot erode quietly.

### Recognize That Pattern Guards Make A Total Match Partial

A guard (`case x if x.isValid =>`) turns a syntactically exhaustive match into a semantically partial one. If the guard fails, Scala falls through to the next case; if no later case matches, you get a `MatchError` even though the compiler thought the match was exhaustive. The compiler does not reason about guard conditions, so the exhaustiveness warning gives false confidence.

When a guard is needed, ensure the match still has a real fallback for the guard-false path, or restructure so the condition lives outside the pattern (e.g., filter the input first, or move the predicate into the matched data). A common trap is `case Some(x) if predicate(x) => handle(x)` with no `case Some(_) | None =>` fallback — `Some` values that fail the predicate throw. Treat any guarded case as a place where totality is no longer guaranteed.

### Model With Sum Types For "One Of" And Product Types For "All Of"

ADTs compose two shapes: sum types ("one of these variants") expressed as sealed trait + case classes/enums, and product types ("all of these fields") expressed as a single case class. The discipline is to choose the shape that matches the domain's actual cardinality.

- Use a **sum type** when a value is exactly one of several mutually exclusive forms. `sealed trait Shape; case class Circle(r: Double) extends Shape; case class Square(side: Double) extends Shape`. Pattern matching is the natural consumer.
- Use a **product type** (case class) when a value always has all of a fixed set of fields. Do not encode "one of" as nullable fields in a single class (`class Shape(maybeRadius, maybeSide)`) — that creates invalid states (both null, both set) that the type system should have prevented.
- Compose them: a product whose field is a sum, a sum whose variants carry products. "Make illegal states unrepresentable" is the heuristic — if a combination of fields is meaningless, the type should not allow it.

Strong choice: `sealed trait Payment` with `case class Paid(amount, method) extends Payment` and `case object Pending extends Payment` makes "paid but no method" impossible. Weak choice: a single `Payment(isPaid: Boolean, method: Option[Method])` where `isPaid=false, method=Some(...)` is a nonsense state the code must defend against.

### Keep Case Classes Honest About Immutability And Equality

Case classes give you `equals`, `hashCode`, `copy`, and pattern extraction for free, but only the fields you declare participate in them. A `case class` containing a mutable field (`var`) or a reference to a mutable object (a `java.util.ArrayList`, a `Array`) breaks the semantic contract: two instances that "look equal" compare unequal after the inner state mutates, and `copy` shares the mutable reference so editing one apparent clone changes the original.

Treat case class fields as immutable in spirit even when the type system allows mutation. Use `val` fields, prefer immutable collections (`List`, `Vector`, `Map` from the standard library) over mutable ones, and if you must hold an array or Java collection, document that equality and copy are now reference-based for that field. A case class with a `var` or a shared mutable collection is a defect waiting to be discovered in a `Set` deduplication or a `Map` lookup.

### Use Partial Functions Deliberately, Not Accidentally

A `match` used as a `PartialFunction` (e.g., in `collect`, `map`, or as a `case` block `{ case x => ... }`) is only defined for the cases it lists. `collect` is safe — it skips inputs that don't match. But passing a partial function literal to `map`, `flatMap`, or as a callback that will be applied to every element throws `MatchError` on the first non-matching input. The syntax looks identical to a total function, so the partiality is invisible at the call site.

Distinguish the two contexts. Use `collect` when you want to filter-and-map by pattern. Use a total function (`x => x match { ... }` with a fallback case) when the caller expects every input handled. Never hand a `{ case ... }` literal to an API that will apply it to arbitrary input unless you have proven every input matches — and even then, prefer an explicit total match so a future input change surfaces as a compile error rather than a runtime throw.

### Prefer Extractors And Custom Patterns Only When They Earn Their Abstraction

Custom `unapply` extractors let you pattern-match against types that are not case classes, which is powerful and frequently overused. A custom extractor hides what is really a function call behind match syntax, which can allocate objects, perform I/O, or have surprising cost — unlike a case class match, which is a field access. Extractors that return `Option[T]` allocate on every match; extractors that return `Option[(A, B)]` allocate a Tuple and an Option.

Use extractors when the abstraction genuinely improves readability and the cost is acceptable (parsing, regular-expression matches, adapting external types). Avoid them on hot paths or for trivial deconstruction where a plain method call would do. Document that a custom pattern is not free the way a case class match is.

## Common Traps

### Leaving A Trait Unsealed And Relying On Exhaustiveness

Declaring `trait Shape` without `sealed`, writing a match over it, and assuming the compiler will warn about missing cases. It will not — open hierarchies cannot be checked. The match is silently partial forever, and the first new subclass that reaches it throws `MatchError`. Seal the trait (and keep subtypes in the same file) the moment you intend exhaustive matching.

### Silencing Non-Exhaustive Warnings With A Catch-All

Adding `_ => ()` or `case _ => default` to make the compiler quiet, then losing the protection that would have flagged a newly added variant. The new variant now silently hits the default, producing wrong behavior with no error. Remove catch-alls on sealed types; if a default is required, make it explicit and observable (log, metric, error).

### Assuming A Guarded Match Is Total

Writing `case Some(x) if x > 0 => ...` and trusting the exhaustiveness warning, then watching `Some(-1)` throw because no case covered the guard-false branch. Guards break totality. Either add a fallback for the guard-false path or move the predicate out of the pattern.

### Using `map` With A Partial Function Literal

Passing `list.map { case Valid(x) => x }` to a method that applies the function to every element, so the first `Invalid` throws `MatchError`. Use `collect` to filter-and-map, or write a total match with a fallback. The two syntaxes look identical at the call site; the difference is whether the caller skips non-matching input.

### Putting A Mutable Field Or Mutable Collection In A Case Class

Declaring `case class Buffer(items: ArrayBuffer[Int])`, putting instances in a `Set`, mutating the array, and finding deduplication and lookup now misbehave because equality changed after construction. Case class equality is defined by the fields' current values; mutable fields make equality time-dependent. Keep case class fields immutable, or accept that equality and `copy` are reference-based for those fields and document it.

### Modeling "One Of" With Optional Fields In A Flat Class

Using `class Payment(amount: Option[Double], method: Option[Method], error: Option[String])` instead of a sum type, then writing defensive `if` chains to reject nonsense combinations like all-`Some` or all-`None`. This produces invalid states the type system should have prevented. Use a sealed trait with one case per valid combination.

### Forgetting That `copy` Shares Mutable References

Calling `buffer.copy()` to "clone" a case class that holds an `Array` or mutable collection, then mutating the clone and seeing the original change. `copy` shares non-case-class field values by reference. Deep-copy mutable fields explicitly, or redesign to hold immutable data.

### Matching On Strings Or Numbers Without A Catch-All

Treating a `String` status field as if it were a closed enum — `status match { case "paid" => ...; case "pending" => ... }` with no fallback — then throwing when the database returns `"PAID"` or a typo. Strings are open by nature; either model the status as a sealed enum and parse at the boundary, or include a deliberate fallback that surfaces the unexpected value.

## Self-Check

- [ ] Every trait that is matched exhaustively is `sealed` (or a Scala 3 `enum`), and the decision to seal vs leave open was made explicitly against whether new variants are a feature or a bug.
- [ ] No catch-all `_` or `case _ => default` exists on a sealed-type match unless the default is a deliberate, observable decision (logged, errored, or metric'd) — not a silent `()`.
- [ ] Every `match` containing a pattern guard (`if`) has a real fallback for the guard-false path, or the predicate was moved out of the pattern; guarded matches are not trusted as total.
- [ ] Partial function literals (`{ case ... }`) are only passed to `collect` or other skip-on-no-match APIs; callers expecting total handling receive a total match with a fallback.
- [ ] Sum types model "one of" (sealed trait + cases) and product types model "all of" (case class); no "one of" domain concept is encoded as nullable fields in a flat class that allows illegal states.
- [ ] Case class fields are immutable (`val`) and hold immutable collections; any mutable field or mutable/shared collection is documented as breaking equality and `copy` semantics.
- [ ] Non-exhaustive match warnings are treated as errors in the build (`-Xfatal-warnings` or equivalent), so exhaustiveness protection cannot erode silently when a variant is added.
- [ ] Custom `unapply` extractors are used only where the abstraction earns its cost; hot-path matches use case classes or plain methods, not allocating extractors.
- [ ] A test or review confirmed that adding a new variant to each sealed trait produces compile errors at every match site that needs updating (the protection actually fires).
