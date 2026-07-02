---
name: deletion_rectification_and_objection_rights.md
description: Use when the agent is handling erasure, rectification, objection, restriction, or portability requests, resolving conflicts with legal hold or other retention duties, or determining when rights do not apply due to exemptions and overriding legal bases.
---

# Deletion, Rectification And Objection Rights

Beyond access, individuals hold a family of rights that require the organization to change or stop processing: erasure, rectification, restriction, objection, and portability. Each has different triggers, scopes, and exceptions, and each creates operational and evidentiary demands. The recurring failure is treating these rights as interchangeable or applying blanket exemptions: refusing all erasure because "we have a legal obligation," or ignoring objection because "we have a legitimate interest." Both are compliance defects.

Use this skill before responding to a deletion, correction, stop-processing, or portability request, or when designing the workflows that handle them. The goal is to make the agent apply each right on its own terms, document the analysis, and resolve conflicts with other duties explicitly.

## Core Rules

### Handle The Right To Erasure On Its Specific Grounds

The right to erasure (GDPR Article 17) is not an absolute right to delete on demand. It applies where specific grounds exist, and it is subject to exceptions.

Apply erasure where:

- the personal data is no longer necessary for the purpose it was collected;
- consent was withdrawn and there is no other lawful basis;
- the individual successfully objected and there are no overriding legitimate grounds;
- the processing was unlawful;
- legal obligation requires deletion.

Refuse or limit erasure where:

- retention is necessary for compliance with a legal obligation;
- the data is needed for the establishment, exercise, or defense of legal claims;
- processing is necessary for public health, archiving, or research under conditions in the law;
- special regime exemptions apply.

Document the ground relied upon, whether granting or refusing. A refusal without a stated ground is not defensible.

### Process Rectification Promptly And Propagate It

The right to rectification (Article 16) requires correcting inaccurate data and completing incomplete data. The operational challenge is propagation: a correction in the primary system that does not reach downstream copies is incomplete.

Process rectification by:

- verifying the inaccuracy or incompleteness through reasonable means;
- correcting the primary record;
- propagating the correction to warehouses, analytics, ML features, and vendor systems;
- notifying recipients to whom the data was disclosed, where appropriate;
- informing the individual of the action taken.

If rectification is refused, explain why and offer the right to a supplementary statement recording the dispute.

### Apply Objection And Stop The Processing Unless Overridden

The right to object (Article 21) lets individuals stop certain processing, with different thresholds depending on the basis.

Handle objection by:

- stopping processing based on legitimate interests or public task unless the controller demonstrates compelling legitimate grounds that override the individual's interests;
- stopping direct marketing upon objection with no balancing test, as this right is absolute;
- stopping processing for scientific, historical, or statistical research unless the processing is necessary for that research.

For direct marketing, objection must be honored immediately and completely, including downstream profiling for marketing. The most common error is honoring the surface opt-out while continuing to profile for marketing.

### Use Restriction As A Temporary Or Conditional Measure

The right to restriction (Article 18) limits processing to storage while a dispute is resolved. It is a holding measure, not deletion.

Restriction applies where:

- the individual contests accuracy, pending verification;
- the processing is unlawful but the individual prefers restriction to erasure;
- the data is no longer needed but the individual needs it for legal claims;
- the individual has objected and the balancing test is pending.

During restriction, store the data but do not otherwise process it, except for storage and with the individual's consent. Mark restricted records clearly so downstream systems respect the limit.

### Deliver Portability In A Usable Format

The right to data portability (Article 20) applies to data the individual provided, processed by consent or contract, and carried out by automated means.

Provide portability by:

- delivering the data in a structured, commonly used, and machine-readable format;
- covering data the individual provided and observed data closely linked to their activity;
- enabling transmission to another controller where technically feasible;
- not including data that would adversely affect the rights of others.

Portability is narrower than access: it does not cover inferred data or data processed on other bases, and it does not require creating data that does not exist.

### Resolve Conflicts With Legal Hold And Other Laws

Erasure and restriction often conflict with legal hold, litigation, tax, employment, or sector retention duties. The conflict must be resolved explicitly, not by defaulting to retention.

Resolve conflicts by:

- checking the legal hold register before acting on erasure;
- retaining only the data and only for the period the overriding basis requires;
- documenting the specific legal obligation or claim basis for retention;
- releasing retention promptly when the overriding basis expires;
- communicating honestly with the individual about what is deleted and what is retained and why.

Retaining everything "just in case" is not a conflict resolution; it is a minimization failure.

### Coordinate Rights Across Controllers And Processors

When data is shared with processors, affiliates, or recipients, rights actions must propagate. A deletion honored in the controller's system but not communicated to processors is incomplete.

Coordinate by:

- invoking contractual obligations on processors to support rights actions;
- notifying recipients to whom data was disclosed of rectification, restriction, or erasure;
- tracking propagation and confirmation from recipients;
- handling portability where data must move between controllers.

### Respond Within Timelines And Document

All these rights share the GDPR one-month response window (extendable for complexity with notice) and CCPA/CPRA's 45-day window. Document the request, analysis, action, propagation, and notification for each.

## Common Traps

### Blanket Refusal Of Erasure Based On Generic Legal Obligation

Citing "we have a legal obligation to retain data" without identifying the specific obligation or scoping it to the relevant data is an unjustified refusal.

### Correcting The Primary Record Only

Rectification that does not propagate to warehouses, ML features, and vendors leaves inaccurate data in use.

### Honoring Marketing Opt-Out At The Surface Only

Stopping marketing emails while continuing to profile the individual for marketing violates the absolute right to object to direct marketing.

### Confusing Restriction With Deletion

Treating a restriction request as fulfilled by deleting the data, or by doing nothing, both misapply the right.

### Over-Delivering Portability

Including inferred data or data of other individuals in a portability response exceeds the right's scope and can harm others.

### Letting Legal Hold Become Indefinite Retention

Applying a hold and never releasing it converts a temporary preservation duty into indefinite retention.

### Failing To Notify Recipients

Rectifying or deleting data without telling recipients to whom it was disclosed leaves inaccurate or unwanted data in circulation.

### Treating All Rights As Access

Responding to erasure or objection with a copy of the data, or responding to portability with a rights metadata letter, misapplies the specific right invoked.

## Self-Check

- Is erasure handled on its specific grounds, with refusal or limitation tied to a documented overriding basis?
- Does rectification propagate from the primary record to warehouses, analytics, ML features, vendors, and recipients?
- Is objection honored absolutely for direct marketing, including downstream profiling?
- Is restriction applied as a temporary or conditional measure, with restricted records marked and downstream systems respecting the limit?
- Is portability delivered in a structured, machine-readable format covering data the individual provided, without including inferred data or others' data?
- Are conflicts with legal hold, litigation, tax, or sector retention resolved explicitly, retaining only what the overriding basis requires?
- Are processors and recipients notified of rectification, restriction, or erasure, with propagation tracked?
- Is each response delivered within the statutory timeline with any extension noticed on time?
- Is the analysis for each right documented, including grounds, exemptions, and actions taken?
- Is the individual informed clearly about what was deleted, corrected, restricted, or ported and what was retained and why?
