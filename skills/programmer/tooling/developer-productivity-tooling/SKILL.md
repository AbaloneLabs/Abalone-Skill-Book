---
name: developer_productivity_tooling.md
description: Use when the agent is improving developer productivity and feedback loop speed, choosing or building scaffolding and code generators, setting up snippets and templates, integrating local AI assistants, selecting debuggers and profilers, enabling hot reload and fast iteration, or measuring and reducing the time between making a change and seeing its effect.
---

# Developer Productivity Tooling

Developer productivity tooling is about shrinking the feedback loop, the time between making a change and learning whether it works. This loop, more than any individual's skill, determines how fast a team ships and how many bugs escape, because a slow loop forces developers to batch changes (harder to debug), context-switch (losing flow), or guess rather than verify. Agents often reach for flashy tools (an AI assistant, a new IDE) when the dominant cost is something mundane (a 40-second build, a 2-minute test suite, a manual restart cycle), and the investment that would transform productivity goes unmade because it is invisible. The highest-leverage productivity work is often the least glamorous: measuring the loop, finding the slowest step, and removing it.

The judgment problem is that productivity tooling has diminishing and sometimes negative returns. A code generator that saves typing but produces boilerplate that must then be maintained can cost more than it saves. A hot-reload setup that is flaky teaches developers to distrust it and restart manually anyway. A local AI assistant that suggests plausible-but-wrong code accelerates the introduction of bugs. The agent must measure before optimizing (what is actually slow), prefer tools that remove steps over tools that speed them up, and recognize that a tool is only valuable if developers trust and adopt it. A productivity tool nobody uses has negative value because it consumed setup and maintenance effort for no gain.

## Core Rules

### Measure the feedback loop before optimizing it

Productivity work without measurement becomes a matter of opinion and fashion. Before investing in tooling, measure the actual loop: time from save to test result, time from commit to CI signal, time from starting work to first running build, frequency of context switches. Instrument or sample these, identify the slowest step, and optimize there. Developers' intuitions about what is slow are often wrong (they acclimate to chronic slowness and overestimate one-time costs). The biggest win is usually in the step that happens most often, not the step that is most annoying in isolation.

### Prioritize removing steps over speeding them up

The fastest step is the one that does not happen. Before making a slow step faster, ask whether it can be eliminated: can the build be incremental so only changed code recompiles? Can tests be selected to run only the affected subset? Can a manual restart be replaced by hot reload? Can a manual scaffolding step be generated? Removing a step from the loop yields larger gains than speeding up a step that still must happen, and it also reduces the cognitive overhead of remembering to do it. Always look for elimination before optimization.

### Make hot reload and fast iteration trustworthy

Hot reload (applying code changes without a full restart) is one of the highest-leverage productivity tools when it works and one of the most damaging when it is flaky. If hot reload sometimes fails to apply a change, or applies it in a way that produces different behavior than a clean restart, developers learn to distrust it and restart manually anyway, losing the benefit while paying the complexity cost. Invest in making hot reload reliable and deterministic, and provide a fast clean-restart as a fallback. If you cannot make hot reload trustworthy, do not ship it; a missing feature is better than an unreliable one that wastes debugging time on tooling-induced phantom bugs.

### Use scaffolding and generators for boilerplate, not for logic

Code generators (scaffolding a new component, endpoint, or test) eliminate repetitive setup and enforce consistency. Use them for boilerplate (the structural code that is the same every time), and ensure the output conforms to current conventions. Do not use generators to produce logic, because generated logic must be read, understood, and maintained like hand-written code but carries the false signal of being "done." Keep generated code minimal, mark it clearly as generated, and make regeneration idempotent so re-running the generator does not clobber manual additions. A generator whose output drifts from conventions is worse than no generator.

### Treat snippets and templates as living, shared artifacts

Snippets and templates encode team conventions and should be shared (in source control), versioned, and maintained. A snippet that encodes an outdated pattern propagates that pattern into every new code written with it. Review snippets like code, update them when conventions change, and prefer them for patterns that recur (logging setup, error handling shape, test scaffolding) rather than for one-off conveniences. Shared snippets also spread best practices: a well-crafted snippet teaches the team the idiomatic way to do something.

