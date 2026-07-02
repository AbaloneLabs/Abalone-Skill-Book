---
name: compliance_data_analytics_and_detection_rules.md
description: Use when the agent is designing compliance analytics, writing or tuning detection rules, building anomaly detection for compliance risks, setting alert thresholds, or evaluating whether automated detection logic effectively surfaces misconduct and control failures.
---

# Compliance Data Analytics And Detection Rules

Compliance analytics turn large volumes of transactions, access logs, communications, and events into alerts that surface potential misconduct, control failures, and regulatory breaches. Done well, analytics extend compliance coverage far beyond what manual testing can reach. Done poorly, they generate noise that desensitizes reviewers, miss the patterns that matter because the rules are naive, or create a false sense of assurance because the dashboard looks green. Detection logic is a control in itself, and like any control it must be designed, tested, tuned, governed, and periodically re-evaluated against the risk it addresses.

Use this skill before building detection rules, tuning thresholds, selecting analytics techniques, evaluating an existing analytics program, or responding to a finding that analytics missed a known issue. The goal is to make the agent treat detection logic as risk-driven, testable, and governable, not as a one-time build that runs unattended.

## Core Rules

### Start From The Risk And The Conduct Pattern, Not From The Data

The most common failure in analytics is starting from available data and building rules around it, rather than starting from the misconduct or control failure to be detected and working backward to the data and logic that would reveal it.

For each analytics objective:

- describe the specific risk or control failure to be detected, in behavioral terms;
- describe how the misconduct or failure would appear in the data, including how a sophisticated actor would try to conceal it;
- identify the data elements required to detect it, across systems;
- identify the legitimate transactions that resemble the misconduct and must be distinguished;
- define what a true positive looks like and what a false positive looks like;
- define the detection objective in measurable terms, such as recall of known-bad patterns and acceptable false-positive rate.

A rule built without this analysis detects the obvious and misses the patterns that a knowledgeable insider would use to evade detection.

### Build Logic That Reflects How Misconduct Is Actually Concealed

Naive rules detect the unsophisticated and miss the rest. Effective analytics anticipate evasion.

Consider:

- structuring or smurfing to stay below thresholds, requiring aggregation logic across time, accounts, and related parties;
- use of intermediaries, related parties, or shell entities, requiring relationship and network analysis;
- timing manipulation, such as transactions recorded just after period-end or just under approval limits, requiring threshold-proximity and timing analytics;
- split or batched transactions to evade review, requiring volume and velocity analysis;
- backdating or reconstruction, requiring timestamp and edit-history analysis;
- unusual access patterns preceding data changes, requiring correlation of access logs with transaction logs;
- gradual drift from normal patterns that point thresholds miss, requiring trend and behavioral analytics.

A rule that flags only transactions above a fixed threshold will miss everything deliberately kept below it. Layer threshold, aggregation, relationship, behavioral, and sequence logic to cover different evasion strategies.

### Calibrate Thresholds Against Risk, Volume, And Capacity

Threshold setting is a tradeoff between detection and noise. A threshold too sensitive floods reviewers and causes alert fatigue; one too lax misses real issues. Both are failures.

Calibrate by:

- running new rules against historical data, including known-bad cases where available, to estimate true-positive and false-positive rates before going live;
- benchmarking thresholds against peer or industry data where available;
- considering reviewer capacity and the expected disposition time per alert;
- setting thresholds that produce an alert volume that can be genuinely reviewed, not merely queued;
- documenting the rationale and data behind each threshold;
- reviewing threshold performance regularly and adjusting based on outcomes.

Alert fatigue is a control failure, not merely an efficiency problem. When reviewers stop reading alerts because there are too many, detection effectively stops. Capacity must be part of threshold design.

### Validate Rules Before And After Deployment

A detection rule is a control and must be tested like one. Validation confirms the rule does what it is supposed to do.

Validate by:

- testing against a population of known historical cases to confirm they would have been caught;
- testing against clearly legitimate transactions to confirm they are not flagged;
- injecting synthetic test cases to confirm the rule fires correctly;
- checking that the rule handles edge cases such as missing data, multi-currency, time zones, and entity resolution;
- confirming the data sources feeding the rule are complete and current;
- confirming the rule logic matches the documented intent, including parameter values;
- re-validating after any change to source systems, data feeds, or rule logic.

A rule that passed validation once but was never re-validated after a system migration may be silently broken. Treat rule changes with the same change-management discipline as any control change.

### Tune Continuously Based On Outcomes, Not Intuition

Detection quality improves only when tuning is informed by what alerts actually found. Without feedback, rules drift toward irrelevance.

Establish a feedback loop:

