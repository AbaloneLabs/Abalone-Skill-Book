---
name: haskell_testing_and_property.md
description: Use when the agent is writing Haskell tests with HUnit or QuickCheck, designing property-based tests and generators, testing pure functions and monadic effects, testing for law invariants like functor and monad laws, using mocking and dependency injection for IO, or diagnosing test coverage gaps, non-exhaustive generators, and flaky property tests in Haskell applications.
---

# Haskell Testing and Property

Haskell's purity makes testing uniquely effective, and that advantage is easy to squander. Because pure functions are deterministic functions of their inputs, property-based testing can generate hundreds of random inputs and check invariants that would require elaborate setup in impure languages. But the same purity that makes testing powerful also creates the trap: developers test the easy pure core exhaustively and leave the effectful shell (parsing, IO, concurrency, error handling) undertested, or they write property tests with weak generators that never exercise edge cases (empty lists, boundary integers, nested bottoms), producing a green suite that misses real bugs. The judgment problem is not "how do I write a QuickCheck property" but designing tests that cover the actual risk surface: the pure core via properties, the effectful boundaries via injection, and the law invariants that keep abstractions sound as they evolve.

The recurring failure mode is a developer who writes a few `HUnit` assertions for the happy path, adds a QuickCheck property with the default generator, and declares the code tested, while the real bugs live in edge cases the generator never produced (empty input, max-bound integers, deep recursion, partial functions hitting missing patterns) or in the IO shell that was never tested because "it is just wiring." The opposite failure is writing property tests so specific that they re-implement the function under test, passing trivially without catching bugs. Real Haskell testing requires generators that cover the input space meaningfully, properties that express laws rather than restate the implementation, and explicit testing of the effectful boundaries that pure tests cannot reach.

## Core Rules

### Test the pure core with properties; test the effectful shell with injection

The architecture (pure core, effectful shell) maps directly to a testing strategy:

- **Pure core**: test with property-based tests (`QuickCheck`) that generate inputs and check invariants/laws. These are fast, deterministic, and high-value.
- **Effectful shell**: test by injecting the effectful dependencies (a `Logger`, a `Repository`, a `Clock`) as interfaces, providing test implementations (in-memory, recorded) in tests. This tests the wiring and sequencing without hitting real IO.

The boundary between pure and effectful is the boundary between property tests and injection-based tests. Keep the pure core large to maximize what property tests cover.

### Design generators that cover the input space meaningfully

A property is only as good as its generator. The default `Arbitrary` instances often miss the cases where bugs hide. Rules:

- Generate edge cases explicitly: empty collections, single-element collections, `minBound`/`maxBound` for bounded types, negative and zero for numbers.
- Use sized generators (`sized`) to control the depth/size of generated structures, so tests run quickly at small sizes and thoroughly at large sizes.
- Generate the full domain, including invalid inputs where the function must handle them gracefully; a parser should be tested with malformed input, not just well-formed.
- Shrink: QuickCheck shrinks failing cases to minimal counterexamples; ensure your `Arbitrary` instances support shrinking (the default does; custom generators should too).

A generator that never produces an empty list will never catch the head-of-empty bug.

### Express properties as laws, not as restatements of the implementation

A strong property expresses a law the function must satisfy independent of its implementation. Rules:

- **Round-trip properties**: `parse (show x) == x` for serializable types; `decode (encode x) == x` for codecs. These catch a large class of bugs without restating the implementation.
- **Identity/idempotence laws**: `f (identity x) == f x`, `f (f x) == f x` for idempotent operations.
- **Equivalence to a reference**: `optimized x == reference x` where the reference is a simpler, obviously-correct (but slow) implementation.
- **Algebraic laws**: functor/monad/applicative laws for your abstractions; these catch subtle correctness bugs as the abstraction evolves.

Avoid properties that restate the implementation (`sort xs == sortBy compare xs` where both are the same algorithm); they pass trivially.

### Test algebraic laws for your abstractions

If you define a `Functor`, `Applicative`, `Monad`, `Semigroup`, `Monoid`, or custom type class instance, test its laws with QuickCheck. Rules:

- Functor law: `fmap f . fmap g == fmap (f . g)`, and `fmap id == id`.
- Monad laws: left/right identity and associativity.
- Monoid laws: identity and associativity.

