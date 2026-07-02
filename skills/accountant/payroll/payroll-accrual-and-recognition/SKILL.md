---
name: payroll_accrual_and_recognition.md
description: Use when the agent is accruing or recognizing payroll expense, setting period cutoff for earned-but-unpaid wages, handling salaried versus hourly pay, overtime, bonuses, commissions, preparing payroll journal entries, walking gross-to-net, allocating employer burden across departments, or reviewing whether wages are recorded in the correct accounting period.
---

# Payroll Accrual And Recognition

Payroll is usually the largest or second-largest expense on the books, and it is also one of the easiest places to misstate a period. Wages are earned continuously but paid in discrete cycles, so the date cash moves rarely matches the date the obligation was incurred. If an agent simply records payroll when the bank funds the pay run, the period that contains the work days can be understated and the next period overstated. The harm is quiet but real: distorted margins, wrong bonus accruals, overstated cash-basis-to-accrual adjustments, and audit adjustments at year-end.

The judgment problem is not "how to type a journal entry." It is deciding what has been earned but not yet paid, where the period boundary actually falls, how to treat variable pay, how to split the employer burden, and how to keep the liability side honest until the run settles. Agents tend to miss this because payroll comes from a sealed subsystem that prints a net number, so the temptation is to post the summary without decomposing it.

This skill is accounting guidance, not legal or tax advice. Payroll tax rules, overtime law, minimum wage, commission rules, and recognition requirements vary enormously by jurisdiction and by entity type. Route definitive payroll tax, employment law, and benefits compliance questions to a qualified payroll provider, HR professional, or tax advisor. The agent's freedom is to structure the recognition and accrual logic correctly and to flag where jurisdiction-specific inputs are required; it is not to assert a single universal rule.

## Core Rules

### Match Wages To The Period They Are Earned

The core recognition principle is that wage expense belongs in the period in which the employee performed the work, not the period in which the paycheck is issued. A pay period that straddles a month-end or year-end must be split. The days worked through the cutoff date are an accrual in the closing period; the remaining days belong to the next period.

Decision criteria:

- identify the pay period date range, not the check date;
- count work days through the period-end date;
- accrue earned gross wages for those days, including regular pay, shift differentials, and recurring allowances;
- reverse the accrual in the following period or net it against the actual run when it posts;
- confirm the calendar has the right number of periods per year and that the straddle is handled every cycle, not only at year-end.

Do not assume a biweekly cycle produces exactly two pay periods per month. Twice-monthly, biweekly, weekly, and semi-monthly calendars all create different straddle patterns, and the 26-versus-24-pay-period distinction materially shifts annual expense.

### Handle Salaried, Hourly, And Overtime Differently

Each pay type has its own recognition logic. Treat them as separate streams rather than one blended number.

- Salaried staff: accrue straight-line over the working days in the period; watch for partial-period hires, terminations, and mid-period raises that change the daily rate.
- Hourly staff: accrue based on hours worked through cutoff, sourced from time records; do not estimate from a prior period without evidence.
- Overtime: accrue only when it has actually been incurred or is reliably estimable; overtime is not a steady-state number and can spike around deadlines or staffing gaps.
- Commissions and piece-rate: recognize when the earning event has occurred, which may lag the underlying sale; follow the commission plan's earned-on definition.

For salaried exempt staff, be careful about docking pay; jurisdiction-specific salary basis rules govern what deductions are permitted and getting this wrong has both accounting and legal consequences outside this skill's scope.

### Accrue Bonuses And Commissions When Probable

Bonuses, commissions, and incentive pay should be accrued when the obligation is probable and reasonably estimable, even if the formal calculation or payout happens later. The trigger is the performance that creates the obligation, not the board approval or the payment date.

Ask:

- is the bonus contractual, discretionary, or tied to a measurable target;
- has the measurement period ended or is it still open;
- can the amount be estimated reliably;
- is there a service condition that spreads recognition over a period;
- will the payout be grossed up for taxes.

Discretionary bonuses that are not communicated or earned by period-end generally are not accrued. Contractual or formula-driven bonuses generally are. When estimation is unreliable, disclose the limitation rather than forcing a number.

### Decompose Gross To Net In The Journal Entry

A payroll journal entry is not a single expense line. It is a decomposition of gross wages into expense, employer burden, employee withholdings, deductions, and net pay. Posting only net pay to expense hides liabilities and understates both the expense and the withholdings.

A complete entry separates:

- gross wages expense by department or cost center;
- employer payroll tax expense and employer benefit contributions as additional expense;
- employee income tax, social security, and other statutory withholdings as liabilities;
- voluntary deductions (retirement, insurance, garnishments) as liabilities;
- net pay to the clearing or payroll bank account;
- reversal of the prior accrual so earned-but-unpaid amounts are not double counted.

