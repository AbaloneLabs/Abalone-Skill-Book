---
name: balance_sheet_classification.md
description: Use when the agent is classifying a specific balance as current or noncurrent, deciding whether a liability is short-term or long-term, determining if debt should be split between current and noncurrent portions, classifying deferred tax assets and liabilities, or resolving ambiguous classification questions such as restricted cash, deposits, or prepaid assets on a balance sheet.
---

# Balance Sheet Classification

Beyond the overall preparation of the balance sheet, individual balances often present ambiguous classification questions that materially affect liquidity ratios, working capital, and covenant compliance. A bank overdraft, a restricted cash balance, a long-term debt maturity due within twelve months, a deferred tax asset, a refundable deposit, a prepaid expense spanning more than a year — each has specific classification rules that agents often resolve by intuition rather than by applying the standard. Misclassification of a single large item (a current portion of long-term debt, a reclassified restricted cash balance) can flip the current ratio and trigger a covenant breach.

Use this skill before classifying a specific ambiguous balance, splitting debt between current and noncurrent portions, classifying deferred tax, or resolving restricted-cash or long-term-prepaid questions. The goal is to prevent the agent from classifying based on appearance or convention and missing the specific rules that govern each item.

Classification rules are broadly consistent between US GAAP and IFRS but differ in detail (for example, the treatment of debt covenant violations and refinancing). This skill provides the general framework; the applicable standard governs the detail. For covenant-sensitive classifications, involve a qualified accountant or auditor. This is structural guidance, not a determination for a specific balance.

## Core Rules

### Classify The Current Portion Of Long-Term Debt Based On Due Date And Refinancing Status

The portion of long-term debt due within twelve months of the reporting date (or within the operating cycle, if longer) is classified as current, unless the criteria for noncurrent presentation are met. Under IFRS, long-term debt is noncurrent when the entity has discretion to refinance or roll over for at least twelve months after the reporting date, evidenced by a completed refinancing or a firm agreement before the reporting date. Under US GAAP, a similar principle applies: debt is noncurrent if refinanced on a long-term basis by the balance sheet date or if a grace period has been negotiated.

A debt covenant violation that makes debt callable on demand generally requires reclassification to current, unless a waiver has been obtained for at least twelve months. The classification is high-stakes because it affects the current ratio and may trigger covenant analysis.

### Classify Restricted Cash Based On The Nature And Duration Of The Restriction

Restricted cash is classified based on the nature and duration of the restriction. Cash restricted for a short-term purpose (a security deposit returnable within twelve months) may be current; cash restricted for a long-term purpose (escrow for a multi-year project, legally restricted compensating balance) is noncurrent. Cash whose restriction will be lifted within twelve months is current; cash restricted beyond twelve months is noncurrent.

Compensating balances held as support for borrowing arrangements are classified based on the term of the borrowing: short-term borrowing support is current; long-term borrowing support is noncurrent. Disclose the nature and amount of restrictions.

### Classify Deferred Tax Assets And Liabilities As Noncurrent Under IFRS, Split Under US GAAP

Under IFRS (IAS 1 as amended), deferred tax assets and liabilities are classified as noncurrent, without splitting between current and noncurrent. Under US GAAP (ASC 740), deferred tax assets and liabilities are classified based on the classification of the related asset or liability (a deferred tax asset related to a current receivable is current; one related to PP&E is noncurrent). Deferred tax not related to an asset or liability is classified based on the expected reversal period.

Confirm which framework applies, because the classification differs significantly and affects the current ratio.

### Classify Prepaid Expenses Based On The Period Of Benefit

Prepaid expenses are current when the benefit will be consumed within twelve months (or the operating cycle). A prepaid expense spanning more than twelve months (a multi-year insurance premium, prepaid rent on a long-term lease) is split: the portion benefiting the next twelve months is current, and the remainder is noncurrent (sometimes presented within other noncurrent assets).

Do not classify the entire multi-year prepayment as current merely because it is a prepayment. Split based on the benefit period.

### Classify Refundable Deposits Based On Expected Recovery Timing

