---
name: modern_java_features.md
description: Use when the agent is writing or migrating Java code using modern language features — records (Java 16+), sealed classes/interfaces (Java 17+), pattern matching for instanceof (16+) and switch (21+), switch expressions (14+), text blocks (15+), var/local variable type inference (10+), record patterns (21+), enhanced switch, or migrating legacy code (POJOs, type checks, verbose switch statements) to these features. Covers when records vs classes, sealed type modeling, pattern matching exhaustiveness, var readability, text block indentation, and the tradeoffs of adopting each feature. Use when deciding whether to migrate existing code or choose these features for new code.
---

# Modern Java Features

Between Java 8 and Java 21, the language gained more new features than in the previous fifteen years combined: `var` (10), `switch` expressions (14), text blocks (15), records (16), pattern matching for `instanceof` (16) and `switch` (21), sealed classes (17), and record patterns (21). These features are not syntactic sugar — they change how you model data and control flow. Records replace boilerplate-heavy POJOs with a concise value-semantic type. Sealed types let you express closed hierarchies that the compiler can check for exhaustiveness. Pattern matching eliminates the cast-after-`instanceof` dance and enables declarative, type-driven dispatch. Together they move Java toward a more declarative, data-oriented style.

The judgment problem is not "how do I write `record Foo(int x)`" but "which of these features genuinely improves this code, which would obscure it, and when migrating, which transformations are safe versus which change semantics." Agents tend to either ignore these features (writing 2024 Java as if it were Java 8, with 60-line POJOs and chains of `instanceof`-and-cast) or over-apply them (making everything a record even when mutability or identity matters, using `var` everywhere until types are unreadable, forcing every `if`-chain into a pattern switch). The cost is code that is either needlessly verbose or subtly wrong — a record used where a mutable class was intended, a sealed hierarchy that breaks when a new subtype is added without recompiling, or a `var` whose inferred type surprises the reader.

## Core Rules

### Use Records For Value Data, Classes For Entities With Identity Or Mutation

A `record` is a transparent carrier of data: it auto-generates a constructor, accessors, `equals`, `hashCode`, and `toString`, all based on its component list. It is the right choice for value types — DTOs, configuration objects, command/query results, tuple-like returns — where the object's identity is its data and two records with equal components are interchangeable. Records are final and immutable by default, which makes them safe to share, cache, and use as map keys without the hazards of mutable objects.

Records are the wrong choice when the object has meaningful identity beyond its data (a JPA `@Entity` that represents a specific row in the database — two `User` objects with the same fields but different IDs are not equal), when it must be mutable (a builder accumulating state, a domain object with lifecycle transitions), or when it participates in inheritance (records cannot extend other classes, though they can implement interfaces). For these, use a class. The test: if you would implement `equals` based on all fields, use a record; if `equals` should be identity-based (default `Object.equals`) or based on a subset (an ID), use a class.

A common mistake is migrating a mutable POJO to a record by adding `with`-style methods — this works but often signals that the type should remain a mutable class. Records shine for immutable data; forcing mutation onto them via defensive copies and `with` methods produces verbose code that a mutable class would express more clearly.

### Model Closed Hierarchies With Sealed Types And Check Exhaustiveness

A `sealed` class or interface declares its permitted subtypes (`permits Foo, Bar, Baz`), and the compiler enforces that only those subtypes (which must be `final`, `sealed`, or `non-sealed`) can extend it. This closes the hierarchy: the set of subtypes is known and finite, which lets the compiler check that a `switch` over the type covers all cases. This is the foundation of type-driven dispatch in modern Java.

Sealed types are the right tool when the set of variants is closed and stable — a result type (`Success | Failure`), a command type in a state machine, a node type in an AST, a domain event. They are wrong when the set of variants is open or extensible by third parties — a plugin system where anyone can add a subtype, or a domain model where new variants are added frequently without recompiling the base. For open hierarchies, use a regular interface or abstract class.

