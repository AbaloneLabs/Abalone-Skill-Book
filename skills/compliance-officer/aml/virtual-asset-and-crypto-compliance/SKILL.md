---
name: virtual_asset_and_crypto_compliance.md
description: Use when the agent is managing VASP and virtual asset compliance, applying the Travel Rule, scoring wallet risk, detecting mixing and tumbling, using chain analytics, handling unhosted wallets, or assessing sanctions nexus in crypto transactions.
---

# Virtual Asset And Crypto Compliance

Virtual assets, including cryptocurrencies and tokens, have become a major channel for money laundering, sanctions evasion, ransomware laundering, and fraud proceeds. FATF has extended AML obligations to virtual asset service providers (VASPs) through the Travel Rule and the revised Recommendation 15, and national regulators (FinCEN in the US, and authorities under EU MiCA and AMLD) impose registration, CDD, transaction monitoring, and reporting duties. Virtual assets move value across borders instantly, often pseudonymously, and through services such as mixers, tumblers, and unhosted wallets that are designed to obscure the trail. A compliance program that treats crypto like traditional banking will miss the distinct typologies; one that ignores crypto entirely is already behind.

Use this skill before designing VASP compliance, applying the Travel Rule, building wallet risk scoring, configuring chain analytics, or handling unhosted wallet and sanctions exposure. The goal is to make the agent think about the pseudonymous nature of crypto, the data that must accompany transfers, the services that obscure trails, and the sanctions nexus that can make a single transaction a violation. Virtual asset compliance is where AML, sanctions, and technology converge, and it evolves rapidly.

## Core Rules

### Apply VASP Registration And AML Obligations

VASPs, including exchanges, custodians, brokers, and some decentralized platform operators, are obligated entities under FATF and most national regimes. They must register or license, conduct CDD, monitor, keep records, and report suspicious activity.

VASP obligations:

- registration or licensing with the competent authority;
- CDD on customers, including for occasional transactions above thresholds;
- ongoing monitoring of virtual asset transactions;
- recordkeeping for the required retention period;
- SAR/STR filing on suspicious activity;
- sanctions screening of customers and counterparties;
- compliance program with a designated officer and training.

Institutions that bank VASPs must apply correspondent-style due diligence to the VASP relationship, including confirming the VASP's regulatory status and controls.

### Implement The Travel Rule

The FATF Travel Rule (Recommendation 16, extended to VASPs) requires originator and beneficiary information to accompany virtual asset transfers between obligated entities, mirroring the wire transfer rules for traditional payments.

Travel Rule requirements:

- the originating VASP must obtain and transmit originator and required beneficiary information;
- the beneficiary VASP must obtain and screen the information;
- for transfers above a threshold, full originator and beneficiary data must travel with the transfer;
- for smaller transfers, reduced information may suffice, but must be available on request;
- intermediary institutions must pass the information along;
- VASPs must take risk-based steps where information is missing, including restricting or suspending the relationship.

The Travel Rule is operationally complex because virtual asset transfers are settled on-chain instantly while compliance data must travel through separate messaging. Use compliant Travel Rule messaging solutions and have fallback procedures for non-compliant counterparties.

### Score Wallet And Counterparty Risk

Wallet risk scoring assesses the exposure associated with a counterparty address based on its on-chain history. Chain analytics tools attribute addresses to service categories and flag exposure to illicit sources.

Wallet risk factors:

- exposure to known illicit services (darknet markets, ransomware, scams, stolen funds);
- exposure to mixers, tumblers, and privacy services;
- exposure to sanctioned addresses or jurisdictions;
- association with unhosted wallets versus hosted (custodial) wallets;
- transaction history, counterparties, and clustering;
- new or low-activity addresses with no reputation.

Set risk thresholds and define the action for each band, from proceed with monitoring to restrict or freeze. Document the methodology and the data sources.

### Detect Mixing, Tumbling, And Obfuscation

Mixers and tumblers pool and redistribute funds to break the link between sender and receiver. They are a primary laundering tool in virtual assets and are increasingly the target of sanctions and enforcement.

