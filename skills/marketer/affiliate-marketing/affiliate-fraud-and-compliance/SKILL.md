---
name: affiliate_fraud_and_compliance.md
description: Use when the agent is monitoring an affiliate program for fraud such as cookie stuffing brand bidding typosquatting or fake leads, designing fraud detection and clawback processes, ensuring FTC disclosure compliance, or weighing network versus in-house tradeoffs for fraud control.
---

# Affiliate Fraud And Compliance

Affiliate fraud is not a rare edge case; it is a persistent pressure that exploits every gap in program design. Because affiliates are paid on outcomes, the incentive to manufacture or intercept those outcomes is structural. Fraud drains budget, distorts attribution, damages brand through deceptive practices, and creates legal exposure when disclosures are missing or claims are false. Compliance failures, especially around FTC disclosure and advertising truthfulness, turn a marketing channel into a regulatory and reputational liability. Programs fail when fraud is treated as a one-time cleanup rather than continuous monitoring, when clawback processes are absent, and when disclosure obligations are left to partners' discretion.

Use this skill before building fraud monitoring, drafting compliance requirements, handling clawbacks, or choosing between network and in-house program infrastructure. The goal is to prevent the agent from treating fraud as an afterthought and from assuming that a network's default protections are sufficient.

## Core Rules

### Understand The Fraud Types You Must Defend Against

Different fraud types exploit different parts of the attribution and payment chain. You cannot monitor what you cannot name.

Major fraud types:

- **Cookie stuffing** forces tracking cookies onto users without a genuine click, often through hidden iframes or browser extensions, so the fraudster is credited for sales they did not influence.
- **Brand and trademark bidding** has affiliates bid on the brand's search terms to capture users already searching for the brand, then claim commission for intercepting existing intent.
- **Typosquatting** registers misspelled domains to catch users mistyping the brand, redirecting them through affiliate links.
- **Fake leads and sales** manufacture conversions through bots, stolen identities, or self-purchases to generate commission.
- **Adware and toolbar injection** uses software installed on a user's device to overwrite or inject affiliate cookies at the point of purchase.
- **Incentivized or forced clicks** trick users into clicking through to set cookies without genuine interest.

Map each type to a detection signal and a prevention control. Generic monitoring misses type-specific patterns.

### Build Continuous Monitoring, Not Periodic Cleanup

Fraud does not wait for quarterly reviews. By the time an audit catches it, budget has leaked and patterns have normalized.

Monitor continuously for:

- spikes in conversions from individual partners or sources;
- conversion patterns that look automated or unnaturally regular;
- high rates of last-click attribution from coupon or extension traffic;
- brand-search keyword overlap with affiliate traffic;
- mismatches between partner-reported reach and actual conversions;
- sudden surges in low-quality or high-return conversions.

Set thresholds and alerts. Treat anomalies as investigation triggers, not as confirmation of fraud by themselves.

### Define And Enforce Clawback Authority

The ability to reverse commissions is what makes the program enforceable. Without clawback, terms are suggestions.

Define clawback for:

- confirmed fraud of any type;
- sales that refund, return, or charge back within a defined window;
- violations of brand bidding or trademark rules;
- undisclosed or non-compliant promotions;
- leads that fail quality validation.

Specify the clawback window, the evidence required, the dispute process, and how reversals are executed in the tracking system. Make clawback a standard operational step, not an exceptional escalation.

### Require FTC And Disclosure Compliance

Affiliate promotions are advertising and are subject to disclosure and truth-in-advertising law. The brand is responsible, not just the partner.

Require:

- clear and conspicuous disclosure of the affiliate relationship, such as #ad or affiliate disclaimers where the content is consumed;
- truthful product claims, no unsupported efficacy or earnings statements;
- compliance with platform-specific advertising rules;
- disclosures in the same language and medium as the promotion;
- documentation that partners acknowledge and follow disclosure rules.

Build disclosure into onboarding and monitoring. Do not rely on partners to self-police, because non-compliance rebounds on the brand.

### Separate Genuine Value From Interception In Attribution

Fraud and low-value interception are related problems: both pay commission for outcomes the brand did not need help creating.

Distinguish by:

- incrementality testing on suspect partner segments;
- comparing new-customer rates across partners;
- analyzing the path to purchase and where the affiliate touch occurs;
- holding out brand-search traffic from affiliate attribution where possible.

Interception is not always fraud, but paying for it is always waste. Treat persistent interception as a terms violation even when it is technically within the rules.

### Weigh Network Versus In-House For Fraud Control

The choice of infrastructure shapes your fraud defenses.

Networks typically provide:

- pre-built fraud monitoring and partner vetting;
- standardized terms and dispute processes;
- aggregated data across many advertisers;
- but less transparency into individual partner behavior and less custom control.

In-house programs provide:

- full visibility into raw traffic and conversion data;
- custom detection rules and immediate intervention;
- direct control over terms and clawback;
- but require building and maintaining the entire fraud stack.

Network protections reduce effort but are not a substitute for your own monitoring. Many fraud patterns are brand-specific and invisible to a generic network filter.

### Document Incidents And Responses

Fraud handling must be repeatable and defensible. Ad hoc responses create inconsistency and weaken enforcement.

For each incident, document:

- the fraud type and evidence;
- the partners and transactions involved;
- the financial impact and recovery via clawback;
- the corrective action and partner outcome;
- the monitoring rule added to catch recurrence.

This record supports future enforcement, demonstrates good-faith compliance to regulators, and reveals systemic weaknesses.

## Common Traps

### Treating Fraud As A One-Time Cleanup

Fraud is continuous pressure. A single audit removes current abusers but does nothing to prevent the next wave.

### Assuming The Network Catches Everything

Network-level fraud filters are generic. Brand bidding, typosquatting, and brand-specific interception often pass network checks and require your own monitoring.

### No Clawback Process

Without a defined, exercised clawback process, confirmed fraud still gets paid. Terms without enforcement are decoration.

### Disclosure Left To Partner Discretion

When disclosure compliance is optional, a meaningful share of partners will omit it, exposing the brand to regulatory action and reputational harm.

### Confusing Interception With Fraud

Not all non-incremental traffic is fraud, but treating it as acceptable because it is technically within the rules still drains margin.

### Overreacting With Broad Partner Purges

Indiscriminate termination of partners after a fraud scare removes good partners along with bad ones and damages program reputation.

### Ignoring Returns And Chargebacks In Commission

Paying commission on sales that later refund or charge back turns the program into a net loss on those transactions.

## Self-Check

- [ ] The major fraud types (cookie stuffing, brand bidding, typosquatting, fake leads, adware, forced clicks) are each mapped to detection signals and controls.
- [ ] Monitoring is continuous with thresholds and alerts, not periodic manual audits.
- [ ] A clawback process is defined with windows, evidence standards, dispute handling, and system execution.
- [ ] FTC and disclosure compliance is required, documented, and monitored, not left to partner discretion.
- [ ] Incrementality testing separates genuine partner value from interception, and persistent interception is treated as a terms issue.
- [ ] The network-versus-in-house decision is justified by the level of fraud control and transparency required.
- [ ] Product claims made by affiliates are reviewed for truthfulness and substantiation.
- [ ] Returns, refunds, and chargebacks are netted out of commission before payout.
- [ ] Incidents are documented with evidence, impact, recovery, and corrective action.
- [ ] Enforcement is consistent and targeted, avoiding indiscriminate purges that remove valuable partners.
