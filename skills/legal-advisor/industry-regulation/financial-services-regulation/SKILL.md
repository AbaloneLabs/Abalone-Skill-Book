---
name: financial_services_regulation.md
description: Use when the agent is advising on financial services regulation, banking or securities licensing, capital and prudential requirements, consumer finance rules, AML and KYC obligations, fintech sandbox eligibility, payment services, digital assets, or reviewing regulatory exposure and enforcement risk in a regulated financial business.
---

# Financial Services Regulation

Financial services is one of the most heavily regulated commercial sectors. A business that takes deposits, lends, arranges investments, moves money, issues payment instruments, underwrites securities, or intermediates risk is usually caught by a licensing perimeter that varies by activity, instrument, and jurisdiction. Agents tend to underestimate this perimeter because many modern products (wallets, BNPL, robo-advice, tokenized assets, embedded finance) look like software features but are treated by regulators as regulated activities.

The judgment problem is not "which license do we need" as a single lookup. It is whether a given product feature triggers a regulated activity, in which jurisdictions, under which authority, with what capital, conduct, reporting, and governance obligations, and whether an exemption, sandbox, or partner-bank model genuinely removes the obligation or merely relocates the liability. Getting this wrong can void contracts, trigger restitution and fines, attract personal liability for directors, and in some jurisdictions expose individuals to criminal sanction.

This skill is advisory only. Regulatory perimeters differ by country and change frequently. Confirm any licensing, capital, or conduct conclusion with qualified counsel in each relevant jurisdiction before acting.

## Core Rules

### Map the activity to the regulatory perimeter first

Before choosing a product architecture, determine whether the activity is a regulated activity in each target market. Regulated activities are usually defined functionally (taking deposits, lending, arranging deals in investments, advising on investments, dealing as principal or agent, operating a collective scheme, effecting insurance, issuing e-money, transmitting money).

- A strong analysis lists each product feature and tests it against each regulated-activity definition, rather than assuming "we are a tech company, not a bank."
- A weak analysis picks a license by analogy to a competitor without reading the statute.
- Capture the full chain: origination, distribution, custody, settlement, and servicing may each be separately regulated even when one party performs them.

### Distinguish authorization, registration, and exemption

Not every regulated activity requires a full license. Options typically include full authorization, limited registration, passporting (in blocs like the EU/EEA), agency or appointed-representative models, white-label or sponsor-bank arrangements, and statutory exemptions for small-scale or limited-scope activity.

- Each option carries different obligations, timelines, and residual liability. A sponsor-bank model does not make the fintech unregulated; it often makes the fintech a service provider subject to outsourcing rules and indirect supervision.
- Document the legal basis for any exemption and the conditions that must remain true for it to hold. If the business scales past a threshold, the exemption may lapse.

### Identify the prudential and ongoing obligations

A license is the beginning, not the end. Regulated firms usually owe ongoing duties: minimum initial and ongoing capital, liquidity and leverage ratios, fit-and-proper tests for controllers and senior managers, governance and risk-management frameworks, conduct-of-business rules, client-money and asset-safeguarding rules, transaction reporting, prudential reporting, recordkeeping, and resolution and recovery planning for larger firms.

- Strong practice maintains an obligations register tied to each license condition, with owners and review dates.
- Weak practice obtains a license and then treats compliance as an annual filing.

### Treat AML/CFT as a first-class obligation

Anti-money-laundering and counter-financing-of-terrorism rules apply broadly, often to entities that are not otherwise licensed as financial institutions, including certain agents, crypto-asset service providers, and lending platforms. Core duties usually include customer due diligence (CDD), enhanced due diligence for higher risk, ongoing monitoring, beneficial-ownership identification, suspicious-activity reporting, sanctions screening, recordkeeping, and an independent AML audit.

- Determine the firm's reporting entity status early; it drives the entire AML program.
- Reliance on a third party for CDD does not transfer the legal responsibility for the relationship.

### Apply conduct-of-business and consumer-protection rules

Regulators increasingly police how products are sold and serviced, not just whether the firm is licensed. Expect rules on suitability and appropriateness, disclosure and key-facts documents, best-execution, conflicts of interest, complaints handling, product governance and target-market assessment, responsible lending and affordability, and fair-treatment outcomes.

- Design the customer journey to evidence fair treatment, not merely to comply with form-filling.
- For consumer credit, responsible-lending rules may require affordability assessments that restrict the product more than the firm's risk appetite would.

### Evaluate fintech, sandbox, and digital-asset pathways carefully

