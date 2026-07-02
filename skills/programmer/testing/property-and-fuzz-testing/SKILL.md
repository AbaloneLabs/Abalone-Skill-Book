---
name: property_and_fuzz_testing.md
description: Use when the agent is deciding whether to test a function with example inputs or with properties, identifying invariants that a piece of logic should satisfy, designing property-based tests, choosing a generator strategy, shrinking a failing case to a minimal reproduction, deciding whether fuzzing is appropriate for a parser, codec, deserializer, or protocol handler, or evaluating when randomized testing adds value over hand-written examples. Also covers property-based testing, generative testing, fuzzing, invariant discovery, shrinking, coverage-guided fuzzing, model-based testing, and the boundary between logic that benefits from properties and logic that does not.
---

# Property And Fuzz Testing

Example-based testing proves a function is correct for the inputs you thought to write down. Property-based testing tries to prove it is correct for inputs you did not think of, by stating an invariant that must hold for a whole class of inputs and then throwing many generated cases at it. Fuzzing is the same idea applied to code that consumes untrusted input, where the goal is to find inputs that crash, hang, or violate a contract rather than to verify a known output. The judgment problem is not "should I use property tests" but "does this logic have a real invariant I can state, and is random input generation the cheapest way to find the bugs I care about."

Agents tend to either ignore property testing entirely (because examples feel sufficient) or apply it everywhere (because it sounds more rigorous). Both miss the point. Property testing is powerful precisely where a function has a genuine invariant that is hard to exhaust with examples — round-trip serialization, sorting, parsing, arithmetic identities, state machine transitions — and weak where the only meaningful check is a specific expected output for a specific input. This skill exists to make the choice deliberate: identify the invariant, confirm it is real, and choose the tool whose failure mode matches the bug class you are hunting.

## Core Rules

### Identify A Real Invariant Before Writing A Property Test

A property test needs an invariant: a statement that must hold for all inputs in a defined class, independent of any specific input. "For any two comparable values, sorting the pair then sorting again yields the same order" is an invariant. "The result for input 5 is 25" is an example, not a property. Before writing a property test, state the invariant in a sentence and confirm it is actually universal — not true only for the cases you have in mind.

Strong invariants tend to take a few recognizable shapes:

- **Round-trip / inverse** — `decode(encode(x)) == x` for serialization, compression, and parsing.
- **Idempotence** — `f(f(x)) == f(x)` for normalization, deduplication, and settling operations.
- **Commutativity / order independence** — `sort(a) == sort(reverse(a))`, or "processing in any order gives the same result."
- **Conservation / no-loss** — "the output collection has the same multiset of elements as the input," "the sum is preserved."
- **Postcondition** — "after sorting, every adjacent pair is in order," "after insertion, the structure satisfies its balance invariant."
- **State machine** — "every reachable state satisfies the invariant," "only valid transitions are accepted."

If you cannot find an invariant of one of these shapes, the function may not benefit from property testing, and example-based tests are more honest. Do not invent a fake property just to use the tool.

### Distinguish Logic That Benefits From Properties From Logic That Does Not

Property testing earns its cost where the input space is large, the invariant is genuine, and hand-written examples cannot realistically cover the space. It does not earn its cost where the input space is tiny, the expected output is a specific value, or the "invariant" is just a restatement of the implementation.

Logic that usually benefits:

- **Parsers, serializers, codecs** — round-trip and structural invariants catch a huge class of bugs that examples miss.
- **State machines and protocols** — transition validity and reachable-state invariants catch invalid sequences.
- **Algorithms over collections** — sorting, merging, partitioning, deduplication; order and conservation invariants apply.
- **Arithmetic and numeric transformations** — identities, bounds preservation, overflow behavior.
- **Anything consuming untrusted input** — fuzzing for crashes, hangs, contract violations.

Logic that usually does not benefit:

- **Business rules with specific expected outputs** — "a $60 cart ships free" is an example, not a property; the test is clearer as an example.
- **UI rendering** — the meaningful check is usually visual or snapshot-based, not an invariant.
- **Thin orchestration** — the behavior is the sequence of calls, which is better checked with examples or integration tests.

Match the tool to the shape of the logic. Forcing property tests on logic that has no real invariant produces tests that are elaborate, slow, and that assert trivialities.

