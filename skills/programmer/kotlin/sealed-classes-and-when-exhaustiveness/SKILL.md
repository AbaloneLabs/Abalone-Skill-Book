---
name: kotlin_sealed_classes_and_when_exhaustiveness.md
description: Use when the agent is designing Kotlin sealed classes and interfaces, sealed hierarchies for domain modeling and state (UiState Loading/Success/Error, Result types, finite state machines), using when expressions with exhaustiveness checking, object singletons vs data classes in sealed hierarchies, sealed across multiple files and modules (Kotlin 1.5+), or is diagnosing "when must be exhaustive" errors, missing else branches that hide bugs, state transitions that are not exhaustive, or sealed subclasses added without updating all when sites. Covers sealed type design, when exhaustiveness, closed vs open hierarchies, state machine modeling, and the pitfalls of else branches that defeat the compiler's checks.
---

# Sealed Classes And when Exhaustiveness In Kotlin

Kotlin's sealed types and `when` exhaustiveness checking together form one of the language's most powerful modeling tools, and also one of the most commonly misused. A `sealed class` (or `sealed interface`) defines a closed hierarchy: all direct subtypes are known at compile time, within the same module (or, since Kotlin 1.5, the same package across files). This closedness lets the compiler verify that a `when` over the hierarchy covers every subtype, turning "did I handle every case" from a runtime risk into a compile error. When a new subtype is added, every exhaustive `when` fails to compile until updated — a refactoring safety net that plain interfaces and `if`/`else` chains do not provide. The judgment problem is to use sealed types to model genuinely closed sets (finite states, fixed alternatives, domain sum types), to write `when` as an exhaustive expression (not a statement with an `else` that swallows new cases), and to avoid the patterns that defeat the compiler's check — the broad `else`, the open hierarchy, the non-sealed interface used where a sealed one was meant.

Agents reach for `when` with an `else` branch out of habit, which compiles for non-sealed receivers but defeats exhaustiveness: a new subtype silently falls into `else`, producing a bug the compiler would have caught. Or they model a state machine with an `enum` plus ad-hoc fields when a sealed hierarchy of state objects would be clearer, or they leave a hierarchy open (`interface`/`abstract class`) when the set of alternatives is actually closed. The remedy is to seal hierarchies that are closed, to write `when` as a value-returning expression without `else` where possible, and to let the compiler enforce completeness.

## Core Rules

### Use Sealed Types For Closed Sets Of Alternatives

A sealed hierarchy models a finite, known set of alternatives: a UI state (`Loading`, `Success`, `Error`), a result (`Success`, `Failure`), an event, a domain sum type (`PaymentMethod.Card`, `.Cash`, `.Voucher`). The defining property is that the set of subtypes is closed — all of them are known at compile time, and new ones cannot be added from outside. Choose sealed when the set is genuinely closed and you want exhaustive handling; choose an open `interface`/`abstract class` when extension by consumers is intended. Modeling a closed set with an open hierarchy loses exhaustiveness and lets an unexpected subtype reach a `when` unhandled.

- `sealed interface UiState { data object Loading : UiState; data class Success(val data: T) : UiState; data class Error(val message: String) : UiState }`.
- Subtypes as `data class` when they carry data (and you want equality/copy), as `object`/`data object` when they are a singleton marker (`Loading`).
- A sealed hierarchy can span files in the same package (Kotlin 1.5+); across modules it must be in the same module/compilation unit for the closed-set guarantee.

### Write when As An Exhaustive Expression Without else

A `when` used as an expression (assigned to a value, returned) over a sealed receiver is checked for exhaustiveness: the compiler requires every subtype covered, with no `else`. This is the safety net. When you add a new subtype, the `when` fails to compile until you handle it. To get this check, write `when` as a value-returning expression and omit `else`. A `when` used as a statement (no result, just side effects) is not checked for exhaustiveness by default — prefer the expression form, or enable the explicit warnings, so the check applies.

- `val text: String = when (state) { is Loading -> "Loading"; is Success -> state.data.toString(); is Error -> state.message }` — exhaustive, no `else`.
- If you must use a statement `when`, add an `else` that throws (`else -> throw IllegalStateException("Unhandled $state")`) so an unhandled case fails loudly rather than silently.
- Avoid `else` in an expression `when` over a sealed type: it makes the check pass for any future subtype, defeating the point of sealing.

### Model State Machines With Sealed Types And Explicit Transitions

