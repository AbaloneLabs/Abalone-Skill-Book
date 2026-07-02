---
name: insurance_vs_indemnity.md
description: Use when the agent is structuring risk transfer between insurance and contractual indemnity, drafting additional insured endorsements, waivers of subrogation, primary and non-contributing clauses, deductible and retention interaction, other-insurance provisions, or coordinating coverage with indemnity obligations in commercial, construction, or M&A agreements, and must avoid gaps, overlaps, and subrogation conflicts.
---

# Insurance vs. Indemnity

Insurance and contractual indemnity are the two principal mechanisms for transferring risk, and in sophisticated transactions they operate together. Insurance spreads risk across a pool of insureds for a premium; indemnity reallocates risk between two specific parties by contract. The two systems interact constantly — an indemnity is often backed by insurance, an insurer may step into the shoes of an indemnitor through subrogation, and a coverage gap on one side can create an unfunded indemnity on the other. When the interaction is mishandled, the parties discover too late that the protection they thought they had is riddled with gaps, overlaps, or outright conflicts.

Agents frequently treat insurance and indemnity as independent silos: the contract team drafts the indemnity, the insurance team places the coverage, and nobody reconciles them. The result is classic failure modes — an indemnity that exceeds the indemnitor's policy limits, an additional insured endorsement that excludes the very claim that triggered the indemnity, a waiver of subrogation that is unenforceable because it was signed after the loss, or two policies both claiming the other is primary. This skill addresses the coordination points that determine whether the combined risk transfer actually works.

This skill is risk-structuring guidance, not legal advice or an insurance coverage opinion. Policy forms, endorsement enforceability, and subrogation rules vary by jurisdiction and insurer. Confirm actual policy language and recommend qualified coverage counsel and a licensed broker for placement decisions.

## Core Rules

### Map the indemnity obligation against available insurance before finalizing either

Before an indemnity clause is signed, confirm that insurance exists (or can be placed) to fund it. The analysis has several steps:

- **Identify the indemnity triggers** — what claims, losses, or liabilities does the indemnitor assume?
- **Match to policy coverage** — which of the indemnitor's policies (CGL, professional liability, D&O, cyber, environmental, auto) respond to each trigger?
- **Check limits vs. indemnity exposure** — do the policy limits cover the reasonably foreseeable loss, including defense costs? An indemnity capped above the available insurance is partly an unsecured promise.
- **Verify the indemnitor can actually place and afford the coverage** — an indemnity backed by unobtainable insurance is illusory.
- **Confirm the indemnitee is an additional insured where intended** — so it can claim directly rather than depending on the indemnitor to tender.

A strong process finalizes the indemnity scope and the insurance program together. Drafting the indemnity first and "letting insurance figure it out later" is a common source of unfunded obligations.

### Use additional insured endorsements to give direct rights, but verify their scope

An additional insured (AI) endorsement extends the indemnitor's policy to cover the indemnitee for covered claims, giving the indemnitee direct rights against the insurer rather than relying on the indemnitor to tender. But AI endorsements have load-bearing limitations:

- **Coverage scope** — many AI endorsements cover liability arising out of the indemnitor's ongoing operations or completed work, not the indemnitee's independent negligence. Confirm the endorsement matches the indemnity scope.
- **Form version** — older broad-form endorsements (e.g., ISO CG 20 10 07) are much broader than current editions; insurers often resist the broader forms. Confirm the actual form attached.
- **Scheduled vs. blanket** — a scheduled endorsement names the specific indemnitee; a blanket endorsement covers any party required by written contract. Blanket is convenient but can create ambiguity about who is covered and when.
- **Limits sharing** — the AI typically shares the indemnitor's policy limits; a claim by the indemnitee can exhaust limits the indemnitor needs for itself. Consider separate limits or an excess layer for the AI.
- **Notice and cooperation** — the AI must comply with policy notice conditions or risk losing coverage.

Do not assume an AI endorsement replicates the indemnity. Read the endorsement against the indemnity scope and confirm the gaps.

### Address subrogation explicitly with waivers

When an insurer pays a loss, it inherits the insured's right to pursue responsible third parties (subrogation). In a contractual relationship where one party indemnifies the other, subrogation can create the perverse result of the insurer suing the indemnitee's own counterparty (or the indemnitee itself) after paying a claim. Manage this with waivers of subrogation:

- **Mutual waivers** — each party waives subrogation against the other for covered losses, commonly required in construction (often mandated by statute or the contract form) and prudent in joint ventures.
- **Timing** — a waiver of subrogation must generally be in place before the loss occurs to be enforceable in many jurisdictions; a post-loss waiver may be void as against public policy or the insurer's vested rights.
- **Policy endorsement** — the insurer must consent to or be bound by the waiver; confirm the policy contains a waiver of subrogation endorsement or that the insurer has approved the contractual waiver.
- **Scope** — confirm the waiver covers the intended parties (affiliates, contractors, subcontractors) and the intended perils.

A missing or late waiver of subrogation can result in the insurer recovering from the very party the contract intended to protect, defeating the risk allocation.

### Resolve primary vs. excess / other-insurance ordering

When both parties are insured and a loss occurs, each policy's "other insurance" clause determines which pays first. Without coordination, both insurers may deny or delay. Address this contractually:

- **Primary and non-contributing** — the indemnitor's policy should be primary and non-contributing, meaning it pays first and the indemnitee's coverage is excess and not obligated to contribute until the primary limits are exhausted.
- **Other-insurance clause consistency** — confirm both policies' other-insurance clauses are compatible; conflicting "excess" clauses on both sides create gridlock.
- **Contractual requirement** — require the indemnitor to obtain an endorsement making its coverage primary over the indemnitee's available insurance.
- **Excess / umbrella layering** — confirm how excess and umbrella policies sit above the primary and whether they drop down if the primary is exhausted or insolvent.

