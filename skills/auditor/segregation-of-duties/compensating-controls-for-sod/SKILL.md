---
name: compensating-controls-for-sod.md
description: Use when the agent is evaluating compensating controls that mitigate segregation of duties conflicts or incompatible duty combinations, assessing owner review of bookkeeper activity, independent bank reconciliation, detail transaction review, master data change monitoring, or deciding whether a compensating control reduces an SoD conflict's residual risk below the significant deficiency threshold.
---

# Compensating Controls for Segregation of Duties

Where segregation of duties conflicts cannot be eliminated — common in smaller entities, single-person finance functions, or roles that inherently combine access — compensating controls are the mechanism that keeps residual risk acceptable. The recurring failure is to accept any nearby review as a compensating control without testing whether it actually offsets the specific conflict, whether it operates with the necessary independence and timeliness, and whether it would catch the fraud scheme the conflict enables. A compensating control that is weaker, later, or less independent than the segregation it replaces provides false comfort.

## Core Rules

### Match the compensating control to the specific conflict and the fraud scheme it enables

A compensating control is only valid against a defined conflict. Start by stating the exact incompatibility (e.g., "the AP clerk can both create vendors and release payments") and the specific fraud scheme it enables (ghost vendor payments). Then identify a control that would detect or prevent that exact scheme. A generic "management review" that does not specifically examine vendor master changes and payment authorisation does not compensate for this conflict. The compensation must be risk-specific, not categorical.

### Require independence as a defining feature

A compensating control loses its value if it is performed by the same person whose conflict it is meant to offset, or by someone who reports to them. The reviewer must be:

- organisationally independent of the conflicted role;
- sufficiently senior to challenge and act on findings;
- not themselves holding an incompatible combination with the role under review.

In owner-managed entities, the owner's personal review of the bookkeeper's activity is often the strongest available compensating control precisely because of this independence. In larger entities, internal audit or an independent controller's review may serve. Document who performs the control and confirm their independence.

### Assess the timeliness of the compensating control against the concealment window

A compensating control that operates long after the conflict-enabled transaction may be too late. For payment fraud, a monthly review of a bank statement discovers the loss after the funds are gone. Match the control's frequency to the speed at which the fraud scheme causes irreversible harm:

- cash and payment conflicts need near-real-time or at least pre-payment review;
- inventory conflicts may tolerate periodic (weekly/monthly) counts;
- journal-entry conflicts need review before the books are closed or very soon after.

A compensating control whose timing lets the fraud complete and be concealed before discovery is weak regardless of its design.

### Evaluate the depth and specificity of the review

A compensating control's strength depends on what the reviewer actually examines. Distinguish:

- **Skim review** — glancing at a summary or a total. Weak; catches only gross anomalies.
- **Exception review** — examining items flagged by a rule or threshold. Better, but only as good as the rule.
- **Detail review** — examining individual transactions, supporting documents, and master data changes. Strong, but feasible only at low volumes.
- **Full independent re-performance** — independently re-doing the reconciliation or recalculation. Strongest, but costly.

Match the depth to the risk. For a high-risk conflict (e.g., payment release with vendor creation), a summary-level review is insufficient; the compensating control must examine the specific transactions or master changes that the conflict exposes.

### Confirm the compensating control is documented and evidenced

A compensating control that operates only in the reviewer's memory provides no assurance. Require evidence:

- dated review notes, annotations, or sign-offs;
- records of items questioned and resolved;
- follow-up of exceptions to correction;
- periodic confirmation that the review occurred across the whole period, not just at year-end.

If the reviewer cannot produce evidence that the control operated throughout the period, treat the conflict as uncompensated for the un-evidenced periods.

### Test that the compensating control actually catches errors

The strongest evidence that a compensating control works is that it has caught something. Look for:

- instances where the review identified an error, a duplicate, a ghost vendor, or an unauthorised entry;
- the reviewer's response and the correction that followed;
- whether the control has ever escalated an issue to a higher level.

A compensating control that has never found anything across years of operation may be genuinely effective (the underlying process is clean) or may be cosmetic (the review is not rigorous enough to catch real errors). Independent testing — seeding or re-performance — resolves which.

### Treat person-dependence as a fragility, not a strength

Many SoD compensating controls depend on a single individual — often the owner in a small entity, or a particular senior reviewer in a larger one. This is a strength when that person is diligent, but a fragility when they are absent, distracted, or pressured. Assess:

- what happens when the reviewer is on leave — does the control lapse?
- is the reviewer's diligence documented and consistent, or variable?
- could pressure on the reviewer (e.g., to meet close deadlines) cause them to skip the review?

Person-dependent compensating controls should be supplemented by system-based or detective controls that do not rely on a single individual, where feasible.

### Factor compensating controls into deficiency severity, not into existence of the conflict

As with all compensating controls, the presence of mitigation does not erase the SoD conflict; it reduces the residual risk that the conflict will lead to material misstatement. Evaluate:

- without the compensating control, how large could the misstatement be?
- with the compensating control, what is the residual likelihood and magnitude?
- is the residual risk still above the significant deficiency or material weakness threshold?

Document the compensating control's contribution to severity explicitly. Do not let a compensating control silently convert a real conflict into "no issue."

## Common Traps

- **Accepting a generic "management review" as compensation** without confirming it examines the specific transactions or master data the conflict exposes.
- **Permitting the conflicted person (or their subordinate) to perform the compensating control**, destroying the independence that makes compensation possible.
- **Relying on a compensating control whose timing is too late** to prevent or promptly detect the fraud scheme (e.g., monthly review of irreversible cash fraud).
- **Accepting skim or summary-level review for a high-risk conflict** that requires detail examination to catch the relevant scheme.
- **Treating an undocumented, memory-only review as a control.** No evidence means no assurance for the periods not evidenced.
- **Assuming a compensating control is effective because it has never found anything**, when the real explanation may be that the review is too shallow to catch real errors.
- **Over-relying on a single diligent individual** whose absence or pressure would cause the control to lapse.
- **Letting the compensating control erase the conflict in the documentation** rather than reducing its assessed severity.
- **Failing to re-test the compensating control each period**, assuming last year's effectiveness persists without evidence.

## Self-Check

- For each SoD conflict I accepted as compensated, did I name the specific fraud scheme it enables and confirm the compensating control addresses that exact scheme?
- Is the compensating control performed by someone independent of the conflicted role, sufficiently senior, and not themselves holding an incompatible combination?
- Does the timing of the compensating control match the speed at which the fraud scheme causes irreversible harm?
- Is the depth of review (skim, exception, detail, re-performance) appropriate to the risk of the conflict?
- Is there documented evidence that the compensating control operated throughout the period, with records of exceptions questioned and resolved?
- Have I looked for evidence that the control has actually caught errors, or tested it independently (seeding, re-performance) to confirm it would?
- If the control depends on a single individual, have I assessed what happens when they are absent or pressured, and considered supplementary system-based controls?
- Did I factor the compensating control into the severity of the conflict rather than using it to claim the conflict does not exist, and is the residual risk below the relevant deficiency threshold?
