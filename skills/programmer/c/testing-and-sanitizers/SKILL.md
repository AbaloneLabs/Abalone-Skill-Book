---
name: c_testing_and_sanitizers.md
description: Use when the agent is writing or reviewing C test code, choosing a testing framework (Unity, CMocka, Check, gtest for C), setting up AddressSanitizer/UBSan/ThreadSanitizer/MemorySanitizer, writing mocks and fakes for hardware or OS dependencies, testing memory-unsafe code, reproducing intermittent bugs, or diagnosing heap corruption, use-after-free, or data races that only appear under instrumentation or specific memory pressure.
---

# C Testing And Sanitizers

C's lack of memory safety means that tests which pass do not prove correctness the way they do in safer languages. A test that exercises the happy path may run over a buffer that happens to be followed by valid memory, may leak memory that the test never checks, may invoke undefined behavior that happens to work on this compiler and flags, or may race in a way that only loses on a different core count. To find these bugs you must combine unit tests with dynamic instrumentation — AddressSanitizer (ASan), UndefinedBehaviorSanitizer (UBSan), ThreadSanitizer (TSan), MemorySanitizer (MSan) — and you must structure tests to exercise the error paths and edge cases that real execution rarely hits. The judgment problem is that "the tests pass" is a much weaker statement in C than in memory-safe languages, and getting value from testing requires deliberate choices about what to instrument, what to mock, and which edge cases to target.

Agents tend to write a few happy-path tests, run them without sanitizers, and declare the code tested, then ship memory bugs that a five-minute ASan run would have caught. The judgment problem is to make sanitizers a default part of the test workflow, to mock external dependencies (hardware, OS, network) so unit tests are deterministic and fast, and to target the specific edge cases where C code fails: boundary indices, zero-length operations, overflow, and every error path. This skill is about treating C testing as an instrumentation-and-edge-case discipline, not a coverage-percentage exercise.

## Core Rules

### Make Sanitizers A Default Part Of The Test Build, Not An Afterthought

The dynamic sanitizers (part of Clang and GCC) find bugs that no amount of happy-path testing will reveal:

- **AddressSanitizer (ASan)**: heap/stack/global buffer overflow, use-after-free, double-free, memory leaks (with LeakSanitizer). Compile and link with `-fsanitize=address`. This is the single highest-value tool for C; turn it on by default for debug/test builds.
- **UndefinedBehaviorSanitizer (UBSan)**: signed overflow, shift past bit width, null dereference, misaligned access, integer conversion issues. `-fsanitize=undefined`. Catches the UB that the compiler may later exploit.
- **ThreadSanitizer (TSan)**: data races in multi-threaded code. `-fsanitize=thread`. Cannot be combined with ASan.
- **MemorySanitizer (MSan)**: reads of uninitialized memory. `-fsanitize=memory`. Requires the entire program (including libc) to be instrumented, so harder to deploy, but uniquely catches uninitialized reads.

Run the test suite under ASan+UBSan as the default, and under TSan for any concurrent code. Sanitizers slow execution 2-10x, which is acceptable for tests. Keep a non-sanitized release build for performance validation.

### Target The Edge Cases Where C Code Actually Fails

C bugs cluster at specific edges that happy-path tests miss. Deliberately write tests for:

- **Boundary indices**: length 0, length 1, the last valid index, one-past-the-end. Off-by-one errors live here.
- **Empty and maximal inputs**: empty string, empty array, full buffer (truncation path), integer at INT_MAX/UINT_MAX (overflow path).
- **Every error path**: each `if (fail)` branch must be exercised by a test that triggers the failure (mock the failing dependency). Untested error paths are where resource leaks and partial-init bugs hide.
- **Alignment and padding**: if code casts a `char*` to a wider type, test with both aligned and unaligned input (unaligned access is UB on some architectures and caught by UBSan/ASan).
- **Encoding edges**: for text code, test multi-byte UTF-8, embedded null bytes, invalid byte sequences.

A test suite that only covers the typical case provides false confidence. The value is in the edge cases.

### Mock External Dependencies For Determinism And Speed

Unit tests must be deterministic, fast, and isolated. External dependencies — hardware registers, system calls, network, files, the clock — make tests slow, flaky, and environment-dependent. Mock them. C's usual mocking technique is a "fake" or "stub": a compile-time or link-time substitution of the real function with a controllable one.

- **Link-time injection**: compile the unit under test against a header declaring the dependency, and link the test against a fake implementation instead of the real one. This is the most common C mocking approach (CMocka, CMock).
- **Weak symbols / function pointers**: declare the dependency as a function pointer the unit calls, and the test sets the pointer to a fake. Or use weak symbols the test overrides.
- **Hardware abstraction**: for embedded code, route all hardware access through a HAL (hardware abstraction layer) board support package that the test replaces with a fake that models the hardware.

