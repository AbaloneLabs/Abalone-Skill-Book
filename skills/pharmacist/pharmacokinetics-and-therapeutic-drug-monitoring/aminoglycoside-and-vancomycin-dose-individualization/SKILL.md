---
name: aminoglycoside-and-vancomycin-dose-individualization.md
description: Use when the agent is dosing or monitoring vancomycin or aminoglycosides (gentamicin, tobramycin, amikacin), calculating a pharmacokinetic regimen from trough or peak levels, adjusting for renal function or dialysis, or deciding when to redraw levels after a dose change.
---

# Aminoglycoside and Vancomycin Dose Individualization

Therapeutic drug monitoring (TDM) for vancomycin and the aminoglycosides exists because these drugs have a narrow therapeutic window: the difference between a dose that is effective and a dose that is toxic is small, and that difference is determined by the patient's individual pharmacokinetics, not by a population average. The danger is that a pharmacist treats TDM as a number-matching exercise — see a trough, look up a target range, adjust the dose — without engaging the clinical context that determines whether the level is even valid, whether the timing was correct, and whether the number reflects the patient's actual clearance or an artifact of the sampling. When TDM becomes mechanical, the pharmacist either chases meaningless numbers or, worse, adjusts a regimen based on a level that was drawn at the wrong time and thereby moves the patient further from safe exposure.

## Core Rules

### Confirm the Level Is Valid Before Acting on It

A drug level is only meaningful if it was drawn at the correct time relative to the dose. A vancomycin trough drawn two hours before the next dose is not a trough — it is an unknown midpoint concentration, and treating it as a trough will overestimate the patient's clearance and produce an inappropriately high dose. Before interpreting any level, the pharmacist must reconstruct the actual dosing and sampling timeline: when was the last dose given, when was the level drawn, and does the recorded time match the laboratory draw time? When the timing is wrong or undocumented, the correct action is to reject the level and redraw it under controlled conditions, not to back-calculate a number from unreliable input. A level drawn from the same line that delivered the drug is contaminated and invalid; the pharmacist must verify that the sample was drawn from a separate site or after adequate line flushing.

### Base the Regimen on the Patient's Actual Clearance, Not a Population Estimate

Vancomycin and aminoglycoside dosing depends on the patient's elimination rate, which is driven primarily by renal function. The pharmacist must calculate the patient's creatinine clearance using a validated equation (Cockcroft-Gault for most adults), and must recognize that the serum creatinine value used in that equation can lag behind actual renal function — a patient whose kidneys are failing today may still have a "normal" creatinine from yesterday. For hemodialysis patients, the pharmacist must know the dialysis schedule, the dialyzer's drug clearance, and whether supplemental ("rebound") dosing is needed after the session. When a patient's renal function is changing rapidly — acute kidney injury, recovering AKI, or new nephrotoxin exposure — a regimen calculated from a single creatinine value is already outdated, and the pharmacist must plan for more frequent monitoring and dose review, not a set-and-forget regimen.

### Use the Right Pharmacokinetic Model for the Drug and the Setting

Vancomycin is now monitored by area under the curve (AUC) rather than trough-only targeting, because trough is an unreliable surrogate for total exposure. The pharmacist must understand when to use Bayesian dosing software versus manual calculation, and must recognize the assumptions each method carries: Bayesian methods depend on the quality of the population model and the number of levels available, while manual methods assume linear pharmacokinetics that may not hold in critically ill patients. For aminoglycosides, the pharmacist must decide between traditional dosing (multiple daily levels) and extended-interval dosing (single random level with a nomogram), and must recognize that extended-interval dosing is inappropriate for some patients — those with endocarditis, ascites, burns, or significant renal impairment — where the distribution and clearance assumptions break down.

### Adjust for the Clinical State That Changes Pharmacokinetics

Critically ill patients do not behave like the population averages in dosing references. The pharmacist must account for the factors that alter volume of distribution and clearance in the ICU: large-volume resuscitation increases volume of distribution and lowers peak concentrations; augmented renal clearance (common in young trauma and sepsis patients) clears the drug faster than predicted and produces subtherapeutic levels; burns change both volume and clearance over time. For aminoglycosides, the pharmacist must target the peak concentration that achieves efficacy (the Cmax/MIC ratio) while respecting the trough window that reduces nephrotoxicity — and must recognize that extending the interval, rather than reducing the dose, is often the safer way to achieve a low trough while preserving the peak.

### Define the Stop Condition, Not Just the Next Dose

TDM is not a perpetual process. The pharmacist must define when monitoring will end: when the patient is stable and on a predictable regimen, when the infection has resolved, when the drug is being de-escalated to an oral agent, or when the patient is being transitioned to outpatient care. Continuing to draw levels on a stable, long-term regimen (unless the clinical state changes) exposes the patient to unnecessary phlebotomy and the team to alert fatigue. Conversely, the pharmacist must escalate monitoring when the clinical state changes — new nephrotoxin, new renal impairment, significant weight change, or change in dialysis status — because a previously stable regimen may suddenly become toxic or subtherapeutic.

