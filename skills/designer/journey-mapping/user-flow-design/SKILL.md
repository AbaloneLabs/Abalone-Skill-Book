---
name: user_flow_design.md
description: Use when the agent is designing user flows, task flows, navigation paths, step sequences, happy paths and alternate paths, entry and exit logic, branching decisions, or determining how a user moves from intent to completion across screens, and must decide flow scope, entry points, error branches, and where to cut a flow versus keep it linear.
---

# User Flow Design

A user flow is not a list of screens connected by arrows. It is a decision about which path a user must take to reach an outcome, which paths are optional, where the system must intervene, and where the user should be free to diverge. The judgment problem is deciding what belongs inside the flow, what is a side excursion, and what is a different flow entirely. Agents tend to fail by drawing a single linear happy path and treating every deviation as an exception, by over-constraining the user into one route when several legitimate intents exist, or by sprawling a flow so wide that it stops being a useful design tool and becomes an unreadable map of everything that could happen.

Use this skill before specifying how a user accomplishes a task end to end: checkout, onboarding, account recovery, content creation, multi-step forms, submission and review processes, or any goal-directed journey. The goal is to produce a flow that captures the real decision points without collapsing real choice or drowning the team in every possible edge.

## Core Rules

### Decide The Flow's Purpose And Boundary Before Drawing

A flow must have a defined start, a defined success, and a clear scope. Before connecting any boxes, state what single user goal this flow serves and where it begins and ends. A checkout flow begins when the user commits to purchase and ends at confirmation; a recovery flow begins at "forgot password" and ends at regained access. Flows that try to cover adjacent goals become unreadable because no one can tell which arrows are the main path and which are side trips.

Establish the boundary by answering:

- what is the user's primary goal at the start of this flow;
- what counts as successful completion;
- what adjacent tasks are explicitly out of scope and belong to a different flow;
- where does the user enter from, and where can they exit to.

If you cannot name the goal and the completion state in one sentence each, the flow is not yet scoped.

### Map The Happy Path First, Then Earn Every Branch

Begin with the single most common path from entry to success, assuming nothing goes wrong and the user wants the primary outcome. This is the spine. Every additional branch, decision, error, and alternate route must earn its place by representing a real, likely user situation, not a theoretical possibility.

For each branch, ask:

- how often does this path occur in real use;
- does the user actively choose it, or does the system force it;
- is it a recovery from a problem, or a legitimate alternate goal;
- does it return to the main flow or end somewhere different.

Flows that enumerate every conceivable edge become unreadable. Flows that include only the happy path miss the situations that cause support load and abandonment. The discipline is including the branches that matter and excluding the ones that do not.

### Distinguish User Decisions From System Decisions

A common confusion is mixing points where the user chooses with points where the system routes. These are different and must be drawn distinctly. A user decision is a fork the user controls: pay by card or wallet, save or discard, continue or cancel. A system decision is a fork the system controls based on state: account exists or not, payment succeeded or failed, permission granted or denied.

Separate them because they imply different design work:

- user decisions need clear options, defaults, and an obvious way to choose;
- system decisions need clear feedback, recovery paths, and no dead ends;
- system decisions that depend on external state need explicit failure handling.

Conflating the two hides where the user actually has agency and where the system can trap them.

### Preserve The User's Ability To Leave And Return

Linear flows drawn as forced marches assume the user will complete the task in one sitting. Real users interrupt, switch devices, get distracted, and come back later. A flow that loses all progress on exit, or that forces the user to restart from the beginning, creates frustration and abandonment.

For any non-trivial flow, decide:

- is progress saved automatically or on explicit action;
- can the user leave and resume at the same step;
- can the user back up to a previous step without losing data;
- what happens to in-flight state if the session times out;
- is there a clear cancel or exit that does not destroy work.

Flows that treat every exit as abandonment punish the user's real behavior.

### Design Entry Points For Multiple Contexts

