---
name: pr_and_commit_hygiene.md
description: Use when the agent is preparing a pull request, structuring commits, writing commit messages or PR descriptions, splitting or squashing changes, force-pushing, cherry-picking, reverting, managing branch lifetime, deciding what belongs in one PR versus another, or trying to make a change reviewable and bisectable.
---

# PR And Commit Hygiene

A pull request and its commit history are not just a delivery mechanism for code; they are the primary record of *why* a change was made and the main tool for later debugging. When production breaks at 2am, the question "which change caused this, and what was it trying to do?" is answered by `git log`, `git bisect`, and the PR description. Hygiene is the practice of making those tools actually work when they are needed, instead of leaving a trail of giant squashed blobs, vague messages, and force-pushes that destroyed the intermediate states.

Agents tend to treat commits and PRs as overhead between writing code and merging it. The result is a history where every commit is "wip," "fix," or "address feedback," where a single PR bundles an unrelated refactor with a bug fix, and where a force-push has erased the only record of how a bug was introduced. The judgment problem is to invest enough structure in the change that it remains useful after it is merged — reviewable now, bisectable later, and revertable in isolation when something goes wrong — without turning every change into a bureaucratic ritual that delays shipping.

## Core Rules

### Make Each Commit A Logical, Self-Contained Change

A commit should do one thing and its message should say what that one thing is. The test of a good commit is that a future reader, or `git bisect`, can treat it as a single unit: it introduces or fixes exactly one logical change, it ideally builds and passes tests on its own, and reverting it removes exactly that change without dragging unrelated work with it.

This matters most when something breaks. If a regression appears, `git bisect` walks the history commit by commit; each commit that does one thing localizes the break instantly, while a commit that bundles a feature with a refactor and a config tweak forces the debugger to untangle which part caused the failure. Prefer many small, well-described commits over one large one, even if the PR will later be squashed on merge — the granular history still helps during review and during the window before the squash.

### Write Commit Messages That Answer Why, Not Just What

A diff already shows what changed. The commit message's job is to carry what the diff cannot: the motivation, the alternatives considered, the context that made this change necessary, and any non-obvious constraints. A message of "fix bug" or "update utils" adds nothing the diff does not already contain.

A strong message has a short subject line (imperative mood, what this commit does, under roughly seventy characters) and a body that explains the reasoning. The body should answer: what problem does this solve, why is this the right approach, what was tried and rejected, and what reviewers should pay attention to. For non-trivial changes, link to the issue, design doc, or incident. A reader six months later should be able to understand the change from the message without spelunking through the diff.

### Size PRs To The Reviewer's Capacity

A PR is a unit of review and a unit of revert. Both favor smaller PRs. A reviewer can hold a few hundred lines of context in mind and give them a careful read; a few thousand lines get skimmed, and the subtle bugs that review exists to catch slip through. A PR that does one thing can be reverted cleanly when it breaks; a PR that does five things cannot be reverted without losing the four that worked.

When a change is large, decompose it into a stack or sequence of PRs, each building on the last and each reviewable independently. Common decompositions: a preparation PR that adds infrastructure with no behavior change, then a PR that migrates callers, then a PR that removes the old path. Each PR is small, each is reviewable, and each leaves the system working. This is more work than one giant PR, but it converts an unreviewable blob into a reviewable sequence.

### Write A PR Description That Lets The Reviewer Review

The PR description is the reviewer's map. A description that says only "see title" or "fixes #1234" forces the reviewer to reconstruct the intent from the diff, which is exactly the work the description should spare them. The description should let a reviewer who has not followed the work understand what to look at.

A useful description covers:

- **What this change does and why** — the problem and the approach, in a few sentences.
- **How to review it** — where to start, which files or commits matter most, what is mechanical versus what needs judgment.
- **How it was tested** — which tests were added or run, what was manually verified, what edge cases were considered.
- **Risk and rollout** — what could go wrong, whether it is behind a flag, how it will be monitored, and the rollback plan.
- **Context** — links to issues, design docs, prior art, or related PRs.

Investing a few minutes in the description saves every reviewer time and produces better review, because reviewers can focus on the risky parts instead of re-deriving the structure.

### Keep Branches Short-Lived And Synced

Long-lived branches are where changes go to cause pain. The longer a branch lives apart from main, the more it diverges, the harder the merge becomes, the more it accumulates conflicts and stale assumptions, and the greater the chance that two long-lived branches collide catastrophically when they both land. Feature branches should be short — days, not weeks — and rebased or merged against main frequently to catch divergence early.

When a change genuinely needs a long-lived branch (a large migration, a platform upgrade), that is a signal to manage it deliberately: integrate from main often, communicate with anyone else working in the same area, and consider landing it incrementally behind a flag rather than as one future merge. A long-lived branch that is silently worked on and then dropped as a giant merge is one of the most common sources of late-cycle disaster.

