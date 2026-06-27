---
name: product_requirements.md
description: Use when the agent is writing, reviewing, or refining product requirements, PRDs, feature scope, acceptance criteria, user stories, edge cases, release boundaries, or implementation-ready product specifications.
---

# Product Requirements

Product requirements are not a long document for its own sake. They are a decision record that tells design, engineering, data, support, sales, operations, and leadership what problem is being solved, what behavior must exist, what tradeoffs were made, and how the team will know the work is acceptable.

Use this skill before writing or reviewing a PRD, feature brief, implementation ticket, user story set, acceptance criteria, launch scope, or product spec. The goal is to prevent the agent from producing requirements that describe happy-path UI but omit edge cases, permissions, data, rollout, instrumentation, support impact, and non-goals.

## Core Rules

### Start With Context And Decision

Requirements should explain why this work exists and what decision has already been made. A reader should understand the problem, target users, scope, and success criteria without reconstructing conversation history.

Include:

- problem statement;
- target user or customer segment;
- business or product objective;
- current workaround or failure;
- proposed solution direction;
- success metrics or expected outcomes;
- constraints and dependencies;
- decision owner and stakeholders where relevant.

Do not start directly with screens or fields unless the context is already documented elsewhere and linked.

### Define In-Scope And Out-Of-Scope

Clear scope is one of the most valuable parts of requirements. Teams waste time when implicit exclusions become late surprises.

Specify:

- user segments included and excluded;
- platforms included and excluded;
- workflows covered;
- data sources and integrations covered;
- roles and permissions covered;
- admin, support, and operational surfaces;
- localization, accessibility, compliance, and analytics expectations;
- what will deliberately remain manual or unchanged.

Out-of-scope items should be specific enough that a designer or engineer can make daily decisions without asking whether every adjacent idea belongs in the release.

### Describe Behavior, Not Just UI

A requirement should define system behavior across states, not only what the interface looks like.

Cover:

- entry points;
- user actions;
- system responses;
- validation;
- empty states;
- loading states;
- errors and recovery;
- permission denials;
- notifications;
- audit or history behavior;
- data persistence;
- background jobs;
- cancellation and rollback;
- multi-device or multi-session behavior.

If the spec only describes the happy path, it is not implementation-ready.

### Write Acceptance Criteria That Can Be Verified

Acceptance criteria should be observable. They should help engineering build, QA test, design review, support prepare, and product decide whether the release is ready.

Good criteria name:

- precondition;
- user action or system event;
- expected result;
- edge case or failure behavior;
- data or permission condition;
- measurable output where useful.

Avoid vague criteria like "works well", "is intuitive", "loads fast", or "handles errors". Replace them with concrete behavior or a testable threshold.

### Include Roles, Permissions, And Data Boundaries

Many product specs omit who can see or do what. This creates security bugs, support confusion, and inconsistent implementation.

For each feature, define:

- who can access it;
- who can create, edit, delete, approve, export, invite, or configure;
- what fields are visible to each role;
- whether behavior differs by plan, tenant, region, or account state;
- what happens when permissions change;
- whether actions are audited.

If permissions are inherited from an existing model, name that model and any exceptions.

### Account For Measurement And Learning

Requirements should say how the team will know whether the feature worked. Instrumentation cannot be an afterthought if the product manager expects to learn from usage.

Define:

- events to track;
- properties needed for analysis;
- funnels or cohorts;
- guardrail metrics;
- experiment or rollout plan;
- qualitative feedback channels;
- success review date.

Do not collect unnecessary personal data. Measurement should answer product questions without creating avoidable privacy or compliance risk.

### Plan Rollout, Migration, And Support

Product requirements should include launch mechanics when relevant. A feature may need beta access, migration, customer communication, admin controls, documentation, support macros, sales enablement, or operational monitoring.

Ask:

- Does this require data migration?
- Is rollout gradual or all at once?
- Can it be feature-flagged?
- What happens to existing users?
- How will support diagnose issues?
- How will the team roll back or disable it?
- What documentation or customer communication is needed?

The spec should not end at "engineers ship code".

## Common Traps

### Writing Requirements As UI Inventory

Listing pages, buttons, and fields is not enough. Requirements must describe behavior, data, permissions, states, and outcomes.

### Avoiding Tradeoffs

A PRD that refuses to say what is excluded forces teams to rediscover tradeoffs during implementation. Good requirements make tradeoffs visible.

### Mixing Problem Discovery With Final Scope

Discovery notes, brainstorms, and decisions are all useful, but implementation requirements should distinguish validated decisions from open questions.

### Leaving Edge Cases To Engineering

Engineers can help discover edge cases, but product should own user-facing behavior: what users see, what is allowed, what happens when something fails, and what support can explain.

### Forgetting Internal Users

Admin tools, support visibility, audit records, and operational workflows often matter for launch. If internal teams must handle the feature, requirements should include them.

### Defining Success As Launch

Shipping is an output. Requirements should connect to an outcome or learning goal.

## Self-Check

- [ ] The requirements include problem context, target users, objective, constraints, and decision state.
- [ ] In-scope and out-of-scope boundaries are explicit enough to guide daily implementation choices.
- [ ] Behavior is specified across entry points, actions, validation, empty, loading, error, permission, and recovery states.
- [ ] Acceptance criteria are observable and testable, not vague quality claims.
- [ ] Roles, permissions, data visibility, tenant boundaries, and audit needs are defined.
- [ ] Data persistence, background jobs, notifications, cancellation, rollback, and multi-session behavior were considered where relevant.
- [ ] Instrumentation, success metrics, guardrails, and learning plan are included.
- [ ] Rollout, migration, feature flags, support, documentation, and rollback expectations were reviewed.
- [ ] Open questions are separated from decisions and have owners or resolution paths.
- [ ] The spec avoids describing only happy-path UI while omitting product behavior.
