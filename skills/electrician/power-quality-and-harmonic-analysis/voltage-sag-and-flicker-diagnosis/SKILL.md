---
name: voltage-sag-and-flicker-diagnosis.md
description: Use when the agent is diagnosing voltage sags, swells, and flicker in power systems, identifying their source, interpreting ITIC and SEMI curves, and specifying mitigation such as UPS, DVR, or static switches.
---

# Voltage Sag and Flicker Diagnosis

Voltage sags, swells, and flicker are among the most expensive power quality problems because they cause process trips, contactor dropouts, and visible lighting disturbance without leaving obvious damage. The judgment problem is that the symptom (a process stopped, lights dimmed) is easy to record but the cause is often remote, intermittent, and shared with other customers on the same feeder. Agents tend to chase the nearest load or assume the utility is at fault, when the actual source may be a motor start across the plant, a fault on an adjacent feeder, or an arc furnace miles away. Correct diagnosis requires matching the event shape, duration, and timing to candidate sources, then specifying mitigation sized to the actual disturbance, not the assumed one.

## Core Rules

### Capture the Event Shape, Duration, and Timing Together
A voltage sag is defined by its residual voltage (typically 10 to 90 percent of nominal) and its duration (half cycle to one minute). The diagnostic value comes from the combination: a sag to 70 percent for three cycles suggests a remote fault cleared quickly; a sag to 50 percent for 200 milliseconds suggests a motor start; a swell above 110 percent suggests capacitor switching or load shedding. Always record with a power quality monitor capable of capturing half-cycle rms values and waveform traces, because averaged or slow-sampled data hides the event entirely. Without the waveform shape, source identification is guesswork.

### Distinguish Source-Side From Load-Side Events
The decisive test is whether the sag is accompanied by a current increase or decrease at the monitoring point. If current rises as voltage sags, the disturbance is load-side: the load is drawing inrush and pulling the bus down. If current stays flat or drops as voltage sags, the disturbance is upstream: a fault or large motor start elsewhere is depressing the source. This single observation, made with a monitor that records simultaneous voltage and current, separates problems the facility owns from problems it must mitigate passively because they originate on the utility system.

### Use ITIC and SEMI Curves to Predict Equipment Ride-Through
The ITIC (Information Technology Industry Council) and SEMI F47 curves define the voltage-versus-duration region in which equipment is expected to ride through without malfunction. Plot each captured event against the applicable curve: events inside the no-damage region should not trip compliant equipment, while events outside it justify mitigation. This converts a subjective complaint into an engineering basis for action. Note that older equipment, contactors, and undervoltage relays often trip well inside the ITIC region, so the curve predicts idealized, not actual, behavior for legacy hardware.

### Characterize Flicker With Pst and Plt, Not Subjective Complaints
Flicker is the subjective impression of fluctuating light output, and it is quantified by the short-term flicker indicator Pst (measured over 10 minutes) and the long-term indicator Plt (aggregated over two hours from 12 Pst values). A Pst of 1.0 represents the threshold of irritability for a reference incandescent lamp under 230V/60W conditions; IEC limits generally target Pst below 1.0. Do not rely on visual complaints alone, because perception depends on lamp type, ambient light, and observer sensitivity. Use a flicker meter compliant with IEC 61000-4-15 to obtain defensible measurements, and correlate high Pst periods with the operation of suspected sources.

### Match the Flicker Frequency to the Source
Flicker severity depends strongly on the frequency of the voltage modulation, with peak human sensitivity around 8 to 10 Hz. Identify the modulation frequency from the flicker meter or waveform: 1 to 2 Hz modulation suggests large motor starts or resistance heaters cycling; 1 Hz and below suggests manual loads or welding; 10 Hz and faster suggests arc furnaces, copy machines, or rapidly cycling drives. The frequency, combined with the source-side test, narrows the candidate list. Arc furnace flicker propagates far on the grid and is often the cause of widespread complaints that no single facility owns.

