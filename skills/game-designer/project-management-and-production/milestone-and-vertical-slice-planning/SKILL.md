---
name: milestone-and-vertical-slice-planning.md
description: Use when the agent is planning production milestones, defining a vertical slice, sequencing feature delivery across a schedule, structuring alpha and beta phases, or deciding what must be proven before greenlight and full production.
---

# Milestone and Vertical Slice Planning

A milestone plan is a bet about which unknowns must be resolved before the team commits the full budget, and the recurring failure is that milestones are structured around deliverable volume rather than around risk reduction. The judgment problem is that a schedule full of features feels like progress while leaving the actual risks untouched, and the team arrives at beta having built everything except the one thing that determines whether the game is fun. Agents miss the critical decisions because a milestone that checks off many tasks looks healthy, the vertical slice is treated as a demo rather than a proof of the core, and the cost of sequencing a feature too late is invisible until the team runs out of time to fix what the late integration reveals. This skill covers how to structure milestones to retire risk early, how to define a vertical slice that actually proves the game, and how to sequence work so that the most uncertain and most central systems are proven while there is still time to change them. The designer has freedom in phasing but is accountable for whether the plan proves the game is worth building before the money is spent.

## Core Rules

### Structure Milestones Around Risk Retirement, Not Deliverable Volume

The purpose of early milestones is not to produce assets but to answer the questions whose answers determine whether the game can succeed. A milestone that delivers a hundred props but leaves the core combat unproven has failed, regardless of how much was produced. The rule is that each milestone should be defined by the risks it retires: can the core loop sustain a session, does the art style read at gameplay speed, can the network handle the target player count, does the target audience understand the moment-to-moment goal. Sequence the riskiest and most central questions first, because those are the ones whose answers can cancel or redirect the project, and a late answer to a fundamental question is a project-ending event. When a milestone's deliverables do not map to a named risk, it is busywork dressed as progress.

### Define the Vertical Slice as a Proof of the Core, Not a Polished Demo

A vertical slice is one complete pass through the core loop at near-final quality, and its job is to prove that the loop is fun and that the production pipeline can deliver final-quality content, not to produce marketing material. The trap is treating the slice as a showcase, which leads the team to polish the parts that look good and quietly skip the parts that are uncertain. The rule is that the slice must include the riskiest interaction at production quality, the full content pipeline exercised end to end, and an honest playtest with the target audience. If the slice is not fun, that is the most valuable possible finding, and the plan must have a branch for redesigning the core rather than pushing forward on the assumption that fun will emerge later.

### Sequence the Most Central and Uncertain Systems Earliest

The features that are both central to the experience and technically or creatively uncertain must be scheduled first, because they are the ones most likely to require iteration, and iteration requires time. The features that are well-understood and peripheral can be scheduled later, because they are unlikely to destabilize the project if they slip. The common error is the inverse: scheduling the safe, familiar features first to show progress, and deferring the risky core until late, when there is no time to fix what the late build reveals. The rule is to front-load the work whose failure would change the project, and back-load the work whose failure would only delay it.

### Make Every Milestone Falsifiable With a Real Playtest

A milestone that cannot fail has not proven anything. Each milestone should include a concrete test, usually a playtest with the intended audience or a measurable performance target, that can return a verdict the team will respect. The rule is that the test must be defined before the milestone begins, so that the team cannot retroactively declare success by moving the goal. When a milestone's success criteria are vague, the team will declare success under pressure and carry an unproven risk into the next phase. When the criteria are sharp and the milestone fails, that failure is information that saves the project money, not a disaster to be hidden.

### Build the Production Pipeline Into the Early Milestones

The content pipeline, the build system, and the tooling are themselves risks, and deferring them until production means discovering at full scale that the pipeline cannot sustain the team's cadence. The rule is that early milestones must exercise the real pipeline at small scale: author content through the real tools, push it through the real build, and load it on the real target hardware. A pipeline that works for a prototype's dozen assets may collapse at thousands, and the only way to know is to stress it early. When the pipeline is treated as a production-phase concern, the team enters full production with a bottleneck that throttles everyone and that cannot be fixed without stopping the line.

### Plan for the Branch Where the Core Does Not Work

A healthy milestone plan includes the branch where the vertical slice reveals the core is not fun, and that branch has a defined response: redesign the loop, change a key system, or in the worst case cancel. The rule is not pessimism but honesty, because a plan that assumes success is a plan that cannot adapt to the most important information the project will generate. The branch need not be elaborate, but the team and the funders must agree in advance that a failed slice triggers a defined reassessment rather than a panicked push forward. Without this agreement, a negative result gets buried and the project ships a core nobody validated.

