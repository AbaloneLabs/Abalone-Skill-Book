---
name: team_workflow_and_branching.md
description: Use when the agent is choosing or changing a team branching strategy such as Git Flow, trunk-based development, or GitHub Flow, defining branch lifetimes and merge rules, selecting rebase versus merge commits, designing release and hotfix branches, resolving CI and review bottlenecks, or scaling a collaboration model as a team grows.
---

# Team Workflow and Branching

A branching strategy is not a git preference; it is a coordination protocol that determines how changes integrate, how releases are produced, how conflicts surface, and how fast feedback reaches developers. Teams often inherit a strategy ("we use Git Flow because the last senior engineer set it up") without revisiting whether it fits their current size, release cadence, and risk tolerance. The cost of a mismatched strategy is paid daily in merge conflicts, stale branches, blocked PRs, and release delays, but because the cost is distributed and chronic, it is rarely traced back to its cause.

The judgment problem is that every workflow trades off integration speed against stability and traceability. Trunk-based development integrates fast and exposes problems early but requires discipline (feature flags, small changes, strong CI) to keep the trunk healthy. Git Flow isolates work and produces clean release history but creates long-lived branches that drift and conflict. There is no universally best workflow; the right choice depends on team size, release frequency, deployment risk, and how much the team can invest in CI and feature-flag discipline. The agent must evaluate the tradeoffs in context, not cargo-cult a popular model, and must recognize when a growing team has outgrown its current workflow.

## Core Rules

### Match the workflow to release cadence and team size, not to fashion

The dominant factor is how often you release and how many people integrate concurrently:

- **Trunk-based / main-based development:** Everyone integrates to a single mainline frequently (ideally daily). Best for continuous deployment, small teams, and teams with strong CI and feature-flag discipline. Minimizes drift and integration conflict.
- **GitHub Flow:** Short-lived feature branches off main, merged via PR back to main, with main always deployable. Good for teams deploying frequently who want lightweight review.
- **Git Flow:** Long-lived develop and release branches, with feature branches off develop. Good for products with scheduled releases, multiple supported versions, and strict release management. Heavy for teams deploying continuously.

A common error is adopting Git Flow for a team that deploys continuously; the release and hotfix branches add ceremony with no benefit and the develop branch drifts from main. Conversely, a team shipping boxed software with multiple supported versions cannot survive on pure trunk-based without release branches. Choose for your actual cadence.

### Keep branches short-lived to minimize integration pain

The longer a branch lives, the more it drifts from mainline, the larger the merge conflict, the harder the review, and the higher the risk that the work is wasted. Treat branch age as a first-class metric. Aim to merge feature branches within a day or two; branches older than a week are a smell. If work cannot be completed quickly, break it into smaller mergeable increments or hide unfinished work behind a feature flag so the branch can merge. Long-lived branches are the single largest source of integration pain and the strongest argument for trunk-based plus feature flags.

### Decide merge versus rebase deliberately and enforce it consistently

How history is composed affects readability, bisecting, and conflict resolution:

- **Merge commits** preserve the exact branch topology and make "what was reviewed" auditable, but can clutter history with merge noise.
- **Rebase** produces a linear, readable history and cleaner bisects, but rewrites shared history (dangerous on shared branches) and obscures when work was actually done relative to integration.

The choice matters less than consistency. A team that mixes both unpredictably gets the worst of both: confusing history and rebase conflicts on shared branches. Pick a policy (e.g., squash-merge feature branches into main, or rebase-before-merge), document it, and enforce it in tooling. Never rebase shared branches that others have pulled.

### Make the mainline always deployable (or be explicit when it is not)

In trunk-based and GitHub Flow, the contract is that main is always in a deployable state. This requires: CI that runs before merge (not just after), feature flags to merge incomplete work safely, and the discipline to revert broken changes immediately rather than "fixing forward" on main. If your team cannot keep main deployable, you do not have trunk-based development; you have an unstable main, which is worse than a deliberate release branch. Be honest about whether the discipline exists before choosing the workflow.

