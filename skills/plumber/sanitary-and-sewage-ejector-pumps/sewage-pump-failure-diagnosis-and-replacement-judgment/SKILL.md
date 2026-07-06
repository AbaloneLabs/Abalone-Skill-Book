---
name: sewage-pump-failure-diagnosis-and-replacement-judgment.md
description: Use when the agent is diagnosing a sewage ejector pump that will not run, distinguishing float, capacitor, thermal overload, and sealed-chamber leak failures, analyzing cycling frequency, deciding whether to repair or replace based on age and cost ratio, or pulling a pump safely with confined space, lockout-tagout, and contamination PPE.
---

# Sewage Pump Failure Diagnosis and Replacement Judgment

A sewage pump that will not run is a diagnostic problem with several distinct causes, and misdiagnosis produces a replaced pump that fails again for the same reason, or a repair on a unit that should have been replaced. The judgment problem is that the symptoms overlap — a pump that hums and trips, a pump that does nothing, a pump that short-cycles — and each points to a different fault (float, run capacitor, thermal overload, sealed-chamber leak) with a different fix. Agents also misjudge the repair-versus-replace decision, rebuilding an old pump whose repair cost approaches replacement, and they pull a pump from a sewage basin without treating the basin as a confined, contaminated, electrically energized hazard. This skill covers the no-run diagnostic tree, run-versus-start capacitors, cycling analysis, the repair-or-replace judgment, and safe pump-pulling with confined-space, LOTO, and contamination controls.

## Core Rules

### Diagnose a No-Run Pump Through the Float, Power, Capacitor, and Overload Tree

A sewage pump that will not run is diagnosed in order of likelihood and safety. First, isolate power and verify the float: a tethered or vertical float may be stuck, tangled, or failed open/closed — manually lift the float (with power isolated at the breaker, then restored briefly) to see if the pump responds; if not, the float or the switch is suspect. Second, confirm power at the pump (voltage at the cord or junction) and that the breaker has not tripped. Third, if the pump hums but does not start, suspect the run or start capacitor (a weak or failed capacitor cannot bring the motor to speed) or a jammed impeller (solids or string around the shaft). Fourth, if the pump ran and stopped, suspect thermal overload (the motor overheated and tripped its internal protector; it will reset when cool) or a sealed-chamber leak (water in the motor housing, tripping the leak sensor or shorting the windings). The trap is condemning the pump at step one. The disciplined rule is to work the diagnostic tree: float, power, capacitor, overload, leak.

### Distinguish Run Capacitor, Start Capacitor, and Thermal Overload Failures

Pumps with capacitor-run or capacitor-start motors fail predictably. A failed run capacitor causes the motor to hum, draw high current, and trip the breaker or run hot without reaching speed; a failed start capacitor (on capacitor-start motors) prevents the motor from coming up to speed under load. A capacitor that is swollen, leaking, or out of capacitance value (measured with a meter) is the fault, and replacement is cheap and often curative. Thermal overload trips when the motor overheats from short-cycling (too-small basin), high head (clogged or undersized discharge), or low voltage; the overload resets when cool, but the underlying cause must be fixed or the failure recurs. The trap is replacing the pump when a capacitor or a thermal cause is the fault. The disciplined rule is to test the capacitor with a meter, check for thermal causes (cycling frequency, head, voltage), and replace the capacitor or fix the cause before condemning the pump.

### Analyze Cycling Frequency to Find the Hidden Cause of Repeated Failure

