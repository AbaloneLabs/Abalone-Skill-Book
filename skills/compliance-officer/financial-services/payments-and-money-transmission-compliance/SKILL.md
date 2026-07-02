---
name: payments_and_money_transmission_compliance.md
description: Use when the agent is assessing payments regulation, money transmission licensing, prepaid access, remittances, payment processing risk, stablecoin issuance, open banking, or funds-transfer compliance for a fintech, processor, bank, or money services business.
---

# Payments And Money Transmission Compliance

Moving money is a licensed, supervised activity in most jurisdictions, and the rules attach to whoever holds, transmits, controls, or settles funds, not only to banks. A technology company that holds customer balances, transmits value between users, issues prepaid access, processes card payments, settles remittances, or issues stablecoins is frequently a money services business and often a money transmitter requiring state licenses. The compliance trap is that the same product can be low-risk consumer software in one design and a regulated funds-transfer service in another, and the difference turns on who holds funds, who controls movement, and whether value can be redeemed.

Use this skill before advising on a payments product, reviewing money transmission licensing, evaluating prepaid or remittance obligations, assessing processor and network rules, or planning a stablecoin or wallet offering. The goal is to make the agent identify the funds flow, the regulated parties, the licensing and registration duties, and the consumer protection obligations before concluding the activity is unregulated.

## Core Rules

### Map The Funds Flow And Identify Regulated Parties

Payments compliance starts with the flow of funds. Who holds value, who moves it, who settles, and who can redeem determine who is regulated.

Map:

- whether the company holds customer funds or only transmits instructions;
- whether balances are stored, prepaid, or held in trust;
- who controls the ledger and movement of value;
- settlement and clearing rails used (card networks, ACH, wire, RTP, blockchain);
- float and interest on held funds;
- redemption and withdrawal rights;
- the role of a partner bank or sponsor;
- whether the company is the originator, receiver, processor, or gateway;
- cross-border legs of the transaction.

A company that holds balances it can use for its own purposes is almost always regulated differently than one that only routes instructions.

### Determine Money Transmission And MSB Registration

In the United States, money transmission is regulated at both federal and state levels, and the definitions differ.

Assess:

- federal MSB registration with FinCEN and AML program obligations;
- state money transmitter licensing in each state where customers reside or funds are transmitted;
- the definition of money transmission in each state, since some capture any value transmission;
- exemptions for payment processors, banks, and certain intermediaries, and their conditions;
- bonding and net worth requirements;
- permissible investments to back outstanding obligations;
- virtual currency activity treatment under state money transmission and currency transmission rules;
- examination authority of state regulators.

Operating without a required license can trigger cease-and-desist orders, restitution, and in some states criminal liability. Exemptions are narrow and fact-specific.

### Apply Consumer Protection Rules To Payment Products

Payments products carry layered consumer protection duties tied to the instrument and the timing.

Identify which apply:

- Regulation E for electronic fund transfers, prepaid access, and error resolution;
- Regulation Z for credit features attached to payments;
- Remittance Transfer Rule for international consumer remittances;
- Regulation CC for check availability;
- Regulation D for reserve and transfer limits on certain accounts;
- Unlawful Internet Gambling Enforcement Act blocking obligations;
- gift card and stored-value rules including dormancy and fee limits;
- state unclaimed property for unused balances.

Each rule has specific disclosure, timing, error-resolution, and recordkeeping duties. Prepaid access triggers both Reg E and, if credit is attached, Reg Z, with combined complexity.

### Control Fraud, AML, And Sanctions In Payment Flows

Payments are a primary channel for fraud, money laundering, and sanctions evasion. Controls must operate at the transaction level.

Implement:

- customer identification and verification at onboarding;
- transaction monitoring for structuring, layering, and unusual patterns;
- sanctions screening of senders, receivers, and beneficiaries against current lists;
- suspicious activity reporting obligations;
- fraud detection and chargeback handling;
- velocity and limit controls;
- watch on high-risk corridors and counterparties;
- beneficial ownership identification for business customers;
- recordkeeping of transfers above thresholds;
- blocking and rejection of prohibited transactions.

