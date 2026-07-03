---
name: rollout_strategy_and_traffic_management.md
description: Use when the agent is choosing a rollout strategy, deciding between phased and global release, managing traffic exposure and migration, or planning how to expose a new feature to users safely over time.
---

# Rollout Strategy And Traffic Management

A rollout strategy is the plan for how a finished feature reaches users over time. It is distinct from launch readiness, which asks whether the work is complete and the teams are prepared. Rollout asks a different question: given that we are ready, how do we expose this to real traffic so that we learn fast, contain risk, and can recover if we are wrong.

Agents miss this because "deploy" and "release" feel like the same event. They ship to production, turn the feature on for everyone, and move on. The harm is that a defect or a migration problem hits the entire user base at once, an irreversible change cannot be undone without data loss, and a bad first impression reaches every customer simultaneously. The opposite failure is rolling out so cautiously that learning takes months and the team loses the will to iterate.

Use this skill before answering broad questions such as "should we beta this", "how do we roll out gradually", "what percentage should we start at", "how do we handle the migration", or "is a big-bang launch okay here". The goal is to prevent the agent from conflating deployment with release and from choosing a rollout pattern without weighing risk, reversibility, and the learning the team actually needs.

## Core Rules

### Separate Deployment From Release

Deployment is code reaching an environment. Release is users experiencing the feature. Decoupling the two through feature flags, config, or server-side control is the foundation of safe rollout, because it lets the team deploy continuously while releasing deliberately.

Ask:

- Can the feature be turned on or off without a redeploy?
- Who controls exposure, and can it be changed quickly under pressure?
- Is the flag short-lived with a planned removal, or permanent infrastructure?

If release requires a deploy and a deploy requires a release window, every rollout becomes a high-stakes all-or-nothing event.

### Choose A Rollout Pattern By Risk, Reversibility, And Learning Need

There is no universally correct rollout. The right pattern depends on what you are trying to learn, how reversible the change is, and how bad a failure would be.

Common patterns, in increasing exposure:

- internal dogfooding: employees only, catches obvious breakage and UX confusion;
- private beta: invited customers under NDA or close relationship, gathers qualitative signal and validates value;
- public beta: opt-in, self-selected users who tolerate rough edges, surfaces scale and edge-case issues;
- percentage ramp: expose a growing slice of traffic, monitor, expand;
- region or segment gated: release to one geography or customer segment first, contain blast radius;
- ring deployment: concentric rings of users or infrastructure, expanding outward;
- big-bang: everyone, immediately.

Reversible, low-risk changes can move quickly through the rings. Irreversible changes such as data migrations, schema changes, or deletions demand the slowest, most gated path the team can tolerate.

### Handle Migration And Backward Compatibility For Breaking Changes

Breaking changes are where rollout strategy earns its cost. A migration that works for new data but corrupts old data, or an API that drops a field existing clients depend on, will fail in production no matter how good the feature is.

For breaking changes, plan:

- backward compatibility windows where old and new both work;
- dual-write or dual-read patterns during transition;
- migration of existing data before, during, or after exposure, with verification;
- a defined end-of-life for the old path;
- communication to integrators and API consumers with enough lead time.

The rollout plan and the migration plan are one document for breaking changes. Treat the migration as a first-class risk, not an engineering detail.

### Define Canary Monitoring And Auto-Rollback Triggers

Gradual rollout only reduces risk if something watches the traffic and acts on it. A canary release that nobody monitors is just a slow big-bang launch.

Define in advance:

- which metrics define health, including error rate, latency, and business guardrails;
- what thresholds trigger a pause versus an automatic rollback;
- how long to hold at each exposure level before expanding;
- who is watching during the ramp and what their authority is.

Auto-rollback is powerful but must be tuned to avoid flapping on noise. Manual gates add judgment but add latency. Most rollouts use both: automatic on clear failure, manual on ambiguous signal.

### Preserve Experiment Integrity During Rollout

