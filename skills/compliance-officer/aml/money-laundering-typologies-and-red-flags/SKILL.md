---
name: money_laundering_typologies_and_red_flags.md
description: Use when the agent is mapping money laundering typologies across placement, layering, and integration, building red flag libraries, or detecting shell companies, professional enablers, trade-based laundering, and virtual asset methods.
---

# Money Laundering Typologies And Red Flags

Money laundering is the process of disguising the illicit origin of criminal proceeds to make them appear legitimate. The classic three-stage model divides the process into placement (introducing illicit funds into the financial system), layering (moving and obscuring the trail through complex transactions), and integration (returning the laundered funds to the criminal under an apparently legitimate guise). FATF, FinCEN, the Egmont Group, and national FIUs publish typology reports that describe how these stages manifest in practice. An AML program that does not understand the typologies relevant to its customers and products cannot detect them.

Use this skill before building red flag libraries, designing monitoring scenarios, training investigators, or assessing whether observed activity matches a known laundering method. The goal is to make the agent think about how each stage works, which typologies are relevant, and which red flags should trigger deeper review. Generic "be alert for unusual activity" guidance is useless; investigators need specific, typology-anchored indicators.

## Core Rules

### Understand The Three Stages And Their Indicators

Each stage of laundering has characteristic patterns. Recognizing the stage helps identify the typology.

- placement: cash deposits, bulk cash smuggling, purchase of monetary instruments, commingling with legitimate business receipts, use of money mules and smurfs;
- layering: wire transfers between accounts and jurisdictions, conversion between currencies and assets, use of shells and nominees, purchase and sale of high-value assets, virtual asset conversion and mixing;
- integration: purchase of real estate and luxury goods, investment in legitimate businesses, loan-back arrangements, repayment of fictitious loans, integration through trade.

Few real cases follow a clean three-stage path, but the framework helps map where activity sits and what it is trying to achieve.

### Build Typology-Anchored Red Flag Libraries

Red flags are most useful when tied to a specific typology. A library of disconnected indicators is hard to apply.

For each typology, document:

- the laundering method and objective;
- the specific behavioral indicators;
- the data and transactions that would reveal it;
- the customer or product context where it is most likely;
- the monitoring scenarios that should detect it;
- the investigation steps to confirm or rule out.

Update the library as new typologies emerge from FATF, FinCEN advisories, FIU reports, and internal cases.

### Detect Shell And Front Company Abuse

Shell companies with no real operations are a core layering and integration tool. Front companies have a nominal business used to commingle illicit and legitimate funds.

Indicators:

- entity with no physical premises, employees, or online presence;
- business activity inconsistent with the stated purpose;
- transactions that do not match the entity's profile;
- nominee directors and shareholders obscuring control;
- rapid formation and dissolution of entities;
- funds passing through shells with no economic rationale;
- use of jurisdictions with opaque ownership registries.

Trace beneficial ownership and test whether the entity has genuine economic substance.

### Recognize Professional Enablers

Lawyers, accountants, trust and company service providers, real estate agents, and other professionals can knowingly or negligently enable laundering by creating structures, conducting transactions, and providing legitimacy.

Enabler indicators:

- use of professional intermediaries to obscure the principal;
- trust and company structures with no clear business purpose;
- transactions conducted in the name of a professional on behalf of an undisclosed client;
- purchase of real estate through opaque structures;
- reliance on professional privilege to avoid disclosure;
- repeated use of the same enabler across unrelated high-risk clients.

Professional enablers are covered by FATF Recommendation 22 and should be within CDD scope where the institution deals with them.

### Detect Trade-Based Money Laundering

Trade-based money laundering (TBML) moves value through over- or under-invoiced trade, phantom shipments, and third-party payments. It is one of the largest channels for moving illicit funds internationally.

TBML indicators:

- invoice values inconsistent with the goods;
- rapid in-and-out movement of goods and payments;
- third-party payments unrelated to the buyer or seller;
- shipments to or from high-risk jurisdictions;
- goods that do not match the business profile;
- unusually complex trade structures with no economic rationale;
- dual-use or controlled goods with proliferation nexus.

TBML requires coordination between trade finance, transaction monitoring, and sanctions screening.

### Detect Virtual Asset Typologies

Virtual assets introduce new laundering methods, including conversion between fiat and crypto, mixing and tumbling, use of unhosted wallets, and peer-to-peer trading. FATF guidance and national VASP regimes set expectations for detecting these.

Virtual asset indicators:

- rapid conversion between fiat and virtual assets;
- use of mixers, tumblers, or privacy coins;
- transfers to or from unhosted wallets with no counterparty identification;
- transactions with sanctioned virtual asset addresses;
- funneling through multiple exchanges to break the chain;
- mismatch between the customer profile and crypto activity.

Virtual asset typologies evolve quickly and require ongoing intelligence refresh.

### Detect Mule Networks And Funnel Accounts

Money mules move illicit funds on behalf of criminals, often wittingly, sometimes unwittingly. Funnel accounts aggregate deposits from many sources and rapidly disburse them.

Indicators:

- multiple deposits from unrelated parties into one account;
- rapid withdrawal or wire of aggregated funds;
- account holder profile inconsistent with the deposit pattern;
- students, elderly, or financially vulnerable individuals used as mules;
- recruitment through work-from-home or romance scams;
- geographic dispersion of depositors and beneficiaries.

Mule networks are common in fraud proceeds laundering and require both detection and customer awareness controls.

## Common Traps

### Generic Red Flags Without Typology Anchoring

"Be alert for unusual activity" is not actionable. Tie each red flag to a specific typology and detection method.

### Treating The Three Stages As Rigid

Real laundering blends stages and can skip placement entirely when proceeds are already in the system. Use the framework as a map, not a script.

### Ignoring Professional Enablers

Enablers create the structures that make laundering possible. Excluding them from the red flag library leaves a major gap.

### Missing TBML Because It Looks Like Legitimate Trade

Trade transactions with invoices and goods can appear legitimate. Test value, counterparties, and rationale rather than accepting documentation at face value.

### Treating Virtual Assets As Out Of Scope

Even non-crypto institutions may have customers who use virtual assets. Ignoring crypto typologies creates a blind spot.

### Focusing On The Account Holder, Not The Network

Laundering runs through networks of accounts, entities, and individuals. Analyzing a single account in isolation misses the pattern.

### Stale Typology Libraries

New methods emerge constantly. A library that is never updated from FATF, FinCEN, and FIU sources becomes obsolete.

## Self-Check

- Are placement, layering, and integration indicators mapped to specific monitoring scenarios?
- Is the red flag library typology-anchored, with method, indicators, data, context, and detection steps for each?
- Are shell and front company indicators covered, including economic substance and nominee abuse?
- Are professional enablers included in the typology library and CDD scope?
- Are trade-based money laundering indicators covered, with coordination across trade finance, monitoring, and sanctions?
- Are virtual asset typologies included, covering conversion, mixing, unhosted wallets, and sanctioned addresses?
- Are mule networks and funnel account indicators covered?
- Is the typology library refreshed from FATF, FinCEN, Egmont, and FIU sources?
- Do investigators analyze networks of accounts and entities, not just isolated accounts?
- Are new typologies translated into monitoring scenarios and training?
