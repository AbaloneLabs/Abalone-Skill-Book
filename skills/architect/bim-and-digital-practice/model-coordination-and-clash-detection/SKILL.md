---
name: model_coordination_and_clash_detection.md
description: Use when the agent is federating discipline models, running clash detection, triaging coordination issues, leading BIM coordination meetings, or resolving multidisciplinary conflicts before construction documents are issued.
---

# Model Coordination And Clash Detection

Model coordination is the discipline of federating the separate models produced by each trade into a single integrated representation, then systematically finding and resolving the conflicts between them before they become construction problems. It is where the architect's holistic integration responsibility is most visible, because no single discipline sees the whole building and the architect must hold the federated model as the source of truth. Agents often treat clash detection as running software and reporting a list, and miss that the value is in the judgment of which clashes matter, which are accepted tolerances, and which represent unresolved design decisions that need an architectural ruling. Done well, coordination produces a building that can be built as drawn; done poorly, it produces a clash report that nobody acts on and a field full of change orders. The goal is a federated model whose remaining clashes are intentional and documented, not a model with zero clashes by suppression.

## Core Rules

### Federate Against A Single Authoritative Coordinate Frame

Before any clash detection, confirm that every discipline model shares the same coordinate origin, survey point, true north, elevation datum, and units, and that each model was exported against that frame without internal repositioning. Run a coordinate verification pass that overlays grids and levels, because a model that is even slightly misaligned will generate hundreds of false clashes that obscure the real ones. The federated model must be assembled in one common data environment with version control, and the version used for each clash run must be recorded, because coordination against a stale model produces conclusions that are already wrong. The architect owns the federated frame because it is the integration reference, and any discipline that cannot conform must be flagged immediately.

### Define Clash Tests By Discipline Pair And System Type

Do not run a single global clash test of everything against everything; the result is an overwhelming list that no one triages. Instead, define discrete clash tests by discipline pair and system type — structure versus architecture, MEP versus structure, MEP versus architecture, duct versus pipe, plumbing versus electrical — each with its own clearance rules. Hard clash tests find geometric intersections; clearance or soft clash tests find violations of required clearance around equipment, access aisles, and code-required clearances; and workflow clashes check logical sequencing like access for installation. Naming each test and running it consistently across milestones produces a trend that shows whether coordination is improving or regressing.

### Triage Clashes By Significance, Not By Count

A clash report with three thousand items is useless until it is triaged into meaningful categories: real conflicts that require design change, accepted tolerances that can be suppressed with a documented reason, duplicate or false clashes from overlapping geometry, and clashes already resolved in a more recent model version. Assign each real clash to an owner and a resolution date, and require that the resolution be modeled, not just agreed in meeting notes. The architect must rule on clashes that cross disciplines or that involve design intent, because those cannot be resolved by the trades alone. Track the count of open real clashes by test across milestones, because a flat or rising count means coordination is not actually happening.

### Resolve The Root Design Issue, Not Just The Clash

Most clashes are symptoms of an unresolved design decision: a beam depth that was never coordinated with the duct routing, a shaft that is too small for the systems it must carry, a ceiling cavity that cannot hold the structure and the MEP together. When a clash recurs across milestones or appears in many locations, treat it as a design problem and convene the disciplines to resolve the underlying parameter — the beam type, the shaft size, the floor-to-floor — rather than patching each instance. The architect's role is to elevate recurring clashes into design decisions, because resolving the same clash in fifty places one at a time is more expensive and less reliable than changing the system.

### Govern Coordination Meetings With Federated Model Authority

Coordination meetings must be run against the live federated model, with each discipline able to open their own model in context, and decisions must be recorded against specific clash IDs with assigned owners and dates. Require that resolutions be modeled before the next meeting, so the meeting verifies fixes rather than re-deciding them. The architect or the designated BIM coordinator chairs the meeting and has authority to rule on cross-discipline disputes, because coordination stalls when every conflict requires escalation. Maintain a decision log tied to clash IDs, because unrecorded verbal decisions that are not modeled and not logged disappear.

### Distinguish Tolerances, Clearances, And Constructability From True Clashes

Not every geometric intersection is a defect. Steel fabrication tolerances, insulation thickness assumptions, and required access clearances around equipment all produce apparent clashes that are accepted in practice. Document accepted tolerances and clearances in the BEP or a coordination rule set, and suppress those clashes with a reason code so they do not reappear in every report. Conversely, a clearance clash around a code-required access aisle or a maintenance clearance is a real defect even though nothing physically intersects, because the building will not function. Train the triage on the difference, because suppressing real clearance clashes as "tolerances" hides problems that surface in construction.

### Coordinate To Construction Tolerance, Not To Perfect Geometry