Rollout and experimentation interact in ways teams often overlook. If a feature is on for a growing percentage of users, and that percentage is not also a clean experiment assignment, the team cannot tell whether observed changes come from the feature or from selection bias.

Consider:

- Is the rollout population a valid experiment cohort, or is it skewed by opt-in, region, or account size?
- Are guardrail metrics defined so that a rollout does not silently regress an existing metric?
- If running an A/B test alongside a ramp, are the two exposures compatible and not double-counting?
- Does gradual exposure delay the experiment read, and is that acceptable?

A rollout that confounds your measurement turns a learning opportunity into a guess.

### Communicate With Users During Gradual Exposure

Gradual rollout creates a communication problem: some users have the feature and some do not. If this is handled badly, support gets tickets from users who feel singled out, and customers compare notes and lose trust.

Decide:

- Is the feature announced broadly or only to those who have it?
- How are beta or early-access expectations set so users tolerate rough edges?
- What does support say to users who ask why they do not have it yet?
- Is there a known-issues channel for the early cohort?

Silent gradual rollout is fine for invisible improvements and risky for anything a user would notice and ask about.

### Know When Big-Bang Is Acceptable, And Why It Usually Is Not

Big-bang release is acceptable when the change is low-risk, fully reversible, thoroughly tested, and the cost of gradual exposure exceeds the cost of failure. Examples include a copy change behind a flag, a purely additive non-breaking endpoint, or a fix to a defect affecting everyone equally.

Big-bang is usually wrong for anything customer-visible and strategic, anything involving a migration, or anything whose failure damages trust on first impression. The appeal of big-bang is simplicity and speed; its cost is maximum blast radius and no learning before full exposure.

## Common Traps

### Equating Deploy With Release

Shipping to production and turning a feature on for everyone are treated as one step, eliminating every safety benefit of gradual exposure and making rollback a redeploy problem.

### Choosing The Pattern By Habit Instead Of Risk

Defaulting to "we always beta" or "we always ramp to 5 then 50 then 100" regardless of reversibility and learning need produces either unsafe launches for risky changes or wasteful ceremony for safe ones.

### Ignoring The Migration Until It Breaks

Teams plan the feature rollout carefully and treat data migration as an engineering afterthought, then discover at 100% that old accounts are corrupted or an integration has broken.

### Monitoring Without Defined Triggers

Dashboards are watched during rollout, but nobody decided what error rate or latency spike would actually cause a pause, so the team watches numbers drift and expands anyway out of momentum.

### Confounding Rollout With Experiment

The ramp percentage is reused as an experiment cohort without valid randomization, so the team cannot distinguish the feature's effect from selection bias and reports confident conclusions from broken data.

### Silent Exposure That Surprises Users

A feature appears for some users with no announcement or expectation-setting, generating support load and a perception that something is wrong or unfair.

### Big-Bang For A Strategic, Visible Feature

The team wants the impact and the clean story of a single launch moment, so they expose everyone at once and turn a containable defect into a company-wide incident.

## Self-Check

- [ ] Deployment and release are decoupled, with the feature controllable through a flag or config without a redeploy.
- [ ] The rollout pattern was chosen based on risk, reversibility, and learning need, not habit or default.
- [ ] Breaking changes have an explicit migration and backward-compatibility plan with verification and a defined end-of-life for the old path.
- [ ] Canary monitoring defines which metrics are watched and what thresholds trigger pause versus automatic rollback.
- [ ] Hold times at each exposure level are defined, and someone with authority is watching during the ramp.
- [ ] The rollout population is a valid experiment cohort, or the team has explicitly separated rollout from measurement to avoid confounding.
- [ ] Guardrail metrics are defined so gradual exposure does not silently regress an existing metric.
- [ ] User communication during gradual exposure is planned, including how support answers users who lack the feature.
- [ ] If big-bang is chosen, it is justified by low risk, reversibility, and testing, not by desire for a clean launch moment.
- [ ] The rollout plan includes the criteria and timeline for reaching full exposure or for deciding to stop.
