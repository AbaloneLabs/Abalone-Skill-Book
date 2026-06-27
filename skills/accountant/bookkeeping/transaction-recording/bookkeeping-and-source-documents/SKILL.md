---
name: bookkeeping_and_source_documents.md
description: Use when the agent is recording business transactions, designing bookkeeping routines, classifying income or expenses, organizing receipts and invoices, separating business from personal activity, or checking whether books are supported by adequate source documents.
---

# Bookkeeping And Source Documents

Bookkeeping is not simply putting numbers into categories. It is building an evidence trail that lets a business understand what happened, prepare useful reports, support tax filings, and explain entries later. Weak bookkeeping often looks acceptable in the moment because totals appear to balance, but it fails when someone asks where a number came from, whether an expense was business-related, why revenue changed, or whether a transaction belongs in a different period.

Use this skill before recording transactions, building a bookkeeping process, importing bank feeds, reconciling receipts, classifying expenses, preparing records for an accountant, or reviewing whether a bookkeeping dataset is reliable enough for reporting. The goal is to prevent the agent from producing books that are easy to enter but hard to verify.

## Core Rules

### Preserve The Source Document Trail

Every recorded transaction should be traceable to evidence. Source documents are not decoration; they are the reason the entry can be trusted.

Check for:

- invoice or receipt;
- sales slip or deposit record;
- bank or credit card statement;
- canceled check or proof of electronic payment;
- purchase order or contract;
- payroll record;
- expense report;
- shipping or delivery evidence;
- customer credit memo;
- vendor statement;
- asset purchase record.

The record should show who was paid or who paid the business, the amount, the date, the business purpose, the payment method, and what was purchased or sold. If a single document does not prove all required facts, link multiple documents rather than pretending one weak document is enough.

### Separate Business, Personal, Taxable, And Nontaxable Activity

Do not let classification hide the nature of the transaction. Business and personal activity should be separated. Taxable and nontaxable receipts should be distinguished. Owner contributions, owner draws, loans, reimbursements, refunds, and transfers should not be recorded as ordinary revenue or expense unless that treatment is justified.

Ask:

- Is this business activity or personal activity?
- Is this income, a liability, an equity contribution, a reimbursement, or a transfer?
- Is this expense, an asset purchase, loan repayment, inventory, prepaid cost, or owner distribution?
- Is the transaction taxable, nontaxable, deductible, nondeductible, or partly deductible under the relevant rules?
- Does the accounting treatment need review by a tax or accounting professional?

When unsure, use a review category rather than forcing a final classification. A temporary review category is better than silently misclassifying a transaction.

### Use A Chart Of Accounts That Supports Decisions

The chart of accounts should be detailed enough to support reporting and review, but not so fragmented that every transaction becomes a judgment call. Design accounts around meaningful business questions.

Good account design helps answer:

- where revenue comes from;
- which expenses are controllable;
- which costs relate to direct delivery;
- which costs are overhead;
- which assets need depreciation or tracking;
- which liabilities are owed and to whom;
- which balances require reconciliation;
- which accounts need tax review.

Avoid creating accounts only because a transaction is unusual. If unusual activity matters, create a policy for how it will be classified and reviewed. If it does not matter, keep the structure simple.

### Record Transactions In The Correct Period

Timing matters. A clean total can still be wrong if revenue or expenses are recorded in the wrong period. Consider whether the business uses cash basis, accrual basis, or another required method, and apply it consistently.

Review:

- invoice date versus payment date;
- service period versus billing date;
- delivery date versus order date;
- payroll earned date versus paid date;
- subscription period covered by a payment;
- deposits received before performance;
- costs paid before the related benefit is used;
- unpaid bills that relate to the current period.

Cutoff errors distort trends, margins, taxes, covenants, commissions, budgets, and management decisions. If period placement is uncertain, flag it explicitly.

### Treat Transfers And Payments Carefully

Bank feed imports often create duplicate or misleading entries. A transfer between accounts is not income or expense. A credit card payment is usually a transfer or liability settlement, not a new expense. Loan proceeds are generally a liability, not revenue. Principal payments and interest payments may need different treatment.

Before accepting imported transactions, identify:

- transfers between business bank accounts;
- credit card payments;
- loan proceeds;
- loan principal payments;
- interest expense;
- merchant processor deposits net of fees;
- refunds;
- chargebacks;
- reimbursements;
- payroll clearing entries.

Do not classify based only on the bank memo. Bank descriptions are often incomplete, abbreviated, or misleading.

