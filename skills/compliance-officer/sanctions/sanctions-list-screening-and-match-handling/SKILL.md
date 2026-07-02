---
name: sanctions_list_screening_and_match_handling.md
description: Use when the agent is screening customers, counterparties, payments, or transactions against OFAC SDN and Consolidated lists, EU, UN, and UK OFSI sanctions lists, deciding whether a match is a true positive, escalating hits, or choosing between blocking and rejecting a transaction.
---

# Sanctions List Screening And Match Handling

Sanctions screening is the front line of trade and financial controls, and it is also where most preventable violations happen. A screening tool that returns no hits is not evidence of compliance; it may reflect stale lists, missing data fields, a misconfigured population, or a threshold set so high that real targets pass through. Conversely, a flood of alerts is not evidence of diligence; most may be false positives that are being cleared without genuine analysis. The compliance officer's job is to make sure the right parties are screened, against the right lists, with a match-handling process that can survive regulatory scrutiny.

Sanctions liability is generally strict. A US person or US-origin transaction that benefits a blocked party is a violation even if no one intended it. That means the screening program must be designed to catch, not merely to document effort. Use this skill before advising on screening scope, match disposition, escalation, blocking versus rejection, or the evidentiary record needed to defend a no-action or enforcement posture.

## Core Rules

### Screen The Right Population At The Right Points

Screening programs fail most often because the population is incomplete. Define exactly who and what is screened, and at which trigger points.

Cover at minimum:

- direct customers and account holders at onboarding and periodically thereafter;
- beneficial owners and controlling persons, not only the named contracting party;
- counterparties in payments, wire transfers, and correspondent banking, including originators and beneficiaries;
- vendors, suppliers, distributors, freight forwarders, and consignees in trade flows;
- vessels, aircraft, and their IMO or tail numbers for maritime and aviation exposure;
- intermediate banks and clearing institutions in cross-border payments;
- investors, directors, and signatories in onboarding for legal entities;
- goods descriptions and free-text payment references for sanctioned geography or parties.

Screen at onboarding, at each material transaction, at list update, and on a periodic risk-based cycle. One-time screening at onboarding is insufficient because lists change and ownership changes.

### Use The Correct Lists And Keep Them Current

OFAC maintains multiple lists with different legal effects, and the EU, UN, and UK maintain their own. Do not treat "sanctions list" as one thing.

Distinguish:

- OFAC SDN List: blocking. Assets of SDNs are frozen and transactions prohibited;
- OFAC Consolidated Sanctions List (CSL), including NS-PLC, SSI, and other program-specific entries;
- Sectoral Sanctions Identifications (SSI) List: directives-based restrictions, not full blocking;
- OFAC Non-SDN Menu-Based Sanctions List and foreign sanctions evaders lists;
- EU consolidated list of persons and entities under restrictive measures;
- UN Security Council Consolidated List, binding on all member states;
- UK OFSI consolidated list, applicable to UK persons and conduct in the UK.

Confirm the update cadence. OFAC, EU, UN, and UK publish updates on different schedules and sometimes intraday. A screening engine running on a stale list can miss a same-day designation. Record the list version and timestamp against each screening event so the state of the list at the time of decision is reconstructable.

### Apply The 50 Percent Rule To Ownership

A party does not need to be listed to be blocked. Under OFAC's 50 Percent Rule, an entity that is 50 percent or more owned, individually or in the aggregate, by one or more SDNs is itself blocked even if it is not separately listed. EU and UK rules have analogous, though not identical, control tests. Screening the named entity alone is insufficient if its ownership includes an SDN. See the dedicated ownership-and-control skill for aggregation analysis.

### Distinguish True Matches From False Positives Methodically

Most alerts are false positives, but clearing them too quickly is the most common root cause of enforcement actions. A match disposition must be documented against defined criteria.

Assess:

- full name match versus partial or phonetic match;
- additional identifiers: date of birth, national ID, passport, address, tax ID, place of birth;
- nationality and residency overlap;
- known aliases and transliteration variants;
- business activity, sector, and geography consistency with the listed party;
- vessel IMO number, call sign, and registered owner;
- ownership links to a listed party.

