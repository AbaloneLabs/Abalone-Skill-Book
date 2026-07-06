---
name: prv-selection-sizing-and-failure-diagnosis.md
description: Use when the agent is selecting or sizing a pressure-reducing valve (PRV) for a building water service, diagnosing low or high water pressure, identifying PRV failure modes (clogging, seat wear, creep, strainer fouling), setting outlet pressure within the 80 psi code limit, or evaluating thermal expansion interaction in a closed system.
---

# PRV Selection Sizing and Failure Diagnosis

A pressure-reducing valve (PRV) is the device that steps high or fluctuating municipal inlet pressure down to a safe, steady building pressure, and sizing or diagnosing it wrong damages every fixture downstream. The judgment problem is that a PRV is too often chosen by pipe size alone, ignoring flow demand and inlet pressure range, so the valve is either oversized (hunting, seat wear, low-flow instability) or undersized (pressure collapse when two fixtures open). Diagnosis is equally error-prone: a "low pressure" complaint is blamed on the PRV when the real cause is a fouled strainer, a partially closed valve upstream, or the municipal supply itself, and a PRV that creeps (outlet pressure slowly rising toward inlet) is missed because the symptom is a weeping T&P valve, not a pressure gauge. This skill covers PRV sizing by flow and inlet range, the 80 psi code ceiling, failure-mode diagnosis, outlet-pressure setting, and the thermal-expansion interaction that a closed system forces.

## Core Rules

### Size the PRV by Flow Demand and Inlet Pressure Range, Not Pipe Size Alone

A PRV must pass the building's peak flow (the sum of probable simultaneous fixture demands, in GPM) at the available inlet pressure while maintaining the set outlet pressure. Manufacturer flow curves give the capacity at a given inlet-to-outlet differential; selecting by nominal pipe size ignores that a 1-inch valve may only deliver 12 GPM at a 40 psi differential while a different 1-inch body delivers 25 GPM. Undersizing shows up as pressure collapse when multiple fixtures open; oversizing causes low-flow instability and seat damage because the valve chatters near closed. The inlet pressure range matters too: a valve must handle both the static (no-flow) inlet, which can spike far above the dynamic, and the minimum dynamic inlet during peak municipal draw. The trap is matching the pipe size and walking away. The disciplined rule is to calculate peak GPM, confirm the inlet pressure range with a gauge, and select a valve whose flow curve meets demand across that range.

### Respect the 80 psi Maximum Code Limit and Set Outlet Pressure Conservatively

The International Plumbing Code (IPC 604.8) and Uniform Plumbing Code (UPC) cap maximum static water pressure at fixtures at 80 psi, requiring a PRV where supply exceeds it. Setting outlet pressure at or near 80 psi is legal but unwise: pressure spikes from water hammer and pump starts overshoot the static setting, and sustained high pressure stresses toilet fill valves, supply tubes, and appliance hoses toward early failure. A typical set point is 50 to 60 psi, which delivers adequate flow at fixtures while leaving margin below the 80 psi ceiling and the T&P relief threshold on water heaters. The trap is cranking the PRV to 75 to 80 psi to satisfy a "low pressure" complaint that is really a flow restriction. The disciplined rule is to set outlet pressure in the 50 to 60 psi range, to verify with a gauge at a nearby hose bibb, and to fix flow restrictions rather than raising pressure.

### Diagnose Low Pressure by Isolating PRV, Supply, and Strainer Before Condemning the Valve

A low-pressure complaint has at least three common causes: a failing PRV, a restricted municipal supply, or a fouled PRV strainer (or upstream shutoff not fully open). Diagnose in order: measure static and dynamic pressure at a hose bibb upstream of the PRV (or at the meter) to confirm the supply; if supply is good, check the PRV strainer — a clogged screen collapses pressure under flow while reading normal at no flow; if the strainer is clean, measure outlet static and dynamic pressure to see if the PRV holds set under flow. A PRV that reads normal at static but drops sharply under flow has a clogged strainer or a worn seat; one that reads low at static has failed closed or is misadjusted. The trap is replacing the PRV without checking the strainer or the supply. The disciplined rule is to measure upstream and downstream, clean the strainer, and confirm the supply before replacing the valve.

### Recognize the Failure Modes: Clogging, Seat Wear, and Creep

PRVs fail in characteristic ways. Clogging (strainer fouled with sediment, scale, or debris from main work) starves flow and drops dynamic pressure while static looks normal. Seat wear (from erosion or chatter) prevents the valve from holding a steady outlet under varying flow, producing pressure that wanders with demand. Creep (outlet pressure slowly rising toward inlet over minutes to hours) is the most dangerous because it is silent: the symptom is a water heater T&P valve weeping, not a pressure complaint, because the closed system has nowhere to absorb the rising pressure. Diagnose creep by installing a gauge downstream, running a fixture, closing all flow, and watching the gauge climb over 15 to 60 minutes. The trap is treating a weeping T&P as a water-heater problem and missing the creeping PRV. The disciplined rule is to test for creep on any T&P-weep call, and to replace a PRV that creeps rather than adjusting it.

