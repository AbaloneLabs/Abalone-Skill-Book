---
name: release_risk_and_readiness.md
description: Use when the agent is assessing whether a release is ready to ship, evaluating release risk, designing rollout and rollback strategy, defining release readiness criteria, or deciding whether to proceed with or hold a release based on quality and risk signals.
---

# Release Risk And Readiness

Release readiness is the judgment about whether a release is safe and valuable to ship, given the quality evidence, the risk profile, and the ability to respond if something goes wrong. It is not a checkbox exercise; it is a decision under uncertainty that weighs the value of shipping now against the risk of shipping problems. Done well, readiness assessment provides clear go or no-go decisions based on defined criteria, with a rollout and rollback plan that contains risk. Done poorly, it either ships recklessly on schedule pressure, producing incidents and erosion of trust, or holds releases indefinitely chasing impossible certainty, delaying value and demoralizing the team. Agents often treat readiness as a sign-off ritual rather than as a risk decision, or default to shipping because the date has arrived.

The harm this skill prevents is the release that harms the product. Shipping a release with unresolved quality issues, untested edge cases, or no rollback plan creates incidents that damage user trust, generate support burden, and consume the team's capacity in firefighting. The opposite harm, never shipping because risk is never zero, forfeits the value and learning the release would provide. Readiness is the discipline of shipping when the value exceeds the residual risk, with a plan to contain what goes wrong.

Use this skill before answering questions such as "is this release ready to ship", "what are the risks of this release", "how should we roll this out", or "should we hold or proceed". The goal is to prevent the agent from treating release as a date-driven event rather than a risk-managed decision.

## Core Rules

### Define Readiness Criteria Before The Release Is Built

Readiness criteria must be defined before development begins, not negotiated at the end when schedule pressure makes objectivity impossible. These criteria specify what must be true for the release to ship: which acceptance criteria must pass, what performance and reliability thresholds must hold, what edge cases must be handled, what instrumentation must be in place, and what known issues are acceptable. Defining these upfront creates an objective standard that resists the temptation to lower the bar when the date approaches.

Readiness criteria should cover functional quality, non-functional quality, observability, support readiness, and communication. A release that functions but cannot be monitored, supported, or rolled back is not ready, even if the features work. Build the criteria comprehensively, so that readiness is assessed on all dimensions that determine whether shipping is safe, not only on whether the code passes tests.

### Assess Risk By Severity And Likelihood, Not By Gut

Release risk should be assessed systematically, identifying what could go wrong, how likely it is, and how severe the impact would be. Risks include functional regressions, performance degradation, data loss or corruption, security vulnerabilities, integration failures, and user-facing confusion. For each risk, estimate likelihood and severity, and prioritize mitigation by the product of the two. A low-likelihood, high-severity risk like data loss deserves as much attention as a high-likelihood, medium-severity one.

Ground risk assessment in evidence rather than feeling. What do the test results show, what did exploratory testing find, what do the performance benchmarks indicate, what edge cases remain untested? A risk assessment based on "it probably works" is not an assessment. Evidence-based risk assessment identifies the specific areas of concern and the confidence level behind each, so that the go or no-go decision rests on something verifiable.

### Design Rollout To Contain Risk Through Staged Exposure

The most powerful risk-management tool is staged rollout: exposing the release to a small subset of users first, monitoring for problems, and expanding exposure only as confidence accumulates. Staged rollout converts a binary, all-or-nothing release into a graduated one, where problems are caught affecting few users before they affect many. Design the rollout stages, the percentage or segment exposed at each stage, the metrics monitored at each stage, and the criteria for proceeding to the next stage.

Staged rollout requires the infrastructure to support it: feature flags, percentage rollouts, segment targeting, and the monitoring to detect problems within each stage. Without this infrastructure, rollout is binary and risk is uncontained. Investing in rollout infrastructure is what enables the team to ship with confidence even when residual risk exists, because the blast radius of any problem is limited.

### Ensure Rollback Is Possible And Practiced

Every release should be reversible, and the ability to reverse it should be verified, not assumed. Rollback is the safety net that makes shipping acceptable even when residual risk exists: if a problem emerges, the release can be reversed, limiting the damage. A release without a viable rollback is a one-way door, where any problem persists until a forward fix ships, which may take hours or days. Ensure that rollback is technically possible, that the process is documented, and ideally that it has been practiced, so that it works under the pressure of a real incident.

Be honest about the limits of rollback. Some changes are hard to reverse: data migrations that transform data, schema changes that break old versions, deprecations that other systems depend on. For these, rollback may be partial or impossible, and the risk assessment must account for the irreversibility. A release that includes irreversible changes demands a higher readiness bar, because the safety net is absent.

### Separate The Go Or No-Go Decision From Schedule Pressure

The decision to ship or hold must be made on readiness and risk, not on the fact that the date has arrived or that stakeholders expect delivery. Schedule pressure is real and legitimate, but it cannot override a genuine readiness problem, because shipping an unready release creates incidents that cost more than the delay would have. Structure the go or no-go decision so that it is based on the readiness criteria, with schedule pressure acknowledged but not decisive.

This requires organizational support: leadership must back the team's authority to hold a release that fails readiness criteria, or the criteria become theater. When holding a release is treated as failure, teams ship unready releases to avoid the appearance of failure, and the resulting incidents are worse. A culture that treats a principled hold as responsible, not as failure, is what makes honest readiness assessment possible.

### Plan For The First Hours And Days After Release

Release readiness extends beyond the moment of deployment into the period immediately after, when problems are most likely to emerge. Plan for this period: who monitors, what metrics are watched, what thresholds trigger investigation or rollback, who is on call, and how users are communicated with. A release that ships and is then unmonitored discovers problems through user complaints, which is the slowest and most damaging detection path. A release that is actively monitored discovers problems through signals, enabling fast response.

The intensity of post-release monitoring should match the risk. A low-risk, easily-rolled-back release needs light monitoring; a high-risk, hard-to-reverse release needs intense monitoring with the team ready to respond. Match the monitoring plan to the risk profile, and stand down the heightened monitoring only when the release has proven stable in production.

## Common Traps

### Date-Driven Shipping

Shipping because the date arrived regardless of readiness. The trap is incidents that cost more than the delay would have.

### Readiness As Sign-Off Theater

Criteria negotiated away under schedule pressure. The trap is the appearance of readiness assessment without its substance.

### Binary Rollout Without Staging

Exposing all users at once. The trap is problems affecting everyone before they are detected.

### Assumed But Unverified Rollback

Believing rollback works without testing it. The trap is discovering rollback fails during a real incident, when the damage is already done.

### Ignoring Irreversible Changes

Treating a release with data migrations as reversible. The trap is a one-way door where problems cannot be contained.

### No Post-Release Monitoring Plan

Shipping and moving on without watching. The trap is problems detected through user complaints rather than signals, slowly and expensively.

## Self-Check

- [ ] Readiness criteria were defined before development and cover functional, non-functional, observability, support, and communication dimensions.
- [ ] Release risk is assessed by severity and likelihood, grounded in evidence from testing and benchmarks.
- [ ] Rollout is staged, with defined exposure stages, monitored metrics, and progression criteria.
- [ ] Rollback is technically possible, documented, and ideally practiced, and its limits are honestly acknowledged.
- [ ] The go or no-go decision is based on readiness criteria, with schedule pressure acknowledged but not decisive.
- [ ] A post-release monitoring plan matches the risk profile, with defined responders and rollback triggers.
- [ ] Irreversible changes are identified and held to a higher readiness bar.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
