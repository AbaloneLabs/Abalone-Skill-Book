---
name: quality_assurance_and_control.md
description: Use when the agent is assuring project quality, performing quality control and inspection, managing defects, conducting reviews and audits, deciding whether work meets acceptance criteria, or reviewing whether quality processes are preventing and catching defects effectively.
---

# Quality Assurance And Control

Planning quality is necessary but not sufficient. Quality must be assured, through processes that prevent defects, and controlled, through inspection and testing that catches the defects that escape. A project that relies on hope and final inspection will ship defects that cost far more to fix than to prevent. The project manager must run assurance and control as live disciplines, manage defects honestly, and use quality findings to improve the work rather than to assign blame.

Use this skill before setting up quality control, running reviews or audits, managing a defect backlog, or diagnosing why defects keep escaping to production. The goal is to prevent the agent from treating quality checks as ceremonial gates and from letting defect backlogs grow without resolution.

## Core Rules

### Run Assurance As Prevention

Quality assurance focuses on the process. Its goal is to ensure that the way work is done reliably produces quality, so that fewer defects are created in the first place.

Assurance activities include defining and following standards, conducting design and code reviews, using defined development practices, providing training, and auditing process adherence. Assurance is preventive and ongoing, not a one-time check.

When defects recur, ask what process gap allowed them, and fix the process, not just the defect. Prevention is cheaper than detection, which is cheaper than production failure.

### Run Control As Detection

Quality control focuses on the product. Its goal is to detect defects in the output before it reaches the next stage or the customer. Control activities include testing, inspection, measurement, and verification against acceptance criteria.

Control should be proportionate to risk. Critical and high-risk features deserve more rigorous and earlier testing. Low-risk work may need less, but no work should reach acceptance without some verification. Design control to catch defects as early as feasible, because the cost of fixing a defect rises sharply the later it is found.

### Manage Defects As A First-Class Backlog

Defects are work. They need to be logged, prioritized, assigned, tracked, and resolved, just like features. A defect backlog that is never triaged or resolved is a hidden quality debt that surfaces at the worst time.

For each defect, capture its description, severity, reproduction steps, owner, and status. Triage regularly to prioritize: critical defects block release; minor defects may be deferred consciously. Make the defect backlog visible so quality debt is a known, managed quantity rather than a surprise.

### Decide Defect Resolution Deliberately

Not every defect must be fixed immediately, and not every defect can be deferred. The decision should be deliberate, based on severity, impact, risk, and cost.

Define decision rules: critical defects must be fixed before release; major defects need a fix or an accepted risk with rationale; minor defects may be scheduled or deferred to a later release. Document accepted defects as known issues with their impact, so they are conscious decisions rather than oversights.

### Use Reviews And Inspections Effectively

Reviews and inspections are among the most effective quality controls, catching defects that testing misses, especially design and requirements defects. Run them with purpose and rigor.

Effective reviews define what is being checked, use checklists based on common defect types, involve the right reviewers, and capture findings as actionable items. Reviews that become rubber stamps or social events provide the appearance of quality without the substance.

### Trace Quality To Requirements And Acceptance

Quality control is meaningful only against defined requirements and acceptance criteria. Testing that is not traced to requirements may verify the wrong things or miss important ones.

Maintain traceability from each requirement to the test or inspection that verifies it. At acceptance, confirm that all requirements have been verified and that acceptance criteria are met or consciously deviated.

### Analyze Quality Findings To Improve

Quality findings are data. Defect patterns reveal where the process is weak, which areas produce the most defects, and where to focus improvement. Use this data rather than treating each defect as an isolated event.

Conduct root cause analysis on significant defects and recurring patterns. Feed the findings back into process change, training, standards, or design. A project that finds the same defects repeatedly is not learning from its quality data.

### Separate Quality Authority From Delivery Pressure

Quality decisions, especially release decisions, must be protected from delivery pressure that would ship poor work to hit a date. Define who has the authority to block release for quality and ensure that authority is respected.

When schedule and quality conflict, the tradeoff should be a conscious decision by the right authority, with the quality risk made explicit. Silently lowering the quality bar to hit a date stores up future failure.

## Common Traps

### Final Inspection Only

Relying on end-of-project inspection finds defects when they are most expensive and hardest to fix.

### Rubber-Stamp Reviews

Reviews that approve without rigor provide appearance without substance.

### Untmanaged Defect Backlog

Defects logged but never triaged or resolved become hidden quality debt.

### Fixing Symptoms Not Causes

Fixing each defect without addressing the process gap guarantees recurrence.

### Control Without Traceability

Testing not traced to requirements may verify the wrong things and miss important ones.

### Quality Under Schedule Pressure

Silently lowering the quality bar to hit a date creates future production failure.

### Defects As Blame

When defects are punished, they are hidden, and the chance to learn is lost.

### Deferred Defects Without Records

Defects deferred without documentation become forgotten oversights rather than conscious decisions.

## Self-Check

- [ ] Quality assurance focuses on process and prevention, with standards, reviews, and audits.
- [ ] Quality control focuses on product detection, with testing and inspection proportional to risk and applied early.
- [ ] Defects are managed as a first-class backlog with description, severity, owner, and status, and are triaged regularly.
- [ ] Defect resolution is decided deliberately based on severity, impact, and risk, with accepted defects documented as known issues.
- [ ] Reviews and inspections use checklists, the right reviewers, and actionable findings rather than rubber-stamping.
- [ ] Quality control is traced to requirements and acceptance criteria so the right things are verified.
- [ ] Quality findings are analyzed for patterns and root causes and fed back into process improvement.
- [ ] Release and quality decision authority is defined and protected from delivery pressure.
- [ ] Schedule-quality tradeoffs are conscious decisions with quality risk made explicit.
- [ ] Defects are treated as information for improvement, not as blame to be hidden.
