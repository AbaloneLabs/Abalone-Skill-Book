---
name: predictive-analytics.md
description: Use when the agent is using predictive models, regression, or machine-learning techniques to form expectations for analytical procedures, evaluating model suitability and data inputs, assessing whether a model's prediction is precise enough to be substantive evidence, understanding model limitations and drift, or deciding when predictive analytics can supplement or replace traditional analytical procedures.
---

# Predictive Analytics in Audit

Predictive analytics uses statistical models — regression, time-series, and increasingly machine learning — to generate expectations against which recorded amounts are compared. When the relationship is genuinely predictable and the model is well built, predictive analytics can produce tighter, more defensible expectations than traditional ratio analysis, especially for large, complex populations. When the model is opaque, the inputs are unreliable, or the relationship is not actually predictable, the output looks precise but has no real evidential value. The discipline is to understand the model well enough to judge whether its predictions are precise and reliable enough to support an audit conclusion, and never to treat a model as a black box whose output is self-justifying.

## Core Rules

### Judge predictive analytics by the same standard as any analytical procedure

A predictive model is a way of forming an expectation; the substantive standard is unchanged. Before relying on a model's output, confirm:

- the underlying relationship is predictable enough to support a precise expectation;
- the model captures the relationship with sufficient precision to detect a material misstatement;
- the inputs are reliable, complete, and appropriate;
- the differences between prediction and recorded amount are investigated and corroborated.

A model does not relax these standards; it is a tool for meeting them. If the model cannot meet them, it is not providing substantive evidence regardless of its sophistication.

### Understand the model well enough to assess its suitability

You do not need to be the model's author, but you must understand it well enough to judge whether it is fit for audit purposes. For any model relied upon, establish:

- what relationship it models (which drivers predict which outcome);
- what type of model it is (linear regression, time-series, tree-based, neural) and whether that type is appropriate to the relationship;
- how it was trained or calibrated, on what data, and over what period;
- its known limitations, assumptions, and failure modes.

If the model is a black box that no one can explain, its output cannot be defended as audit evidence. Prefer simpler, interpretable models where they meet the precision requirement; complexity is justified only where it materially improves precision and the model remains understandable.

### Validate the model's predictive performance on relevant data

A model's claim to precision must be validated, not assumed. Confirm:

- how well the model predicts on data it was not trained on (out-of-sample performance);
- whether performance holds across the segments, periods, and conditions relevant to this audit;
- the typical size of prediction error relative to the materiality threshold the procedure must detect;
- whether performance has degraded over time (model drift), especially if the business or data has changed.

A model that performed well historically but has drifted may no longer be precise enough; validation should be current and relevant, not a historical claim.

### Assess the reliability and appropriateness of inputs

A model's output is only as reliable as its inputs. Confirm:

- inputs are complete, accurate, and obtained independently of the account being tested where possible;
- inputs are measured on a consistent basis across the period;
- inputs are at the right granularity (monthly drivers for monthly predictions);
- any management-provided inputs (forecasts, assumptions) are corroborated.

Models fed by management forecasts or by data from the same system as the account under test carry circularity risk: the prediction and the recorded amount share inputs, so they will tend to agree regardless of whether the recorded amount is correct. Independent inputs are essential for the prediction to be a meaningful check.

### Calibrate precision to materiality and risk

The model's prediction error must be smaller than the misstatement the procedure is meant to detect. For a material, high-risk assertion, a model whose typical error is 8% cannot detect a 5% misstatement and provides no substantive evidence for that threshold. Establish the model's precision (prediction interval or typical error) and confirm it is finer than the relevant materiality-adjusted threshold. If it is not, the model can support risk assessment or broad reasonableness but not a substantive conclusion at that threshold.

### Investigate differences with the same rigour as traditional analytical procedures

Differences between the model's prediction and the recorded amount must be investigated and corroborated, not explained away. Require:

- specific, quantified causes for material differences, verified by the auditor;
- confirmation that the cause is real and not a symptom of the model's weakness;
- correction of any identified errors.

A model that routinely produces differences "explained" by vague narratives is not being used as evidence; it is producing noise that is being rationalised. The investigation standard is the same as for any analytical procedure.

### Watch for overfitting, drift, and regime change

Three technical failures undermine predictive models:

- **Overfitting**: the model fits historical data so closely that it captures noise rather than the underlying relationship, and predicts poorly on new data.
- **Drift**: the underlying relationship changes over time (new products, new markets, new systems) so the model trained on old data no longer applies.
- **Regime change**: a structural break (acquisition, divestiture, pandemic, regulatory change) invalidates the model's assumptions entirely.

For each model relied upon, assess whether these failures are present or likely, and validate against current data. A model that worked last year may not work this year if the business has changed.

### Use predictive analytics to supplement, and selectively to replace, traditional procedures

Predictive analytics is most powerful as a supplement: it can cover large populations, identify anomalies for detail testing, and provide tighter expectations than ratio analysis for predictable relationships. It can replace traditional analytical procedures where it is demonstrably more precise and equally well understood. It should not replace tests of details for high-risk, judgement-heavy, or non-predictable assertions, where the model's precision and the relationship's predictability cannot support the conclusion.

### Document the model, the validation, and the conclusion

For any model relied upon as substantive evidence, document:

- the model type, its drivers, and how it was trained;
- the validation of its performance on relevant, current data;
- the inputs, their source, and their independence from the account under test;
- the precision and how it compares to the materiality threshold;
- the differences identified, their investigation and corroboration;
- the conclusion and how the model's evidence combines with other procedures.

A conclusion resting on a model without this documentation is indefensible. The documentation is what makes the model auditable rather than a black box.

## Common Traps

- **Treating a model as a black box** whose output is self-justifying, without understanding the relationship, the model type, or the limitations.
- **Assuming sophistication equals precision** — a complex model that no one can explain provides weaker, not stronger, evidence than a simple interpretable one.
- **Relying on a model whose precision is coarser than the misstatement threshold** it is meant to detect, so a materially wrong answer would still "pass."
- **Feeding the model with inputs from the same system or from management forecasts**, creating circularity where prediction and recorded amount share inputs.
- **Skipping validation on current, relevant data**, assuming historical performance persists despite business change.
- **Accepting narrative explanations of differences** without specific, quantified, corroborated causes.
- **Missing overfitting, drift, or regime change** that has silently invalidated the model since it was built.
- **Using predictive analytics to replace detail testing** for high-risk or non-predictable assertions where the model cannot support the conclusion.
- **Failing to document the model, validation, inputs, precision, and investigation**, leaving the conclusion indefensible.

## Self-Check

- Am I judging the model by the same substantive standard as any analytical procedure — predictable relationship, precise expectation, reliable inputs, corroborated differences?
- Do I understand the model well enough — relationship, type, training, limitations — to assess its suitability, or is it a black box?
- Has the model's predictive performance been validated on current, relevant, out-of-sample data, with typical error compared to the materiality threshold?
- Are the inputs complete, accurate, consistently measured, and independent of the account under test, avoiding circularity?
- Is the model's precision finer than the misstatement threshold it must detect for this assertion?
- For material differences, did I investigate and corroborate specific, quantified causes, not narratives?
- Have I assessed overfitting, drift, and regime change, and validated against current data if the business has changed?
- Am I using the model to supplement traditional procedures, and to replace them only where it is demonstrably more precise and well understood?
- Is the model, its validation, inputs, precision, differences, and conclusion documented so the evidence is auditable?
