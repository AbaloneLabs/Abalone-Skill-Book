---
name: typescript_type_system_design.md
description: Use when the agent is designing TypeScript types and interfaces, choosing between type aliases and interfaces, modeling unions and intersections, simulating nominal typing with branding, reasoning about structural compatibility and excess property checks, or reviewing whether a type design is too loose or too clever. Covers structural typing consequences, branded/opaque types, discriminated union design, type vs interface tradeoffs, and the safety-vs-complexity balance in type modeling.
---

# Type System Design

TypeScript's type system is structural: types are compatible based on their shape, not their name. This is powerful and convenient, but it means two types with the same shape are interchangeable even when they represent different domain concepts, and a type that looks precise can be quietly permissive. The judgment problem is modeling domain concepts as types that catch real mistakes without becoming so clever that the next reader cannot follow them.

Agents tend to reach for the first shape that satisfies the compiler — often a bag of optional fields, or a union with no discriminator — and to add types reactively rather than designing them. The harm is delayed: a `UserId` and an `OrderId` both typed as `string` get swapped silently, an object with extra fields passes a type check it should fail, a "precise" type built from mapped and conditional types becomes unmaintainable. The real work is choosing the right modeling tool (interface vs type, union vs intersection, branded primitive), understanding what structural typing permits and forbids, and balancing safety against readability.

## Core Rules

### Design For Structural Typing, Don't Fight It

Structural typing means compatibility is about shape. A value is assignable to a type if it has all the required members with compatible types, regardless of the value's declared type name. Design with this in mind:

- Define types as the minimal set of properties a consumer needs (interface segregation), so unrelated types that happen to share those properties can substitute — this is a feature for test doubles and adapters.
- Remember that excess properties are allowed on assignment from a variable (`const x: Foo = someObject` where `someObject` has extra fields is fine) but checked on object literals (`const x: Foo = { extra: 1 }` errors). This asymmetry surprises people; design APIs so the literal case catches the mistakes that matter.
- Do not try to enforce "this type only" through structural means alone; if identity matters, use branding (below).

### Choose `type` And `interface` By What You Need

Both can describe object shapes; the differences are about capabilities, not style alone.

- **`interface`**: open for declaration merging (same-name interfaces combine), supports `extends`, and error messages name the interface. Good for extensible object contracts and library APIs consumers may augment.
- **`type` alias**: supports unions, intersections, tuples, mapped/conditional types, and primitives. Good for combining types (`A & B`), unions (`A | B`), and anything that is not a plain object shape.

There is no universal winner. Use `interface` for plain extensible object contracts; use `type` for unions, intersections, and computed types. Consistency within a codebase matters more than the specific choice.

### Model "One Of Several Shapes" As Discriminated Unions

When a value can be one of several distinct shapes, model it as a union with a shared literal property (the discriminant), not as a single type with many optional fields. Discriminated unions enable exhaustive narrowing and make invalid states unrepresentable.

Strong:

```ts
type Result =
  | { status: "ok"; value: number }
  | { status: "error"; message: string };
```

Weak: `{ status: string; value?: number; message?: string }`, which permits `status: "ok"` with a `message` and no `value` — an invalid state the compiler cannot catch.

The discriminant should be a `Literal` (`"ok" | "error"`, not `string`), and every member should carry exactly the fields valid for that case. See the narrowing skill for how to narrow these safely.

### Use Branding For Nominal Distinction When Identity Matters

Because TypeScript is structural, two `string`-based ids are interchangeable. When mixing them is a real bug (user id vs order id, sanitized vs unsanitized HTML, Celsius vs Fahrenheit), simulate nominal typing with branding:

```ts
type UserId = string & { readonly __brand: "UserId" };
```

Construct branded values through a validation function that checks the input and casts once at the boundary. Downstream code then cannot accidentally pass an `OrderId` where a `UserId` is required. Reserve branding for cases where confusion is likely and costly; over-branding every primitive adds ceremony without value.

### Prefer Union And Intersection Composition Over Inheritance Hierarchies

Deep class inheritance and complex `extends` chains are often better expressed as composition of small types with `&` (intersection) and `|` (union). Composition is more flexible, easier to refactor, and avoids the tight coupling of inheritance.