- capture the disposition of every alert as true positive, false positive, or other, with rationale;
- aggregate false-positive drivers to identify rules or parameters generating noise without value;
- aggregate true-positive patterns to identify emerging risks that may warrant new rules;
- review alert-to-investigation and investigation-to-action conversion rates;
- periodically retire or rewrite rules that produce no value;
- document tuning changes with rationale, approval, and re-validation.

Tuning that suppresses alerts to reduce volume without analyzing whether real cases were among them can quietly reduce detection. Require evidence that suppressed alerts were reviewed and were false positives.

### Govern Rules As Controlled Assets

Detection rules are assets that require governance. Ungoverned rules proliferate, conflict, decay, and become impossible to audit.

Govern by maintaining for each rule:

- a unique identifier and descriptive name;
- the risk addressed and detection objective;
- the data sources, logic, and parameters;
- the owner accountable for the rule;
- the version history and change log;
- the validation evidence and last validation date;
- the performance metrics and thresholds;
- the review cadence and next review date;
- the dependencies on systems, data feeds, and reference data.

Periodically review the rule inventory for redundancy, gaps, and rules that no longer correspond to a current risk. A rule library that has grown without pruning is a sign of weak governance.

### Account For Data Limitations And Model Risk

Analytics are only as good as the data and models behind them. Limitations must be understood and disclosed, not hidden.

Address:

- data completeness, accuracy, timeliness, and reconciliation, treated as a prerequisite;
- missing fields, defaulted values, and data-entry errors that can distort results;
- entity resolution challenges where the same party appears under different identifiers;
- model or algorithmic bias that may systematically miss or over-flag certain populations;
- explainability, so that an alert can be understood and defended, especially where machine learning is used;
- drift in data distributions or model performance over time.

Where machine learning or advanced models are used, apply model risk management practices including documentation, independent validation, ongoing monitoring, and human oversight of outcomes. A black-box model that cannot explain its alerts is hard to defend to a regulator and hard for reviewers to act on.

### Close The Loop From Alert To Action To Risk Reduction

Detection that does not lead to action does not reduce risk. The value of analytics is realized only when alerts are investigated, findings are remediated, and patterns feed back into risk assessment.

Ensure:

- clear ownership of alert investigation and defined service levels;
- investigation quality standards and documentation;
- escalation paths for significant findings;
- feedback of investigation outcomes into rule tuning;
- aggregation of analytics findings into risk assessment and reporting;
- measurement of end-to-end effectiveness from detection to risk reduction.

## Common Traps

### Building Rules From Available Data Rather Than From Risk

Data-first analytics detect the obvious and miss the concealed. Start from the conduct pattern.

### Fixed Thresholds That Invite Evasion

Anything below a static threshold is a roadmap for avoidance. Layer aggregation, relationship, and behavioral logic.

### Alert Volume That Exceeds Review Capacity

Flooding reviewers causes fatigue and effective shutdown of detection. Calibrate thresholds to capacity.

### Suppressing Alerts To Reduce Noise Without Analyzing Them

Blanket suppression can hide real cases. Require evidence-based tuning.

### Never Re-Validating Rules After System Change

A rule that worked once may be silently broken after a migration. Re-validate after changes.

### Treating A Green Dashboard As Evidence Of Compliance

No alerts may mean no misconduct, or it may mean the rules are broken or blind. Absence of alerts is not absence of risk.

### Black-Box Models Without Explainability Or Monitoring

Models that cannot explain alerts are hard to act on and defend. Apply model risk management.

## Self-Check

- Does each analytics objective start from a specific risk and conduct pattern, including how a sophisticated actor would conceal it, before data and logic are chosen?
- Does detection logic address evasion through structuring, intermediaries, threshold proximity, timing, backdating, and drift, rather than relying on fixed thresholds alone?
- Are thresholds calibrated against historical data, known-bad cases, peer benchmarks, and reviewer capacity, with rationale documented?
- Is each rule validated against known-bad and known-good cases, synthetic injections, edge cases, and source data quality before and after deployment?
- Is there a feedback loop capturing alert dispositions, false-positive drivers, true-positive patterns, and conversion rates to inform continuous tuning?
- Is each rule governed with an identifier, risk, logic, owner, version history, validation evidence, performance metrics, review cadence, and dependencies?
- Are data completeness, accuracy, entity resolution, bias, explainability, and model drift assessed, with model risk management applied where relevant?
- Does the analytics program close the loop from alert to investigation to remediation to risk-assessment feedback and effectiveness measurement?
- Are retired or rewritten rules documented with rationale, and is the rule inventory periodically reviewed for redundancy and gaps?
- Is the absence of alerts interpreted with caution, recognizing that a green dashboard may reflect broken or blind rules rather than absence of risk?