A refundable deposit (security deposit on a lease, utility deposit) is classified based on when it is expected to be recovered. If the deposit is refundable within twelve months (short-term lease, utility deposit), it is current. If it is tied to a long-term lease and refundable only at lease end (years away), it is noncurrent.

### Classify Receivables Based On Realization Timing, Not Origin

A receivable is current when expected to be realized within twelve months or the operating cycle. A long-term receivable (a loan to an officer repayable over five years, a long-term installment receivable) is noncurrent, split into current portion (amount due within twelve months) and noncurrent portion. Do not classify a multi-year receivable as current merely because it is a receivable.

### Apply The Substance Test To Compound And Mezzanine Instruments

Instruments with both debt and equity features require substance testing. Mandatorily redeemable shares are liabilities. Preferred shares with contingent redemption features may be temporary equity (mezzanine) under US GAAP if redemption is outside the entity's control. Convertible debt is split into liability and equity components. The classification drives where the item appears and affects leverage ratios.

Under IFRS, there is no mezzanine category — instruments are classified as liability or equity based on the substance of the contractual arrangement. Under US GAAP, certain redeemable instruments are presented in temporary equity between liabilities and equity (the "mezzanine").

### Classify Assets And Liabilities Held For Sale As Current

Assets (and liabilities) of a disposal group that meet the held-for-sale criteria are classified as current, presented as a single line (assets held for sale, liabilities held for sale), at the lower of carrying amount and fair value less costs to sell. They are not depreciated or amortized while classified as held for sale. The reclassification from noncurrent to current occurs when the criteria are met, not when the sale is contemplated.

### Disclose Classification Judgments And Their Effects

Disclose material classification judgments (refinancing of debt, classification of compound instruments, restrictions on cash) and their effects on the current ratio and working capital. Users rely on this to assess liquidity and comparability.

## Common Traps

### Classifying All Debt As Noncurrent Despite A Within-Twelve-Month Maturity

The current portion of long-term debt (principal due within twelve months) must be classified as current unless refinancing criteria are met. Leaving it noncurrent understates current liabilities and overstates the current ratio.

### Treating A Covenant Violation As Noncurrent Without A Waiver

A covenant violation that makes debt callable requires current classification unless a waiver for at least twelve months has been obtained. Ignoring the violation leaves debt misclassified.

### Classifying Restricted Cash As Unrestricted

Restricted cash is not part of general cash and equivalents. Present it separately and classify based on the restriction's duration. Including it in unrestricted cash overstates liquidity.

### Splitting Deferred Tax Under IFRS

Under IFRS (as amended), deferred tax is entirely noncurrent. Splitting it into current and noncurrent is incorrect under current IFRS.

### Classifying A Multi-Year Prepayment Entirely As Current

A prepaid expense spanning more than twelve months must be split. Classifying the full amount as current understates noncurrent assets and overstates current assets.

### Classifying Held-For-Sale Assets As Noncurrent

Once the held-for-sale criteria are met, assets are reclassified to current. Leaving them in noncurrent misstates the current ratio and prevents the held-for-sale measurement from being visible.

### Ignoring Mezzanine Equity Under US GAAP

Redeemable instruments outside the entity's control are temporary equity under US GAAP. Including them in permanent equity or as liabilities misstates the capital structure.

## Self-Check

- Is the current portion of long-term debt (due within twelve months) classified as current, with noncurrent presentation only when refinancing criteria are met by the reporting date?
- Are covenant violations that make debt callable reclassified to current unless a waiver for at least twelve months has been obtained?
- Is restricted cash presented separately from unrestricted cash and classified based on the duration of the restriction?
- Are deferred tax assets and liabilities classified as noncurrent under IFRS, or split based on the related asset/liability under US GAAP?
- Are prepaid expenses spanning more than twelve months split between current and noncurrent portions?
- Are refundable deposits and long-term receivables classified based on expected recovery timing, not their origin?
- Are compound and mezzanine instruments substance-tested, with temporary equity presented correctly under US GAAP?
- Are held-for-sale assets and liabilities reclassified to current when criteria are met?
- Are material classification judgments and their effects on liquidity ratios disclosed, and has a qualified accountant or auditor reviewed covenant-sensitive classifications?