### Handle History Rewrites And Force-Pushes Deliberately

Force-push rewrites history, which is powerful and destructive. It is appropriate on your own unmerged feature branch to clean up commits before review — squashing fixup commits, reordering for clarity, rewriting messages. It is almost never appropriate on a shared branch, on main, or on a branch others have based work on, because it erases the commits they depend on and breaks their local clones.

Rules of thumb:

- Force-push only to branches you alone own, and only before merge.
- Prefer `--force-with-lease` over `--force`; it refuses to overwrite if someone else has pushed, preventing you from clobbering a collaborator's work.
- Never force-push to main or any shared integration branch. If main is wrong, fix it forward with a new commit or a revert, not a rewrite.
- If you must rewrite a shared branch, coordinate with everyone affected first; do not discover the breakage when they pull.

### Revert Cleanly, Don't Patch Over

When a change breaks production, the fastest and safest response is usually to revert the offending PR or commit, not to push a forward fix while the system is down. A revert restores a known-good state immediately and buys time to diagnose. A revert is only possible if the change was a clean unit; a change tangled with unrelated work reverts incompletely and may break dependencies.

Treat reverts as a first-class operation, not a failure to be avoided. Reverting is a sign the safety system worked. After reverting, communicate what happened, diagnose the root cause, and re-land the change with the fix and the regression test that would have caught it. Cherry-pick a fix forward onto a release branch when the main fix cannot wait for the next release, but do so knowing each cherry-pick is a manual divergence that must eventually be reconciled.

## Common Traps

### The "WIP" And "Fix" Commit Avalanche

Commits named "wip," "fix," "address feedback," or "asdf" carry no information and accumulate until the history is unreadable. They help nobody, including the author, once context is lost. Either write meaningful commits as you go, or clean them up with an interactive rebase before pushing for review. A history of placeholder commits is debt that someone — often you — pays later.

### The Omnibus PR

A PR that bundles a refactor, a bug fix, a dependency upgrade, and a new feature cannot be reviewed carefully and cannot be reverted selectively. When one part breaks, the whole PR is blocked or reverted, taking the good parts with it. Split by intent: one PR per logical change, even if that means a stack of dependent PRs.

### Squashing Away The Useful History

Squash-merging can be fine, but squashing before review — collapsing a well-structured sequence of commits into one blob — discards the very structure that makes the change reviewable and bisectable. Keep granular commits through review; squash only at merge if your team's policy requires it, and ensure the squashed message still captures the why.

### Force-Pushing Over Collaborators

Force-pushing a branch a teammate just pulled destroys their work and breeds distrust. If a branch is shared, coordinate rewrites or avoid them. The convenience of a clean history on your machine is not worth the cost of silently invalidating someone else's local state.

### The Stale Long-Lived Branch

A branch worked on for weeks without integrating from main diverges until the merge becomes a multi-day conflict resolution exercise, often rushed and error-prone. Integrate from main frequently, or land the work incrementally. The pain of frequent small merges is far less than the pain of one giant reconciliation.

### Commit Messages That Describe The Diff

A message that restates the diff ("change X to Y in file Z") adds nothing. The diff already says that. The message must carry the motivation and context the diff cannot. If the message could be generated by reading the diff, it is not doing its job.

### Revert-Phobia

Treating a revert as a defeat and instead pushing forward fixes under pressure leads to longer outages and riskier changes. Reverting to a known-good state is almost always faster and safer than hot-fixing unknown territory. Normalize revert as a standard incident response, not a last resort.

## Self-Check

- [ ] Each commit does one logical thing, builds and passes tests on its own where feasible, and could be reverted in isolation without dragging unrelated changes.
- [ ] Commit messages have a clear imperative subject and a body explaining the motivation, alternatives considered, and non-obvious context — not just a restatement of the diff.
- [ ] The PR is sized to be reviewable; large changes were decomposed into a stack or sequence of independent PRs rather than one omnibus.
- [ ] The PR description explains what and why, where to focus review, how it was tested, the risk and rollback plan, and links to relevant context.
- [ ] Branches are short-lived and synced frequently against main; any long-lived branch is managed deliberately with frequent integration and communication.
- [ ] Force-pushes target only your own unmerged branches, use `--force-with-lease`, and never touch main or shared branches without coordination.
- [ ] When a change broke production, the response was a clean revert to a known-good state followed by diagnosis, not a forward patch under pressure.
- [ ] Cherry-picks to release branches are tracked as manual divergences that will be reconciled, not forgotten.
- [ ] No commit in the history is a placeholder ("wip," "fix," "asdf") that survived to merge without being rewritten or given a real message.