### Distinguish Alpha, Beta, and Cert by What Risk Each Phase Retires

The phases mean different things and the team often blurs them. Alpha is the phase where the game is feature-complete but unpolished, and its job is to prove the full scope hangs together. Beta is the phase where the game is content-complete and the job is to fix, balance, and optimize. Cert is the phase where the job is to satisfy platform holders and ship. The rule is that work appropriate to one phase must not be deferred to a later phase: features cannot be added in beta, content cannot be added in cert. When these boundaries blur, the team tries to finish features during the phase meant for polish, and the game ships rough because the polish phase was consumed by unfinished scope.

## Common Traps

### The Milestone That Looks Green but Retires No Risk

A milestone delivers a large volume of assets, levels, and UI, the burn-down chart looks healthy, and leadership reports the project is on track. The trap is that none of the delivered work answered a fundamental question about whether the game is fun or feasible, so the project's real risk is unchanged despite the appearance of progress. The false signal is the volume of completed tasks. The harm is that the team reaches a late milestone having built extensively on an unproven core, and the eventual discovery that the core is weak invalidates months of work.

### The Vertical Slice That Skips the Risky Part

The slice is built to look impressive, so the team polishes the combat encounter that was already working and omits the stealth system that has never felt right, rationalizing that it will come later. The trap is that the slice was the moment to prove the uncertain system, and skipping it means the risk survives into full production, where it is far more expensive to address. The false signal is the polished, demo-ready slice that plays well in a controlled showing. The harm is that the unproven system fails at scale, and the team has already committed the budget on the assumption it would work.

### Front-Loading Safe Work to Show Progress

The schedule puts the familiar systems and the asset production first because they are predictable, and defers the risky core systems to the middle of production. The trap is that this produces reassuring early milestones while accumulating the project-ending risk for later, when there is no slack to absorb a failure. The false signal is the consistent on-time delivery of early milestones. The harm is that the core is proven late, and when it needs iteration the schedule has no room, forcing either a ship-date slip or a quality compromise on the game's most important system.

### Vague Success Criteria That Cannot Fail

A milestone is declared complete when the team feels good about it, with no predefined test that could return a negative verdict. The trap is that without a falsifiable criterion, success becomes a matter of morale and politics, and under pressure the team will always declare success to avoid conflict. The false signal is the consensus that the milestone went well. The harm is that unproven risks accumulate invisibly, because the milestones that should have caught them were structured to be unable to fail.

### Treating the Pipeline as a Production-Phase Concern

The team prototypes with hacked-together tools and manual content placement, planning to build the real pipeline once production starts. The trap is that the pipeline is itself a major risk, and discovering at production scale that authoring is slow, builds are fragile, or content drops fail throttles the entire team. The false signal is that the prototype works, which feels like the pipeline is solved. The harm is that production begins without a pipeline that can sustain it, and the team loses months building tooling while the schedule burns.

### Deferring Polish Into a Phase That Cannot Hold It

The plan assumes polish happens in the final weeks, but the final weeks are also consumed by cert submission, platform requirements, and bug fixing. The trap is that polish needs time and attention that the end of a project never has, so the features that were "good enough" in beta ship unpolished because there was never a moment to return to them. The false signal is the plan's allocation of a polish phase. The harm is a game that functions but feels rough, which is exactly the quality level players and reviewers punish most harshly.

## Self-Check

- Does each milestone map to a named risk it retires, with the riskiest and most central questions sequenced earliest?
- Is the vertical slice defined as a proof of the core loop and the production pipeline at near-final quality, including the riskiest interaction rather than only the parts that already work?
- Are the most central and uncertain systems scheduled early enough that a failure leaves time to iterate, rather than deferred to a phase with no slack?
- Does every milestone have a predefined, falsifiable success test, usually a real playtest or measurable target, that can return a verdict the team will respect?
- Have the content pipeline, build system, and tooling been exercised at small scale in early milestones rather than deferred to full production?
- Is there an agreed branch in the plan for the case where the vertical slice reveals the core is not fun, with a defined response rather than an assumption of success?
- Are alpha, beta, and cert boundaries respected so that features are not added in beta and content is not added in cert, with polish given a phase that can actually hold it?
