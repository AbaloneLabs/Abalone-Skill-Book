---
name: python_type_hinting_and_mypy.md
description: Use when the agent is adding or reviewing Python type hints, configuring mypy/pyright strictness, choosing between Union/Optional/Any/Protocol, designing generics, narrowing types with isinstance/guards/overloads, deciding where runtime validation (pydantic, typeguard, beartype) is needed versus static checking, or migrating an untyped codebase to typed. Covers gradual typing strategy, strictness level selection, stub files, TypedDict, Literal, and the boundary between compile-time and runtime type guarantees.
---

# Type Hinting And Mypy

Python type hints are a gradual, opt-in contract, not a guarantee. The interpreter ignores annotations at runtime, so a program can be fully annotated and still crash on a `None` attribute access or a wrong-shape dict. The judgment problem is deciding where static types should carry the safety load, where runtime validation must enforce it, and how strict to be without making the type system fight the code.

Agents often treat typing as decoration: add `dict` everywhere, sprinkle `Optional`, silence mypy with `# type: ignore` or `Any`, and declare victory. The harm is delayed. Loose types give false confidence (mypy passes but proves nothing), while overly strict or clever types make refactoring painful and push developers to bypass the checker. The real work is choosing annotation precision per boundary, picking a strictness level that matches the codebase's maturity, and keeping the static/runtime split honest.

## Core Rules

### Match Annotation Precision To The Boundary

Not every annotation needs the same precision. Calibrate by how far the value travels and how costly a mismatch is.

- **Internal helpers** can use coarse types (`list`, `dict`, `int`) when the scope is small and readable.
- **Public APIs, library entry points, and cross-module contracts** deserve precise types: `list[User]` not `list`, `dict[str, Order]` not `dict`, `Literal["asc", "desc"]` not `str`.
- **Data crossing trust boundaries** (HTTP, files, config, untrusted JSON) must be validated at runtime; static types alone cannot protect them.

Ask: if this value were wrong, where would the error surface? The closer to the source the type stops the error, the more precision it deserves.

### Choose Union, Optional, And Any Deliberately

These three are not interchangeable, and overusing `Any` is the most common type-erosion mistake.

- **`X | Y` (Union)**: the value is genuinely one of several shapes. Model it as a union and narrow at use sites. Prefer discriminated unions (with a `Literal` tag or `isinstance` guard) so narrowing is mechanical.
- **`X | None` (Optional)**: absence is a valid, intentional state. Do not use `Optional` to mean "I'm not sure" — that is `Any` in disguise and forces every caller to handle `None`.
- **`Any`**: an explicit escape hatch. Every `Any` disables checking for that position, so treat it as a debt to pay down. Prefer `object` (top type that still forbids attribute access) when you only need "some value."

Strong choice: `def find(key: str) -> User | None`. Weak choice: `def find(key: str) -> Any` "to keep it flexible."

### Use Protocol For Structural Contracts, ABCs For Nominal Ones

`typing.Protocol` lets you describe "anything with these methods" without forcing callers to inherit. This is usually the right tool for dependency injection, callbacks, and plugin points, because it avoids coupling consumers to a concrete class hierarchy.

Use abstract base classes (`abc.ABC`) when you genuinely want a nominal family: shared implementation, `isinstance` semantics that matter, or a controlled inheritance tree. Do not reach for ABCs merely to define an interface that Protocol expresses more loosely.

### Design Generics Around Variance And Caller Inference

Generics (`TypeVar`, `Generic`) preserve type relationships across a function or class. Use them when a parameter and return type are linked (`def first(xs: list[T]) -> T`) or when a container is parameterized (`class Repository(Generic[T])`).

Decisions that matter:

- Let the checker infer type arguments at call sites; do not force callers to annotate them unless inference is ambiguous.
- Pick variance intentionally. Containers are usually invariant; use `TypeVar("T", covariant=True)` only when the type is read-only (e.g., frozen sequences), and understand that covariance on a mutable container is unsound.
- Bound or constrain `TypeVar`s (`TypeVar("T", bound=Comparable)`, `TypeVar("T", str, bytes)`) only when the operations inside genuinely require it. Unbounded `TypeVar` is fine when you only move values around.

### Prefer Narrowing Over Casts And Any

When you know more than the type checker, narrow the type rather than assert it away.

- `isinstance(x, int)` narrows in `if`/`elif` branches and is the safest guard.
- `TypeGuard` / `TypeIs` (PEP 647 / 742) let user-defined predicates narrow; use them for structural checks the built-ins cannot express.
- `@overload` lets you express call-signature-dependent return types (e.g., `get(key, default)` returns `V` when a default is given, `V | None` otherwise). Prefer overloads over a single loose signature.
- Reserve `cast(T, x)` for places where external knowledge (a C extension, an untyped library, a deserializer) guarantees a type the checker cannot see. `cast` is unchecked, so it can hide real bugs.

