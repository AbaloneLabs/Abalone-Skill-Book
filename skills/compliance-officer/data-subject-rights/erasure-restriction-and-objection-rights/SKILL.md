---
name: erasure-restriction-and-objection-rights.md
description: Use when the agent is handling data subject erasure or deletion requests, applying the legal obligation and other exemptions to erasure, processing restriction requests, evaluating the right to object to direct marketing or legitimate interests processing, or determining whether compelling legitimate grounds override an objection.
---

# Erasure, Restriction, and Objection Rights

Beyond access and portability, data subjects have rights that directly alter how the organisation processes their data: the right to erasure (deletion), the right to restriction (limiting processing), and the right to object (stopping certain processing). These rights are powerful but conditional, and the conditions are where the judgment lies. Erasure is not absolute — it is balanced against legal obligations and other grounds. Restriction is a temporary or partial measure with specific triggers. Objection to direct marketing is near-absolute, but objection to legitimate-interests processing requires a balancing test. This skill addresses the judgment involved in applying these rights correctly: granting them where required, limiting them where exemptions apply, and documenting the reasoning either way.

## Core Rules

### Apply the right to erasure with its specific grounds and exemptions

The right to erasure applies where:

- The data is no longer necessary for the purpose;
- Consent is withdrawn and no other basis applies;
- The data subject objects and there are no overriding grounds (for legitimate-interests-based processing);
- The data was unlawfully processed;
- Erasure is required by legal obligation.

But erasure does not apply where processing is necessary for:

- Exercising the right of freedom of expression and information;
- Complying with a legal obligation (tax, employment, AML retention);
- Public interest or official authority tasks;
- Archiving, scientific, historical, or statistical purposes (with safeguards);
- Establishing, exercising, or defending legal claims.

When an erasure request arrives, identify the ground for erasure and check for an applicable exemption. Where an exemption applies, the data may be retained (and processing restricted) rather than deleted. Document the analysis.

### Restrict rather than delete where retention is required but processing should pause

Where the data subject contests accuracy, the processing is unlawful but the subject opposes erasure, the controller no longer needs the data but the subject does for legal claims, or the subject has objected and the balancing is pending, the right to restriction applies. During restriction, the data may be stored but not otherwise processed. Restriction is the middle path between deletion and unrestricted processing; use it where retention is justified but active processing is not.

### Treat objection to direct marketing as near-absolute

The right to object to processing for direct marketing is absolute: once the subject objects, the processing for that purpose must stop, with no balancing test and no "compelling grounds" exception. This includes profiling to the extent related to direct marketing. Implement objection to marketing as an immediate stop, not a balancing exercise. Confirm the stop across all marketing systems and processors.

### Apply the balancing test for objection to legitimate-interests processing

Where the data subject objects to processing based on legitimate interests (not marketing, not direct marketing profiling), the controller must stop unless it demonstrates compelling legitimate grounds that override the subject's interests, rights, and freedoms, or the processing is for legal claims. The balancing test considers:

- The nature and importance of the controller's legitimate interest;
- The impact on the data subject;
- The subject's reasonable expectations;
- Whether less intrusive alternatives exist.

Document the balancing. If compelling grounds are not demonstrable, stop the processing. Do not treat the objection as a mere request to be considered; the burden is on the controller to justify continuation.

### Propagate erasure, restriction, and objection across systems and processors

These rights apply to all the data subject's personal data in the organisation's control, including data held by processors and data already shared with recipients. When granting erasure:

- Delete across all systems, including backups (within backup cycles) and analytics;
- Notify processors to delete or restrict;
- Notify recipients to whom the data was disclosed, so they can delete.

A deletion from the primary system that leaves data in processors' or recipients' hands is incomplete. Build propagation into the fulfillment workflow.

### Handle the interaction between rights and retention obligations

The most common tension is between erasure requests and retention obligations (tax, AML, employment law). The resolution is typically:

- Delete the data from active processing systems (so it is not used for new purposes);
- Retain the minimum necessary in a restricted, access-controlled store for the legal retention period;
- Delete fully when the retention period expires.

Do not use retention as a blanket refusal; apply it specifically to the data that must be retained, and delete or restrict the rest.

### Respond within the deadline and document the decision

The standard response period for erasure, restriction, and objection is one month, extendable by two months for complexity with notification. Respond with:

- Confirmation of the action taken (deleted, restricted, objected processing stopped);
- If refused or limited, the reasons and the right to complain to the supervisory authority and to seek judicial remedy;
- Notification of recipients where data was deleted or restricted.

Document the request, the analysis, the decision, the actions taken, and the response. Undocumented decisions are indefensible if challenged.

### Distinguish objection from withdrawal of consent

Objection is a right that applies regardless of the basis; withdrawal of consent applies only where consent is the basis. A data subject who objects to legitimate-interests processing is exercising the objection right (with the balancing test), not withdrawing consent. A subject who withdraws consent is exercising the withdrawal right (which stops consent-based processing immediately). Identify which mechanism applies.

## Common Traps

### Refusing erasure by invoking retention obligations too broadly

Citing "legal obligation" to retain all data when only specific categories must be retained for specific periods. Apply retention specifically; delete or restrict what is not required.

### Treating objection to marketing as a balancing exercise

Direct marketing objection is absolute. Applying a balancing test or "compelling grounds" analysis to marketing objection is a violation. Stop immediately.

### Failing to propagate deletion to processors and recipients

Deleting from the primary system but leaving data with the marketing vendor, the analytics platform, or the third party to whom it was shared. Propagate the action.

### Not applying the balancing test for legitimate-interests objection

Treating objection to legitimate-interests processing as automatically stopping it (without the controller's right to demonstrate compelling grounds) or, more commonly, ignoring the objection and continuing without balancing. Conduct and document the balancing.

### Deleting active data but leaving backups indefinitely

Backups are often overlooked; deleted data persists in backup tapes or snapshots. Define how deletion propagates to backups within defined cycles.

### Missing the one-month deadline or failing to document the decision

Requests are not tracked, deadlines slip, or decisions are made but not documented. Track, respond on time, and document the analysis and actions.

## Self-Check

- For erasure requests, are the specific grounds for erasure and the applicable exemptions (legal obligation, legal claims, public interest) identified and documented?
- Is restriction applied where retention is justified but active processing should pause (accuracy contest, unlawful processing, legal-claim need, pending objection)?
- Is objection to direct marketing treated as absolute, stopping processing immediately across all systems and processors?
- For objection to legitimate-interests processing, is the balancing test conducted and documented, with the burden on the controller to demonstrate compelling grounds?
- Do erasure, restriction, and objection actions propagate across all systems, backups (within cycles), processors, and recipients?
- Is the tension between erasure and retention obligations resolved by deleting active processing data while retaining only the minimum required in restricted storage?
- Are responses provided within one month (or the notified two-month extension), with reasons and remedy information where refused or limited?
- Is the distinction between objection (any basis, balancing for legitimate interests) and consent withdrawal (consent basis, immediate stop) correctly applied?