Uncoordinated other-insurance provisions are a leading cause of coverage litigation; resolving them in the contract and endorsements is far cheaper.

### Coordinate deductibles, retentions, and self-insured retentions with indemnity baskets

The indemnity basket/deductible and the insurance deductible/retention interact and should be aligned:

- **Who bears the deductible?** If the indemnitor's policy has a deductible, the indemnitor typically bears it. Confirm the indemnity does not require the indemnitee to fund it.
- **RWI retention vs. seller basket** — in M&A, the representations and warranties insurance retention often mirrors what the seller basket would have been; confirm alignment so neither party bears an unintended gap.
- **Self-insured retention (SIR)** — an SIR requires the insured to pay and defend up to a threshold before the insurer engages; this affects who controls defense and whether the indemnitee is protected during the SIR layer. Confirm the indemnitor can fund the SIR.
- **Defense within limits vs. outside** — whether defense costs erode limits affects how much indemnity is actually available; coordinate with the indemnity cap.

### Watch for coverage exclusions that defeat the indemnity

Even with insurance in place, exclusions can eliminate coverage for the exact claims the indemnity is meant to address. Common traps:

- **Contractual liability exclusion** — most policies exclude liability assumed under a contract, with a carve-back for liability that would exist without the contract. Confirm the indemnity falls within the carve-back or obtain a specific endorsement.
- **Professional liability / professional services exclusion** — a CGL policy typically excludes professional services; if the indemnity covers professional negligence, a separate professional liability policy is required.
- **Cyber / data breach exclusions** — increasingly common; a data-breach indemnity backed only by a CGL may be unfunded.
- **Pollution / environmental exclusions** — absolute pollution exclusions can void coverage for environmental indemnities.
- **Insured vs. insured exclusion** — relevant in D&O and joint venture contexts; confirm the indemnity is not barred.
- **Known loss / prior acts** — confirm the claim arises from acts within the policy period and not a known loss at binding.

Map each indemnity trigger against the corresponding policy exclusions and obtain endorsements or separate policies to fill gaps.

### Decide whether indemnity or insurance is the primary remedy

In some structures, the parties intend insurance to be the primary remedy and indemnity to be a backstop; in others, indemnity is primary and insurance merely funds it. State the intended order:

- **Insurance-primary** — the indemnitee must first claim under available insurance before seeking indemnity; common where the indemnitee has its own robust coverage.
- **Indemnity-primary** — the indemnitor must perform regardless of available insurance; the indemnitee is not obligated to claim under its own policy first.
- **No double recovery** — regardless of order, prevent the indemnitee from collecting twice; coordinate subrogation and waiver rights.

Leaving the order unstated invites disputes about whether the indemnitee must exhaust its own coverage first.

## Common Traps

- **Drafting the indemnity without confirming insurability.** An indemnity that exceeds available limits or covers an uninsurable risk (e.g., punitive damages, known losses) is an unsecured promise.
- **Assuming an additional insured endorsement equals the indemnity.** AI endorsements are narrower than the indemnity in scope, form, and limits; read the actual endorsement.
- **A late or missing waiver of subrogation.** This allows the insurer to pursue the counterparty after paying a loss, defeating the risk allocation. Put waivers in place before the loss and confirm the policy endorses them.
- **Conflicting other-insurance clauses.** Both policies claiming to be excess creates gridlock and litigation; contractually require primary and non-contributing coverage.
- **Overlooking the contractual liability exclusion.** Most general liability policies exclude assumed contractual liability; without a carve-back or endorsement, the indemnity is unfunded.
- **Mismatched RWI retention and seller basket.** A gap or overlap between the policy retention and the seller indemnity basket creates an unfunded layer or double recovery.
- **Ignoring an SIR's effect on defense control.** With a large self-insured retention, the insured controls and funds defense up to the threshold, which can leave the indemnitee exposed if the indemnitor is undercapitalized.
- **Assuming defense costs are outside limits.** If defense erodes limits, the indemnity cap may be exhausted by defense alone. Confirm the policy structure.
- **Forgetting that insurance does not cover intentional misconduct or fraud.** An indemnity for fraud is effectively uninsured; the indemnitee must rely on the indemnitor's solvency.
- **Treating insurance and indemnity as separate workstreams.** The contract team and insurance team must reconcile scope, limits, exclusions, and ordering together, or gaps and conflicts emerge at claim time.

## Self-Check

- Has the indemnity scope been mapped against available insurance, confirming that each trigger has corresponding coverage with adequate limits?
- Are additional insured endorsements obtained, and has the actual endorsement form been read against the indemnity scope to confirm coverage (including form version and limits sharing)?
- Are waivers of subrogation in place before any loss, endorsed by the insurer, and scoped to the intended parties and perils?
- Is the other-insurance ordering resolved contractually (primary and non-contributing for the indemnitor's policy), with consistent policy language?
- Are deductibles, retentions, and SIRs allocated to the intended party, and is defense control during the retention layer addressed?
- Have policy exclusions (contractual liability, professional services, cyber, pollution, insured vs. insured, known loss) been checked against each indemnity trigger, with endorsements or separate policies filling gaps?
- Is the intended order of remedies (insurance-primary vs. indemnity-primary) stated, with no double recovery?
- If RWI is involved, is the retention aligned with the seller basket and are exclusions reconciled?
- Has the agent confirmed that fraud, punitive damages, and intentional misconduct are effectively uninsured and that the indemnitee is not relying on coverage that will not respond?
- Has the agent flagged that policy forms and endorsement enforceability are jurisdiction- and insurer-specific, and recommended coverage counsel and a licensed broker for placement?
