---
name: model_fairness_and_bias_auditing.md
description: Use when the agent is auditing a machine-learning model for bias or fairness, measuring group or individual fairness metrics (demographic parity, disparate impact, equalized odds, equal opportunity, calibration within groups), diagnosing bias sources in data, labels, features, or model training, choosing between pre-processing, in-processing, and post-processing interventions, or weighing fairness-accuracy tradeoffs for a model that affects people (credit, hiring, healthcare, criminal justice, insurance, content moderation, benefits allocation). Also covers the failure mode of fixing bias against a single metric while violating another, of trusting an aggregate fairness number that hides subgroup harm, of removing a protected attribute while proxies still carry the bias, and of treating fairness as a post-hoc checkbox rather than a constraint designed in from the start. Use for any model whose decisions affect people's access, cost, opportunity, or treatment, especially in regulated or high-stakes contexts where conservative, defensible, and human-overseen decisions are required.
---

# Model Fairness And Bias Auditing

A model that is accurate on average can be systematically wrong about a specific group of people, and in a system that allocates credit, jobs, healthcare, housing, or liberty, systematic error is harm. The judgment problem is that fairness is not a single property a model either has or lacks; it is a family of competing definitions, and the most common failure is to pick one metric, optimize it, declare the model "fair," and thereby hide the harm that the chosen metric does not measure. Demographic parity, equalized odds, equal opportunity, calibration within groups, and individual fairness are not reconcilable in general — there are formal impossibility results showing that no classifier can satisfy several of them simultaneously when base rates differ across groups. So "is this model fair?" is not a question with a yes/no answer; it is a question about which harms matter for this decision, which groups are protected, which metrics correspond to those harms, and what tradeoffs the business and its regulators are willing to accept. The discipline is to make those choices deliberately and transparently, to measure fairness sliced by subgroup and not only in aggregate, to locate where the bias actually enters (data, labels, features, model), and to intervene at the right stage — while never pretending that a single post-hoc metric resolves the question.

Agents tend to under-invest here because the visible path is easy: compute a fairness number, and if it crosses a threshold, ship. The harm appears when the metric hides a subgroup. A model that satisfies demographic parity overall can still deny every qualified member of a small subgroup if that subgroup is too small to move the aggregate. A model with a "protected attribute removed" can still discriminate via proxies (zip code, name, school) that encode the attribute. A model that passes equalized odds on the data it was audited on can fail on a subgroup that was scarce in the audit sample. And a model that is "fair" by one definition can be deeply unfair by another that the business chose not to measure. The judgment is to treat fairness as a set of constraints to be negotiated and verified across groups, to locate the bias source before intervening, to recognize the impossibility results rather than chasing a metric that cannot be jointly satisfied, and — for high-stakes decisions affecting people — to keep a human in the loop and to defer final authority rather than automating harm.

This skill covers group versus individual fairness, the major metrics and their tradeoffs, bias sources, the pre/intra/post-processing intervention spectrum, and the conservative posture appropriate to high-stakes decisions. It complements the training-data-quality skill (dataset bias and representativeness) and the model-interpretability-and-explanation skill (explaining decisions to affected users); here the focus is the specific judgment of measuring and mitigating disparate harm.

## Core Rules

### Define Fairness Against The Specific Harm And The Specific Decision, Not As A Generic Checkbox

Fairness is not a universal property; it is defined relative to a harm. Denying a loan, misdiagnosing a disease, rejecting a resume, and denying bail are different harms with different fairness-relevant questions. Define what fairness means for this decision before measuring anything.

- **Name the protected groups explicitly.** Fairness is measured across groups defined by attributes that warrant protection (race, gender, age, disability, and context-dependent others). Decide which groups are in scope for this model and this jurisdiction, and measure each — not "minorities" as a blob.
- **Name the harm the metric is meant to prevent.** Disparate harm takes different forms: being denied an opportunity you qualified for (a false negative in selection), being burdened with a cost others are not (a higher interest rate), being surveilled or policed more intensely (a higher false-positive rate), or receiving an uncalibrated risk score that misleads a downstream decision-maker. Different harms map to different metrics.
- **Do not collapse fairness to a single number.** A model can satisfy one fairness criterion and violate another that matters more. Report several metrics relevant to the harm, and explain the tradeoffs, rather than declaring fairness on the strength of one.
- **Involve the people the decision affects, where possible.** Fairness definitions embed value judgments about which harms matter; those judgments are more defensible when shaped by affected communities, domain experts, and (for regulated domains) legal review — not chosen by the modeling team alone.

### Measure The Major Group-Fairness Metrics And Understand What Each Protects Against

The standard group-fairness metrics each encode a different theory of justice. Know what each one measures and what it leaves unprotected, so the choice is deliberate.

