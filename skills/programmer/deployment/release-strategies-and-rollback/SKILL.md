---
name: release_strategies_and_rollback.md
description: Use when the agent is planning how to ship a change safely, choosing between blue-green, canary, rolling, or big-bang deployment, designing a rollback plan, deciding whether to roll back or roll forward, using feature flags or dark launching to decouple deploy from release, coordinating a database migration with a deployment, setting deployment cadence, or reviewing whether a release pipeline is safe. Also covers progressive delivery, traffic shifting, deployment vs release distinction, automated deployment gates, human-in-the-loop checkpoints, and the compatibility constraints that make rollback possible. Use before any change goes to production, when a release caused an incident, or when auditing release safety.
---

# Release Strategies And Rollback

Shipping a change to production is two decisions bundled together: *deploy* (put the new code in a position to serve traffic) and *release* (actually route users onto it). Treating them as one atomic act — build, push, serve everyone — is the highest-risk way to ship, because the first time you learn whether the change is correct is the moment every user is exposed to it. The discipline of release strategies is to separate these decisions, expose the change gradually, keep a known-good state to return to, and make "undo" a real, tested operation rather than a hope. A team that cannot roll back does not have a deployment system; it has a one-way door.

Agents tend to focus on getting the new version *out* and treat rollback as the thing you do if something goes wrong, discovered under pressure. This inverts the real priority: the rollback path is the safety system, and it must be designed, tested, and kept viable *before* the release, because the moment you need it is the worst possible time to discover it does not work. The judgment problems are: how do I expose this change so that harm is bounded and detectable, how do I keep the option to return to the previous state, and — critically — what constraints (migrations, schema, irreversible side effects) have quietly made rollback impossible without anyone noticing. This skill covers release strategy and rollback as a system. The mechanics of safe schema changes themselves are covered by the schema-and-migration skill, which this skill references but does not duplicate.

## Core Rules

### Separate Deploy From Release, And Make Release Gradual

Deploying code and releasing it to users are different operations, and conflating them is the root of most release risk. Decouple them:

- **Deploy** puts the new version into the environment without serving production traffic — installed, started, health-checked, but behind a switch.
- **Release** routes real users onto the new version, ideally incrementally.

When these are separate, you can deploy at any time without risk, observe the new version under synthetic or shadow traffic, and release only when you have evidence it works. The mechanisms that decouple them — feature flags, traffic-shifting load balancers, routing rules — let you expose the change to 1% of users, watch, then 5%, 25%, 100%, rolling back at any step by moving a switch rather than redeploying. The cost is infrastructure and process complexity; the benefit is that a broken release affects 1% of users for minutes instead of 100% for the duration of a redeploy. Prefer gradual release for any change whose failure would harm users, and especially for changes you cannot fully test in pre-production.

### Choose The Strategy By Blast Radius, State, And Reversibility

The common strategies make different tradeoffs; choose by the properties of what you are releasing:

- **Rolling (incremental).** Replace instances a few at a time across the fleet. Old and new versions run simultaneously during the rollout. Pros: simple, resource-efficient, no extra capacity needed. Cons: old and new must be compatible (shared storage, shared schema, API clients), and rollback means rolling the other way, which is slow for large fleets. Good for stateless services with backward-compatible changes.
- **Blue-green.** Run two complete environments; switch traffic from blue (current) to green (new) all at once. Pros: instant switch and instant rollback (move traffic back), no version-mix period. Cons: requires roughly double the capacity, and the switch is all-or-nothing, so a subtle bug hits everyone at once unless combined with canary. Good when you need fast, clean rollback and can afford the capacity.
- **Canary.** Release to a small subset first, observe, then expand. Pros: limits blast radius, catches problems early, gives real signal before full exposure. Cons: requires traffic routing and good observability to detect problems in the small population, and canary populations must be representative (not just internal users) or you miss real-world failure. Good for any change where early detection of harm is worth the operational cost.
- **Big-bang / all-at-once.** Replace everything simultaneously. Pros: none worth having in production. Cons: maximum blast radius, no gradual detection, rollback requires redeploy. Acceptable only for trivial changes or environments with no users.

The strong default for user-facing production changes is canary on top of rolling or blue-green: gradual exposure with a fast switch back. Reserve big-bang for genuinely trivial or non-production releases.

### Design And Test The Rollback Before You Need It

Rollback is a safety system, and a safety system that has never been tested is assumed not to work. Before releasing:

- **Know what "previous good" means.** Identify the exact artifact (image, commit, version) you will return to, and confirm it is still available and deployable.
- **Verify the rollback path is unblocked.** A migration that already ran, a feature flag default that already flipped, a queue message already consumed, or a third-party API already called may make the previous version unable to operate on current state. If any of these block rollback, you no longer have a two-way door — plan accordingly (see the next rule).
- **Test rollback in pre-production** as part of the release pipeline, not just rollout. A rollback that has only ever been theorized will fail the first time it is real.
- **Define the rollback trigger in advance.** What signal (error rate, latency, specific errors) triggers rollback, and who is authorized to call it? Deciding this during the incident wastes the most valuable minutes.