### Design release branches for the version-support reality, not for aesthetics

Release branches are justified when you must support multiple versions in production simultaneously (e.g., patching the last release while developing the next). If you only ever have one version in production (continuous deployment), release branches add overhead with no benefit; cherry-pick or revert-and-redo on main instead. When you do use release branches, define clearly: they are cut from main at release time, receive only fixes (cherry-picked from main, not new features), and are eventually retired. Avoid merging release branches back to main except for fixes that must also land on mainline.

### Define hotfix paths before you need them

A hotfix is an urgent fix to production that cannot wait for the normal flow. Define in advance: where the fix is made (on the release branch, or on main and cherry-picked), how it is tested, how it is deployed, and how it is propagated back to mainline so the fix is not lost in the next release. Hotfix paths discovered during an incident are slower and more error-prone than paths defined calmly in advance.

### Recognize and address CI and review bottlenecks

A workflow is only as fast as its slowest gate. If PRs wait days for review, or CI takes an hour, the branching strategy cannot save you. Common bottlenecks: review backlog (too few reviewers, PRs too large to review well), slow CI (no parallelization, redundant runs), and serialization (one PR blocks another). Address these directly: require smaller PRs, parallelize and cache CI, set review SLAs, and avoid chains of dependent PRs. Changing the branching model will not fix a review or CI bottleneck.

## Common Traps

### Inheriting Git Flow for a continuously deploying team

Git Flow's develop and release branches add ceremony designed for scheduled releases. A continuously deploying team pays the overhead (branch drift, extra merges) with none of the benefit. Match the workflow to cadence.

### Long-lived feature branches that drift and conflict

A branch open for two weeks will conflict badly on merge, discourage the author from integrating, and produce a giant unreviewable PR. Force small, short-lived branches and use feature flags to merge incomplete work.

### Rebasing shared branches and rewriting history others depend on

Rebase rewrites history. On a branch others have pulled, this creates duplicate commits, lost work, and confusion. Only rebase private branches or before a PR is merged; never force-push shared branches.

### Treating "main is always deployable" as an aspiration rather than a contract

If broken code lands on main and is "fixed forward" instead of reverted, main is not deployable and the workflow's premise is broken. Either enforce the contract (CI gates, revert-first culture, feature flags) or choose a workflow with explicit release branches.

### Mixing merge and rebase unpredictably

Inconsistent history composition produces confusing logs, hard-to-bisect regressions, and merge conflicts that appear and disappear. Pick one policy per branch type and enforce it.

### Changing the workflow to solve a CI or review problem

If PRs are slow to merge, the cause is often review backlog or slow CI, not the branching model. Switching from Git Flow to trunk-based without fixing the bottleneck just moves the pain. Diagnose the real bottleneck first.

### Release branches that accumulate features instead of only fixes

A release branch that receives new features is no longer a release branch; it is a second mainline that will diverge and conflict. Release branches receive fixes only.

## Self-Check

- Did you choose the workflow (trunk-based, GitHub Flow, Git Flow) based on your actual release cadence, team size, and deployment risk, rather than copying a popular model?
- Are feature branches short-lived (ideally a day or two), with a policy or metric to catch branches that drift too long?
- Is the merge-versus-rebase policy defined per branch type, documented, and enforced in tooling, with a rule never to rebase or force-push shared branches?
- If using trunk-based or GitHub Flow, is main genuinely always deployable, enforced by pre-merge CI, revert-first culture, and feature flags for incomplete work?
- Are release branches used only when multiple versions must be supported simultaneously, and do they receive fixes only (cherry-picked from main), not new features?
- Is there a defined, pre-agreed hotfix path (where the fix is made, how it is tested, how it propagates back to mainline) that does not have to be invented during an incident?
- Have you diagnosed the actual integration bottleneck (review backlog, slow CI, serialized PRs) rather than assuming the branching model is the problem?
- Are PRs sized to be reviewable, and is there a review SLA so branches do not wait indefinitely for feedback?
