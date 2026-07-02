---
name: type_classes_and_implicits.md
description: Use when the agent is designing type classes in Scala, writing given/using clauses or Scala 2 implicits, using context bounds, deriving type class instances with Magnolia or Scala 3 derivation, defining implicit conversions, or debugging ambiguous implicits, diverging implicit expansion, or wrong-instance resolution; also when refactoring dependency injection, organizing given imports, or deciding between type classes and subtyping for a polymorphism problem. Covers resolution priority, orphan instances, implicit conversion hazards, and the readability cost of heavy implicit machinery.
---

# Type Classes And Implicits

Type classes are Scala's mechanism for ad-hoc polymorphism — the ability to add behavior to a type after the fact, without modifying the type or its inheritance hierarchy. Done well, they give you `Eq`, `Show`, `Ord`, `Monoid`, JSON serialization, and `Functor` instances that compose cleanly and are checked at compile time. Done poorly, they produce the most insidious bugs in the language: the compiler silently picks the wrong instance, two instances are equally valid and resolution is ambiguous, a conversion fires when you did not ask for it and changes a value's meaning, or a derivation expands forever and the build hangs. The feature's danger is that the failure is invisible in the source — the code says one thing and the compiler does another, decided by resolution rules the author often did not intend.

Agents tend to reach for implicits as a convenient way to pass context (database handles, execution contexts, configs) and to define type classes by analogy to Haskell, without internalizing that Scala's resolution is governed by priority rules, import scope, and inheritance that interact in non-obvious ways. The judgment problem is deciding when a problem actually needs a type class versus plain subtyping or a plain parameter, how to place instances so the right one always wins and the wrong one never compiles, and where the boundary is between elegant abstraction and code that no reader can trace. The harm of getting this wrong ranges from a confusing build error to a silently-wrong runtime behavior where the "obvious" instance was never the one selected.

## Core Rules

### Reach For A Type Class Only When Ad-Hoc Polymorphism Is The Real Requirement

A type class is the right tool when you need to define behavior for types you do not own, when the set of types is open, or when behavior should be specified per-type at the use site (a `Monoid` for `Int` under addition vs multiplication). It is the wrong tool when subtyping would do (you own the types and they share a real interface), when a plain parameter would do (you are just passing a dependency), or when the "polymorphism" is really one or two call sites that could take a function argument.

The cost of a type class is resolution complexity and readability. Every reader must mentally run the resolution rules to know which instance applies, and every change to instance scope can silently shift behavior. If a plain `trait` with `extends` expresses the same thing more directly, prefer it. Strong choice: a `JsonWriter[A]` type class because you want to add JSON support to types you do not control and the set of serializable types is open. Weak choice: a `UserRepository` "type class" with one production instance and one test instance — that is a dependency, pass it as a parameter.

### Place Instances To Make Resolution Deterministic And Orphan-Free

The compiler selects instances by a priority order: companion objects of the type, companion objects of the type class, and explicit imports, with more specific instances beating more general ones. The practical consequence is that instance placement determines which instance wins, and misplacement produces either ambiguity (two equally-valid instances) or the wrong instance silently winning.

- **Canonical instances live in the type class companion or the type companion.** An `Eq[Int]` belongs in `object Eq` or `object Int`'s companion. This is where the compiler looks first and where readers expect to find it.
- **Avoid orphan instances** — an instance of `TypeClass[A]` defined in neither `TypeClass`'s companion nor `A`'s companion. Orphans compile fine, but they cause ambiguity the moment two libraries define the same orphan, and they make resolution depend on which imports are in scope. If you cannot put the instance in a companion (you own neither the type class nor the type), put it in a dedicated package object and document that users must import it deliberately.
- **One canonical instance per type per type class.** Defining two `Eq[Int]` instances and selecting between them by import is a maintenance hazard; readers cannot tell which is active. If two behaviors are needed (sum vs product `Monoid`), make them distinct types or named values, not competing instances.