- Build a type from parts: `type User = BaseEntity & Contactable & Timestamps`.
- Prefer "has-a" relationships (a type that includes another type's fields) over "is-a" class hierarchies for data shapes.
- Use classes when you need runtime behavior + identity + polymorphism; use type composition for data shapes.

### Make Invalid States Unrepresentable Where The Cost Is Worth It

The strongest argument for careful type design is preventing states that should never exist: a "pending" order with a `shippedAt` date, a logged-out user with a session token, a result that is both ok and error. Model these as unions or separate types so the compiler rejects the invalid combination.

Balance this against complexity. A type that perfectly models every domain rule but requires three levels of generics to read has traded one class of bugs (invalid states) for another (maintainability). Prefer to eliminate the highest-cost invalid states and accept looser modeling for low-cost ones.

### Keep Public Types Stable And Narrow

Types exported from a library or module are a contract. Once consumers depend on a type's shape, changing it is a breaking change. Design public types to be:

- **Narrow**: only the fields consumers need, not your full internal entity.
- **Stable**: prefer additive evolution (new optional fields) over changes to existing fields.
- **Owned deliberately**: define boundary types in the module that owns the contract, not in whichever module happens to produce or consume them (see the module-boundary-design skill).

Internal types can be loose and refactored freely; public types cannot.

## Common Traps

### Optional Fields Where A Union Was Meant

`{ status?: "ok" | "error"; value?: number; message?: string }` permits every combination, including no status, or a value with an error status. A discriminated union eliminates these invalid states. Reach for unions whenever fields are mutually dependent.

### `string` For Everything That Happens To Be Text

User ids, email addresses, URLs, enum-like values, and localized strings are all `string`, so they are interchangeable and offer no compile-time protection. Use `Literal` unions for closed sets (`"asc" | "desc"`), branding for distinguishable ids, and precise types for structured text where confusion is costly.

### Excess Property Checks Giving False Confidence

Excess property checks apply only to object literals, not to variables. `const x: User = obj` where `obj` has extra fields does not error. If your design depends on rejecting extra fields at runtime (e.g., before storing), you need a runtime check, not just a type annotation.

### Over-Clever Mapped And Conditional Types

Types built from deep `infer`, recursive conditional types, and template literal types can be correct but unreadable, and they produce incomprehensible error messages. If a type needs a comment explaining the mechanism, consider whether a simpler type plus a runtime check would be clearer. Optimize for the reader and for compiler performance (very complex types slow the language server).

### Inheritance Where Composition Was Better

`class AdminUser extends User extends Person extends Entity` chains couple unrelated change reasons and make testing hard. Compose with intersection types or composition for data; reserve `extends` for genuine polymorphic behavior.

### Treating `any` As A Type

`any` disables type checking entirely and is assignable to and from everything, so it silently propagates. Prefer `unknown` (a safe top type that forces narrowing before use) when you must accept an unknown value, and eliminate `any` at boundaries with runtime validation. See the strict-mode skill.

### Declaration Merging Surprises

Two `interface Foo` declarations in the same scope merge silently. This is useful for augmentation but can cause surprising combined shapes if you did not intend it. Use `type` aliases (which do not merge) when you want a single fixed definition.

### Unions That Are Not Truly Discriminated

A union without a common literal property requires manual `in` checks or casts to narrow, and exhaustiveness checks do not work. Always include a discriminant so narrowing is mechanical and exhaustive.

## Self-Check

- [ ] Object shapes are modeled with the right tool (`interface` for extensible contracts, `type` for unions/intersections/computed types); the choice is consistent within the codebase.
- [ ] "One of several shapes" is a discriminated union with a literal discriminant, not a bag of optional fields; invalid combinations are unrepresentable.
- [ ] Distinguishable primitives (ids, units, sanitized vs raw) use branding where confusion is costly; branding is not applied to every primitive indiscriminately.
- [ ] Data shapes use composition (intersection/union) over deep inheritance; classes are reserved for behavior + identity + polymorphism.
- [ ] Public/exported types are narrow, stable, and owned by the contract module; internal types are free to change.
- [ ] Excess property checks are not relied upon for runtime rejection of extra fields; runtime validation is used where it matters.
- [ ] `any` is absent or justified; `unknown` is used for genuinely unknown input and narrowed before use.
- [ ] No type expression is so complex it needs a paragraph to explain; mapped/conditional/recursive types are used sparingly with readability in mind.
- [ ] Unions all carry a discriminant so narrowing and exhaustiveness checks work mechanically.
- [ ] The highest-cost invalid states are eliminated by the type design; lower-cost looseness is accepted deliberately.
