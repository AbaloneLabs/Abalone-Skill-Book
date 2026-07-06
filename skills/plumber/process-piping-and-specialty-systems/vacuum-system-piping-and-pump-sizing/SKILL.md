---
name: vacuum-system-piping-and-pump-sizing.md
description: Use when the agent is sizing vacuum piping for a lab, medical, or industrial system, selecting pipe material for vacuum service, calculating conductance and effective pumping speed at the point of use, sizing the vacuum pump and receiver, or troubleshooting insufficient vacuum at remote outlets.
---

# Vacuum System Piping and Pump Sizing

Vacuum systems move gas (air, vapor, or process gas) from points of use back to a central pump, and unlike pressure piping, the flow physics are different: in vacuum, gas is rarefied, conductance (the ease of flow through a pipe) drops sharply with small diameters and long runs, and the effective pumping speed at the point of use is always far less than the pump's rated speed. The judgment problem is that a vacuum system that "works" on the bench — the pump holds deep vacuum at the receiver — can deliver almost no usable vacuum at a remote outlet 50 feet away through undersized piping, because the piping conductance chokes the flow. An installer who sizes vacuum piping like pressure piping, who ignores the conductance loss, or who selects the wrong pump type for the vacuum level will deliver a system that cannot support the lab, medical, or industrial process it serves. This skill covers conductance, pump selection, material, and the role limits that place medical-vacuum and high-vacuum work with certified specialists.

## Core Rules

### Size Piping by Conductance, Not by Pressure-Pipe Rules

In vacuum service, the pipe's conductance (its capacity to pass gas) is the dominant constraint, and conductance drops dramatically with decreasing diameter and increasing length — far more sharply than in pressure piping. A 1/2-inch vacuum line that carries adequate flow at 10 feet may be useless at 50 feet, because the conductance of the long small line chokes the pump's effective speed at the outlet to a fraction of its rated capacity. The disciplined approach is to calculate the conductance of each run (using vacuum conductance formulas or tables for the relevant flow regime — viscous for rough vacuum, molecular for high vacuum), determine the effective pumping speed at each point of use (a function of the pump speed and the total conductance in series), and upsize the piping until the effective speed at the most remote outlet meets the process requirement. The trap is using pressure-pipe sizing rules ("1/2-inch carries the flow") and ending up with a system where the pump runs but the remote outlet sees almost no vacuum. For rough vacuum (lab and medical), keep runs short and diameters generous; for high vacuum, conductance is so critical that every inch of small line matters.

### Match the Pump Type to the Required Vacuum Level

Vacuum pumps are categorized by the vacuum level they achieve and the mechanism, and selecting the wrong type is a common failure. Rough vacuum (down to about 1 Torr) is served by oil-sealed rotary vane, liquid ring, dry screw, or claw pumps, and is typical for medical/surgical vacuum, lab rough vacuum, and industrial hold-down. Medium to high vacuum (down to 10^-3 Torr and below) requires roots blowers (boosters) backing a roughing pump, or oil-diffusion, turbomolecular, or cryogenic pumps, and is typical for semiconductor, coating, and research applications. The trap is specifying a rough-vacuum pump (a liquid ring or rotary vane) for an application that needs high vacuum, or vice versa — a high-vacuum turbomolecular pump for a lab rough-vacuum need that would be served by a simple vane pump at a fraction of the cost. The disciplined approach is to determine the required vacuum level and the gas load (the amount of gas or vapor the process generates), then select the pump type and capacity to achieve that level at that load. For medical/surgical vacuum, the pump type, redundancy, and receiver are specified by NFPA 99 (medical gas) and require ASSE/Medical Gas certification.

### Select Material for Vacuum Level, Cleanliness, and Chemical Compatibility

Vacuum piping material depends on the vacuum level, the cleanliness requirement, and the gases/vapors being pumped. For rough vacuum (lab and medical), copper (Type L, brazed), stainless steel, or approved plastic may be used; medical vacuum per NFPA 99 has specific material and joint requirements. For high vacuum, only materials with low outgassing rates and the ability to be baked are acceptable — typically stainless steel with metal (CF ConFlat) or specific O-ring (KF) flanges — and ordinary threaded or glued joints leak too much for high vacuum. For chemical or corrosive service, the material must resist the pumped vapors (stainless, Hastelloy, or specific plastics for acids and solvents). The trap is using threaded black steel or glued PVC for a vacuum system that leaks too much to hold the required level, or using a material that the process vapors corrode. The disciplined approach is to match material to the vacuum level and the process chemistry, use appropriate joint methods (brazing for medical/lab rough vacuum, metal or specific O-ring flanges for high vacuum), and leak-test the system.

### Size the Receiver and Provide Redundancy for Critical Service

A vacuum receiver (tank) between the pump and the distribution serves as a buffer, smoothing pump cycling and providing a reserve of vacuum for demand surges. For critical service (medical/surgical vacuum), the receiver, the pump redundancy, and the control scheme are specified by code: NFPA 99 requires medical vacuum systems to have redundancy (typically two or more pumps, each sized to handle the full load), a receiver, and alarms for pump failure and low vacuum. The trap is a single-pump, no-receiver system for a critical application, so a pump failure or a demand surge drops the system vacuum below what the process (or the surgical suite) requires. The disciplined approach is to size the receiver for the demand profile, provide pump redundancy for critical service per NFPA 99, and install alarms and indicators that alert staff to pump failure or low vacuum before it affects the process.