The exhaustiveness benefit is real and valuable: when you add a new subtype to a sealed interface, every `switch` over that interface that does not handle the new case becomes a compile error, forcing you to update all dispatch sites. Without sealing, adding a subtype silently leaves switches incomplete, and the missing case is discovered at runtime. The tradeoff is that sealing couples the base type to its subtypes (they must be in the same module or package), which limits independent evolution.

### Use Pattern Matching To Replace instanceof-And-Cast Chains

Pattern matching for `instanceof` (`if (obj instanceof Foo f) { ... use f ... }`) eliminates the cast that traditionally followed the type check. Pattern matching for `switch` (Java 21+) goes further: it dispatches on type, binds variables, and supports guards (`case Foo f when f.x() > 0`), replacing chains of `if-else instanceof` with a declarative switch that the compiler checks for exhaustiveness (especially over sealed types).

The benefit is not just brevity — it is correctness. The traditional `instanceof`-cast chain has a subtle bug: the variable bound by the cast can go out of sync if the code is refactored (you check `instanceof Foo` but cast to `Bar`). Pattern matching binds the correctly-typed variable directly, eliminating that mismatch. And pattern `switch` over a sealed type gives compile-time exhaustiveness checking that an `if-else` chain cannot.

Use pattern matching when you have type-based dispatch over a small set of types, especially sealed types. Do not force every conditional into a pattern switch — a simple `if (x instanceof Foo)` with one branch is clearer as an `if` than as a switch. And prefer polymorphism (a method on each subtype) over external type-based dispatch when the behavior belongs to the type itself; pattern matching is for when the behavior is external to the type (a visitor, a serializer, a renderer).

### Use var For Local Type Inference Where The Type Is Obvious

`var` lets the compiler infer the local variable's type from its initializer, reducing verbosity: `var list = new ArrayList<String>();` instead of `ArrayList<String> list = new ArrayList<>();`. It is valuable when the type is obvious from the right-hand side (a constructor call, a literal) or when the exact type is an implementation detail that the reader does not need to know (`var stream = list.stream().filter(...)`).

`var` is harmful when it hides a type the reader needs to know: `var result = service.process(input)` leaves the reader guessing whether `result` is a `ProcessResult`, a `Future<ProcessResult>`, an `Optional<ProcessResult>`, or something else — they must hover or navigate to the method to understand the code. The rule: use `var` when the type is obvious from the initializer or is an unimportant implementation detail; use an explicit type when the type carries information the reader needs (a return type of a complex method, a diamond-incompatible generic, a `Collection<Foo>` where the concrete type would be misleading).

Never use `var` with diamond inference on an interface type where the concrete type matters: `var list = new ArrayList<>();` infers `ArrayList`, not `List`, which changes the declared type and can cause surprising behavior (if later code assigns a `LinkedList`, it fails). When you want `List`, write `List<String> list = new ArrayList<>();`. `var` "locks in" the inferred concrete type, which is sometimes wrong.

### Use Text Blocks For Multi-Line Strings, And Understand Indentation

Text blocks (`"""..."""`) provide a clean way to write multi-line string literals — SQL queries, JSON, HTML, shell scripts — without escape sequences and concatenation. They are strictly better than concatenated string literals for multi-line content. The key subtlety is incidental whitespace management: text blocks preserve the content's indentation relative to the closing `"""`, which means the visual indentation of the text in source code is stripped, but you must understand the rules to avoid surprising leading or trailing whitespace.

The closing `"""` delimiter controls the indentation: the compiler removes the same number of leading spaces from every line as the indentation of the closing delimiter. If the content needs leading whitespace that is deeper than the closing delimiter, use `String.indent(n)` or `String.stripIndent()` to adjust. For JSON and SQL where leading whitespace is irrelevant, this is not a problem; for formats where it matters (YAML, Makefiles), be careful.

Text blocks do not handle interpolation (use `String.formatted()` or `MessageFormat` on the result), and they do not escape backslashes by default (a regex like `\d` must be `\\d` unless you are on Java 15+ where text blocks can be combined with the new escape processing). For interpolated strings, consider `StringTemplate` (preview in Java 21) or template engines for complex cases.

