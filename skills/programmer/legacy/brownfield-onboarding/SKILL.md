---
name: brownfield_onboarding.md
description: Use when the agent is joining or helping someone join an existing (brownfield) codebase, trying to understand legacy code that lacks documentation, discovering implicit knowledge and unwritten conventions held by maintainers, identifying the riskiest areas to change first, building a safety net before modifying unfamiliar code, or planning incremental improvement without a rewrite. Also covers the failure mode of changing code one does not understand, trusting comments over behavior, missing tribal knowledge that only senior maintainers hold, and treating a legacy system as a greenfield project.
---

# Brownfield Onboarding

Brownfield onboarding is the work of becoming effective in a codebase that already exists, that someone else wrote, and that carries years of decisions, workarounds, and implicit knowledge that no document records. The judgment problem is that the code on the page is not the whole system: the comments are often stale, the names are sometimes misleading, and the most important constraints — why a seemingly wrong approach was taken, which module is fragile and why, which "obvious" refactor has been tried and reverted — live in the heads of maintainers and in the history of past incidents. A new contributor who reads the code and starts fixing what looks wrong will, with high probability, reintroduce a bug that was already fixed, break an implicit invariant, or touch a module that everyone knows is load-bearing and dangerous. The discipline is to treat understanding as a deliberate first deliverable, to mine the implicit knowledge before changing anything, to identify the riskiest areas and avoid them until a safety net exists, and to improve incrementally rather than to rewrite.

Agents tend to under-invest in understanding because the pressure to ship returns quickly and the code looks comprehensible at a glance. The harm appears as regressions, as broken invariants, and as the slow realization that the "obvious" change was obvious to the last three people who tried it and who all reverted it for the same undocumented reason. The judgment is to read the code alongside its history (blame, past incidents, reverted PRs), to talk to the maintainers who hold the tribal knowledge, to characterize the current behavior with tests before changing it, and to start in the lowest-risk areas where mistakes are cheap. Onboarding is not finished when you can navigate the code; it is finished when you know where the bodies are buried and which doors not to open.

## Core Rules

### Treat Understanding As The First Deliverable

In a brownfield system, understanding is not a warm-up; it is the work that prevents every subsequent mistake. Budget time for it explicitly, and resist the pressure to start changing things before you can explain the system's behavior and its constraints.

- **Map the system before changing it.** Understand the major components, their boundaries, the data flow, and the critical paths. A diagram you draw yourself (and verify against reality) is worth more than one you inherit.
- **Read the code alongside its history.** `git blame`, the commit messages, the linked issues, and the reverted PRs explain why the code is the way it is. The "why" is rarely in the comments; it is in the history of decisions and incidents.
- **Distinguish what the code does from what it should do.** Legacy code often behaves differently from its stated intent; verify behavior by reading and running it, not by trusting names or comments.

### Mine The Implicit Knowledge Before Changing Anything

The most important constraints in a legacy system are usually unwritten: which module is fragile, which "obvious" refactor has been tried and reverted, which feature is actually deprecated but still called, which external system has quirks the code works around. This knowledge lives in maintainers' heads and in tribal memory, and mining it before changing code prevents reintroducing solved problems.

- **Talk to the maintainers who have been there longest.** Ask not just "how does this work" but "where are the traps," "what have people tried and reverted," and "what would you never touch without a lot of testing." Thirty minutes with a senior maintainer can save weeks of rediscovering a known trap.
- **Read the incident history and the postmortems.** Past incidents reveal the load-bearing parts and the failure modes; a module that has caused several incidents is fragile and deserves extra care.
- **Capture the implicit knowledge as you find it.** When a maintainer explains a trap, write it down (a comment, a runbook, an ADR) so the next person does not have to rediscover it by breaking something.

### Identify The Riskiest Areas And Avoid Them Until A Safety Net Exists

Not all code is equally dangerous to change. Some modules are well-tested and forgiving; others are load-bearing, fragile, and undocumented. The judgment is to identify the risky areas early and to avoid changing them until tests characterize the current behavior, so that a mistake is caught rather than shipped.

- **Map risk by test coverage, change frequency, and incident history.** High-churn, low-coverage, incident-prone modules are the riskiest; start elsewhere.
- **Do not refactor risky code without a characterization net.** Before changing a fragile module, write tests that capture its current behavior (even the buggy parts) so a regression is detectable. Changing fragile code without a net is how regressions ship.
- **Prefer additive changes in risky areas.** Adding a new path alongside the old one is safer than modifying the old one in place; the old path keeps working while the new one is validated.

### Build A Safety Net Before Modifying

A safety net is the set of tests, checks, and observability that catches mistakes before they reach users. In legacy code, the net is often thin or absent, so the first task before meaningful change is to build it: characterization tests for the behavior you intend to preserve, and observability for the paths you intend to touch.

