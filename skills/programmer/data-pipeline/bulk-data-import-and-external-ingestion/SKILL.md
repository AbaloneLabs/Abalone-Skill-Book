---
name: bulk_data_import_and_external_ingestion.md
description: Use when the agent is importing bulk data from an external partner, vendor, customer, or third-party source, ingesting CSV, flat-file, or API-delivered data into a production system, designing an import pipeline, handling schema mismatches and dirty data from sources you do not control, backfilling from external feeds, or reviewing whether an import can corrupt or overload the system.
---

# Bulk Data Import And External Ingestion

Importing data from a source you do not control is fundamentally different from handling data your own application produced. Your own code follows your schemas, your invariants, and your conventions because you wrote it. External data follows whatever the partner, vendor, customer, upstream team, or legacy export decided — and they change it without telling you, send files late, send the wrong format, send duplicate rows, send more rows than the system can absorb, or send data that violates assumptions you did not know you were making. The judgment problem is that external data is untrusted, unstable, and unbounded, and treating it like trusted internal input is how imports corrupt production tables, overwhelm downstream services, and silently introduce wrong values that propagate for weeks before anyone notices.

Agents tend to design an import as a straightforward read-transform-write: parse the file, map the columns, insert the rows. This works on the sample file the partner sent and fails on the next one, because the real failure modes of external ingestion are not in the happy path. The partner renames a column; the date format switches from ISO to locale-specific; a field that was always populated is suddenly empty; the file that had 10,000 rows now has 10 million; a value that was always numeric now contains a free-text note; the encoding changes from UTF-8 to something with a byte-order mark; the daily file arrives twice, or not at all. Each of these is routine in external data and catastrophic if the import assumes stability. The disciplined engineer designs an import that validates before it commits, isolates bad data so one bad row does not poison the batch, is idempotent so a re-run does not duplicate, is observable so a silent drift is caught, and is reversible so a bad import can be undone without restoring the whole database.

This skill is about ingesting external data into a system you own. It complements the data-migration skill (which transforms data you already control) and the data-pipeline skills (which move data between internal systems). Here the defining constraint is that you do not control the source, and the source will not respect your assumptions.

## Core Rules

### Treat External Data As Untrusted, Unstable, And Unbounded

The single most important shift is to stop assuming the source behaves. External data has three properties that internal data does not, and every design decision flows from them:

- **Untrusted.** The data may be wrong, malformed, incomplete, duplicated, or adversarial. Validate everything before it touches a production table. Never insert external rows directly into the system of record without a validation and quarantine layer.
- **Unstable.** The schema, format, encoding, volume, and delivery schedule will change without notice. The partner will "improve" their export and break your parser. Design the import to fail loudly on unexpected shapes rather than silently misinterpreting them.
- **Unbounded.** The file can be far larger than the sample, or far larger than the system can absorb in one pass. Never load an external file fully into memory, and never let an import run unthrottled against a shared production database.

If you remember only one thing: the sample file the partner sent you is a courtesy, not a contract. The production file will differ from it in ways that matter.

### Validate Before You Commit, And Fail Loudly On Shape Changes

An import should never discover a problem by writing bad data and seeing what breaks downstream. Validate the data in a staging area before any of it reaches the system of record. Validation has two layers, and both matter:

- **Shape validation (schema-level).** Are the expected columns present, with the expected types and formats? Has a column been renamed, removed, added, or retyped? A column-count check, a header check, and a type check on each field catch the most common breakage (the partner changed the export) before a single row is processed. Fail loudly — stop the import and alert — when the shape is not what you expect. Silent acceptance of a changed shape is how a column shift causes every row to be misinterpreted.
- **Value validation (row-level).** Does each value fall within plausible bounds? Required fields populated, enums within the allowed set, dates within range, numerics within bounds, foreign keys resolvable, uniqueness constraints satisfiable. Rows that fail value validation should be quarantined, not inserted, and the good rows should proceed (or the whole batch should fail, depending on the tolerance — see below).

The goal is that a malformed file is rejected at the gate, and a malformed row is quarantined, before either can corrupt the system of record. An import that writes first and validates via downstream errors has it backwards.

### Decide A Row-Failure Policy: All-Or-Nothing Versus Quarantine-And-Continue

When some rows in a batch are valid and some are not, you must decide whether the batch is atomic or partial, and the choice depends on the meaning of the data:

- **All-or-nothing (atomic batch).** If the rows are interdependent (a set of related records that must all load together to be consistent), a single bad row should fail the whole batch, roll back, and alert. Partial loading would leave the dataset in an inconsistent state.
- **Quarantine-and-continue (partial batch).** If the rows are independent (a list of customer records, product catalog entries), load the valid rows, quarantine the invalid ones to a rejection table with the reason, and report the rejection count. One bad row should not block 99,999 good ones.

The trap is choosing the wrong policy. Quarantining-and-continuing on interdependent data leaves it half-loaded; failing the whole batch on independent data blocks the entire import behind a single bad row. Make the choice explicit, and document it so the next person knows whether a partial load is a success or a failure.

### Make The Import Idempotent And Re-runnable

External delivery is unreliable: the same file arrives twice, the daily job runs twice after a redeploy, the partner re-sends yesterday's file with corrections. An import that inserts rows on each run will duplicate data on every re-run. Design the import so that running it twice produces the same result as running it once:

- **Use a stable external key** from the source (the partner's record id) as the deduplication anchor, and upsert (insert-or-update) on that key rather than blindly inserting.
- **Track what has been imported.** Record the source file, the batch id, the row range, and a checksum, so a re-run of the same file is detected and skipped rather than re-applied.
- **Make each batch reversible.** Tag imported rows with their source batch so a bad import can be rolled back by deleting or reverting exactly the rows that batch introduced, without a full database restore.

Idempotency is what makes it safe to retry a failed import, to re-run after a fix, and to tolerate duplicate delivery. An import that is not idempotent is an import that will eventually duplicate or corrupt data, because re-runs are inevitable.

### Isolate, Throttle, And Never Import Directly Into The Hot Path

A bulk import is, by definition, a large write operation against a database that is also serving live traffic. Running it naively locks tables, saturates the write path, exhausts connections, and degrades the experience for every live user. Treat the import as a potentially harmful workload and isolate it:

- **Import into a staging table first**, never directly into the production table. Validate, transform, and review in the staging area, then promote the validated batch in a controlled way. This keeps bad data out of the hot path and lets you inspect before you commit.
- **Batch and throttle the writes.** Insert in bounded batches with pauses, respect the database's capacity, and avoid long-running transactions that hold locks. A single giant transaction locks and blocks; many small batches flow through.
- **Run imports off-peak or in a separate process** that does not compete with request-serving for connections and CPU. For very large or frequent imports, consider a replica or a dedicated ingest path.

The failure mode of an unthrottled import is a production slowdown or outage that coincides with every import run. If users complain whenever the nightly feed loads, the import is competing with them for resources it should not.

### Handle The Schema-Mismatch Reality Of External Sources

External schemas drift. A column is added, removed, renamed, retyped, or reordered, and your positional or name-based parser breaks or silently misreads. Defend against this:

- **Parse by column name, not position**, when the source provides headers, so reordering does not misalign fields. But also validate that the expected names are present, because renamed columns are just as breaking as reordered ones.
- **Pin to a schema version or contract** with the source when possible, and detect when the delivered file no longer matches it. A version field, a column manifest, or a documented column list lets you detect a change rather than misinterpreting it.
- **Decide explicitly how to handle extra columns** (ignore them, or fail because the shape changed) and missing columns (fail, or default with a warning). Ignoring unexpected changes silently is how an import keeps "working" while reading the wrong data.

The robust posture is: any change to the source shape is an event that requires a human decision, not something the import silently absorbs. A parser that accepts anything will eventually accept the wrong thing.

### Make The Import Observable: Counts, Rejections, And Drift

A silent import is a dangerous import. Because external data is untrusted and unstable, you need to see what it did, not just that it "finished." Instrument every import run with:

- **Counts that must reconcile.** Rows received, rows passed validation, rows quarantined, rows inserted, rows updated. These should add up, and a sudden change in the ratio (a jump from 0.1% to 15% rejections) signals that the source changed or broke.
- **Alerting on anomaly.** A rejection rate above a threshold, a row count far outside the historical range, a file that did not arrive on schedule, or a shape-validation failure should alert a human. The most common import incident is "the file changed three weeks ago and nobody noticed."
- **A rejection log that is actionable.** Quarantined rows should record the row, the value, and the specific reason, so someone can fix and re-submit rather than guessing why rows disappeared.

An import that reports only "success" or "failure" hides the gradual drift that precedes every import incident. Make the numbers visible and the anomalies loud.

### Plan For The Source That Is Late, Missing, Or Malicious

External delivery is not reliable. Design for the operational realities:

- **Late or missing delivery.** The file does not arrive on schedule. Detect the absence (a scheduled check that the expected file exists), alert, and have a defined escalation rather than silently having no data for the day.
- **Duplicate or corrected delivery.** The same file arrives twice, or a corrected version supersedes the earlier one. Idempotency and batch tagging (above) make this safe.
- **Adversarial or hazardous content.** A customer-uploaded file, a partner feed that was compromised, or a genuinely malformed file can contain pathologically large fields, executable content, or values designed to exploit a parser. Treat imported content as untrusted input: bound field lengths, avoid executing or interpreting embedded content, and parse with hardened libraries.

An import that assumes the source is always correct, present, and benign will eventually meet a source that is none of those.

## Common Traps

### Inserting External Rows Directly Into The Production Table

The fastest implementation — parse and insert straight into the live table — is also the most dangerous, because malformed or unexpected data reaches the system of record before it is validated. The fix is to always land external data in a staging table, validate and transform there, and promote a reviewed batch. The system of record should never be the validation layer.

### Silent Acceptance Of A Changed Source Shape

The partner renames or reorders columns, and the import either misreads every row (positional parsing) or drops a field silently (name parsing that ignores unknowns), and the wrong data loads without an error. The fix is explicit shape validation that fails loudly when the expected columns are not exactly as expected, treating any shape change as an event requiring review. A parser that accepts anything will accept the wrong thing.

### Loading The Entire File Into Memory

A 50 MB sample file is fine to read into memory; the 5 GB production file is not, and the import crashes with an out-of-memory error or, worse, the language's CSV library reads it all before failing. The fix is streaming parsing that processes the file row by row or in chunks, never holding the whole file in memory. Size your buffers for the production file, not the sample.

### An Unthrottled Import That Degrades Production

The import runs as one large transaction or an unbounded insert loop, locking tables and saturating the write path, so live users see slowdowns or errors whenever it runs. The fix is bounded batches with throttling, importing into a staging table, and running off the request-serving path. An import that competes with users for the database is an import that will be blamed for every outage.

### Non-Idempotent Import That Duplicates On Re-Run

The job runs twice (a redeploy, a manual retry, a duplicate delivery) and inserts every row again, because there is no deduplication key. The fix is upserting on a stable external key and tracking imported batches so a re-run is a no-op. Without idempotency, every retry is a corruption event.

### Choosing The Wrong Row-Failure Policy

Quarantine-and-continue on interdependent data leaves a batch half-loaded and inconsistent; all-or-nothing on independent data blocks the entire import behind a single bad row for days. The fix is to make the policy an explicit, documented decision based on whether the rows are interdependent, and to review whether the choice still holds when the data changes.

### Trusting The Sample File As A Contract

The import is built and tested against the sample the partner sent, which has clean data, consistent formats, and modest size, and it breaks on the first real file. The fix is to test against messy, large, and edge-case inputs (empty fields, unicode, duplicate rows, format variations) and to treat the sample as illustrative, not normative. The production file will differ from the sample in ways that matter.

### Silent Drift Because Nobody Watches The Numbers

The import "succeeds" every night, but the rejection rate has crept from 0.1% to 8% over a month as the source degraded, and nobody noticed until a downstream report was wrong. The fix is to instrument counts and rejection rates, alert on anomaly, and make the rejection log actionable. The most common import incident is a slow drift that a dashboard would have caught.

## Self-Check

- [ ] External data is treated as untrusted, unstable, and unbounded — it lands in a staging table, is validated before any row reaches the system of record, and is never inserted directly into a production table.
- [ ] Shape validation (expected columns, types, formats) fails loudly on any change to the source schema, so a renamed, reordered, or retyped column stops the import rather than silently misreading it.
- [ ] Row-level validation quarantines invalid rows with a reason, and the row-failure policy (atomic vs. quarantine-and-continue) is an explicit decision matching whether the rows are interdependent.
- [ ] The import is idempotent: it upserts on a stable external key, tracks imported batches and checksums, and re-running the same file is a no-op rather than a duplication.
- [ ] The import is throttled and isolated — bounded batches, no giant transactions, no full-file memory loads, run off the request-serving path — so it does not lock or degrade the production database.
- [ ] Schema-mismatch handling is explicit: parsing by name where possible, pinning to a contract or version, and deciding deliberately how extra, missing, or renamed columns are treated.
- [ ] The import is observable: received/passed/quarantined/inserted counts reconcile, rejection-rate and volume anomalies alert, and the rejection log records the row, value, and reason so it is actionable.
- [ ] The design accounts for late, missing, duplicate, corrected, and hazardous delivery — absence detection, idempotent re-runs, bounded field lengths, and hardened parsing of untrusted content.