### Evaluate local AI assistants on failure modes, not just successes

Local AI assistants (completion, inline chat) can accelerate routine code, but their value depends on how they fail. A suggestion that is plausible but subtly wrong is worse than no suggestion, because it takes longer to verify than to write, and the wrongness is often in the details a busy developer skims. Evaluate an AI assistant by its error rate on realistic code, how easily its mistakes are caught in review, and whether it nudges developers toward or away from the codebase's conventions. Use AI for boilerplate and well-constrained tasks where verification is cheap; be cautious with it for logic, security-sensitive code, or domain-specific invariants where errors are costly and hard to spot. Always treat AI output as a draft to verify, never as an authority.

### Choose debuggers and profilers that fit the failure mode

The right tool depends on what you are debugging. A debugger (step, breakpoint, inspect) is for understanding control flow and state. A profiler (CPU, memory, allocation) is for finding where time or resources go. A tracer (distributed tracing) is for understanding cross-service latency. Logging is for post-hoc reconstruction. Match the tool to the question; using print statements where a debugger or profiler would do is a sign of missing tooling, not a virtue. Invest in learning the profiler and debugger for your stack; the productivity gain from diagnosing a performance problem in minutes instead of days is enormous, and these tools are consistently underused.

### Measure adoption, not just availability

A productivity tool that is set up but not used provides no value. Measure whether developers actually use the tooling (hot reload adoption, generator invocation counts, snippet usage), and investigate non-adoption: usually it signals the tool is unreliable, slow, or produces output developers do not trust. Adoption is the real metric; a tool with 100% availability and 20% adoption is a failed investment. Gather feedback and iterate on the tool, because a productivity tool is a product whose users are the team.

## Common Traps

### Optimizing without measuring

Intuition about what is slow is often wrong. Measure the loop, find the actual bottleneck, and optimize there rather than chasing the most annoying or most visible step.

### Speeding up a step that could be eliminated

The biggest gains come from removing steps (incremental builds, affected-test selection, hot reload), not from making a still-required step slightly faster.

### Shipping flaky hot reload

Unreliable hot reload teaches developers to restart manually, losing the benefit while paying the complexity. Make it deterministic or provide a fast fallback; do not ship it flaky.

### Generating logic instead of boilerplate

Generated logic must be maintained like hand-written code but carries a false "done" signal. Generate structure, not behavior, and keep output minimal and idempotent.

### Letting snippets and templates go stale

A snippet encoding an outdated pattern propagates it into all new code. Treat snippets as shared, versioned, reviewed artifacts and update them with conventions.

### Trusting AI output as authority

Plausible-but-wrong AI suggestions are costly because the errors hide in details. Treat AI output as a draft to verify, and be cautious in security-sensitive or logic-heavy code.

### Using the wrong tool for the failure mode

Print-statement debugging where a profiler is needed, or a profiler where a distributed trace is needed, wastes enormous time. Match the tool to the question and learn the profiler and debugger for your stack.

### Measuring availability instead of adoption

A tool that exists but is not used has negative value. Track adoption and investigate non-use, which usually signals an unreliable or untrusted tool.

## Self-Check

- Did you measure the actual feedback loop (save-to-test, commit-to-CI, start-to-build, context-switch frequency) before choosing what to optimize, rather than relying on intuition?
- Before speeding up a slow step, did you check whether the step can be eliminated entirely (incremental builds, affected-test selection, removing manual restarts)?
- If hot reload is offered, is it reliable and deterministic, with a fast clean-restart fallback, rather than flaky enough to force manual restarts?
- Are code generators used for boilerplate (structure, conventions) and not for logic, with output marked as generated, minimal, and idempotent on re-run?
- Are snippets and templates shared in source control, versioned, reviewed, and updated when conventions change, rather than left to drift?
- For any local AI assistant, have you evaluated its failure modes and error rate on realistic code, and constrained its use to tasks where verification is cheap?
- Are you matching the debugging tool (debugger, profiler, tracer, logging) to the actual failure mode, and have you invested in learning the profiler and debugger for your stack?
- Are you measuring adoption (not just availability) of productivity tooling, and investigating non-use as a signal that the tool is unreliable or untrusted?
