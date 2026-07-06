---
name: detail-test-design.md
description: Use when the agent is designing tests of details for a significant account or assertion, selecting the appropriate procedure (confirmation, inspection, recalculation, observation, vouching, tracing), determining direction of testing, deciding evidence source and timing, or building a detail test that addresses a specific risk with sufficient appropriate evidence.
---

# Detail Test Design

A test of details examines individual transactions or balances to obtain direct evidence about whether they are correctly stated. Designing the right test is what converts a risk assessment into evidence: the wrong procedure, the wrong direction, or the wrong source produces effort without assurance. The discipline is to design each detail test backwards from the specific assertion and risk it must address, choosing the procedure whose evidence most directly and persuasively supports (or refutes) that assertion, rather than defaulting to whatever procedure is most convenient or customary.

## Core Rules

### Design backwards from the assertion and risk

Start with the assertion being tested (existence, completeness, accuracy, valuation, rights, cutoff, presentation) and the specific risk of misstatement to it. Then ask: what evidence would most directly confirm or refute that this assertion holds for this population? The answer dictates the procedure. Examples:

- **Existence of receivables**: external confirmation with debtors is direct evidence.
- **Completeness of liabilities**: searching for unrecorded items by examining subsequent payments or unmatched receiving reports tests in the right direction.
- **Valuation of inventory**: observation of physical counts plus testing of cost and net realisable value.
- **Cutoff**: examining transactions around the period boundary in both directions.

A procedure that does not map to a specific assertion is not targeted; it is generic effort.

### Match the procedure type to the assertion

Different procedures provide evidence of different strength for different assertions:

- **External confirmation**: strong for existence and rights; weak for completeness (a missing balance may not appear).
- **Inspection of documents / assets**: strong for existence (physical assets, signed contracts); strength varies for valuation.
- **Recalculation**: strong for accuracy (depreciation, accruals, revenue).
- **Observation**: strong for existence of physical assets and for control operation; weak for assertions about point-in-time balances unless performed at the right date.
- **Vouching (source to record)**: tests existence and occurrence (does the recorded item have a real source?).
- **Tracing (record to source, or source to record)**: tracing from source to record tests completeness; vouching from record to source tests existence.

Selecting the wrong procedure type is a common reason a detail test fails to support the intended conclusion.

### Get the direction of testing right

Direction is decisive for existence vs completeness, the two assertions most often confused:

- To test **existence / occurrence** (is the recorded item real?), start from the records and vouch to supporting source documents or external evidence.
- To test **completeness** (is everything that should be recorded actually recorded?), start from the source (shipping log, receiving reports, subsequent disbursements) and trace forward to the records.

Testing existence-direction for a completeness risk, or vice versa, produces evidence about the wrong assertion. State the direction explicitly for every test and confirm it matches the assertion.

### Choose the most persuasive evidence source

Evidence persuasiveness varies by source. Prefer, where feasible:

- **External evidence** (third-party confirmations, bank statements, external valuations) over internal.
- **Documentary evidence** created at or near the transaction date over reconstructive evidence.
- **Original documents** over copies or system reproductions.
- **Evidence from independent parties** over evidence from the entity's own staff.

Where internal evidence must be used (common for many procedures), corroborate it with external or independent evidence where possible. A detail test resting solely on internally generated documents for a high-risk assertion provides weaker assurance and may need to be supplemented.

### Time the procedure to the assertion

Timing matters most for balance-sheet assertions and for cutoff:

- **Existence at period-end**: procedures should be performed at or near the balance sheet date, or rolled forward/back with appropriate procedures to bridge the gap.
- **Cutoff**: examine transactions in the days before and after the period boundary.
- **Transactions over the period**: sample across the period, with attention to high-risk points (close, peak volume).

A detail test performed months before period-end for a period-end existence assertion requires roll-forward procedures; a test performed only at interim without roll-forward does not support the year-end balance.

### Address completeness through deliberate source-based procedures

Completeness is the hardest assertion to test because the missing items are, by definition, not in the records. Design completeness procedures that start outside the records:

- examine subsequent cash disbursements for unrecorded liabilities;
- reconcile shipping/receiving records to recorded revenue/inventory;
- review unmatched receiving reports or unmatched purchase orders;
- perform cut-off testing in both directions;
- use analytical procedures to identify unexpected gaps.

Defaulting to record-based sampling tests existence, not completeness; completeness requires reaching outside the ledger.

### Design for the specific risk, not the generic risk

A population may carry several risks, each requiring a different test. For receivables: existence (confirmation), valuation/allowance (ageing, credit review, subsequent cash receipts), rights (confirmation of factoring or pledges), completeness (cut-off, sales trace). Design a test for each significant risk rather than a single test assumed to cover all assertions. The most common gap is a strong existence test paired with no valuation test for an area where valuation is the real risk.

### Combine procedures for sufficient appropriate evidence

A single procedure rarely provides enough evidence for a material, high-risk assertion. Combine:

- confirmation with recalculation of the confirmed balance;
- observation with recalculation of cost and testing of net realisable value;
- vouching with analytical procedures;
- substantive procedures with the results of controls testing.

State how the combination provides sufficient appropriate evidence for the assertion, so the overall conclusion is defensible.

### Document the design rationale, not just the procedure

For each detail test, document: the assertion and risk addressed, the procedure type, the direction, the evidence source and its persuasiveness, the timing, the sample or coverage, and how the result will be evaluated. This documentation is what makes the test defensible and what forces the discipline of risk-first design. A procedure performed without documented design rationale is hard to review and hard to defend if the conclusion is challenged.

## Common Traps

- **Defaulting to a customary procedure** (e.g., always confirming receivables) without confirming it addresses the specific risk, which may be valuation rather than existence.
- **Getting the direction of testing wrong** — vouching to test completeness, or tracing to test existence — producing evidence about the wrong assertion.
- **Relying on internal evidence only** for a high-risk assertion, without external or independent corroboration.
- **Performing period-end existence procedures at interim without roll-forward**, leaving a gap to year-end.
- **Testing existence strongly but neglecting valuation** in areas (receivables, inventory, investments) where valuation is the real risk.
- **Designing a single test to cover all assertions**, missing the assertion-specific risks.
- **Defaulting to record-based sampling for completeness**, which tests existence instead.
- **Failing to combine procedures**, expecting one procedure to carry a material assertion alone; **Documenting the procedure without the design rationale**, leaving the test indefensible and hard to review

## Self-Check

- For each detail test, did I start from the specific assertion and risk, and choose the procedure whose evidence most directly addresses it?
- Did I match the procedure type (confirmation, inspection, recalculation, observation, vouching, tracing) to the assertion it best supports?
- Is the direction of testing correct — vouching for existence/occurrence, tracing from source for completeness?
- Did I choose the most persuasive evidence source available, preferring external and contemporaneous over internal and reconstructive?
- Is the timing appropriate — at or near period-end for balance assertions, with roll-forward where interim, and across the period for transactions?
- For completeness risks, did I design source-based procedures that reach outside the records rather than record-based sampling?
- Did I design a separate test for each significant risk, rather than assuming one test covers all assertions?
- Did I combine procedures to provide sufficient appropriate evidence, and document how the combination supports the conclusion?
- Is the design rationale documented — assertion, risk, procedure, direction, source, timing, coverage, evaluation — so the test is defensible?