### Adopt Features Incrementally And Consistently Within A Codebase

Modern features are most valuable when adopted consistently. A codebase where some DTOs are records and others are 60-line POJOs is inconsistent and harder to navigate. The migration guidance: for new code, default to records for value types, sealed types for closed hierarchies, pattern matching for type dispatch, `var` for obvious locals, and text blocks for multi-line strings. For existing code, migrate opportunistically — when you touch a class, convert it to a record if it is a value type; when you touch an `instanceof` chain, convert it to pattern matching — rather than doing a risky big-bang migration.

Be aware of version constraints: if the codebase targets an older JDK, not all features are available (records require 16+, sealed 17+, pattern switch 21+). Check the `--release` flag and the actual runtime before using a feature. Also be aware that some features interact: sealed types + pattern switch give exhaustiveness checking, which is the full benefit; using pattern switch without sealing loses the compile-time exhaustiveness guarantee.

## Common Traps

### Using A Record For An Entity With Identity

A JPA `@Entity` as a record breaks because records' `equals`/`hashCode` are component-based, but entity equality should be ID-based. Use a class for entities; records are for value types.

### Forgetting That Records Are Final And Cannot Extend Classes

A record cannot extend another class (it already extends `java.lang.Record`). It can implement interfaces, but if you need inheritance, use a class or a sealed interface with record implementations.

### Sealing A Hierarchy That Third Parties Need To Extend

Sealed types couple the base to its subtypes; if external code needs to add variants, sealing prevents it. Use a regular interface for open extension points.

### var Hiding An Important Return Type

`var result = service.process()` hides whether the result is a value, a `Future`, or an `Optional`. Use an explicit type when the return type carries information the reader needs.

### var With Diamond Inferring The Concrete Type

`var list = new ArrayList<String>()` infers `ArrayList`, not `List`; later assigning a `LinkedList` fails. Write `List<String> list` when the interface type matters.

### Pattern Switch Without Sealing Losing Exhaustiveness

A pattern `switch` over a non-sealed type is not checked for exhaustiveness; adding a subtype silently leaves switches incomplete. Seal the type to get compile-time checking.

### Text Block Indentation Surprising The Reader

The closing `"""` controls whitespace stripping; misplacing it produces unexpected leading spaces or stripped indentation. Test the output for formats where whitespace matters (YAML, Makefiles).

### Migrating To Features The Runtime Does Not Support

Records (16+), sealed (17+), pattern switch (21+). Check `--release` and the deployment JDK before using; a feature that compiles may fail at runtime on an older JVM.

## Self-Check

- [ ] Records are used for value types (DTOs, configs, results) where equality is component-based; classes are used for entities with identity, mutable state, or inheritance needs — no JPA entity or mutable accumulator is a record.
- [ ] Sealed types are used for closed, stable hierarchies (results, commands, AST nodes, domain events) where exhaustiveness checking adds value; open extension points use regular interfaces, not sealed types.
- [ ] Pattern matching (`instanceof` binding, pattern `switch`) replaces `instanceof`-and-cast chains and type-based `if-else` dispatch; polymorphism is preferred when the behavior belongs to the type itself.
- [ ] Pattern `switch` is used over sealed types so the compiler checks exhaustiveness; non-sealed type switches include a default or are documented as intentionally non-exhaustive.
- [ ] `var` is used where the type is obvious from the initializer or is an unimportant implementation detail; explicit types are used when the return type carries information the reader needs, and `var` is not combined with diamond inference when the interface type matters.
- [ ] Text blocks are used for multi-line strings (SQL, JSON, HTML) with correct incidental-whitespace management via the closing delimiter; interpolation is handled via `formatted()` or template engines, not expected from the text block itself.
- [ ] Feature adoption is consistent within the codebase (all value DTOs are records, not a mix) and respects the target JDK version (`--release`); no feature is used that the deployment runtime does not support.
- [ ] Existing code is migrated opportunistically (convert on touch) rather than via risky big-bang rewrites, and migrations preserve semantics (a record's component-based equality is correct for the type being converted).
