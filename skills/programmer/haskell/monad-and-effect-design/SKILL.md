---
name: monad_and_effect_design.md
description: Use when the agent is writing Haskell code with IO, do notation, monad transformers (ReaderT, StateT, ExceptT, stacks), effect systems (mtl, polysemy, effectful, fused-effects), or designing pure vs effectful boundaries; debugging space leaks from laziness, strictness issues, or unexpected thunk buildup; reasoning about referential transparency, side-effect ordering, or the cost of lazy evaluation; or deciding how to structure effectful programs without losing the benefits of purity. Covers laziness semantics, strictness annotations, transformer stack ordering, and the tradeoffs of pure functional effect design.
---

# Monad And Effect Design

Haskell's central design decision is that effects are explicit in the type: a function that reads a file, throws an error, or mutates state says so in its type (`IO`, `Either e`, `State s`), and a function whose type is pure can be relied on to do none of those things. This is the source of the language's strengths (refactoring is safe, testing is easy, concurrency is less scary) and also the source of its characteristic difficulty: effects are managed by monads, monads compose through transformers or effect systems, and laziness means the cost model is invisible from the source. The failure modes are all about the gap between what the type says and what the program does at runtime — a "pure" function that builds a giant thunk and OOMs, a transformer stack ordered so the wrong effect swallows the wrong error, an `IO` action that looks like a value but runs once and in a specific order, or an effect system whose inference sends the reader on a hunt through instance resolution.

Agents tend to treat Haskell monads as a syntax to learn rather than a design discipline, and to assume that "pure" and "lazy" are unqualified goods. The judgment problem is deciding, for each piece of logic, what effects it actually needs (and expressing only those in the type), how to compose effects without losing error-handling precision or stacking transformers into unreadability, and how to manage laziness so the program's memory behavior matches its apparent structure. Getting this wrong produces code that type-checks, passes property tests, and either leaks memory until it crashes or hides its real cost behind pure-looking signatures.

## Core Rules

### Express Only The Effects You Need, In The Weakest Monad That Captures Them

The power of effect typing comes from precision. A function that needs only to read a configuration should be in `Reader Config`, not `IO`; a function that can fail should be in `Either e`, not `Maybe` (if the error matters) or `IO` (if no I/O is needed). The weaker the monad, the more places the function can be called, the easier it is to test, and the more the type tells the reader about what the function cannot do.

Reach for `IO` only when the function genuinely performs I/O or non-determinism that cannot be expressed purely. Every function pushed into `IO` unnecessarily is a function that becomes harder to test, harder to compose, and harder to reason about — `IO` is the escape hatch, not the default. Strong choice: parsing is `Either ParseError AST`, validation is `Either [Error] a`, config access is `Reader Config`, and only the top-level `main` is `IO`. Weak choice: making everything `IO` because it is convenient, then losing the ability to test any of it without mocking the world.

### Order Transformer Stacks So Error Semantics Match Intent

A monad transformer stack like `ReaderT Config (ExceptT Error IO)` composes effects, and the order determines how those effects interact — most importantly, how errors propagate through state and how state survives errors. `StateT s (ExceptT e m)` discards state on error (the error propagates with the pre-error state lost); `ExceptT e (StateT s m)` preserves state on error (you can inspect the state at the point of failure). These are different programs, and choosing the wrong one produces subtle bugs where state is or is not available after a failure.

The general rule is that the outermost transformer governs the "outer" semantics, and errors propagate outward. Think about what should happen when each effect fails: should a `State` update survive an `Except`? Should a `Reader` environment be available during error recovery? Order the stack so the answer matches intent, and document the choice. A stack of five transformers ordered by accident is a common source of "why did my state reset" and "why did my error get swallowed" bugs. Prefer modern effect systems (mtl, effectful, fused-effects) when the stack grows beyond two or three, because they make the interaction semantics explicit rather than implicit in ordering.

### Treat Laziness As A Cost Model You Must Actively Manage

Haskell is lazily evaluated by default, which means expressions are not computed until their results are needed — and "needed" is a runtime property that depends on how the result is consumed. The benefit is composability (you can define `sum [1..]` and take the first ten elements). The cost is that laziness builds thunks (deferred computations) and thunks occupy memory until forced. A function that accumulates a large lazy structure (a fold that builds a giant thunk instead of a number, a map over a long list that defers every element) can OOM on inputs that look small in the source.

Manage strictness deliberately. Use `seq`, strictness annotations (`!` on fields and patterns), and `BangPatterns` to force evaluation where laziness would build unbounded thunks. Prefer `foldl'` (strict fold) over `foldl` (lazy fold) for numeric accumulation. Understand that `Data.Map` and `Data.List` are mostly lazy, while `Data.Vector` is mostly strict — choose the structure whose strictness matches the access pattern. The default reaction to a space leak is to add strictness; the disciplined reaction is to identify where the thunk is being built and force exactly there.

### Distinguish Values From Actions, Because IO Is Not A Value

In Haskell, `putStrLn "x"` is a value of type `IO ()` — a description of an action, not the execution of one. The action runs only when it is sequenced into `main` (directly or transitively) and executed by the runtime. A common beginner mistake (and a recurring source of "why didn't my log line run") is constructing an `IO` value and never sequencing it — the action is never performed, even though it appears in the source.

This extends to `let` bindings and `where` clauses: `let x = print "debug"` does not print; only sequencing `x` in an `IO` block (or binding it with `<-`) does. When debugging "my effect didn't happen," the first check is whether the action was actually sequenced into the effectful flow, not just defined. The flip side is the benefit: you can build, transform, and pass around actions as values, which is the basis of elegant effectful APIs.

### Keep Referential Transparency By Keeping Effects In The Type

