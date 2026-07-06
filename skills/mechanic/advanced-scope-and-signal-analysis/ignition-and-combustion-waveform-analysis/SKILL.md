---
name: ignition-and-combustion-waveform-analysis.md
description: Use when the agent is scoping ignition primary or secondary waveforms, analyzing coil-on-plug or distributor ignition events, diagnosing misfire through spark line and burn time, evaluating compression waveform or in-cylinder pressure traces, or interpreting ignition KV, dwell, and firing voltage on engine performance diagnosis.
---

# Ignition and Combustion Waveform Analysis

Ignition waveform analysis is the scope technique that turns a misfire or a performance complaint into a visible picture of what is happening in the cylinder at the moment of combustion, and it is the skill that separates the technician who proves the cause from the one who swaps coils. The judgment problem is that an ignition waveform carries information about the spark (the firing voltage, the spark line, the burn time), the cylinder (the compression, the mixture, the fuel quality), and the ignition system (the coil, the plug, the wire), and a technician who reads only "is there spark" misses the diagnosis the waveform is offering. A technician who replaces a coil for what is a lean cylinder, or a plug for what is low compression, hands back a vehicle with the same misfire. This skill covers the disciplined capture and interpretation of ignition and combustion waveforms.

## Core Rules

### Understand the Anatomy of an Ignition Waveform: Firing Voltage, Spark Line, Burn Time

The disciplined scope technician reads an ignition waveform in three regions, each carrying different information. The firing voltage (the tall spike at the start of the event, often 5 to 20 kV on a secondary trace) is the voltage required to ionize the gap and initiate the arc — a high firing voltage indicates a wide gap, high compression, a lean mixture, or a fouled plug; a low firing voltage indicates a low-compression cylinder, a rich mixture, or a shorted plug. The spark line (the near-horizontal segment after the firing voltage) shows the voltage sustaining the arc — a clean, stable spark line at a few hundred volts indicates a healthy burn; a jagged or oscillating spark line indicates turbulence or a marginal arc. The burn time (the duration of the spark line, typically 0.8 to 2.0 milliseconds) is the time the arc is sustained — a short burn time indicates the energy dumped too fast (a wide gap, a lean mix), a long burn time indicates a rich or low-compression condition. The disciplined technician compares all cylinders on a raster or overlay display and reads the cylinder that deviates, because the misfiring or weak cylinder stands out from the others.

### Compare Cylinders Side by Side to Find the Outlier

The most powerful ignition-scope technique is the multi-cylinder comparison, because a single waveform in isolation has no reference, while a waveform compared to its neighbors reveals the problem cylinder instantly. The disciplined approach captures all cylinders (on a distributor system with an amp probe around the coil wire or a secondary pickup on each wire; on a coil-on-plug system with a sync probe and a cylinder identifier), displays them overlaid or in a raster, and identifies the cylinder whose firing voltage, spark line, or burn time deviates from the others. A single high firing voltage points to that cylinder's plug, gap, or compression; a single short burn time points to a lean injector or a vacuum leak at that cylinder; a uniformly high firing voltage across all cylinders points to a common cause (a worn-out set of plugs, a lean condition, a bad batch of fuel). The tradeoff is that the multi-cylinder setup takes time, but it is the only way to find the outlier that a single trace hides.

### Use the Firing Voltage to Distinguish Lean, Rich, Compression, and Plug Faults

The firing voltage is the single most diagnostic value on an ignition waveform, and the disciplined technician reads it to separate the causes of a misfire or a weak cylinder. A high firing voltage on one cylinder points to a wide plug gap (wear), a cracked porcelain (the arc jumps an internal crack), a lean mixture (a clogged injector, a vacuum leak at the intake port), or high compression (carbon buildup) — each raises the pressure or the resistance the arc must overcome. A low firing voltage on one cylinder points to a fouled plug (oil or carbon shorting the gap), a rich mixture (an over-fueling injector), or low compression (worn rings, a burned valve, a leaky head gasket) — each lowers the resistance. The disciplined technician pairs the firing voltage with a compression or leak-down test and a fuel-trim check to confirm the cause, because the waveform points to the cylinder and the category, and the confirming test names the component.

### Evaluate Coil-on-Plug Primary and Secondary Waveforms and the Coil Ramp

