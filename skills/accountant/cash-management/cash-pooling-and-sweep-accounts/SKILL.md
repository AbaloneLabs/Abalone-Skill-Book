---
name: cash_pooling_and_sweep_accounts.md
description: Use when the agent is setting up or reviewing cash pooling structures, physical and notional pools, sweep accounts and zero-balance accounts, intercompany lending between pool participants, or evaluating whether concentration, interest allocation, and bank fees are working as intended and within legal and tax limits.
---

# Cash Pooling And Sweep Accounts

Cash pooling and sweep structures exist to concentrate an entity's cash so that surplus balances in one account offset deficits in another, reducing external borrowing and improving interest efficiency. Done well, they are a powerful treasury tool. Done casually, they create a tangle of hidden intercompany loans, tax and transfer-pricing exposure, misallocated interest, broken segregation between entities, and balances that no one can reconcile. The structures look simple on a diagram, one master account and several participants, but the accounting, legal, and tax reality is that every movement between participants is a transaction that must be documented, priced, and settled. An agent that treats a pool as an automatic cash shuffle rather than a set of inter-entity obligations will produce records that cannot survive audit, tax review, or a dispute between participating entities.

Use this skill before establishing a cash pool, configuring sweep or zero-balance accounts, allocating pool interest, reconciling intercompany pool positions, or reviewing whether an existing structure is operating correctly. The goal is to prevent the agent from deploying a concentration structure that moves cash efficiently but breaks intercompany, tax, and control integrity.

Banking regulations, intercompany lending rules, thin-capitalization and transfer-pricing rules, interest-allocation requirements, and notional-pooling permissions vary sharply by jurisdiction and currency, and pooling across borders adds restrictions on currency convertibility, withholding tax, and central-bank reporting. Do not state a single universal rule. Pool design, intercompany loan documentation, and tax treatment should be confirmed with a qualified treasury, tax, and legal professional and the entity's bank. This skill provides operational concentration judgment, not a final determination for any specific structure.

## Core Rules

### Understand The Difference Between Physical And Notional Pooling

The two main pooling methods behave differently and carry different risks, and confusing them produces accounting and legal errors.

In a physical, or cash, pool, surplus balances are physically transferred to a header or master account, and deficit accounts are funded from it. Every movement is an actual cash transfer and an intercompany loan that must be recorded and settled.

In a notional pool, balances are not physically moved. The bank offsets them notionally for interest calculation, so a credit balance in one participant offsets a debit balance in another for interest purposes, while each account retains its own legal balance.

Decide deliberately:

- physical pooling gives stronger concentration but requires intercompany loan accounting and settlement;
- notional pooling preserves legal balances but may be restricted or unavailable in some jurisdictions or currencies;
- cross-border notional pooling is often limited by regulation and tax rules;
- some structures combine physical and notional elements, which must be mapped carefully.

Never describe or account for a structure without confirming which method is actually in place.

### Treat Every Pool Movement As An Intercompany Transaction

In a physical pool, cash moving from a participant to the header is a loan from the participant to the header, and cash moving back is a repayment or an advance to the participant. These are not wash entries. They create intercompany receivables and payables that must be tracked, agreed, and settled.

For each participant track:

- the running intercompany position, whether net lender or net borrower;
- the principal amount advanced or received;
- the interest owed or earned on the position;
- the settlement dates and clearing of the balance;
- the documentation supporting the intercompany arrangement.

Intercompany pool positions that are never settled or documented become permanent unexplained balances that fail audit and create tax exposure.

### Allocate Interest Based On Each Participant's Real Contribution

Pool interest is earned or paid because participants contribute surplus or draw deficit. That interest must be allocated back to participants according to their actual net position, not assigned arbitrarily to the header.

Establish an allocation method that reflects:

- each participant's average daily net contribution or draw;
- whether the pool earns interest on net credit positions and pays on net debit positions;
- the bank's interest rate and fee structure applied to the pool;
- any spread the header retains versus what is passed to participants;
- the treatment of participants in deficit who are effectively borrowing.

Interest allocated without a defensible method, or captured entirely by the header, distorts each entity's results and can create transfer-pricing and tax problems, especially when participants are in different tax jurisdictions.

### Map The Legal And Entity Boundaries Before Concentrating

Pooling moves cash across entity lines. That is only lawful and clean if the entities are permitted to lend to each other, if the arrangement is documented, and if the structure respects each entity's separate legal status.

Before pooling, confirm:

- which entities participate and whether they are in the same group and jurisdiction;
- whether each entity's governing documents and any shareholder or lender restrictions permit intercompany lending;
- whether third-party lender covenants restrict cash concentration or require consent;
- whether any participant is subject to restrictions, insolvency risk, or minority interests that complicate pooling;
- whether pooling across borders triggers regulatory or tax consent.

