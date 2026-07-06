---
name: jam-teamwork-and-pipeline-constraints.md
description: Use when the agent is working in a game jam team under extreme constraints, managing jam teamwork and pipeline, or evaluating whether the jam team's workflow serves the short timeframe or collapses into merge conflicts, unmerged work, and pipeline bottlenecks where the final build is missing half the team's contributions because the integration never happened.
---

# Jam Teamwork and Pipeline Constraints

A game jam team operates under extreme constraints — tiny timeframe, small team, everyone exhausted — and the teamwork and pipeline (how the team collaborates and integrates work) is often the difference between a jam game that ships complete and one that collapses into merge conflicts, unmerged work, and a final build missing half the team's contributions because the integration never happened. The judgment problem is that the team must divide work cleanly (so members work in parallel without conflict), must integrate continuously (so work merges into the build throughout, not at the end), and must manage the pipeline constraints (asset formats, engine limits, build process) before they bottleneck. Agents tend to miss this because the creative excitement of a jam deprioritizes pipeline, and because the integration that everyone assumes will happen at the end does not happen when the team is exhausted and time is gone. The harm is jam games that ship without their team's work, that fail to build at the deadline, or that collapse under pipeline problems that were solvable if planned. This skill covers how to manage jam teamwork and pipeline to ship complete, and avoid the integration, conflict, and bottleneck traps. The agent has latitude in the workflow, but the obligation to integrate and ship is not optional.

## Core Rules

### Divide Work Cleanly to Enable Parallel Progress

Work must be divided cleanly — each member owns clear areas (mechanics, art, audio, level design) with minimal overlap — so the team works in parallel without conflict, because overlapping work produces merge conflicts and blocked dependencies, and clean division enables the parallel progress a jam timeframe requires. The decision rule: divide work cleanly with clear ownership, minimize overlap, and avoid conflicting parallel work. Overlapping division creates conflict, because the work was not cleanly separated.

### Integrate Continuously, Not at the End

Work must be integrated continuously — each member merges their work into the shared build throughout the jam, not at the end — because end-only integration (the team works separately and merges at the deadline) fails when the exhausted team runs out of time, and continuous integration keeps the build whole and reveals integration problems early. The decision rule: integrate continuously into the shared build, and avoid end-only integration. End-only integration fails, because the merge never happened in time.

### Maintain a Always-Shippable Build

The team should maintain an always-shippable build — at every point, the current build is submittable — because a jam can end at any moment (deadline, crash, exhaustion), and a build that is only shippable at the end risks having nothing to submit if the end goes wrong, and an always-shippable build guarantees a submission. The decision rule: maintain an always-shippable build, integrate toward it, and avoid builds that are only shippable at the end. End-only-shippable builds risk no submission, because the build was not always ready.

### Plan the Pipeline and Asset Constraints Early

The pipeline — how assets flow into the build, the engine and format constraints, the build process — must be planned early, because pipeline problems (wrong formats, engine limits, build failures) discovered late bottleneck the jam, and early planning prevents the pipeline from becoming the constraint. The decision rule: plan the pipeline and constraints early, test the build process, and avoid late pipeline discovery. Late pipeline discovery bottlenecks, because the constraints were not planned.

### Communicate Dependencies and Blockers Immediately

Dependencies (one member's work waiting on another's) and blockers (a member stuck) must be communicated immediately, because uncommunicated dependencies produce blocked work (waiting silently) and uncommunicated blockers produce stuck work (struggling silently), and immediate communication lets the team adapt before the time is lost. The decision rule: communicate dependencies and blockers immediately, and avoid silent waiting or struggling. Silent dependency and blocker communication loses time, because the team could not adapt.

### Prioritize Submission-Ready Over Ambitious at the End

As the deadline approaches, the team must prioritize submission-ready (a complete, working jam game) over ambitious (a more feature-rich but unfinished one), because a complete simple game is a successful jam submission while an unfinished ambitious one is a failure, and the courage to cut ambition for completion is what ships. The decision rule: prioritize submission-ready at the end, cut ambition for completion, and avoid unfinished ambition. Unfinished ambition fails the jam, because the completion was sacrificed for scope.

## Common Traps

### Overlapping Division Creating Conflict

The team divides work with overlap, and the parallel work conflicts. The trap is that the team collaborates. The false signal is that the members are productive. The harm is that the overlapping division produces merge conflicts (two members editing the same files), the blocked dependencies (one member waiting on another's overlapping work), the parallel progress is hindered by the conflict, the time is lost to conflict resolution, and the jam suffers from the non-clean division, because the work was not cleanly separated.

### End-Only Integration Failing

The team plans to integrate at the end, and the integration fails. The trap is that the integration will happen at the end. The false signal is that the members are working. The harm is that the end-only integration is deferred to the exhausted deadline, the merge of separately-developed work reveals conflicts and incompatibilities, the exhausted team runs out of time, the integration never completes, and the final build is missing half the team's work because the merge failed, because the integration was not continuous.

### End-Only-Shippable Builds Risking No Submission

The team maintains a build that is only shippable at the end, and the submission is risked. The trap is that the build will be ready at the end. The false signal is that the build is developing. The harm is that the end-only-shippable build depends on the deadline going right, the crash or exhaustion or integration failure at the end leaves nothing shippable, the jam submission is missed or is broken, the team has nothing to show, and the jam fails to produce a submission, because the build was not always shippable.

### Late Pipeline Discovery Bottlenecking

The team discovers pipeline problems late, and the pipeline bottlenecks the jam. The trap is that the pipeline will work out. The false signal is that the assets are being made. The harm is that the late pipeline discovery (wrong formats, engine limits, build failures) emerges when time is short, the pipeline becomes the constraint blocking the build, the team scrambles to fix pipeline problems under deadline pressure, the time is lost to pipeline firefighting, and the jam suffers from the unplanned pipeline, because the constraints were not planned early.

### Silent Dependency and Blocker Communication Losing Time

The team fails to communicate dependencies and blockers, and the time is lost. The trap is that the members are independent. The false signal is that the members are working. The harm is that the uncommunicated dependencies produce silent waiting (a member blocked on another's work without saying), the uncommunicated blockers produce silent struggling (a member stuck without asking for help), the time is lost to the silent waits and struggles, the team cannot adapt, and the jam loses time that communication would have saved, because the communication was not immediate.

### Unfinished Ambition Failing the Jam

The team prioritizes ambition over completion at the end, and the jam fails. The trap is that the ambitious game will be better. The false signal is that the scope is impressive. The harm is that the ambition prevents completion, the unfinished ambitious game is missing features and content at the deadline, the submission is incomplete or broken, the complete simple game that would have succeeded is passed over, the courage to cut ambition was absent, and the jam fails to produce a submission-worthy game, because the completion was sacrificed for scope.

## Self-Check

- Is work divided cleanly with clear ownership and minimal overlap to enable parallel progress?
- Is work integrated continuously into the shared build rather than deferred to the end?
- Is an always-shippable build maintained so a submission exists at every point?
- Is the pipeline (asset flow, engine constraints, build process) planned and tested early?
- Are dependencies and blockers communicated immediately so the team can adapt?
- Is submission-ready prioritized over ambition as the deadline approaches, cutting scope for completion?
- Did I confirm the jam team ships a complete integrated game rather than collapsing under pipeline and integration failures?
