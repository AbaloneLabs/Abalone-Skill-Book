---
name: typescript_generics_and_inference.md
description: Use when the agent is writing TypeScript generics, adding TypeVar constraints and bounds, designing conditional and mapped types, relying on type inference, deciding how many type parameters a function needs, or reviewing whether a generic is too clever or too loose. Covers generic constraints (extends), conditional types with infer, mapped types, variance, inference pitfalls, over-generic API design, and the tradeoff between type safety and readability for generic utilities.
---

# Generics And Inference

Generics let a single definition work over a family of types while preserving the relationships between them. Done well, a generic function or type is both reusable and precisely typed at every call site. Done poorly, generics accumulate type parameters nobody can reason about, inference fails in the cases that matter, or the type machinery becomes more complex than the runtime code. The judgment problem is deciding when generics are worth introducing, how many parameters they need, what to constrain, and where to stop before the types become unreadable.

Agents tend to add generics reflexively ("make it generic so it's reusable"), over-constrain or under-constrain them, write conditional/mapped types that look impressive but defeat inference, or trust inference in cases where it widens to `any`/`unknown` silently. The harm is delayed: a generic that infers `any` provides no safety, a generic with five parameters is impossible to call without explicit annotations, and an over-clever conditional type produces error messages that obscure the real problem. The real work is introducing generics only when they preserve a real type relationship, constraining them to what the body needs, and verifying inference at realistic call sites.

## Core Rules

### Introduce Generics Only To Preserve Type Relationships

A generic is justified when it links types across a definition — a parameter and a return type, two parameters, or a container and its elements. If every type parameter could be replaced with a concrete type (or `unknown`) without losing information, the generic adds complexity without value.

- Strong: `function first<T>(xs: T[]): T` — the return type depends on the input.
- Strong: `class Box<T> { constructor(public value: T) {} }` — the box preserves the element type.
- Weak: `function log<T>(x: T): void` — `T` is unused in the output; `function log(x: unknown): void` is simpler and equivalent.

Each type parameter should appear in at least two positions (input and output, or multiple inputs), or carry a constraint that the body genuinely uses. Unused parameters are noise.

### Constrain Generics To What The Body Actually Needs

An unconstrained `T` lets you move values around but not call any method on them. Add a constraint (`<T extends SomeType>`) when the body needs specific capabilities, and constrain to the minimal interface that satisfies those needs.

- If you only need `length`, constrain to `{ length: number }`, not to `string | any[]`.
- If you need comparison, constrain to a `Comparable` interface, not to `number`.
- Prefer structural constraints (a small interface) over nominal ones (a specific class), so more types satisfy them.

Over-constraining (requiring a concrete class) defeats reuse; under-constraining (leaving `T` bare when the body calls `.map`) produces errors at every call site. Match the constraint to the real requirement.

### Let Inference Work; Annotate Only When It Cannot

TypeScript infers type arguments from call sites in most cases, and inferred types are usually what you want. Annotate explicitly only when inference is ambiguous, wrong, or absent.

- `first([1, 2, 3])` infers `T = number`; no annotation needed.
- `first<number>([])` may be needed when the argument is empty and inference would widen to `unknown`/`never`.
- `pair(1, "a")` infers a union; if you want a tuple or a specific combination, annotate or restructure.

Verify inference at realistic call sites, especially empty collections, callbacks, and overloaded functions, where inference often widens unexpectedly. A generic that requires every caller to annotate is usually misdesigned.

### Use Conditional Types And `infer` Sparingly And Purposefully

Conditional types (`T extends U ? X : Y`) and `infer` let you compute types from other types — extracting return types, unwrapping Promises, building mapped shapes. They are powerful and easy to overuse.