Record which identifiers matched, which did not, and the reasoning. "Cleared as false positive" with no rationale is not defensible. Require a second-line review or four-eyes check for near matches and for high-risk jurisdictions.

### Decide Block Versus Reject Correctly

When a true match to an SDN is confirmed, the legal response depends on the program and the transaction stage.

- Blocking applies to SDNs and comprehensive embargo programs: the property or funds must be frozen, placed in a blocked account, and reported. The funds may not be returned, transferred, or used.
- Rejecting applies where the transaction is prohibited but there is no blockable interest, for example a payment to a comprehensively embargoed jurisdiction that has not yet been received. The transaction is refused and not processed, and the rejection is reported.

The distinction matters because returning blocked funds is itself a violation. Train operations staff to recognize that a suspected SDN match freezes the funds pending review; they must not unwind the transaction to avoid the problem.

### Escalate True Matches Through A Defined Path

A confirmed match must trigger a defined workflow, not an ad hoc decision.

Include:

- immediate freeze or hold on the transaction or account;
- notification to the sanctions compliance officer or designated escalation point;
- a blocking or rejection notice prepared from approved templates;
- regulatory reporting within required timeframes (OFAC blocking reports within ten business days, annual reports of blocked property);
- recordkeeping of the action, basis, and supporting data;
- decision on whether the matter is an apparent violation requiring self-disclosure analysis.

### Maintain An Audit Trail For Every Decision

Regulators reconstruct what happened from records, not memory. Each screening event, alert, disposition, and escalation must leave an immutable, time-stamped record with the decision-maker, the data reviewed, and the rationale.

## Common Traps

### Treating Zero Hits As Evidence Of Compliance

A clean screen can reflect a broken population, a missing field, or a stale list. Validate the screening engine periodically with known-listed test names to confirm it would catch a real target.

### Clearing Alerts On Name Alone

Common names produce many false positives, but clearing them without checking date of birth, nationality, identifiers, or ownership creates real risk. Require identifier-based dispositions.

### Screening Only The Contracting Party

The named customer may be clean while its ultimate beneficial owner is an SDN. Failing to screen ownership and control is a recurring enforcement theme.

### Returning Funds That Should Be Blocked

Operations staff under pressure may reverse a payment to resolve an alert. If the funds are blockable, reversal is a separate violation. Freeze first, then analyze.

### Using A Single Global List

Running only the OFAC SDN list ignores EU, UN, and UK obligations and ignores OFAC's own sectoral and non-SDN lists. Match list scope to the institution's jurisdictional exposure.

### Ignoring Free-Text And Payment Reference Fields

Sanctions evasion increasingly relies on payment messaging. Screening only structured name fields misses references to sanctioned regions, vessels, or parties embedded in remittance information.

### Relying On Customer Self-Attestation Of Ownership

Customer-provided ownership data may be incomplete or stale. Corroborate beneficial ownership for higher-risk relationships and rescreen on ownership change.

## Self-Check

- Is the screened population defined to include customers, beneficial owners, counterparties, vessels, intermediaries, and consignees, not only the contracting party?
- Are screening trigger points defined for onboarding, each transaction, list updates, and periodic risk-based rescreening?
- Are OFAC SDN, CSL, SSI, EU, UN, and UK OFSI lists all in scope where jurisdictionally relevant, with documented update cadence and list-version recording?
- Does each match disposition record matched and non-matched identifiers and the reasoning, rather than only a cleared status?
- Is the 50 Percent Rule applied so that entities owned in the aggregate by SDNs are treated as blocked even when unlisted?
- Is the block-versus-reject decision based on the program and transaction stage, with frozen funds never returned or transferred pending review?
- Does a confirmed match trigger a defined escalation path with freeze, notification, reporting, and recordkeeping?
- Are blocking and rejection reports filed within required timeframes and is an annual blocked property report produced?
- Does every screening event leave an immutable, time-stamped audit trail with decision-maker and rationale?
- Is the screening engine tested periodically with known-listed names to confirm it would catch a real target?