A finite state machine (UI state, order lifecycle, auth flow) is a closed set of states plus a closed set of transitions. Model the states as a sealed hierarchy, and express transitions as functions that take the current state and an event and return the next state, with a `when` that is exhaustive over both. The compiler then verifies every state/event combination is handled (or deliberately marked unreachable). This is far safer than a mutable `var state: String` and ad-hoc `if` branches, where an illegal transition is a runtime bug.

- States: `sealed interface OrderState { object Pending; object Paid; object Shipped; object Cancelled }`.
- Transitions: `fun OrderState.on(event: OrderEvent): OrderState = when (this) { ... } when (event) { ... }`, exhaustive over both.
- Illegal transitions throw or return an error state deliberately, never silently no-op.

### Choose data class vs object For Subtypes By Whether They Carry Data

Each subtype is either a `data class` (carries data, gets equality/copy/componentN) or an `object`/`data object` (a singleton marker, no data). `Loading` is an `object`; `Success(data)` is a `data class`. Mixing these deliberately keeps the hierarchy readable and gives the right semantics. A `data class` with no fields is usually better as an `object` (single instance, no allocation); a subtype with many fields is a `data class` for the generated equality and copy. Use `value class` for a single-field wrapper where appropriate.

### Prefer Sealed interface Over Sealed class For Flexibility

A `sealed interface` allows subtypes that are also `data class`, `enum class`, `object`, or even other `class` hierarchies, and a type can implement multiple sealed interfaces. A `sealed class` requires subtypes to inherit from it, restricting to a single-inheritance line. Prefer `sealed interface` for new hierarchies (more flexible composition); use `sealed class` when shared behavior or state in the base is needed.

### Keep when Branches Meaningful, Not Auto-Generated Boilerplate

An exhaustive `when` with one meaningful branch per subtype is the goal; a `when` with five branches that all do nearly the same thing suggests the hierarchy is over-differentiated or the handling belongs in the subtypes (polymorphism via methods on each subtype). Push behavior onto the subtypes where it varies per type and is intrinsic; keep the `when` for cases where an external operation must dispatch on the type (rendering, mapping, state transition). Do not let `when` become a giant switch that duplicates logic across call sites.

## Common Traps

### else Branch Defeating Exhaustiveness

`when (state) { is Success -> ...; else -> default }` compiles for any future subtype and silently routes new cases to `default`. Omit `else` in expression `when` over sealed types.

### Statement when Not Checked For Exhaustiveness

A `when` used only for side effects is not exhaustiveness-checked. Use it as an expression, or add a throwing `else`.

### Open Hierarchy Where A Closed One Was Meant

An `interface` or `abstract class` (non-sealed) used for a closed set loses the compiler check; an unexpected implementation reaches `when` unhandled. Seal it.

### Sealed Across Modules Losing The Guarantee

A sealed type's subtypes must be in the same module/compilation unit; subtypes in a downstream module are not part of the closed set, and exhaustiveness is not enforced for them. Keep the hierarchy within one module.

### Object vs data class Misuse

A `data class` with no fields allocated per use; an `object` is a single instance. Use `object`/`data object` for marker states.

### New Subtype Silently Handled By a Broad else

Adding `Cancelled` to an order state with an `else -> "unknown"` branch hides the new case. Remove `else` so the compiler flags it.

### Exhaustive when Duplicating Logic Across Call Sites

Five `when` blocks each dispatching on the same sealed type with similar logic suggests pushing the behavior onto the subtypes.

## Self-Check

- [ ] Closed sets of alternatives are modeled as sealed hierarchies (`sealed interface`/`sealed class`), and open hierarchies are used only where consumer extension is intended.
- [ ] `when` over sealed types is written as a value-returning expression without `else`, so adding a subtype fails to compile until handled; statement `when` uses a throwing `else`.
- [ ] State machines are modeled with sealed states and exhaustive transition functions over both state and event, with illegal transitions failing deliberately.
- [ ] Subtypes are `data class` (data-carrying, equality/copy) or `object`/`data object` (singleton marker) as appropriate, and value classes wrap single fields where fitting.
- [ ] `sealed interface` is preferred for new hierarchies for composition flexibility; `sealed class` where shared base behavior is needed.
- [ ] The sealed hierarchy is within one module/compilation unit so the closed-set guarantee and exhaustiveness hold; no subtype is added from a downstream module expecting coverage.
- [ ] No broad `else` in an expression `when` over a sealed type, and no `when` is a giant duplicated switch that should be polymorphic behavior on the subtypes.
- [ ] The design has been considered under adding a new subtype (compiler flags every site), illegal transitions, and cross-module use, and remains exhaustive and safe.
