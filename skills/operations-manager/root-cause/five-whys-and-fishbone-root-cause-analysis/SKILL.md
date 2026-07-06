---
name: five-whys-and-fishbone-root-cause-analysis.md
description: Use when the agent is using Five Whys, fishbone diagrams, cause-and-effect analysis, contributing factor review, causal hypothesis testing, or structured operational problem analysis to move beyond symptoms and identify system conditions that created a failure.
---

# Five Whys And Fishbone Root Cause Analysis

Five Whys and fishbone analysis are not rituals for filling a template. They are ways to test causal hypotheses and find controllable system conditions. Agents often use them mechanically, writing a tidy chain that ends at training, communication, or human error. This skill helps the agent use causal tools with evidence, branching, and operational judgment so the result supports real prevention.

## Core Rules

### Start From A Well-Defined Problem

Use a precise failure statement before asking why. The problem should name what failed, expected standard, affected segment, time, volume, and impact. If the problem is vague, the why chain will be vague.

Do not analyze multiple problems in one chain. A missed SLA, wrong customer message, and failed reconciliation may share causes, but each needs a clear starting point.

### Treat Each Why As A Hypothesis

Each answer should be testable with evidence. Ask what evidence supports it, what contradicts it, and what else could explain the same symptom. If an answer cannot be tested, rewrite it.

Avoid moving from one assumption to the next. A plausible story is not a root cause unless evidence supports it.

### Allow Branching Causes

Many operational failures have multiple contributing factors: workload, unclear SOP, tool design, missing access, weak training, incentive pressure, vendor delay, supervision gap, and detection failure. A single linear Five Whys chain may oversimplify.

Use fishbone categories to widen thinking: people, process, tools, data, environment, policy, measurement, management system, vendor, customer input, and controls. Then test which branches mattered.

### Distinguish Root Cause From Contributing Factor

A root cause is a condition that, if changed, would materially reduce recurrence. A contributing factor made the event more likely or worse. Both matter, but they should not be blurred.

Some causes are outside direct control, such as weather or customer behavior. In those cases, root cause work should focus on controllable preparedness, detection, response, and resilience.

### Avoid Human Error Endpoints

If the chain ends with "agent forgot," "manager missed," or "team did not communicate," ask why the system allowed or encouraged that outcome. Consider workload, prompts, checklist design, unclear ownership, fatigue, training, permission, incentives, interruption, and quality controls.

Individual accountability may still be relevant, especially for willful violation, but prevention usually requires system action.

### Include Detection And Recovery Causes

Failures often become incidents because detection or recovery failed. Ask why the issue was not caught earlier, why impact spread, why escalation was delayed, and why recovery took as long as it did. These may be separate root causes from the original trigger.

Good analysis covers trigger, escape, detection, response, and recovery.

### Connect Causes To Corrective Actions

Every corrective action should map to a cause or contributing factor. Training maps to a knowledge or skill gap; a control maps to detection or prevention; a staffing change maps to capacity; a policy update maps to ambiguity; automation maps to repetitive error or timing.

If an action does not map to a cause, remove it or label it as general improvement rather than corrective action.

### Test Whether The Action Would Have Prevented The Event

For each proposed action, ask whether it would likely have prevented, detected earlier, reduced impact, or sped recovery for the actual event. If not, the action may be cosmetic.

Also check for side effects. A stronger approval control may reduce errors but slow service; an automation may reduce manual mistakes but create silent failures.

### Record Uncertainty And Follow-Up

Some causes remain uncertain. Record confidence level, evidence gaps, and follow-up needed. Do not force a neat answer because the template expects one.

Where uncertainty is high, use monitoring or a staged action plan and revisit after new evidence appears.

## Common Traps

- Starting Five Whys from a vague complaint rather than a precise failure.
- Writing why answers that are plausible but unsupported by evidence.
- Forcing a single linear chain when multiple factors contributed.
- Ending at "training issue," "communication issue," or "human error" without system explanation.
- Treating external events as root causes and ignoring controllable preparedness or response.
- Ignoring detection failure and focusing only on the original trigger.
- Listing corrective actions that do not map to identified causes; choosing actions because they are easy, not because they would prevent recurrence
- Hiding uncertainty to make the analysis look complete; using a fishbone diagram as a brainstorming artifact without testing which branches actually mattered

## Self-Check

- Does the analysis start from a precise operational failure statement?
- Is each why answer treated as a testable hypothesis with supporting evidence?
- Are multiple causal branches considered where the event was not linear?
- Are root causes separated from contributing factors and consequences?
- Has the analysis avoided stopping at human error without system context?
- Are trigger, escape, detection, response, and recovery factors considered?
- Does every corrective action map to a specific cause or contributing factor?
- Would the proposed action likely have prevented, detected, reduced, or shortened the actual event?
- Are tradeoffs and side effects of corrective actions visible?
- Are uncertainty, evidence gaps, and follow-up monitoring documented?