### Pick A Strictness Level That Fits The Codebase

Mypy and pyright offer graduated strictness. The right level depends on maturity, not ambition.

- **New or greenfield code**: start strict (`--strict` or pyright `recommended`/`strict`). Fixing types early is cheap.
- **Existing untyped codebase**: start lenient (`--ignore-missing-imports`, no `disallow_untyped_defs` yet), then ratchet up module by module. Use `# mypy: strict` per-file opt-ins for new modules.
- **Library vs application**: libraries should be stricter and ship a `py.typed` marker so downstream users get checking; applications can afford looser internal modules.

Document the chosen baseline in `pyproject.toml` or `mypy.ini` so the decision is shared, not per-developer.

### Keep Static And Runtime Checking In Their Lanes

Static checking proves properties for all executions the checker can see; runtime checking proves a property for the value in hand. They are complementary, and confusing them produces both false safety and wasted effort.

- Static types cannot validate data from `json.load`, `yaml.safe_load`, `os.environ`, HTTP bodies, or `**kwargs` from untyped callers. Validate those with `pydantic`, `attrs`/`cattrs`, `msgspec`, `dataclass` + manual checks, or `beartype`/`typeguard`.
- Runtime checkers (`beartype`, `typeguard`) check annotations at call time. They are good for catching contract violations in tests or dev, but they do not replace static analysis and they add overhead — do not scatter them across hot paths without measuring.
- Do not annotate a function as accepting `dict[str, int]` and then trust untrusted JSON to match; the static type describes the post-validation world, and validation must happen first.

## Common Traps

### Using `dict` And `list` Without Parameters

Bare `dict` or `list` mean `dict[Any, Any]` / `list[Any]`, which disables checking on the contents. This is the silent type-erosion pattern: mypy reports no error because nothing is checked. Always parameterize containers, or use `dict[str, Any]` explicitly when heterogeneity is real and acknowledged.

### `Optional` As A Default For Uncertainty

Marking everything `Optional` "to be safe" forces every caller to handle `None`, pollutes logic with defensive checks, and hides the genuine optional cases. Reserve `Optional` for values that are legitimately absent; if you are unsure of the type, say `object` or fix the type.

### `# type: ignore` And `cast` As First Responses

Both silence the checker without proving anything. `# type: ignore[code]` at least documents which error is suppressed; bare `# type: ignore` hides future regressions. Before suppressing, ask whether a `Protocol`, an `@overload`, a `TypeGuard`, or a runtime check would express the real invariant. Reserve suppression for genuinely unsound third-party stubs.

### Treating TypedDict Like A Class

`TypedDict` describes a dict's shape statically but offers no runtime enforcement — `json.load` still returns `dict[str, Any]`. A `TypedDict` is a documentation and checking tool, not a validator. If the dict crosses a trust boundary, validate into a `dataclass`/`pydantic` model first.

### Trusting Third-Party Stubs Blindly

`--ignore-missing-imports` turns an untyped dependency into `Any`, silently disabling checking across a large surface. Prefer installing real stubs (`types-requests`, `boto3-stubs`) or shipping your own `*.pyi` for critical dependencies. Audit which imports resolve to `Any`.

### Over-Engineering Types

Conditional types, deeply nested mapped types, and clever recursive `TypeVar` constructions can be correct but unreadable. If a type expression needs a paragraph to explain, a runtime check or a simpler model is usually clearer. Optimize for the next reader, not for maximal precision.

### Assuming Annotations Run At Runtime

Annotations are evaluated (and by default stored) at definition time, but they do not check anything. `x: int = "hello"` runs fine. Do not write code that depends on annotations enforcing contracts; that requires an explicit runtime checker.

## Self-Check

- [ ] Public APIs and cross-module contracts use precise, parameterized types (`list[User]`, `dict[str, int]`, `Literal[...]`), not bare `dict`/`list`.
- [ ] `Any` appears only where a real escape hatch is justified, and `Optional` is used for genuine absence, not uncertainty.
- [ ] Structural contracts use `Protocol`; nominal families use `abc.ABC` — the choice is intentional, not habitual.
- [ ] Generics preserve caller relationships, variance is chosen deliberately, and type arguments are inferred rather than redundantly annotated.
- [ ] Type narrowing uses `isinstance`, `TypeGuard`/`TypeIs`, or `@overload`; `cast` and `# type: ignore[code]` are rare and justified.
- [ ] The chosen mypy/pyright strictness baseline is documented and matches the codebase maturity; new code is stricter than legacy.
- [ ] Data from JSON, env vars, HTTP, files, or untyped callers is runtime-validated before being treated as its static type.
- [ ] No bare `# type: ignore` remains; every suppression names a code and a reason, or is replaced by a real narrowing.
- [ ] `TypedDict` is not relied upon for runtime validation of untrusted input.
- [ ] The type annotations are readable to the next maintainer; no type expression requires a paragraph of explanation.
