---
name: sample_evaluation_tools.md
description: Use when the agent is evaluating sample results, computing deviation rates and projected misstatement, calculating upper deviation limits and precision, comparing sample outcomes to tolerable limits, distinguishing anomalous from projected errors, or documenting the evaluation and conclusion of an audit sample.
---

# Sample Evaluation Tools

Sample evaluation is where the raw results of testing become an audit conclusion, and it is where small computational or conceptual errors have outsized effects. A deviation rate computed on the wrong base, a projected misstatement that ignores the sampling method, or a precision adjustment applied incorrectly can flip a conclusion from acceptable to unacceptable or vice versa. Agents often reach for the evaluation tool's output without checking that the inputs, the method, and the comparison base match the sampling plan, treating the printed result as the answer. The harm this prevents is a conclusion that is mathematically precise but logically wrong, which can lead to issuing or withholding an opinion on the basis of an evaluation that does not actually match the test that was performed.

Use this skill whenever sample results are evaluated, whenever deviation rates or projected misstatement are computed, and whenever a conclusion is drawn by comparing results to tolerable limits. The goal is an evaluation whose method matches the sampling plan, whose computation is verifiable, and whose conclusion is stated in defensible statistical terms.

## Core Rules

### Evaluate Using The Method That Generated The Sample

The evaluation must match the sampling method. Attribute samples are evaluated with deviation rates and upper deviation limits; monetary-unit samples with projected misstatement and precision; variables samples with classical projection and confidence intervals. Mixing methods produces a number that has no valid interpretation.

Match evaluation to method by:

- confirming the evaluation tool matches the sampling approach;
- using deviation rate and upper limit for attribute sampling;
- using projected misstatement and precision for monetary-unit sampling;
- using mean or ratio estimation for variables sampling;
- never applying a variables formula to an attribute sample or vice versa.

### Compute The Deviation Rate Or Projection On The Correct Base

The base determines the result. A deviation rate computed on the sample size differs from one computed on the population, and a projection based on the wrong sampling unit misstates the misstatement.

Confirm the base by checking:

- the denominator for deviation rate is the sample size tested;
- the sampling unit for projection matches the selection unit;
- monetary-unit projections use the recorded value as the base;
- stratified samples are evaluated within and across strata correctly;
- items not tested are excluded from the denominator only with documented reason.

### Apply Precision And Upper Limits Correctly

Precision and upper limits exist because a sample estimates, with uncertainty, the true population error. Omitting precision understates risk; applying it twice overstates it. Apply the limit that matches the method and the risk of overstatement or understatement.

Apply limits by:

- using the upper deviation limit for attribute sampling;
- using the upper limit on misstatement for monetary-unit sampling;
- adding precision to projected misstatement for the upper bound;
- confirming whether a one-sided or two-sided interval is appropriate;
- never comparing raw projected misstatement to tolerable error without precision.

### Compare The Upper Limit To The Tolerable Error

The decision rule compares the upper limit, not the observed error, to the tolerable error. A sample with zero observed deviations can still exceed tolerable error once the upper limit is applied.

Apply the decision rule by:

- comparing the upper limit to the tolerable error or deviation rate;
- concluding acceptable only when the upper limit is below tolerable;
- concluding unacceptable when the upper limit exceeds tolerable;
- documenting the comparison and the resulting conclusion;
- not relaxing the tolerable error to make the result fit.

### Distinguish Anomalous Errors From Projected Errors

Anomalous errors are isolated, non-representative misstatements that are not projected to the population, while projected errors are representative and must be projected. Misclassifying one as the other distorts the evaluation.

Classify each error by:

- determining whether the error is isolated to one item or representative;
- projecting representative errors to the population;
- treating anomalous errors as specific to the item, not projected;
- documenting the basis for the anomalous classification;
- considering whether an apparently anomalous error signals a control issue.

### Handle Zero-Error And High-Error Samples Honestly

A zero-error sample does not prove the population is correct, only that errors are unlikely to exceed the tolerable rate at the chosen confidence. A high-error sample may require expanding the sample or revising the risk assessment.

Handle edge results by:

- stating zero-error conclusions in statistical terms with the upper limit;
- not claiming the population is free of error from a clean sample;
- following the plan's rule for expanding sample size on high errors;
- not discarding errors to reach an acceptable result;
- escalating persistent high-error results to reconsider the risk assessment.

### Document The Evaluation And Conclusion Completely