- Reach for them when a real relationship cannot be expressed otherwise (deriving a function's parameter type, transforming a shape).
- Keep them shallow where possible; nested conditional types with multiple `infer` branches produce incomprehensible errors and slow the compiler.
- Test them with concrete type arguments (`type T = MyUtility<Foo>;`) and inspect the resolved type, rather than assuming the machinery works.
- Document non-obvious conditional types with an example; the next reader (and the error reporter) will need it.

### Prefer Mapped And Utility Types Over Hand-Rolled Shapes

TypeScript's built-in utilities (`Partial`, `Required`, `Pick`, `Omit`, `Record`, `ReturnType`, `Parameters`) and mapped types (`{ [K in keyof T]: ... }`) express common transformations clearly. Use them instead of redefining shapes.

- `type UserUpdate = Partial<Pick<User, "name" | "email">>` is clearer than a hand-written interface.
- Mapped types over `keyof T` stay in sync when `T` changes; hand-written copies drift.
- Compose utilities rather than building one large mapped type; small named pieces are easier to read and reuse.

### Be Aware Of Variance And Its Consequences

TypeScript's variance is largely inferred and not strictly checked, which means some generic relationships that look safe are not. Arrays are invariant in principle but treated co-variantly for reads in practice, which can let unsafe assignments through.

- Do not assume `Dog[]` is safely assignable to `Animal[]` for writes; it is not sound, even though TypeScript permits the read direction.
- When designing generic containers, decide whether they are read-only (covariant is fine) or read-write (invariant is correct), and express that with `readonly` or by separating read and write interfaces.
- Avoid `T extends any` (meaningless) and unconstrained `T` in positions where variance could cause unsoundness; test with subtypes.

### Balance Safety Against Readability

Every generic adds cognitive load. A utility with four type parameters, three constraints, and a conditional return type may be perfectly type-safe and completely unusable. Decide deliberately how much type machinery a piece of code warrants:

- For widely-used library utilities, more type precision can pay off (callers benefit at every site).
- For internal one-off code, a concrete type or `unknown` is often better than a generic.
- When a generic's type signature is longer than its body, consider whether the generality is real or aspirational.

## Common Traps

### Unused Or Decorative Type Parameters

`function parse<T>(s: string): T` looks generic but `T` is unconstrained and uninferrable, forcing every caller to annotate and providing no safety. Either infer `T` from an input, validate at runtime and return a concrete type, or require a parser/decoder argument.

### Over-Constraining To A Concrete Class

`<T extends User>` when the body only needs `User`'s fields prevents passing compatible shapes and test doubles. Constrain to the minimal structural interface instead.

### Inference Widening To `any` Or `unknown`

In empty arrays, loose callbacks, and some overloads, inference picks a wider type than intended, silently disabling checks. Verify inferred types at edge call sites; annotate or provide defaults where inference is too loose.

### Conditional Types That Defeat Inference

Complex conditional types can prevent inference at call sites, forcing explicit annotations and producing errors that point at the wrong place. If callers cannot use your function without annotating type arguments, simplify the types.

### Too Many Type Parameters

A function with four or five type parameters is hard to call, hard to read, and usually a sign that the abstraction is too broad. Split the function, reduce parameters by deriving some from others, or accept a less generic design.

### Trusting `keyof` And Mapped Types Without Checking

`keyof T` on a type with optional or index-signature properties can produce surprising keys (`string | number` for index signatures). Inspect the resolved type of mapped utilities with concrete inputs before relying on them.

### Variance-Assumption Bugs

Assuming `Subtype[]` is assignable to `Supertype[]` for writes leads to runtime type errors that the compiler did not flag. Treat arrays as invariant for write-heavy use, or separate read-only views.

### Generic Defaults That Hide Intent

`<T = any>` provides a default that can silently make a generic behave like `any` when the caller omits the argument. Use defaults deliberately and document them; an accidental `any` default defeats the generic's purpose.

## Self-Check

- [ ] Every type parameter appears in at least two positions or carries a constraint the body uses; no decorative parameters.
- [ ] Constraints are minimal and structural (`{ length: number }`, a small interface), not over-constrained to concrete classes.
- [ ] Inference works at realistic call sites (including empty collections, callbacks, overloads); explicit annotations are needed only where inference is ambiguous or too wide.
- [ ] Conditional types and `infer` are shallow, tested with concrete arguments, and documented where non-obvious.
- [ ] Built-in utilities (`Partial`, `Pick`, `Omit`, `Record`, `ReturnType`) and mapped types are preferred over hand-rolled shapes.
- [ ] Variance is considered for generic containers; read-only vs read-write is expressed via `readonly` or separated interfaces.
- [ ] The number of type parameters is proportional to the real generality; functions with many parameters were split or simplified.
- [ ] No generic defaults to `any` without intent; defaults are documented and do not silently disable checking.
- [ ] Resolved types of generic utilities were inspected with concrete inputs, not assumed correct.
- [ ] The generic's complexity is justified by its reuse; internal one-off code uses concrete types instead of over-general generics.
