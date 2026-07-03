---
name: electrical-inspection-checklist-and-deficiency-identification.md
description: Use when the agent is performing electrical inspections, reviewing work for code compliance, identifying deficiencies and red-tag items, preparing for inspector visits, or evaluating whether installed wiring meets National Electrical Code requirements before sign-off.
---

# Electrical Inspection Checklist and Deficiency Identification

An inspection is the last line of defense between latent defects and energized, occupied buildings. The judgment problem is that inspectors and electricians alike tend to inspect for what is easy to see — neat workmanship, labeled breakers, visible connectors — while the failures that cause fires and shocks are often hidden inside boxes, behind devices, or in the relationship between components that only code knowledge reveals. A checklist that is treated as a box-ticking exercise produces a false sense of assurance, because the most dangerous deficiencies are precisely the ones that do not appear on a generic list. This skill covers how to inspect systematically, what inspectors actually prioritize, how to distinguish cosmetic issues from code violations, and how to identify the red-tag items that must be corrected before energization.

## Core Rules

### Inspect Against the Applicable Code Cycle, Not Memory

The NEC is revised on a three-year cycle, and local jurisdictions frequently adopt amendments that change requirements. An inspector or electrician who works from remembered rules is inspecting against a code that may no longer be in force. The defense is to confirm the adopted code edition and any local amendments before evaluating any installation. The trap is applying a rule from an older cycle that has been relaxed, or enforcing a rule from a newer cycle that has not yet been adopted — both produce incorrect deficiency calls that erode credibility and waste correction effort.

### Sequence the Inspection to Match the Construction Timeline

Inspections are not a single event; rough-in inspection happens before walls close, and the final inspection happens after devices and covers are installed. Work that will be concealed must be inspected before concealment, because a deficiency found after drywall is up becomes exponentially more expensive to correct. The trap is deferring all inspection to the end, when hidden splices, unsupported cables, and box-fill violations become invisible. The defense is to map every inspection to the construction milestone it protects and to refuse to cover work that has not passed the appropriate rough inspection.

### Prioritize Life-Safety Deficiencies Over Workmanship Issues

Not all deficiencies carry the same risk. An ungrounded receptacle, an undersized equipment grounding conductor, a missing arc-fault device, or an improperly bonded service carries direct shock or fire risk and is a red-tag item. A slightly crooked cover plate or a label that is peeling is a workmanship issue that does not affect safety. The trap is spending inspection effort and correction budget on visible cosmetic issues while missing a hidden parallel neutral path or a multiwire branch circuit with a removed tie handle. The defense is to classify every deficiency by its failure consequence and address life-safety items first.

### Verify the Relationship Between Components, Not Just Each Part

A code-compliant installation is not merely a collection of individually compliant parts; the interaction between parts is where many defects hide. A multiwire branch circuit with handle-tied breakers is compliant, but the same configuration with independent single-pole breakers is a violation. A receptacle with an equipment grounding conductor is compliant, but if that conductor is downstream of a GFCI that is incorrectly wired line-load, the ground path may be compromised. The trap is inspecting each component in isolation and pronouncing it correct. The defense is to trace the functional path — source, overcurrent protection, conductors, devices, and termination — and verify the relationships.

### Document Deficiencies With Location, Code Reference, and Correction

A deficiency report that says "fix the grounding" is useless to the person who must correct it. A useful deficiency entry specifies the exact location, the code article violated, and the specific correction required. The trap is vague documentation that forces re-inspection to rediscover the problem, or that allows the correction to be made incorrectly because the requirement was not stated. The defense is to record each deficiency with a photograph, a location description, the NEC article number, and the corrective action, so that the correction can be verified against a clear standard rather than memory.

### Treat Red-Tag Items as Energization Blockers

Certain deficiencies are severe enough that the installation must not be energized until corrected: open neutrals that energize equipment frames, reversed polarity on life-safety circuits, missing or severed equipment grounding conductors, panelboards with mixed neutrals and grounds, and any condition that places voltage on a surface intended to be grounded. The trap is treating a red-tag as a recommendation that can be deferred to a punch list. The defense is to maintain a hard distinction between red-tag items that block energization and yellow-tag items that require correction but do not present immediate hazard, and to communicate that distinction unambiguously.

### Re-Inspect Corrections, Not Just the Original Work

