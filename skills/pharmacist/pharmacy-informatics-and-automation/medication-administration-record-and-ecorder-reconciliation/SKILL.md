---
name: medication-administration-record-and-ecorder-reconciliation.md
description: Use when the agent is reconciling the medication administration record against the electronic order set, resolving MAR and eMAR discrepancies, managing hold and discontinued orders, correcting timing or rate mismatches, verifying that documented administrations match active orders, or auditing the closed loop between prescribing and administration.
---

# Medication Administration Record and eOrder Reconciliation

The medication administration record (MAR) and the electronic order (eOrder) set are supposed to be two views of one truth: what was ordered should match what is being administered and what is documented as given. In reality, they drift apart constantly, and that drift is where serious medication errors hide. Orders are placed, changed, held, or discontinued but the MAR lags or carries a stale version; a dose is given against an order that was already stopped; a rate change in the eOrder never reaches the infusion pump; a verbal order is entered but never signed; a patient-specific dose is documented without the order ever existing. The judgment problem is that reconciliation is treated as a clerical task when it is a clinical safety function: the pharmacist is the only role that sees both the order intent and the administration reality, and the discrepancies that look minor on screen (a one-hour timing mismatch, a held dose still showing as active, a duplicated order) are exactly the ones that produce double dosing, missed therapy, or wrong-rate infusion harm. The work is to close the loop deliberately, not to assume the system stays in sync.

## Core Rules

### Reconcile Order Intent Against MAR Reality, Not Just Field-by-Field Matching

True reconciliation asks whether what the prescriber intended matches what the nurse is giving and documenting, not merely whether the drug name and dose fields are identical. Check that the active order reflects the current clinical intent (the right drug, dose, route, rate, frequency, and duration for the current indication), that the MAR reflects the active order, and that documented administrations reflect what was actually given. Look for the mismatches that field-matching misses: an order that was verbally changed at the bedside but never updated, a weight-based dose where the weight in the system is stale, a renal dose adjustment that was communicated but not reordered, a hold condition (low platelets, low potassium, hypotension) that is no longer being enforced.

### Resolve Discrepancies Actively and Document the Resolution

When a discrepancy is found, resolve it rather than noting it. Contact the prescriber to clarify intent, discontinue stale or duplicate orders, correct the MAR where a documented administration is wrong, and ensure the next dose reflects the reconciled order. Document the discrepancy, the resolution, and the communication so that the next handoff does not re-introduce the error. A discrepancy left in place because it is "probably fine" is a latent error waiting for a distraction or a new clinician to convert it into harm. Reconciliation that does not end in a documented resolution is incomplete.

### Manage Hold, Discontinue, and Reorder Transitions Explicitly

The most error-prone moments are transitions in order status: an order placed on hold that continues to show as active, a discontinued order whose MAR entry persists, a reordered medication that creates a duplicate active order, or a temporary hold that no one remembers to lift. When an order is held, verify the MAR reflects the hold and that the nurse knows the hold condition and the threshold to resume. When an order is discontinued, verify it is removed from the active administration workflow and that any related infusion or pump is stopped. When an order is reordered (especially after a transfer or a system downtime), check for duplication. These transitions are where double dosing and missed doses concentrate.

### Verify Infusion Rate, Concentration, and Pump Programming Against the eOrder

For infusions, reconciliation must extend to the physical setup: the drug concentration in the bag, the rate programmed into the pump, and the units in the order must all be consistent. A common and dangerous discrepancy is a unit mismatch (milligrams per hour versus milliliters per hour, or micrograms versus milligrams), or a concentration change (the pharmacy compounds a different concentration than the order assumes) that silently doubles or halves the dose at the pump. Use smart-pump interoperability where available to push the order directly to the pump, and where it is not available, reconcile the pump programming against the order manually. Never assume the rate on the MAR equals the rate in the pump.

### Reconcile at Every Transition of Care and After Every Downtime

Reconciliation is most valuable at the transitions where errors concentrate: admission, transfer between units and levels of care, and discharge. At each transition, compare the home medication list, the prior unit's active orders, the new unit's orders, and the MAR, and resolve discrepancies before the first dose on the new unit. After any system downtime (planned or unplanned), reconcile all orders entered on paper or in downtime mode against the electronic record, because downtime entries are a major source of duplicate, missing, and mismatched orders. Treat the post-downtime reconciliation as a mandatory safety step, not a cleanup task.