Referential transparency — the property that an expression can be replaced by its value without changing the program — is the load-bearing benefit of Haskell, and it is preserved exactly when effects stay in their types. A function `Int -> Int` that secretly reads a global mutable variable (via `unsafePerformIO`, an `IORef` smuggled through a pure API, or `trace`) breaks referential transparency: the same input now produces different outputs depending on when it is called, refactoring becomes unsafe, and testing requires reproducing the hidden state.

Resist `unsafePerformIO` and debugging `trace` in committed code; they defeat the type system's effect tracking. If a function has effects, put them in the type, even if it is inconvenient. The discipline is what makes the rest of the codebase trustworthy. A single `unsafePerformIO` to "avoid threading IO through" can poison a whole module's reasoning.

### Choose Between Monad Transformers And Effect Systems By Stack Complexity

For one or two effects, a hand-rolled transformer stack is clear and direct. For three or more, or for effects that vary across many call sites, a dedicated effect system (mtl for typeclass-based effects, effectful or fused-effects for first-class effects, polysemy for higher-order effects) reduces boilerplate and makes the effect set per function explicit. The tradeoff is inference difficulty and compile times for larger effect systems, against the readability cost of deep transformer stacks.

Pick one effect system per project and use it consistently; mixing transformer stacks and multiple effect systems in one codebase fragments the effect vocabulary. The wrong choice is not "transformers vs effects" but "an ad-hoc stack of six transformers ordered differently in different modules."

### Use `do` Notation For Sequencing, But Understand What It Desugars To

`do` notation is syntactic sugar for `(>>=)` (bind) and is the readable way to sequence effectful computations. It is essential for clarity, but it hides what is actually happening: each `<-` is a bind, each line without `<-` is a sequence (`>>`), and the whole block desugars to nested lambdas. When debugging effect-interaction bugs (an error not propagating, a state update not surviving), expand the `do` block mentally into binds to see the real control flow.

Avoid `do` notation for trivial cases where a single bind or `fmap` would do — `x <- action; pure (f x)` is clearer as `f <$> action`. Overuse of `do` for single-line computations hides that the operation is a simple map and adds noise.

## Common Traps

### Pushing Everything Into IO For Convenience

Making functions `IO` because threading effects precisely is tedious, then losing testability and the type-level guarantee that pure code has no side effects. Express effects in the weakest monad that captures them; reserve `IO` for genuine I/O.

### Ordering A Transformer Stack So Errors Discard State You Need

Using `StateT s (ExceptT e m)` and finding that the state at the point of failure is lost, when the recovery logic needed it. Order the stack (`ExceptT e (StateT s m)`) so state survives errors when recovery needs it; the order is a semantic choice, not a stylistic one.

### Letting Laziness Build Unbounded Thunks

Using `foldl` for numeric accumulation and watching memory grow because each step defers, building a giant thunk that OOMs on large inputs. Use `foldl'` (strict) for numeric folds, and add strictness annotations where accumulation would otherwise defer.

### Defining An IO Action Without Sequencing It

Writing `let log = putStrLn "debug"` in a `do` block and never binding it with `<-` or sequencing it, so the action is never performed. `IO` values are descriptions; they run only when sequenced into the effectful flow.

### Smuggling Effects Through A Pure Signature With `unsafePerformIO` Or `trace`

Using `unsafePerformIO` to read a global or `trace` to debug, breaking referential transparency so the same input produces different outputs across runs. Keep effects in the type; remove debugging `trace` before commit.

### Stacking Transformers Without Reasoning About Interaction

Building a stack of `ReaderT` over `StateT` over `ExceptT` over `IO` and assuming effects compose linearly, when in fact error propagation, state survival, and reader availability all depend on ordering. Reason about each effect's failure semantics; switch to an effect system when the stack grows beyond readability.

### Assuming `Data.List` And `Data.Vector` Have The Same Strictness

Treating `Data.Vector` as a drop-in for lists and discovering strictness differences (Vector forces its elements, List defers them) that change memory behavior. Choose the structure whose strictness matches the access pattern; do not assume lazy and strict structures are interchangeable.

### Treating `do` Notation As Magic Rather Than Desugared Binds

Reading a `do` block as "imperative code" and missing that it is nested binds, so effect-interaction bugs (errors swallowed, state not surviving) are invisible. Expand `do` to binds mentally when debugging effect semantics.

## Self-Check

- [ ] Each function is in the weakest monad that captures its actual effects; `IO` is reserved for genuine I/O, and pure logic is in pure types (`Either`, `Reader`, `State`, plain functions).
- [ ] Transformer stacks are ordered so error propagation and state survival match intent; the ordering decision is documented, and an effect system is used when the stack exceeds readable depth.
- [ ] Laziness is managed: numeric folds use `foldl'`, accumulating data structures have strictness annotations where deferral would build thunks, and a space-leak investigation would identify the specific thunk source rather than adding `seq` blindly.
- [ ] No `IO` action is defined without being sequenced into the effectful flow; "my effect didn't run" was traced to sequencing, not assumed.
- [ ] Referential transparency is preserved: no `unsafePerformIO` or committed `trace` smuggles effects through pure signatures; effects stay in the type even when inconvenient.
- [ ] The effect system (transformers or a dedicated library) is consistent across the project, not a mix of ad-hoc stacks ordered differently per module.
- [ ] `do` notation is used for sequencing multiple effects but not for trivial single-bind cases better expressed as `<$>`/`<*>`; effect-interaction bugs are debugged by expanding `do` to binds.
- [ ] A memory profile was run on representative inputs to confirm the program's space behavior matches its apparent structure, with no surprise thunk buildup in production-shaped workloads.
