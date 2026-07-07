---
name: c_error_handling_and_return_values.md
description: Use when the agent is writing or reviewing C error handling — return codes, errno, out-parameters, cleanup with goto, resource teardown on error paths, NULL sentinel returns, error propagation up a call chain, library API error contracts, or diagnosing swallowed errors, leaked resources on error paths, double-cleanup, or inconsistent error reporting conventions within a codebase.
---

# C Error Handling And Return Values

C has no exceptions. Every function that can fail must signal failure through its return value or an out-parameter, and every caller must check that signal and decide what to do. There is no language-enforced error propagation: a caller can silently ignore a returned error code, and the compiler will not warn. There is no automatic cleanup: resources acquired before a failure must be released by hand on every error path, and forgetting one path leaks. The judgment problem is that error handling in C is a discipline the programmer imposes on the code, not a feature the language provides, and the cost of getting it wrong is leaks, silent corruption, and security holes that only surface on the rare path that was never tested.

Agents tend to write the happy path and treat error handling as an afterthought, then produce functions that leak on the second-allocation-fails path, that swallow errors by returning a default, or that mix incompatible error conventions (some functions return -1 on error, some return NULL, some set errno, some return a success count) so callers cannot check consistently. The judgment problem is to choose an error-reporting convention per API and apply it uniformly, to structure every function so cleanup is correct on every exit path, and to propagate errors with enough context that a caller can act on them. This skill is about treating error handling as the primary design problem of a C function, not the leftover.

## Core Rules

### Choose One Error-Reporting Convention Per API And Apply It Uniformly

A C API should have a single, documented error convention so callers know how to check. The common conventions are:

- **Return a status code** (0 for success, non-zero/error code for failure): best for functions whose primary output is a side effect or where the return value is naturally a status. POSIX uses this (`int` return, -1 on error with errno set).
- **Return a sentinel** (NULL pointer, or -1 for a function returning an index/size): best for functions whose primary output is a pointer or value, where a sentinel is unambiguous. `malloc` returns NULL, `fopen` returns NULL, `getchar` returns EOF.
- **Return a success count** (number of items processed, -1 on error): used by `scanf`, `fread`. Distinguish "0 items, not an error" from "-1, error" carefully.
- **Out-parameter for the value, return a status**: when a function must return both a value and a status, return the status and write the value through a pointer out-parameter (`int parse(const char *s, Result *out)`).

Pick one convention for the API, document it in a header comment, and use it for every function. Mixing conventions within one API forces callers to remember per-function rules and guarantees bugs.

### Propagate Errors With Context, Not Just A Bare Code

A bare `-1` propagated up five call levels tells the top-level caller nothing about where or why the failure happened. Propagate context: either return a distinct error code per failure site (an enum), or use a string/struct error object that accumulates context as it unwinds. The minimum is to preserve the original error (the errno value or the low-level code) and add a layer of "what was being attempted."

Common patterns: a project-wide error enum (`typedef enum { OK = 0, ERR_IO, ERR_PARSE, ERR_NOMEM } Status;`), or an error object (`typedef struct { int code; const char *msg; } Error;`) that each layer enriches. Avoid overwriting errno with a later call before logging it — many libc functions set errno, so capture it into a local immediately.

### Use goto For Centralized Cleanup On Error Paths

The idiomatic C pattern for a function that acquires multiple resources is to use `goto` to a single cleanup label, so every error path runs the same teardown in reverse acquisition order. This is the one place `goto` is not just acceptable but correct, because the alternative (duplicating cleanup at each error site) is longer, drift-prone, and easy to get wrong.

```
Status do_work(void) {
    void *a = acquire_a();
    if (!a) return ERR_A;
    void *b = acquire_b();
    if (!b) { status = ERR_B; goto cleanup_a; }
    void *c = acquire_c();
    if (!c) { status = ERR_C; goto cleanup_b; }
    /* use a, b, c */
    status = OK;
    release_c(c);
cleanup_b:
    release_b(b);
cleanup_a:
    release_a(a);
    return status;
}
```

Structure cleanup so each label releases exactly the resources acquired so far, in reverse order. Initialize pointers to NULL so a `release(x)` that handles NULL works even if the cleanup chain is entered partway. This single discipline eliminates the largest class of C resource leaks.

### Check Every Call That Can Fail, And Decide Explicitly

A C function that ignores a returned error code is a latent bug. Check every call that documents a failure mode, and decide explicitly: propagate the error, recover, or abort. "Decide explicitly" means a conscious choice, not silence. If you choose to ignore an error (e.g., a close that is best-effort), comment why, so a reader knows it was deliberate.

