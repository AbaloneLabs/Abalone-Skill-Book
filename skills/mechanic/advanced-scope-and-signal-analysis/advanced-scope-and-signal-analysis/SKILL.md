---
name: advanced-scope-and-signal-analysis.md
description: Use when the agent is using an oscilloscope to diagnose sensors, actuators, CAN bus, ignition, or current ramp signals, interpreting waveform amplitude frequency and duty cycle, performing voltage drop and current measurements, or evaluating intermittent electrical faults that scan tool data cannot capture.
---

# Advanced Scope and Signal Analysis

The oscilloscope is the tool that separates the technician who diagnoses from the technician who swaps parts, because it shows what the voltmeter and the scan tool cannot: the actual shape, timing, and quality of a signal over time. The judgment problem is that most electrical and electronic faults are dynamic — a sensor glitches for two milliseconds, an actuator drops a pulse, a CAN message corrupts one frame in a thousand, an ignition event misfires under load — and the scan tool (which samples slowly and averages) and the voltmeter (which averages) cannot see them. A technician who chases an intermittent stall or a random misfire with only a scan tool will replace sensors and modules that test "good" on a meter, because the fault is in the signal quality, not the steady-state value. This skill covers the disciplined use of the oscilloscope and signal analysis: capturing the fault, interpreting the waveform, and proving the cause.

## Core Rules

### Choose the Right Tool: Scope for Dynamic, Meter for Steady-State, Scan Tool for System

