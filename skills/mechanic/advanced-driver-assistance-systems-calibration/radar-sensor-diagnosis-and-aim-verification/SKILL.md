---
name: radar-sensor-diagnosis-and-aim-verification.md
description: Use when the agent is diagnosing adaptive cruise or AEB radar faults, verifying radar aim and bracket alignment, evaluating radar blockage or temperature faults, replacing a front or rear corner radar, or interpreting radar-specific DTCs and mounting-height concerns on advanced driver assistance systems.
---

# Radar Sensor Diagnosis and Aim Verification

Radar sensors are the eyes of adaptive cruise control, automatic emergency braking, and blind-spot monitoring, and they are far less forgiving than a camera because a radar beam cannot be "seen" and its aim is set mechanically by the bracket and the mounting surface, not optically by a lens. The judgment problem is that a radar fault (a disabled function, a warning lamp, a false alert, a missed obstacle) is almost never a failed sensor and almost always an aim, a mounting, a blockage, or a vehicle-level condition (ride height, alignment, bumper fit), and because the radar's accuracy depends on fractions of a degree that no scan-tool number can fully convey. A technician who replaces a radar for what is a bent bracket, or who re-aims a radar on a vehicle with the wrong ride height, hands back a vehicle whose AEB may still fire late or not at all. This skill covers the disciplined diagnosis of radar-specific ADAS faults: verifying the aim, the mounting, and the conditions before the sensor.

## Core Rules

### Treat Every Radar Fault as an Aim or Mounting Problem Until Proven Otherwise

The disciplined radar diagnosis starts from the assumption that the sensor is good and the installation or the vehicle geometry is wrong, because that is true in the overwhelming majority of cases. A radar that reports a misalignment code, a "sensor not learned," a blockage, or an out-of-range reading is pointing at its physical relationship to the vehicle, not at its internal electronics. The disciplined technician checks the bracket (is it bent, corroded, loose, or replaced with a non-OEM part after a collision), the mounting surface (is the bumper or grille properly seated and aligned, is the sensor seated fully in its pocket), the fasteners (are they torqued, are the correct isolators installed), and the sensor face (is it clean, dry, un-iced, and undamaged) before any scan-tool condemnation. The tradeoff is that this physical inspection is slow and requires removing trim, but a radar replaced on a bent bracket fails identically and the customer pays twice.

### Verify Ride Height and Vehicle Level Before Any Radar Calibration or Aim Check

A front radar is aimed relative to the vehicle's pitch and ride height, and a corner radar is aimed relative to the vehicle's roll and the bumper's position. The disciplined pre-calibration measures the ride height at every specified point (the OEM procedure names the measurement points, the fuel level, the tire pressure, and the cargo condition), confirms the vehicle is on a level surface (a vehicle on even a slight slope is aimed to the slope), and verifies the tire size and pressure are correct (oversized tires or low pressure change the pitch). A vehicle that has had springs, struts, a lift kit, or a load-leveling repair must have the ride height verified and corrected before the radar is touched, because a radar aimed on a low or nose-down vehicle looks at the ground and the AEB never sees the car ahead. The tradeoff is that the ride-height check takes minutes, but skipping it is the most common cause of a calibration that "passes" on the tool but fails on the road.

### Distinguish Blockage and Temperature Codes From Hardware Faults

Radar sensors set blockage codes when the beam path is obstructed (a dirty front bumper, snow, mud, a bumper cover full of water, a license plate or bracket in the beam path) and temperature codes when the sensor overheats (often from a failed internal heater or from being mounted too close to a heat source). The disciplined diagnosis reads the code, cleans the sensor face and the bumper area, clears the code, and road-tests to see if it returns before condemning the sensor. A blockage code that returns immediately after cleaning points to a mounting obstruction (a non-OEM bumper, a winch, a plate) in the beam path; a temperature code that returns points to the sensor's heater or its thermal management. The tradeoff is that cleaning and clearing feel like "not fixing it," but a large fraction of radar faults are environmental, and replacing a sensor for a mud-packed bumper repeats the fault.

### Use the OEM Static or Dynamic Aim Procedure and Verify the Result on the Road

