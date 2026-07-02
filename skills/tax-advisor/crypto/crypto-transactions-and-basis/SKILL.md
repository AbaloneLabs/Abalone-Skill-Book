---
name: crypto_transactions_and_basis.md
description: Use when the agent is computing gain or loss on cryptocurrency transactions, tracking basis across wallets and exchanges, handling crypto-to-crypto trades and transfers, or reconciling exchange records with lot-level basis for accurate reporting.
---

# Cryptocurrency Transactions And Basis

Computing gain or loss on cryptocurrency transactions requires tracking every acquisition and disposal at the lot level across potentially dozens of wallets, exchanges, and blockchains. Unlike traditional securities, where a single broker typically holds all positions and reports basis, crypto users self-custody and transact across fragmented platforms with inconsistent or absent reporting. The basis tracking problem is the central operational challenge of crypto taxation, and it is where most errors occur. A taxpayer who cannot establish lot-level basis for their crypto cannot accurately compute gain or loss and may default to a zero-basis assumption that massively overstates income.

Agents commonly miss that transfers between the taxpayer's own wallets are not taxable events (but must be tracked for lot continuity), that crypto-to-crypto trades require valuing both sides in fiat at the time of the trade, that exchange records may be incomplete or inaccurate, and that basis from mining or staking rewards is the fair market value at receipt. The harm is a taxpayer whose reported gain or loss is materially wrong, either overstating gain (due to missing basis records) or understating gain (due to missed transactions).

This skill covers cryptocurrency transaction analysis and basis tracking under U.S. federal tax law. It is not tax advice; crypto basis tracking is complex and fact-specific, and a qualified tax professional must be consulted for actual situations.

## Core Rules

### Track Every Acquisition And Disposal

Every acquisition of crypto creates a lot with a basis and a holding period, and every disposal triggers gain or loss recognition. The full transaction history must be reconstructed to compute accurate gain or loss. Acquisitions include purchases on exchanges, mining rewards, staking rewards, airdrops, compensation in crypto, and receipts from other parties. Disposals include sales for fiat, crypto-to-crypto trades, purchases of goods or services, gifts to others, and payments.

Obtain the complete transaction history from all sources: exchange export files, wallet transaction logs, blockchain explorers, and payment processor records. Aggregate the transactions chronologically and identify each as an acquisition, disposal, or transfer. Missing transactions lead to incorrect basis and gain or loss computations. For active users, the volume can be thousands of transactions per year, making manual reconstruction impractical without software. Use crypto tax software to aggregate, but verify the output against source records.

### Distinguish Transfers From Taxable Events

Transfers of crypto between the taxpayer's own wallets or accounts are not taxable events. Moving Bitcoin from an exchange to a hardware wallet, or from one wallet to another, does not trigger gain or loss. However, the transfer must be tracked to maintain lot continuity: the specific lots moved must be identified so that the basis and holding period carry over to the receiving wallet. The transfer may incur network fees (gas fees), which have their own treatment (generally part of the transfer, though the disposition of the fee portion can be analyzed separately).

Identify which transactions are transfers (non-taxable) versus disposals (taxable). A common error is treating a transfer as a taxable event (overstating gain) or treating a disposal as a transfer (understating gain). Confirm that the sending and receiving addresses both belong to the taxpayer for transfers. For transactions involving third parties, treat as acquisitions or disposals. Document the wallet ownership to support the transfer classification.

### Value Crypto-To-Crypto Trades In Fiat

When crypto is traded for another crypto, the transaction is taxable, and the gain or loss is computed based on the fair market value of the crypto received (or the crypto disposed of, whichever is more clearly valued) in U.S. dollars at the time of the trade. Both sides of the trade must be valued in fiat to establish the amount realized and the new basis. The fair market value is typically the spot price on a major exchange at the time of the trade.

Obtain the fiat value of both the crypto disposed of and the crypto received at the time of each trade. Exchange records may provide this, but for decentralized exchange trades or peer-to-peer transactions, the taxpayer may need to look up historical prices from a reliable source. The fiat value of the crypto received becomes the basis of the new lot. The gain or loss is the fiat value received minus the basis of the crypto disposed of. Do not record the trade at the crypto-to-crypto ratio without fiat valuation; the gain or loss cannot be computed without a fiat denominator.

### Establish Basis For Mining, Staking, And Reward Income

Crypto received as mining rewards, staking rewards, or other network compensation is ordinary income at the fair market value on the date the taxpayer receives it (or has dominion and control over it). That fair market value becomes the basis of the crypto for computing gain or loss on later disposal. The holding period begins on the receipt date. Mining income may be subject to self-employment tax if the mining is a trade or business.

Determine the fair market value of each reward at the receipt date and report it as ordinary income. The basis for the lot is the same fair market value. When the reward crypto is later sold or traded, the gain or loss is the disposal value minus the receipt-date fair market value. Track rewards separately from purchased crypto, because the income recognition at receipt is a separate event from the capital gain or loss on disposal. For staking rewards that accrue continuously, establish a consistent valuation method (daily snapshot, for example) and apply it throughout.