Pooling that ignores entity boundaries can pierce corporate separateness, breach covenants, and create unenforceable intercompany claims. When in doubt, do not pool until legal review confirms it is permitted.

### Configure Sweeps And Zero-Balance Accounts Intentionally

Sweep and zero-balance accounts move cash automatically, which is efficient but also invisible. If the sweep rules are wrong, cash moves in directions no one intended, and the accounting follows automatically without review.

Define and verify:

- the target balance each account sweeps to or maintains, such as zero or a set floor;
- the direction of the sweep and the header account receiving or funding it;
- the timing of sweeps, end of day or intra-day, and the cut-off used;
- the treatment of overdrafts and whether deficit accounts are funded or charged;
- the bank's fees for the sweep service and how they are allocated.

Review the actual sweep behavior against the configured rules regularly. A misconfigured sweep can quietly create unintended overdrafts, fees, or intercompany positions.

### Reconcile The Pool To The Bank And The Books Continuously

Because pooling moves cash constantly, the reconciliation must keep pace. A pool that is not reconciled each cycle will accumulate unmatched intercompany positions, unrecorded interest, and fees that distort every participant's books.

Reconcile regularly:

- each participant account to its bank statement;
- the header account to the bank and to the sum of participant positions;
- intercompany pool positions between participants and the header;
- interest allocated to each participant against the bank's interest calculation;
- bank fees and charges to the correct participant or the header.

Unreconciled pool positions are the most common source of audit findings in treasury, because the automation hides the discrepancies until year-end.

### Respect Tax And Transfer-Pricing Rules On Intercompany Interest

When participants in different tax positions or jurisdictions lend to and borrow from each other through a pool, the interest between them is not optional. It must be priced at arm's length, documented, and reported.

Consider:

- whether intercompany interest must be charged at a market rate under transfer-pricing rules;
- whether withholding tax applies to cross-border intercompany interest;
- whether thin-capitalization or interest-limitation rules restrict the deductibility of intercompany borrowing;
- whether each participant's local tax authority requires documentation of the arrangement.

A pool that funnels interest to a low-tax header without arm's-length pricing is a transfer-pricing exposure, not an efficiency.

## Common Traps

### Treating Pool Movements As Automatic Wash Entries

Recording pool transfers as if they net to nothing ignores that each movement is an intercompany loan. The result is undocumented, unsettled intercompany balances that fail audit and create tax risk.

### Assigning All Interest To The Header

Capturing pool interest in the header while participants contribute the balances distorts each entity's results and can breach transfer-pricing rules. Interest must follow each participant's real net position.

### Pooling Across Entities Without Confirming It Is Permitted

Concentrating cash across separate legal entities without checking governing documents, lender covenants, and legal restrictions can breach contracts and pierce corporate separateness. Entity boundaries must be confirmed before pooling.

### Confusing Physical And Notional Pooling

Accounting for a notional pool as if cash moved, or a physical pool as if balances stayed put, produces records that do not match reality. Always confirm which structure is in place.

### Ignoring Cross-Border And Currency Restrictions

Notional pooling, currency convertibility, withholding tax, and central-bank reporting rules vary by country. A cross-border pool designed without checking these rules can be unenforceable or illegal.

### Letting Sweeps Run Without Review

Automated sweeps are efficient but invisible. A misconfigured rule can create unintended overdrafts, fees, or intercompany positions that no one notices until reconciliation, if it happens at all.

### Failing To Reconcile Intercompany Pool Positions

Because pooling moves cash constantly, unmatched positions accumulate fast. A pool that is not reconciled each cycle will carry errors that surface only at year-end when they are hard to fix.

### Overlooking Bank Fees And Their Allocation

Pool and sweep services carry fees that must be allocated to the right participant or the header. Letting the bank absorb or the header silently bear all fees distorts participant results.

## Self-Check

- Has the structure been confirmed as physical pooling, notional pooling, or a hybrid, and is the accounting consistent with that actual method?
- Is every physical pool movement recorded as an intercompany loan with principal, interest, settlement, and documentation, rather than as a wash entry?
- Is pool interest allocated to each participant based on its real average daily net contribution or draw, with any header spread explicitly justified?
- Have entity boundaries, governing documents, lender covenants, and legal restrictions been confirmed before concentrating cash across entities?
- Are sweep and zero-balance rules defined for direction, target balance, timing, overdraft treatment, and fee allocation, and is actual behavior reviewed against the configuration?
- Is the pool reconciled each cycle across participant accounts, the header, intercompany positions, interest, and bank fees?
- Have transfer-pricing, withholding tax, thin-capitalization, and interest-limitation rules been considered for intercompany interest, especially across borders?
- Are cross-border currency, convertibility, and regulatory reporting restrictions confirmed before pooling internationally?
- Have jurisdiction-specific banking, tax, and legal rules been confirmed with a qualified professional before the structure is relied upon?