- **Demographic parity (statistical parity): the model selects each group at the same rate.** It protects against under-selection of a group but ignores whether selected individuals are qualified; it is appropriate when the goal is equal representation and base-rate differences are themselves the product of historical bias, but it can force unqualified selections and is often inappropriate for risk-based decisions.
- **Disparate impact (the four-fifths rule): the selection rate for a protected group is at least 80% of the rate for the favored group.** A legal/regulatory threshold in some jurisdictions (e.g., US employment); useful as a flag but a blunt instrument that says nothing about error rates or calibration.
- **Equalized odds: the model has equal true-positive and false-positive rates across groups.** It protects against a group being burdened with more false alarms or denied more true opportunities; it is often the right metric when the costs of the two error types matter and differ in their effect on groups.
- **Equal opportunity: equal true-positive rates across groups.** A relaxation of equalized odds focused on ensuring qualified members of each group are selected at equal rates; appropriate when the primary harm is missing qualified candidates.
- **Calibration within groups: for a given predicted score, the actual outcome rate is the same across groups.** It protects against the score meaning different things for different groups; important when the score is consumed by a downstream decision-maker who will interpret it uniformly.
- **Choose by the harm, and report the tension.** Because these criteria conflict when base rates differ, choosing one usually worsens another; report the tradeoff explicitly rather than hiding it.

### Recognize The Impossibility Results — You Cannot Satisfy All Criteria At Once

There are formal results (Kleinberg, Chouldechova, and others) proving that when base rates differ across groups, no classifier can simultaneously satisfy calibration within groups and equalized odds (or demographic parity). Chasing all metrics at once is chasing the impossible.

- **When base rates differ, tradeoffs are unavoidable.** If the true positive rate differs across groups, demanding equalized odds will distort calibration, and demanding calibration will produce unequal error rates. This is mathematics, not a failure of effort.
- **Make the tradeoff explicit and own it.** Document which criterion was prioritized, which was sacrificed, and why, in terms of the harm. A silent tradeoff is an undefended one; an explicit tradeoff is a decision that can be reviewed.
- **Do not present a single satisfied metric as proof of fairness.** "We passed equalized odds" is not "the model is fair"; it is "we satisfied one criterion, possibly at the cost of another we did not measure." Report the criteria that were deprioritized and their resulting values.

### Locate Where The Bias Actually Enters Before Intervening

Bias is not a property the model invents; it enters somewhere in the pipeline, and the right intervention depends on where. Intervening at the wrong stage wastes effort and can make things worse.

- **Data bias: the training data under-represents or misrepresents a group.** A group scarce in the data is learned poorly; a group over-represented among negatives is learned as negative. Check representativeness and label distribution per group before training.
- **Label bias: the labels themselves encode historical discrimination.** "Was arrested," "was promoted," "defaulted on a loan" are labels shaped by past decisions that were themselves biased; a model that learns them faithfully reproduces the bias. This is the hardest source to fix, because the "ground truth" is contaminated.
- **Feature bias: features carry group information, directly or as proxies.** A protected attribute removed from the features can be fully reconstructed from proxies (zip code, name, school, purchase history). Removing the attribute does not remove the bias if the proxies remain.
- **Model/optimization bias: the model or the loss function amplifies disparity.** A model optimizing average accuracy can sacrifice a small group's accuracy to improve the majority's; the objective itself can encode the bias. Check whether the optimization target is the source.
- **Match the intervention to the source.** Data bias calls for more or better data; label bias calls for relabeling or redefining the target (hard, and sometimes impossible); feature bias calls for proxy auditing or feature suppression; model bias calls for fairness-aware objectives or constraints.

### Choose The Intervention Stage Deliberately: Pre-, In-, Or Post-Processing

Fairness interventions fall into three families, each with strengths and limits. Choose the stage that matches the bias source and the operational constraints.

- **Pre-processing: transform the data before training.** Reweighting, resampling, or repairing the dataset to reduce disparity (e.g., reweighing rows so groups are balanced by outcome). Pros: model-agnostic, preserves the chosen model. Cons: cannot fix label bias, and may not achieve the target criterion if the model re-introduces disparity.
- **In-processing: constrain or regularize the model during training.** Fairness-aware objectives, adversarial debiasing, constrained optimization that penalizes violations of equalized odds or demographic parity. Pros: can achieve the criterion more directly. Cons: model-specific, harder to implement and debug, and the constraint trades against accuracy in ways that must be measured.
- **Post-processing: adjust the model's outputs after training.** Group-specific thresholds that equalize error rates, or recalibration per group. Pros: model-agnostic, easy to apply to an existing deployed model. Cons: requires the protected attribute at decision time (which may be illegal or unavailable), and can feel like a patch over a deeper problem.
- **Weigh operational legality.** Some interventions require using the protected attribute at decision time, which is restricted or prohibited in some jurisdictions; a post-processing threshold keyed on race may be illegal even if it improves fairness. The legally available interventions constrain the choice.

### For High-Stakes Decisions Affecting People, Keep A Human In The Loop And Defer Final Authority

When a model affects access to credit, employment, healthcare, housing, education, or liberty, fairness metrics are necessary but not sufficient, and automation of the final decision is often inappropriate. The conservative posture is to treat the model as decision support, not decision-maker.

