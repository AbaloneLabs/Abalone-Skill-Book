---
name: systematic-fault-diagnosis.md
description: Use when the agent is troubleshooting an electrical fault, applying the half-split method, selecting and using test instruments correctly, interpreting voltage and current readings, or diagnosing dead circuits, tripped breakers, and malfunctioning equipment.
---

# Systematic Fault Diagnosis

Electrical troubleshooting fails most often not because the electrician lacks knowledge of circuits, but because they abandon systematic method under time pressure and start replacing parts based on guesses. A circuit is dead, a breaker keeps tripping, a motor will not start, and the electrician begins swapping components, resetting breakers, and probing randomly until something appears to work — without ever identifying the actual fault. The result is a repair that fails again because the root cause was never addressed, parts that were needlessly replaced, and in the worst case a new hazard introduced by the random changes. The judgment problem is that systematic diagnosis feels slow while guessing feels fast, but the systematic approach finds the fault in fewer total steps and produces a repair that lasts, while guessing often creates a longer outage and a recurring problem. This skill covers the methodology and instrument discipline that make fault diagnosis reliable rather than a series of hopeful stabs.

## Core Rules

### Gather Symptoms and Define the Fault Before Touching Anything

Before opening a panel or testing a wire, define exactly what the fault is and what conditions surround it. What is not working, and what is still working? When did it start, and what changed at that time? Is the breaker tripped, and will it hold when reset, or does it trip immediately? Is the fault intermittent or constant? Does it affect one circuit, one phase, or the whole system? Gathering this information narrows the search before any test is performed. The trap is rushing to the first plausible hypothesis without defining the fault boundary, which leads to testing in the wrong area and replacing parts that are not defective. The defense is to interview the user, observe the symptoms, map what works and what does not, and write down a specific fault statement before beginning any testing.

### Apply the Half-Split Method to Isolate the Fault Efficiently

The half-split method (also called divide-and-conquer) is the most efficient way to isolate a fault in a linear circuit or system. Instead of testing point by point from one end, the electrician tests at the midpoint of the suspected fault path. If the signal or voltage is correct at the midpoint, the fault is downstream of that point; if it is absent or incorrect, the fault is upstream. Each test eliminates half of the remaining circuit from consideration, converging on the fault in logarithmically few steps. The trap is testing sequentially from the source, which works but is slow on long circuits, or testing at random points based on guesses, which can miss the fault entirely or chase red herrings. The defense is to identify the signal path, test at the logical midpoint, and use each result to halve the search space until the fault is isolated to a specific component or connection.

### Verify the Test Instrument Before Trusting Its Reading

Every measurement is only as good as the instrument that produces it, and a malfunctioning tester produces a confident but wrong reading that can send the diagnosis in entirely the wrong direction. Before relying on a voltage reading, verify the tester on a known-live source (the live-dead-live principle). Before relying on a continuity reading, verify the tester by touching the leads together. A low-battery meter, a blown fuse in the meter, a broken test lead, or a meter set to the wrong range all produce false readings that look authoritative. The trap is trusting a reading from an unverified instrument, particularly a zero voltage reading that could mean either "dead circuit" or "dead tester." The defense is to verify the instrument on a known source before and after the critical measurement, and to treat any unverified reading as unreliable.

### Measure Under Load to Reveal High-Resistance Faults

Many faults — loose connections, corroded terminals, partially failed switches, undersized conductors — only appear when current flows. A high-resistance connection may read full voltage with a high-impedance meter (which draws essentially no current), because the voltage drop across the connection is negligible at near-zero current. But when the load is connected and current flows, the high-resistance connection drops voltage, the load receives reduced voltage, the connection heats, and the symptoms appear. The trap is measuring voltage at a receptacle or terminal with the load disconnected, reading full voltage, and concluding the supply is good — when the fault only manifests under load. The defense is to measure voltage with the load operating, to use a load (such as a hair dryer or test load) that draws significant current, and to compare the loaded and unloaded voltage to reveal the voltage drop caused by a high-resistance connection.

### Use the Right Instrument for the Measurement

Different measurements require different instruments, and using the wrong one produces misleading results. A digital multimeter with high input impedance is correct for measuring voltage in electronic circuits, but it can read full voltage through a very high-resistance path (a "phantom voltage") that cannot deliver current, leading to a false conclusion that the circuit is live. A low-impedance voltage tester (a solenoid tester or a loaded tester) draws current and will collapse a phantom voltage, revealing whether the voltage can actually support a load. A clamp meter measures current without breaking the circuit. An insulation tester (megger) applies high voltage to measure insulation resistance, revealing degradation that a continuity test cannot. The trap is using a single instrument for everything and misinterpreting its readings. The defense is to select the instrument matched to the measurement, to understand what each instrument can and cannot reveal, and to cross-check ambiguous readings with a second instrument type.

### Confirm the Fault Is Found Before Repairing, and Verify After Repairing

