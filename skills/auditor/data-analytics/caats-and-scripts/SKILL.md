---
name: caats-and-scripts.md
description: Use when the agent is designing or using computer-assisted audit techniques and audit scripts, extracting and processing data with tools like SQL, Python, Excel, or dedicated audit software, validating script logic and data integrity, managing version control and reproducibility, or deciding when a CAAT adds reliable evidence versus introducing unmanaged risk.
---

# CAATs and Scripts

Computer-assisted audit techniques (CAATs) and audit scripts — whether SQL queries, Python notebooks, Excel models, or dedicated audit software — let the auditor process entire populations, run complex tests, and produce evidence that manual procedures could not. They also introduce a new evidence risk: a script that looks correct but contains a logic error, processes the wrong data, or cannot be reproduced produces a confidently wrong result. The discipline is to treat the script itself as part of the evidence, validating its logic, its data, and its reproducibility with the same rigour applied to any other audit procedure.

## Core Rules

### Validate the data before trusting the script's output

A correct script run on the wrong or incomplete data produces a wrong result. Before relying on any CAAT output:

- reconcile the input dataset to the source system or ledger (record counts, monetary totals);
- confirm the dataset covers the full period, locations, and transaction types in scope;
- confirm field definitions, formats, and units are consistent and understood;
- confirm no records were dropped, filtered, or duplicated in extraction.

Document the reconciliation. An input dataset that is not reconciled to an independent total is an unverified foundation; everything built on it inherits that uncertainty.

### Validate the script logic independently

Script logic is where the most dangerous errors hide, because a wrong script runs cleanly and produces plausible output. Validate logic by:

- **Peer review**: have someone other than the author review the logic against the intended test.
- **Re-performance on a controlled subset**: run the script on a small, hand-verified sample and confirm the output matches expected results.
- **Testing edge cases**: empty populations, zero values, negative values, duplicates, missing fields, date boundaries.
- **Cross-checking totals**: confirm the script's output reconciles to inputs (e.g., total of tested items plus exceptions equals input total).

Do not accept a script's output on the author's say-so. The validation is part of the evidence; without it, the output is an assertion.

### Document the script, its purpose, and its logic

For each CAAT, document:

- the assertion or risk the script addresses;
- the input data, its source, and its reconciliation;
- the logic, in plain language and in the code or query;
- the assumptions and known limitations;
- the output and how it is interpreted.

The documentation should let another auditor understand, review, and re-run the script. A script whose logic exists only in its author's head is not reviewable evidence.

### Ensure reproducibility and version control

Audit evidence must be reproducible: another auditor, at a later date, should be able to re-run the script and obtain the same result. Ensure:

- the script and its inputs are saved and version-controlled (not lost when a notebook is closed or a temp file is overwritten);
- the environment (tool versions, libraries, parameters) is recorded;
- the script is parameterised or documented so it can be re-run on the same or a comparable dataset;
- changes to the script during the audit are tracked, with rationale.

A result that cannot be reproduced because the script or inputs were lost or changed is not defensible evidence.

### Reconcile output to the source and to other procedures

The script's output should reconcile both backwards (to the input data) and forwards (to the conclusion). Confirm:

- the output's totals reconcile to the input (nothing lost or double-counted);
- exceptions identified reconcile to items investigated and resolved;
- the conclusion drawn from the output is consistent with other audit evidence (no unexplained contradictions).

Unreconciled outputs are a red flag; the reconciliation is what confirms the script processed the intended population correctly.

### Manage the risk of Excel and end-user computing tools

Excel and similar tools are powerful CAATs but carry specific risks:

- formulas can be overwritten or corrupted (especially in shared files);
- cell references can break when rows are added or sorted;
- macros can contain errors or be disabled;
- version control is often informal, with multiple copies and unclear final versions.

For Excel-based CAATs, apply stronger controls: lock input cells, document formulas, use defined names rather than cell references where possible, maintain a clear final version, and have the model reviewed. Treat a material Excel model with the same seriousness as a script in a programming language.

### Match the tool to the task and to the auditor's competence

Use the right tool for the task and for the team's competence:

- **SQL** for relational data extraction and aggregation, where the data is in a database.
- **Python or R** for complex logic, statistics, and reproducible scripts.
- **Excel** for small datasets, simple logic, and ad-hoc analysis where its risks are manageable.
- **Dedicated audit software** for standardised tests with built-in validation.

Do not use a tool the team cannot validate. A complex Python notebook that no one on the audit can review is higher risk than a simpler Excel model that can be checked. Competence to validate is part of the tool selection.

### Confirm the CAAT addresses a specific assertion or risk

As with any procedure, a CAAT must map to an assertion or risk. Before building a script, state what assertion it tests and how the output provides evidence. A CAAT built because the data is available, without a clear evidential purpose, produces effort without assurance. The assertion-link discipline applies regardless of the tool's sophistication.

### Treat the CAAT as evidence, including its limitations

The CAAT's output is evidence only to the extent the data, logic, and reproducibility are validated. Record the limitations: what the script does not test, what assumptions it makes, what could make its output misleading. A CAAT presented as conclusive without its limitations documented overstates the evidence. Honest documentation of limitations is what makes the evidence usable and defensible.

## Common Traps

- **Trusting script output without validating the input data**, so a correct script produces a confidently wrong result on incomplete data.
- **Accepting script logic without peer review or re-performance**, allowing logic errors to propagate through the whole population.
- **Using a tool the team cannot validate**, so the output cannot be reviewed and its correctness is an assertion.
- **Losing reproducibility** through poor version control, overwritten files, or undocumented environments.
- **Failing to reconcile output to input and to the conclusion**, missing errors in processing.
- **Over-relying on Excel** for material models without locking, documenting, and reviewing formulas.
- **Building CAATs without a clear assertion link**, producing data processing rather than audit evidence; **Documenting the output but not the script, logic, assumptions, and limitations**, leaving the procedure unreviewable
- **Treating the CAAT as conclusive** without recording what it does not test or what could mislead; **Re-running last year's script without re-validating** its logic and data against this year's population and systems

## Self-Check

- Did I reconcile the input dataset to the source system or ledger for completeness and accuracy before running the script?
- Was the script logic validated by peer review, re-performance on a controlled subset, edge-case testing, and cross-checking of totals?
- Is the script documented — purpose, input, logic, assumptions, limitations, output — so another auditor can review and re-run it?
- Are the script and its inputs version-controlled and reproducible, with the environment recorded?
- Does the output reconcile backwards to the input and forwards to the conclusion and other evidence?
- For Excel-based CAATs, are formulas locked, documented, and reviewed, with a clear final version?
- Is the tool matched to the task and within the team's competence to validate?
- Does the CAAT map to a specific assertion or risk, rather than being built because the data was available?
- Are the script's limitations documented, so the evidence is not overstated?
- If reusing a prior script, did I re-validate its logic and data against the current population and systems?