### Respect Scope and Escalation Boundaries

Know where the agent's authority and competence end. When the question requires a license, a specialist's judgment, a final approval, or expertise the agent does not hold, the correct action is to escalate rather than to produce a confident answer that overreaches. Scope discipline protects the recipient from harm caused by an unqualified conclusion and protects the agent from liability. State explicitly when the output is advisory and must be confirmed by the qualified person.

## Common Traps

### Treating a Trough Target as the Goal Rather Than a Surrogate for Exposure

The historical practice of targeting a vancomycin trough of 15 to 20 mg/L produced a generation of pharmacists who treat the trough number as the therapeutic goal itself. The mechanism of the trap is that the trough is easy to measure and easy to act on, so it displaces the actual goal — adequate AUC exposure — in the pharmacist's mind. The harm is twofold: a patient can have a "therapeutic" trough and still be underexposed (if clearance is high and the interval is long), or can have a "high" trough that is actually appropriate (if the dosing interval is short). When the pharmacist optimizes the number rather than the exposure, the patient receives unnecessary dose increases, escalating nephrotoxicity risk, or unnecessary dose reductions that cause treatment failure. The false signal is that a level within the target range guarantees efficacy and safety, when in fact the range is a crude approximation that must be interpreted in the context of the full concentration-time profile.

### Drawing Levels Without Verifying the Timeline

A pharmacist receives a vancomycin trough of 25 mg/L, flags it as high, and recommends halving the dose. The trap is that the "trough" was drawn 30 minutes after the dose infused, not before it — the number reflects a peak, not a trough, and the patient's actual trough may be perfectly therapeutic. The mechanism is that the laboratory reports a number and the pharmacist acts on the number without reconstructing the dosing and sampling timeline. The harm is that the patient's regimen is adjusted based on a level that does not represent the pharmacokinetic parameter it claims to represent, and the adjustment moves the patient in the wrong direction — reducing a dose that was already too low, or increasing a dose that was already too high. The false signal is that a laboratory value with a timestamp is authoritative, when in fact the timestamp may be wrong, the draw may have been from the wrong line, or the dose may have been given late.

### Applying Population Averages to a Patient Whose Clearance Has Changed

A patient was started on vancomycin three days ago when their creatinine clearance was 80 mL/min. Today their creatinine has doubled. The pharmacist, reviewing the trough, applies the original clearance estimate and calculates a dose that continues to assume normal renal function. The mechanism is that the pharmacist anchors to the initial pharmacokinetic calculation and does not update it as the clinical state evolves. The harm is that the drug accumulates in a patient whose clearance has fallen, the trough climbs silently, and nephrotoxicity develops — sometimes compounding the very renal injury that caused the accumulation. The false signal is that a regimen that was correct at initiation remains correct, when in fact vancomycin and aminoglycoside regimens must be reassessed every time renal function changes.

### Extended-Interval Dosing Applied Where the Assumptions Do Not Hold

Extended-interval aminoglycoside dosing relies on a high peak followed by a drug-free interval that allows renal tubular uptake to recover. The trap is applying this approach to a patient for whom the assumptions fail: a patient with endocarditis needs sustained concentration, not a high peak; a patient with ascites or large burns has an expanded volume of distribution that prevents the high peak; a patient with significant renal impairment never clears the drug during the extended interval and accumulates toxicity. The mechanism is that the nomogram feels like a shortcut — one level, one lookup, one dose — and the pharmacist applies it without checking the exclusion criteria. The harm is that the patient receives a regimen that is either ineffective (no adequate peak) or toxic (no drug-free interval), and the single-level monitoring cannot detect the failure because it assumes the model is correct.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

### Treating a confident conclusion as a substitute for evidence

A well-written, confident-sounding conclusion is accepted as proof. The trap is that the tone of certainty masks unstated assumptions, missing verification, or unresolved uncertainty, and the confident framing prevents anyone from asking the question that would expose the gap.

## Self-Check

- Did I reconstruct the actual dosing and sampling timeline before interpreting the level, or did I act on the laboratory number without verifying when the dose was given and when the level was drawn?
- Did I calculate the patient's creatinine clearance from current labs, and did I consider whether renal function is changing rapidly enough that a single value is already outdated?
- Did I use AUC-based monitoring for vancomycin where appropriate, or did I treat the trough as the therapeutic goal rather than a surrogate for total exposure?
- For aminoglycosides, did I confirm the patient meets the criteria for extended-interval dosing, or did I apply the nomogram to a patient whose volume of distribution, indication, or renal function excludes that approach?
- Did I account for ICU-specific pharmacokinetic changes — augmented renal clearance, large-volume resuscitation, burns — that make population averages unreliable?
- Did I define when monitoring will end and when it must escalate, or am I drawing levels on autopilot without a stop condition or a trigger for reassessment?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
