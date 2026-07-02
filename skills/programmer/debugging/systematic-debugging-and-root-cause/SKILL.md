---
name: systematic_debugging_and_root_cause.md
description: Use when the agent is investigating a bug, crash, wrong output, flaky test, production incident, performance regression, or unexpected behavior, and must form a hypothesis, narrow the cause, reproduce the failure, distinguish a symptom from a root cause, or decide when the investigation is complete. Also covers debugging methodology, hypothesis-driven debugging, binary search over changes and inputs, bisection with git, minimal reproduction, root cause analysis, five-whys, avoiding confirmation bias during investigation, reading stack traces and logs, and the discipline of not stopping at "it works now" without understanding why it was broken.
---

# Systematic Debugging And Root Cause

Debugging is a reasoning problem, not a search problem. The goal is not to make the failing test pass or to make the symptom stop; it is to build a correct mental model of why the system produced the wrong behavior, so that the fix addresses the cause and the failure mode is permanently closed. An agent that changes code until the symptom disappears has not debugged — it has gambled, and the same bug will return in a slightly different form, often in production, because the underlying cause was never identified.

Agents tend to fail at debugging in two opposite ways. Some jump to a fix based on the first plausible hypothesis, change code, see green, and move on — never confirming the hypothesis, never understanding the mechanism, and leaving the real cause in place. Others collect endless context, read every related file, and never form a falsifiable hypothesis, treating investigation as reading rather than reasoning. Both miss the point: debugging is a disciplined loop of forming a specific, falsifiable hypothesis, designing the cheapest experiment that could disprove it, and letting the result narrow the space. The judgment problem is deciding what hypothesis is worth testing, how to test it cheaply, how to narrow the search space, and when the cause is understood well enough to fix it rather than patch the symptom.

## Core Rules

### Reproduce The Failure Before Trying To Fix It

A failure you cannot reproduce is a failure you cannot confirm you have fixed. Before changing anything, obtain a reliable reproduction: a specific input, sequence of actions, environment state, or timing condition that triggers the wrong behavior on demand.

- If the failure is deterministic, capture the exact input and command that triggers it.
- If the failure is intermittent or timing-dependent (a race, a flaky test, a load-sensitive crash), do not treat "it sometimes happens" as a reproduction. Identify the condition that raises the probability — concurrency, specific data, memory pressure, time of day, request ordering — and construct a scenario that makes it likely.
- If the failure only appears in production, gather the request, the logs, the data state, the version, and the configuration at the time of failure before attempting any local reproduction.

A reproduction is the foundation of every later step. Without it, any "fix" is unverifiable; you will not know whether you fixed the bug or merely changed the timing enough to hide it.

### Form A Specific, Falsifiable Hypothesis

"I think it's a caching issue" is not a hypothesis; it is a hunch. A hypothesis is a specific, testable statement of the form: "If X is true, then doing Y should produce Z; if X is false, it should not." The value of a hypothesis is that it can be wrong, and finding out it is wrong is progress.

For each candidate cause, state:

- The exact mechanism you believe is producing the symptom.
- A prediction: what you should observe if the hypothesis is correct, and what you should observe if it is not.
- The cheapest experiment that distinguishes the two.

Prefer hypotheses that, if true, would explain the *entire* symptom over hypotheses that explain only part of it. A cause that explains three of four observations is probably not the cause; keep looking for one that explains all four. The hypothesis that survives every attempt to disprove it is the strongest candidate for the root cause.

### Narrow The Search Space With Binary Search

When the cause is unknown and the system is large, do not read everything. Use bisection to shrink the space where the cause can live.

- **Over changes:** if the bug appeared after a window of work, use `git bisect` (or an equivalent) to find the exact commit that introduced the behavior. This is often the fastest possible localization.
- **Over inputs:** if the failure depends on input, find the smallest input that still fails. Strip fields, shorten strings, remove steps, until you have the minimal reproduction. The boundary between the minimal failing input and a slightly different passing input usually pinpoints the assumption the code violates.
- **Over code:** if the failure is localized to a path, insert checks or bisect the execution by adding assertions at the midpoint. Determine which half contains the divergence from expectation, then bisect again.
- **Over time:** if the failure is intermittent, vary one dimension at a time (load, concurrency, data size, network latency) to find which dimension controls the probability.

Binary search is powerful because each experiment eliminates half the remaining space regardless of the outcome. A hypothesis that is wrong still narrows the search. Avoid experiments whose result cannot distinguish between hypotheses.