A radar aim is set by the OEM procedure — a static aim with a specific target at a specific distance and offset, or a dynamic aim under specific drive conditions — and the disciplined technician follows it exactly and verifies the result by a road test, not by the tool's "pass" alone. The static aim requires the OEM target positioned to the millimeter, the vehicle level, and the scan-tool aim routine run to completion; a dynamic aim requires the specified road, speed, and traffic conditions. The verification road test exercises the function (adaptive cruise following a car, AEB with a target, blind-spot with a passing vehicle) and confirms no false alerts, no late warnings, and no disabled functions. The tradeoff is that the road test is the only real proof, because a tool that reports "calibration complete" has verified the sensor's internal aim, not its real-world performance.

### Replace a Radar With the Correct OEM Part and Transfer the Bracket Correctly

When a radar must be replaced (physical damage, internal failure confirmed by the OEM diagnostic), the disciplined replacement uses the correct OEM part (radars are vehicle- and option-specific, and a wrong part number may fit physically but have the wrong beam pattern or software), transfers or replaces the bracket correctly (the bracket is part of the aim, and a bent bracket transferred to the new sensor carries the fault), and performs the full calibration after the install (a new radar is not aimed until calibrated). The tradeoff is that the bracket and the calibration add time, but a sensor bolted to a bent bracket and handed back without calibration is a safety system that does not work.

## Common Traps

### Replacing the Radar for a Bent Bracket — The AEB warning sets after a minor front-end collision, the technician replaces the radar, and the warning returns because the bracket behind the bumper is bent and the new sensor is aimed at the same wrong angle. The trap mechanism is that the bracket sets the aim and a collision bends the bracket, and the sensor reads the bracket, not itself. The false signal is the code naming the sensor; the harm is an expensive sensor replaced for a cheap bracket. The disciplined technician inspects and measures the bracket before the sensor.

### Calibrating the Radar on a Vehicle With the Wrong Ride Height — The radar is calibrated, the tool reports success, and the AEB still fires late on the road because the vehicle sits low and the radar aims at the pavement. The trap mechanism is that the radar is aimed relative to ride height, and a low vehicle aims the beam low regardless of the calibration. The false signal is the tool reporting "pass"; the harm is a safety system that under-performs. The disciplined technician verifies ride height before the calibration.

### Condemning the Sensor for a Blockage or Environmental Code — The radar sets a blockage code in winter, the technician replaces the sensor, and the code returns the next snowfall because the cause was ice and mud on the bumper. The trap mechanism is that blockage codes are environmental, and the sensor is reporting a real obstruction. The false signal is the code pointing at the sensor; the harm is an unnecessary sensor in a vehicle that just needed a wash. The disciplined technician cleans the area and clears the code before the sensor.

### Using an Aftermarket Bumper or Bracket and Aiming Through an Obstruction — After a collision, an aftermarket bumper or a winch is installed, the radar is calibrated, and it false-alerts or disables because the bumper or the accessory is in the beam path. The trap mechanism is that the beam path must be clear, and an obstruction scatters the beam. The false signal is the calibration "completing"; the harm is a system that false-alerts or disables. The disciplined technician verifies the beam path is clear with OEM-approved components.

### Trusting the Tool's "Calibration Complete" Without a Road Test — The static calibration completes, the technician returns the vehicle, and the customer reports the adaptive cruise still surges or the AEB still false-brakes. The trap mechanism is that the tool verifies the sensor's internal aim, not the real-world function. The false signal is the "pass" on the tool; the harm is a vehicle returned with an unverified safety system. The disciplined technician road-tests and exercises the function before return.

## Self-Check

- Did I inspect the bracket, the mounting surface, the fasteners, and the sensor face before condemning the radar?
- Did I verify the ride height, tire size and pressure, fuel level, and vehicle level before any aim check or calibration?
- For a blockage or temperature code, did I clean the sensor area, clear the code, and road-test to see if it returns before replacing the sensor?
- Did I follow the OEM static or dynamic aim procedure exactly, with the correct target and conditions?
- Did I road-test after the calibration and exercise the function (adaptive cruise, AEB, blind-spot) to confirm no false alerts and correct behavior?
- When replacing a radar, did I use the correct OEM part, inspect or replace the bracket, and perform the full calibration after install?
- Did I confirm the beam path is clear of aftermarket obstructions (bumpers, plates, winches)?
- Did I document the before-and-after calibration and the road-test verification on the repair order?
