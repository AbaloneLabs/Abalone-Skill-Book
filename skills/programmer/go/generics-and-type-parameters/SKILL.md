---
name: generics_and_type_parameters.md
description: Use when the agent is writing or reviewing Go generics (type parameters, generic functions, structs, methods, and interfaces), designing generic data structures or algorithms, choosing between type parameters and interfaces/any, applying type constraints and the constraints package, instantiating generics with type inference, or diagnosing over-parameterization, slow generic compilation, constraint errors, methods that cannot have extra type parameters, or code that is clearer without generics. Covers when to parameterize, constraint design, the cost of unnecessary generics, and the boundary between generics and runtime polymorphism.
---

# Generics And Type Parameters

Go added type parameters (generics) in 1.18, and the temptation they create is the whole problem. Before generics, the only ways to write reusable collection or algorithm code were `interface{}`/`any` with type assertions (which lose compile-time type safety and add boxing) or code generation (which duplicates source). Generics fix both, and that fix is so welcome that agents reach for type parameters everywhere: a function that takes one concrete type gets a `[T any]` "for flexibility," a struct with one field gets parameterized, a tiny helper gets a constraint. Each of these compiles, each looks modern, and each makes the code harder to read, harder to instantiate, and sometimes slower to compile — for a flexibility that no caller ever uses. A type parameter is a contract with every caller; it should be added only when at least two distinct types actually need the same logic.

