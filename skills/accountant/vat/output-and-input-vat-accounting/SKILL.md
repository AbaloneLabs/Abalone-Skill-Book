---
name: output_and_input_vat_accounting.md
description: Use when the agent is recording output VAT on sales, claiming input VAT on purchases, determining VAT recoverability, separating standard-rated supplies, or setting up value added tax ledger accounts and postings for a taxable business.
---

# Output And Input VAT Accounting

Value added tax is a transaction tax charged at each stage of supply, with businesses acting largely as collectors for the tax authority. The mechanics seem simple. Charge output tax on sales, claim input tax on purchases, and remit the net. But the simplicity is deceptive. Whether tax applies, at what rate, whether input tax is recoverable, when the tax point arises, and how to handle cross-border supplies all depend on rules that vary by jurisdiction and that change frequently. Books that balance can still be wrong in ways that create liability, penalties, and unrecoverable costs.

Use this skill before recording VAT on transactions, designing VAT ledger accounts, determining recoverability of input tax, or reviewing whether VAT postings are defensible. The goal is to prevent the agent from producing VAT entries that net out cleanly but misapply rates, misclassify supplies, or claim input tax that is not recoverable. VAT and GST rules differ enormously by jurisdiction, registration thresholds, entity type, and the nature of supplies. This skill provides structural judgment, not final tax determinations. A qualified tax or VAT professional must confirm any actual treatment.

## Core Rules

### Confirm The Supply Is Within Scope Before Charging Or Recovering

Before recording any VAT, confirm the supply is within the scope of the relevant VAT or GST regime. Scope depends on the place of supply rules, which determine whether a transaction is taxed domestically, zero-rated, treated as an export, treated as a reverse charge, or outside scope entirely. Do not assume a domestic sale is automatically standard-rated.

Check:

- is the customer a taxable person, a consumer, or a business in another jurisdiction?
- is the supply of goods, services, digital services, or immovable property?
- where does the place of supply rules locate the transaction?
- does a reverse charge or domestic reverse charge apply?
- is there a registration or threshold consideration that changes treatment?

### Record Output VAT At The Correct Rate And Tax Point

Output VAT is charged on taxable supplies at the applicable rate. The rate depends on the classification of the supply and the jurisdiction. The tax point, the date the tax becomes due, may differ from the invoice date or payment date and may trigger earlier or later reporting.

Capture for each taxable sale:

- the supply classification and applicable rate;
- the tax point date;
- the net, tax, and gross amounts;
- the customer status and location;
- any reverse charge indicator;
- evidence supporting the rate applied, such as the nature of goods or services.

Posting output tax without confirming the rate and tax point invites under-collection, over-collection, and period misallocation.

### Apply Input VAT Recoverability Rules Before Claiming

Input VAT is not automatically recoverable. Recoverability depends on whether the purchase relates to taxable supplies and whether any restriction, partial exemption, or block applies. Claiming input tax that is not recoverable creates an overstatement that the authority can disallow with interest and penalties.

Common restrictions include:

- input tax on costs supporting exempt supplies;
- input tax on entertainment, certain motor cars, or other blocked items;
- input tax where no valid tax invoice or equivalent evidence exists;
- input tax on purchases used for non-business purposes;
- input tax affected by partial exemption methods.

### Handle Partial Exemption Deliberately

A business making both taxable and exempt supplies is partially exempt and can usually recover only a proportion of its residual input tax. The method for apportioning residual input tax varies by jurisdiction and may require an agreed or standard method. Do not apply full recovery to mixed-use inputs without an apportionment. Track exempt and taxable input tax separately and perform the apportionment each period.

### Maintain Valid Supporting Evidence For Every Claim

Input tax claims require valid evidence, typically a compliant tax invoice or equivalent document showing the supplier details, the customer details, the amount, the rate, and the tax amount. Bank statements alone are usually insufficient. Missing or non-compliant evidence means the input tax is not recoverable regardless of whether the purchase was genuine.

### Separate VAT Ledger Accounts For Control And Audit

Maintain separate control accounts for output tax, input tax, and VAT payable or receivable. Mixing VAT into sales and purchase accounts obscures the liability and makes reconciliation impossible. Use clear subledgers or tracking categories to separate standard-rated, reduced-rated, zero-rated, exempt, and reverse charge activity.

### Apply Reverse Charge Mechanics Correctly

Cross-border services and certain domestic supplies may be subject to reverse charge, where the customer accounts for the tax rather than the supplier. The reverse charge is both an output tax and an input tax entry, often netting to zero for fully taxable businesses but still requiring both postings. Forgetting one side of the reverse charge, or applying it where it does not belong, distorts the return and the recoverability position.

### Reconcile VAT To Sales And Purchase Ledgers

VAT control accounts should reconcile to the underlying sales and purchase ledgers and ultimately to the filed return. Unreconciled differences accumulate and become very difficult to resolve. Reconcile each period and investigate variances immediately rather than carrying them forward.

## Common Traps

### Treating VAT As A Simple Net Of Output Minus Input

The net remittance is the end result, but treating it as the whole task hides rate errors, misclassified supplies, and unrecoverable input tax. Each side must be validated independently before netting.

### Claiming Input Tax Without A Valid Invoice

A genuine business expense is not enough. Without a compliant tax invoice or equivalent evidence, input tax is generally not recoverable. Relying on bank entries or supplier statements alone is a common cause of disallowed claims.

### Ignoring Partial Exemption

Businesses with exempt supplies cannot recover all input tax. Applying full recovery to mixed-use costs overstates the claim and creates exposure. Partial exemption must be computed using an appropriate method each period.

### Applying The Wrong Place Of Supply Treatment

A service to a foreign business may be outside scope or reverse charge rather than standard-rated. Charging domestic output tax on a reverse charge supply, or failing to reverse charge when required, are both common and costly errors.

### Using The Wrong Tax Point

The tax point may be the invoice date, the payment date, or the date of supply, whichever applies first under the rules. Posting tax to the wrong period causes return errors and cash flow distortion.

### Failing To Post Both Sides Of A Reverse Charge

The reverse charge requires both an output tax and an input tax entry. Posting only one side leaves the return unbalanced and misstates the recoverability position even when the net effect is zero.

### Letting VAT Control Accounts Drift

Without periodic reconciliation, small posting errors accumulate into large unreconciled balances that are hard to unwind. Carry-forward variances are a red flag for auditors and authorities.

### Assuming Rates Are Stable

VAT rates change, and registration thresholds and scope rules change. Applying last year's rate to this year's supply is a silent error. Confirm current rates and rules for the relevant jurisdiction each period.

## Self-Check

- Has the supply been confirmed as within scope, with the place of supply rules and any reverse charge correctly applied before posting?
- Is output VAT recorded at the correct rate and tax point, with evidence supporting the classification of each supply?
- Is input VAT only claimed where recoverable, with valid tax invoices or equivalent evidence for every claim?
- Have partial exemption apportionments been applied to mixed-use inputs rather than claiming full recovery?
- Are output tax, input tax, and VAT payable or receivable maintained in separate control accounts that reconcile to the ledgers?
- Are reverse charge transactions posted with both the output and input sides accounted for?
- Has the VAT position been reconciled to the sales and purchase ledgers and to the filed return, with variances investigated and resolved?
- Are current rates, thresholds, and scope rules for the relevant jurisdiction confirmed rather than assumed from a prior period?
- Has a qualified VAT or tax professional reviewed the treatment for the specific jurisdiction and entity before relying on it?