AML obligations for money transmitters include a written program, designated compliance officer, training, and independent review. Sanctions screening must be current because lists change.

### Evaluate Card Network And Processor Obligations

Card payments add network rules and processor responsibilities on top of law.

Assess:

- merchant category code and high-risk merchant handling;
- chargeback and dispute management;
- network registration and sponsor relationships;
- surcharge and convenience fee rules by network and state;
- tokenization and credential storage;
- interchange and network fee compliance;
- monitoring for friendly fraud and merchant collusion;
- compliance with card brand security standards for stored data.

Network rules can be stricter than law and can result in fines or loss of network access.

### Address Stablecoin And Digital Asset Payment Risk

Stablecoins and tokenized value used for payments trigger overlapping regimes.

Analyze:

- whether the stablecoin is a security, commodity, or money transmission instrument;
- reserve backing, segregation, and attestation;
- redemption rights and timing;
- custody of customer digital assets;
- AML and travel rule obligations for transfers;
- cross-border and sanctions screening on wallets;
- consumer disclosures on value stability and risk;
- operational resilience of the issuing and redemption mechanism.

Regulators increasingly expect stablecoin issuers to maintain high-quality reserves, publish attestations, and provide reliable redemption.

### Manage Cross-Border And Open Banking Obligations

Cross-border payments and open banking add data, licensing, and access rules.

Cover:

- licensing or partnership in destination jurisdictions;
- cross-border data transfer and privacy compliance;
- open banking consent, access scope, and revocation;
- screen scraping versus regulated API access;
- liability allocation between banks, third-party providers, and customers;
- currency conversion disclosure;
- intermediary bank and correspondent risk.

Open banking regimes impose security, consent, and dispute duties on third-party providers that access customer accounts.

## Common Traps

### Treating A Wallet As Unregulated Because It Is Software

A wallet that holds redeemable balances is frequently a money transmitter. The regulatory question is who holds and controls funds, not whether the product is branded as software.

### Assuming A Bank Sponsorship Eliminates Licensing

A sponsor bank does not always eliminate state money transmitter licensing for the non-bank partner. Analyze who holds funds and who is the transmitter in each state.

### Prepaid Access Without Reg E And Error Resolution

Stored value triggers Regulation E, including initial disclosures, periodic statements or alternatives, and error resolution timelines. A wallet that lacks these is noncompliant.

### Sanctions Screening That Is Not Current

Specially designated nationals lists update frequently. A one-time screen or stale list can miss designated parties and create strict liability.

### Remittance Disclosures And Error Resolution Ignored

International consumer remittances trigger upfront disclosure, cancellation rights, and error resolution duties. Treating remittance as a domestic transfer misses the rule.

### Stablecoin Reserves That Are Not Segregated Or Attested

Commingled reserves or opaque backing create consumer harm and regulatory risk. Reserves should be segregated, liquid, and independently attested.

### Open Banking Access Without Consent Management

Third-party access to accounts requires clear consent, scope limits, revocation, and security. Access without these controls violates open banking rules and privacy law.

## Self-Check

- Is the funds flow mapped to identify who holds, transmits, controls, settles, and redeems value, and is the regulated party identified for each leg?
- Are federal MSB registration and state money transmitter licensing analyzed for each state, including exemptions, bonding, net worth, and permissible investments?
- Are Reg E, Reg Z, remittance, Reg CC, UIGEA, gift card, and unclaimed property obligations mapped to each product?
- Are AML program, transaction monitoring, sanctions screening, SAR filing, fraud controls, and recordkeeping operational at the transaction level?
- Are card network rules, high-risk merchant handling, chargebacks, surcharge limits, and security standards addressed?
- For stablecoins, is the instrument classification, reserve segregation and attestation, redemption rights, custody, travel rule, and cross-border screening analyzed?
- Are cross-border licensing, data transfer, open banking consent, scope, revocation, and liability allocation covered?
- Is the distinction between holding funds and merely routing instructions clearly analyzed to determine money transmission duties?
- Are consumer disclosures, error resolution, and cancellation rights present for prepaid and remittance products?
- Are exemptions claimed only where conditions are documented and not merely assumed?