### Account for the Closed-System Thermal Expansion Interaction

A PRV (and any check valve or backflow preventer) creates a closed system: heated water from the water heater cannot expand back into the municipal main, so thermal expansion drives pressure up until the T&P relieves at 150 psi or 210°F. Every PRV installation on a system with a storage water heater therefore requires a thermal expansion tank sized to the heater volume and set pressure, installed on the cold side. A PRV set at 60 psi with no expansion tank will routinely push the system to 150 psi as the heater recovers, weeping the T&P and stressing every fitting. The trap is installing or replacing a PRV without checking for an expansion tank. The disciplined rule is to verify a functioning, correctly sized and precharged expansion tank on every closed system, and to add one where the PRV creates a closed system that lacks one.

## Common Traps

### Sizing the PRV by Pipe Size and Ignoring the Flow Curve

The plumber matches the PRV to the pipe size and installs it, without checking the flow curve against peak demand. The trap is that two 1-inch valves can have very different capacities. The mechanism is that capacity depends on body geometry and the inlet-to-outlet differential, not nominal size. The false signal is that "it fits the pipe." The harm is pressure collapse when multiple fixtures open, or low-flow chatter that wears the seat. The defense is to calculate peak GPM, confirm the inlet pressure range, and select from the manufacturer flow curve.

### Raising Outlet Pressure to 80 psi to Fix a Flow Restriction

The plumber responds to a low-pressure complaint by raising the PRV set point toward the 80 psi ceiling. The trap is that the complaint is usually a flow restriction (a fouled strainer, a partially closed valve, undersized piping), not low static pressure. The mechanism is that raising static pressure does not fix a dynamic-flow bottleneck and stresses the system. The false signal is that "more pressure fixes everything." The harm is fixture, hose, and supply-tube failures and a system running at the code ceiling with no margin. The defense is to diagnose the flow restriction, set the PRV at 50 to 60 psi, and verify with a gauge.

### Replacing the PRV Without Checking the Strainer or the Supply

The plumber condemns the PRV on a low-pressure call and replaces it, without measuring upstream pressure or cleaning the strainer. The trap is that the strainer or the supply was the cause. The mechanism is that a fouled strainer collapses dynamic pressure while static reads normal, and a weak supply affects everything downstream. The false signal is that "the PRV is the pressure device, so it must be the problem." The harm is an unnecessary replacement that does not fix the restriction. The defense is to measure static and dynamic pressure upstream and downstream, clean the strainer, and confirm the supply before replacing.

### Missing Creep Because the Symptom Is a Weeping T&P

The plumber responds to a weeping water-heater T&P valve, replaces the T&P, and leaves, without testing the PRV for creep. The trap is that the T&P is weeping because the PRV is creeping the closed system up to 150 psi. The mechanism is that a worn PRV seat lets inlet pressure bleed into the outlet when flow stops. The false signal is that "the T&P was bad." The harm is recurrence (the new T&P weeps) and chronic overpressure stressing every fitting. The defense is to test for creep with a downstream gauge on every T&P-weep call, and to replace a creeping PRV.

### Installing a PRV Without a Thermal Expansion Tank on the Closed System

The plumber installs or replaces a PRV, creating a closed system, without verifying a thermal expansion tank. The trap is that heated water can no longer expand into the main, so pressure spikes to T&P relief on every recovery. The mechanism is that the PRV's check action traps the expanding water. The false signal is that "the PRV is working, pressure is set." The harm is chronic T&P weeping, fitting stress, and eventual failure. The defense is to verify a correctly sized and precharged expansion tank on every closed system, and to add one where the PRV creates a closed system without one.

## Self-Check

- Did I size the PRV from the manufacturer flow curve against the building's peak GPM and the confirmed inlet pressure range, rather than by pipe size alone?
- Is the outlet pressure set in the 50 to 60 psi range, verified with a gauge at a nearby hose bibb, and below the 80 psi code maximum with margin for spikes?
- On a low-pressure complaint, did I measure static and dynamic pressure upstream of the PRV to confirm the municipal supply before condemning the valve?
- Did I inspect and clean the PRV strainer, and confirm it is not the cause of dynamic pressure collapse?
- Did I distinguish the failure mode — clogging (strainer), seat wear (wandering outlet under flow), or creep (outlet climbing toward inlet at no flow) — with the right test?
- On any T&P-weep call, did I test the PRV for creep with a downstream gauge over 15 to 60 minutes rather than assuming the T&P failed?
- Did I verify that the closed system created by the PRV has a correctly sized and precharged thermal expansion tank on the cold side?
- Is the PRV installed with an accessible shutoff upstream and downstream, and a strainer for service?
- Did I confirm the inlet pressure range handles both static spikes and minimum dynamic draw, not just the average?
- Did I record the set outlet pressure, the upstream supply reading, the strainer condition, and the expansion-tank status in the service record?