Detection:

- flag transactions to or from known mixer or tumbler services;
- detect patterns consistent with peeling chains and coin joins;
- watch for rapid movement through multiple addresses or services to break the trail;
- monitor use of privacy coins or privacy-enhancing features;
- assess the customer rationale for using such services.

Some mixers have been designated as sanctioned entities, making any transaction with them a prohibition. Screen against current OFAC and other designations of virtual asset addresses.

### Handle Unhosted Wallets

Unhosted (non-custodial, self-hosted) wallets are controlled directly by the user, not by a VASP. They create a CDD gap because the counterparty is not an obligated entity.

Unhosted wallet controls:

- apply risk-based measures to transfers involving unhosted wallets;
- obtain and verify the owner of the unhosted wallet where required;
- use chain analytics to assess the wallet's exposure;
- apply transaction limits or enhanced review for high-risk unhosted wallets;
- document the rationale for permitting or restricting unhosted wallet transfers.

Regulatory expectations for unhosted wallets vary by jurisdiction and are tightening. Track the applicable rule and calibrate controls accordingly.

### Manage The Sanctions Nexus

Virtual assets are a significant sanctions evasion channel. OFAC has designated virtual asset addresses, and transfers to or from sanctioned addresses are prohibited regardless of the asset type.

Sanctions controls:

- screen counterparty addresses against OFAC and other sanctions lists, including designated addresses;
- block or reject transactions involving sanctioned addresses and report as required;
- monitor for exposure to sanctioned jurisdictions through virtual assets;
- assess exposure to ransomware actors, who are frequently sanctioned;
- coordinate sanctions screening with wallet risk scoring.

A transaction can be a sanctions violation even when the underlying customer is not sanctioned, if the counterparty address is. Address-level screening is essential.

### Monitor Typologies And Update Controls

Virtual asset typologies evolve quickly: new mixers, bridges, privacy tools, and scams appear regularly. Maintain intelligence from FATF, FinCEN advisories, OFAC actions, and chain analytics providers, and translate new typologies into wallet scoring rules and monitoring scenarios. Static controls age fast in this domain.

## Common Traps

### Treating Crypto Like Traditional Banking

Virtual assets move instantly, pseudonymously, and across services that traditional banking controls do not address. Apply crypto-specific typologies and tools.

### Ignoring The Travel Rule

The Travel Rule applies to VASPs. Transferring value without originator and beneficiary data is a violation, even if the on-chain settlement is valid.

### Missing Sanctioned Addresses

OFAC designates specific virtual asset addresses. Screening only names and not addresses misses prohibited transactions.

### Underestimating Unhosted Wallet Risk

Unhosted wallets have no obligated counterparty. Without risk-based measures, they become a laundering channel.

### Stale Wallet Scoring

Illicit services, mixers, and designations change constantly. Scoring rules that are never updated lose accuracy.

### Over-Relying On A Single Tool

Chain analytics attribution is probabilistic. Use multiple sources and apply judgment, not blind thresholds.

### Banking VASPs Without Correspondent Diligence

Institutions that provide bank accounts to VASPs must apply enhanced diligence to the VASP itself, not just its customers.

## Self-Check

- Is the VASP registered or licensed, with a compliance program covering CDD, monitoring, recordkeeping, and reporting?
- Does the Travel Rule implementation transmit and screen originator and beneficiary information, with fallback for non-compliant counterparties?
- Is wallet risk scoring based on exposure to illicit services, mixers, sanctioned addresses, and unhosted wallets?
- Are mixing, tumbling, and obfuscation patterns detected and escalated?
- Are unhosted wallet transfers subject to risk-based measures, owner verification, and limits?
- Is address-level sanctions screening in place against OFAC and other designations?
- Are sanctioned-address transactions blocked, rejected, and reported?
- Is ransomware exposure monitored, given frequent sanctions designations?
- Are wallet scoring rules and monitoring scenarios refreshed from FATF, FinCEN, OFAC, and chain analytics intelligence?
- Are institutions banking VASPs applying correspondent-style due diligence to the VASP relationship?
