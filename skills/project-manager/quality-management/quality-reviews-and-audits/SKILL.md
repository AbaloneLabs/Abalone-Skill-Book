---
name: quality_reviews_and_audits.md
description: Use when the agent is planning quality reviews or audits, designing review processes like peer review or design review, determining audit scope and criteria, ensuring reviews are effective rather than perfunctory, or diagnosing why reviews failed to catch defects that escaped to later phases or to the customer.
---

# Quality Reviews And Audits

Reviews and audits are structured examinations of work products or processes to verify quality and compliance. Reviews examine work products (designs, code, documents) for defects and completeness; audits examine processes for compliance with standards. The judgment problem is that reviews and audits are often performed as perfunctory ceremonies that consume time without catching defects, because they lack clear criteria, trained reviewers, or a non-normative culture. A review that rubber-stamps work adds cost without value; an audit that checks compliance without improving the process is bureaucracy without benefit. The skill is designing reviews and audits that actually find problems and drive improvement.

Use this skill before establishing review or audit processes, before conducting a design or peer review, before planning a quality audit, or when reviews have failed to catch escaping defects. The goal is to prevent the agent from conducting perfunctory reviews, from reviewing without clear criteria, from confusing process compliance with quality outcomes, or from letting reviews become blame rituals that suppress candor.

## Core Rules

### [ ] Define Clear Review Objectives And Criteria

A review without objectives and criteria is a discussion, not a quality check. Define what the review is examining (requirements compliance, design soundness, code defects) and the criteria for acceptance. Clear criteria make the review focused and its results actionable.

- [ ] Define the review's objectives: what is being examined and why.
- [ ] Establish acceptance criteria against which the work product is judged.
- [ ] Provide reviewers with checklists or standards to guide the review.
- [ ] Ensure criteria are specific enough to identify defects, not just impressions.

### [ ] Choose The Right Review Type For The Work

Different review types suit different purposes. Informal peer reviews provide quick feedback; technical reviews examine correctness; inspections use formal, data-driven defect detection; management reviews assess status and direction. Match the type to the work product and the risk.

- [ ] Use informal peer review for quick feedback on early or low-risk work.
- [ ] Use technical reviews for design and architecture correctness.
- [ ] Use inspections for formal, rigorous defect detection on high-risk work.
- [ ] Use management reviews for status, direction, and go/no-go decisions.

### [ ] Use Trained, Independent Reviewers

Reviewers who created the work cannot see their own defects; reviewers unfamiliar with the standards cannot judge compliance. Effective reviews use trained reviewers who are independent of the work being reviewed. Independence and training are the determinants of review effectiveness.

- [ ] Use reviewers independent of the work being reviewed.
- [ ] Ensure reviewers are trained in review techniques and standards.
- [ ] Rotate reviewers to bring fresh perspectives.
- [ ] Avoid self-review or review by the direct author only.

### [ ] Prepare Reviewers Before The Review

Reviewers who see the material for the first time in the meeting cannot review effectively. Distribute materials in advance and require pre-review preparation. The meeting should confirm and discuss findings, not introduce the material.

- [ ] Distribute review materials in advance with adequate preparation time.
- [ ] Require reviewers to prepare and note findings before the meeting.
- [ ] Use the meeting to confirm, discuss, and prioritize findings, not to read material.
- [ ] Track preparation to ensure reviewers come ready.

### [ ] Focus On Finding Defects, Not Assigning Blame

A review culture that assigns blame for defects suppresses candor and hides problems. Reviews should focus on finding and fixing defects, treating them as process failures, not personal failures. A non-punitive culture produces more honest and effective reviews.

- [ ] Frame reviews as defect-finding, not blame-assigning.
- [ ] Treat defects as process failures, not personal failures.
- [ ] Encourage candor about defects without fear of punishment.
- [ ] Avoid public criticism of authors during reviews.

### [ ] Track Review Findings To Resolution

Findings identified in a review must be tracked to resolution, with owners and due dates. Findings that are noted but not fixed waste the review. Closure tracking ensures the review produces actual quality improvement.

- [ ] Log all review findings with owners and due dates.
- [ ] Track findings to resolution and verify fixes.
- [ ] Re-review significant fixes to confirm they resolved the defect.
- [ ] Avoid letting findings accumulate without closure.

### [ ] Measure Review Effectiveness

Review effectiveness is measured by the defects found and, critically, by the defects that escape despite the review. Track review metrics: defects found per review, defect escape rate, review effort. Measurement identifies whether reviews are adding value or are perfunctory.

- [ ] Track defects found per review and defect escape rate.
- [ ] Measure review effort against defects found to assess efficiency.
- [ ] Compare review effectiveness across review types and reviewers.
- [ ] Use effectiveness data to improve the review process.

### [ ] Conduct Audits For Process Compliance And Improvement

Audits examine whether processes are being followed and whether they are effective. Unlike reviews, which examine products, audits examine the process. Use audits to verify compliance with quality standards and to identify process improvement opportunities.

- [ ] Define audit scope, criteria, and frequency in advance.
- [ ] Examine both compliance (is the process followed?) and effectiveness (does it work?).
- [ ] Use audit findings to drive process improvement, not just compliance reporting.
- [ ] Conduct audits by independent, trained auditors.

### [ ] Distinguish Audit Findings From Review Findings

Audit findings address process gaps; review findings address product defects. Mixing them confuses response. Process gaps require process change; product defects require correction. Keep the two separate in tracking and response.

- [ ] Track audit findings (process) separately from review findings (product).
- [ ] Respond to audit findings with process changes.
- [ ] Respond to review findings with product corrections.
- [ ] Avoid conflating the two response types.

### [ ] Use Review And Audit Results For Continuous Improvement

The aggregate of reviews and audits reveals systemic quality issues. Analyze patterns across reviews and audits to identify recurring defect types, process gaps, and improvement opportunities. Individual reviews fix individual defects; aggregate analysis improves the system.

- [ ] Analyze patterns across reviews and audits for systemic issues.
- [ ] Identify recurring defect types and their process causes.
- [ ] Feed findings into continuous improvement and process change.
- [ ] Verify that process changes reduced the targeted defect types.

## Common Traps

### [ ] Perfunctory Reviews

Conducting reviews as ceremonies that rubber-stamp work without finding defects.

### [ ] No Clear Criteria

Reviewing without objectives or acceptance criteria, producing impressions not findings.

### [ ] Self-Review Or Untrained Reviewers

Using authors or untrained reviewers who cannot see defects.

### [ ] No Preparation

Introducing material in the meeting rather than preparing in advance.

### [ ] Blame Culture

Punishing authors for defects, suppressing candor and hiding problems.

### [ ] Findings Not Tracked

Noting findings without tracking them to resolution, wasting the review.

### [ ] No Effectiveness Measurement

Conducting reviews without measuring whether they find defects and prevent escapes.

### [ ] Compliance Without Improvement

Auditing for compliance without using findings to improve the process.

## Self-Check

- [ ] Are review objectives and acceptance criteria defined before the review?
- [ ] Is the review type matched to the work product and risk level?
- [ ] Are reviewers trained, independent, and not the authors of the work?
- [ ] Are materials distributed in advance with required preparation?
- [ ] Does the review culture focus on finding defects rather than assigning blame?
- [ ] Are review findings tracked to resolution with owners and due dates?
- [ ] Is review effectiveness measured by defects found and escape rate?
- [ ] Do audits examine both process compliance and effectiveness for improvement?
- [ ] Are audit findings (process) distinguished from review findings (product)?
- [ ] Are aggregate review and audit results analyzed for systemic improvement?
