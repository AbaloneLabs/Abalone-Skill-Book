---
name: automation_design.md
description: Use when the agent is designing automated audit procedures, building scripts or analytics to test full populations, deciding which procedures to automate versus perform manually, validating the completeness and accuracy of data extracts, or ensuring automated testing does not introduce errors, bias, or false assurance through flawed logic.
---

# Automation Design

Automation promises to expand audit coverage, reduce manual effort, and enable testing of entire populations rather than samples. These benefits are real, but automation also introduces new failure modes that manual testing does not have. A script that tests every transaction provides false assurance if the underlying data extract is incomplete, if the logic misclassifies exceptions, or if the script silently skips records it cannot parse. Automation does not inherit the reliability of the auditor who wrote it; it inherits the reliability of its weakest assumption. An automated procedure that runs without error is not the same as one that runs correctly. The auditor designing automation must treat the script, the data, and the logic as evidence to be validated, not as a black box to be trusted.

Use this skill when building or reviewing automated audit procedures, scripting data analytics, designing continuous audit scripts, evaluating tool selection, or validating that automated results are complete and accurate. The goal is to ensure automation genuinely improves assurance rather than substituting one form of untested trust for another.

## Core Rules

### Decide What To Automate Based On Suitability, Not Convenience

Not every procedure benefits from automation. Automation is most valuable where the population is large, the rule is well-defined, the test is repetitive, and the data is reliable and accessible. It is least valuable where judgment, interpretation, or physical observation is central.

Assess suitability by asking:

- is the test rule-based and deterministic, or does it require professional judgment per item?
- is the population large enough that automation saves meaningful effort?
- is the data available in a reliable, structured form?
- can the logic be expressed precisely, or does it require interpretation that resists coding?
- will the effort to build, validate, and maintain the automation be recovered over its useful life?

Automating a judgment-heavy procedure can force false precision, where nuanced assessment is reduced to a rigid rule that misses what a human reviewer would catch. Automate what is genuinely automatable; keep judgment where judgment belongs.

### Validate Data Completeness And Accuracy Before Relying On It

Automated procedures are only as reliable as the data they process. The most common failure of audit analytics is not a bug in the script but a flaw in the data extract: missing records, duplicated rows, stale snapshots, filtered subsets presented as complete, or fields that mean something different than assumed.

Before relying on an extract, validate:

- reconcile the extract total to an independent control total (record count, monetary total, or hash);
- confirm the extract covers the full population and period intended;
- check for duplicate, missing, or null records and investigate anomalies;
- verify field definitions and that values fall within expected ranges;
- confirm the extract was taken from the production system at the appropriate point in time;
- document who provided the extract, how it was transferred, and whether its integrity was preserved.

An extract that cannot be reconciled to an independent total cannot support a conclusion about the population. Treat un-reconciled data as insufficient evidence.

### Make The Logic Explicit And Reviewable

Automated logic must be transparent enough that a reviewer can understand what the script does, why, and what its limitations are. Opaque scripts, whether written by the auditor or inherited from a tool, cannot be challenged or trusted.

Ensure logic is reviewable by:

- documenting the objective of each script in plain language;
- commenting the code to explain non-obvious steps and assumptions;
- mapping the script logic back to the audit objective and the control or assertion being tested;
- recording the parameters, thresholds, and inclusion or exclusion rules applied;
- identifying known limitations, such as populations the script does not cover or conditions it does not test;
- retaining the script and its parameters so the procedure can be rerun and reproduced.

A script that cannot be explained to a reviewer is not yet fit to support an audit conclusion.

### Test The Script Against Known Cases

A new or modified script must be tested before its results are relied upon. Testing confirms the script does what it is intended to do and does not silently produce wrong answers.

Test by:

- running the script against a population containing known exceptions and confirming it detects them;
- running it against a clean population and confirming it does not flag false positives;
- testing boundary conditions around thresholds and dates;
- testing how the script handles missing, null, or malformed data;
- comparing results to a manual test on a small sample to confirm consistency;
- re-running after any logic change to confirm the change had the intended effect and no unintended side effects.

