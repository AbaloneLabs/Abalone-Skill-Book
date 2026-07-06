---
name: metadata_quality_assurance_workflows.md
description: Use when the agent is building quality assurance into metadata workflows, establishing review and validation checkpoints, configuring automated quality checks, training metadata creators on quality, or sustaining metadata quality over the life of a collection.
---

# Metadata Quality Assurance Workflows

Quality assessment finds problems; cleanup fixes them after the fact. Quality assurance, QA, prevents problems from entering the collection in the first place. QA is the set of workflow design choices, validation checks, review checkpoints, training practices, and feedback loops that keep metadata quality high as a continuous property rather than something restored by periodic cleanup. Without QA, every batch of new metadata reintroduces the same defects, and cleanup becomes a permanent treadmill. With QA, quality is built in at creation, errors are caught before publication, creators learn from feedback, and the collection's quality is sustained without constant remediation. Designing QA is a systems task: it requires understanding where errors originate, where in the workflow they can be caught most cheaply, what can be automated versus what needs human review, and how to sustain attention to quality over time without making it a burden that creators circumvent.

Use this skill when designing metadata quality assurance workflows, establishing review and validation checkpoints, configuring automated checks, training creators, or sustaining quality over a collection's life. The goal is to prevent the agent from relying on post-hoc cleanup, automating checks that creators bypass, omitting human review for judgment-dependent quality, or letting QA atrophy after initial setup.

## Core Rules

### Build Quality Assurance Into Creation, Not After It

The cheapest place to fix a metadata error is at the moment of creation, while the creator has the object and context in mind. QA should make the right action the easy action.

Creation-time QA:

- input forms that enforce mandatory fields and valid formats;
- controlled vocabulary lookups that prevent free-text drift;
- inline validation that flags errors before the record is saved;
- templates and defaults that guide consistent population;
- real-time feedback on common mistakes.

When QA is a separate review step after creation, errors are more expensive to fix and more likely to be deferred or skipped. Build checks into the creation interface.

### Configure Automated Validation For Mechanical Quality

Much of metadata quality is mechanical: mandatory fields populated, controlled vocabularies used, formats correct, cardinality respected. Automate these checks.

Automated validation:

- mandatory field enforcement at save;
- controlled vocabulary validation against authority sources;
- format validation for dates, identifiers, and structured fields;
- cardinality checks for single versus repeatable elements;
- duplicate detection on identifiers and key access points.

Configure validation to match the application profile. Automated checks catch the bulk of mechanical errors consistently and without burdening human reviewers.

### Reserve Human Review For Judgment-Dependent Quality

Automated checks cannot assess accuracy, appropriateness of subject assignment, or quality of descriptions. Reserve human review for these.

Human review targets:

- accuracy of transcribed and assigned values against the object;
- appropriateness and specificity of subject headings;
- clarity and specificity of descriptions;
- correct disambiguation of names and authorities;
- consistency of judgment across complex cases.

Route human review to trained reviewers, and use sampling for high-volume collections where full review is not feasible. Document what human review covers so gaps are explicit.

### Establish Review Checkpoints In The Workflow

QA needs defined points where review happens, so it is not optional or ad hoc. Build checkpoints into the workflow.

Checkpoints:

- at record completion, before a record leaves the creator;
- at batch or collection publication, before metadata goes live;
- at vendor or batch-load acceptance, before external data enters the system;
- at periodic intervals for already-published collections.

Define what each checkpoint checks, who reviews, and what happens when problems are found. Checkpoints make QA a structural part of the workflow rather than a personal habit.

### Provide Timely, Specific Feedback To Creators

QA only improves future quality if creators learn from it. Feedback must be timely and specific enough to change behavior.

Feedback practice:

- return review findings promptly, while the work is fresh;
- be specific about what was wrong and how to fix it;
- explain the principle, not just the correction, so the lesson generalizes;
- track recurring errors to identify training needs;
- recognize improvement to reinforce good practice.

Vague or delayed feedback does not change behavior. Specific, timely feedback turns QA into a learning system.

### Train Creators On Quality Standards And Rationale

Creators cannot meet quality standards they do not understand. Training is a QA foundation.

Training:

- train all creators on the application profile, element semantics, and vocabularies;
- explain the rationale for rules so creators can handle edge cases;
- provide examples of correct and incorrect metadata;
- offer refresher training when standards or profiles change;
- document common errors and how to avoid them.

Training is not one-time onboarding. Sustain it as standards, tools, and collections evolve.

### Monitor Quality Metrics Continuously

QA effectiveness must be measured. Continuous monitoring catches quality drift before it becomes a crisis.

Monitoring:

- track quality metrics over time, completeness, consistency, conformance;
- segment metrics by creator, collection, or batch to locate drift;
- set thresholds that trigger investigation when quality drops;
- report metrics to creators and management regularly;
- correlate metrics with workflow or staffing changes.

Monitoring turns QA from an assumption into evidence. Drift detected early is far cheaper to correct.

### Sustain QA Over The Collection's Life

QA is not a project with an end date. Collections evolve, standards change, staff turn over, and quality can erode if QA is not sustained.

Sustainability:

- document the QA workflow so it survives staff turnover;
- budget QA as a recurring activity, not a one-time investment;
- review and update validation rules as the profile evolves;
- retrain creators after changes to standards or tools;
- periodically audit whether QA is catching the right problems.

QA that is set up once and never reviewed becomes a ritual that checks the wrong things. Maintain it as a living system.

## Common Traps

### Relying On Post-Hoc Cleanup Instead Of Creation-Time QA

Cleanup after the fact is more expensive and recurring. Build QA into creation.

### Automated Checks Creators Can Bypass

Validation that can be skipped or overridden is ineffective. Make checks enforceable.

### Omitting Human Review For Judgment Quality

Automated checks miss accuracy and appropriateness. Reserve human review for judgment.

### Ad Hoc Review Without Checkpoints

Optional review gets skipped under pressure. Build checkpoints into the workflow.

### Vague Or Delayed Feedback

Feedback that does not teach does not improve quality. Be specific and timely.

### One-Time Training

Standards and tools change. Sustain training over time.

### No Quality Monitoring

Unmeasured quality drifts unnoticed. Monitor metrics continuously.

### QA Set Up Once And Never Maintained

Unreviewed QA checks the wrong things over time. Sustain and update it.

## Self-Check

- Is quality assurance built into the creation interface through enforced fields, vocabularies, and inline validation?
- Is automated validation configured to match the application profile for mandatory fields, formats, vocabularies, and cardinality?
- Is human review reserved for judgment-dependent quality like accuracy, subject appropriateness, and description clarity?
- Are review checkpoints established at completion, publication, vendor acceptance, and periodic intervals?
- Is feedback to creators timely, specific, explanatory, and tracked for recurring patterns?
- Are creators trained on standards, rationale, and examples, with refresher training after changes?
- Are quality metrics monitored continuously, segmented, and reported to detect drift?
- Is the QA workflow documented, budgeted, and sustained over the collection's life with periodic audits?