### Size Mitigation to the Actual Event Profile
Mitigation must match the disturbance type and duration. A UPS or DC bus support rides through sags of any depth for its battery autonomy, but is expensive and introduces maintenance. A dynamic voltage restorer (DVR) injects the missing voltage for short sags and is economical for frequent shallow sags. A static transfer switch moves the load to a healthy feeder within a quarter cycle if an alternate source exists. A soft starter or reduced-voltage starter reduces the depth of motor-start sags at the source. Selecting mitigation without the event profile leads to over-investment (UPS where a soft starter suffices) or under-investment (a UPS too small for the actual sag depth and duration).

### Coordinate With Utility Reporting and Responsibility
For utility-source sags and flicker, engage the utility with documented event records including timestamps, magnitudes, and the source-side test results. Utilities maintain sag performance data and may have records of faults or recloser operations that match the events. Establish whether the events exceed the utility's published power quality envelope, because some mitigation cost may be shared or the utility may adjust protection settings. Do not assume the utility will fix it, and do not assume it is solely the customer's problem; the source-side test determines ownership.

## Common Traps

### Blaming the Nearest Load Without the Current Test
The mechanism is that a process trips and the most recently added or largest nearby load is suspected. The false signal is the temporal coincidence of the trip and some load activity. The harm is that mitigation is installed on the wrong equipment, the real source (often a remote fault) continues, and the trips persist, eroding confidence in the diagnosis and wasting budget.

### Using Averaged Data That Hides Short Sags
The mechanism is that a logger records one-second or one-minute rms averages. The false signal is a clean-looking voltage trend with no obvious events. The harm is that sub-cycle to few-cycle sags, which are the most common cause of contactor dropouts and drive trips, are completely invisible, and the diagnosis concludes no power quality problem exists when equipment is repeatedly tripping.

### Treating All Flicker as the Same Phenomenon
The mechanism is that flicker complaints are lumped together without measuring the modulation frequency. The false signal is that lights visibly flicker, confirming a problem. The harm is that mitigation sized for low-frequency motor-start flicker (a soft starter) is installed when the actual source is high-frequency arc-furnace flicker imported from the grid, which requires source-side grid mitigation or series capacitors, and the local fix does nothing.

### Over-Specifying UPS for Shallow Sags
The mechanism is that a UPS is specified to solve every sag because it is the most robust solution. The false signal is that a UPS would indeed prevent the trips. The harm is capital and maintenance cost far exceeding the problem, because most sags are shallow and brief and could be handled by a much cheaper DVR, ferroresonant transformer, or improved ride-through, and the UPS adds battery replacement and efficiency losses for decades.

### Ignoring Contactor and Relay Drop-Out Characteristics
The mechanism is that mitigation is sized for the load's electronics while ignoring the holding coils of contactors and relays. The false signal is that the drive or controller has adequate ride-through. The harm is that the contactor drops out at 70 percent voltage for a few cycles, isolating the load even though the electronics would have survived, and the process trips anyway because the mechanical interlock opened.

## Self-Check

- Does the monitoring setup capture half-cycle rms and waveform traces, not just long-interval averages?
- For each sag, is the simultaneous current behavior recorded to determine whether the source is upstream or load-side?
- Are captured events plotted against the ITIC or SEMI curve to establish whether compliant equipment should have ridden through?
- Is flicker quantified with Pst and Plt using an IEC 61000-4-15 compliant meter, rather than subjective observation?
- Has the flicker modulation frequency been identified and matched to a candidate source class (motor start, welding, arc furnace)?
- Is the proposed mitigation matched to the event depth, duration, and frequency, rather than defaulting to UPS for all cases?
- Are contactor and relay drop-out voltages checked, since they often trip before the protected electronics?
- Has the utility been engaged with timestamped event data to confirm or rule out source-side responsibility?