Treat rollback as a first-class release operation with its own runbook, not as an emergency improvisation. If rollback is not viable for a given change, that fact must be explicit and the release must compensate with extra gradual-release safety, because you have given up your safety net.

### Check Whether Database Or State Changes Have Silently Broken Rollback

The most dangerous rollback failures come from changes to shared state. Code is reversible; data often is not. Before assuming you can roll back, check every state-touching change in the release:

- **Schema migrations.** A migration that adds a column is usually safe for rollback (old code ignores it); a migration that drops, renames, or narrows a column can make the old version unable to read or write the data. Expansive, backward-compatible migrations (add, don't remove) keep both versions working; destructive changes must be split across multiple releases so rollback never lands on incompatible data. (The schema-and-migration skill covers the migration mechanics in depth.)
- **Data backfills and transformations.** If the release transforms existing rows, the old version sees transformed data it may not expect.
- **External side effects.** Messages already published, emails already sent, payments already charged, webhooks already delivered cannot be un-sent by a rollback. The old version resumes, but the side effects persist.
- **Feature flag state and configuration.** A flag default that changed, or config that the new version wrote, may leave the old version in a state it does not understand.

The rule: if the release changes shared state irreversibly, rollback to the old *code* may not restore the old *behavior*. In those cases, prefer roll-forward (fix and redeploy) or design the change to be backward-compatible so both versions coexist. Never discover mid-incident that the migration you ran has made rollback impossible.

### Use Feature Flags To Decouple Release From Deploy And To Limit Blast Radius

Feature flags turn "release" into a runtime switch rather than a deployment event. They let you deploy code dormant, enable it for internal users, then a beta cohort, then everyone — and disable it instantly without redeploying if it misbehaves. This is especially valuable for:

- **Risky logic changes** that are hard to canary by traffic (e.g., a new pricing calculation): ship behind a flag, enable per-tenant, disable instantly.
- **Dark launching** — run the new code path in production using real traffic but discard its output, comparing its behavior to the old path. You learn whether the new path is correct under real load before any user sees its results.
- **Decoupling deploy from release cadence** — deploy multiple times a day, release on a separate schedule.

Flags have costs: they multiply the code paths you must test (every flag combination is a configuration), stale flags become technical debt, and a flag that gates a migration or schema change can create its own compatibility hazards. Treat flags as inventory: every flag has an owner, a sunset date, and a removal task. A flag that lives forever is not a release tool; it is untested branching.

### Prefer Roll Back Over Roll Forward Under Pressure, With Defined Exceptions

When a release causes an incident, the default is to roll back to the known-good state, not to patch forward. Roll back is fast and returns to understood behavior; a hotfix written under pressure is unverified code shipped into a broken system, which can compound the failure. (The incident-response skill covers this decision during a live outage.)

The exceptions, where roll forward is correct, are narrow and should be known in advance:

- Rollback is impossible because an irreversible migration or side effect already occurred.
- The old version has a known worse defect.
- The fix is trivial and already validated (a one-line config, a flag flip).

Make the roll-back-vs-forward decision deliberately and state the reason. "We rolled forward because rollback was blocked by the migration" is a useful record; "we just fixed it" is a signal that the decision was not made consciously. After any rollback-triggering release, hold a postmortem on why the problem reached production and how the release strategy did or did not contain it.

### Match Deployment Cadence To The System's Ability To Detect And Contain Failure

How often you deploy and how safely you deploy are linked. Frequent small deployments are safer than rare large ones, because each change is smaller (less surface for bugs), failures are easier to localize (the last deploy is the prime suspect), and the deployment muscle stays trained. A team that deploys once a month treats each deploy as a high-stakes event; a team that deploys many times a day treats each as routine, and the routine is where safety lives.

But frequent deployment only helps if the detection and containment systems keep up. Before increasing cadence, ensure:

- observability detects a bad release within minutes (error rate, latency, business metrics), not hours;
- the rollback path is automated or low-friction, so containment is fast;
- automated gates (tests, canary analysis, deployment checks) catch what they can before humans have to.

Pushing cadence without these is just shipping faster into a system that cannot notice or undo mistakes. The goal is not maximum frequency; it is a cadence at which each release is small, monitored, and reversible.

### Keep A Human In The Loop At The Points Where Judgment Cannot Be Automated

Automation should handle the mechanical, repetitive, and verifiable parts of deployment: build, test, promote, health-check, shift traffic. But some decisions genuinely require human judgment and should not be fully automated away:

- **Promotion past a gate.** When canary metrics are ambiguous — slightly elevated latency, a new but low-rate error — a human decides whether to proceed, hold, or roll back. Auto-promotion on "no critical alerts" can advance a slow-burning problem.
- **Release of high-stakes changes.** Pricing, security, data migration, or irreversible operations warrant an explicit human approval even in a highly automated pipeline.
- **Rollback decisions during partial failure.** Whether to roll back or hold for investigation is a judgment call that depends on context automation does not have.

The discipline is to automate everything that is mechanical, and to make the remaining human checkpoints meaningful — a human approval that rubber-stamps every release is not a checkpoint, it is theater. Design the pipeline so that when a human is asked, their input actually matters.

## Common Traps

### Deploying And Releasing As One Atomic Act

Building, pushing, and serving 100% of traffic in one step means the first real test of the change is also the moment of maximum exposure. Any defect hits everyone immediately, and the only response is a redeploy. Separate deploy from release and shift traffic gradually so defects are caught at 1% not 100%.

### Assuming Rollback Works Because It Was Never Needed

The rollback path is declared in the runbook but never exercised, so the first real rollback discovers that the previous artifact was garbage-collected, the migration is incompatible, or the procedure has a missing step. Test rollback in pre-production and as part of the release drill; an untested rollback is a hope, not a capability.

### A Migration That Quietly Makes Rollback Impossible

The release includes a column rename or a destructive schema change. Code rollback lands the old version on data it cannot read, and the "rollback" makes things worse. This is usually discovered mid-incident. Keep migrations expansive and backward-compatible within a release; split destructive changes across releases so rollback never hits incompatible state.

### Canary That Tests The Wrong Population

Routing canary traffic only to internal users or only to a non-representative cohort, so the canary passes but the release fails for real users whose data, scale, or behavior differs. Make canary populations representative of real traffic, and canary on the dimensions that actually vary (tenants, geographies, request shapes).

### Feature Flags That Become Permanent Branching

Flags are added to release safely but never removed, accumulating into a combinatorial explosion of untested code paths. Every flag is a branch the tests must cover; flags without sunset dates become load-bearing dead code. Track flags as inventory with owners and removal tasks.

### Auto-Promoting Past Ambiguous Canary Signals

The pipeline auto-promotes because no *critical* alert fired, even though latency is slightly up and a new error appeared at low rate. The slow-burning problem reaches full release. Require a human decision when signals are ambiguous, and tune canary analysis to detect degradation, not just hard failures.

### Big-Bang Deployments In Production "Because It's Faster"

Replacing all instances at once to save time or capacity. The blast radius is maximal, detection is after-the-fact, and rollback is a full redeploy. The time saved is repaid many times over the first time it causes an outage. Use rolling or blue-green with canary.

### Releasing Without A Defined Rollback Trigger and increasing Deploy Frequency Without Detection Or Rollback Capacity

The team releases, then waits to see if anyone complains, then debates whether "this is bad enough to roll back." The decision is made late, by committee, under pressure. Define the rollback trigger (specific metrics and thresholds) and the owner before the release, so containment is a reflex, not a meeting.

Shipping faster to gain safety, but without the observability to detect a bad release quickly or the automation to roll it back fast. Higher frequency without containment is just faster harm. Build detection and rollback first, then raise cadence.

## Self-Check

- [ ] Deploy and release are separated: new code can be deployed without serving production traffic, and release shifts users onto it gradually (canary/rolling/blue-green), not all at once for user-facing production changes.
- [ ] The deployment strategy was chosen by blast radius, statefulness, and reversibility — rolling for compatible stateless changes, blue-green when fast switchback and capacity allow, canary layered on top for early detection — and big-bang is reserved for trivial or non-production releases.
- [ ] The rollback target (exact previous artifact) is identified and confirmed deployable before release, the rollback path is tested in pre-production as part of the pipeline, and the rollback trigger (specific metrics, thresholds, and owner) is defined in advance rather than debated mid-incident.
- [ ] Every state-touching change in the release (schema migration, data backfill, external side effects, flag/config state) was checked for whether it makes rollback to old code produce old behavior; destructive or irreversible changes were split across releases or made backward-compatible so rollback never lands on incompatible state.
- [ ] Feature flags are used to decouple release from deploy and limit blast radius, but every flag has an owner, a sunset date, and a removal task — flags are tracked inventory, not permanent branching.
- [ ] Under incident pressure, rollback is the default over roll-forward hotfixes, with the narrow exceptions (irreversible migration, worse old version, trivial validated fix) stated explicitly and recorded.
- [ ] Deployment cadence is matched to detection and containment capability: observability catches bad releases in minutes, rollback is low-friction, and automated gates run before human checkpoints — frequency was raised only after these exist.
- [ ] Human-in-the-loop checkpoints exist only where judgment is required (ambiguous canary signals, high-stakes changes, rollback-vs-hold decisions) and are meaningful, not rubber-stamp approvals; everything mechanical is automated.