Law violations are subtle bugs that break every caller that relies on the abstraction. QuickCheck's `checkers` library automates many of these.

### Force evaluation in tests to surface bottoms

Because of laziness, a test that constructs a value but never forces it may pass while the value holds a bottom. Rules:

- Use `force` (`Control.DeepSeq`) on results before asserting, to surface bottoms at test time.
- `evaluate` in `IO` to force to WHNF where full force is not needed.
- For property tests, force both the input and the output to catch bottoms in either.

A test suite that never forces can pass while the code throws `*** Exception: ...` in production when the value is finally used.

### Test partial functions and non-exhaustive patterns

Partial functions (pattern matches that do not cover all cases, `head`, `!!`, `fromJust`) produce bottoms on bad input. Rules:

- Enable `-Wall` and `-Wincomplete-patterns` so the compiler flags non-exhaustive matches.
- For functions that should be total, ensure all patterns are covered and test the edge cases.
- For functions that are intentionally partial, document the precondition and test that valid inputs succeed; consider returning `Maybe`/`Either` instead.

### Mock effects via interfaces, not via monkey-patching

Haskell has no runtime monkey-patching; effectful dependencies are injected as functions or records of functions (the "tagless final" or "MTL" style, or a record-of-functions). Rules:

- Define the effect as an interface (a type class or a record of functions).
- Provide a production implementation (real IO) and a test implementation (in-memory, recorded).
- Test the shell by injecting the test implementation and asserting on the recorded calls and in-memory state.

This tests the sequencing and error handling of the shell without hitting real databases or networks.

### Measure and improve coverage

Use `stack test --coverage` or `cabal test --enable-coverage` to identify untested code. Rules:

- Focus coverage improvement on the pure core and the error paths, which are where bugs hide.
- Coverage of happy-path IO wiring is less critical than coverage of edge cases in the core.
- A property test that exercises many inputs contributes more coverage than a single assertion.

### Keep tests deterministic and fast

Property tests with random generation should set a fixed seed in CI for reproducibility, while allowing local runs to vary. Keep the pure test suite fast (seconds) so it runs on every save; isolate slow integration tests separately.

## Common Traps

### Default generators that miss edge cases

`Arbitrary` for lists rarely produces empty lists by default in some setups; numbers rarely hit bounds. Customize generators to include edge cases.

### Properties that restate the implementation

`sort xs == mySort xs` where `sort` and `mySort` share logic passes trivially. Use a law or a different reference implementation.

### Testing only the happy path

A few assertions on typical inputs miss the empty, boundary, and malformed-input cases where bugs hide. Generate the full domain.

### Effectful shell untested

"Pure core is tested, the IO is just wiring" leaves the wiring bugs (wrong order, missing error handling, resource leaks) untested. Inject test implementations and test the shell.

### Bottoms hidden by lazy tests

A test that never forces the result passes while the value holds a bottom. `force` results in tests.

### Non-exhaustive patterns shipped

A missing pattern produces a runtime bottom only when the unmatched case is hit, which tests may miss. Enable `-Wincomplete-patterns`.

### Over-specific generators that never vary

A generator that always produces the same shape of input is a unit test in disguise. Use sized, random generators that vary structure.

### Flaky property tests from non-determinism

A property that depends on wall-clock time, random sources not under the generator's control, or external state flakes in CI. Make properties pure functions of generated inputs; inject time/random as interfaces.

## Self-Check

- Is the pure core tested with property-based tests and the effectful shell tested via injected test implementations, matching the architecture?
- Do generators cover edge cases (empty, single-element, bounds, negative/zero, malformed input) and support shrinking, with size controlled via `sized`?
- Are properties expressed as laws (round-trip, identity, idempotence, equivalence to a reference, algebraic laws) rather than restatements of the implementation?
- Are algebraic laws (functor, applicative, monad, semigroup, monoid) tested for custom instances, ideally via a law-checking library?
- Are test results forced (`force`/`evaluate`) to surface bottoms at test time, for both inputs and outputs?
- Are partial functions flagged by `-Wall`/`-Wincomplete-patterns`, with total functions preferred or preconditions documented and tested?
- Are effectful dependencies injected as interfaces with test implementations, so the shell's sequencing and error handling are tested without real IO?
- Is coverage measured and improved on the pure core and error paths, with the pure suite fast and deterministic (fixed seed in CI) and slow integration tests isolated?
