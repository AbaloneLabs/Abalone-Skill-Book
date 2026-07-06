---
name: bridge-load-rating-and-evaluation.md
description: Use when the agent is performing a bridge load rating, evaluating an existing bridge against HL-93 or legal and permit loads, determining inventory versus operating rating factors, deciding whether a bridge must be posted for weight restriction, or preparing an evaluation under AASHTO LRFR. Applies before a rating factor is finalized, while selecting the analysis method, the load conditions, and the condition and system factors, and when judging posting, permit, and scour-critical consequences.
---

# Bridge Load Rating and Evaluation

Bridge load rating is the engineering evaluation that determines the safe live-load capacity of an existing bridge and expresses it as a rating factor against a defined loading. It is governed by the AASHTO Manual for Bridge Evaluation and, for bridges designed or evaluated under load-and-resistance-factor methods, the AASHTO LRFR provisions, with the National Bridge Inspection Standards (23 CFR 650) requiring every bridge on public roads to carry a current load rating. The harm this skill prevents is a rating that overstates capacity and lets an overweight crossing proceed until a girder yields, a deck punches through, or a connection fractures; and, conversely, an overly conservative rating that triggers needless posting, detours, and economic cost. Load rating is a safety-of-the-public activity, and the judgment calls (as-built versus in-place section properties, condition factors, live-load distribution, dynamic allowance, and the treatment of deterioration and scour) change the answer far more than arithmetic precision. Agents must treat the work as decision support for a licensed engineer who seals the rating, and must defer final posting, permit, and load-restriction decisions to that engineer.

## Core Rules

### Establish the Correct Evaluation Basis and Rating Levels

Before computing any factor, fix the evaluation basis: design load rating (using HL-93 to confirm the as-designed capacity), legal load rating (using the AASHTO legal trucks and state-specified routine permit vehicles), and permit load rating (for specific overload vehicles). Each yields two reference ratings under LRFR, the inventory rating (the load that can be carried indefinitely, roughly the design reliability level) and the operating rating (the maximum permissible occasional load at a lower reliability). The inventory rating governs routine service and long-term management; the operating rating governs permit and one-time decisions but must not be used to justify chronic overloads. Choosing the wrong level, or reporting only the more favorable operating factor, misrepresents the bridge. Confirm which rating the owner requires for the decision at hand and report both inventory and operating factors for design and legal ratings so the reliability context is explicit.

### Capture As-Built and In-Place Condition, Not the Original Design

A load rating reflects the bridge as it stands today, not the day it opened. Gather the as-built plans, shop drawings, material certifications, and any strengthening or repair records, then reconcile them with the most recent inspection: section loss in steel, delamination and spalling in concrete, corrosion of prestressing strand, reduced bearing area, cracked or frozen bearings, and any collision or fire damage. Convert observed deterioration into reduced section properties, reduced capacity, and a condition factor (LRFR condition factor ranges from 0.85 to 1.00 based on NBI condition and system behavior). Using nominal as-designed properties for a deteriorated member is the single most common source of an unsafe rating, because the calculation looks rigorous while the input is wrong.

### Model Live Load, Distribution, and Dynamic Allowance Consistently

Apply the HL-93 notional load (the design truck or tandem, combined with the lane load, and the 33 percent fatigue truck for fatigue checks) for design load rating, and the AASHTO legal trucks (Type 3, 3-S2, 3-3, and the four specialized hauling vehicles EV2/EV3 for posting evaluation) for legal rating. Use the AASHTO LRFD live-load distribution factors appropriate to the superstructure type, girder spacing, span length, and skew, and apply the dynamic load allowance (33 percent for deck joints, 15 percent for fatigue and other components, reduced for earth on backfills). For curved, skewed, or unusual structures, refined analysis (grillage or finite element) may be required because the approximate distribution factors do not apply. Inconsistent distribution factors or a missing impact factor can swing a rating factor by tens of percent, so verify the applicability limits of every approximate factor before using it.

### Treat Posting, Permit, and Scour Decisions as Separate Consequences

