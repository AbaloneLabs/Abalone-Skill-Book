---
name: acceptance_criteria_and_edge_cases.md
description: Use when the agent is writing acceptance criteria, enumerating edge cases for a feature, defining Given-When-Then scenarios, specifying boundary and failure behavior, or reviewing whether a spec is testable and complete.
---

# Acceptance Criteria And Edge Cases

Acceptance criteria are the contract that lets engineering, design, QA, support, and product agree on what "done" means. The recurring failure is writing criteria that describe the happy path and vague qualities, leaving every hard decision to be rediscovered during build, test, or after launch. Edge cases are not rare curiosities; they are where security bugs, data corruption, support spikes, and broken trust actually happen.

A spec that reads cleanly on the happy path can still be untestable, because it never says what happens when inputs are empty, permissions are missing, a concurrent edit collides, the network times out, or a migration runs against existing accounts. The product manager's job is to make behavior explicit across states so that acceptance criteria enable QA to write tests, design to review states, and leadership to sign off on launch with confidence.

Use this skill before answering questions such as "what should the acceptance criteria be for this feature", "what edge cases are we missing", "is this spec testable", "how do we write Given-When-Then", or "what failure behavior should we define". The goal is to prevent the agent from producing criteria that are vague quality claims or that cover only the path where everything succeeds.

## Core Rules

### Make Criteria Observable And Testable

Every acceptance criterion must describe behavior that someone can observe and verify, not a quality judgment. "Works well", "is intuitive", "loads fast", and "handles errors gracefully" are not criteria; they are aspirations that two people will interpret differently.

A testable criterion names a precondition, a trigger, and an observable result, ideally with a measurable threshold where useful. Compare "the export is fast" with "an export of up to 10,000 rows completes and begins downloading within 5 seconds at the 95th percentile". The second can be tested, the first can only be argued about. Replace every qualitative word with the concrete behavior or threshold it is meant to guarantee.

### Structure Scenarios With Given-When-Then

Given-When-Then gives each criterion a consistent shape that engineering and QA can map directly to tests. It forces the writer to separate the starting state from the action from the expected outcome, which is exactly where vague specs hide ambiguity.

The structure is:

- Given: the precondition, including data state, role, plan, device, or prior action.
- When: the user action or system event that triggers the behavior.
- Then: the observable result, including visible UI, data change, message, side effect, or error.

Write one scenario per distinct behavior, not one giant scenario that bundles many. The discipline of stating the Given exposes hidden assumptions about what state must already exist, which is where most unspoken edge cases live.

### Enumerate Edge Cases By Category

Edge cases are missed because teams think of them ad hoc instead of scanning systematically. Walk the feature through a fixed set of categories so that common failure states are checked deliberately rather than hoped for.

Useful categories:

- empty, zero, null, and missing values;
- boundary values: minimum, maximum, just below and just above limits;
- permissions and roles: who can view, edit, delete, or access;
- concurrency: simultaneous edits, race conditions, double submission;
- timeouts, retries, and partial failures;
- offline, low connectivity, and reconnection;
- large data volumes and pagination;
- migration of existing data and backward compatibility;
- rollback and feature-flag disablement;
- multi-tenant, plan, region, and account-state differences;
- localization, time zones, currencies, and date formats;
- accessibility: keyboard, screen reader, contrast, and focus behavior.

Treat this as a scanning checklist, not a literal list to copy. The point is to force each category to be answered: either it is in scope with defined behavior, or it is an explicit non-goal.

### Define Negative Paths And Error Recovery

Most specs define what happens when the user does the right thing and go silent when they do not. Negative paths are first-class behavior: invalid input, unauthorized access, conflicting state, cancelled actions, and system failures.

For each error specify:

- what triggers the error;
- what the user sees, in plain language they can act on;
- what happens to the data: is it saved, discarded, or held for retry;
- whether the user can recover without support, and how;
- what is logged or surfaced to support and operations.

"Show an error message" is not a criterion. "When upload fails due to size limit, the file is not stored, the user sees which limit was exceeded and the file's size, and the original form input is preserved for retry" is a criterion. Recovery behavior is product behavior, not an engineering detail to defer.