### Track Assets, Inventory, And Long-Lived Costs

Some purchases should not be expensed immediately without review. Assets, inventory, improvements, deposits, prepaid costs, and certain setup costs may require tracking beyond the date of payment.

For significant purchases, capture:

- acquisition date;
- vendor;
- total cost;
- description;
- asset location or custodian;
- expected use;
- serial number if relevant;
- financing details;
- improvements or related installation costs;
- disposal date when applicable.

This protects depreciation, gain or loss calculations, insurance records, lending support, and operational control. If capitalization rules are unclear, route the item for accounting review rather than deciding casually.

### Maintain Consistent Documentation And Retention

Records should be organized so another person can follow the story later. A bookkeeping process is weak if only the person who entered the data can understand it.

Keep records by period, entity, account, vendor, customer, project, or transaction type as appropriate. Use stable file names, link documents to entries when possible, and avoid storing evidence only in email threads or personal drives. Electronic records should meet the same practical reliability expectations as paper records: complete, readable, organized, backed up, and protected from unauthorized changes.

Retention requirements depend on jurisdiction, transaction type, tax position, employment records, property records, and legal risk. Do not invent a universal retention period. Use the governing policy or official rule, and keep records longer when litigation, audit, property basis, unresolved tax filings, or employment matters require it.

### Keep Corrections Transparent

Corrections should explain what changed and why. Do not delete history in a way that hides the original error. Use adjusting entries, voids, reversals, audit logs, or documented corrections depending on the system and policy.

For each correction, capture:

- original transaction;
- error identified;
- person approving the correction;
- corrected classification or amount;
- date of correction;
- supporting document;
- impact on prior reports or filings.

If a correction affects a closed period, tax filing, investor report, lender covenant, payroll report, or customer invoice, escalate before making the change.

## Common Traps

### Treating Bank Activity As Complete Accounting

A bank statement shows cash movement, not the full business event. It may omit unpaid invoices, unpaid bills, accrued wages, prepaid services, deposits, inventory movement, depreciation, and obligations created outside the bank account. Do not equate "imported from bank" with "properly recorded."

### Letting Convenience Override Sensitivity

It is tempting to attach every document and expose every note to every downstream user. Bookkeeping can include payroll data, personal addresses, bank details, tax identifiers, customer information, medical information, legal matters, and owner compensation. Share only the information needed for the task, and restrict sensitive evidence when summaries are enough.

### Overusing Miscellaneous Accounts

Miscellaneous categories hide risk. A small amount may be acceptable, but repeated use of uncategorized, ask-my-accountant, other expense, other income, or general administrative accounts means the process is not resolving classification. Review these accounts regularly.

### Ignoring Entity Boundaries

Do not combine transactions from different entities, projects, trusts, funds, owners, or locations unless the reporting structure explicitly requires it. Entity confusion can break tax reporting, legal protection, intercompany balances, owner equity, and management analysis.

### Recording Net Amounts Without Understanding Gross Activity

Payment processors, marketplaces, payroll providers, and financing platforms often deposit net amounts after fees, refunds, taxes, tips, chargebacks, or reserves. Recording only the net deposit can understate revenue and expenses. Break out gross activity when reporting requires it.

### Assuming A Prior Classification Was Correct

Recurring rules and bank feed matches can repeat old mistakes. A vendor may provide multiple services. A payment description may change. A transaction that looked like software last month may be equipment this month. Review recurring rules and high-risk vendors periodically.

### Mixing Bookkeeping With Advice

Recording evidence is not the same as deciding tax strategy, revenue recognition policy, lease treatment, inventory method, or payroll classification. When the entry depends on a professional judgment, document the question and route it to the right reviewer.

## Self-Check

- Can each material transaction be traced to a source document or a documented explanation?
- Are business, personal, owner, loan, transfer, taxable, and nontaxable activities separated?
- Are revenue, expenses, assets, liabilities, and equity classified based on substance rather than bank memo text?
- Are transactions recorded in the correct period under the applicable accounting method?
- Are transfers, credit card payments, loan proceeds, refunds, chargebacks, and reimbursements handled without duplicate income or expense?
- Are significant assets, inventory, prepaid costs, deposits, and long-lived costs flagged for tracking or review?
- Are sensitive documents limited to the people or systems that actually need them?
- Are uncategorized and miscellaneous accounts reviewed instead of becoming permanent dumping grounds?
- Are corrections transparent, approved when necessary, and preserved in the audit trail?
- Are retention requirements based on applicable policy or official rules rather than a guessed universal period?
