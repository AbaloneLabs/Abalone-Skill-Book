---
name: segregation_of_duties_and_key_controls.md
description: Use when the agent is evaluating segregation of duties during an audit, identifying incompatible duties and conflict of interest, assessing privileged and administrator access, mapping key controls to significant risks, or designing compensating controls where segregation cannot be fully achieved.
---

# Segregation Of Duties And Key Controls

Segregation of duties is the principle that no single person should control enough of a process to both cause and conceal an error or fraud. When duties are combined, a person who can initiate a transaction, approve it, record it, and reconcile it can also hide what they have done. Segregation is one of the oldest and most powerful controls, and it is also one of the most frequently compromised, because small teams, system privileges, and convenience all push toward concentration. Key controls are the small set of controls that actually address significant risks. An audit that lists every control without identifying which ones are key will spread effort evenly and miss the controls that matter. Agents often treat segregation as a yes-or-no checklist and key controls as whatever controls happen to be documented, which misses both the override pathways and the controls worth testing.

Use this skill before relying on controls, before assessing deficiency severity, and whenever access rights, role design, or process ownership are under review. The goal is to identify where duties or access create concealment risk, to find the controls that genuinely address significant risks, and to judge whether compensating controls are real and adequate.

## Core Rules

### Understand Why Segregation Matters

Segregation exists to prevent a single person from completing and concealing a harmful act. The logic is that combining authorization, custody, recording, and reconciliation in one person removes the independent check that would catch the error or fraud.

The four functional duties that should be separated are:

- authorization or approval of transactions;
- custody of assets, including cash, inventory, or data;
- recording or posting of transactions to the books;
- reconciliation or independent review of the records against assets or source data.

When one person holds two or more of these over the same asset or process, concealment becomes possible. The risk is not only theft; it is also undetected error, biased estimates, and override of other controls.

### Identify Incompatible Duties By Process

Segregation must be assessed process by process, because the incompatible combinations differ by cycle.

Common incompatible combinations include:

- vendor or customer master setup and payment or receipt processing;
- purchase order creation and goods receipt or invoice approval;
- payroll setup, time approval, and payroll release;
- bank account operation and bank reconciliation;
- cash receipt handling and accounts receivable recording;
- inventory custody and inventory record keeping;
- journal entry preparation and journal entry approval;
- fixed asset recording and physical asset custody;
- user access administration and transaction posting in the same system;
- system development or configuration and production deployment;
- treasury dealing and treasury reconciliation or confirmation.

For each in-scope process, map who performs each functional duty and flag where one person spans incompatible duties. A clean org chart does not prove segregation; role-level mapping does.

### Assess Access Rights And Privileged Accounts

System access can override process segregation. A person may appear segregated on paper but hold system privileges that let them perform every step.

Investigate:

- who holds administrator, superuser, or privileged accounts;
- whether privileged access is logged, reviewed, and time-limited;
- whether shared or generic accounts are used and by whom;
- whether access matches current roles or includes stale entitlements;
- whether segregation is enforced in the system or only in policy;
- whether users can self-grant or escalate privileges;
- whether emergency or firecall access is controlled and reviewed;
- whether service accounts perform transactions and who controls them.

Privileged access is the modern equivalent of holding all four functional duties. Treat broad or unaudited privileged access as a segregation failure even when the org chart looks clean.

### Map Key Controls To Significant Risks

Not every control is worth testing. Key controls are those that address significant risks, important assertions, fraud pathways, or compliance obligations. Identifying them focuses effort where it matters.

To identify key controls:

- list the significant risks and important assertions for the engagement;
- for each, find the control or controls that prevent or detect it;
- prefer controls that operate at the level of the risk, not generic oversight;
- confirm the control is precise, covers the population, and produces evidence;
- rank controls by the magnitude and likelihood of the risk they address;
- ensure each significant risk has at least one mapped key control.

A control that addresses a minor risk is not key, however well documented. A control that addresses a significant risk and is the primary line of defense is key, however informal it looks.

### Evaluate Whether Key Controls Are Independent And Override-Resistant

A key control loses value if the person whose activity it controls can influence or bypass it. Independence and override resistance are part of what makes a control key.

Assess each key control for:

- independence of the performer from the activity controlled;
- whether management can override it without detection;
- whether it depends on information the controlled party can manipulate;
- whether it is the only control over the risk, making it singularly important;
- whether collusion among a small group could defeat it;
- whether it leaves tamper-evident evidence.

Where override is easy or the control is the sole defense, treat the risk as higher and consider additional or compensating controls.

### Design And Evaluate Compensating Controls

Perfect segregation is often impossible in small entities, specialized roles, or emergency situations. When segregation cannot be achieved, compensating controls must reduce the residual risk, and they must be real, not nominal.

Valid compensating controls include:

- independent review of transactions or reports by someone outside the process;
- bank statements sent directly to, and reviewed by, an independent owner;
- dual authorization for payments or changes above a threshold;
- detailed exception reporting reviewed and acted upon;
- external confirmations or third-party statements reconciled independently;
- surprise counts or audits of cash, inventory, or sensitive assets;
- monitoring analytics that flag unusual patterns;
- mandatory rotations or vacations that force handovers.

A compensating control must be precise, timely, independent, and evidenced. A vague statement that management reviews activity is not compensating. Document exactly what the compensating control does, who performs it, and how it reduces the specific concealment risk.

### Consider Collusion And Small Teams

In small teams, formal segregation may be structurally impossible, and collusion among the few staff becomes more feasible. The assessment must reflect this reality rather than pretend it away.

For small entities or teams:

- identify where duties are unavoidably combined;
- assess the heightened collusion and override risk;
- design stronger compensating controls, often involving the owner or an external party;
- consider substantive procedures as the primary approach where controls cannot be relied upon;
- be skeptical of explanations that assume trust replaces control.

Trust is not a control. Where segregation is weak, the audit response must lean more heavily on substantive testing and independent evidence.

### Assess The Severity Of Segregation Failures

A segregation failure is a control deficiency whose severity depends on what could go wrong, not only on what has been detected. Evaluate severity conservatively.

Assess:

- the maximum exposure if the combined duties were exploited;
- the likelihood given incentives, override opportunity, and history;
- whether fraud, theft, or material misstatement could result;
- the duration the combined duties have existed;
- whether compensating controls reduce the likelihood or magnitude;
- whether prior errors or incidents have occurred in the area;
- the sensitivity of the assets or data involved.

A segregation failure over cash disbursements with no compensating control can be a significant or severe deficiency even if no loss has yet been found, because the concealment opportunity is what defines the risk.

### Document The Map And The Conclusion

The segregation and key-control assessment must be documented well enough to be reviewed and to drive testing.

Document:

- the role-level duty map for each in-scope process;
- the incompatible combinations identified;
- the privileged and shared access findings;
- the list of significant risks and the key controls mapped to each;
- the independence and override assessment for each key control;
- the compensating controls relied upon and why they are adequate;
- the severity conclusions for each segregation failure;
- the implications for the audit approach and for deficiency reporting.

A conclusion that segregation is adequate must be supported by the map and the access review, not asserted.

## Common Traps

### Treating Segregation As A Yes-or-No Checklist

Marking a process as segregated without mapping roles and access misses the combinations that create concealment risk. Segregation is a role-level and access-level analysis.

### Ignoring Privileged And Shared Access

A clean org chart means nothing if one administrator holds privileges spanning the whole process. Always assess system access alongside duties.

### Listing Every Control As Key

Calling every documented control key dilutes effort and hides the few controls that actually matter. Key controls are defined by the significant risks they address.

### Accepting Vague Management Review As Compensating

A compensating control must specify what is reviewed, against what criteria, by whom, and what happens to exceptions. Unspecified review is not compensating.

### Assuming Trust Or Long Tenure Reduces Risk

Long-serving, trusted staff are often the ones with the most concentrated access and the best opportunity. Trust is not a control and can increase concealment risk.

### Forgetting Collusion In Small Teams

In small teams, even formally segregated duties can be defeated by collusion among two or three people. Reflect this in risk assessment and in the choice of audit approach.

### Downgrading Severity Because No Loss Was Found

Segregation failures are judged on concealment opportunity and potential exposure, not on detected losses. The absence of a known theft does not make the deficiency minor.

### Disconnecting The Map From Testing And Reporting

A duty map that never informs which controls are tested or how deficiencies are reported is wasted analysis. The map must drive the audit response.

## Self-Check

- Are the four functional duties, authorization, custody, recording, and reconciliation, assessed for separation in each in-scope process?
- Is a role-level duty map produced that identifies incompatible combinations, rather than relying on an org chart?
- Are privileged, shared, generic, and service accounts reviewed for access that overrides process segregation?
- Are key controls identified by mapping them to significant risks and important assertions, not by documentation volume?
- Is each key control assessed for independence, override resistance, sole-defense status, collusion exposure, and tamper-evident evidence?
- Where segregation cannot be achieved, are compensating controls specified precisely with performer, criteria, timing, evidence, and exception handling?
- Is the heightened collusion and override risk in small teams reflected in the assessment and in the audit approach?
- Is the severity of each segregation failure assessed on exposure, likelihood, duration, fraud potential, and compensating controls, not on detected losses?
- Is the full map, access review, key-control mapping, compensating controls, and severity conclusions documented and reviewable?
- Does the assessment drive which controls are tested and how deficiencies are reported, rather than standing as an isolated artifact?