The evaluation must be documented so a reviewer can recompute the result and understand the conclusion. A conclusion without the computation is an assertion, not evidence.

Document:

- the sampling method and parameters used;
- the number of items tested and errors found;
- the deviation rate or projected misstatement;
- the precision or upper limit applied;
- the comparison to tolerable error and the decision rule;
- the conclusion and its effect on the audit;
- the preparer and reviewer sign-off.

### Reconcile The Evaluation To The Sampling Plan

The evaluation should be traceable back to the plan: same population, same method, same definition of error, same decision rule. Any deviation must be explained.

Reconcile by:

- confirming the population matches the one sampled;
- confirming the error definition matches the plan;
- confirming the decision rule is the one fixed before selection;
- explaining any items excluded from evaluation;
- confirming the conclusion follows from the documented rule.

### Carry The Conclusion Through To The Audit Response

The evaluation is not the end point; its conclusion must drive a documented audit response. A sample that exceeds tolerable error demands a specific response, and a sample that passes still carries implications for reliance and further procedures. Leaving the result as a number without an action leaves the test incomplete.

Translate the conclusion into action by:

- stating whether the result supports the planned level of reliance;
- defining additional procedures when the upper limit exceeds tolerable error;
- considering whether errors indicate a control deficiency to report;
- assessing whether errors are concentrated in a stratum or segment needing separate testing;
- linking the conclusion to the overall opinion and materiality decisions;
- documenting management's response to identified errors where relevant.

## Common Traps

### Comparing Observed Error To Tolerable Error

Observed error almost always understates the true population error. Comparing the raw observed rate or amount to tolerable error ignores sampling risk and can produce a false comfort conclusion. Always compare the upper limit.

### Using The Wrong Evaluation Method For The Sample

Applying an attribute formula to a monetary-unit sample, or a variables formula to an attribute sample, yields a number with no valid meaning. Match the evaluation tool strictly to the sampling method.

### Projecting Anomalous Errors To The Population

Projecting an error that is specific to one item inflates the projected misstatement and can flip a conclusion. Classify each error as anomalous or representative with documented reasoning.

### Discarding Errors To Reach An Acceptable Result

Removing errors as "exceptions" or "not really errors" to bring the result under tolerable is a form of result manipulation. Evaluate all errors against the documented error definition.

### Ignoring Precision Entirely

Reporting projected misstatement without precision makes the result look more precise than it is. Precision reflects sampling risk and must be included in the upper limit comparison.

### Relaxing Tolerable Error After Seeing Results

Widening tolerable error once the result is known undermines the entire test. The tolerable error is fixed in the plan and must not be adjusted to fit the outcome.

### Claiming Certainty From A Clean Sample

A zero-error sample still carries an upper limit. Stating that the population is correct, rather than that errors are unlikely to exceed the tolerable rate, overstates what the sample supports.

### Forgetting To Reconcile Evaluation To Plan

An evaluation that drifts from the plan, in population, method, or error definition, is not the same test that was designed. Reconcile the evaluation back to the plan every time.

### Stopping At The Number Without An Audit Response

Computing a result and recording it is not the end of the test. A conclusion that exceeds tolerable error, or that reveals a cluster of errors in one segment, demands additional procedures or a revised risk assessment. Leaving the evaluation as a standalone figure leaves the test incomplete.

## Self-Check

- Does the evaluation method match the sampling method that generated the sample?
- Is the deviation rate or projected misstatement computed on the correct base and sampling unit?
- Is precision or the upper limit applied and included in the comparison to tolerable error?
- Is the decision rule comparing the upper limit, not the observed error, to tolerable error?
- Are anomalous errors distinguished from projected errors with documented reasoning?
- Is a zero-error conclusion stated in statistical terms rather than as certainty?
- Are all errors evaluated against the documented error definition, with none discarded to fit?
- Has the tolerable error been kept as fixed in the plan, without post-hoc relaxation?
- Does the documentation allow a reviewer to recompute the result from inputs to conclusion?
- Is the evaluation reconciled to the sampling plan in population, method, and decision rule?
- Is the conclusion's effect on the audit clearly stated and signed off?
- Does the evaluation translate into a documented audit response, including any additional procedures where the upper limit exceeds tolerable error?
- Are stratified results evaluated within and across strata, and reconciled to the overall conclusion?
- Is the basis for classifying each error as anomalous or representative documented and supportable?
- Are partial errors measured and weighted consistently with the documented error definition?
- Is the evaluation tool's output independently checked rather than accepted at face value?
