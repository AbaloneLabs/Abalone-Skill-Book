---
name: bug-triage-and-severity-classification.md
description: Use when the agent is triaging game bugs, assigning severity and priority, deciding what to fix versus defer for ship, managing a bug backlog during certification, or balancing fix effort against player impact and cert risk.
---

# Bug Triage and Severity Classification

Bug triage is the daily decision of which imperfections ship and which do not, and the judgment problem is that every bug looks fixable in isolation while the aggregate fix list is always larger than the time remaining. The recurring failure is that severity is assigned by how dramatic the bug looks rather than by its actual impact on the player and the ship, so dramatic-but-rare bugs get fixed while subtle-but-pervasive ones ship, and the cert-blocking bugs are discovered late because they were buried under cosmetic ones. Agents miss the critical distinctions because triage feels like a sorting exercise rather than a risk decision, the cost of a fix is rarely weighed against the cost of not fixing, and the pressure to show a shrinking bug count rewards closing easy bugs over resolving important ones. This skill covers how to assign severity against player impact and ship risk, how to distinguish fix from defer from accept, and how to run triage so that the bugs that matter are not starved by the bugs that are easy. The designer has authority over severity for design bugs but must triage jointly with engineering and QA against a shared standard.

## Core Rules

### Assign Severity by Player Impact and Ship Risk, Not by Dramaticness

The severity of a bug is a function of how many players will hit it, how badly it harms their experience, and whether it blocks cert or corrupts save data, not how visually striking the failure is. A crash that requires a convoluted sequence one percent of players will attempt is less severe than a subtle input lag that affects every combat encounter. The rule is to score severity against three axes: frequency (how many players, how often), impact (how badly it harms the experience or the data), and blocking status (does it prevent cert, corrupt progress, or break a core loop). The trap is letting the most dramatic report set the priority, because dramatic bugs are memorable and easy to advocate for, while the pervasive quiet ones do the real damage to review scores and retention. Weight frequency and impact over spectacle.

### Separate Severity From Priority and Fix Them Independently

Severity is how bad the bug is; priority is when it gets fixed. A high-severity bug that is extremely rare may be lower priority than a medium-severity bug that affects every player, because the fix order is determined by the combination of severity, frequency, fix cost, and ship proximity. The rule is to assign severity by the bug's nature and priority by the project's needs, and never to collapse them into a single field, because doing so hides the reasoning and makes the backlog impossible to triage under pressure. When severity and priority are conflated, the team fixes high-severity-low-frequency bugs first because they sound urgent, and ships with the medium-severity pervasive ones unfixed.

### Weigh Fix Cost Against Ship Cost for Every Deferred Bug

Deferring a bug is not free; it is a decision to ship the bug, and that decision has a cost measured in player experience, review scores, support load, and patch burden. The rule is that every defer decision should be made by comparing the cost of the fix, in time and risk, against the cost of shipping the bug. A bug that costs two hours to fix but will generate hundreds of support tickets is a clear fix; a bug that costs three weeks and affects a vanishingly rare path may be a clear defer. The trap is deferring by default under deadline pressure without evaluating the ship cost, so the backlog fills with deferred bugs whose collective ship cost exceeds the cost of fixing the top few. Make the defer an explicit acceptance of ship cost, not a passive default.

### Protect the Bug Budget From Cosmetic Drain

Near ship, the team is tired and the easy bugs are tempting, because closing them shrinks the count and feels like progress. The rule is that the fix budget, the limited hours remaining, must be defended against cosmetic and low-impact bugs that consume effort without reducing ship risk. The discipline is to fix the cert-blockers and the experience-harming pervasive bugs first, and to leave cosmetic issues for a day-one patch or to accept them. When the budget is spent on visible-but-harmless bugs, the harmful ones ship because the hours ran out, and the shrinking count was an illusion of progress. Triage must be willing to close cosmetic bugs as won't-fix to protect the budget for what matters.

### Require Reproduction Steps and Frequency Evidence Before Severity

A bug report without reproduction steps is a rumor, and assigning severity to a rumor guarantees misallocation. The rule is that no bug receives a final severity until it has been reproduced, its frequency has been estimated, and its conditions have been characterized. The trap is triaging from the reporter's description, which is usually dramatized and often incomplete, leading to over-severity on dramatic reports and under-severity on subtle ones. When a bug cannot be reproduced, its severity is provisional and it should be returned to QA for investigation rather than fixed speculatively or closed as invalid. Reproduction is the evidence that makes triage a decision rather than a guess.

### Treat Save Corruption and Progression Blocks as Automatic High Severity