On a coil-on-plug (COP) system, the disciplined scope captures either the primary (the low-voltage command at the coil) or the secondary (the high-voltage output, with a capacitive or inductive pickup). The primary waveform shows the coil current ramp (the ramping current as the coil charges), the dwell (the charge time), and the collapse (the sharp drop as the field collapses and induces the spark) — a ramp that does not reach the expected current indicates a coil with high internal resistance or a weak driver; a dwell that is wrong indicates a command or a module fault. The secondary waveform shows the firing voltage and the burn. The disciplined COP diagnosis uses a sync probe (a reference pickup on cylinder one) to identify the firing order and capture each coil in sequence, and it compares the coils to find the weak one. The tradeoff is that COP scoping requires the right probes and a known firing order, but it diagnoses a coil without swapping parts.

### Use the In-Cylinder Pressure Trace (Compression Waveform) for Dynamic Compression

An in-cylinder pressure transducer, screwed into the spark plug hole, captures the compression waveform while the engine cranks or runs, and it reveals dynamic compression, valve timing, and leaks that a static compression test cannot. The disciplined interpretation reads the compression peak (relative to the other cylinders), the expansion pocket (the shape of the power or compression stroke reveals valve opening events — a rounded pocket indicates a leaking valve, a sharp early drop indicates a leaking ring), and the cranking vacuum pulses. The tradeoff is that a pressure transducer is a specialty tool, but it diagnoses a mechanical cylinder fault (burned valve, slipped timing, leaking rings) without teardown.

## Common Traps

### Reading Only "Is There Spark" and Missing the Waveform Detail — The technician confirms the coil fires and declares the ignition good, but the spark line is jagged and the burn time is short, indicating a lean cylinder that still misfires. The trap mechanism is that the presence of spark is not the health of the burn, and a marginal arc misfires under load. The false signal is the spark "being there"; the harm is a misfire misdiagnosed as ignition-good. The disciplined technician reads the firing voltage, spark line, and burn time, not just the presence of the event.

### Condemning a Coil for a Lean Cylinder — One cylinder has a high firing voltage, the technician replaces the coil, and the high voltage persists because the cause was a clogged injector or a vacuum leak making the cylinder lean. The trap mechanism is that a lean mixture raises the firing voltage (lean mixtures are harder to ionize), and the coil is the easy target. The false signal is the high firing voltage pointing at the coil; the harm is an unnecessary coil. The disciplined technician pairs the high firing voltage with a fuel-trim and injector check.

### Replacing a Plug for Low Compression — One cylinder has a low firing voltage, the technician replaces the plug, and the low voltage persists because the cylinder has low compression (a burned valve, worn rings). The trap mechanism is that low compression lowers the firing voltage (less resistance), and the plug is the easy target. The false signal is the low firing voltage pointing at the plug; the harm is an unnecessary plug. The disciplined technician pairs a low firing voltage with a compression or leak-down test.

### Scoping One Cylinder in Isolation Without a Reference — The technician captures one cylinder's waveform, sees something that "looks off," and condemns the coil — but without comparing to the other cylinders, there is no reference, and the "off" may be normal. The trap mechanism is that a single trace has no baseline, and the technician reads noise as a fault. The false signal is the trace "looking different"; the harm is a misdiagnosis. The disciplined technician compares all cylinders to find the outlier.

### Ignoring the Coil Current Ramp and Condemning the Driver — The coil fires weakly, the technician condemns the ECM driver, but the real cause is a coil with high internal resistance that never reaches full current. The trap mechanism is that the primary ramp shows the coil's health, and a weak ramp is the coil, not the driver. The false signal is the weak spark pointing at the driver; the harm is an unnecessary ECM. The disciplined technician reads the coil current ramp before the driver.

## Self-Check

- Did I read the firing voltage, the spark line, and the burn time of each cylinder, not just the presence of the spark event?
- Did I capture and compare all cylinders overlaid or in a raster to find the outlier, rather than scoping one cylinder in isolation?
- For a high firing voltage, did I check the plug gap, the porcelain, the injector (lean), and the compression before the coil?
- For a low firing voltage, did I perform a compression or leak-down test before replacing the plug or the coil?
- On a coil-on-plug system, did I use a sync probe and the correct firing order, and read the primary coil current ramp and dwell?
- Did I distinguish a lean cylinder (high firing voltage, short burn) from a rich cylinder (low firing voltage, long burn) from a compression fault?
- For a suspected mechanical cylinder fault, did I use an in-cylinder pressure transducer to capture the dynamic compression and valve events?
- Did I save and document the waveform captures to support the diagnosis and the repair?
