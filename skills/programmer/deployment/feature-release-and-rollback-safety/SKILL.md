---
name: feature_release_and_rollback_safety.md
description: Use when the agent is releasing a feature behind a flag, designing a kill switch or percentage rollout or targeted cohort release, writing a per-feature rollback runbook, deciding between flag-flip rollback and redeploy rollback, coordinating a data migration with a feature release using the expand-contract pattern, handling irreversible side effects such as sent emails or charged payments or delivered webhooks, running a hotfix under incident pressure, or building a release checklist and staged rollout across environments. Use when the release is per-feature rather than per-version, and when rollback safety at the feature level is the concern.
---

# Feature Release And Rollback Safety

A deployment puts a new version of the software into an environment. A release turns a feature on for users. These are different decisions, and the most dangerous releases are the ones that conflate them — where shipping the code and exposing the feature happen in the same irreversible act, so that the first time you learn the feature is wrong is the moment every user is exposed to it, and the only way back is another deployment. Feature-level release safety is the discipline of separating these decisions and making each feature's exposure gradual, observable, and reversible independently of every other feature in the same deployment. Done well, a broken feature is disabled in seconds without touching anything else; done poorly, a single bad feature forces a full redeploy that takes down unrelated work along with it.

Agents tend to treat a feature flag as a one-line toggle and the rollback as "turn it off," and to stop there. The real judgment is elsewhere: whether the flag's off-path actually restores old behavior or only old code (because a migration already changed the data underneath it), whether the feature's side effects can even be reversed (an email cannot be un-sent), whether the rollback is instant or requires a redeploy, and whether the hotfix path under pressure has been thought through before the pressure exists. This skill covers feature-flag-driven release granularity, the expand-contract migration pattern that keeps rollback possible, per-feature rollback runbooks, and the irreversible-side-effect problem that quietly makes some rollbacks impossible. The general design and lifecycle of flags is covered by the feature-flag-design skill; the high-level comparison of release strategies and the rollback philosophy are covered by the release-strategies skill. This skill is the feature-level, flag-driven, migration-aware dimension of release safety that sits between them.

## Core Rules

### Separate Deploy From Release As A Per-Feature Discipline

The deploy-versus-release distinction is most powerful when applied per feature, not per version. A single deployment can carry many features, each behind its own flag, each released on its own schedule:

- **Deploy code dormant, release per feature.** Ship the new code with every new feature's flag off. The deployment itself carries no user-facing risk, because no feature is exposed. Then release each feature independently, in its own order, on its own evidence.
- **Independently reversible.** When each feature has its own flag, a problem with one feature is fixed by flipping that one flag, without redeploying and without affecting the other features that shipped in the same deployment. This is the core benefit: blast radius is per feature, not per version.
- **Independently gradual.** Each feature can be released to its own cohort at its own pace — one feature at 100% while another is still at 5% for a beta tenant — because they do not share a release lever.

The cost is flag discipline: each flag is a code path that must be tested and eventually removed. The benefit — independent, reversible, gradual release of each feature — is the entire point of feature-flag-driven release and is worth the discipline. Strong choice: a deployment that ships five features behind five flags, released and rolled back independently. Weak choice: a deployment that ships five features all enabled at once, so any one defect forces a redeploy that affects all five.

### Choose The Release Mechanism By The Failure Mode You Need To Contain

Feature release is not one mechanism. Different mechanisms contain different failure modes, and the choice should match the risk:

- **Kill switch.** A boolean flag that enables or disables the feature for everyone. Fastest possible rollback (one flip, seconds, no redeploy). Good for features whose failure mode is binary (works or does not) and where the cost of an exposed defect is bounded. The off-path must be real and tested.
- **Percentage rollout.** The flag exposes the feature to an increasing percentage of users (1%, 5%, 25%, 50%, 100%), with verification at each step. Bounds blast radius and catches regressions early. Requires enough traffic at each percentage for the signal to be meaningful, and requires the same metric-and-threshold discipline as canary.
- **Targeted cohort.** The flag exposes the feature to a specific set of users (internal, a beta tenant, a region) rather than a random percentage. Good when you want signal from a specific population before broad exposure, or when the feature is entitlement-gated.

The strong default for risky features is layered: targeted cohort first (signal from a known population), then percentage rollout (bound blast radius broadly), with a kill switch always available on top (instant off if anything goes wrong). Weak choice: a single boolean enable for everyone, which is a big-bang release with a flag in front of it.

### Write A Per-Feature Rollback Runbook Before The Release

Rollback that is improvised under pressure is slow and wrong. Each feature release should have a written rollback runbook that answers the operational questions before they are asked:

- **What is the rollback action, and how fast is it?** A flag flip is seconds and reversible; a redeploy is minutes and may itself fail. State which one applies to this feature and why. A feature whose only rollback is a redeploy has given up the main benefit of flag-driven release.
- **What is the rollback trigger?** Which metrics, error rates, or symptoms mean "roll back now," defined in advance so the decision is mechanical rather than a debate during an incident.
- **Who is authorized to roll back, and how?** The person on call, the release owner, anyone with access? Make the authority and the access path explicit so rollback is not blocked on finding the one person who knows how.
- **What does the off-path assume about state, and is that assumption still valid after release?** If the release migrated data or wrote new state, the off-path may not behave as expected (see the migration rules below). State this explicitly so the operator knows whether flag-flip rollback is real or illusory.

A runbook that has never been walked through is a hope. For high-stakes features, rehearse the rollback in pre-production so the first real rollback is not also the first attempt.

### Use The Expand-Contract Pattern To Keep Migrations From Breaking Rollback

The most common reason a feature rollback fails is that a data migration has made the old code path unable to operate on the current data. Code is reversible; data often is not. The expand-contract (also called parallel-change) pattern keeps rollback possible by splitting a migration into phases across releases:

- **Expand (backward-compatible).** Add the new schema or data without removing the old. The new feature can write the new shape; the old code still reads and writes the old shape. Both versions coexist. Rollback to the old code is safe, because the old shape is intact.
- **Migrate (the feature releases).** With both shapes present, release the feature behind its flag. Backfill or dual-write as needed. Rollback at any point returns to the old code, which still works against the old shape.
- **Contract (remove the old, in a later release).** Only after the feature is fully released and stable, and only when you are confident you will not roll back to the old code, remove the old shape. This is the destructive step, and it must come last.

The rule that follows: never combine expand and contract in the same release. A release that adds the new shape and drops the old one in one step has no rollback path — the old code lands on data it cannot read. Destructive changes (drop column, rename, narrow type, delete rows) must be their own release, scheduled after the feature that needed them is confirmed stable. Strong choice: a three-release sequence — expand, release feature, contract — where each release is independently rollback-safe. Weak choice: a single release that renames a column and ships the feature that uses the new name, guaranteeing that rollback to the old code fails on the migrated data.

### Handle Irreversible Side Effects By Prevention, Not Rollback

Some operations cannot be undone by any rollback. An email already sent, a payment already charged, a webhook already delivered, a notification already pushed, a file already written to a user's storage — these persist after the code is rolled back. The feature is "off" again, but the side effects remain, and users experience them as bugs the rollback did not fix:

- **Identify irreversible side effects before release.** List every external action the feature takes (sends email, charges payment, calls a third-party API, writes to shared storage). For each, ask: if we roll back, is this action already done? If yes, rollback does not contain the harm.
- **Defer or queue side effects until the feature is confirmed stable.** Send the email from a queue with a delay, so a rollback within the delay window cancels unsent messages. Confirm a payment only after the feature has cleared its release thresholds. Stage webhooks for delivery rather than firing synchronously.
- **Make side effects idempotent and reconcilable.** If the feature may run twice (during a rollback-and-retry, or because a flag flipped back and forth), ensure the side effect happens once, not once per execution. A reconciliation job that detects and reverses erroneous side effects is the backstop for actions that were taken and should not have been.
- **Accept that some releases are one-way and compensate accordingly.** A feature that charges real money and cannot be reversed is a one-way release. Compensate with extra release safety (longer canary, human approval, staged rollout) proportional to the irreversibility, because you have no rollback for the side effect even if you have rollback for the code.

The unifying principle: rollback reverts code and sometimes data, but it never reverts the outside world. Design features so that their contact with the outside world is delayed, idempotent, or reconcilable, and so that the truly irreversible actions happen only after the feature has earned trust.

### Stage The Rollout Across Environments, And Treat Each Stage As Evidence

Releasing straight to production is releasing on faith. Staging the rollout across environments (local, integration, staging, production-canary, production-full) turns each stage into evidence that the next stage is safe:

- **Each environment should catch a different class of defect.** Local catches unit-level bugs fast; integration catches contract and interaction bugs; staging catches environment-and-data-shape bugs; production canary catches real-user-and-scale bugs. If two environments catch the same things, one of them is not earning its place.
- **Promotion is gated on the previous stage's evidence.** Do not promote to the next environment on a schedule; promote on signal. A staging environment that is ignored ("it always passes, just promote") provides no evidence and is theater.
- **Production is the last and most consequential stage, not the first real test.** By the time a feature reaches production canary, it should already have cleared the cheaper environments, so production exposure is bounded and the remaining risk is the kind only real users reveal.

Strong choice: a promotion pipeline where each stage has defined entry criteria and the production stage is a small canary before full release. Weak choice: a pipeline that skips staging "to save time" and discovers environment-specific defects in production.

### Build A Release Checklist And Make The Gates Meaningful

A release checklist is valuable only if its items are real gates rather than theater. Each item should be a check that has, at some point, caught a real defect — otherwise it is friction that trains people to rubber-stamp:

- **Include the rollback-relevant items explicitly.** Is the rollback action known? Is the off-path tested? Are irreversible side effects deferred or reconcilable? Is the migration expand-only within this release? These are the items whose absence causes incidents.
- **Include the evidence items.** Did the feature clear staging? Are the release metrics instrumented and dashboarded? Is the rollback trigger defined?
- **Make each item a gate, not a suggestion.** A checklist item that can be checked without the work being done is worse than no item, because it provides false assurance. Where an item can be automated (tests passed, metrics instrumented), automate it so the gate cannot be bypassed by a click.

The discipline is to keep the checklist short enough to be read and meaningful enough to matter. A fifty-item checklist no one reads protects nothing; a ten-item checklist where every item has caught a real defect protects a great deal.

## Common Traps

### Big-Bang Feature Enable Behind A Flag

Shipping a feature behind a flag but enabling it for 100% of users at once, which is a big-bang release with a flag in front of it. The flag provides no blast-radius benefit; any defect hits everyone. Use percentage rollout or targeted cohort so exposure is gradual.

### Flag-Flip Rollback That Does Not Restore Old Behavior

Flipping a flag off and assuming the feature is undone, when a migration has already changed the data the old code path reads. The code reverts but the behavior does not, because the old path lands on incompatible data. Use expand-contract so rollback to old code works against current state.

### Expand And Contract In The Same Release

Renaming a column and shipping the feature that uses the new name in one release, so rollback to the old code fails on the renamed data. Split destructive changes into their own later release, after the feature is confirmed stable.

### Assuming Rollback Contains Irreversible Side Effects

Rolling back a feature that sent emails or charged payments and treating the rollback as a full recovery, when the emails were sent and the payments were charged. Rollback reverts code, not the outside world. Defer, queue, or reconcile side effects, and treat truly irreversible releases as one-way.

### Rollback Runbook That Exists Only In The Author's Head

Releasing without a written rollback action, trigger, and owner, so that under incident pressure the team debates what to do and loses the most valuable minutes. Write the runbook before the release and rehearse it for high-stakes features.

### A Hotfix Written Under Pressure Shipped Into A Broken System

Choosing to roll forward (write a hotfix) instead of rolling back, because the fix seems small, then compounding the failure because unverified code was shipped into an already-broken system. Default to rollback under pressure; reserve hotfix for the narrow cases where rollback is impossible, the old version is worse, or the fix is trivial and already validated.

### Staging Environment That Is Theater

Promoting through staging as a formality because "it always passes," so environment-specific defects surface only in production. Make each stage earn its place by catching a distinct class of defect, and gate promotion on signal rather than schedule.

### Checklist Items That Are Not Real Gates and releasing Multiple Features On One Shared Flag

A release checklist whose items can be checked without the work being done, providing false assurance and training the team to rubber-stamp. Make each item a real gate, automating it where possible so it cannot be bypassed by a click.

Gating several features behind a single flag to "keep it simple," so that a defect in one forces rolling back all of them, defeating the per-feature blast-radius benefit. Give each independently releasable feature its own flag.

## Self-Check

- [ ] New features are deployed dormant behind their own per-feature flags and released independently, so a defect in one feature is contained by flipping that one flag without redeploying or affecting other features in the same deployment.
- [ ] The release mechanism (kill switch, percentage rollout, targeted cohort) is chosen to match the feature's failure mode, and risky features layer targeted cohort then percentage rollout with a kill switch always available, rather than a single boolean enable for everyone.
- [ ] A per-feature rollback runbook exists before release, stating the rollback action (flag-flip vs redeploy) and its speed, the rollback trigger (specific metrics/symptoms), the authorized owner and access path, and whether the off-path's state assumptions still hold after release.
- [ ] Data migrations follow the expand-contract pattern: expand (backward-compatible) and feature release happen in one release, and the destructive contract (drop/rename/narrow) is a separate later release only after the feature is confirmed stable — expand and contract are never combined in one release.
- [ ] Every irreversible side effect (sent email, charged payment, delivered webhook, pushed notification, written shared file) is identified before release, and is deferred via a queue/delay, made idempotent and reconcilable, or accepted as a one-way release with compensating extra safety proportional to its irreversibility.
- [ ] The rollout is staged across environments (local, integration, staging, production canary, production full), each stage catches a distinct class of defect, and promotion to the next stage is gated on the previous stage's signal rather than on a schedule.
- [ ] The release checklist contains only real gates (rollback action known, off-path tested, irreversible side effects handled, migration expand-only, metrics instrumented, trigger defined), each item has caught or could catch a real defect, and automatable items are automated so they cannot be bypassed by a click.
- [ ] Under incident pressure the default is rollback over roll-forward hotfix, with the narrow roll-forward exceptions (rollback impossible due to irreversible migration or side effect, old version worse, trivial validated fix) stated explicitly and recorded, and the hotfix path was thought through before the pressure existed.
