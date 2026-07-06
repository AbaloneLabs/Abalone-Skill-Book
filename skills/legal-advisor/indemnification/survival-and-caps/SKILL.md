---
name: survival_and_caps.md
description: Use when the agent is drafting or negotiating survival periods, liability caps, baskets, deductibles, tipping baskets, sandbagging provisions, statutes of limitation interaction, tax and special indemnities, or limitation-of-liability mechanics in M&A, commercial, or transactional agreements, and must reconcile internal consistency and economic realism.
---

# Survival and Caps

In transactional and commercial agreements, the survival period and the liability cap (with its baskets, deductibles, and carve-outs) together determine whether the representations, warranties, and indemnities in the agreement are economically meaningful. A perfectly drafted rep is worthless if it expires before the claim can arise; an uncapped indemnity is catastrophic if the loss is large; and a cap that is too low relative to deal size can make the protection illusory. These provisions are the financial engine of the risk allocation, and they interact in non-obvious ways.

Agents often copy survival and cap language from a prior deal without modeling it against the actual transaction. The result is internally inconsistent terms (a survival period longer than the limitations period, a basket larger than the cap, carve-outs that swallow the cap), economically unrealistic thresholds, or silent gaps where a category of claim has no survival or no limit at all. Because these provisions are negotiated in basis points and time periods, small drafting differences translate into large money outcomes.

This skill is transactional guidance, not legal advice. Statutes of limitation, tax survival rules, and anti-sandbagging enforceability vary by jurisdiction and deal structure. Confirm governing law and recommend qualified counsel for non-standard structures.

## Core Rules

### Set survival periods by claim category, not as a single blanket

A single survival period for all reps is usually wrong. Different risks materialize on different timelines, and a one-size survival period either expires too early for latent claims or runs too long for routine reps. Categorize and set survival deliberately:

- **General business reps** — typically survive to the shorter of the contractual period or the general statute of limitations (often 2–3 years), reflecting that these claims are discoverable and litigable within a normal window.
- **Fundamental reps** — organization, authority, capitalization, title to shares, no broker — often survive longer (e.g., indefinitely or to the longest limitations period) because they go to the validity of the deal itself.
- **Tax reps** — frequently survive until the expiration of the applicable tax statute of limitations plus extensions, which can be 3–7 years and is often the longest survival in the deal.
- **IP, data privacy, environmental, employee benefits, and compliance reps** — often given extended survival to match the latent risk and longer statutory periods.
- **Fraud and willful misrepresentation** — generally survive indefinitely and are typically non-waivable by contract in many jurisdictions.

State each category and its period explicitly. A blanket "all reps survive for 3 years" under-protects tax and IP claims and is a common drafting failure.

### Reconcile survival with the applicable statute of limitations

Survival cannot create a claim that the statute of limitations would bar, and a survival period longer than the limitations period is partly illusory. Reconcile them:

- Confirm the general contractual limitations period under governing law for breach of contract and misrepresentation.
- Set survival so that claims can actually be brought within the limitations window (i.e., survival should generally be shorter than or equal to the limitations period, or the survival should expressly extend the limitations period by agreement if enforceable).
- For tax, confirm the actual limitations period including extensions and any tolling for voluntary disclosures or audits.
- Consider a tolling or standstill agreement that pauses the limitations clock during the claims process, which is common in sophisticated deals.

A survival period that expires before a claim can reasonably be discovered (e.g., a 12-month survival for a rep about a latent environmental condition) is a known trap; extend it or add a discovery trigger.

### Structure the cap and basket with economic realism

The cap and basket should be modeled against deal size, expected loss, and the cost of bringing a claim:

- **Cap** — typically a percentage of deal value (commonly 10–30% for middle-market M&A, 100% for fundamental reps and fraud). Confirm the cap is high enough to make the protection meaningful but not so high as to be uninsured or uncollateralized.
- **Basket** — the threshold the buyer must absorb before recovery. Two structures:
  - **True deductible / first-dollar basket** — the buyer bears the basket amount and recovers only the excess. Lower recovery.
  - **Tipping basket** — once the threshold is exceeded, the buyer recovers from dollar one. Higher recovery; favored by buyers.
- **Mini-baskets** — small per-rep deductibles that prevent nuisance claims on individual reps without imposing a global threshold.
- **Relationship to cap** — a basket that approaches the cap makes the indemnity nearly illusory; a basket far below the cap provides real protection. Model the ratio.

Drafters should run the numbers: at what claim size does recovery begin, and what is the maximum recovery? If the answer is unrealistic, the economics are wrong.

### Identify and negotiate the carve-outs from the cap

Caps almost always have carve-outs for categories where full recovery is intended. Standard carve-outs include:

- **Fraud and willful misrepresentation** — universally carved out and often non-waivable.
- **Fundamental reps** — organization, authority, capitalization, title — typically subject to the full purchase price or an elevated cap.
- **Tax indemnities** — often carved out to the full tax liability plus interest and penalties.
- **IP infringement and ownership** — frequently carved out given the potentially catastrophic exposure.
- **Breach of confidentiality and non-compete** — often carved out due to difficulty of quantifying harm.
- **Specific indemnities for known liabilities** — disclosed risks (e.g., pending litigation, specific tax positions) usually carry separate, often uncapped, indemnities.

Confirm each carve-out is stated and that the carve-outs do not collectively swallow the cap to the point that the cap is meaningless — or, if that is intended, say so.

### Address sandbagging in both directions

Sandbagging refers to whether a buyer who knew of a breach at closing can still claim indemnity for it. Two approaches:

- **Pro-sandbagging (buyer-favorable)** — buyer can recover for a breached rep regardless of knowledge. Default in some jurisdictions (e.g., Delaware).
- **Anti-sandbagging (seller-favorable)** — buyer waives indemnity for matters it knew about at closing.

Draft the position explicitly rather than leaving it to default law, because the default varies and the outcome is significant. If anti-sandbagging, consider whether the buyer's knowledge is measured by actual knowledge, constructive knowledge, or the knowledge of specific individuals, and whether disclosure schedule items count as "knowledge."

### Handle special indemnities and escrows separately

Specific indemnities for identified risks (e.g., a known lawsuit, a particular tax exposure) are often structured outside the general cap and basket. Decide:

- Whether the special indemnity has its own cap (often the reasonably estimated exposure) or is uncapped.
- Whether it shares the general survival period or has its own.
- Whether it is funded by an escrow, holdback, or parent guarantee, and the release mechanics.
- Whether recovery under the special indemnity reduces the general cap (typically it does not).

### Coordinate with insurance (RWI) and limitation of liability

Representations and warranties insurance (RWI) has reshaped survival and cap economics:

- **RWI structure** — the policy often replaces seller indemnity for general reps, with the seller retaining a deductible-equivalent retention and a separate indemnity for fundamental reps and excluded matters.
- **Retention vs. basket** — the RWI retention often mirrors what the seller basket would have been; confirm the alignment.
- **Seller tail** — the seller's survival for non-RWI-covered matters may be shorter than the policy term; reconcile.
- **Subrogation** — the insurer may pursue the seller after paying the buyer; address seller awareness of this and any waiver.

In commercial (non-M&A) contracts, coordinate the cap with the indemnity and the limitation of liability clause so that the intended categories (indemnified third-party claims, IP, confidentiality) sit outside the general consequential-damages exclusion and cap.

## Common Traps

- **A blanket survival period for all reps.** This under-protects tax, IP, and latent-risk reps and over-protects routine ones. Categorize.
- **Survival longer than the statute of limitations.** The extra period is partly illusory because the claim is time-barred anyway. Reconcile or add a tolling agreement.
- **A basket larger than or close to the cap.** This makes the indemnity economically meaningless for realistic claim sizes. Model the ratio.
- **Confusing true deductible with tipping basket.** They produce very different recovery. State which structure applies.
- **Carve-outs that swallow the cap.** If every significant category is carved out, the cap is cosmetic and the seller has uncapped exposure; either accept that or narrow the carve-outs.
- **Leaving sandbagging to default law.** The default varies by jurisdiction and the outcome is significant; draft the position explicitly.
- **Ignoring RWI interaction.** A seller indemnity that duplicates RWI coverage wastes negotiating leverage; align retention, survival, and exclusions with the policy; **Setting mini-baskets that block all realistic claims.** Per-rep deductibles set too high prevent recovery for the very claims they are meant to cover
- **Forgetting fraud is typically non-waivable.** Attempting to cap or waive fraud indemnity is often unenforceable; do not rely on it; **Mismatched special indemnity and escrow release dates.** If the escrow releases before the special indemnity survival expires, the buyer has an unfunded claim. Align the dates

## Self-Check

- Are survival periods set by claim category (general, fundamental, tax, IP, fraud) rather than as a single blanket, and is each period reconciled with the applicable statute of limitations?
- Is the cap stated, and are the carve-outs (fraud, fundamental reps, tax, IP, specific indemnities) enumerated without collectively swallowing the cap unintentionally?
- Is the basket structure explicit (true deductible vs. tipping), and has the drafter modeled the ratio of basket to cap and to expected claim sizes?
- Is the sandbagging position drafted explicitly, with knowledge defined, rather than left to default law?
- Are special indemnities for known risks structured with their own cap, survival, and funding (escrow/holdback/guarantee), with release dates aligned to survival?
- If RWI is in play, are the seller retention, survival, and exclusions aligned with the policy terms, and is subrogation addressed?
- Are mini-baskets (if any) set at a level that does not block realistic claims?
- Has the drafter confirmed that fraud and willful misrepresentation carve-outs are not waivable in a way that would void the intended protection?
- Are all time periods, amounts, and categories internally consistent with no category left without a survival or a limit?
- Has the agent flagged jurisdiction-specific enforceability (tax survival, anti-sandbagging, limitations periods) and recommended counsel review for non-standard structures?
