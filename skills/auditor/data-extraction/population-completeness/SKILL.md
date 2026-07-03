---
name: population_completeness.md
description: Use when the agent is verifying that an audit population is complete, reconciling a data extract to the general ledger or subledger, confirming all locations, entities, currencies, and periods are included, handling deleted, voided, or manual items, identifying missing transaction ranges, or performing sequence and gap completeness checks before sampling or testing.
---

# Population Completeness

Population completeness is the foundation of every test that draws from a dataset. If the population is missing a location, a currency, a period, or a class of transactions such as voids or manual entries, then every sample, every analytic, and every conclusion built on it is biased in ways the auditor cannot see from inside the data. Agents tend to accept the extract as the population because the row count and total look right, but a reconciling total can hide offsetting omissions, and a complete-looking file can still exclude the exact transactions where risk concentrates. The harm this prevents is issuing a conclusion that appears well-supported but actually rests on a population that silently excluded the riskiest items, which undermines the entire procedure and can require the work to be redone from scratch.

Use this skill before defining a sampling frame, before running any population-based analytic, and whenever an extract is reconciled to the GL. The goal is documented evidence that the population contains every transaction it should, with no gaps, no excluded classes, and no unexplained differences to the ledger.

## Core Rules

### Define The Population Explicitly Before Testing It

The population must be defined before it can be tested for completeness. A vague definition such as "all sales" leaves room for arguments later about whether intercompany, consignment, returns, or manual entries were meant to be included.

The definition should specify:

- the assertion and account the population supports;
- the entity or entities in scope;
- the locations or branches included;
- the currencies included and the conversion basis;
- the period and the exact cutoff date;
- the document or transaction types included;
- the treatment of voids, reversals, drafts, and holds;
- the treatment of manual, adjusting, and system-generated entries.

### Reconcile The Extract To The GL At Multiple Levels

A single grand-total reconciliation can pass while a whole entity or period is missing. Reconcile at several levels so that offsetting omissions are exposed.

Reconcile by:

- grand total value and count to the GL control account;
- entity or company code subtotal to the consolidation;
- location or branch subtotal where applicable;
- currency subtotal, with conversion basis stated;
- period, month, or posting-date bucket;
- document type or transaction class;
- adjusted versus unadjusted basis, as relevant to the assertion.

### Confirm Every Expected Scope Element Is Present

An extract that silently defaults to one entity or one ledger is one of the most common completeness failures. Confirm presence, not just totals, for every element the engagement requires.

Confirm presence of:

- every entity and company code on the consolidation list;
- every location or branch in scope;
- every currency in which the entity transacts;
- every period from opening to cutoff;
- every document type that should appear;
- every ledger (general, consolidation, adjustment) in scope;
- any newly acquired or newly created entities for the period.

### Handle Deleted, Voided, And Manual Items Deliberately

These items are often excluded by default reports and are exactly where completeness failures and risk concentrate. Decide, per assertion, whether they belong in the population and confirm their presence or justified absence.

For each class, determine:

- whether voided or reversed documents are included or excluded, and why;
- whether deleted records are recoverable from an audit table or log;
- whether manual journal entries are in the population;
- whether held, draft, or future-dated documents are included;
- whether system reversals net to zero or appear as paired entries;
- whether the treatment is consistent with the assertion being tested.

### Perform Sequence And Gap Completeness Checks

For populations with sequential identifiers such as invoice or check numbers, sequence checks are a powerful completeness test that totals alone cannot provide. A gap in the sequence is evidence that a record may be missing.

Sequence checks to perform:

- identify the sequential key and confirm it is truly sequential;
- list every gap in the sequence with the surrounding numbers;
- obtain an explanation for each gap (voided, cancelled, not used);
- confirm the explanation is supported by source documentation;
- check the lowest and highest numbers against expected ranges;
- verify no duplicate sequence numbers exist;
- confirm the sequence covers the full period with no unexplained breaks.

### Investigate Every Unexplained Difference

A difference between the extract and the GL is a signal, not a nuisance. Even an immaterial difference can indicate a systematic exclusion such as a status code filter or a missing entity.

For each difference:

- quantify it in value and count;
- isolate it to an entity, period, or document type where possible;
- obtain the underlying records that explain it;
- confirm the explanation is consistent across the population;
- decide whether the difference affects the population definition;
- document the conclusion and who reviewed it.

### Document The Completeness Conclusion With Evidence

Completeness is a conclusion that must be supported, not assumed. The workpaper should allow a reviewer to see what was tested, what was found, and why the population is considered complete.

The documentation should include:

- the population definition and the assertion it supports;
- the reconciliations performed and their results;
- the scope elements confirmed present;
- the sequence and gap checks and their resolutions;
- the treatment of voids, manual, and deleted items;
- any residual limitations and their impact;
- the preparer and reviewer sign-off.

### Re-Confirm Completeness After Population Changes

Populations change when late entries post, when scope expands, or when an error is corrected. A completeness conclusion reached once does not survive a refreshed extract.

Re-confirm completeness after:

- receipt of a refreshed or re-extracted population;
- posting of late adjustments or year-end entries;
- expansion of scope to a new entity, location, or currency;
- correction of an error found in the original extract;
- rollover to a new period or fiscal year.

### Tie Completeness To The Specific Assertion

Completeness is not a universal property; it is relative to the assertion being tested. A population that is complete for testing existence of receivables may be incomplete for testing completeness of revenue, because the two assertions draw on different directions and different document classes.

Match the population to the assertion by:

- confirming the population direction (debit versus credit) matches the assertion;
- including all document classes relevant to the assertion, not just the dominant one;
- excluding items that do not belong to the asserted account or class;
- reconciling to the specific GL account or subledger the assertion addresses;
- documenting why the population as defined is appropriate for that assertion.

## Common Traps

### Equating A Matching Grand Total With A Complete Population

A grand total that ties can hide an entity that is missing and another that is double-counted, because the errors offset. Always reconcile at entity, currency, and period level so offsetting omissions are exposed.

### Letting The System Default Decide The Scope

Many reports default to one company code, one ledger, or the current fiscal year. Running "all transactions" and receiving only the default entity is a frequent and serious failure. Confirm scope explicitly against the consolidation list every time.

### Excluding Voids And Manuals By Assumption

Voids, reversals, and manual entries are often the riskiest part of a population, and they are the items most likely to be excluded by default report filters. Never assume their absence is correct; confirm it against the assertion being tested.

### Ignoring Gaps In A Sequential Identifier

A gap in an invoice or check sequence is direct evidence that a record may be missing. Dismissing gaps as "normal" without obtaining supporting documentation undermines the completeness test entirely.

### Treating An Unexplained Difference As Immaterial

An unexplained difference, however small, is a symptom that the population may not be complete. Investigating the cause is more important than the size of the difference; materiality is decided after the cause is known.

### Forgetting Newly Acquired Entities Or New Currencies

A new subsidiary or a new currency introduced mid-period is easy to miss because it was not in last year's scope. Check the consolidation list and the currency master for additions within the period.

### Reconciling Only Value And Forgetting Count

A value tie with a wrong count can indicate duplicate or dropped records that net to the same total. Reconcile both value and count, and investigate any case where one ties and the other does not.

### Treating Completeness As A One-Time Check

Completeness established on the first extract does not hold for a refreshed or rolled-forward population. Each new version of the file must be re-reconciled and re-confirmed before it is relied on.

## Self-Check

- Is the population defined explicitly, including entities, locations, currencies, period, and document types?
- Does the extract reconcile to the GL at entity, currency, and period level, not just at the grand total?
- Has the presence of every expected scope element been confirmed against the consolidation list?
- Are voids, reversals, manual entries, and held or draft documents deliberately included or excluded per the assertion?
- Have sequence and gap checks been performed, with every gap explained and supported by documentation?
- Is every unexplained difference investigated for cause before any materiality decision is made?
- Are both value and count reconciled, with discrepancies between them investigated?
- Are newly acquired entities, new locations, and new currencies confirmed present for the period?
- Does the completeness documentation state the definition, reconciliations, scope confirmation, and residual limitations?
- Has completeness been re-confirmed after any refresh, scope expansion, or correction to the population?
- Would a reviewer be able to reproduce the completeness conclusion from the workpaper evidence alone?
