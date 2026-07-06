---
name: medication-error-root-cause-analysis.md
description: Use when the agent is investigating a medication error or near miss, leading a root cause analysis after a sentinel event, mapping causal factors with a fishbone or five-whys method, or writing an action plan to prevent recurrence.
---

# Medication Error Root Cause Analysis

A medication error root cause analysis (RCA) is a forensic and systems exercise, not a blame exercise. The recurring judgment problem is that teams converge too quickly on a human explanation ("the pharmacist was careless," "the nurse double-checked wrong") and stop investigating before the system conditions that made the error inevitable are exposed. The discipline of RCA is to keep pushing past the proximate human act until the latent system failures (look-alike packaging, ambiguous order sets, missing independent checks, production pressure, training gaps) are identified, because only system-level causes yield system-level fixes. An RCA that names a person as the root cause produces no actionable prevention and guarantees recurrence, because the next person placed in the same system will make the same error.

## Core Rules

### Treat Every RCA as a Systems Investigation, Not a Personnel Investigation

The first principle, borrowed from high-reliability organizations and enforced by accreditation, is that the goal is to understand how the system allowed or caused the error, not to assign individual fault. The RCA team must explicitly set aside discipline and performance management, which are separate processes, and focus on the conditions, design flaws, and latent failures that shaped the human action. A pharmacist who selected the wrong concentration did so inside a system that stocked similar vials together, lacked barcode verification, and was under production pressure; the RCA must surface those factors. Even when individual recklessness is present, the system question remains: why did the system not catch it? Framing the RCA as a personnel matter suppresses candor and produces a shallow analysis.

### Reconstruct the Event Timeline Before Hypothesizing Causes

Before the team proposes any cause, it must reconstruct the complete timeline of the event from order entry through discovery, using order records, dispensing logs, administration records, interviews, and the physical products and packaging. Each step must be pinned to a time, an actor, and the information available to that actor at that moment. The reconstruction frequently overturns the initial assumption: what looked like a "dosing error" may reveal that the weight was entered in pounds instead of kilograms three steps earlier, or that a verbal order was misheard and never read back. Hypothesizing causes before the timeline is fixed guarantees the team investigates the wrong event.

### Push Past the Proximate Cause to Latent System Causes

The proximate cause is the last human action before the harm (the pharmacist released the wrong product). The root causes are the latent system conditions upstream (the two products were stored adjacent with no tall-man lettering, the verification screen did not display the concentration, the pharmacist was covering two areas simultaneously). The team must apply iterative questioning (five-whys) or a structured fishbone (people, process, technology, environment, materials, management) until it reaches causes that, if fixed, would have prevented the error regardless of which individual was working. If the only cause the team can name is "human error," the RCA is incomplete; human error is the starting point of the investigation, not its conclusion.

### Distinguish Active Failures From Latent Conditions and Weak Defenses

A useful framework separates active failures (the sharp-end actions of the people involved), latent conditions (the organizational decisions that shaped the environment, such as staffing levels, training, and design choices), and weak defenses (the safeguards that should have caught the error but did not, such as barcode scanning, independent double checks, and clinical decision support). Most medication errors pass through multiple failed defenses before reaching the patient, and a strong RCA maps every defense that failed or was absent. The action plan should strengthen defenses so that the next similar error is caught, not rely on the next person being more careful. A single failed defense is a near miss; an error that passed three defenses reveals a deeply degraded safety system.

### Scope the Action Plan to the Causes Found, With Owners and Due Dates

Every identified cause must generate at least one action, and every action must have a single accountable owner, a due date, and a measurable success criterion. Vague actions ("educate staff," "emphasize safety") are not actions because they cannot be verified and do not change the system. Strong actions are forced functions (physically prevent the error), constraints (make the wrong action harder), or automation (remove the human dependency); weak actions are rules, policies, and training, which rely on human vigilance and are the least reliable. The team should rank actions by hierarchy of effectiveness and prefer the stronger ones, and must include a plan to measure whether the fix actually reduced recurrence, because an unmeasured action is an assumption.

### Close the Loop With Measurement and Reanalysis