### Specify Data State Preconditions

Behavior depends on the state the system is in when the action occurs, and most ambiguity lives in unstated preconditions. A criterion that omits the starting data state cannot be tested reliably because QA cannot reproduce it.

Make preconditions explicit:

- what records exist and in what state;
- what role, plan, or permissions the user holds;
- whether the user has done a prior action in this session;
- what account, tenant, or workspace context applies;
- whether the feature is enabled by flag, plan, or region.

When the same action yields different results depending on state, write separate scenarios for each state rather than burying the condition in prose. This is what makes the spec survive contact with real data.

### Cover Boundary And Failure Behavior Explicitly

Boundaries are where systems break and where support tickets cluster. Define behavior at and beyond the edges of acceptable input and load, not only within the normal range.

Address:

- the exact threshold where behavior changes, such as a character, file size, or row limit;
- what happens at the boundary versus one step past it;
- behavior under peak load or slow response from a dependency;
- what is partial, queued, or rejected when capacity is exceeded;
- how the system degrades rather than failing silently.

Explicit boundary behavior prevents the situation where engineering and product each assume the other defined the limit, and neither did.

### Make Criteria Usable By QA, Design, And Launch Sign-Off

Acceptance criteria are not written only for engineering. QA turns them into test cases, design uses them to review states, and leadership uses them to decide whether the release is ready. Criteria that serve only one audience underdeliver.

For usability across functions:

- group criteria by user journey or feature area so they can be traced;
- distinguish must-pass criteria from nice-to-have within a release;
- ensure every user-facing state has a corresponding criterion;
- link criteria to instrumentation or metrics where behavior is measurable;
- make it obvious which criteria block launch and which are acceptable to defer.

When criteria are observable, categorized, and state-aware, sign-off becomes a comparison against a shared list rather than a debate about intent.

## Common Traps

### Vague Quality Claims Disguised As Criteria

Words like "intuitive", "smooth", "robust", or "user-friendly" cannot be tested and let each party assume a different standard. They feel rigorous but provide no contract.

### Happy-Path-Only Specifications

Criteria that cover only the successful flow leave errors, permissions, empty states, and recovery undefined. These gaps become bugs, support load, and launch-day surprises.

### Leaving Edge Cases To Engineering

Engineers will discover edge cases, but product owns the user-facing decision: what users see, what is allowed, and what happens on failure. Deferring all edge-case behavior to engineering abdicates the product role and produces inconsistent behavior.

### Unstated Preconditions

When the starting data state is assumed rather than written, QA cannot reproduce the scenario and engineers guess at context. The same action then behaves differently across implementations.

### Missing Boundary Definitions

If no one states the limit, each team picks its own. Users hit inconsistent ceilings, and the feature fails unpredictably at scale rather than degrading by design.

### Error Messages Without Recovery

Specifying that an error appears but not what the user can do about it leaves users stuck and drives support tickets. Recovery is part of the criterion, not an afterthought.

### Criteria That Cannot Drive Launch Sign-Off

When criteria are scattered, qualitative, or unranked, leadership cannot tell which must pass to launch. Sign-off then becomes opinion-based instead of evidence-based.

## Self-Check

- [ ] Every criterion is observable and testable, with qualitative words replaced by concrete behavior or thresholds.
- [ ] Scenarios use Given-When-Then and separate precondition, trigger, and expected result.
- [ ] Edge cases were scanned systematically across empty, boundary, permission, concurrency, timeout, offline, volume, migration, rollback, multi-tenant, localization, and accessibility categories.
- [ ] Negative paths and error recovery define what the user sees, what happens to data, and how they recover.
- [ ] Data state preconditions are explicit for each scenario, including role, plan, tenant, and prior action.
- [ ] Boundary behavior is defined at and just beyond the threshold, including degradation under load.
- [ ] Criteria are grouped and ranked so QA, design, and launch sign-off can trace and use them.
- [ ] Must-pass criteria are distinguished from deferrable ones, and launch-blocking conditions are explicit.
- [ ] No criterion relies on untestable words like "intuitive", "fast", or "graceful" without a defined threshold.
- [ ] The spec defines behavior across states, not only the path where everything succeeds.
