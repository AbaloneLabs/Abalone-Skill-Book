---
name: timing-system-timing-and-cam-crank-correlation.md
description: Use when the agent is diagnosing a jumped timing, a cam-crank correlation code, a timing belt or chain that jumped a tooth, a timing system that is one tooth off after a belt or chain replacement, a VVT learned value offset, or deciding whether a cam-crank correlation fault is a jumped timing belt, a stretched chain, a mis-indexed cam sensor, a VVT phaser stuck, or a timing belt installed one tooth off.
---

# Timing System Timing and Cam-Crank Correlation

The timing system's fundamental job is to keep the crankshaft and the camshaft(s) in their correct rotational relationship, and modern engine computers monitor this relationship continuously by comparing the crank position sensor signal to the cam position sensor signal. When that relationship drifts out of spec, the ECM sets a cam-crank correlation code and the engine runs poorly, starts hard, or will not start at all. The judgment problem is that a cam-crank correlation fault has many causes that all produce the same code: a jumped timing belt or chain, a stretched chain, a mis-indexed cam or crank sensor, a VVT phaser stuck advanced or retarded, a timing belt installed one tooth off after a service, or a sheared woodruff key or dowel pin. A technician who replaces the timing belt for a mis-indexed sensor, or who chases the sensor for a jumped chain, hands back a vehicle that still will not run right. This skill covers the disciplined isolation of timing and correlation faults.

## Core Rules

### Never Assume the Code Means the Belt or Chain Jumped — Establish the Baseline First

The disciplined cam-crank correlation diagnosis starts with the baseline, not the conclusion. The code says the cam and crank are out of sync, but the cause could be mechanical (jumped belt, stretched chain, sheared key), electrical (mis-indexed sensor, damaged tone wheel, wiring fault), or hydraulic (VVT phaser stuck). The disciplined technician first confirms the code is current and not historical, then checks whether the engine runs (a jumped belt may still run poorly; a sheared key may not run at all), then checks the freeze frame for the conditions (RPM, load, temperature) when the code set. The baseline directs the next step: a non-running engine points to a major mechanical failure; a running engine with a correlation code points to a drift or a stuck phaser. The tradeoff is that the baseline takes time, but jumping straight to a timing belt replacement without it is the most common diagnostic error.

### Verify the Mechanical Timing With the Engine at Top Dead Center Before Condemning Sensors

The mechanical timing verification is the decisive test for a cam-crank correlation fault, and it separates a mechanical timing problem (jumped belt, stretched chain, sheared key) from an electrical problem (sensor, tone wheel). The disciplined procedure rotates the engine to top dead center on the compression stroke of cylinder one, then checks the timing marks (the crank mark, the cam mark(s), and the injection pump mark on diesels) for alignment. If the marks align, the mechanical timing is correct and the fault is electrical or hydraulic (sensor, tone wheel, VVT phaser). If the marks do not align, the mechanical timing has jumped and the belt, chain, or key must be inspected. The tradeoff is that the TDC verification requires rotating the engine and accessing the timing marks (which may require removing covers), but it is the only test that definitively separates mechanical from electrical causes.

### Evaluate the VVT System's Contribution to the Correlation Before Condemning the Timing Belt

On engines with variable valve timing, the VVT phaser can advance or retard the cam, and a phaser stuck advanced or retarded shifts the cam's position relative to the crank and sets a cam-crank correlation code that mimics a jumped timing belt. The disciplined diagnosis checks the VVT system's contribution by commanding the phaser with the scan tool (if supported) and watching the cam timing respond, and by checking the VVT oil control valve (the solenoid that feeds oil to the phaser) for sticking or a blocked screen. A phaser that does not respond to the command, or that is stuck at full advance or retard, sets a correlation code. The tradeoff is that the VVT check requires scan-tool command capability, but condemning the timing belt for a stuck phaser is a frequent and costly error.

### Check the Cam and Crank Sensors and Tone Wheels for Indexing and Damage