### Provide Traps, Filters, and Drains for Contaminants

Vacuum systems pull whatever is at the point of use — liquids, solids, biological material, chemical vapors — back toward the pump, and the pump must be protected. Lab and medical vacuum systems require inlet traps and filters to catch liquids and solids before they reach the pump (a liquid slug can damage an oil-sealed pump, and biological material contaminates the system). Chemical service may require corrosion protection or specific pump types (liquid ring for wet/corrosive loads). The system must include drains at low points to remove condensed liquids. The trap is omitting the inlet protection, so liquids and solids damage the pump or biological/chemical contamination migrates through the system. The disciplined approach is to install appropriate traps, filters, and drains at the pump inlet and at points of use, and to specify a pump type compatible with the expected contaminants (liquid ring for wet loads, oil-mist filters for oil-sealed pumps).

### Respect the Role Limits — Medical and High-Vacuum Need Certification and Specialists

Rough-vacuum industrial and lab systems (non-medical) are within a licensed plumber or pipefitter's scope. Medical/surgical vacuum systems are medical gas systems governed by NFPA 99 and require installation and certification by ASSE 6030 (or equivalent) Medical Gas installers and verifiers, with the work performed under permit and inspection. High-vacuum systems (semiconductor, research) are designed by vacuum specialists and physicists. Confirm scope and certification before taking on vacuum work, and escalate medical and high-vacuum systems to certified and specialized personnel.

## Common Traps

### Pressure-Pipe Sizing on a Vacuum System

The installer sizes the vacuum piping using pressure-pipe flow rules, runs 1/2-inch lines to remote outlets, and the pump holds deep vacuum at the receiver but the remote outlets see almost no usable vacuum. The trap is that vacuum conductance is far more sensitive to diameter and length than pressure flow, and pressure-pipe rules badly undersize vacuum lines. The mechanism is that the long small line's conductance chokes the pump's effective speed at the outlet. The false signal is "the pump holds vacuum." The harm is a system that cannot support the process at the remote points. The defense is to size by conductance and effective pumping speed, keeping runs short and diameters generous.

### Wrong Pump Type for the Vacuum Level

The installer specifies a liquid-ring or rotary-vane rough-vacuum pump for a high-vacuum application, or a costly turbomolecular pump for a rough-vacuum need. The trap is that pump types are categorized by achievable vacuum level, and the wrong type either cannot reach the required level or is wildly over-specified. The mechanism is that each pump mechanism has a physical limit on the vacuum it can produce. The false signal is "it's a vacuum pump." The harm is a system that does not meet the process requirement, or money wasted on the wrong pump. The defense is to determine the required vacuum level and gas load and select the pump type accordingly.

### Threaded or Glued Joints on a High-Vacuum System

The installer uses threaded steel or glued PVC joints on a system that needs to hold high vacuum, and the system cannot reach the required level because every joint leaks. The trap is that high vacuum requires metal or specific elastomer-sealed flanges (CF, KF), and ordinary threaded or glued joints leak too much. The mechanism is that high vacuum is unforgiving of any leak, and threaded/glued joints cannot seal at the molecular level required. The false signal is "the joints are tight." The harm is a system that cannot achieve or hold vacuum. The defense is to use appropriate flanged joints for high vacuum and to leak-test the system with a helium leak detector.

### No Inlet Protection Slugs the Pump

The installer omits the inlet trap and filter, and a liquid slug or biological/chemical contaminant reaches the pump, damaging an oil-sealed pump or contaminating the system. The trap is that vacuum pulls everything at the point of use toward the pump, and the pump is the most expensive and least tolerant component. The mechanism is that liquids damage oil-sealed pumps and solids/contaminants degrade or poison the pump and migrate through the system. The false signal is "the pump is running." The harm is pump damage, system contamination, and process failure. The defense is to install inlet traps, filters, and drains, and to select a pump type compatible with the expected contaminants.

## Self-Check

- Did I size every vacuum line by conductance and effective pumping speed at the most remote outlet (not by pressure-pipe flow rules), keeping runs short and diameters generous?
- Did I select the pump type (rotary vane, liquid ring, dry screw, roots booster, turbomolecular) for the required vacuum level and gas load, not by category alone?
- Did I select piping material and joint method for the vacuum level, cleanliness, and chemical compatibility — using metal or specific O-ring flanges for high vacuum, and code-specified material for medical vacuum?
- For critical or medical service, did I provide pump redundancy, a receiver, and alarms per NFPA 99, so a pump failure or demand surge does not drop the system below the required vacuum?
- Are inlet traps, filters, and drains installed to protect the pump from liquids, solids, and contaminants, and is the pump type compatible with the expected process chemistry?
- Did I leak-test the system (pressure-decay for rough vacuum, helium leak detection for high vacuum) to verify it holds the required level?
- Did I confirm my scope and certification (ASSE 6030 for medical gas) before taking on this work, and did I escalate medical and high-vacuum systems to certified and specialized personnel?
- Are the vacuum-level requirement, conductance calculations, pump selection, material and joint spec, and leak-test results documented for the owner and verifier?
