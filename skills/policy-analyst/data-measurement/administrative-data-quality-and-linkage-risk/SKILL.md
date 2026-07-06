---
name: administrative_data_quality_and_linkage_risk.md
description: Use when the agent is using administrative data, government records, case management systems, linked datasets, registries, claims data, education records, enforcement data, benefits data, or operational data for policy analysis, monitoring, targeting, or evaluation.
---

# Administrative Data Quality And Linkage Risk

Administrative data can be powerful because it covers real programs and real populations, but it was usually collected to operate a system, not to answer policy questions. Definitions, incentives, missingness, eligibility rules, staff practices, and system changes can distort analysis. Agents often treat administrative data as objective because it is official. This skill helps the policy analyst assess whether administrative records are fit for policy use.

## Core Rules

### Understand Why The Data Exists

Administrative data reflects the purpose of the system that produced it: paying benefits, tracking cases, enforcing rules, issuing permits, billing services, recording incidents, or managing staff workload. Data collected for operations may omit people who never applied, cases diverted elsewhere, informal activity, unmet need, or discouraged applicants.

Ask what behavior the data system rewards, requires, and ignores. Official records show system contact, not always population reality.

### Define Each Field Operationally

Do not assume a field name means what it sounds like. "Closed case" may mean resolved, transferred, denied, inactive, or administratively closed. "Wait time" may start at referral, application, eligibility determination, appointment, or service start. "Complaint" may mean allegation, verified issue, duplicate report, or inquiry.

Get data dictionaries, business rules, system manuals, and staff explanations where possible. If definitions changed over time, mark breaks in series.

### Assess Missingness And Coverage

Missing data may be random, but often it is patterned. Language, disability, rural access, digital exclusion, immigration concern, stigma, or staff workload can affect whether data is captured. Coverage may exclude informal providers, nonparticipants, private actors, or people denied at the front door.

Before drawing conclusions, ask who is absent from the dataset and why. The missing population may be central to the policy problem.

### Check Data Quality Incentives

Front-line staff and organizations may have incentives to code cases in ways that affect funding, performance metrics, enforcement risk, waitlists, eligibility, or public reporting. Data quality can also suffer from rushed entry, duplicate records, legacy systems, or unclear categories.

Do not accuse without evidence, but do consider how incentives shape records. Validation audits and staff interviews can reveal issues.

### Treat Linkage As A Source Of Error

Linking datasets can improve analysis but creates matching errors, privacy risk, and representativeness problems. False matches and missed matches may differ by name changes, transliteration, unstable housing, family structure, age, data entry quality, or identifier availability.

Document linkage method, match rates, unmatched groups, privacy controls, and sensitivity checks. Do not treat linked data as complete just because a merge succeeded.

### Track System And Policy Changes

Administrative data series can shift when eligibility rules change, forms change, reporting requirements change, enforcement intensity changes, portals launch, staff training occurs, backlogs are cleared, or definitions are revised. Apparent trends may reflect data process rather than real-world change.

Create a timeline of policy, system, and reporting changes before interpreting trend breaks.

### Protect Privacy And Purpose Limits

Administrative data often includes sensitive personal information. Use legal authority, data-sharing agreements, minimization, access controls, de-identification, suppression, and ethical review where appropriate. Policy curiosity is not enough to justify every linkage or reuse.

Respect purpose limitations and community trust. Data use can harm people if it enables surveillance, exclusion, or unauthorized disclosure.

### Communicate Fitness For Use

Administrative data may be excellent for one question and poor for another. State whether the data is fit for descriptive monitoring, targeting, evaluation, equity analysis, forecasting, or operational improvement. Include caveats that affect interpretation.

Do not bury data limitations in an appendix if they change the conclusion.

### Validate With System Users and specify Data Extracts And Document Reproducibility

Before relying on administrative data, ask people who enter, manage, or are affected by the data how records are created in practice. Front-line staff may know workarounds, optional fields, backlog practices, or categories that analysts cannot see from extracts alone. Service users may know why certain groups avoid the system.

Validation does not replace data audit, but it catches practical meaning that tables often hide.

Administrative extracts are not self-describing. The same system can produce different files depending on the pull date, filter, snapshot point, inclusion of closed or archived cases, treatment of amendments, and timezone of timestamps. An extract pulled today and one pulled next month can disagree because backlogs cleared, late entries posted, or a system upgrade changed a default. When you request data, write down the as-of date, the selection criteria, the fields requested, the system version, and the contact who produced it. Keep the extract, the request, and the data dictionary together so a future analyst can reproduce or explain a number. Reproducibility is part of data quality: a figure that cannot be regenerated is weaker evidence than one that can.

## Common Traps

- Treating official administrative data as automatically accurate and complete.
- Assuming field names match policy concepts.
- Ignoring people who never enter the administrative system.
- Missing patterned missingness affecting marginalized groups.
- Overlooking incentives to code cases strategically.
- Treating linked data as error-free; ignoring data-series breaks caused by rule, form, or system changes
- Reusing sensitive data without checking legal authority and purpose; reporting precise statistics from low-quality fields
- Mentioning limitations without explaining how they affect the policy answer; interpreting data extracts without asking how records are actually created and used

## Self-Check

- Is the operational purpose of the dataset understood?
- Are key fields defined through data dictionaries, rules, or staff knowledge?
- Are missingness and coverage gaps assessed?
- Are data quality incentives and coding practices considered?
- Are linkage method, match rate, unmatched groups, and errors documented?
- Are system, policy, and reporting changes tracked over time?
- Are privacy, legal authority, data minimization, and access controls addressed?
- Is fitness for use stated for each analytic purpose?
- Are limitations connected to interpretation, not merely listed?
- Would a program operator recognize the data description as accurate?; have data entry practices and user experiences been checked where they affect interpretation?