Users do not always enter a flow from the front door. They arrive from deep links, notifications, search results, emails, other screens, or returning sessions. A flow designed only for the canonical entry assumes a context the user may not have.

Identify the plausible entry points and decide for each:

- can the user start mid-flow with enough context, or must they begin at the start;
- what state must be present to allow a deep entry;
- what happens if required prerequisites are missing;
- does a deep link land the user at the right step with the right data.

An onboarding flow reached from a referral link, a recovery flow reached from an email, and a checkout flow reached from a saved cart all need entry logic, not just a single start screen.

### Make Failure And Recovery A First-Class Part Of The Flow

Errors are not footnotes to a flow; they are part of it. A flow that shows only success paths hides the moments where users get stuck, retry, abandon, or contact support. For each system decision and each external dependency, map what happens on failure: validation errors, network failures, timeouts, permission denials, conflicting state, and abandoned attempts.

For each failure point, decide:

- does the user stay on the same step or move to a recovery screen;
- can the user retry in place, or must they restart;
- is partial progress preserved;
- is the error message actionable, telling the user what to do next;
- is there a graceful fallback when the primary path is impossible.

A flow with no failure branches is incomplete, regardless of how clean the happy path looks.

### Decide Granularity By Audience And Use

The same task can be drawn at screen level, step level, or decision level depending on who reads the flow. A flow for engineers implementing the routing differs from a flow for stakeholders validating the journey differs from a flow for researchers testing comprehension. Decide the granularity before drawing, and do not mix levels within one diagram.

Common levels:

- journey level: phases and emotional or contextual stages, light on screens;
- task level: named steps and decisions, the typical design artifact;
- screen level: specific pages and transitions, close to implementation.

Mixing levels produces diagrams that are too detailed for stakeholders and too abstract for builders.

## Common Traps

### Drawing Only The Happy Path

A flow with a single straight line from start to finish looks clean but hides every place users actually get stuck. Errors, alternates, and interruptions are where most abandonment happens.

### Sprawling Every Possible Edge

The opposite failure is including every conceivable branch until the diagram is unreadable. A flow that tries to map everything maps nothing useful. Include branches that occur in real use, not every theoretical fork.

### Forcing A Single Linear Route

When users have several legitimate intents or orders of operation, forcing one sequence adds friction. Allow reordering, skipping, and alternate routes where the task permits.

### Losing Progress On Exit

Flows that restart from the beginning after an interruption punish users who leave for any reason. Preserve progress and allow resumption for any non-trivial task.

### Conflating User And System Decisions

Drawing system routing as if it were a user choice hides where the user has no agency. Keep decision types visually and logically distinct.

### Ignoring Deep Entry

Designing only the front-door entry assumes users always arrive at the start. Deep links, notifications, and returning sessions need their own entry logic.

### Dead Ends On Failure

A failure branch that leads nowhere, with no retry, recovery, or guidance, turns a recoverable error into an abandonment. Every failure path must lead somewhere actionable.

### Mixing Flow Granularity

A diagram that jumps between journey phases and individual screen transitions confuses every audience. Pick one level of detail and hold it.

## Self-Check

- [ ] The flow has a named single goal, a defined start, and a defined success state, with adjacent tasks explicitly excluded.
- [ ] The happy path is drawn first, and every additional branch is justified by how often and why it occurs in real use.
- [ ] User decisions and system decisions are drawn distinctly, so agency and routing are not confused.
- [ ] The user can leave, return, back up, and cancel without losing all progress for any non-trivial flow.
- [ ] All plausible entry points, including deep links, notifications, and returning sessions, have entry logic and required state defined.
- [ ] Every system decision and external dependency has a mapped failure path with retry, recovery, or graceful fallback.
- [ ] The granularity is consistent throughout and matches the intended audience, without mixing journey, task, and screen levels.
- [ ] No branch leads to a dead end; every path, including failures, ends at an actionable next step.
- [ ] The flow reflects real user behavior and likely situations rather than only the ideal sequential path.