The judgment problem is not "how do I write `func Map[T, U any]`" but three questions: does this logic genuinely vary over more than one type, would an interface or a concrete type express the intent more clearly, and does the constraint I write capture a real requirement or just `any` noise. Agents trained on languages with reified, rich generics (C#, Java, Rust) tend to over-parameterize Go code and to expect features Go deliberately omits (no specialization, no methods with extra type parameters, no variadic type parameters). Agents new to generics tend to write constraints that are too loose (`any`) or too tight (a custom constraint that mirrors one type's method set). Both produce code that compiles and is worse than the non-generic version it replaced.

## Core Rules

### Parameterize Only When At Least Two Types Need The Same Logic

The single most important decision is whether to use a type parameter at all. A type parameter is justified when the same logic is applied to two or more distinct types and you want compile-time type safety without code duplication — a `Stack[T]`, a `Filter[T]`, a `Map[T, U]`. It is not justified when there is only one type (parameterizing a `UserRepository` as `Repository[T]` when `T` is always `User` adds indirection and removes readability), when an interface would do (a function that only calls `String()` on its argument needs `fmt.Stringer`, not `[T Stringer]`), or when the logic is trivial (a one-line helper parameterized for reuse that has one caller).

The test is concrete: list the distinct types that will instantiate the generic. If the list has one entry, do not parameterize. If the list has two or more entries that share uniform logic, parameterize. If the entries need different logic, you do not have a generic — you have an interface with multiple implementations, or separate functions. Parameterizing "for the future" is premature abstraction; Go's philosophy is to write the concrete thing now and generalize when the second case appears, because the second case often never appears and the abstraction is then dead weight.

### Choose Between Type Parameters And Interfaces Deliberately

Generics and interfaces solve overlapping problems, and the choice is a design decision with real tradeoffs. A type parameter preserves the concrete type at compile time: `func First[T any](s []T) T` returns the actual `T`, the caller uses it with no cast, and there is no boxing. An interface erases the concrete type: `func First(s []any) any` returns `any`, the caller must type-assert, and value types box. For uniform logic over a known set of types where you want to keep the type, generics win. For runtime polymorphism over an open set of implementations (plugins, serialization, a handler registry), interfaces win.

The decision rules: if the function needs to return or store the same type it received, use a type parameter (interfaces cannot express "the output type equals the input type"). If the set of acceptable types is open and discovered at runtime, use an interface. If the logic only calls methods on the value and never relies on its concrete identity, an interface is usually clearer than a parameterized type. If you need a heterogeneous container (`map[string]any` for arbitrary JSON), `any` is correct and generics cannot help. Do not use generics to fake what interfaces do, and do not use interfaces where the type-preserving property of generics matters — each misuse produces code that is either unsafe (interface + assertions) or over-abstract (generics over one type).

### Design Constraints To Capture Real Requirements, Defaulting To Composable Stdlib Constraints

A constraint is the contract a type parameter must satisfy. The weakest constraint is `any` (no requirement); the strongest is a method set or union that exactly matches what the logic calls. The discipline is to constrain to exactly what the logic needs: if `Map` only reads elements and produces outputs, `[T any]` is right; if `Sum` needs numeric addition, use a constraint that permits `+` over numeric types. Go's `constraints` package (and `cmp.Ordered`) provides composable building blocks (`Ordered`, `Signed`, `Unsigned`, `Float`, `Integer`, `Number`) — prefer these and unions of them over hand-rolled constraints, because they are standard and readable.

Constraints can be interface types with methods, unions of types (`interface{ ~int | ~int64 | ~float64 }`), or approximations (`~int` matches any type whose underlying type is `int`, including named types like `type MyInt int`). Use the tilde (`~`) when you want named types based on the underlying type to satisfy the constraint — without it, `int | int64` rejects `MyInt`. Avoid constraints that mirror a single implementation's method set (a constraint listing exactly the methods of one struct is a smell — it means the generic has one real instantiation). Do not over-constrain: a constraint stricter than the logic requires excludes types that would work, for no benefit. Constraint design is interface design under another name; the same "small, purposeful, composable" rules apply.

### Rely On Type Inference, And Write APIs So Inference Works

Go has type inference: at most call sites you write `Map(xs, f)` and the compiler infers `T` and `U` from the arguments, rather than forcing `Map[int, string](xs, f)`. Good generic API design makes inference work at every call site, because explicit type arguments are noise that hurts readability. The rule is to order type parameters so they can be inferred from function arguments: put the inferred parameters first, and only require explicit instantiation when the type cannot be derived from arguments (e.g., a function taking no arguments that returns `T`, like `func New[T any]() T`, forces `New[MyType]()`).

When inference fails, ask whether the API shape is the problem before forcing explicit arguments everywhere. A function whose type parameter does not appear in any parameter cannot be inferred and is often a sign the parameter is unnecessary. Two type parameters where one could be derived from the other (via a constraint relationship) sometimes let you drop one. Test every generic API by writing calling code: if callers routinely write explicit type arguments, the API is fighting inference and should be reshaped.

### Accept That Methods Cannot Have Additional Type Parameters

Go deliberately restricts generic methods: a method on a generic type can use the type's own type parameters, but it cannot declare additional type parameters of its own. `type Set[T comparable] struct{...}` can have `func (s Set[T]) Add(v T)`, but it cannot have `func (s Set[T]) Map[U any](f func(T) U) Set[U]` — that is a compile error. This is a deliberate design choice (to keep method resolution simple and to avoid a class of hard-to-resolve generics), and it shapes how you design generic types.

The workaround is a top-level function instead of a method: `func Map[T, U any](s Set[T], f func(T) U) Set[U]`. This is the idiomatic Go shape for operations that would be generic methods in other languages (`Map`, `Filter`, `Reduce` over a generic collection are free functions, not methods). Do not fight this by embedding type information to fake method-level parameters; design the API as functions that take the collection as the first argument. Agents coming from languages with generic methods (C#, Java, Scala) find this restrictive, but the free-function shape is idiomatic and reads well once accepted.

### Do Not Expect Specialization, Variadic Parameters, Or Reified Types

Go's generics are deliberately limited compared to richer systems, and expecting features that are absent leads to frustration and convoluted code. There is no specialization: you cannot write a special version of a generic function for `T = int` that the compiler picks. There are no variadic type parameters: you cannot write `func Tuple[T ...any]` to handle arbitrary arities (use code generation or a fixed set of arities). Types are not reified: `T` does not exist at runtime, so you cannot `switch` on `T` or get `reflect.TypeOf` of a type parameter in a useful generic way (you can pass a type via the value, but the parameter itself is erased). There is no operator overloading, so a constraint that wants `+` must restrict to types where `+` is built in (numbers, strings) via a union.

Design within these limits. If you need type-specific behavior, use an interface with a method (the type provides its own specialization) or a type switch on a concrete value, not generic specialization. If you need variadic arity, generate code or accept that you write `Map2`, `Map3`. If you need runtime type information, pass it explicitly (a `func() T` factory, or operate on `reflect.Type` outside the generic). Fighting the limits produces code that compiles by accident and is unreadable; accepting them produces clean, idiomatic Go.

### Watch The Compile-Time Cost And Readability Of Heavy Generics

Generics are not free for the compiler or the reader. Complex generic code with many type parameters, deep constraint unions, and nested instantiations can slow compilation noticeably and produce verbose error messages that are hard to act on. A generic utility used in one place is often slower to compile and harder to read than the concrete equivalent. This is not an argument against generics — it is an argument for using them where the reuse is real and the readability gain is clear, and for keeping generic APIs small and focused.

Readability suffers when type parameters pile up. A signature like `func Transform[T, U, V any, K comparable](m map[K]T, f func(T, U) V) map[K]V` is correct and hard to scan. Prefer to factor so each generic function has the fewest type parameters that express the logic, and name them meaningfully (`T` for element, `K` for key, `V` for value — single letters are conventional and fine for one or two, but a fourth or fifth parameter earns a longer name or a refactor). If a generic function is hard to read, that is a signal it may be doing too much; consider splitting it or replacing it with concrete code.

## Common Traps

### Parameterizing A Single-Use Type Or Function

`type UserRepository[T any] struct{...}` instantiated only ever as `UserRepository[User]`. The parameter adds noise and removes readability for zero reuse. Write the concrete type; generalize when a second type appears.

### Using `any` Everywhere A Constraint Should Be Specific

A `[T any]` constraint on a function that calls `T` methods fails to compile, leading to a hand-rolled `any` plus type assertion that defeats generics. Constrain to the method set or union the logic actually needs.

### Over-Constraining To One Type's Method Set

A custom constraint that lists exactly the methods of the single struct that satisfies it. The generic has one instantiation; an interface or the concrete type would be clearer.

### Forgetting The Tilde (`~`) For Named Types

`interface{ int | int64 }` rejects `type MyInt int`, surprising callers whose named numeric types should work. Use `~int | ~int64` when named types based on the underlying type should qualify.

### Expecting Generic Methods With Extra Type Parameters

`func (s Set[T]) Map[U any](...)` does not compile. Use a free function `func Map[T, U any](s Set[T], ...) Set[U]` instead.

### Using Generics Where An Interface Is Clearer

A function that only calls `String()` on its argument, parameterized as `[T Stringer]` instead of taking `fmt.Stringer`. If the logic only uses methods and never relies on the concrete type identity, an interface is clearer and more idiomatic.

### Forcing Explicit Type Arguments By API Shape

A generic API where every caller must write `Foo[int, string](...)` because the parameters cannot be inferred. Reshape the API so inference works; test by writing calling code.

### Assuming Generics Add Runtime Cost Like Interfaces

The opposite trap: avoiding generics because "they box like `any`." Go generics are compile-time and monomorphized/dictionaries-based; they do not box value types the way `any` does. Use them where type preservation matters.

### Fighting Missing Features (Specialization, Variadics, Reification)

Convoluted code trying to specialize per type or handle variadic arity inside one generic. Use interfaces, type switches, or code generation for what generics deliberately do not provide.

## Self-Check

- [ ] Every type parameter is justified by at least two distinct types that need the same logic; no type or function is parameterized for a single instantiation or "for future flexibility."
- [ ] The choice between generics and interfaces is deliberate: type parameters are used where the type-preserving property matters (output type relates to input type) or where uniform logic avoids `any`+assertions; interfaces are used for runtime polymorphism and open implementation sets.
- [ ] Constraints capture exactly what the logic requires, defaulting to composable stdlib constraints (`cmp.Ordered`, the `constraints` package) and unions; the tilde (`~`) is used where named types based on an underlying type should qualify; no constraint mirrors a single implementation's method set.
- [ ] Generic APIs are shaped so type inference works at call sites (inferred parameters ordered to be derivable from arguments), and calling code does not routinely require explicit type arguments.
- [ ] Operations that would be generic methods are written as free functions (no methods with additional type parameters), and the API has been designed around this Go restriction rather than fighting it.
- [ ] The design accepts Go's deliberate limits (no specialization, no variadic type parameters, no reified types, no operator overloading) and does not use convoluted code to fake them; type-specific behavior uses interfaces or type switches on concrete values.
- [ ] Generic functions have the fewest type parameters that express the logic, are readable, and do not noticeably harm compile time; a hard-to-read generic signature has been considered for splitting or replacing with concrete code.
- [ ] The generic API was tested by writing calling code from a second consumer's perspective, confirming it is actually reusable and not over-fit to its first caller.
