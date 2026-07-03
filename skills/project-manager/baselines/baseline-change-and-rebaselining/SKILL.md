---
name: baseline_change_and_rebaselining.md
description: Use when the agent is deciding whether to rebaseline a project, managing controlled baseline changes versus performance variance, or preserving baseline integrity so the baseline remains a valid measurement reference over time.
---

# Baseline Change And Rebaselining

A baseline is only useful as long as it is stable, but no real project holds perfectly to its original plan. This creates a tension at the heart of baseline management: change the baseline too readily and it stops measuring anything, because it simply tracks whatever happened; never change it and it stops measuring anything, because it no longer reflects the work being done. The skill is distinguishing the two legitimate reasons to touch a baseline, approved change and structural re-baselining, from the illegitimate one, cosmetic adjustment to make the numbers look better, and applying each with discipline.

The judgment problem is deciding when a baseline change is warranted, what kind of change it is, and how to preserve the integrity of the reference through the change. Agents tend to blur the distinction between an approved change, which updates the baseline while preserving its history, and a re-baseline, which resets the reference point, and they sometimes reach for re-baselining to erase embarrassing variance. The discipline is to keep the baseline honest: let it absorb approved changes through versioning, reserve full re-baselining for genuine structural shifts, and never use it to hide performance.

## Core Rules

### Distinguish Approved Change Updates From Re-baselining

These are different operations with different purposes. An approved change update incorporates a formally approved change into the baseline, preserving the prior version so the change is traceable; the reference shifts but its lineage is intact. A re-baseline replaces the baseline with a new one because the old one no longer reflects reality, resetting the measurement reference and preserving the historical variance separately. Conflating them produces a baseline whose history is either over-fragmented by routine changes or erased by structural resets. Decide which operation a given situation calls for before acting.

### Update The Baseline For Approved Changes, Not For Variance

When a change is formally approved through change control, the baseline must be updated to reflect it, so the reference always represents the current approved plan. But performance variance, the project running ahead or behind, is not a change to the baseline; it is exactly what the baseline is supposed to measure. Updating the baseline to match actual performance destroys its measurement value. The rule is simple: approved changes update the baseline; actual performance does not. If you find yourself wanting to move the baseline to match reality, ask whether you are updating for a change or hiding variance.

### Reserve Re-baselining For Genuine Structural Shifts

A full re-baseline is warranted only when the existing baseline can no longer serve as a meaningful reference, typically because accumulated changes, corrected assumptions, or reprioritization have made it unrecognizable. The test is whether variance against the current baseline still tells you anything useful. If the baseline is so far from reality that variance is noise, a re-baseline is legitimate. If it is merely showing uncomfortable but real performance, re-baselining is evasion. Make this judgment on measurement integrity, not on whether the numbers look bad.

### Preserve Historical Variance When Re-baselining

A re-baseline resets the reference, but it must not erase the past. The variance accumulated against the old baseline is real performance data and must be retained, archived, and explained. Re-baselining without preserving history is cosmetic manipulation: it makes the new baseline look clean by destroying the evidence of how the project actually performed. Always archive the prior baseline, its variance record, and a documented rationale for why the re-baseline was necessary. The history is what makes the new baseline defensible.

### Apply The Same Completeness Criteria To A New Baseline

A re-baseline is, in effect, establishing a new baseline, and it must meet the same completeness criteria as the original: decomposed scope, mapped dependencies, identified critical path, cost tied to the WBS, documented assumptions. A re-baseline that is sloppier than the original simply replaces a broken reference with a worse one. Treat re-baselining with the same rigor as initial establishment, including formal approval against criteria. The temptation to rush a re-baseline under pressure is exactly when rigor matters most.

### Version And Time-Stamp Every Baseline Change

Every baseline, whether updated for a change or replaced by a re-baseline, must be versioned and time-stamped, with a clear record of what changed and why. This version history is what allows anyone to reconstruct, at any later point, what the reference was on a given date and how it evolved. Without versioning, the baseline is a moving target that cannot support audit, dispute resolution, or lessons learned. Versioning is not optional record-keeping; it is the mechanism that makes the baseline trustworthy.

### Guard Against Cosmetic Re-baselining

The most dangerous baseline abuse is re-baselining to make performance look better. When the schedule is slipping and the budget is over, the temptation is to declare a fresh start that resets variance to zero. This hides problems rather than solving them, misleads sponsors, and destroys the credibility of all future reporting. The defense is governance: require that any re-baseline be justified against the measurement-integrity test, approved by an authority outside the delivery team, and accompanied by preserved history. Make cosmetic re-baselining hard to do and easy to detect.

### Keep The Baseline And The Forecast Separate

The baseline is the approved reference; the forecast is the current prediction of where the project will end up. They are different and must not be confused. Updating the forecast as reality changes is normal and expected; updating the baseline to match the forecast is not. A common error is to let the baseline drift toward the forecast until the two are indistinguishable, at which point measurement ceases. Maintain the baseline as the fixed reference and the forecast as the living estimate, and report both so variance between them is visible.

## Common Traps

### Re-baselining To Erase Uncomfortable Variance

The numbers look bad, so the team resets the baseline to zero and reports clean variance. The trap is that the performance problem is hidden, not solved, and sponsors are misled. Reserve re-baselining for structural shifts and preserve history.

### Updating The Baseline To Match Actuals

The project drifted, so the baseline is moved to where the project actually is, destroying its measurement value. The trap is treating variance as a change. Only approved changes update the baseline.

### Conflating Change Updates With Re-baselining

Routine approved changes trigger full re-baselines, fragmenting history; or structural shifts are handled as minor updates, leaving a broken reference. The trap is not naming which operation is needed. Decide deliberately.

### Re-baselining Without Rigor

A new baseline is rushed under pressure and is less complete than the original. The trap is replacing a flawed reference with a worse one. Apply the same completeness criteria and approval.

### Erasing Historical Variance

The old baseline and its variance are discarded so the new one looks clean. The trap is destroying the evidence of real performance. Archive the prior baseline and its variance with rationale.

### Letting The Baseline Drift Toward The Forecast

The baseline is quietly adjusted toward the current forecast until the two merge and variance disappears. The trap is that measurement silently stops. Keep baseline and forecast separate and report both.

### Unversioned Baseline Changes

The baseline changes without versioning or time-stamps, so no one can reconstruct the reference at a past date. The trap is an untrustworthy, unauditable baseline. Version every change.

## Self-Check

- [ ] Is each baseline modification clearly classified as an approved-change update versus a structural re-baseline before it is applied?
- [ ] Are approved changes incorporated into the baseline while performance variance is left to be measured, not absorbed?
- [ ] Is full re-baselining reserved for cases where the existing baseline no longer supports meaningful measurement, not where numbers look bad?
- [ ] When re-baselining occurs, is the prior baseline archived along with its variance record and a documented rationale?
- [ ] Does a re-baseline meet the same completeness criteria and formal approval as the original baseline establishment?
- [ ] Is every baseline change versioned and time-stamped with a record of what changed and why?
- [ ] Are cosmetic re-baselines prevented through governance requiring justification, external approval, and preserved history?
- [ ] Are the baseline, the fixed approved reference, and the forecast, the living prediction, kept separate rather than allowed to converge?
- [ ] Can the baseline reference at any past date be reconstructed from the version history alone?
- [ ] Is the integrity of the baseline as a measurement reference preserved through every change, so future variance remains trustworthy?