Strong choice: `given Ord[User]` lives in `object User`, so it is always in scope wherever `User` is, and there is exactly one. Weak choice: scattering `given`s across service objects and relying on import order to pick the right one.

### Prefer Scala 3 `given`/`using` Over Scala 2 Implicit Parameters

Scala 3 reformed the syntax specifically to make the mechanism more readable and less surprising. `using` parameters are clearly marked at the definition and call site (or elided only when the compiler can infer them), `given` declarations are visually distinct, and the `summon[A]` function is explicit where Scala 2's `implicitly[A]` was easy to confuse with the parameter syntax. New code should use Scala 3 syntax; when maintaining Scala 2, follow the same discipline the new syntax enforces — name your implicits, group them in companion objects, and avoid implicit conversions.

The deeper lesson from the redesign is that implicit parameters are a context-passing mechanism, not a way to hide arguments for brevity. If a reader cannot tell what context a function needs without tracing resolution, the abstraction has failed. Prefer explicit `using` clauses at the call site when the context is important to correctness (a transaction, a serialization format), and reserve elision for truly mechanical context (an `ExecutionContext` whose identity does not affect the result).

### Treat Implicit Conversions As Almost Always A Mistake

An implicit conversion (`given Conversion[A, B]` in Scala 3, `implicit def a2b(a: A): B` in Scala 2) inserts a conversion the caller did not write, changing the type and possibly the meaning of an expression without any visible call site. They are convenient and they are the source of the most surprising Scala behaviors: a method that "works" because of a conversion you forgot existed, a performance cliff because a conversion allocates on every element, or a silent semantic shift where `String` becomes `Int` via parsing and a malformed value throws deep in a chain.

Use explicit conversions (`.toInt`, `.toString`, a named method) unless the conversion is genuinely transparent and lossless (a value class to its underlying type, a subtype upcast). Scala 3 requires `import scala.language.implicitConversions` or a language flag to enable them precisely to make this a deliberate choice. Extension methods (Scala 3 `extension`, Scala 2 implicit classes) cover the legitimate use case (adding methods to a type) without the semantic hazards of type-changing conversions.

### Use Context Bounds For Readable Type-Class Constraints

A context bound (`def sum[A: Monoid](xs: List[A])`) is shorthand for a `using Monoid[A]` parameter and is the idiomatic way to declare that a function requires a type class instance. It keeps the type-class constraint in the type signature where readers expect it, rather than buried in the parameter list. Use context bounds when the instance is needed only to satisfy the constraint (you will `summon` it inside); use an explicit `using` parameter when you need to name the instance or pass it along.

The trap is over-using context bounds to hide dependencies that are not really type-class constraints. A `def process[A: Database, B: Logger]` that uses the "instances" as service handles is abusing the syntax to avoid passing dependencies explicitly. Context bounds are for type-class polymorphism; if the "instance" is a service, pass it as a parameter.

### Derive Instances Deliberately, And Know What Derivation Cannot See

Scala 3 derivation (`derives Eq`, `derives Show`) and libraries like Magnolia generate type class instances for case classes and sealed traits automatically by combining instances of the fields' types. This is a huge productivity win for boilerplate-heavy instances like `Eq`, `Show`, and JSON, but it has limits: derivation only works for types whose structure is visible at the derivation site, it can produce surprising instances for types with custom equality needs, and recursive types can cause derivation to diverge.

Derive for mechanical instances (structural equality, structural show, default JSON) and override with hand-written instances where the semantics differ (a `User` whose equality should ignore a timestamp, a type whose JSON shape should differ from its field layout). Do not assume a derived instance is correct because it compiles — review what it actually produces for types with non-obvious semantics.

### Make Resolution Debuggable By Naming Instances And Avoiding Ambiguity

When resolution fails or picks the wrong instance, the error messages are notoriously hard to read. The defense is authoring: name every `given` (Scala 3 `given myInstance: Eq[Int] with ...`), keep one canonical instance per type per class, and structure imports so the active instance is obvious. When debugging, use `summon[TypeClass[A]]` at the use site to confirm which instance the compiler selects before assuming the logic is wrong.