Sandboxes and innovation hubs can reduce time-to-market but usually do not waive the law. They provide supervised testing, temporary relief, or no-action positions, often with conditions and reporting burdens. Digital assets (tokens, stablecoins, crypto exchanges) are increasingly brought inside existing or new regulatory perimeters (e.g., MiCA, money-transmitter rules, securities frameworks).

- Classify the digital asset functionally: is it a security, a payment instrument, e-money, a commodity, a utility, or something novel? The classification drives the regime.
- Stablecoin issuance and custody are increasingly regulated as payment systems or e-money; do not assume "decentralized" removes liability.

### Allocate regulatory liability in partnerships

Embedded-finance, banking-as-a-service, and white-label arrangements distribute regulated functions across parties. Regulators hold the licensed entity responsible for outsourced functions and increasingly hold the technology provider accountable for failings that harm consumers.

- Contracts should allocate responsibility for regulatory breaches, audit rights, data access on termination, and step-in rights.
- A bank partner can terminate the arrangement on regulatory concern, which can collapse a fintech's business overnight; plan for continuity.

### Plan for cross-border and group-wide supervision

A firm operating across borders may face consolidated supervision, home/host authority allocation, local incorporation requirements, data-localization, and local-director requirements. Some activities require a local presence; others can be passported.

- Do not assume one EU or one US license covers all activity globally; each state and country can have its own perimeter.
- Group structures may trigger holding-company supervision and recovery planning.

## Common Traps

### "We are just a technology provider, not a regulated entity"

The most common and dangerous trap. If the firm performs regulated functions, holds client funds, sets pricing, or controls the customer relationship, regulators will look through the label. Arranging or facilitating a regulated activity is often itself regulated. Confirm the analysis against the functional definitions, not the marketing language.

### Choosing a license by competitor analogy

Competitors may be operating under a grandfathered regime, an exemption that no longer applies, a no-action letter that is fact-specific, or may simply be non-compliant. Copying their structure copies their risk without their context. Read the statute and any applicable guidance.

### Treating the sponsor bank as the compliance department

In banking-as-a-service, the fintech remains responsible for its own conduct, AML where it is the distributor or agent, marketing accuracy, complaints, and data protection. The bank's compliance program does not substitute for the fintech's, and regulators have pursued both parties when controls fail.

### Underestimating AML scope

Firms often assume AML applies only to banks. Many non-bank lenders, money transmitters, crypto platforms, crowdfunding portals, and even certain art and luxury-goods dealers are reporting entities. Missing AML registration can be a standalone enforcement risk independent of the core license.

### Assuming sandbox participation waives the law

Sandboxes usually provide supervised testing within boundaries, not immunity. Exiting the sandbox without a path to full authorization can force product withdrawal. Read the sandbox's exit conditions before relying on it.

### Ignoring state-level and sub-national variation

In federal systems, states often license separately (US money-transmitter laws, US state securities "blue sky" rules, Canadian provincial securities rules). A single federal determination rarely disposes of the question. Budget for multi-state analysis.

### Treating digital-asset classification as settled

Classification is jurisdiction-specific and evolving. A token that is a utility in one country may be a security or e-money in another. A single global token offering without jurisdiction-by-jurisdiction analysis creates broad exposure. Re-classify when the token's economics or governance changes.

### Filing as compliance

Obtaining a license and filing periodic returns is necessary but not sufficient. Regulators assess governance, risk culture, conduct outcomes, and complaints trends. A firm that files on time but harms consumers will still face enforcement.

## Self-Check

- For each product feature, have I tested it against each regulated-activity definition in each target jurisdiction, and recorded the conclusion and the legal basis?
- Have I identified whether the firm needs full authorization, registration, an exemption, or a partner model, and documented the conditions that must remain true for that basis to hold?
- Is there an obligations register covering capital, governance, conduct, reporting, AML, recordkeeping, and complaints, with owners and review dates?
- Have I determined the firm's AML/CFT reporting-entity status and the corresponding CDD, monitoring, reporting, and audit duties?
- For any partner-bank or embedded-finance arrangement, have I allocated regulatory liability, audit rights, data access on termination, and business-continuity risk in writing?
- For digital assets, have I classified the asset functionally in each jurisdiction and re-confirmed the classification if the economics changed?
- Have I considered state, provincial, or sub-national licensing in addition to any national authorization?
- If relying on a sandbox, exemption, or no-action position, do I know the exit conditions and the path to full authorization?
- Have I flagged that perimeters change and that conclusions must be confirmed with qualified local counsel before launch?