- **Keep meaningful human review for consequential adverse decisions.** A model can prioritize, score, or flag, but a person should review and be able to override adverse outcomes, with the authority and information to do so genuinely (not rubber-stamp).
- **Provide affected individuals with explanation, contestability, and recourse.** Someone denied by the model should be able to learn why (see the interpretability skill), to contest the decision, and to have a human re-examine it. This is increasingly a legal requirement, not just good practice.
- **Do not overclaim what the fairness audit establishes.** An audit that passes measured criteria establishes that those criteria were satisfied on the audited data for the measured groups — not that the model is "fair" in the full sense, nor that it will remain fair on unseen subgroups or under distribution shift.
- **Escalate beyond competence.** Fairness in regulated or high-stakes domains involves legal, ethical, and domain-specific judgment that exceeds a modeling workflow; defer to qualified reviewers (legal, compliance, domain experts, affected-community representatives) and document that the model's deployment is their decision, not the modeler's.

## Common Traps

### Fixing Bias Against One Metric While Violating Another

Optimizing a single fairness criterion (e.g., demographic parity) and declaring the model fair, while the intervention worsened equalized odds or calibration for the same groups. Report multiple relevant metrics and the explicit tradeoffs; do not declare fairness on one.

### Trusting An Aggregate Fairness Number That Hides Subgroup Harm

Reporting fairness across broad groups (e.g., "all minorities") when a small subgroup within that blob is being systematically harmed but is too small to move the aggregate. Measure per-subgroup; do not let a small disadvantaged group be averaged away.

### Removing The Protected Attribute And Assuming Bias Is Gone

Dropping the protected column (race, gender) from the features and concluding the model cannot discriminate, when proxies (zip code, name, school, device) fully reconstruct the attribute and carry the bias. Audit for proxies; removal of the attribute is not removal of the bias.

### Label Bias Reproduced As "Ground Truth"

Training on labels that encode historical discrimination (arrests, promotions, loan approvals shaped by past biased decisions) and treating the model's fidelity to them as correctness, when it is faithfully reproducing the bias. Inspect the label's provenance; contaminated labels cap how fair the model can be.

### Chasing An Impossible Combination Of Criteria

Demanding simultaneous calibration within groups, equalized odds, and demographic parity when base rates differ across groups — a combination provably unsatisfiable — and treating the inability to achieve it as a tuning failure. Recognize the impossibility results; choose and document the tradeoff.

### Post-Processing That Requires The Protected Attribute Illegally

Applying group-specific thresholds to equalize error rates, requiring the protected attribute at decision time in a jurisdiction where its use is restricted, trading a fairness gain for a legal violation. Check the legality of using the attribute before choosing the intervention stage.

### Fairness As A Post-Hoc Checkbox

Adding a fairness metric at the end of the pipeline to satisfy a requirement, after a black-box model was already built on biased data, rather than designing fairness as a constraint from data collection through deployment. Design fairness in; locate the bias source before intervening.

### Automating A High-Stakes Adverse Decision

Letting the model make the final decision on credit, hiring, healthcare, or liberty without meaningful human review, contestability, or recourse, because the fairness metrics passed. Keep a human in the loop for consequential adverse decisions; the model is decision support, not the decision-maker.

## Self-Check

- [ ] Fairness was defined against the specific harm and decision (which harms matter, which protected groups are in scope for this jurisdiction), not as a generic checkbox, and the relevant metrics were chosen because they map to those harms.
- [ ] Multiple group-fairness metrics relevant to the harm were measured (demographic parity / disparate impact, equalized odds / equal opportunity, calibration within groups), and the tradeoffs among them are reported explicitly rather than a single satisfied metric being presented as proof of fairness.
- [ ] The impossibility results were recognized: where base rates differ across groups, the choice to satisfy one criterion at the cost of another is documented and owned, not treated as a tuning failure.
- [ ] Performance and fairness are measured sliced by subgroup (not only broad groups), so a small disadvantaged group is not averaged away in an aggregate number.
- [ ] The bias source was located before intervening — data (under-representation), labels (historical discrimination encoded as ground truth), features (proxies for the protected attribute), or model (objective sacrificing a small group) — and the intervention matches the source.
- [ ] Removing the protected attribute was not assumed to remove bias: proxies were audited (zip code, name, school, device, purchase history) and found to carry or not carry the group information.
- [ ] The intervention stage (pre-processing, in-processing, post-processing) was chosen deliberately against the bias source and operational constraints, and the legality of using the protected attribute at decision time was confirmed for the jurisdiction.
- [ ] For high-stakes decisions affecting people, meaningful human review, contestability, explanation, and recourse are in place for adverse outcomes, and the model is treated as decision support with final authority deferred to qualified human reviewers — not as an automated decision-maker.
- [ ] The highest-risk cases were verified — a subgroup harmed despite a passing aggregate metric, a proxy reconstructing a removed attribute, contaminated labels capping achievable fairness, an intervention that required the attribute illegally, and an automated adverse decision without recourse — not only the clean single-metric-passes path.
