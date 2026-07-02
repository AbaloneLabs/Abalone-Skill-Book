---
name: php_type_system_and_modern_php.md
description: Use when the agent is writing or reviewing PHP code involving declared parameter, property, and return types, nullable and union types, strict_types declaration, migrating untyped legacy PHP to typed modern PHP, handling mixed and dynamic typing traps, designing with readonly properties and promoted constructor arguments, or deciding where static type checking and runtime types agree and where they diverge in a PHP codebase.
---

# The Type System And Modern PHP

Modern PHP (7.0 onward, accelerating through 8.x) has a real type system: declared parameter, property, and return types; nullable and union types; `readonly` properties; `strict_types`; and a growing static-analysis ecosystem (PHPStan, Psalm). Used well, this catches bugs at the boundary, documents contracts, and lets static analyzers prove properties across a codebase. Used badly — loose types, `mixed` everywhere, missing `strict_types`, types that lie about what the code actually returns — it gives false confidence: the code "has types" but the types do not constrain anything meaningful, and the dynamic-typing traps that PHP is famous for (silent coercions, null in unexpected places, arrays that are sometimes lists and sometimes maps) remain fully active.

Agents often add types as decoration: `string` everywhere, `?array` for "some data," `mixed` to silence the analyzer, no `declare(strict_types=1)`. The harm is that PHP without `strict_types` coerces arguments under weak typing (`"5" + 5` is 10, `null` becomes `""` in some contexts), so a declared `int` parameter accepts `"5"` silently and the type annotation documents an intent the runtime does not enforce. The judgment problem is to declare types that match the real contract, to enable `strict_types` so the declarations are enforced, to choose nullable and union types deliberately, to know where static analysis and runtime diverge, and to migrate legacy untyped code without pretending that sprinkling `string` over it makes it typed.

## Core Rules

### Declare Types Everywhere, And Make Them Precise

Every function parameter, return type, and property should have a declared type. Vague types (`mixed`, `array`, `object`) are better than nothing but should be narrowed as far as the contract allows.

- Parameter types document what callers must provide and let the runtime reject wrong types at the call boundary. Return types document what callers can rely on and prevent drift.
- Property types (and `readonly`) document object shape and let the runtime reject invalid state. Typed properties are a major correctness win over untyped `public $foo`.
- Narrow `array` to a specific shape where possible: PHP cannot express `list<int>` at runtime (arrays are unified maps/lists), but static analyzers can, so use `@param list<int>` or `@return array<string, User>` annotations to give PHPStan/Psalm the precise shape. The runtime type is `array`; the analysis type is the shape.
- Avoid `mixed` except at genuine boundaries (a deserializer, a generic container, an interop shim). Every `mixed` is a place where the analyzer gives up; prefer `object`, a specific class, or a union of known types.

### Enable strict_types So Declarations Are Enforced

By default, PHP uses weak (coercive) typing for scalar type declarations: a function declaring `int $x` accepts `"5"` and coerces it. `declare(strict_types=1)` at the top of a file enables strict typing within that file, so a `int` parameter rejects `"5"` and requires an actual int.

- The strict_types declaration is per-file and affects only the file that declares it (the call site), not the file defining the function. This is subtle: strictness is determined by the file that makes the call, so a library author cannot force strictness on callers.
- For application code, enable `strict_types=1)` in every file as a matter of policy; it makes the declared types meaningful rather than advisory. Many frameworks and style guides (PSR-12 extension, some project rulesets) require it.
- Be aware that strict_types changes behavior: under strict mode, `intval("5")` still works (the function does the conversion), but `function f(int $x); f("5")` throws `TypeError`. The boundary checks become real.

The discipline: declared types without strict_types are documentation that the runtime may ignore; with strict_types they are enforced contracts.

### Choose Nullable And Union Types Deliberately

PHP's `?T` (nullable), `T|U` (union), and the special `mixed`/`null` types express "this can be more than one thing." Each has a cost: every caller must handle the alternatives, and the more alternatives, the more branches.

- **`?T`** means "T or null." Use when null is a legitimate, intentional value (an optional field, a lookup that may miss). Do not use `?T` to mean "I'm not sure"; that is `mixed` in disguise and forces every caller to null-check.
- **`T|U`** (union) means "one of these types." Use when a value is genuinely one of a few shapes (a response that is either a success object or an error object). Prefer narrowing at use sites (`instanceof` checks, which static analyzers understand) so each branch deals with one type.
- **`mixed`** means "anything." Reserve for genuine dynamic boundaries; treat every `mixed` as a debt to narrow when you understand the real shape.

A proliferation of nullable and union parameters usually signals a function doing too much; consider splitting it so each variant has a precise type.

### Use Readonly Properties And Promoted Constructor Args For Immutable Value Objects

PHP 8.1's `readonly` properties and 8.0's constructor property promotion make it concise to build immutable, typed value objects — the foundation of robust domain modeling.

- `readonly` properties can be written once (in the constructor) and never changed, enforcing immutability at the language level. Use for value objects, DTOs, and any type whose identity is its data.
- Constructor promotion (`public function __construct(private readonly string $name, private readonly int $age)`) declares and assigns in one line, reducing boilerplate.
- Immutability eliminates a class of bugs (accidental mutation of shared objects) and makes concurrency and caching safer, because an immutable object's state cannot change under you.

Prefer readonly value objects over mutable "structs" with public setters; the setter pattern invites partial mutation and invalid intermediate states.

