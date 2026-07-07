---
name: launch_risk_and_contingency_planning.md
description: Use when the agent is identifying launch risks, building contingency plans for a release, preparing rollback and recovery strategies, planning for launch failures and degraded scenarios, or deciding go/no-go and readiness gates for a product launch.
---

# Launch Risk And Contingency Planning

A launch is a moment of concentrated risk. Everything that was built, integrated, tested, and prepared converges on a single event, and the systems, teams, and customers that must cooperate are all stressed at once. The judgment problem is that launches fail not usually from a single catastrophic bug but from the compounding of small unpreparednesses: a capacity assumption that was never load-tested, a support team that learned of a known issue from a customer, a rollback path that was never rehearsed, a dependency that slipped silently. Contingency planning is the discipline of imagining how the launch breaks before it breaks, so that when something goes wrong the response is rehearsed rather than improvised under pressure. Teams that skip this work do not avoid risk; they simply meet it unprepared.

Use this skill before finalizing a launch plan, before defining go/no-go criteria, before building rollback and recovery strategies, or before a launch readiness review. The goal is to prevent the agent from treating launch as a single irreversible event with no fallback, from defining readiness criteria that are aspirational rather than operational, or from assuming that because testing passed, the launch cannot fail.

## Core Rules

### Identify Risks Across Technical, Operational, And Adoption Dimensions

Launch risk is not only the risk of the software breaking. It spans technical failure, operational unpreparedness, and adoption shortfall, and each requires different mitigation.

Identify risks across:

- technical: performance under load, integration failures, data migration errors, security exposure;
- operational: support unprepared, sales misinformed, success overwhelmed, documentation missing;
- adoption: customers do not understand the change, value is unclear, migration is painful;
- dependency: a partner, vendor, or platform change is late or broken;
- communication: announcement outruns readiness or misleads about availability;
- rollback: the change cannot be reversed cleanly if it fails.

A risk register that covers only bugs misses the majority of real launch failures.

### Define Concrete Go/No-Go Criteria In Advance

The decision to launch should be made against pre-defined, measurable criteria, not against enthusiasm as the date approaches. Criteria defined in advance resist the pressure to launch because the date has arrived.

Define criteria for:

- technical readiness: test coverage, performance benchmarks, security review complete;
- operational readiness: support trained, docs published, sales enabled;
- data readiness: migrations validated, instrumentation verified;
- dependency readiness: partners and integrations confirmed;
- rollback readiness: a tested reversal path exists;
- risk acceptance: known issues documented and accepted by owners.

Each criterion should have an owner, evidence, and a clear pass condition. Criteria that cannot be evidenced are not criteria.

### Plan And Rehearse The Rollback Path

Many launches cannot be cleanly reversed, and not knowing this until failure is catastrophic. The rollback path must be designed, tested, and understood before launch, not improvised during an incident.

For rollback, determine:

- whether the change is reversible, partially reversible, or irreversible;
- the technical mechanism and its blast radius;
- the data implications, can migrated data be unwound;
- the time required and who executes it;
- the decision authority and trigger for invoking rollback;
- the customer communication if rollback occurs.

If a launch is irreversible, the risk acceptance must be explicit and the pre-launch confidence proportionally higher.

### Build Degraded-State And Fallback Scenarios

Between perfect launch and total failure lies a range of degraded states: slow performance, partial feature failure, elevated error rates, a subset of customers affected. Planning only for success or total failure leaves the team unprepared for the common middle.

Plan for degraded states by:

- defining acceptable degraded behavior and how it is triggered;
- building feature flags or kill switches to disable problematic components;
- preparing fallback experiences or manual processes;
- setting thresholds that trigger each fallback;
- assigning owners and communication for each degraded scenario.

A launch that can degrade gracefully survives incidents that would otherwise become crises.

### Load-Test And Stress-Test Realistic Scenarios

Performance assumptions that are never tested become launch failures. Load-testing must reflect realistic, not optimistic, conditions.

Test:

- expected launch-day traffic, including announcement-driven spikes;
- peak concurrent usage patterns;
- failure cascades, what happens when a dependency slows or fails;
- data volume at scale, not only test data;
- geographic and integration edge cases.

A system that performs in test may fail under real launch load. Test the real scenario.

### Prepare The Incident Response Before Launch

When something goes wrong at launch, the incident response must be ready, not assembled in real time. The team, process, and communication should be staged before go-live.

Prepare:

- an incident channel and on-call rotation for the launch window;
- a communication tree for internal and customer updates;
- pre-drafted customer communications for common failure modes;
- escalation paths to engineering, leadership, and PR;
- a war-room or monitoring setup for the launch window.

### Stage The Rollout To Limit Blast Radius

A big-bang launch maximizes blast radius. Staged rollout limits exposure and creates checkpoints to catch problems before they affect everyone.

Stage by:

- launching to a small cohort or internal users first;
- expanding in defined increments with checkpoints;
- using feature flags to control exposure;
- defining the metrics and thresholds that gate each expansion;
- preserving the ability to pause or reverse between stages.

Staged rollout converts launch from a binary event into a controlled progression.

### Define The Launch Window's Decision Authority

During the launch window, decisions must be made quickly: continue, pause, rollback, communicate. Ambiguity about who decides produces delay and conflict under pressure.

Define in advance:

- who has authority to call rollback or pause;
- who communicates to customers and when;
- the escalation path for ambiguous situations;
- the criteria that trigger each decision;
- the post-launch review to capture learnings.

### Capture Known Issues And Set Expectations

No launch is flawless. Known issues that are documented, communicated, and owned are far less damaging than issues that surface as surprises. Set expectations honestly.

Document and communicate:

- known limitations and their workarounds;
- features deferred to a follow-up;
- segments or scenarios not yet supported;
- the timeline for addressing known gaps.

## Common Traps

### Bug-Only Risk Thinking

Identifying only technical bugs and missing operational, adoption, dependency, and rollback risks.

### Aspirational Go/No-Go

Criteria that cannot be evidenced or are waived under date pressure, launching unprepared.

### Untested Rollback

Assuming reversal is possible without designing or rehearsing it, discovering irreversibility during failure.

### Binary Launch Thinking

Planning only for success or total failure, unprepared for the common degraded middle.

### Unload-Tested Assumptions

Performance beliefs never validated against realistic launch-day load.

### Improvised Incident Response

Assembling the response during the crisis rather than staging it before launch.

### Big-Bang Blast Radius

Maximizing exposure by launching to everyone at once instead of staging.

### Surprise Known Issues

Allowing documented limitations to surface as customer-discovered surprises.

## Self-Check

- [ ] Risks are identified across technical, operational, adoption, dependency, communication, and rollback dimensions.
- [ ] Go/no-go criteria are concrete, measurable, evidenced, and owned, defined before launch pressure.
- [ ] The rollback path is designed, tested, and understood, with irreversibility explicitly accepted where it exists.
- [ ] Degraded-state and fallback scenarios are planned with triggers, owners, and communications.
- [ ] Load and stress testing reflect realistic launch-day conditions, including spikes and dependency failures.
- [ ] Incident response, channel, on-call, communications, escalation, is staged before the launch window.
- [ ] Rollout is staged to limit blast radius, with checkpoints and metrics gating each expansion.
- [ ] Decision authority for rollback, pause, and communication is defined before the launch window.
- [ ] Known issues are documented, communicated, and owned rather than surfacing as surprises.
- [ ] Pre-launch confidence is proportionate to irreversibility, with higher bars for irreversible changes.
