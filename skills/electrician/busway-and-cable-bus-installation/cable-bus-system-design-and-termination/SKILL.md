---
name: cable-bus-system-design-and-termination.md
description: Use when the agent is designing cable bus systems, spacing insulated conductors in a ventilated enclosure, sizing ampacity, accommodating thermal expansion, and terminating at switchgear and transformers as an alternative to rigid busway.
---

# Cable Bus System Design and Termination

Cable bus is a hybrid system: insulated conductors held in precise spacing blocks inside a ventilated enclosure, combining the ampacity of parallel cables with the routing flexibility of busway. The judgment problem is that its ampacity depends on the maintained spacing between conductors, the enclosure ventilation, and the termination method, and all three are field-sensitive. Agents tend to treat cable bus as just parallel cable in a tray, when in fact the spacing blocks, the ventilation path, and the termination kits are what give the system its rating. A cable bus run with missing or mis-installed spacing blocks, a blocked enclosure, or improper terminations loses its ampacity, overheats, and fails. The skill exists to force the agent to design and terminate the system as a manufacturer-engineered assembly, not as field-improvised cable.

## Core Rules

### Maintain Conductor Spacing With the Specified Spacing Blocks
The ampacity of cable bus is achieved because the insulated conductors are held at a defined center-to-center spacing by molded spacing blocks at regular intervals, which allows each conductor to dissipate heat to the air and to the enclosure. The spacing is not optional; it is the basis of the system's ampacity rating. Install the blocks at the manufacturer's specified interval, oriented correctly for the conductor configuration (phases, neutral, equipment ground), and verify that no block is omitted, broken, or forced over insulation damage. A run with missing or wrong blocks reverts toward random cable-in-tray ampacity, which is far lower, and overheats at rated current.

### Size Ampacity From the Ventilated Enclosure Rating, Not Cable Tray Tables
Cable bus ampacity is rated by the manufacturer for the specific enclosure and ventilation configuration, and it is typically higher than the same conductors in a random-fill tray because of the controlled spacing and the ventilated enclosure. Do not apply generic NEC Article 392 cable tray ampacity tables to a cable bus system, because those tables assume different heat dissipation. Use the manufacturer's listed ampacity for the exact configuration, derate for ambient temperature above the rating basis and for any enclosure modification that reduces ventilation. Document the basis of the ampacity, because an inspector or future engineer will need to verify it against the listed assembly.

### Preserve the Enclosure Ventilation Path
The ventilated enclosure is integral to the ampacity rating, because it creates a chimney effect that moves air past the conductors. Do not block the ventilation openings with sealing material, paint, adjacent equipment, or building modifications, and do not substitute a solid enclosure for a ventilated one to keep out dust without re-rating the system. Where the environment requires dust or drip protection, use a manufacturer-approved enclosure and apply the corresponding derating. A cable bus run that has its ventilation blocked loses a substantial fraction of its ampacity and will overheat at loads that were previously safe.

### Terminate With the Manufacturer's Termination Kits and Stress Control
Cable bus terminations at switchgear, transformers, and taps use manufacturer-provided termination kits that include stress cones or stress control, sealing, and the hardware to transition from the spaced conductors to the lug. The termination is where the controlled spacing ends and where electrical stress concentrates, so it must follow the kit instructions exactly. Do not substitute field-fabricated terminations or omit the stress control, because the insulation at the termination sees both thermal and electrical stress and will track or fail prematurely without the kit components. Torque the lugs to specification and verify the phase arrangement matches the equipment.

### Accommodate Thermal Expansion and Conductor Movement
The conductors in cable bus expand and contract with load and ambient, and the system must allow that movement without imposing stress on the terminations or the spacing blocks. Provide the expansion and slack specified by the manufacturer at terminations and at directional changes, anchor the enclosure at the designated points, and use supports that allow longitudinal movement where required. A run that is rigidly anchored at both ends or that has no provision for movement transmits expansion force into the terminations, working the lugs loose and stressing the insulation until a failure occurs at the most constrained point.