A common failure is "I added an instance and now there are two and resolution is ambiguous" — which is the compiler protecting you from silently picking one. Do not resolve ambiguity by deleting one instance blindly; resolve it by deciding which instance is canonical and removing the other, or by making the two behaviors distinct named values.

## Common Traps

### Defining Orphan Instances That Cause Ambiguity Later

Putting `given JsonWriter[Money]` in a service object because it was convenient, then a second library defines its own `JsonWriter[Money]` and every multi-import site becomes ambiguous. Put instances in the type class companion or the type companion; if neither is possible, isolate the instance in a package object and document the required import.

### Using Implicit Conversions To "Fix" Type Mismatches

Adding `given Conversion[String, UserId]` so a function that takes `UserId` accepts a raw `String`, then watching a malformed ID propagate as a parsed value that throws far from the call site. Use explicit constructors (`UserId.from(string)`); the conversion hides the failure boundary.

### Relying On Import Order To Select Instances

Importing `import instances.v1._` in one file and `import instances.v2._` in another so the "same" type class resolves differently per file. This makes behavior depend on which imports are active and is unreadable. Have one canonical instance per type per class; if two behaviors are needed, make them distinct types.

### Abusing Context Bounds To Hide Service Dependencies

Writing `def checkout[A: PaymentGateway: Inventory: Tax]` to avoid passing services explicitly, turning what should be clear dependency injection into type-class machinery. Services are dependencies; pass them as parameters. Context bounds are for type-class polymorphism, not DI.

### Assuming Derived Instances Match Hand-Written Semantics

Using `derives Eq` on a `case class User(id, name, updatedAt)` and discovering equality is broken because two users with different `updatedAt` never compare equal. Derivation is structural; review derived instances for types whose semantics differ from structural equality.

### Letting Implicit Search Diverge On Recursive Types

Deriving an instance for a recursive ADT where the derivation machinery expands the same instance infinitely, producing a build hang or a stack overflow during compilation. For recursive types, write the instance by hand with explicit recursion, or guide derivation with the library's recursion support.

### Treating `implicitly`/`summon` Calls As Free

Calling `implicitly[ExpensiveType]` inside a hot loop, not realizing each call runs resolution and may construct the instance. Capture the instance once (`val ev = summon[T]`) outside the loop; resolution is not memoized at the call site.

### Confusing Given Priority With Definition Order

Believing that a `given` defined "later" in a file wins, when actual priority is governed by companion-object vs import scope vs specificity rules. Resolution is not textual. Place instances by the priority rules (companion first), not by intuition about source order.

## Self-Check

- [ ] The decision to use a type class was made against a real need for ad-hoc polymorphism (open type set, types you do not own, per-type behavior); problems expressible as subtyping or plain parameters use those instead.
- [ ] Every instance lives in the type class companion or the type companion, or — if neither is possible — in a documented package object; no orphan instances that would cause ambiguity when combined with another library.
- [ ] There is exactly one canonical instance per type per type class; where two behaviors are needed they are distinct named types or values, not competing instances selected by import.
- [ ] No implicit conversions are used for type-changing semantics; extension methods cover method-addition needs, and any genuine conversion is explicit at the call site.
- [ ] Context bounds are used only for genuine type-class constraints; services and dependencies are passed as explicit `using`/parameters, not hidden behind context-bound syntax.
- [ ] Derived instances (`derives`, Magnolia) were reviewed for types whose equality/show/serialization semantics differ from the structural default, and overridden by hand where needed.
- [ ] Recursive types that would cause derivation to diverge have hand-written instances with explicit recursion, and the build does not hang or stack-overflow on instance resolution.
- [ ] Each `given` is named, and a `summon[TypeClass[A]]` at a representative use site confirmed the compiler selects the intended instance before assuming the logic is correct.
- [ ] A reader tracing a type-class call can determine the active instance from companion placement and explicit imports alone, without running mental resolution over scattered source files.