### Know Where Static Analysis And Runtime Types Diverge

PHP's runtime types enforce parameter, return, and property types (with strict_types), but they cannot express generics, array shapes, or complex invariants. Static analyzers (PHPStan, Psalm) fill this gap, and their type annotations (`@param`, `@return`, `@var`, generics like `@return list<User>`) describe a richer type world than the runtime enforces.

- The runtime enforces declared types; the analyzer enforces declared types plus annotations. Code can pass the analyzer and fail at runtime (if annotations lie), or pass at runtime and fail the analyzer (if annotations are missing or wrong).
- Keep annotations honest: an `@return list<User>` that actually returns `array<string, User>` misleads the analyzer and every caller. Update annotations when you change the code.
- Run the analyzer at a meaningful level (PHPStan max level, Psalm with strict settings) in CI; a codebase where the analyzer passes at level 5 but fails at level 8 has hidden type debt.

### Migrate Legacy Untyped Code Incrementally And Honestly

Migrating a legacy PHP codebase to typed code is valuable but must be done without lying. Adding `string` to a parameter that actually receives `string|int|null` introduces a type that the runtime will enforce (and break on) or that you will weaken with coercion.

- Start by adding return types (less likely to break callers) and parameter types where the call sites are known and controlled. Add types at module boundaries first, where the contract is clearest.
- Use the analyzer to find the real types: PHPStan/Psalm infer the actual types flowing through untyped code, which tells you what type to declare. Declaring the inferred type is honest; declaring an aspirational type is a lie that breaks later.
- Do not enable strict_types on a file until its types are honest; strict mode will throw on the coercions the code currently relies on. Migrate the coercions first.
- Accept that some legacy code has genuinely dynamic behavior that resists typing; wrap it in a typed boundary (a repository, a service) and keep the legacy internals untyped until they can be refactored.

## Common Traps

### Declaring Types Without strict_types

`function add(int $a, int $b): int` called as `add("5", "5")` returns 10 under weak typing, silently coercing. The type declaration looks like a guarantee but is advisory. Enable `strict_types=1)` for the declaration to be enforced.

### mixed Everywhere To Silence The Analyzer

`function process(mixed $data): mixed` makes the analyzer happy and proves nothing. Every `mixed` disables checking for that position; narrow to the real type, or `object`, or a union, as soon as you understand the shape.

### Nullable As A Default For Uncertainty

`?string $name` "to be safe" forces every caller to handle null, pollutes logic with null-checks, and hides the genuine optional cases. Reserve `?T` for legitimate absence; if you are unsure, find out and use the real type.

### Array Types That Hide Shape

`array $items` tells the caller nothing — is it a list? a map? what are the elements? Use `list<T>` or `array<K, V>` annotations for the analyzer, and prefer a value object or a typed collection class when the shape is complex.

### Readonly Property Mutated Via Reference Or Reflection

`readonly` prevents direct assignment after construction, but code that mutates via `&` references or reflection can still change the value, breaking immutability expectations. Do not bypass readonly; if you need mutation, the type is not a value object.

### Annotations That Lie About The Code

`@return User` on a function that returns `User|null` misleads the analyzer and every caller, who will not null-check. Keep annotations synchronized with the code; treat annotation drift as a bug.

### Forcing Strict Types On Legacy Code That Relies On Coercion

Enabling `strict_types=1)` on a file that calls `add("5", "5")` throws `TypeError` in production. Migrate the coercions first, then enable strict mode. Do not flip the flag across a legacy codebase in one commit.

### Treating PHP Arrays As Either Lists Or Maps Without Deciding

PHP arrays are ordered maps that also serve as lists, sets, and tuples. Code that assumes a list (`for $i=0; $i<count; $i++`) breaks on a map with non-sequential keys; code that assumes a map breaks on a list passed to a function expecting key-value pairs. Decide which the data is and use `array_is_list()`/`array_values()` to enforce or normalize.

## Self-Check

- [ ] Every function parameter, return value, and property has a declared type, narrowed as far as the contract allows (specific classes, `list<T>` annotations, unions of known types) rather than `mixed` or bare `array`.
- [ ] `declare(strict_types=1)` is present in every application file, so declared types are enforced rather than coerced, and the file's coercions were migrated before the flag was enabled.
- [ ] Nullable (`?T`) is used only for legitimate absence, union types (`T|U`) for genuine multi-shape values, and `mixed` only at true dynamic boundaries, each with intent to narrow.
- [ ] Value objects and DTOs use `readonly` properties and constructor promotion for immutability, and no code bypasses readonly via references or reflection.
- [ ] Static analysis (PHPStan/Psalm) runs in CI at a meaningful strict level, annotations are honest and synchronized with the code, and analyzer-reported type debt is tracked.
- [ ] Legacy untyped code is migrated incrementally with honest types (analyzer-inferred where needed), strict_types enabled per-file only after coercions are removed, and dynamic internals wrapped in typed boundaries.
- [ ] Array shapes are explicit (`list<T>`, `array<K, V>` annotations or value objects), and code that assumes list semantics uses `array_is_list()`/`array_values()` to enforce or normalize.
- [ ] No annotation lies about the code (`@return User` when the function can return null); annotations are updated whenever the code changes.
- [ ] The runtime types and the analyzer types agree on the contract, and the places they diverge (generics, shapes) are covered by annotations the analyzer enforces.
- [ ] No `mixed`, bare `array`, or `?T`-for-uncertainty remains without a plan to narrow it to the real type.