The disciplined diagnostic chooses the tool that can see the fault. A steady-state value (battery voltage, a sensor's average voltage, a resistance) is measured with a multimeter. A system-level view (which codes, what the commanded and actual values are, the data stream) is read with a scan tool. A dynamic signal (a sensor waveform, an actuator current ramp, a CAN bus frame, an ignition event, a glitch) is captured with an oscilloscope. The trap is using the wrong tool: a voltmeter on a glitchy sensor averages the glitch away and reads "normal"; a scan tool on a fast CAN fault samples too slowly to see the corrupted frame. The disciplined technician matches the tool to the fault's time scale — milliseconds and below is scope territory.

### Capture the Fault in the Act: Trigger, Timebase, and Reproduction

The hardest part of scope diagnosis is capturing an intermittent fault, and the disciplined approach uses the scope's trigger and the right timebase to catch it. The trigger is set to the fault's signature: a voltage spike (trigger on a rising edge above the normal peak), a dropout (trigger on a falling edge below the normal floor), a missing pulse (trigger on a pulse-width violation). The timebase is set to the event's scale: a CAN frame is microseconds; a sensor glitch is milliseconds; an actuator ramp is tens of milliseconds; a slow drift is seconds. The fault is reproduced (a wiggle test on the harness, a load test, a heat or cold test, a road test with the scope recording) so the scope captures the event. The tradeoff is that capturing an intermittent fault takes patience and the right trigger setup, but a scope that is not triggered on the fault captures nothing useful.

### Interpret Sensor Waveforms by Shape, Amplitude, Frequency, and Duty Cycle

Each sensor type has a characteristic waveform, and the disciplined interpretation knows the expected shape and reads the deviations. A digital Hall-effect sensor (crank, cam, wheel speed) produces a square wave with a clean high and low and sharp transitions; a worn or contaminated sensor produces rounded edges, low amplitude, or missing pulses. An analog magnetic sensor (some crank and cam, some ABS) produces an AC sine wave whose amplitude rises with speed; a weak signal at cranking speed causes a no-start or a misfire code. A MAF sensor produces an analog voltage (or frequency, on some types) that should track throttle and RPM smoothly; a glitchy or lagging MAF signal causes drivability faults. A throttle position or accelerator pedal sensor produces a smooth analog voltage; a dead spot or a glitch causes a stumble. The disciplined technician compares the captured waveform to the known-good shape and reads the fault in the deviation.

### Use Current Ramping to Evaluate Actuators and Motors Without Teardown

A current ramp — the oscilloscope trace of the current through an actuator (a fuel injector, a solenoid, an ignition coil, a motor) as it energizes — reveals the mechanical and electrical health of the actuator without disassembly. The disciplined interpretation: a fuel injector current ramp shows the initial inrush, the plateau, and the "pintle bump" (the dip and rise when the injector opens) — a missing or distorted pintle bump indicates a stuck or sluggish injector; an ignition coil current ramp shows the charge and the dwell, and a shorted coil draws too much current; a solenoid current ramp shows the engagement. The tradeoff is that current ramping takes a low-amp probe and setup, but it diagnoses injectors and coils without removing them, saving hours of labor.

### Scope the CAN Bus and Network for Communication Faults

A CAN bus fault (a U-code, a no-communicate module, an intermittent network drop) is diagnosed on the scope by viewing the bus waveforms: the CAN-High and CAN-Low signals are differential, and a healthy bus shows clean, mirror-image square waves with the correct differential voltage (typically 2.5 V at rest, diverging to about 3.5 V and 1.5 V during a dominant bit). The disciplined scope diagnosis: a bus with reduced amplitude indicates a shorted or loaded bus (a failed module pulling the bus down); a bus with distortion or ringing indicates a termination fault (the two 120-ohm terminators at each end must be present; measure the bus resistance at 60 ohms across the network with the power off); a bus with a corrupted frame indicates a noisy or failing module. The tradeoff is that CAN diagnosis on a scope is advanced, but it is the only way to find an intermittent network fault that a scan tool cannot see.

### Perform Voltage Drop Testing to Find High-Resistance Connections

A voltage drop test measures the voltage lost across a connection or a cable under load, and it finds high-resistance faults (a corroded connector, a loose ground, a frayed cable) that a resistance measurement at zero current cannot see — because the resistance only shows up under load. The disciplined test: place the voltmeter leads across the connection (positive on the source side, negative on the load side) while the circuit is operating under load; a good connection drops near zero volts (millivolts); a bad connection drops a significant voltage (tenths or whole volts), and that dropped voltage is the energy not reaching the load. The tradeoff is that voltage drop testing requires the circuit to be operating, but it is the definitive test for high-resistance faults that cause dim lights, slow cranking, and weak actuators.

## Common Traps

### Chasing an Intermittent Fault With a Scan Tool Alone — The vehicle stalls intermittently, the technician reads the scan tool data, finds nothing, and replaces the crank sensor on a guess — but the fault was a two-millisecond dropout in the sensor signal that the scan tool's slow sampling averaged out. The trap mechanism is that the scan tool samples too slowly to see fast glitches, and the "normal" data misleads. The false signal is the normal scan data; the harm is parts replaced for a fault that needed a scope. The disciplined technician scopes the suspect signal during the fault reproduction.

### Reading a Steady-State Voltage and Missing a Dynamic Fault — The sensor reads the correct voltage on a meter, so the technician declares it good — but the signal glitches under vibration or heat, causing the intermittent fault. The trap mechanism is that a voltmeter averages and cannot see a dynamic fault. The false signal is the correct steady voltage; the harm is a "good" sensor that fails under conditions. The disciplined technician scopes the signal under the fault conditions.

### Triggering the Scope Wrong and Capturing Nothing Useful — The technician sets up the scope without the right trigger, captures a long recording of normal signal, and finds no fault — because the trigger did not isolate the event. The trap mechanism is that an untriggered or wrongly triggered scope captures everything and highlights nothing. The false signal is the "no fault found" recording; the harm is wasted time and a missed fault. The disciplined technician sets the trigger to the fault signature.

### Condemning a Module for a Termination or Wiring Fault on CAN — The CAN network drops intermittently, the technician condemns a module, and the fault persists because the cause was a missing terminator or a chafed wire. The trap mechanism is that CAN faults are often wiring or termination, not module failure, and the scope shows the bus distortion that points to the wiring. The false signal is the module losing communication; the harm is an expensive module replaced for a wiring fault. The disciplined technician scopes the bus and measures the termination resistance before the module.

### Measuring Resistance Instead of Voltage Drop for a High-Resistance Fault — The technician measures the ground strap resistance at zero current, reads near zero ohms, declares it good — but under the starter's load, the corroded connection drops volts and causes a slow crank. The trap mechanism is that resistance at zero current does not reveal the voltage drop under load. The false signal is the low ohm reading; the harm is a "good" ground that fails under load. The disciplined technician performs a voltage drop test under load.

## Self-Check

- Did I choose the scope (not the meter or scan tool) for a dynamic, intermittent, or fast fault?
- Did I set the trigger to the fault signature (spike, dropout, missing pulse) and the timebase to the event's scale?
- Did I reproduce the fault (wiggle, load, heat, road test) while the scope was recording?
- For a sensor fault, did I compare the captured waveform to the known-good shape and read the fault in the deviation (amplitude, frequency, edge sharpness, missing pulses)?
- For an actuator fault, did I use a current ramp to evaluate the injector, coil, or solenoid without teardown?
- For a CAN or network fault, did I scope the CAN-High and CAN-Low waveforms and measure the termination resistance (60 ohms)?
- For a high-resistance or low-voltage fault, did I perform a voltage drop test under load rather than a resistance test at zero current?
- Did I capture a known-good waveform for comparison if the fault was subtle?
- Did I save and document the scope capture to support the diagnosis and the repair?
