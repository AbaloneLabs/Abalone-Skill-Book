---
name: tax_transparency_entity_classification_and_controlling_persons.md
description: Use when the agent is classifying entities as active or passive non-financial entities under CRS, tracing controlling persons of passive entities, validating entity self-certifications, or handling complex ownership structures for cross-border tax transparency and automatic exchange of information.
---

# Tax Transparency Entity Classification And Controlling Persons

Entity classification is the most error-prone area of CRS and FATCA compliance, because it determines whether an entity account is reportable on its own or whether the institution must trace through to controlling persons. Under the OECD CRS Commentary, FATCA regulations, and equivalent guidance, financial institutions must classify entity account holders as active or passive non-financial entities (NFEs), financial institutions, or other categories, and for passive NFEs, identify the controlling persons and their tax residences. The central judgment problem is that classification is not a self-evident label. It requires analyzing the entity's income, assets, and activities against defined tests, and for passive entities, tracing ownership to controlling persons using the same standard as AML or KYC. An entity misclassified as active when it is passive, or a passive entity whose controlling persons are not traced, results in under-reporting and compliance failure.

Use this skill before classifying entities under CRS or FATCA, tracing controlling persons of passive entities, validating entity self-certifications, or handling complex ownership for tax transparency. The goal is to make the agent think about the active versus passive tests, controlling person tracing, self-certification validation, and the decisions that are easy to make too casually. A classification that defaults to active without analysis, or that stops at the entity without tracing to controlling persons, defeats the transparency purpose.

This skill addresses jurisdiction-specific obligations. NFE definitions, active versus passive tests, controlling person standards, and documentation requirements differ across CRS, FATCA, and national implementations. Always confirm the applicable guidance and consult qualified tax or legal counsel for specific classification decisions.

## Core Rules

### Classify Entity Account Holders Against The Correct Tests

The first step for any entity account is classification. The institution must determine whether the entity is a financial institution, an active NFE, a passive NFE, or another specified category.

Classification framework:

- if the entity is a financial institution, the account is generally not reportable as an entity account but the entity has its own reporting obligations;
- if the entity is a non-financial entity, determine whether it is active or passive;
- active NFEs are generally not reportable on their own, but controlling persons of passive NFEs must be identified and reported if they are reportable persons;
- certain categories, such as governments, international organizations, central banks, and listed companies, have specific treatment.

The classification must be documented. An entity cannot be assumed active or passive without applying the tests, and the rationale for the classification must be recorded.

### Apply The Active NFE Tests Correctly

Active NFE status depends on meeting specific tests related to income and assets. The institution must apply the tests as defined in the applicable regime.

Active NFE tests typically include:

- less than 50 percent of gross income in the prior three years is passive income, such as dividends, interest, rents, and royalties;
- less than 50 percent of assets produce or are held for the production of passive income;
- the entity is a publicly traded NFE or a related entity;
- the entity is a government, international organization, central bank, or wholly owned by one;
- the entity is a non-profit meeting specified criteria;
- the entity is a start-up in its initial period, a treasury center, or meets another specific exception.

The institution must obtain information sufficient to apply the tests, typically through a self-certification that addresses income and assets. An entity that self-certifies as active without supporting detail is not reliably classified.

### Identify Passive NFEs And Trace To Controlling Persons

When an entity is a passive NFE, the institution must identify the controlling persons and determine whether they are reportable. This is where most classification errors occur.

Passive NFE controlling person requirements:

- identify the controlling persons using the AML or KYC standard applicable in the jurisdiction;
- controlling persons are typically natural persons who exercise control through ownership or other means, consistent with the beneficial ownership threshold;
- determine the tax residence of each controlling person;
- report the account with respect to any controlling person who is a reportable person;
- trace through layered ownership structures to reach the natural persons.

A passive NFE whose controlling persons are not traced is under-reported. The institution cannot treat the entity as the end of the analysis; it must look through to the natural persons who control it, using the same ownership tracing as AML.

### Validate Entity Self-Certifications

Entity self-certifications must be validated against other information in the institution's possession. An unvalidated self-certification is not reliable.

Validation requirements:

- confirm the self-certification is complete and addresses the entity's classification and, for passive NFEs, the controlling persons;
- check the self-certification against account opening documents, AML records, and other information;
- identify inconsistencies, such as an entity claiming active status while holding primarily income-producing assets;
- resolve inconsistencies through follow-up and re-documentation before relying on the classification;
- retain the self-certification and the validation evidence.

A self-certification that conflicts with the institution's own records, such as an entity claiming active status when the account shows passive income activity, must be reconciled. Relying on an inconsistent self-certification is a classification error.

### Handle Complex And Layered Ownership Structures

Complex ownership structures complicate controlling person identification. The institution must trace through layers to reach the controlling persons.

Complex structure handling:

- obtain the ownership structure of the passive NFE, including intermediate entities;
- trace through each layer to the natural persons who are controlling persons;
- apply the controlling person standard cumulatively through indirect ownership;
- identify control through means other than ownership, consistent with the AML standard;
- document the full ownership chain and the controlling persons identified;
- escalate structures where controlling persons cannot be identified.

A passive NFE owned through multiple holding companies requires tracing through each layer. Stopping at the immediate parent, or accepting an entity as a controlling person, defeats the transparency purpose. Controlling persons are natural persons.

### Apply The Correct Standard For Controlling Persons

The controlling person standard is tied to the AML or KYC standard of the jurisdiction. The institution must apply the correct threshold and definition.

Controlling person standard:

- the standard is typically the same as beneficial ownership under AML, commonly 25 percent or the jurisdiction's threshold;
- where no natural person meets the ownership threshold, controlling persons include senior managing officials;
- control through means other than ownership is included;
- the standard may differ slightly across jurisdictions, so the applicable jurisdiction's definition must be applied.

Applying the wrong threshold, or applying a fixed percentage without checking the jurisdiction's standard, produces incorrect controlling person identification.

### Re-Classify When Circumstances Change

Entity classification can change over time. An active NFE can become passive, and ownership structures can change.

Re-classification triggers:

- a change in the entity's income or asset profile that affects active versus passive status;
- a change in the controlling persons of a passive NFE;
- a change in circumstances identified through account monitoring or a new self-certification;
- the expiration or invalidation of a prior self-certification;
- periodic review of entity accounts as required by the regime.

The institution must monitor for changes and re-classify when they occur. A classification made at account opening that is never revisited becomes stale and potentially incorrect.

## Common Traps

### Defaulting To Active Without Applying The Tests

Assuming an entity is active without analyzing income and assets leads to under-reporting of passive NFEs and their controlling persons.

### Stopping At The Entity For Passive NFEs

Passive NFEs must be traced to controlling persons. Stopping at the entity misses the reportable natural persons.

### Unvalidated Entity Self-Certifications

A self-certification not checked against other information is unreliable. Inconsistencies must be resolved.

### Not Tracing Through Layered Ownership

Complex structures require tracing through each layer to natural persons. Stopping at an intermediate entity is an error.

### Applying The Wrong Controlling Person Standard

The controlling person standard is tied to the jurisdiction's AML threshold. Applying a fixed or incorrect percentage produces errors.

### No Re-Classification After Changes

Entity classification changes over time. Classifications never revisited become stale and incorrect.

### Accepting Entities As Controlling Persons

Controlling persons are natural persons. Recording an entity as a controlling person defeats the transparency purpose.

## Self-Check

- Is each entity account holder classified as financial institution, active NFE, passive NFE, or another category with documented rationale?
- Are the active NFE tests applied correctly, based on income and asset thresholds, with supporting information from the self-certification?
- For passive NFEs, are controlling persons identified using the applicable AML or KYC standard and traced through layered ownership to natural persons?
- Are entity self-certifications validated against account opening documents, AML records, and other information, with inconsistencies resolved?
- Are complex and layered ownership structures traced through each layer, with control through non-ownership means included?
- Is the correct controlling person standard applied, consistent with the jurisdiction's beneficial ownership threshold?
- Are entities re-classified when circumstances change, including income profile, ownership, and self-certification validity?
- Are the classification rationale, self-certification, validation evidence, and ownership chain documented and retained?
- Are controlling persons correctly identified as natural persons rather than entities?
- Is the entity classification approach confirmed against the applicable CRS Commentary, FATCA regulations, and national guidance rather than a generic standard?