Keep the gross wage figure visible somewhere in the entry or supporting schedule. If the only number that ties is net pay, the entry is incomplete.

### Allocate Employer Burden To The Right Cost Objects

Employer payroll taxes, benefit contributions, and paid-leave accruals are part of total compensation and should follow the employee's labor distribution. Sending all employer burden to a single overhead account distorts product costing, grant reporting, and project profitability.

Allocation criteria:

- use the same department, location, project, or grant code as the base wage;
- handle blended rates where an employee splits time across cost objects;
- treat exempt versus nonexempt burden pools separately if rates differ materially;
- accrue the employer burden in the same period as the related wages so the burden is not shifted into the payment period.

For grant-funded or unionized environments, allocation precision is not optional; it drives reimbursements and compliance.

### Use A Payroll Clearing Account And Reconcile It

A payroll clearing or suspense account is the bridge between the payroll subsystem and the general ledger. Net pay, withholdings, and deductions flow into clearing and then out to the bank, the tax authorities, and the benefit vendors. If clearing is not reconciled, stale balances accumulate and hide unremitted taxes, uncashed checks, or duplicate postings.

Reconcile clearing every cycle:

- clearing should briefly hold net pay until checks clear or direct deposits settle;
- tax and deduction liabilities should move out on their remittance schedule;
- uncashed or stale checks need an escheat or reissue process;
- a non-zero clearing balance after remittance should be explained, not ignored.

### Document Rates, Assumptions, And Sources

Every payroll accrual depends on inputs: pay rates, hours, tax rates, benefit elections, allocation percentages, and calendar logic. Document what drove the number so a reviewer, auditor, or next-period accountant can reproduce it.

Keep, at minimum:

- the pay period and cutoff date used;
- the source time or salary data;
- the accrual calculation method;
- the tax and benefit rates applied;
- the reversal entry reference;
- the approver of the accrual.

## Common Traps

### Recording Payroll On Check Date Instead Of Earned Date

Posting the entire pay run to the check date shifts days worked in one period into the next period's expense. This is the single most common payroll cutoff error and it compounds every straddle cycle. The fix is always to split by work days through the period boundary.

### Treating Net Pay As The Expense

Net pay is what the employee receives; it is not the cost of the employee. Recording only net pay omits the employee taxes, voluntary deductions, and often the employer burden, understating expense and hiding liabilities. Always reconstruct gross and burden.

### Estimating Overtime As A Flat Percentage

Overtime is lumpy. Applying a fixed overtime percentage to straight-time wages produces a smooth but wrong accrual that misses deadline spikes, seasonal patterns, and staffing gaps. Use actual or recently evidenced overtime hours where possible.

### Forgetting To Reverse The Accrual

An accrual that is not reversed (or netted against the subsequent run) double counts the wages once the real payroll posts. Build the reversal into the recurring process and confirm the accrual balance returns to zero before the next cutoff.

### Dumping All Employer Burden Into One Overhead Account

Pooling employer taxes and benefits into a single overhead line makes product, project, and grant costing wrong and hides the true cost of labor by department. Allocate burden on the same basis as base wages.

### Ignoring Partial-Period Hires, Raises, And Terminations

A mid-period raise changes the daily accrual rate from that date forward. A new hire who starts three days before cutoff earns only those days. A terminating employee may have a final pay with vacation payout that needs separate treatment. Applying a flat monthly salary ignores all of these.

### Trusting The Payroll Report Without Decomposing It

A payroll register is a source document, not a journal entry. It still must be decomposed into expense, liabilities, and net pay, mapped to the right accounts and departments, and reconciled to clearing. Posting the register total as one line is a control failure.

## Self-Check

- Is wage expense recognized in the period the work was performed, with straddle pay periods split at the cutoff?
- Are salaried, hourly, overtime, commission, and bonus streams recognized using the correct logic for each?
- Are bonuses and commissions accrued only when probable and reasonably estimable, with discretionary amounts excluded until earned?
- Does the journal entry decompose gross wages, employer burden, employee withholdings, deductions, and net pay into the right accounts?
- Is employer tax and benefit burden allocated to the same department, project, or grant as the base wages?
- Is the payroll clearing account reconciled each cycle with every residual balance explained?
- Is the prior-period accrual reversed or netted so wages are not double counted?
- Are partial-period hires, raises, and terminations reflected in the daily accrual rate?
- Are the rates, assumptions, sources, cutoff date, and approver documented so the accrual can be reproduced?
- Are jurisdiction-specific payroll tax and employment law questions flagged for a qualified payroll provider or tax advisor rather than answered with a single universal rule?