### Make Generators Match The Domain, Not The Language Primitives

A property test is only as good as the inputs it generates. Generating random integers and strings is easy but often generates inputs the real code will never see, while missing the structured inputs where bugs hide. A generator should produce values that respect the domain's shape: valid dates, well-formed-but-edge-case JSON, identifiers within the allowed alphabet, collections within realistic size bounds.

For each property, design the generator deliberately:

- What is the full set of inputs the code claims to handle? The generator should cover it.
- What inputs are out of scope (wrong type, malformed)? Decide whether the property applies to them or whether a separate property checks rejection.
- What are the structured edge cases the generator should be biased toward — empty, single-element, maximum-size, boundary values, unicode, nested structures?

A property test with a naive generator can run a million cases and miss the one shape that breaks the code. Invest in the generator; it is where the bug-finding power lives.

### Use Shrinking To Turn A Random Failure Into A Minimal Reproduction

When a property test fails on a generated input, the raw failing case is often large and opaque — a 200-character string, a deeply nested structure, a 50-element list. The value of a property testing framework is not just finding the failure but shrinking it to the smallest input that still fails, which is usually human-understandable and points directly at the bug.

Take shrinking seriously:

- Prefer frameworks and generators that shrink automatically, and write generators in a way that supports shrinking (composing small shrinkable generators rather than producing opaque blobs).
- When a shrunk case appears, read it as a diagnosis: the minimal failing input usually reveals the exact assumption the code violated.
- Add the shrunk case as a regression example test, so the specific bug is pinned even if the property is later changed.

A property test that finds a failure but produces an unshrinkable, unreadable case is half a tool; the shrinking is what makes the failure actionable.

### Choose Fuzzing When The Bug Class Is "Unhandled Input"

Fuzzing is property testing aimed at robustness rather than correctness. The invariant is usually negative: "for no input does this crash, hang, leak, return malformed output, or violate a security contract." Fuzzing is the right tool when the code consumes input that is untrusted, externally supplied, or adversarial — file parsers, network protocol decoders, deserializers, regex engines, command-line argument parsers.

For fuzzing, decide:

- What is the correctness boundary — what must never happen (crash, infinite loop, memory unsafety, out-of-spec output)?
- What is the input entry point, and how do you feed generated bytes or structures into it?
- Is coverage-guided fuzzing appropriate (the fuzzer mutates inputs to explore new code paths), or is structural generation more effective (you generate well-formed inputs and mutate them)?

Fuzzing finds bugs that example-based testing almost never finds, because the bugs live in input shapes no human would write. It is disproportionately valuable for any code that faces external input.

### Decide The Run Budget Deliberately

Property and fuzz tests trade thoroughness for time. A property test run on 100 cases is fast but samples little of the input space; the same property run on 100,000 cases samples more but slows the suite. Fuzzing can run for seconds or for days. The right budget depends on the cost of a miss and the cost of the run.

A durable pattern:

- Run a small number of cases (tens to hundreds) on every commit, in the fast feedback loop, to catch regressions cheaply.
- Run a large number of cases (or a long fuzz session) nightly or pre-release, to explore the input space deeply.
- When a deep run finds a bug, add the shrunk case as a permanent example so the fast loop catches regressions on it forever.

Do not run a million-case property test on every save; do not run a fuzz campaign only once. Match the run budget to the phase of development.

### Combine Properties With Examples, Do Not Replace One With The Other

Properties and examples catch different bugs. Examples pin down specific, important cases — including the ones a stakeholder named, the ones in the spec, and the boundaries a developer knows matter. Properties cover the space between examples and catch inputs no one thought to write. A strong suite has both: examples for precision and communication, properties for breadth and surprise.

Do not delete your example tests when you add a property test. The example tests document intent and serve as readable specifications; the property tests guard the space the examples do not cover.

### Treat A Property That Never Fails As A Warning, Not A Success

A property test that runs thousands of cases and never fails might mean the code is correct. It might also mean the invariant is trivially true, the generator only produces inputs the code handles easily, or the assertion is too weak to catch real violations. A property that has never failed deserves scrutiny: is the invariant meaningful, is the generator reaching the interesting part of the input space, is the assertion actually checking something that could be wrong?