The cam and crank position sensors and their tone wheels (the reluctor wheels the sensors read) are the source of the correlation signal, and a mis-indexed sensor (installed in the wrong position or with the wrong air gap), a damaged tone wheel (bent teeth, debris, a sheared locating pin), or a wiring fault can corrupt the signal and set a correlation code. The disciplined diagnosis checks the sensor installation (correct part number, correct air gap, secure mounting), the tone wheel (visible damage, secure mounting, correct indexing to the shaft), and the signal with a scope (the cam and crank signal patterns should match the OEM's expected pattern and phase relationship). A scope trace is the definitive check for a signal problem, because it shows the actual signal the ECM sees. The tradeoff is that the scope check requires a scope and the OEM's expected pattern, but it catches signal faults that a code reader misses.

### Re-Verify the Timing and the Correlation After Any Timing Belt or Chain Service

The disciplined timing service re-verifies the timing and the cam-crank correlation after any belt or chain replacement, because a belt or chain installed one tooth off is a common installation error that produces a correlation code and poor running. The re-verification rotates the engine two full revolutions (to settle the tensioner and the belt's position) and checks the timing marks for alignment, then clears the codes and runs the engine to confirm no correlation code returns. A belt one tooth off may run but set a code; a belt two or more teeth off may bend valves on an interference engine. The tradeoff is that the re-verification takes a few minutes, but it catches an installation error before the engine is damaged or the vehicle is returned.

## Common Traps

### Replacing the Timing Belt for a Mis-Indexed Cam Sensor — A cam-crank correlation code sets, the timing belt is blamed, and the cause is a cam sensor installed in the wrong position or with the wrong air gap. The trap mechanism is that the sensor's mis-indexing corrupts the signal, and the sensor is not checked. The false signal is the correlation code pointing at the timing; the harm is a needless belt job. The disciplined technician verifies the mechanical timing at TDC before condemning the belt.

### Condemning the Sensors for a Jumped Timing Belt — A cam-crank correlation code sets, the sensors are blamed, and the cause is a timing belt that jumped a tooth. The trap mechanism is that the jumped belt shifts the cam's actual position, and the belt is not checked. The false signal is the correlation code pointing at the sensors; the harm is needless sensor replacement. The disciplined technician verifies the mechanical timing at TDC.

### Missing a VVT Phaser Stuck Advanced or Retarded — A cam-crank correlation code sets, the timing belt is diagnosed, and the cause is a VVT phaser stuck advanced or retarded. The trap mechanism is that the stuck phaser shifts the cam's position, and the phaser is not checked. The false signal is the correlation code pointing at the timing; the harm is a needless belt job. The disciplined technician checks the VVT phaser's response.

### Leaving a Timing Belt One Tooth Off After a Service — A timing belt is replaced, the engine runs poorly or sets a correlation code, and the cause is the belt installed one tooth off. The trap mechanism is that the one-tooth error is a common installation mistake, and the timing is not re-verified. The false signal is the engine "running" after the service; the harm is a correlation code and potential valve damage. The disciplined technician re-verifies the timing after the service.

### Missing a Sheared Woodruff Key or Dowel Pin — A cam-crank correlation code sets, the timing system is diagnosed, and the cause is a sheared woodruff key (on the crank sprocket) or dowel pin (on the cam sprocket) that lets the sprocket spin independently of the shaft. The trap mechanism is that the sheared key lets the sprocket slip, and the key is not inspected. The false signal is the timing marks appearing to align (with the sprocket spun); the harm is a misdiagnosis. The disciplined technician inspects the keys and dowels when the sprockets are removed.

## Self-Check

- Did I confirm the cam-crank correlation code is current and check the freeze frame conditions?
- Did I verify the mechanical timing with the engine at TDC on the compression stroke before condemning sensors?
- Did I check the VVT phaser's response to a scan-tool command before condemning the timing belt?
- Did I check the cam and crank sensors for correct installation, air gap, and tone wheel damage?
- Did I scope the cam and crank signals and compare the phase relationship to the OEM's expected pattern?
- After any timing belt or chain service, did I rotate the engine two revolutions and re-verify the timing marks?
- Did I clear the codes and run the engine to confirm no cam-crank correlation code returns?
- Did I document the baseline findings, the TDC verification, the VVT check, and the repair on the repair order?
