---
name: beneficial_ownership_identification_and_tracing.md
description: Use when the agent is identifying ultimate beneficial owners, tracing ownership through layered legal entities and trusts, applying ownership thresholds, handling nominees and control-through-other-means, or determining who ultimately owns or controls a customer for AML and transparency compliance.
---

# Beneficial Ownership Identification And Tracing

Beneficial ownership identification is the process of looking past the legal entity that is the nominal customer to find the natural person who ultimately owns or controls it. Under FATF Recommendation 24, the FinCEN Customer Due Diligence Rule, the EU AMLD, the US Corporate Transparency Act, and equivalent national regimes, obliged entities must identify the beneficial owners of their legal entity customers and keep that information current. The central judgment problem is that ownership is rarely a simple majority shareholder. It can be layered through chains of holding companies, obscured by nominees and professional directors, held through trusts and foundations, or exercised through control mechanisms that have nothing to do with shareholding. An institution that records the nominal entity without tracing to the natural person has not identified the customer at all.

Use this skill before designing beneficial ownership identification procedures, tracing ownership through complex structures, applying ownership thresholds, or advising on how to handle nominees, trusts, and control-through-other-means. The goal is to make the agent think about ownership chains, threshold application, non-ownership control, and the decisions that are easy to make too casually. Accepting a nominee or stopping at the first holding company defeats the entire purpose of beneficial ownership transparency.

This skill addresses jurisdiction-specific obligations. Ownership thresholds, UBO definitions, and tracing requirements differ across FATF member states, the United States, the European Union, and other national regimes. Always confirm the applicable national law and regulator guidance, and consult qualified AML or legal counsel for specific ownership-tracing decisions.

## Core Rules

### Apply The Correct Ownership Threshold

Beneficial ownership identification turns on a threshold that defines when a natural person's ownership interest is significant enough to require identification. The threshold varies by jurisdiction.

Common thresholds and rules:

- many regimes use 25 percent direct or indirect ownership as the trigger for identification;
- some regimes use a lower threshold, such as 10 or 15 percent, or require identification of any owner above a de minimis level;
- the threshold applies to both direct ownership and indirect ownership through chains of entities;
- where no natural person meets the ownership threshold, at least one senior managing official must be identified;
- control through means other than ownership must be identified regardless of the ownership threshold.

The institution must apply the threshold required by the applicable regime, not a generic default. Using the wrong threshold, or applying it only to direct ownership, is a fundamental error.

### Trace Through Chains Of Entities To The Natural Person

Ownership is often layered through multiple legal entities. The institution must trace through the entire chain until it reaches the natural person who ultimately owns or controls the customer.

Tracing process:

- obtain the ownership structure of the customer entity, including all intermediate holding companies;
- for each intermediate entity, obtain its ownership structure;
- continue tracing until every ownership chain ends at a natural person;
- apply the ownership threshold cumulatively through indirect holdings;
- document the full ownership chain, not just the immediate parent.

A common error is to record the immediate parent company and stop. If the parent is itself owned by other entities, the chain is incomplete. The beneficial owner is the natural person at the end of the chain, not the intermediate entity.

### Identify Control Through Means Other Than Ownership

Beneficial ownership is not limited to shareholding. A person can control an entity through voting rights, board appointment power, contractual arrangements, or other means without holding any shares.

Non-ownership control mechanisms to identify:

- voting rights that exceed the ownership percentage through dual-class shares or voting agreements;
- power to appoint or remove the majority of the board;
- contractual control through shareholder agreements, options, or veto rights;
- control exercised through a trust, where the trustee or protector holds effective control;
- control through nominee arrangements where the nominee acts on instructions;
- de facto control where a person exercises dominant influence informally.

The institution must ask how control is exercised, not only who holds shares. A structure where ownership is dispersed but one person controls the board is controlled by that person.

### Handle Nominees And Professional Directors Correctly

Nominees, professional directors, and bare trustees are legal title holders who act on behalf of others. Recording them as the beneficial owner hides the true controller.

Handling nominees:

- identify when a shareholder or director is acting as a nominee;
- require disclosure of the person on whose behalf the nominee acts;
- trace through the nominee to the actual beneficial owner;
- escalate structures where the nominee refuses or cannot disclose the principal;
- treat opaque nominee arrangements as a risk factor requiring enhanced diligence.

Professional directors and corporate service providers can serve a legitimate function, but when they appear across many unrelated entities with no clear principal, it is a red flag for obscured ownership.

### Address Trusts, Foundations, And Similar Arrangements

Trusts, foundations, and similar structures separate legal and beneficial ownership in ways that complicate identification. The institution must identify the relevant parties, not just the trustee.

For trusts, identify:

- the settlor who created the trust;
- the trustee who holds legal title;
- the protector if one exists and holds control powers;
- the beneficiaries or class of beneficiaries;
- any other person with effective control, such as a protector with removal powers.

For foundations, identify the founder, the council or board, and the beneficiaries. These structures are not inherently illicit, but they require deeper tracing than a simple corporate shareholder.

### Re-Identify Ownership When The Structure Changes

Beneficial ownership is not static. Entities restructure, shareholders change, and new parties are introduced. The institution must re-identify beneficial ownership when the structure changes.

Re-identification triggers include:

- change in shareholders or ownership percentages;
- addition of a new holding company or intermediate entity;
- change in directors or authorized signers;
- introduction of a trust or nominee arrangement;
- corporate restructuring, merger, or acquisition affecting the customer;
- periodic review due date for high-risk entities.

A beneficial ownership snapshot taken at onboarding and never refreshed will become stale. Ongoing CDD must include ownership refresh.

### Escalate When Ownership Cannot Be Traced

When ownership cannot be traced to a natural person, the institution must escalate rather than accept an incomplete picture.

Escalation options:

- apply enhanced due diligence to the relationship;
- require additional documentation or independent verification;
- obtain senior management approval to continue;
- consider restricting the account or product access;
- exit the relationship if ownership remains opaque after reasonable effort.

Accepting a structure where no beneficial owner can be identified, without escalation, is not compliant CDD. The inability to trace ownership is itself a risk indicator.

## Common Traps

### Stopping At The Immediate Parent

Recording the parent company without tracing further leaves the chain incomplete. The beneficial owner is the natural person at the end, not an intermediate entity.

### Applying The Threshold Only To Direct Ownership

Indirect ownership through chains must be calculated cumulatively. Applying 25 percent only to direct holdings misses owners who control through layered structures.

### Recording The Nominee As The Beneficial Owner

Nominees hold legal title for others. Recording the nominee without tracing to the principal hides the true controller.

### Ignoring Non-Ownership Control

A person who controls the board or holds contractual control is a beneficial owner even with zero shares. Ask how control is exercised, not only who holds stock.

### Treating Trusts As Opaque

Trusts have identifiable parties. Failing to identify settlor, trustee, protector, and beneficiaries leaves the ownership picture incomplete.

### No Ownership Refresh After Restructuring

Ownership changes over time. A snapshot taken at onboarding and never refreshed becomes inaccurate and indefensible.

### Accepting Untraceable Ownership Without Escalation

When ownership cannot be traced, the institution must escalate, restrict, or exit. Proceeding without escalation is a serious deficiency.

## Self-Check

- Is the correct ownership threshold applied for the applicable jurisdiction, covering both direct and indirect ownership?
- Is ownership traced through the full chain of entities until every chain ends at a natural person?
- Is control through means other than ownership, including voting rights, board appointment, and contractual control, identified and documented?
- Are nominees, professional directors, and bare trustees traced to the actual principal rather than recorded as the beneficial owner?
- For trusts and foundations, are settlor, trustee, protector, council, and beneficiaries identified as required?
- Is beneficial ownership re-identified when the structure changes, including ownership, director, or restructuring events?
- When ownership cannot be traced, is the relationship escalated for EDD, restriction, or exit rather than accepted as-is?
- Is the full ownership chain documented, including intermediate entities and the rationale for the threshold calculation?
- Are opaque or nominee-heavy structures treated as risk factors requiring enhanced diligence?
- Is the beneficial ownership design confirmed against the applicable national law and regulator guidance rather than a generic standard?