A pump that fails repeatedly is usually being killed by an operating condition, not a defect. Count the cycles per hour (starts per hour): a pump short-cycling (more than the manufacturer's maximum starts per hour, often 10 to 20 for small motors) overheats the motor and wears the capacitor and contacts, and the cause is an undersized basin (too little active volume) or a leaking check valve (the column drains back and re-triggers the pump). A pump running too long per cycle may be fighting high head (clogged discharge, undersized pipe) or low voltage. A pump that runs but does not move sewage may have a worn impeller or a jam. The trap is replacing the pump without measuring the cycling. The disciplined rule is to measure the starts per hour and run time, find the operating cause (basin size, check valve, head, voltage), and fix it with the pump.

### Decide Repair Versus Replace by Age, Cost Ratio, and Code Upgrades

The repair-or-replace decision weighs the pump age, the repair cost as a fraction of replacement, and any code or capacity upgrades due. A pump under its expected service life (commonly 7 to 10 years for residential sewage ejectors, longer for commercial) with a single cheap failure (a capacitor, a float) is a repair candidate. A pump near or past its service life, or with a major failure (shorted windings, leaking sealed chamber, cracked housing), or whose repair cost exceeds roughly half the replacement cost, is a replace candidate. Replacement is also the call when the pump is undersized for added fixtures, when the basin lacks simplex-to-duplex redundancy the code now expects, or when the unit no longer meets current code (solids handling, venting, alarm). The trap is rebuilding a pump that will fail again soon. The disciplined rule is to compare repair cost to replacement, factor age and code upgrades, and replace when the ratio or the age favors it.

### Pull the Pump Safely: Confined Space, LOTO, and Contamination PPE

Pulling a pump from a sewage basin is a confined-space-adjacent, contaminated, and electrically hazardous task. Lock out and tag out the pump circuit at the breaker before any work, and verify zero voltage at the pump. Treat the basin as a contaminated space: wear impermeable gloves, eye protection, and clothing that will be decontaminated or discarded; avoid splash and aerosol exposure (do not pressure-wash the basin contents). For a deep basin or a vault, treat it as a confined space — test the atmosphere, ventilate, and have an attendant if entry is required (most pump pulls are done from above without entry, using the lifting chain or rope). Have a containment area ready for the pulled pump (it is sewage-contaminated), and disinfect tools and surfaces after. The trap is yanking the pump live and bare-handed. The disciplined rule is to LOTO, verify dead, use contamination PPE, avoid entry where possible, and decontaminate after.

## Common Traps

### Condemning the Pump Without Testing the Float

The plumber sees the pump is not running and orders a replacement, without checking the float. The trap is that the float is the most common failure and is cheap to fix. The mechanism is that a stuck, tangled, or failed float opens or closes the circuit. The false signal is that "the pump is dead." The harm is an unnecessary replacement and a recurring float problem. The defense is to isolate power, manually lift the float, restore power briefly, and confirm whether the pump responds before condemning it.

### Replacing the Pump When a Capacitor or Thermal Cause Is the Fault

The plumber replaces a humming pump instead of testing the capacitor or the thermal condition. The trap is that the capacitor is a cheap, common failure. The mechanism is that a failed run or start capacitor prevents the motor from reaching speed. The false signal is that "it hums, so the motor is bad." The harm is an unnecessary replacement and a new pump that may face the same thermal cause. The defense is to test the capacitor with a meter, check for thermal causes (cycling, head, voltage), and replace the capacitor or fix the cause first.

### Replacing the Pump Without Measuring the Cycling That Killed It

The plumber swaps a failed pump without checking why it failed. The trap is that the operating condition (short-cycling, high head, low voltage) will kill the new pump too. The mechanism is that repeated starts or overload wear the motor and capacitor. The false signal is that "the new pump will fix it." The harm is repeat failure of the new pump. The defense is to measure starts per hour and run time, find the operating cause (basin size, check valve leak, head, voltage), and fix it with the pump.

### Rebuilding an Old Pump Whose Repair Cost Approaches Replacement

The plumber rebuilds a 12-year-old pump with a major failure because "the parts are available." The trap is that the remaining life is short and the repair cost is high. The mechanism is that age and a major failure (shorted windings, cracked housing) make further failure likely. The false signal is that "it can be rebuilt." The harm is a repaired pump that fails again soon, costing more over time. The defense is to compare repair cost to replacement (replace at roughly half), factor age and code upgrades, and replace when the ratio or age favors it.

### Pulling the Pump Live and Bare-Handed Without LOTO or Contamination PPE

The plumber yanks the pump with the breaker on and no gloves, standing over an open sewage basin. The trap is that the basin is contaminated and the circuit is energized. The mechanism is that live work risks shock and the sewage risks infection and splash exposure. The false signal is that "it's a quick pull." The harm is electrical injury, infection, or aerosol exposure. The defense is to LOTO at the breaker, verify zero voltage, wear impermeable gloves and eye protection, avoid basin entry where possible, and decontaminate tools and surfaces after.

## Self-Check

- Did I work the no-run diagnostic tree in order — float (isolated, manually lifted), power at the pump, capacitor, thermal overload, sealed-chamber leak — before condemning the pump?
- Did I test the run and start capacitors with a meter (checking value, swelling, leakage) and replace a failed capacitor before replacing the pump?
- Did I check for thermal-overload causes — short-cycling (starts per hour), high head (clogged or undersized discharge), low voltage — and fix the underlying condition?
- Did I measure the cycling frequency (starts per hour and run time) to find the operating cause of repeated failure, rather than assuming a defect?
- In the repair-or-replace decision, did I weigh pump age against expected service life (commonly 7 to 10 years residential), repair cost as a fraction of replacement (replace at roughly half), and any code or capacity upgrades due?
- Did I lock out and tag out the pump circuit at the breaker and verify zero voltage at the pump before any pulling work?
- Did I treat the basin as a contaminated space — impermeable gloves, eye protection, splash and aerosol avoidance, and a containment area for the pulled pump?
- For a deep basin or vault, did I treat it as a confined space — atmosphere test, ventilation, and an attendant if entry is required — and pull from above using the lifting chain or rope where possible?
- Did I disinfect tools and surfaces after the pull, and dispose of or decontaminate contaminated PPE?
- Did I document the diagnosed fault, the measurement (cycling, voltage, capacitance), the repair-or-replace rationale, and the LOTO and PPE controls in the service record?
