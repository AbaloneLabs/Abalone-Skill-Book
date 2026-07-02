---
name: lawful_basis_and_consent_management.md
description: Use when the agent is selecting a lawful basis for processing personal data, designing consent flows, assessing legitimate interests, managing consent withdrawal, or reconciling GDPR opt-in rules with CCPA opt-out expectations.
---

# Lawful Basis And Consent Management

Every act of processing personal data needs a documented legal justification. Under the GDPR this is the lawful basis in Article 6, and the choice is not cosmetic: it governs whether you need consent, whether individuals can object, how long you can keep data, and what happens when a relationship ends. Under CCPA/CPRA and similar US laws the framing is different (sale, share, sensitive data, opt-out versus opt-in), but the operational question is the same: what gives you the right to process this data for this purpose, and can you prove it?

Use this skill before launching a product, adding a new data use, onboarding a vendor, or reviewing a consent banner. The goal is to make the agent identify the correct basis per processing purpose, document the decision, and avoid the common failure of defaulting to consent or legitimate interest without analysis.

## Core Rules

### Identify The Lawful Basis Per Processing Purpose

Lawful basis is determined per purpose, not per dataset or per system. One field can be processed under multiple bases depending on why it is used.

Map each processing purpose to one of the GDPR Article 6 bases:

- consent (Article 6(1)(a));
- contract (Article 6(1)(b));
- legal obligation (Article 6(1)(c));
- vital interests (Article 6(1)(d));
- public task (Article 6(1)(e));
- legitimate interests (Article 6(1)(f)).

For each purpose record:

- the specific purpose described in operational terms, not "business operations";
- the selected basis and the reasoning;
- whether special category data (Article 9) or criminal offence data (Article 10) is involved, because these need an additional condition;
- whether a corresponding transparency obligation applies.

If you cannot articulate the purpose precisely, you cannot pick the basis. A lawful basis decision made after the fact is weak evidence.

### Apply The Correct Basis To Common Scenarios

The right basis depends on necessity, not convenience.

Use contract when processing is objectively necessary to deliver what the individual asked for:

- fulfilling an order the customer placed;
- providing the core service the account was created for;
- taking steps before entering a contract such as a quote or identity check.

Do not stretch contract to cover marketing, analytics, personalization, or advertising. Those are not necessary to perform the contract.

Use legal obligation when a specific law requires the processing:

- tax recordkeeping;
- anti-money-laundering checks;
- employment law records;
- mandatory breach reporting records.

Cite the actual statute or regulation. "We are required by law" without a reference is not documentation.

Use legitimate interests only after a documented three-part test:

- purpose: is there a legitimate interest (commercial, network security, fraud prevention)?
- necessity: is the processing actually necessary, or is there a less intrusive alternative?
- balancing: does the individual's reasonable expectation and rights override your interest?

Use vital interests rarely, typically life-or-death emergencies where consent cannot be obtained. Use public task mainly for public authorities exercising official authority or legal mandate.

### Make Consent Valid, Not Merely Present

Consent is only a valid basis if it meets the quality bar. A pre-ticked box, a bundled acceptance, or a continue button that implies agreement is not valid consent.

Valid consent requires:

- freely given: no detriment for refusing, no bundled services, no power imbalance (employment contexts are problematic);
- specific: separate consent per distinct purpose, not one global acceptance;
- informed: who you are, what data, which purposes, who receives it, how to withdraw;
- unambiguous: a clear affirmative action, silence or pre-ticked boxes do not count;
- withdrawable: as easy to withdraw as to give.

For special category data, children's data, and most cookies, consent is usually the required basis and the bar is higher. Record the consent event: what was shown, when, what version of the notice, and the mechanism.

### Document And Balance Legitimate Interests

Legitimate interest is flexible but carries a documentation burden that teams underestimate. A legitimate interest assessment (LIA) is not a paragraph of boilerplate.

A defensible LIA includes:

- the interest being pursued and why it is legitimate;
- the data minimization choices made;
- whether individuals would reasonably expect the processing;
- whether the individual could suffer harm, discrimination, or surprise;
- safeguards applied such as aggregation, pseudonymization, opt-out, or rate limiting;
- the conclusion that the interest is not overridden.

Pure marketing to existing customers can sometimes rely on legitimate interest (or the soft opt-in under ePrivacy), but behavioral advertising, profiling with significant effects, and processing children's data usually cannot.

### Reconcile GDPR Opt-In With CCPA Opt-Out

GDPR and CCPA/CPRA describe different models and teams often confuse them. Do not assume a single global setting works.

Reconcile the regimes:

- GDPR: consent is often opt-in before processing for non-essential purposes.
- CCPA/CPRA: the default is that processing may proceed, but consumers have opt-out rights for sale, share, and cross-context behavioral advertising, plus opt-in for minors.
- Sensitive personal information under CPRA triggers a right to limit use, which is operationally closer to opt-in for certain secondary uses.

Map each purpose to the controlling regime per jurisdiction, and design controls that satisfy the strictest applicable rule rather than guessing.

### Keep Basis Decisions Alive

Lawful basis is not a one-time decision. It must be reviewed when purpose, data, recipients, retention, or technology change.

Set review triggers:

- new secondary use of existing data;
- sharing with a new vendor or affiliate;
- new analytics, AI, or personalization feature;
- acquisition or product pivot;
- change in how children or sensitive data is handled;
- regulator guidance update.

Keep a register of lawful basis decisions with version history so you can show the basis in force at the time of a complaint or request.

## Common Traps

### Defaulting To Consent Because It Sounds Safe

Consent is the most fragile basis because it can be withdrawn at any time, and weak consent is no basis at all. If processing is genuinely necessary for the contract or required by law, relying on consent creates avoidable risk and operational churn.

### Bundling Consent Across Purposes

A single accept button that covers essential service, marketing, profiling, and advertising is invalid under GDPR. Each distinct purpose needs a separate, granular choice.

### Treating Legitimate Interest As A Loophole

Legitimate interest is not an escape hatch when consent is inconvenient. Without a documented balancing test it fails, and the individual retains the right to object.

### Ignoring The Difference Between Sale And Processing

CCPA obligations hinge on definitions of sale and share. Saying "we do not sell data" while passing identifiers to adtech for cross-context advertising is a common and costly mismatch.

### Failing To Make Withdrawal As Easy As Giving Consent

If withdrawal is buried in account settings while consent was a prominent button, the consent is not validly withdrawable and the whole basis is at risk.

### Forgetting Special Category Data Conditions

Article 9 data (health, biometrics, ethnicity, religion, sexual orientation, political opinions, trade union membership, genetic data) needs both an Article 6 basis and an Article 9 condition. Many teams capture the Article 6 basis and stop.

### Reusing An Old Basis For A New Purpose

Repurposing data collected for billing into marketing or AI training without a fresh basis analysis is one of the most common enforcement themes.

## Self-Check

- Is a lawful basis identified and documented for each distinct processing purpose, not per dataset?
- Is the chosen basis the correct one (contract for core service delivery, legal obligation with a cited law, legitimate interest with a balancing test)?
- If consent is used, is it freely given, specific, informed, unambiguous, and as easy to withdraw as to give?
- For legitimate interest, is there a documented three-part LIA covering purpose, necessity, and balancing?
- Are special category data and children's data handled with the additional conditions they require?
- Are GDPR opt-in expectations and CCPA/CPRA opt-out rights reconciled per purpose and jurisdiction?
- Is the consent or basis event recorded with version, timestamp, and mechanism?
- Are basis decisions reviewed when purpose, recipients, retention, or technology change?
- Is marketing, analytics, personalization, and adtech separated from essential processing rather than bundled?
- Can the organization demonstrate the basis in force at the time of a complaint or individual request?
