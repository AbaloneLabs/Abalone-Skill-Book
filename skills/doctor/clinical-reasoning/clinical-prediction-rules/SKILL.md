---
name: clinical-prediction-rules.md
description: Use when the agent is applying a validated clinical prediction rule or score (Wells for PE or DVT, CURB-65 for pneumonia, CHA2DS2-VASc for stroke risk, qSOFA or SOFA for sepsis, HEART for chest pain, Ottawa ankle or knee rules, Centor for streptococcal pharyngitis), deciding whether a rule applies to a given patient, interpreting a score in context, or documenting the reasoning behind a score-driven decision.
---

# Clinical Prediction Rules

Validated clinical prediction rules (CPRs) are among the most useful and most misused tools in medicine. A well-derived rule — Wells for pulmonary embolism, CURB-65 for pneumonia mortality, CHA2DS2-VASc for stroke risk, qSOFA for sepsis, HEART for chest pain, the Ottawa ankle and knee rules, Centor for streptococcal pharyngitis — distills large datasets into a small set of variables that estimate a probability or a threshold for action. Used well, they standardize decisions, reduce unnecessary imaging, protect against both over- and under-treatment, and make reasoning auditable. Used badly, they become a substitute for thinking: a number is generated, judgment is suspended, and the patient's individual context is ignored. The recurring failure is to treat a score as an answer rather than as one input into a decision that still belongs to the clinician and the patient. This skill covers the disciplined application of clinical prediction rules: knowing which rule fits the clinical question, confirming the patient is within the population the rule was validated in, scoring it correctly, interpreting the result as a probability or action threshold rather than a verdict, recognizing when the rule disagrees with clinical judgment and what to do about it, and documenting the decision so the reasoning survives. The clinician's job is to use the rule to discipline intuition, not to outsource judgment to arithmetic.

## Core Rules

### Match the Rule to the Clinical Question, Not to the Patient's Label

Before applying any rule, confirm it answers the question you are actually asking. Wells for pulmonary embolism estimates pre-test probability of PE to guide imaging decisions; the revised Geneva score serves a similar purpose and is useful when the clinician wants an objective score that does not include the subjective "alternative diagnosis less likely" item. CURB-65 estimates 30-day mortality in community-acquired pneumonia to guide site-of-care decisions, while the Pneumonia Severity Index (PSI/PORT) is a more granular alternative that weights age and comorbidity heavily. CHA2DS2-VASc estimates annual stroke risk in atrial fibrillation to guide anticoagulation, and HAS-BLED estimates bleeding risk to temper it. qSOFA is a bedside screen for poor outcomes in suspected infection, while the full SOFA score requires labs and quantifies organ dysfunction. HEART stratifies 30-day major adverse cardiac events in chest pain to guide disposition. The Ottawa ankle and knee rules decide whether imaging is needed after an injury. Centor (or Centor/McIsaac) estimates the probability of group A streptococcal pharyngitis to guide testing and treatment. Selecting the wrong rule — applying a mortality score to a disposition question, or a stroke-risk score to an acute decision — produces a number that is precise and irrelevant. State the decision the score is meant to inform before computing it.

### Confirm the Patient Is in the Validated Population

A rule is only as good as the population it was derived and validated in, and applying it outside that population is a common source of error. Check whether the patient resembles the derivation cohort in age, comorbidity, setting (emergency vs inpatient vs primary care), and disease spectrum. Many rules were derived in adults and do not perform the same way in children (the Ottawa ankle rule has pediatric variants; adult CURB-65 is not validated in children). Some were derived in specific settings and may mis-calibrate elsewhere: qSOFA, for example, has limited sensitivity as a sole screen for sepsis and is not recommended as the only trigger for sepsis evaluation, despite its bedside convenience. Rules may underperform in elderly patients, in pregnant patients, in immunocompromised patients, or in populations with different disease prevalence. If the patient sits outside the validated population, use the score cautiously, weight clinical judgment more heavily, and document why the score's output is being overridden or modified.

### Score the Rule Correctly and Completely

A surprisingly large fraction of score-based errors come from mis-scoring: omitting a variable, misremembering a weight, or applying a simplified version that omits items. Use the full, current version of the rule. For CHA2DS2-VASc, recall that age contributes zero, one, or two points depending on the band, and that "vascular disease" includes prior MI, peripheral artery disease, and aortic plaque — not hypertension alone. For CURB-65, each of Confusion, Urea > 7 mmol/L, Respiratory rate, Blood pressure, and age 65 and over contributes one point. For HEART, each domain (History, ECG, Age, Risk factors, Troponin) is scored and the sum stratifies risk. For Wells PE, include the subjective "alternative diagnosis less likely than PE" item consciously, because it materially shifts the score. When a rule has been revised (revised Geneva, modified Centor/McIsaac), use the version that is current and locally endorsed. When in doubt about a component, look it up rather than guessing, because a one-point error can move a patient across a treatment threshold.

### Interpret the Score as a Probability or Threshold, Not a Diagnosis

