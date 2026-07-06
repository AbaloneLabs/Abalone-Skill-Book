---
name: qa_gate_and_acceptance.md
description: Use when the agent is defining or operating a quality gate at translation handoff or delivery, setting acceptance criteria and thresholds, deciding whether a batch passes or fails, handling conditional acceptance and rework, managing sign-off authority, or deciding whether translated content is accepted for release, publication, or payment.
---

# QA Gate And Acceptance

A quality gate is the decision point at which translated content is judged ready to move to the next stage: from translator to reviewer, from reviewer to client, or from client to publication. Acceptance is the verdict the gate produces: pass, conditional pass with rework, or fail. This is where quality stops being a set of measurements and becomes a binding decision with consequences for schedule, cost, liability, and payment. Agents frequently mishandle the gate in two opposite directions. They either treat it as a formality, signing off because the deadline has arrived and the text looks fluent, which lets unconfirmed quality ship and shifts the defect discovery to the end user. Or they treat it as an aspiration, demanding perfection and rejecting batches that meet the agreed threshold, which destroys schedule and budget and erodes trust between translator and client. A well-designed gate is neither. It is a documented, threshold-based, role-assigned decision that accepts content when it meets the agreed bar, rejects it when it does not, and conditions acceptance on defined rework when the gap is fixable. The harm of a weak gate is that quality claims become unverifiable; the harm of a capricious gate is that the supply relationship breaks down and no work ever ships.

Use this skill when defining acceptance criteria and thresholds, operating a handoff or delivery gate, deciding pass, conditional pass, or fail, assigning sign-off authority, or resolving disputes about whether content is acceptable. The goal is a gate that produces a defensible, consistent, consequence-aware acceptance decision.

## Core Rules

### Define Acceptance Criteria Before The Work Starts

Acceptance criteria must exist before translation begins, not be invented at the gate when a batch is late. Criteria should state the quality dimensions that matter, the threshold or score a batch must meet, the sampling basis on which the score is computed, the defects that trigger automatic failure regardless of score, and the rework rules when the threshold is missed. Defining criteria in advance aligns translator, reviewer, and client on the same bar and prevents the gate from becoming a negotiation driven by schedule pressure. Criteria invented after the fact are not criteria; they are rationalizations of a decision already made for other reasons. Publish the criteria in the brief and reference them at the gate.

### Tie The Threshold To Content Tier And Risk

A single acceptance threshold for all content is wrong. High-risk content, medical labels, legal contracts, safety warnings, demands a strict threshold and automatic-failure rules for critical defects such as a wrong dose or reversed negation. Medium-risk content, marketing or documentation, can accept a threshold that permits a small number of minor defects. Low-risk, high-volume content may accept post-edited machine translation against a lighter threshold. The gate must apply the threshold defined for the content's tier. Applying a strict threshold to low-risk content wastes money and delays release; applying a loose threshold to high-risk content invites harm. The tier-to-threshold mapping is part of the framework and must be honored at every gate.

### Distinguish Pass, Conditional Acceptance, And Fail

A binary gate, accept or reject, is too coarse for real projects. Use three outcomes. Pass means the batch meets the threshold and proceeds without rework. Conditional acceptance means the batch is acceptable in principle but contains defined defects that must be fixed, either critical and major errors that block release, or a bounded set of minor errors within a rework budget; the batch proceeds once the defined rework is completed and verified. Fail means the batch misses the threshold by a margin that indicates a systemic problem, such as widespread terminology inconsistency or pervasive accuracy errors, and must be returned for substantive rework or re-translation rather than spot fixes. Defining the boundary between conditional and fail prevents both the trap of accepting everything with a punch list and the trap of rejecting everything that is not perfect.

### Make Critical Defects Automatic Blockers

Regardless of the overall score, certain defects must block acceptance until fixed. A wrong dosage, a reversed negation in a warning, a mistranslated legal obligation, a broken safety instruction, or a corrupted placeholder that breaks the deliverable are not negotiable against a threshold. Define the classes of critical defect that trigger automatic blocking and apply them without exception. The purpose of a score is to govern the gray area; the purpose of automatic blockers is to ensure that the gray area never includes defects that cause harm. A gate that lets a critical defect pass because the overall score was good enough has failed its core function.

### Assign Sign-Off Authority By Competence And Accountability