Fakes should be controllable (the test sets return values and error injection) and observable (the test can assert the fake was called with expected arguments). A fake that just returns success is weaker than one that records calls.

### Use Fuzz Testing For Parsers And Untrusted-Input Code

For code that consumes untrusted input (parsers, decoders, network protocol handlers), fuzz testing finds bugs unit tests miss by generating millions of malformed inputs. libFuzzer (Clang) and AFL are the standard C fuzzers. Integrate a fuzz target (a function that takes a byte buffer and exercises the parser) into the build, and run it under ASan+UBSan so any overflow or UB is caught.

Fuzzing is disproportionately effective for C because memory bugs that would be caught at runtime in safer languages instead corrupt silently; the fuzzer plus ASan surfaces them. Run fuzz targets in CI for a fixed time budget, and check in regression tests for any crash the fuzzer finds so it does not regress.

### Assert Invariants And Use _Static_assert For Compile-Time Checks

Beyond testing behavior, assert invariants the code depends on. Runtime `assert(cond)` catches logic errors during testing (disable in release with `NDEBUG`). `_Static_assert` catches layout, size, and configuration assumptions at compile time with zero runtime cost. Use both liberally: `assert(idx < len)` at function entry documents and checks the contract; `_Static_assert(sizeof(struct Header) == 16)` locks the ABI.

### Reproduce Intermittent Bugs Under Instrumentation Before "Fixing" Them

An intermittent crash or corruption is almost always a memory or concurrency bug that happens to surface only under specific timing or memory layout. Do not "fix" it by adding a speculative change and hoping. Reproduce it: run under ASan (for memory bugs) or TSan (for races) with a loop or stress harness that increases the chance of triggering it. The sanitizer will pinpoint the actual bug. A fix without a reproducing test is a guess and will regress.

## Common Traps

### Tests Pass But Code Was Never Run Under ASan

A test suite run without sanitizers gives false confidence; buffer overflows and use-after-free that happen to not crash are invisible. Make ASan+UBSan the default test build.

### Only Testing The Happy Path

Tests that call the function with typical inputs miss the off-by-one, the empty input, the overflow, and every error path. C bugs live at the edges; target them deliberately.

### Error Paths Never Exercised

Every `if (fail) return ERR;` branch is untested unless a test forces the failure. Use fakes/mocks to inject failures (malloc returns NULL, read returns -1) and assert the error path cleans up correctly.

### Mocking That Only Returns Success

A fake that always returns success tests the happy path through the dependency, not the failure handling. Fakes must be able to inject errors and record calls.

### Fixing An Intermittent Bug Without Reproducing It

Adding a defensive check without a reproducing test is a guess; the real bug remains and the "fix" may mask it. Reproduce under ASan/TSan first, then fix the root cause with a regression test.

### Ignoring UBSan Findings As "Theoretical"

Signed overflow, shift past width, and misaligned access are undefined behavior; the compiler may exploit them under optimization. A UBSan finding is a real bug even if it "works" today. Fix it.

### Fuzzing Without ASan

Running a fuzzer without ASan finds only the crashes that segfault; memory corruptions that do not immediately crash are missed. Always fuzz under ASan+UBSan.

### Asserts Disabled In Release Masking Bugs

`assert` is compiled out with `NDEBUG` in release, so a bug caught only by an assert ships. Use explicit error handling for real validation; reserve asserts for invariant checks that indicate a logic bug.

## Self-Check

- [ ] The test build runs under AddressSanitizer and UndefinedBehaviorSanitizer by default, and concurrent code is additionally run under ThreadSanitizer; a non-sanitized build exists only for performance validation.
- [ ] Tests deliberately target edge cases: zero/one/max lengths, boundary indices, full buffers (truncation), integer overflow, alignment, and encoding edges — not just typical inputs.
- [ ] Every error path (`if (fail)`) is exercised by a test that injects the failure via a controllable fake/mock, and the test asserts correct cleanup on that path.
- [ ] External dependencies (hardware, OS, network, clock) are abstracted behind a layer the test replaces with fakes that both inject failures and record calls.
- [ ] Parsers and untrusted-input code have fuzz targets integrated into the build, run under ASan+UBSan, with regression tests checked in for any crash found.
- [ ] Invariants are asserted at runtime (`assert`) and layout/configuration assumptions are locked with `_Static_assert` at compile time.
- [ ] Intermittent bugs are reproduced under instrumentation (ASan/TSan with a stress harness) before fixing, and each fix has a regression test that would have caught the original bug.
- [ ] UBSan findings are treated as real bugs (signed overflow, shift, misalignment are UB), not dismissed as theoretical, and asserts are not relied upon for release-build validation.