A clinical prediction rule produces a probability or a risk category, not a diagnosis. A low-risk HEART score does not mean the patient is not having a cardiac event; it means the 30-day risk is low enough that early discharge with close follow-up is reasonable. A CURB-65 of 2 supports outpatient or brief inpatient management but does not override a patient who looks toxic or cannot manage oral medications at home. A low Wells score combined with a negative D-dimer safely reduces imaging; a high Wells score mandates imaging regardless of the D-dimer, because the post-test probability remains too high to dismiss. Understand the calibration of the rule — does a given score correspond to a 1 percent risk, a 5 percent risk, a 15 percent risk — and communicate that probability honestly. Avoid presenting a "low-risk" result as "ruled out," because the residual risk is real and the patient must understand the warning signs that warrant return.

### Reconcile the Score With Clinical Judgment, and Document the Reconciliation

The most important rule of using prediction rules is that they advise judgment; they do not replace it. When the score and your clinical assessment agree, the decision is straightforward. When they disagree, do not silently defer to either — make the disagreement explicit and reason through it. If the patient looks far sicker than the score suggests (CURB-65 low but the patient is septic-appearing and hypoxic), trust the clinical picture and escalate. If the score flags high risk but the patient looks well, investigate why — a missing data point, a variable inflated by chronic disease, or a genuinely high-risk presentation in a deceptively well-looking patient. Document both the score and the clinical reasoning: record the score, the probability or category it implies, the decision, and the rationale when judgment overrides or modifies the score. This documentation protects the patient, supports continuity, and provides medicolegal evidence of a thoughtful process.

### Know When a Rule Does Not Apply and When to Abandon It

Some clinical situations fall outside any useful rule, and forcing a score onto them is worse than not scoring. Rules assume the patient resembles the derivation cohort; when the presentation is atypical, when critical data are missing, when the patient is at the extremes of age or physiology, or when the stakes are very high, set the rule aside and reason from first principles. A rule that produces a nonsensical output in a clearly sick patient should be discarded, not defended. Similarly, do not apply a rule designed to reduce testing to a patient in whom you have already decided to test or treat — computing a low-risk score to justify not imaging a patient you are worried about is rationalization, not decision support.

### Use the Rule to Drive a Shared, Documented Decision

For rules that govern preference-sensitive or threshold decisions (CHA2DS2-VASc and anticoagulation, HEART and disposition, PSA-style decisions), use the score as the anchor for a shared decision making conversation. Present the absolute risk the score implies, the options (treat, do not treat, observe, test further), the benefits and harms of each, and the patient's values. A CHA2DS2-VASc that yields a stroke risk of, say, 4 percent per year is not self-executing; the decision to anticoagulate must weigh bleeding risk (HAS-BLED), fall risk, adherence, monitoring burden, and patient preference. Document the score, the conversation, and the chosen plan.

## Common Traps

### Applying the Wrong Rule to the Question

Using a mortality score for a disposition question, or a risk score for an acute diagnostic decision. Counter by stating the decision the score must inform before selecting the rule.

### Using a Rule Outside Its Validated Population

Applying an adult-derived score to a child, or an emergency-derived rule in a primary-care cohort, without checking calibration. Counter by confirming the patient resembles the derivation population and down-weighting the score if they do not.

### Mis-Scoring by Omitting or Misweighting Variables

Dropping a component or misremembering a weight, shifting the patient across a threshold. Counter by using the full current version of the rule and looking up components when uncertain.

### Treating a Low Score as Ruled Out

Equating "low risk" with "no disease" and discharging without a safety net. Counter by interpreting the score as a residual probability and communicating the warning signs that warrant return.

### Deferring Blindly to the Score Against Clinical Judgment

Overriding a clearly sick patient because the score is low, or abandoning concern because the number is reassuring. Counter by making disagreement explicit and trusting the clinical picture when the patient looks worse than the score.

### Using a Score to Rationalize a Predetermined Decision

Computing a low-risk score to justify not testing a patient you are worried about. Counter by scoring before the decision is fixed, and being honest about whether the score is informing or defending the plan.

### Forgetting to Document the Score and the Reasoning

Recording the disposition without the score, the probability it implies, or the rationale when judgment overrode it. Counter by documenting the score, the decision, and the reconciliation with clinical judgment.

## Self-Check

- Did I select a rule that answers the actual clinical question (probability of disease, mortality risk, imaging threshold, treatment threshold), rather than applying a familiar score by reflex?
- Did I confirm the patient is within the population the rule was derived and validated in (age, setting, comorbidity, disease spectrum), and down-weight the score if they fall outside it?
- Did I score the rule correctly and completely, using the current full version, and look up any uncertain component rather than guessing?
- Did I interpret the result as a probability or risk category with an honest residual risk, rather than as a binary "in" or "out"?
- When the score disagreed with my clinical assessment, did I make the disagreement explicit, reason through it, and trust the sicker-looking patient over a reassuring number?
- Did I avoid using the rule to rationalize a decision I had already made, and instead let it inform the decision?
- For preference-sensitive or threshold decisions, did I use the score as the anchor for a shared decision making conversation that weighed benefits, harms, and patient values?
- Did I document the score, the probability or category it implies, the decision, and the rationale when clinical judgment overrode or modified the score?
- This guidance is educational and informational only and is not medical advice; a qualified healthcare professional must be consulted for actual clinical decisions; when in doubt or when red flags appear, seek immediate specialist consultation and follow institutional protocols.
