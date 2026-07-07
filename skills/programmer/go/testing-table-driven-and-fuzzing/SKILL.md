---
name: go_testing_table_driven_and_fuzzing.md
description: Use when the agent is writing or reviewing Go tests — table-driven tests, subtests and t.Run, testify vs standard library, test helpers and t.Helper, golden files, mocks and fakes, httptest servers and clients, the testing/iotest and testing/fstest packages, race detector, benchmarking with Benchmark and b.N, setup/teardown, or writing fuzz tests with the native Go 1.18+ fuzzing framework (FuzzFunc, corpus, seed corpus, fuzzing targets).
---

# Go Testing, Table-Driven Tests, And Fuzzing

Go ships a testing framework in the standard library (`testing`) and a built-in `go test` runner, and since Go 1.18 a native fuzzing framework. The idioms are specific: table-driven tests with `t.Run` subtests, benchmarks with `b.N`, and fuzz targets with `f.Add`/`f.Fuzz`. The judgment problem is that Go's testing is deliberately minimal — there is no built-in assertion library, no built-in mock framework — so the discipline of writing good tests is entirely on the programmer. Tests that do not use subtests produce one pass/fail per function with no granularity, benchmarks that ignore `b.N` measure the wrong thing, and fuzzing that is not wired into `go test -fuzz` finds nothing.

Agents tend to write a single test function with many unrelated assertions (no subtest granularity), to reach for `testify` assertions when a plain `if` + `t.Fatalf` is clearer, to write benchmarks that allocate in the loop, or to add a fuzz function without a seed corpus or without running it under the race detector. The judgment problem is to structure tests as table-driven subtests for granularity and shared setup, to keep assertions plain and readable, to benchmark correctly (reset timers, report allocs, respect `b.N`), and to use fuzzing where parsers and untrusted-input code benefit. This skill is about using Go's testing idioms deliberately rather than fighting them with framework-heavy patterns.

## Core Rules

### Use Table-Driven Tests With t.Run Subtests For Granularity And Shared Setup

The idiomatic Go test is a table-driven test: a slice of struct cases, each with a name, inputs, and expected output, looped over with `t.Run(tc.name, func(t *testing.T) {...})`. This gives each case its own subtest (run individually with `-run TestName/case_name`), shares setup, and makes adding a case trivial (add a struct to the slice). Prefer this over separate test functions per case or a single function with sequential assertions.

```
func TestParse(t *testing.T) {
    cases := []struct {
        name string
        in   string
        want int
    }{
        {"simple", "42", 42},
        {"negative", "-7", -7},
        {"empty", "", 0},
    }
    for _, tc := range cases {
        t.Run(tc.name, func(t *testing.T) {
            got, err := Parse(tc.in)
            // assertions...
        })
    }
}
```

Name cases descriptively (the subtest name shows in output and can be selected with `-run`). Use `t.Parallel()` in subtests for independent cases (call `tc := tc` before the closure, or rely on Go 1.22+ loop variable semantics).

### Use Plain Assertions (if + t.Fatalf); Reserve Frameworks For Genuine Value

Go's standard library has no assertion functions; the idiom is `if got != want { t.Fatalf("got %v, want %v", got, want) }`. This is verbose but readable, and the failure points exactly at the line. `testify/assert` reduces boilerplate but adds a dependency, changes the failure style (assert vs require), and can obscure the failing line. Use plain assertions by default; reach for `testify` (or another library) only when the team has standardized on it and the boilerplate reduction is worth the dependency.

Use `t.Helper()` in test helper functions so failure line numbers point to the caller, not the helper. This is essential for readable custom assertion helpers.

### Benchmark Correctly: Respect b.N, Reset Timers, Report Allocations

A Go benchmark function runs `b.N` times, where `b.N` is chosen by the runner to get a stable measurement. The loop must use `b.N`, and setup that should not be measured must happen before `b.ResetTimer()`. Allocation reporting (`b.ReportAllocs()`) reveals GC pressure. Common mistakes:

- Not looping `for i := 0; i < b.N; i++` — the benchmark measures one iteration.
- Setup inside the loop — the setup is measured, not the operation.
- Not resetting the timer after setup — setup time inflates the result.
- Compiler optimizing away the work — assign results to a package-level sink (`_ = result` or `sink = result`) so the compiler cannot elide the call.

Run benchmarks with `go test -bench=. -benchmem -count=N` and compare with `benchstat` to detect significant differences (avoid concluding from a single noisy run). Use `-cpuprofile`/`-memprofile` to profile.

### Wire Fuzzing Into The Build With Seed Corpus And Run Under The Race Detector

Go 1.18+ native fuzzing (`func FuzzParse(f *testing.F)`) generates random inputs for a target function, finding crashes (panics, infinite loops) and (with the race detector) data races. To use it well:

- Write a fuzz function that takes `*testing.F`, adds seed corpus with `f.Add(seedInputs...)` (realistic inputs that exercise known paths), and calls `f.Fuzz(func(t *testing.T, in string) {...})` to run the property under test.
- The fuzz body should call the function under test and assert invariants (e.g., `Parse(s)` does not panic, or `Format(Parse(s))` round-trips), not just "does not crash" — stronger properties find more bugs.
- Run with `go test -fuzz=FuzzParse -fuzztime=Ns` to fuzz continuously; corpus entries that crash are saved in `testdata/fuzz/` as regression tests.
- Run fuzzing under the race detector (`-race`) so concurrency bugs in the fuzzed code are caught.

Fuzzing is disproportionately valuable for parsers, decoders, and any code consuming untrusted input. Commit the seed corpus and any crash regressions so they run in normal `go test`.

### Use httptest, iotest, And fstest For Realistic, Deterministic Dependencies

The standard library provides test doubles that are more realistic than hand-rolled mocks:

- `net/http/httptest.NewServer` / `NewRequest` / `NewRecorder`: test HTTP clients and servers without real network. `NewRecorder` captures handler responses; `NewServer` runs a real server on a local port for client testing.
- `testing/iotest`: wraps readers with behaviors (slow close, timeout, one-byte reads) to test how your code handles problematic readers. Use `iotest.TimeoutReader` to test timeout handling.
- `testing/fstest.TestFS`: validates a custom `fs.FS` implementation against the contract.

Prefer these and small hand-written fakes over a mock framework. The test double should be controllable (inject failures) and observable (record calls).

### Manage Setup And Teardown With t.Cleanup

`t.Cleanup(func(){...})` registers a teardown that runs at the end of the test (and its subtests), in reverse registration order — the Go equivalent of defer but tied to the test lifecycle. Use it for setup that needs teardown (starting a server, creating a temp dir with `t.TempDir()` which auto-cleans). `t.TempDir()` returns a temp directory that is cleaned up automatically; prefer it over manual temp-file management.

For package-level setup/teardown, use `TestMain(m *testing.M)` (e.g., to initialize a database once for the package), calling `m.Run()` and then teardown.

## Common Traps

### Single Test Function With Many Sequential Assertions

No subtests means one pass/fail per function and no way to run a specific case. Use table-driven `t.Run` subtests for granularity.

### Benchmark Not Using b.N Or Measuring Setup

`for i := 0; i < 100; i++` measures 100 fixed iterations; setup inside the loop is measured. Use `b.N`, setup before `b.ResetTimer()`.

### Reaching For testify When Plain if + Fatalf Is Clearer

`assert.Equal(t, got, want)` adds a dependency and changes failure style; `if got != want { t.Fatalf(...) }` is plain and points at the line. Use frameworks only when the team standardizes on them.

### Fuzz Function With No Seed Corpus Or Weak Property

A fuzz target with no `f.Add` seeds starts from random bytes and finds little; a target that only checks "no panic" finds less than one that checks a round-trip property. Add realistic seeds and a strong invariant.

### Not Running Tests Under The Race Detector

`go test` without `-race` misses data races. Run `go test -race` in CI for any concurrent code.

### Compiler Optimizing Away The Benchmark Work

If the benchmark's result is unused, the compiler may elide the call. Assign to a package-level sink to prevent elision.

### Mock Framework Instead Of A Small Hand-Written Fake

Go's structural interfaces make small fakes trivial; a mock framework adds code generation and lock-in. Extract a small interface and write a fake.

### Temp Files Not Cleaned Up

Manual `os.MkdirTemp` without cleanup leaks. Use `t.TempDir()` which auto-cleans via `t.Cleanup`.

## Self-Check

- [ ] Tests are structured as table-driven `t.Run` subtests with descriptively named cases, allowing individual case selection with `-run` and shared setup.
- [ ] Assertions are plain (`if got != want { t.Fatalf(...) }`) with `t.Helper()` in custom assertion helpers; a mock/assertion framework is used only where the team has standardized on it.
- [ ] Benchmarks loop on `b.N`, do setup before `b.ResetTimer()`, report allocations with `b.ReportAllocs()`, and prevent compiler elision by sinking results; results are compared with `benchstat` over multiple `-count` runs.
- [ ] Fuzz targets have a realistic seed corpus (`f.Add`), assert a strong invariant (not just "no panic"), are run with `-fuzz`/`-fuzztime`, and crash regressions are committed to `testdata/fuzz/`.
- [ ] Tests use `httptest`, `iotest`, and `fstest` for realistic, deterministic test doubles, and small hand-written fakes over a mock framework.
- [ ] Setup/teardown uses `t.Cleanup` (and `t.TempDir` for temp files), with `TestMain` for package-level one-time setup.
- [ ] The test suite runs under `-race` in CI, and concurrent code is exercised by tests that would surface races.
- [ ] Fuzzing and race detection cover all parsers, decoders, and untrusted-input code paths.