### Distinguish Symptom From Root Cause

Most bugs present a symptom (wrong output, crash, error message) that is several steps removed from the cause. The symptom is where the failure becomes visible; the cause is where the failure actually originates. Fixing the symptom removes the visible problem while leaving the cause in place to produce a different symptom later.

A useful discipline is to keep asking "why?" at each layer until you reach a cause that, if fixed, would prevent the entire class of failure, not just this instance.

- Symptom: "the report shows the wrong total."
- Why: "a line item was counted twice."
- Why: "the deduplication key was null for some rows."
- Why: "the key is derived from a field that is optional in the source feed, and nulls were not handled."

Fixing the report to divide by two is a symptom fix. Fixing the null handling is the root cause. The root cause is the layer where a change prevents the failure from recurring in any form, including forms you have not yet seen.

Be honest about when you have reached the root cause. If your "fix" would not obviously prevent a closely related variant of the bug, you are still at a symptom layer.

### Confirm The Hypothesis Before Fixing

Before writing the fix, confirm that the hypothesized cause actually produces the observed symptom. The strongest confirmation has two parts:

- **Positive confirmation:** with the cause present, the failure occurs.
- **Negative confirmation:** with the cause removed (or the hypothesized condition changed), the failure stops.

If you only observe one direction, the cause may be correlated but not causal. A flaky test that passes after a "fix" may have passed because the timing changed, not because the bug was addressed. The negative confirmation — making the failure reliably disappear by removing only the hypothesized cause — is what turns a guess into an understood fix.

Where practical, write a test that reproduces the failure from the cause alone, then verify the fix makes that test pass. This test becomes a permanent regression guard.

### Understand The Mechanism, Not Just The Location

"Found it: line 42 had a null check missing" is a location, not an understanding. Before fixing, explain to yourself the full mechanism: what invariant was violated, what assumption was wrong, what state was possible that the code did not account for, and why the system did not catch it earlier.

Understanding the mechanism matters because it tells you:

- whether the fix is local or whether the same flawed assumption exists elsewhere;
- whether a guard, a type change, an assertion, or a redesign is the right fix;
- whether existing tests should have caught this and why they did not;
- whether the failure mode is a one-off or a class of bugs that needs a structural response.

A fix applied without understanding the mechanism is a patch over a hole whose size and shape you do not know.

### Use The Cheapest Tool That Distinguishes Hypotheses

Debugging tools have different costs and different signal qualities. Match the tool to the question.

- **Reading code and logs** is cheapest and often enough for logic errors with clear traces. Use it first when the failure leaves a clear footprint.
- **Adding targeted logging or assertions** at suspected points is cheap and effective when the failure leaves no trace. Print the exact state you hypothesize is wrong, not generic "got here" markers.
- **A debugger** (breakpoints, stepping, watch expressions) is powerful for state-based bugs but slow for timing or concurrency bugs, because pausing changes the timing and can hide the failure.
- **A minimal reproducing test** is the most valuable long-term artifact: it confirms the cause, validates the fix, and guards against regression.
- **Profiling and tracing** are the right tools for performance and timing bugs, where the cause is in the pattern of execution rather than a single wrong value.

Avoid reaching for the most powerful tool first. A debugger session that explores state without a hypothesis burns time. Know what you are looking for before you attach a tool.

### Treat Intermittent Failures As Real, Not As Noise

Flaky tests and intermittent production errors are frequently dismissed as "timing issues" or "environment noise" and left unfixed. This is a mistake. An intermittent failure is a deterministic bug whose triggering condition you have not yet identified. The randomness is in your understanding, not in the system.

When a failure is intermittent:

- Capture everything about the runs that fail versus the runs that pass: data, ordering, load, clock, cache state, concurrent activity.
- Look for a shared condition among the failing runs that is absent from the passing runs.
- Construct a scenario that raises the probability of failure until it is reproducible, then debug deterministically.

A flaky test that is ignored will eventually fail a release build, block a deploy, or mask a real regression. Treat every intermittent failure as an open investigation until its cause is understood.

### Decide When The Investigation Is Complete

An investigation is complete when you can answer all of the following, not merely when the symptom stops:

- What is the root cause — the specific wrong assumption, missing case, or violated invariant?
- What is the mechanism by which it produced this symptom?
- Does the fix prevent the entire class of failure, or only this instance?
- Could the same flawed pattern exist elsewhere in the codebase, and have those locations been checked?
- Is there a regression test that fails without the fix and passes with it?
- Why did existing tests, type checks, or reviews not catch this, and should that gap be closed?

If any of these is unanswered, the investigation is not complete even if the symptom is gone. Stopping at "it works now" is the most common debugging failure, because it leaves the cause in place to recur.

## Common Traps

### Fixing The First Plausible Cause Without Confirming

The first hypothesis that sounds right is often wrong, but it feels right because it fits the available story. Changing code based on an unconfirmed hypothesis, seeing the symptom stop, and declaring victory is confirmation bias dressed up as debugging. The symptom may have stopped for an unrelated reason (timing, data, cache). Confirm with a negative test before believing the fix.

### Stopping At "It Works Now"

When a change makes the symptom disappear, the temptation is to stop. But a symptom that disappears may have been hidden, not fixed — the timing shifted, the cache cleared, the data changed. If you cannot explain the mechanism by which your change fixed the bug, you have not fixed it; you have moved it. Always explain the mechanism before accepting a fix.

### Changing Multiple Things At Once

Applying several fixes, refactors, and tweaks in one pass and then seeing the bug disappear makes it impossible to know which change mattered. Change one thing at a time, observe, and keep the cause isolated. Bulk changes turn debugging into an uncontrolled experiment whose results cannot be interpreted.

### Debugging Under A Wrong Mental Model

If your mental model of how the system works is wrong, your hypotheses will be wrong in the same way, and you will chase causes that cannot exist. When investigation stalls, question the model itself: is the code actually doing what you assume? Is the data flowing the path you believe? Is the version you are running the one you think? Many stuck investigations unblock the moment the model is corrected, not when more code is read.

### Over-Generalizing From One Bug

After finding a cause, it is tempting to assume the same pattern is everywhere and to add defensive checks broadly. This adds noise and can mask other bugs by making the code tolerate invalid states. Fix the specific cause, check for genuinely analogous locations, and prefer making invalid states unrepresentable (types, assertions) over scattering runtime guards.

### Treating Logs As The System

Logs show what the code was instrumented to show, not what actually happened. A bug that occurs in uninstrumented code is invisible in the logs, and reasoning about "what the logs say" as if it were the full execution leads to hypotheses about the wrong layer. When logs are insufficient, add targeted instrumentation at the suspected point rather than reasoning about absence of evidence.

### Reproducing In A Different Environment And Assuming Equivalence

A reproduction that works on your machine but not in production, or vice versa, may be triggering a different cause that produces a similar symptom. When environments differ, confirm that the local reproduction exercises the same code path and the same cause, not merely the same symptom. A fix validated only against a dissimilar reproduction can ship and fail identically in production.

### Ignoring The Possibility Of Two Bugs

When a fix does not fully resolve the symptom, the assumption is often that the fix was incomplete. Sometimes there are two independent causes producing overlapping symptoms, and fixing one reveals the other. If a confirmed fix leaves residual failure, consider a second distinct cause rather than over-fixing the first.

## Self-Check

- [ ] The failure has a reliable reproduction (deterministic input, or a constructed scenario that raises an intermittent failure's probability until it is reproducible); the fix is validated against that reproduction.
- [ ] At least one specific, falsifiable hypothesis was formed and tested, with a predicted positive and negative observation, rather than changing code speculatively.
- [ ] The search space was narrowed deliberately (git bisect over changes, minimal failing input, midpoint assertions, or single-dimension variation for intermittent bugs), not by reading everything.
- [ ] The root cause is identified at the layer where a fix prevents the entire class of failure, not merely this instance; the symptom layer was not mistaken for the cause.
- [ ] The hypothesis was confirmed in both directions: the cause produces the failure, and removing only the cause stops it.
- [ ] The full mechanism is explained — the violated invariant, the wrong assumption, the unhandled state — not just the file and line of the fix.
- [ ] Only one change was applied per observation, so the cause-effect link is unambiguous.
- [ ] Intermittent failures were treated as deterministic bugs with unidentified triggers, not dismissed as noise or timing.
- [ ] A regression test exists that fails without the fix and passes with it, and the reason existing safeguards did not catch the bug was considered.
- [ ] The investigation is complete: root cause, mechanism, class-of-failure prevention, analogous locations checked, regression test, and safeguard gap are all answered — not merely "the symptom stopped."