Be especially careful with functions whose error is easy to miss: `snprintf` returns the number of bytes that *would* have been written, so a truncated result is not signaled by a negative return; `fwrite` returns a count that may be less than requested; `strtol` sets errno on overflow but returns a sentinel that overlaps valid input. Read the man page for the exact failure signaling of each function you call.

### Distinguish Initialization Failure From Runtime Failure

A function that initializes a long-lived object (opens files, allocates buffers, connects) should fail completely or not at all: if any step fails, undo the steps that succeeded and return an error, leaving no half-initialized object. The caller never receives a partially-initialized object that must be checked before use. This is the C analog of RAII: full construction or clean failure.

For runtime operations on an already-initialized object, the object remains valid after a failure (the operation did not happen, but the object is usable). Be clear in the API contract which kind of failure it is, because it changes whether the caller must tear down the object.

### Capture errno Immediately, Before Any Other Call

`errno` is only valid immediately after a libc call that documents it sets errno, and only if the call reported failure (errno is not cleared on success). Any intervening libc call may overwrite it. Capture `int e = errno;` into a local the instant you detect the failure, before logging, before other calls. Logging itself often calls libc functions that clobber errno.

Never check errno to determine if a call failed; check the call's return value, then consult errno for the reason. `errno` is thread-local in modern libc, so it is safe to use across threads, but the capture-immediately rule still holds.

### Make NULL A Documented, Handled Possibility

Functions returning a pointer commonly return NULL on failure. Callers must check. A caller that dereferences without checking is a null-deref crash waiting for the allocation-fails path. For functions that return NULL both as "empty" and as "error" (rare, ambiguous), change the API to separate the two cases, because callers cannot distinguish them.

## Common Traps

### Leaking On The Second-Allocation-Fails Path

`a = malloc(...); b = malloc(...); if (!b) return ERR;` leaks `a`. Use the goto-cleanup pattern so every error path releases resources acquired so far.

### Swallowing An Error By Returning A Default

`if (read(...) < 0) return 0;` hides the failure from the caller, which proceeds as if the read succeeded with zero bytes. Propagate the error instead, or document the swallow explicitly.

### Mixing Error Conventions In One API

Some functions return -1 on error, some return NULL, some return a count with -1 for error. Callers guess and get it wrong. Pick one convention per API and document it.

### Ignoring snprintf Truncation

`snprintf` returns the total length that would have been written; a return >= buffer size means truncation, not an error. Check the return value against the buffer size to detect truncation, or the output is silently cut.

### Overwriting errno Before Logging

`log("failed: %s", strerror(errno));` looks correct but if `log` calls libc internally it may clobber errno. Capture `int e = errno;` first, then `strerror(e)`.

### Inconsistent Cleanup When A Function Has Multiple Exit Points

A function with early returns that each release some resources but not others, drifts as resources are added. Centralize cleanup with a single label and goto, or a single return path.

### Using errno To Detect Failure

`fopen(...); if (errno) ...` is wrong because errno may be set by an earlier call and not cleared on success. Check the return value (NULL), then errno.

### Returning A Pointer That Becomes Invalid

A function returns a pointer to a stack buffer, a freed heap block, or a static buffer that the next call overwrites. The caller dereferences it later and gets garbage or a crash. Return a value, allocate and document ownership, or require the caller to pass a buffer.

## Self-Check

- [ ] The API uses one documented error-reporting convention (status code, sentinel, count, or out-parameter) applied uniformly across every function, with no per-function guessing required of callers.
- [ ] Functions that acquire multiple resources use a centralized goto-cleanup pattern (or equivalent) so every error path releases exactly the resources acquired so far, in reverse order.
- [ ] Errors are propagated with context (distinct codes or an enriched error object), not bare sentinels that lose the source of failure.
- [ ] Every call that can fail is checked and the result is acted on explicitly — propagate, recover, or document a deliberate ignore — with no silently swallowed errors.
- [ ] errno is captured into a local immediately after detecting a failure, before any other call (including logging) that could clobber it, and is never used as the failure-detection signal.
- [ ] Initialization functions either fully construct the object or clean up and return failure, never leaving a half-initialized object for the caller to check.
- [ ] Truncation-prone functions (snprintf, fread with short counts) have their specific failure signaling checked, not just the negative-return path.
- [ ] No returned pointer refers to a stack buffer, freed memory, or a shared static buffer that a subsequent call overwrites; ownership of returned memory is documented.