A legal rating factor below 1.0 triggers a posting evaluation, and the posting analysis uses the comparison of the rating factor to the posting thresholds in the MBE to determine whether posting is required and at what tonnage. For permit vehicles, run a one-by-one (or automated routing) analysis that places the actual truck, accounts for the presence of a companion legal lane load when required, and checks every controlling limit state including fatigue-prone details. For bridges over water, the load rating must also consider the scour-critical foundation: the LRFR requires evaluating the foundation at the scour-impaired condition, because a foundation rated adequate on unscoured ground can be unstable after the design storm. Do not treat posting, permit, and scour as a single number; each has its own limit states, load combinations, and operational consequences, and conflating them hides the real risk.

### Select the Correct Analysis Method and Limit States

Choose between the load factor rating (LFR, ASD-based, legacy), the LRFR (current standard for post-2003 designs), and the allowable stress rating for older structures, and do not mix methods within one rating without explicit justification. For LRFR, run the strength, service, and fatigue-and-fracture limit states: Strength I and II govern legal and permit capacity, Service I controls permit deflection and crack control for concrete, Service III controls prestress tension in concrete, and Fatigue I/II govern steel details. A rating that checks only strength misses service cracking in concrete girders and fatigue in steel fracture-critical members. Document the method, the limit states checked, and the controlling member and location so the engineer can audit the result.

### Document Assumptions, Unknown Materials, and Engineering Judgment

Older bridges often lack material certifications, and the MBE provides default values (for example, yield strengths for steel of unknown vintage, or concrete strengths based on era and core testing) that must be applied with documented conservatism. Where properties are assumed, use the lower-bound defaults and note that the rating is provisional until verified by testing. Record every assumption (bearing condition, composite action, effective flange width, end restraint, presence of diaphragms) because a load rating is a legal document that will be re-examined after any future incident. A rating with undocumented assumptions is indefensible, and an undocumented conservative assumption that later proves unconservative is a liability.

## Common Traps

### Using As-Designed Properties for a Deteriorated Bridge

The rating uses the original section modulus and yield strength while the inspection reports heavy section loss and pack rust, so the computed factor looks adequate but the real capacity is far lower. The false signal is a clean, passing rating factor; the harm is an overload crossing that buckles or fractures a member that was already weakened. Always reconcile rating input with the latest inspection and reduce properties for observed deterioration.

### Reporting Only the Operating Rating

The operating rating is reported as the headline number because it is higher, masking that the inventory rating (the safe indefinite load) is below 1.0. The false signal is a single favorable factor; the harm is chronic legal traffic exceeding the inventory capacity and accelerating fatigue and service damage. Always report inventory and operating factors for design and legal ratings.

### Ignoring the Applicability Limits of Distribution Factors

Approximate AASHTO distribution factors are used outside their valid range of girder spacing, span, or skew, producing factors that are unsafe for wide spacing or large skew. The false signal is a textbook-compliant factor; the harm is a girder carrying more load than the rating assumes. Verify the applicability limits and use refined analysis when they are exceeded.

### Omitting Scour from the Foundation Rating

The superstructure rating is high, so the bridge is declared adequate, but the foundation is scour-critical and unstable after the design flood. The false signal is a passing superstructure factor; the harm is a bridge that survives traffic but fails in the next major storm. Always include the scour-impaired foundation condition in the rating for water crossings.

## Self-Check

- Is the evaluation basis fixed (design, legal, permit) and are both inventory and operating rating factors reported for design and legal ratings?
- Are section properties, material strengths, and the condition factor based on the most recent inspection and observed deterioration, not the as-designed values?
- Are HL-93 for design rating and the AASHTO legal trucks (including EV2/EV3 for posting) used, with distribution factors verified against their applicability limits?
- Are the strength, service, and fatigue-and-fracture limit states all checked, with the controlling member and location identified?
- Is a legal rating factor below 1.0 carried into a posting evaluation with the correct tonnage, and are permit vehicles checked with the required companion lane load?
- For bridges over water, is the foundation rated at the scour-impaired condition and is the bridge correctly classified as scour-critical?
- Are all assumed material properties, bearing conditions, and analysis choices documented so a licensed engineer can seal the rating?