The person who signs off must have both the competence to judge the content and the accountability for the consequences. For high-risk content, sign-off authority belongs to a qualified reviewer or subject-matter expert, not to a project manager working against a deadline. For lower-risk content, a project reviewer may sign off against the threshold. Define who can sign off for each tier and what they must verify before signing. Sign-off is an attestation that the defined quality has been achieved, and the signer must be someone whose attestation is meaningful. Delegating sign-off to whoever is available empties the gate of authority.

### Compute The Score On A Defined Sampling Basis

The acceptance score is only meaningful if the sampling basis is defined and honored. State whether the score is computed over one hundred percent of the content, over a defined statistical sample, or over a risk-targeted sample, and state the inference the score supports. A score computed over a convenience sample does not support a claim about the whole batch. A score computed over a sample that excluded the high-risk segments is not a valid basis for accepting high-risk content. The sampling basis is part of the criteria, and the gate must record what was actually reviewed, not just the resulting number.

### Handle Disagreements Through A Defined Escalation Path

Translators, reviewers, and clients will disagree at the gate: a translator disputes a reviewer's classification, a client rejects a batch the reviewer passed, a reviewer flags a defect the translator considers a preference. Define an escalation path before these occur: who mediates, what evidence is used, how preferences versus errors are adjudicated, and what the final authority is. A gate without an escalation path resolves disputes by power rather than by criteria, which erodes trust and produces inconsistent decisions. The escalation path should reference the error typology and severity definitions so disagreements are resolved against shared standards rather than opinion.

### Record The Decision And Its Basis

Every gate decision should be recorded: the batch, the criteria applied, the sampling basis, the score, any critical-defect blockers, the outcome, the rework list if conditional, and the sign-off with date and authority. This record is the evidence that quality was confirmed rather than assumed, it supports trend analysis, and it is essential if a defect later reaches the end user and the acceptance decision must be reconstructed. A gate that produces no record has no defensibility and no learning value.

## Common Traps

### Signing Off Because The Deadline Arrived

Treating the gate as a formality under schedule pressure lets unconfirmed quality ship and moves defect discovery to the end user.

### Demanding Perfection Against An Agreed Threshold

Rejecting batches that meet the agreed bar destroys schedule and budget and breaks the supply relationship; the gate enforces the threshold, not an aspiration.

### Inventing Criteria At The Gate

Acceptance criteria defined after the fact are rationalizations, not criteria, and they cannot align translator, reviewer, and client.

### One Threshold For All Content

A single threshold either over-spends on low-risk content or under-protects high-risk content; the threshold must follow the tier.

### Letting Critical Defects Pass On A Good Score

A gate that trades a critical defect against an overall score has failed its core function of preventing harm.

### Sign-Off By Whoever Is Available

Delegating sign-off to someone without competence or accountability empties the gate of authority and meaning.

### Score On An Undefined Or Convenience Sample

A score computed over an undefined or convenience sample does not support a claim about the whole batch and falsifies the acceptance decision.

### No Escalation Path For Disputes

Without a defined path, gate disputes resolve by power rather than criteria, eroding trust and producing inconsistent decisions.

## Self-Check

Before operating a quality gate or recording an acceptance decision, verify:

- Acceptance criteria, including dimensions, threshold, sampling basis, automatic-failure defect classes, and rework rules, were defined and published before the work started.
- The threshold applied at the gate matches the content's tier and risk level, not a single universal setting.
- The outcome distinguishes pass, conditional acceptance with a defined rework list, and fail for systemic problems, rather than collapsing to a binary.
- Critical defects such as wrong dosages, reversed negations, or broken safety instructions are automatic blockers regardless of the overall score.
- Sign-off authority is assigned to a person with the competence and accountability appropriate to the content's tier.
- The acceptance score is computed on a defined sampling basis, full, statistical, or risk-targeted, and the inference it supports is stated.
- A defined escalation path exists for disputes between translator, reviewer, and client, referencing the error typology and severity definitions.
- The gate decision and its basis, batch, criteria, sampling, score, blockers, outcome, rework list, and sign-off, are recorded for defensibility and trend analysis.
- No batch is signed off solely because the deadline arrived, and no batch meeting the threshold is rejected for not being perfect.
- The gate produces a defensible, consistent, consequence-aware decision rather than a formality or an aspiration.