Certain bug categories have consequences that compound: a crash that loses thirty minutes is bad, but a bug that corrupts a save file destroys tens of hours and the player's trust in the product. The rule is that save corruption, progression blocks that prevent completing the game, data loss, and cert-failing violations are automatic high severity regardless of frequency, because their impact is catastrophic and irreversible from the player's perspective. The trap is weighing their low frequency against their severity and deferring them, when even a single player losing a fifty-hour save generates disproportionate harm through reviews and refunds. These categories do not get the normal frequency-versus-impact tradeoff; they get fixed or explicitly accepted with leadership signoff.

### Run Triage With All Three Disciplines Present

Triage that excludes any discipline produces bad decisions. Without QA, frequency and reproduction are guesses; without engineering, fix cost and regression risk are guesses; without design, the experience impact is a guess. The rule is that the standing triage meeting includes representatives of all three, and that severity and priority are assigned by consensus against a written rubric, not by any single discipline unilaterally. When one discipline triages alone, it optimizes for its own concerns: QA over-severifies everything, engineering under-severifies to protect the schedule, design over-weights experience and under-weights cost. The joint meeting forces the tradeoffs into the open where they can be decided rather than assumed.

## Common Traps

### The Shrinking Count Illusion

The team closes fifty bugs in a week, the count drops, and leadership reports healthy progress. The trap is that the closed bugs were the easy cosmetic ones, while the cert-blockers and pervasive experience bugs remain open and unfixed, so the shrinking count masks unchanged ship risk. The false signal is the downward trend in total bugs. The harm is that the project appears on track until the end, when the remaining bugs turn out to be the expensive important ones and there is no budget left to fix them.

### Severity Inflation From Dramatic Reports

A tester reports a visually spectacular bug, a model stretching grotesquely or a physics explosion, and the team assigns it high severity because it looks severe. The trap is that dramaticness is unrelated to player impact, and the bug may be rare, harmless, and trivial, while a quiet input-handling bug that affects every session goes under-severified because it looks mundane. The false signal is the visceral reaction to the report. The harm is that fix effort flows toward spectacle and away from the subtle bugs that actually drive negative reviews.

### Deferring by Default Under Deadline Pressure

As ship approaches, the team defers bugs reflexively to protect the date, treating defer as the safe option. The trap is that defer is not safe; it is a decision to ship the bug, and deferring without evaluating ship cost accumulates a backlog of accepted harm that the team has not consciously accepted. The false signal is that the schedule is protected. The harm is a game that ships with a long tail of known issues whose collective impact exceeds the cost of fixing the worst few, plus a day-one patch burden that could have been avoided.

### Fixing the Bug Without Fixing the Reproduction

A bug is fixed, but the reproduction steps were never nailed down, so the fix is verified by playing hopefully rather than by reproducing the original failure and confirming it no longer occurs. The trap is that without a reproduction, the fix cannot be verified, and the bug often recurs or was never actually fixed. The false signal is that the bug is marked closed. The harm is regression, because the unverifiable fix does not hold, and the bug reappears at cert or in the wild where it is far more expensive.

### Closing Unreproducible Bugs as Invalid

A bug cannot be reproduced in triage, so it is closed as invalid or works-as-designed to clear the backlog. The trap is that many real bugs are intermittent or condition-dependent, and closing them as invalid discards real defects that will surface in player hands. The false signal is the clean backlog. The harm is that known-but-unreproduced issues ship to players, who encounter them without the context the team had, generating support load and negative reviews for bugs the team had already seen and dismissed.

### Conflating Severity and Priority Into One Field

The tracking system uses a single urgency field, and the team assigns it by gut, mixing how bad the bug is with when to fix it. The trap is that this collapses the reasoning and makes the backlog impossible to triage rationally, because a high-severity rare bug and a medium-severity pervasive bug both end up "high urgency" and compete for the same budget without the tradeoff being visible. The false signal is the simplicity of one field. The harm is misallocated fix effort and a backlog nobody can reason about under pressure.

## Self-Check

- Is each bug's severity scored against frequency, impact, and blocking status, with pervasive quiet bugs weighted over dramatic rare ones?
- Are severity and priority assigned as separate fields, so that fix order reflects the combination of severity, frequency, cost, and ship proximity?
- For every deferred bug, has the ship cost been weighed against the fix cost, and is the defer an explicit acceptance rather than a passive default?
- Has the fix budget been protected from cosmetic drain, with low-impact bugs closed as won't-fix where necessary to preserve effort for cert-blockers and pervasive issues?
- Does every triaged bug have confirmed reproduction steps and a frequency estimate before its severity is finalized?
- Are save corruption, progression blocks, data loss, and cert violations treated as automatic high severity regardless of frequency?
- Is triage run with QA, engineering, and design present, assigning severity and priority by consensus against a written rubric rather than by any single discipline?