### Compare Cable Bus to Rigid Busway on the Application Requirements
Cable bus and rigid busway serve overlapping applications but differ in cost, ampacity range, voltage, routing flexibility, and tap capability. Cable bus is often more economical for long straight runs at medium voltage and high current, accepts voltage up to 35 kV or higher, and tolerates building movement better than rigid busway. Rigid busway offers compact dimensions, factory plug-in tap capability, and a shorter installation time for standard low-voltage runs. Select based on the run length, voltage, required taps, environmental conditions, and cost, and do not assume one is universally better; the wrong choice is either overpriced or functionally limited.

### Coordinate Phase Arrangement and Neutral and Ground Placement
The conductor arrangement in cable bus (phase transposition, neutral position, equipment ground location) affects both ampacity and impedance, and it must match the manufacturer's design and the termination requirements. Transposed phases reduce impedance and balance inductive heating on long runs, while an incorrectly placed neutral or ground can induce circulating currents and overheating. Verify the arrangement at every spacing block and at both terminations, because a single reversed or misplaced conductor changes the electromagnetic field and can overheat an adjacent conductor or the enclosure.

## Common Traps

### Omitting or Mis-Installing Spacing Blocks
The mechanism is that spacing blocks are skipped, broken, or forced in to save installation time or because the conductor path is tight. The false signal is that the conductors are in the enclosure and the run looks complete. The harm is that the conductors touch or nest, lose the heat dissipation the spacing provides, and the run carries far less than rated current, overheating and failing at loads the system was supposed to support, because the spacing is the basis of the ampacity.

### Applying Cable Tray Ampacity Tables to a Cable Bus System
The mechanism is that an engineer sizes the system using generic cable tray ampacity from NEC 392 rather than the manufacturer's listed rating. The false signal is that the calculated ampacity appears conservative. The harm is either a wastefully oversized system (because cable bus actually rates higher than tray) or, more dangerously, an undersized system if the derating and configuration are not understood, and the basis of the rating is not traceable to the listed assembly, creating an inspection and liability problem.

### Blocking the Enclosure Ventilation
The mechanism is that ventilation openings are sealed, painted, or obstructed to keep out dust, water, or for aesthetics. The false signal is a cleaner-looking enclosure. The harm is the loss of the chimney effect that the ampacity depends on, so the conductors overheat at rated current, insulation ages, and the system fails, because the ventilated enclosure is a rated component, not a cosmetic shell.

### Field-Fabricating Terminations Without the Kit
The mechanism is that terminations are made with generic lugs and no stress control to save the cost of the kit or to use on-hand hardware. The false signal is that the connection is electrically continuous. The harm is insulation tracking and failure at the termination where electrical and thermal stress concentrate, because the stress cone and sealing components in the kit are what keep the insulation intact at the point where the controlled spacing ends.

### Rigidly Anchoring a Run With No Expansion Provision
The mechanism is that the enclosure and conductors are anchored at both ends and at every support with no movement allowance. The false signal is a rigid, stable-looking installation. The harm is that thermal expansion force transfers into the terminations and the most constrained blocks, working lugs loose, abrading insulation, and eventually failing at a termination or block, because the system was designed to move and was instead locked.

## Self-Check

- Are spacing blocks installed at the specified interval, correct orientation, and complete with none omitted or broken?
- Is the ampacity taken from the manufacturer's listed rating for the exact enclosure and ventilation configuration, not generic cable tray tables?
- Is the enclosure ventilation preserved, with no blocked openings and approved derating where dust or drip protection is added?
- Are terminations made with the manufacturer's kit, including stress control and sealing, with lugs torqued to specification?
- Is thermal expansion accommodated with manufacturer-specified slack and anchors, with supports that allow longitudinal movement?
- Has cable bus been selected over rigid busway based on a documented comparison of length, voltage, taps, environment, and cost?
- Is the phase, neutral, and ground arrangement verified at every block and both terminations, including any required transposition?
- Is the basis of the ampacity rating documented and traceable to the listed assembly for inspection and future verification?