The most informative property tests are the ones that have failed at least once and whose shrunk cases now live in the suite as regression examples. A property that has never failed in its life may be testing nothing.

## Common Traps

### The Trivial Invariant

Stating an invariant that is true by construction ("the output is not null," "the result has the same type as the input") produces a property test that can never fail and that verifies nothing. The trap is that it looks rigorous because it uses the property-testing framework. A real invariant is one that could be violated by a buggy implementation; if you cannot imagine how the code could violate it, the invariant is too weak.

### The Naive Generator

Generating random primitives without domain structure produces inputs that are either all trivially handled or all rejected, missing the structured edge cases where bugs live. A property test with a naive generator runs many cases and finds nothing, which is mistaken for correctness. The generator is the test; invest in it.

### Property Testing Logic That Has No Invariant

Applying property tests to business rules, UI logic, or orchestration that has no genuine invariant produces elaborate tests that assert trivialities or that restate the implementation. The cost is high and the value is low. Use examples where examples are the honest representation of the requirement.

### Ignoring Shrinking

Treating the first failing case as the bug, rather than shrinking it to the minimal reproduction, leaves the failure opaque and hard to act on. The shrinking step is what turns a random failure into a diagnosis. Skipping it wastes the main advantage of property testing over ad-hoc random testing.

### Fuzzing Only Happy Paths

Fuzzing with a generator that only produces well-formed input finds bugs in handling well-formed input, which is the smaller bug class. The larger class — crashes, hangs, and contract violations on malformed, truncated, or adversarial input — requires the fuzzer to produce and mutate malformed input. If the fuzzer never sees a malformed input, it cannot find the bugs that matter for robustness.

### Treating A Long Fuzz Run As A One-Time Check

Running a fuzz campaign once at the start and never again gives a snapshot of correctness at one moment in time. Code changes; new code paths open; old invariants break. Fuzzing and deep property runs need to be part of the recurring pipeline (nightly, pre-release), not a one-time ceremony.

### Assuming A Green Property Run Means The Invariant Holds Everywhere

A property test samples the input space; it does not prove the invariant holds for all inputs unless the space is finite and fully enumerated. A green run means no counterexample was found in the cases tried, not that none exists. Report property results as "no counterexample found in N cases," not as "proved correct," and bias the case count and generator toward the parts of the space where a counterexample would plausibly hide.

### Replacing Examples Entirely With Properties

Dropping example tests in favor of properties removes the readable specification that examples provide. A failing example named "free_shipping_above_50" communicates intent; a failing property "prop_total_invariant" communicates a violated abstraction. Keep both; they serve different audiences and catch different bugs.

## Self-Check

- [ ] Each property test states a real invariant (round-trip, idempotence, order independence, conservation, postcondition, or state-machine validity) that a buggy implementation could plausibly violate, not a trivially-true restatement.
- [ ] Property and fuzz testing is applied to logic that has a genuine invariant or consumes untrusted input (parsers, codecs, state machines, collection algorithms, arithmetic), and not forced on logic whose meaningful check is a specific expected output.
- [ ] Generators produce domain-shaped inputs (valid dates, well-formed-but-edge-case structures, realistic sizes, boundary values) rather than naive random primitives, and are biased toward structured edge cases.
- [ ] Failing cases are shrunk to a minimal reproduction, and each shrunk case is added as a permanent regression example so the fast loop catches it forever.
- [ ] Fuzzing targets the negative invariant (no crash, hang, leak, or contract violation) and produces both well-formed and malformed/mutated input, not only happy-path shapes.
- [ ] The run budget is matched to the phase: small case counts in the fast feedback loop, large counts or long fuzz sessions nightly or pre-release.
- [ ] Example tests are retained alongside property tests; examples document intent and pin specific cases, properties cover the space between them.
- [ ] Property results are reported as "no counterexample found in N cases," not as a proof of correctness, and the case count and generator target the parts of the input space where a counterexample would plausibly hide.
- [ ] Every property that has never failed was scrutinized for whether the invariant is meaningful, the generator reaches the interesting space, and the assertion could actually fail.
- [ ] Fuzzing and deep property runs are part of the recurring pipeline, not a one-time check performed at the start of the project.