When a deficiency is corrected, the correction itself must be inspected, because corrections are frequently made hastily and introduce new defects. A re-pulled cable may exceed bend radius, a re-terminated neutral may now share a busbar with a grounded conductor, or a replaced breaker may be the wrong amperage. The trap is assuming that a reported correction is correct and signing off without verification. The defense is to treat every correction as new work that requires its own inspection against the same standard, and to never close a deficiency based on a verbal report of completion.

## Common Traps

### The "Looks Good" Visual Pass

An inspector or electrician opens a panel, sees neat routing and labeled breakers, and pronounces the work compliant without pulling devices or testing continuity. The trap is that visual neatness is uncorrelated with electrical correctness. A beautifully dressed panel can have neutrals and grounds sharing a busbar, double-tapped lugs, or conductors undersized for the breaker. The false signal is aesthetic order; the harm is that hidden defects pass inspection and energize with latent faults that surface later as overheating, shock, or fire. The defense is to verify electrically, not just visually, and to pull a sample of devices to confirm terminations.

### Missing the Multiwire Branch Circuit Violation

A multiwire branch circuit uses a shared neutral, and it requires that the ungrounded conductors be tied or ganged so that opening one disconnects both. An electrician removes the handle tie to use the slots for something else, or installs two independent single-pole breakers, and the shared neutral is now unprotected. The trap is that the circuit appears to function normally, but if one breaker is off and the neutral carries current from the other leg, the imbalance can overload the neutral and create shock hazard on a de-energized circuit. The false signal is normal operation; the harm is neutral overload and shock risk that manifests only under specific switching conditions.

### Ignoring Box-Fill and Conduit-Fill Calculations

A box or raceway that is physically full may still be within fill limits, and a box that has room to spare may exceed the calculated fill. The trap is judging fill by eye rather than by calculation, because the volume allowance depends on conductor size, device volume, grounding conductors, and internal clamps. An overfilled box causes conductor insulation damage from compression, leading to future faults. The false signal is "there's still space"; the harm is insulation failure that develops over years and causes a short or ground fault long after the wall is closed and the inspector is gone.

### Confusing GFCI Test-Button Operation With Actual Protection

A GFCI receptacle with a functioning test button trips when the button is pressed, which the installer takes as proof of protection. The trap is that the test button only verifies the internal trip mechanism; it does not verify that the equipment grounding conductor is present, that the line-load wiring is correct, or that downstream receptacles are actually protected. A miswired GFCI with line and load reversed will not provide downstream protection and may not trip on a real ground fault. The false signal is a successful button test; the harm is unprotected receptacles labeled as protected, and potential shock exposure at downstream locations.

### Accepting an Ungrounded Circuit With Bootleg Grounds

A three-prong receptacle is installed on an old two-wire circuit, and the installer connects the equipment grounding terminal to the neutral to make a tester show correct wiring. This is a bootleg ground, and it is extremely dangerous because any open neutral upstream will energize the receptacle frame and any connected equipment at line voltage. The trap is that a standard three-light tester reads "correct" on a bootleg ground, giving false assurance. The false signal is the tester indication; the harm is a frame energized at 120V that appears safe, a condition that has killed people plugging in appliances.

### Overlooking Clearance and Working Space Violations

A panelboard is installed with less than the required 36 inches of clearance in front, or with equipment encroaching on the dedicated equipment space above. The trap is that these violations are often introduced by other trades — ductwork run above a panel, shelving installed in front — after the electrician has left, and they are easy to miss if the inspection focuses only on the electrical work itself. The false signal is that the panel itself is compliant; the harm is that a worker cannot safely access the panel for operation or maintenance, and the violation is discovered only when someone is injured trying to work in cramped conditions.

## Self-Check

- Did I confirm the adopted code edition and any local amendments before evaluating the installation, rather than relying on remembered rules?
- Did I inspect concealed work at the rough-in stage before walls were closed, and have I refused to sign off on work that was concealed without inspection?
- For each deficiency identified, did I record the exact location, a photograph, the specific NEC article violated, and the required correction?
- Did I classify each deficiency as a red-tag energization blocker or a yellow-tag correction item, and did I communicate that distinction clearly to the responsible party?
- Did I verify component relationships — multiwire branch circuit handle ties, neutral-ground separation, GFCI line-load wiring, equipment grounding continuity — rather than inspecting each part in isolation?
- Did I pull a sample of devices and test electrically (continuity, polarity, grounding) rather than relying on a visual-only inspection?
- Did I re-inspect every reported correction against the original standard rather than closing deficiencies based on verbal confirmation?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