### Apply Specific Identification Or FIFO To Disposals

When disposing of crypto, the taxpayer must identify which specific lots are disposed of. Specific identification is permitted if the taxpayer can identify the specific units (by lot, transaction ID, or wallet location) disposed of, similar to securities. If specific identification is not adequately made, the default is first-in-first-out (FIFO). The choice of lots affects the gain or loss and the holding period character.

For specific identification, the taxpayer must have records sufficient to identify which lots were disposed of at the time of the transaction. Transferring specific lots to an exchange for sale, or selecting lots within an exchange interface, can support specific identification. If the records do not support specific identification, apply FIFO. The method should be applied consistently and documented. Do not retroactively assign lots at tax time unless the contemporaneous records support it. The lot selection strategy (highest basis to minimize gain, or lowest basis to maximize loss harvesting) should be planned before disposals.

### Reconcile Exchange Records With Wallet And Blockchain Data

Exchange records (export files, 1099s) may be incomplete or inaccurate. Exchanges may not capture all transactions, may misclassify transfers, or may have incorrect historical prices. The taxpayer's own wallet and blockchain records are the source of truth and should be reconciled against the exchange data. Discrepancies must be investigated and resolved.

Obtain the exchange export files and the wallet transaction logs, and reconcile them. Identify transactions that appear in one source but not the other, and classify them correctly. For example, a withdrawal from an exchange should appear as a disposal in the exchange records and a transfer (non-taxable) in the wallet records; if it appears as a taxable event in one but not the other, the classification is inconsistent. Resolve all discrepancies before finalizing the gain or loss computation. Maintain the reconciliation as part of the tax file.

### Handle Lost, Stolen, And Forked Assets

Lost or stolen crypto may qualify for a casualty or theft loss deduction, though the rules are strict and the current law limits personal casualty loss deductions. Crypto lost due to a collapsed exchange or a hack may qualify for a theft loss, subject to the limitations and substantiation requirements. Hard forks that result in the taxpayer receiving new tokens create income at the fair market value when dominion and control is established, with that value as basis.

Confirm the treatment of any lost, stolen, or forked assets. For losses, establish the amount, the event, and the deductibility under current law. For forks, establish the receipt date, the fair market value, and the income recognition. These events are often overlooked because they are outside normal trading activity, but they have tax consequences. Document the facts and apply the current rules.

## Common Traps

### Treating Wallet-To-Wallet Transfers As Taxable

Transfers between the taxpayer's own wallets are not taxable. Treating them as disposals overstates gain. Confirm both addresses belong to the taxpayer.

### Failing To Value Crypto-To-Crypto Trades In Fiat

Each side of a crypto-to-crypto trade must be valued in U.S. dollars. Recording only the crypto amounts prevents gain or loss computation.

### Defaulting To Zero Basis When Records Are Missing

Missing basis records do not mean zero basis. Reconstruct basis from historical acquisition records. A zero-basis assumption massively overstates income.

### Overlooking Mining And Staking Reward Income

Rewards are ordinary income at fair market value at receipt. The same value is the basis for later disposal. Missing the income understates tax.

### Applying FIFO Without Considering Specific Identification

FIFO may not be optimal. Specific identification can minimize gain or optimize loss harvesting. The choice should be made with records supporting it, not defaulted.

### Trusting Exchange Records Without Reconciliation

Exchange exports may miss transactions, misclassify transfers, or have wrong prices. Reconcile against wallet and blockchain data before finalizing.

### Ignoring Lost, Stolen, Or Forked Assets

Lost or stolen crypto may have loss treatment (subject to limits), and forked tokens create income. These events are outside normal trading and easy to miss.

## Self-Check

- [ ] Has the complete transaction history been aggregated from all exchanges, wallets, and blockchain sources, with each transaction classified as acquisition, disposal, or transfer?
- [ ] Have transfers between the taxpayer's own wallets been identified as non-taxable, with lot continuity maintained across the transfer?
- [ ] Have all crypto-to-crypto trades been valued in U.S. dollars at the time of the trade, establishing both the gain or loss and the new lot basis?
- [ ] Has mining, staking, and reward income been reported as ordinary income at fair market value at receipt, with that value as basis?
- [ ] Has specific identification or FIFO been applied to disposals, with the method documented and the lot selection strategy considered?
- [ ] Have exchange records been reconciled with wallet and blockchain data, with discrepancies investigated and resolved?
- [ ] Have lost, stolen, and forked assets been analyzed for loss deduction or income recognition under current rules?
- [ ] Is the basis and gain or loss analysis documented with lot-level detail, source records, and reconciliation, supporting the reported amounts?
- [ ] Has the agent noted that this is general U.S. federal crypto transaction information, not tax advice, and recommended consulting a qualified tax professional for the specific situation?