### Use the Closed-Loop Audit to Find Systemic Discrepancies

Beyond individual patient reconciliation, audit the closed loop (prescribe, transcribe, dispense, administer, document) for patterns: orders that are consistently administered late, drugs with frequent holds, units with high discrepancy rates, orders that are frequently overridden. These patterns reveal systemic problems (a confusing order sentence, a workflow bottleneck, a training gap) that no individual reconciliation will fix. Feed these findings back into order set design, CDS rules, and workflow changes so that the discrepancies stop being generated rather than merely being caught.

## Common Traps

### Treating MAR-eOrder Matching as a Clerical Check Rather Than Clinical Review

The pharmacist scans the MAR and the order, sees the drug and dose match, and signs off, missing that the order is clinically wrong for the current patient state (the renal function has declined, the indication has resolved, the hold condition is met). The mechanism is that field-matching feels like thoroughness, and the false signal is that matching fields mean a safe order. The harm is that a clinically inappropriate but technically matching order is administered. Reconciliation must include clinical judgment about whether the order is right for the patient now, not just whether the fields agree.

### Leaving a Discrepancy in Place Because It Seems Minor

A small timing mismatch or a held dose still showing as active is noted but not resolved, because the next dose is hours away and it seems unimportant. The mechanism is that minor discrepancies rarely cause immediate harm, so they feel safe to defer, and the false signal is that no harm today means no harm. The harm is that the discrepancy is forgotten, a new clinician assumes the MAR is correct, and the error propagates into a double dose or a missed dose. Every discrepancy must be resolved and documented, not deferred.

### Assuming the Pump Rate Equals the MAR Rate for Infusions

The MAR shows the intended rate and the pharmacist assumes the pump is programmed to match, but the concentration in the bag differs from what the order assumes, or the nurse programmed the pump in different units, so the patient receives double or half the intended dose. The mechanism is the assumption that the documentation reflects the physical setup, and the false signal is a consistent-looking MAR. The harm is a serious infusion error, especially with high-alert medications like heparin, insulin, and vasoactives. Infusion reconciliation must include the concentration and the pump programming.

### Failing to Reconcile After a System Downtime

After a downtime, paper or downtime-mode orders are entered into the electronic system, but duplicates, omissions, and mismatches are introduced and never cleaned up, so the patient ends up with two active orders for the same drug or a missing critical order. The mechanism is that the system is back up so the crisis feels over, and the false signal is that electronic entry equals reconciliation. The harm is double dosing or missed therapy discovered only when an adverse event occurs. Post-downtime reconciliation is a mandatory, explicit step.

### Ignoring Duplicate Orders Created at Transfer

At transfer, the receiving team enters new orders without discontinuing the sending unit's orders, so the patient has two active orders for the same medication, and the MAR may reflect both, leading to double administration. The mechanism is that transfer is busy and duplication is easy, and the false signal is that the new orders are the correct ones. The harm is double dosing, especially with high-alert drugs. Transfer reconciliation must explicitly search for and discontinue duplicate orders.

### Overlooking the edge case or exception

The typical routine medication is reconciled well, but the exception (a complex titratable infusion, a clinical trial drug, a patient-specific compounded preparation, an off-label pediatric weight-based order, or a medication with a narrow therapeutic window) is skipped. The trap is that the standard reconciliation path is well-handled while the exception silently produces harm because the boundary condition was never tested.

## Self-Check

- Did I reconcile clinical intent against MAR reality (right drug, dose, route, rate, frequency, duration for the current indication), not just field-by-field matching?
- Did I actively resolve every discrepancy I found, contact the prescriber where needed, and document the resolution and communication?
- Did I verify hold, discontinue, and reorder transitions explicitly, ensuring held orders reflect hold conditions, discontinued orders are removed from administration, and reorders do not duplicate?
- For every infusion, did I reconcile the concentration, the pump programming, and the units against the eOrder, rather than assuming the MAR rate equals the pump rate?
- Did I reconcile at every transition of care and after any downtime, comparing home, prior, and new orders before the first dose?
- Did I audit the closed loop for systemic discrepancy patterns and feed findings back into order set and workflow design?
- Did I confirm that documented administrations reflect what was actually given, and correct the MAR where documentation is wrong?
- Did I stay within scope, escalating clinical order changes to the prescriber and documenting the pharmacist's reconciliation role clearly?