The federated model is a representation, not a fabrication drawing, and demanding zero geometric discrepancy between models is both impossible and unnecessary. Establish the coordination tolerance appropriate to each system — a few millimeters for prefab connections, several centimeters for cast-in-place concrete — and treat discrepancies within tolerance as accepted. The architect must ensure the team coordinates to the right tolerance for the construction method, because over-precise coordination wastes effort and under-precise coordination produces field conflicts. Document the tolerance basis so reviewers understand why a residual discrepancy is acceptable.

## Common Traps

### Running One Global Clash Test And Drowning In The List

The team runs architecture, structure, and MEP against each other in a single test, generates thousands of clashes, and either suppresses them in bulk or abandons the report as unmanageable. The mechanism is that a single test feels thorough and discrete tests feel like extra setup, and the false signal is that a high clash count means thorough coordination. The harm is that real conflicts are buried in noise, no one owns the list, and the report becomes shelfware while real clashes persist into construction. Clash tests must be defined by discipline pair and system type, run consistently, and triaged into owned, dated resolutions.

### Treating Clash Count As The Metric Of Coordination Quality

The team celebrates driving the clash count to zero, but the zero was achieved by suppressing accepted tolerances, hiding real conflicts in inactive worksets, or modeling systems at such generic a level that they cannot clash. The mechanism is that clash count is an easy number to report and stakeholders reward the trend, and the false signal is that zero clashes means a coordinated model. The harm is that the model looks clean while concealing unresolved design decisions, and construction discovers conflicts that the model should have caught. The metric must be open real clashes owned and resolved on schedule, plus a record of accepted tolerances, not the raw count.

### Resolving Clashes Verbally Without Modeling The Fix

In the coordination meeting, the disciplines agree that the duct will reroute around the beam, the decision is minuted, and the model is never updated, so the next clash run reports the same conflict and the team re-litigates it. The mechanism is that verbal agreement feels like resolution and modeling the change feels like redundant effort, and the false signal is that the issue was decided. The harm is that decisions evaporate, the same clashes recur, and eventually a conflict reaches construction because the agreed fix was never actually modeled or detailed. Every coordination decision must be modeled, verified in the next federated run, and tied to a clash ID in the log.

### Suppressing Real Clearance Clashes As Tolerances

The team suppresses clashes around mechanical equipment, electrical panels, and accessible restrooms as "fabrication tolerance" when they are actually violations of required maintenance, code, or accessibility clearance. The mechanism is that suppression is faster than redesign and the geometric intersection looks small, and the false signal is that the items nearly fit. The harm is that the building is constructed with equipment that cannot be serviced, panels that cannot be opened, and aisles that fail accessibility review, all of which surface as expensive field corrections. Clearance clashes must be evaluated against the actual required clearance, not against geometric intersection alone, and only true fabrication tolerances may be suppressed.

### Coordinating Against A Stale Or Misaligned Federated Model

The clash run federates a structural model from two weeks ago against a current MEP model, or a model whose coordinate origin shifted during export, and the report is filled with clashes that do not exist in reality. The mechanism is that federation feels automatic and the version provenance is not checked, and the false signal is that the software produced a report and so it is valid. The harm is that the team chases phantom clashes, loses confidence in the process, and may suppress a real conflict as a false positive. The federated model version, the discipline model versions, and the coordinate verification must be recorded for every run, and stale inputs must be rejected.

### Letting One Discipline's Model Lag The Design

The architectural model advances through design development while the structural or MEP model remains at schematic resolution, so the federated model cannot reveal real conflicts because one system is under-modeled. The mechanism is that discipline models progress at different paces and no one enforces milestone parity, and the false signal is that the model federates and so coordination is happening. The harm is that clashes emerge late when the lagging discipline catches up, leaving no time to resolve them before documents are issued. Discipline models must be required to reach the same LOD at each milestone, and lagging models must be flagged as a coordination risk.

## Self-Check

- [ ] Has every discipline model been verified against a single authoritative coordinate frame, origin, datum, and units before federation, with the version recorded?
- [ ] Are clash tests defined by discipline pair and system type, including hard, clearance, and workflow clashes, and run consistently across milestones?
- [ ] Is each clash triaged into real, accepted-tolerance, duplicate, or already-resolved, with real clashes assigned to an owner and a resolution date?
- [ ] Are recurring clashes elevated to root design decisions — beam depth, shaft size, floor-to-floor — rather than patched instance by instance?
- [ ] Are coordination meetings run against the live federated model, with decisions tied to clash IDs, owners, and dates, and fixes modeled before the next meeting?
- [ ] Are accepted tolerances and clearances documented with reason codes, and are real clearance clashes around equipment, panels, and accessible routes treated as defects, not suppressed?
- [ ] Is coordination performed to the appropriate construction tolerance for each system, with the tolerance basis documented?
- [ ] Do all discipline models reach the same LOD at each milestone, so the federated model can actually reveal conflicts?