A script that has never been tested against known cases is an unverified control. Treat its output as preliminary until validation is complete.

### Preserve Reproducibility

Audit evidence must be reproducible. An automated result that cannot be regenerated later, because the script was overwritten, the data was not retained, or the environment changed, cannot support a conclusion under review.

Ensure reproducibility by:

- retaining the exact version of the script used, with version control or dated copies;
- retaining the data extract processed, or a hash that identifies it;
- documenting the environment, tool versions, and parameters used;
- storing the output alongside the inputs and logic so the result can be reconstructed;
- noting any manual steps performed on the output and their rationale.

If a reviewer cannot reproduce the result from the retained materials, the procedure's evidentiary value is compromised.

### Recognize The Limits Of Automation

Automation extends reach but does not replace professional judgment, skepticism, or corroborating evidence. A clean automated result does not prove absence of risk; it proves the specific tests run did not flag exceptions.

Recognize limits by:

- documenting what the automation tested and, equally important, what it did not test;
- considering whether exceptions the script is not designed to catch could still exist;
- complementing automated testing with targeted manual procedures for judgment areas;
- treating automation as one source of evidence, to be combined with inquiry, observation, and other procedures;
- resisting the tendency to treat full-population testing as inherently conclusive.

Full population coverage of a narrow test is not the same as comprehensive assurance over the assertion.

### Manage The Lifecycle Of Automated Procedures

Automated procedures are not built once and used forever. Source systems change, business rules evolve, and scripts that worked last year may silently fail this year. Automation requires maintenance.

Manage the lifecycle by:

- reviewing scripts at each use to confirm they still apply to the current environment;
- re-validating data feeds and field definitions when source systems are upgraded or modified;
- retiring scripts that no longer serve their purpose rather than leaving them to run unreviewed;
- documenting ownership so someone is accountable for maintenance;
- scheduling periodic review of standing continuous audit scripts.

An unmaintained script is a control that has degraded without anyone noticing.

## Common Traps

### Trusting The Data Extract Without Reconciliation

The single most common automation failure is relying on an extract that is incomplete or inaccurate. Always reconcile to an independent control total before processing.

### Opaque Logic No Reviewer Can Challenge

Complex scripts without documentation or comments cannot be reviewed. If the logic cannot be explained, the result cannot be relied upon.

### Full Population Testing Treated As Conclusive

Testing every record against a narrow rule provides deep but narrow assurance. Do not let the breadth of coverage mask the narrowness of what was tested.

### Silent Failures On Edge Cases

Scripts that skip records they cannot parse, or that treat nulls as zeros, can produce clean-looking results that omit real exceptions. Test how the script handles imperfect data.

### Automating Judgment Poorly

Forcing a judgment-based procedure into rigid code can produce false precision. If the test genuinely requires interpretation, automate the data gathering but keep the judgment human.

### No Reproducibility

If the script, data, and parameters are not retained, the result cannot be reconstructed under review. Reproducibility is part of evidentiary sufficiency.

### Set And Forget

Scripts that ran correctly once may fail after system changes. Re-validate automation at each use against the current environment.

## Self-Check

- Is each automated procedure chosen because it is genuinely suitable for automation, not merely because the data is available?
- Has the data extract been reconciled to an independent control total for completeness and accuracy before reliance?
- Is the script logic documented, commented, and reviewable, with parameters, thresholds, and limitations explicit?
- Has the script been tested against known exceptions, clean data, boundary conditions, and malformed records before its output was relied upon?
- Are the script version, data, parameters, and output retained so the result can be reproduced by an independent reviewer?
- Are the limits of the automation documented, including what was not tested, so full-population results are not over-interpreted?
- Is automation complemented by manual procedures where judgment, observation, or corroboration is required?
- Is there a lifecycle process to re-validate scripts when source systems change and to retire obsolete automation?
- Is ownership of each automated procedure clearly assigned for maintenance?
- Could an independent reviewer reproduce the result and understand its scope and limitations from the retained documentation?