- **Write characterization tests before refactoring.** These tests assert the current behavior (including its quirks) so that a change which breaks an invariant is caught. They are not about whether the behavior is correct; they are about whether it changed.
- **Add observability before changing critical paths.** You cannot safely change what you cannot see; logging, metrics, and tracing on the path you intend to modify let you detect regressions early.
- **Grow the net incrementally, focused on the areas you will change.** Attempting to test the whole legacy system at once is rarely feasible; target the net at the riskiest and soonest-changed areas.

### Improve Incrementally Rather Than Rewrite

The temptation in legacy code is the grand rewrite — to replace the messy system with a clean one. Grand rewrites almost always underestimate the implicit behavior the old system has accumulated (the edge cases, the workarounds, the compat) and they run long, parallel, and risky. Incremental improvement — the strangler pattern, adding tests, extracting modules, replacing one piece at a time — delivers value continuously and keeps the system working throughout.

- **Prefer the strangler pattern to the big rewrite.** Build the new path alongside the old, route traffic incrementally, and retire the old once the new is proven, rather than freezing the world for a replacement.
- **Make each change small, reversible, and independently valuable.** A change you can revert in minutes is a change you can safely make; a change that requires a flag-day cutover is a change that can sink a quarter.
- **Let value, not aesthetics, drive the change.** Improve the parts that block real work (the bug-prone module, the slow query, the untestable component) rather than the parts that are merely ugly.

### Respect Scope and Escalation Boundaries

Know where the agent's authority and competence end. When the question requires a license, a specialist's judgment, a final approval, or expertise the agent does not hold, the correct action is to escalate rather than to produce a confident answer that overreaches. Scope discipline protects the recipient from harm caused by an unqualified conclusion and protects the agent from liability. State explicitly when the output is advisory and must be confirmed by the qualified person.

## Common Traps

### Changing Code You Do Not Understand

Modifying a module based on a surface reading, reintroducing a bug that was already fixed or breaking an implicit invariant. Treat understanding as the first deliverable; read the code alongside its history and mine the implicit knowledge before changing.

### Trusting Comments Over Behavior

Acting on what the comments or names say the code does, when the actual behavior differs. Verify behavior by reading and running the code; treat comments and names as hints, not contracts.

### Missing The Tribal Knowledge

Skipping the conversation with senior maintainers and rediscovering known traps by breaking them. Thirty minutes with a long-tenured maintainer can prevent weeks of regression; capture what you learn so the next person does not repeat the discovery.

### Touching The Riskiest Area Without A Safety Net

Refactoring a load-bearing, fragile, low-coverage module without first writing characterization tests, so a regression ships undetected. Build the net before modifying; prefer additive changes in risky areas.

### The Grand Rewrite

Replacing the messy system wholesale, underestimating the implicit behavior the old system has accumulated, running long and risky while the old system keeps needing fixes. Prefer incremental improvement (strangler pattern, small reversible changes) over the big rewrite.

### Driven By Aesthetics Over Value

Refactoring code because it is ugly rather than because it blocks real work, spending effort on cosmetic improvement while the bug-prone and slow parts remain. Let value drive the change.

### Onboarding Considered Done At Navigation

Treating onboarding as complete when you can find your way around, before you know which areas are dangerous and which doors not to open. Onboarding is done when you know where the bodies are buried.

## Self-Check

- [ ] Understanding is treated as the first deliverable: the major components, boundaries, data flow, and critical paths are mapped (and verified against reality), and the code is read alongside its history (blame, commits, incidents, reverted PRs) to learn the "why."
- [ ] Implicit/tribal knowledge is mined before changing: long-tenured maintainers are asked about traps, reverted attempts, and fragile areas; incident history is reviewed; and discovered knowledge is captured so the next person does not rediscover it by breaking something.
- [ ] Behavior is verified by reading and running the code, not by trusting comments or names; the distinction between what the code does and what it should do is explicit.
- [ ] The riskiest areas (high-churn, low-coverage, incident-prone, load-bearing) are identified early and avoided until a safety net exists; risky changes are additive rather than in-place.
- [ ] A safety net is built before modifying: characterization tests capture current behavior (including quirks), observability is added to paths about to change, and the net is grown incrementally focused on soonest-changed areas.
- [ ] Improvement is incremental (strangler pattern, small reversible independently-valuable changes) rather than a grand rewrite, and is driven by value (what blocks real work) rather than aesthetics.
- [ ] The highest-risk cases were verified — changing a module whose history was understood, modifying fragile code only after characterization tests existed, capturing tribal knowledge before the maintainer who holds it leaves, and resisting a rewrite that underestimated accumulated behavior — not only the clean well-documented path.
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