When a fault is suspected to be at a specific point, confirm it with a measurement before replacing or repairing anything. Do not replace a component because it "might be bad" — prove it is bad or prove the circuit downstream is good. After the repair, verify that the fault is gone by reproducing the original test conditions and confirming the symptom has resolved. The trap is replacing a part based on suspicion, having the circuit work (perhaps coincidentally, or because a connection was disturbed during the work), and concluding the replaced part was the fault — when the actual fault was a loose connection that was inadvertently tightened during the work and will loosen again. The defense is to confirm the fault location with a measurement before repair, and to verify the repair under the same conditions that revealed the fault, so that the resolution is proven, not assumed.

## Common Traps

### Guessing and Swapping Parts Instead of Diagnosing

A motor will not start, and the electrician immediately replaces the contactor, then the overload, then the motor, without measuring voltage at the motor terminals or checking the control circuit. The motor still does not start, and now three good parts have been replaced at the customer's expense. The trap is that replacing parts feels like progress and occasionally works by luck, reinforcing the habit. The mechanism of harm is that the actual fault — a broken wire, a failed control relay, a tripped auxiliary — is never found, the parts replacement did not fix it, and the outage continues while the cost mounts. The false signal is that each replacement "might be the one." The defense is to refuse to replace any part without a measurement proving it is defective, and to follow the half-split method to isolate the fault before touching any component.

### Trusting a Voltage Reading From an Unverified Meter

An electrician reads zero volts on a circuit with a digital multimeter and concludes the circuit is de-energized or dead. But the meter's fuse is blown, or the battery is low, or a test lead is broken at the insulation. The zero reading is false, and the diagnosis proceeds on the assumption that the circuit has no voltage, when in fact it is live. The trap is that the meter displayed a number confidently, and zero felt like a clear result. The mechanism of harm is that the diagnosis goes in the wrong direction — the electrician searches for an open circuit or a failed breaker when the circuit is actually energized, wasting time and potentially creating a shock hazard if they proceed to work on the "dead" circuit. The false signal is the authoritative-looking digital display. The defense is to verify the meter on a known-live source before and after the measurement (live-dead-live), and never to trust an unverified zero reading.

### Measuring Voltage Unloaded and Missing a High-Resistance Fault

An electrician measures 120 volts at a receptacle with nothing plugged in, and concludes the receptacle and circuit are good. But when a load is plugged in, the voltage drops to 90 volts and the load malfunctions. The trap is that the unloaded measurement showed full voltage because the high-impedance meter draws no current, and the high-resistance fault (a loose splice upstream) only drops voltage when current flows. The mechanism of harm is that the fault is missed, the customer is told the circuit is fine, and the malfunction continues or worsens as the high-resistance connection heats and degrades. The false signal is the full voltage reading at the receptacle. The defense is to measure voltage with a load connected, to compare loaded and unloaded readings, and to suspect a high-resistance connection whenever the loaded voltage is significantly lower than the unloaded voltage.

### Chasing a Phantom Voltage With a High-Impedance Meter

An electrician measures 60 volts to ground on a wire that should be de-energized, using a high-impedance digital multimeter. The reading is stable and looks real, so the electrician searches for a source feeding this wire — chasing a cross-connection that does not exist. The trap is that the high-impedance meter reads capacitive or inductive coupling from adjacent energized wires, producing a "phantom voltage" that cannot deliver any current. The mechanism of harm is wasted diagnostic time chasing a non-existent fault, and in the worst case a wrong conclusion that a circuit is energized when it is not (or vice versa). The false signal is the stable voltage reading on the meter. The defense is to re-measure with a low-impedance tester (a solenoid tester or a loaded tester), which draws current and collapses the phantom voltage to near zero if it is coupling-induced, confirming whether the voltage can support a load.

### Declaring the Repair Complete Without Verifying Under Original Conditions

An electrician tightens a loose connection in a junction box, closes the box, and reports the circuit repaired, without re-testing the circuit under the load that originally revealed the fault. The circuit works initially, but the underlying cause — a thermal intermittent or a failing conductor — resurfaces later. The trap is that the symptom appeared to resolve when the connection was tightened, so the repair felt complete. The mechanism of harm is that the actual fault was not fully addressed, the customer experiences a recurrence, and confidence in the repair is lost. The false signal is that the circuit worked when the electrician left. The defense is to reproduce the original fault conditions after the repair — apply the same load, run for the same duration, recreate the same operating state — and confirm the symptom is gone before declaring the repair complete.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I define the fault boundary — what works, what does not, when it started, and whether it is constant or intermittent — before performing any test?
- Did I apply the half-split method to isolate the fault, testing at logical midpoints and using each result to narrow the search space, rather than testing sequentially or randomly?
- Did I verify my test instrument on a known source before and after the critical measurement, and treat any unverified reading as unreliable?
- Did I measure voltage and current under load, not just unloaded, to reveal high-resistance faults that only manifest when current flows?
- Did I select the right instrument for the measurement — a low-impedance tester to collapse phantom voltages, a clamp meter for current, an insulation tester for insulation degradation?
- Did I confirm the fault location with a measurement before replacing or repairing any component, rather than swapping parts based on suspicion?
- Did I verify the repair by reproducing the original fault conditions and confirming the symptom is resolved, rather than declaring completion based on the circuit appearing to work?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