An RCA is not complete when the action plan is written; it is complete when the actions are implemented and the recurrence rate is shown to have fallen. The team must define a leading metric (e.g., rate of similar near misses, audit of the new safeguard's use) and re-evaluate at a defined interval. If recurrence happens despite the actions, the RCA was incomplete or the actions were ineffective, and a new analysis is required. Treating the RCA as a closed document the day it is filed guarantees that ineffective fixes persist undetected.

## Common Traps

### Stopping at the Proximate Human Cause

The team identifies the person who made the last action before the error, names "human error" as the root cause, and stops. The mechanism is that the human act is the most visible and most emotionally salient factor, and the false signal is that naming the actor explains the event. The harm is that the latent system causes remain unfixed, and the next person in the same system repeats the error, while leadership believes the problem was solved because someone was "addressed." Human error is the beginning of the analysis, never the end.

### The "Re-educate Staff" Non-Action

The action plan consists of "re-educate staff" and "reinforce the policy," because these are easy to write and easy to execute. The mechanism is that education and policy feel like responsible responses and are measurable as completed, and the false signal is that a delivered in-service changes behavior. The harm is that education relies on human memory and vigilance under the same conditions that produced the error, so recurrence is essentially guaranteed. Education is the weakest layer of error prevention and should supplement, never replace, system and engineering controls.

### Anchoring on the First Plausible Story

The team forms a hypothesis in the first ten minutes (often driven by the most senior or most vocal member) and then collects evidence selectively to confirm it. The mechanism is confirmation bias, and the false signal is that the first story fits the known facts. The harm is that disconfirming evidence (the timeline detail that breaks the story, the second product involved) is ignored, and the true root cause is never investigated. The timeline reconstruction must precede hypothesis, and the team must actively seek evidence that would disprove the leading story.

### Single-Cause Thinking

The team identifies one cause, declares it the root cause, and writes actions against only that one. The mechanism is that single-cause explanations are cognitively clean, and the false signal is that one cause is sufficient because it "explains" the event. The harm is that medication errors almost always result from multiple contributing factors, and fixing only one leaves the others in place to cause the next error. A robust RCA maps the full set of contributing causes and acts on each.

### Confusing Near Miss With No-Harm and Stopping the Analysis

Because the error was caught before reaching the patient (a near miss) or reached the patient without harm, the team treats it as low priority and conducts a shallow analysis. The mechanism is that harm severity drives perceived importance, and the false signal is that no harm means no real system problem. The harm is that the same system failure that produced the near miss will, under slightly different timing, produce a fatal error, and the cheap opportunity to learn was wasted. Near misses are the most valuable safety data a pharmacy has and deserve the same rigor as harmful events.

### Action Plans Without Owners, Dates, or Measurement

The RCA identifies good causes and lists reasonable actions, but assigns no owner, no due date, and no success metric, so the actions are never implemented or never verified. The mechanism is that the RCA document feels like the deliverable, and the false signal is that a written plan equals a solved problem. The harm is that the analysis was wasted, the causes persist, and the next RCA investigates the same error. Every action needs a single owner, a date, and a measurable check.

## Self-Check

- Did the team explicitly frame the RCA as a systems investigation, separating it from any disciplinary process, and focus on latent conditions rather than individual fault?
- Was the complete event timeline reconstructed from records and interviews before any cause was hypothesized?
- Did the analysis push past the proximate human act to latent system causes (design, staffing, training, defenses), using five-whys or a fishbone?
- Were all failed or absent defenses (barcode, double check, decision support) mapped, not just the single last action?
- Does every identified cause have at least one action, with stronger engineering or forced-function actions preferred over education and policy?
- Does each action have a single accountable owner, a due date, and a measurable success criterion, rather than vague "re-educate" or "reinforce" language?
- Is there a defined metric and reanalysis date to confirm the fix actually reduced recurrence, treating the RCA as closed only when improvement is demonstrated?
- Were near misses analyzed with the same rigor as harmful events, recognizing them as low-cost learning opportunities?
- Does the output defer final patient-care and personnel decisions to the qualified pharmacy leadership and patient safety committee where the question exceeds the agent's competence?
