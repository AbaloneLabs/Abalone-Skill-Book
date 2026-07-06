---
name: advanced-driver-assistance-systems-calibration.md
description: Use when the agent is calibrating ADAS sensors after windshield replacement, collision repair, or suspension service, performing static or dynamic target calibration for radar, camera, or lidar, diagnosing lane keep, adaptive cruise, AEB, or blind spot faults, or evaluating aim, ride height, and target setup on advanced driver assistance systems.
---

# Advanced Driver Assistance Systems Calibration

Advanced driver assistance systems (ADAS) — adaptive cruise control, lane keeping, automatic emergency braking, blind spot monitoring, and rear cross-traffic — depend on cameras, radar, and lidar aimed at the road with fractions-of-a-degree precision, and any change to the vehicle's geometry (a windshield replacement, a collision repair, a suspension replacement, a wheel alignment) can misalign a sensor and cause the system to misread the lane, the vehicles, or the obstacles. The judgment problem is that ADAS faults (a warning lamp, a disabled function, a false alert, a missed detection) are almost always calibration or alignment issues, not sensor failures, and because the calibration requires specific targets, specific conditions, and specific procedures that vary by OEM and by sensor. A technician who replaces a radar for what is a ride-height misalignment, or who skips the calibration after a windshield, hands back a vehicle whose safety systems misread the road. This skill covers the disciplined calibration and diagnosis of ADAS: knowing when to calibrate, setting up the targets, and verifying the result.

## Core Rules

### Know When Calibration Is Required: After Any Geometry Change

The disciplined ADAS service knows when calibration is required and does not skip it. Calibration is required after: a windshield replacement (the forward camera mounts to the glass), a collision repair that moved or replaced a sensor or its mounting, a suspension replacement or repair that changes the ride height (the radar and the camera aim depends on the vehicle's level), a wheel alignment (the steering angle sensor and the thrust line affect the ADAS reference), a sensor replacement, and a module replacement or reprogramming. The tradeoff is that the calibration adds time and cost to these services, but skipping it leaves the ADAS misaligned and the safety systems unreliable. The disciplined technician checks the OEM calibration requirements for every geometry-changing service and includes the calibration in the estimate.

### Set Up the Static Calibration With the Correct Targets, Position, and Conditions

A static calibration (in the bay, with targets) requires the correct OEM targets (the specific boards, mats, or panels for that vehicle and sensor), the correct positioning (the target distances, offsets, and heights, measured precisely — many OEMs specify the target position to the millimeter and the vehicle level to a fraction of a degree), and the correct conditions (the vehicle level, the tire pressure correct, the fuel level specified, the cargo specified, no extraneous objects in the target area). The disciplined setup verifies the vehicle is level (a vehicle on a slope calibrates the sensors to the slope), the targets are positioned per the OEM procedure (measured, not estimated), and the bay is clear of reflective objects and other vehicles that could confuse the radar. The tradeoff is that the setup is exacting and the space requirements are large (many OEMs require a flat, level bay of specific dimensions), but a wrong setup produces a wrong calibration that the system accepts as correct.

### Perform the Dynamic Calibration Under the Specified Drive Conditions

A dynamic calibration (on the road) requires driving the vehicle under specific conditions — the speed range, the road type (clear lane markings, no sharp curves, light traffic), the duration, and sometimes a specific sequence (a straight stretch, gentle curves, a following distance behind another vehicle) — until the system self-calibrates, indicated by the scan tool or the dashboard. The disciplined dynamic calibration chooses the right road and the right conditions (a road with clear, standard lane markings, in good weather, with light traffic), follows the OEM drive sequence, and verifies the calibration completes (the scan tool confirms, or the lamp clears). The tradeoff is that the dynamic calibration depends on road and weather conditions that may not be available, and a calibration that does not complete may require a static calibration or a retry on a better road.

### Verify Ride Height and Vehicle Level Before Any Camera or Radar Calibration

The forward camera and the front radar are aimed relative to the vehicle's level and ride height, and a vehicle with the wrong ride height (worn springs, a modified suspension, an overloaded cargo) calibrates the sensors to the wrong reference. The disciplined pre-calibration verifies the ride height is within spec (measured at the specified points, with the specified fuel and cargo), the tire pressure is correct, and the vehicle is level. A vehicle that has had suspension work or that sits low must have the ride height corrected before the calibration, or the calibration is wrong. The tradeoff is that the ride height check takes minutes, but a calibration on a low vehicle aims the sensors low and the systems misread.

### Diagnose ADAS Faults as Calibration, Alignment, or Sensor, In That Order

An ADAS fault (a warning lamp, a disabled function, a false alert) is diagnosed in order of likelihood: calibration and alignment first (the most common cause, especially after a geometry change), the sensor and its mounting next (a loose bracket, a damaged sensor), and the module and the wiring last. The disciplined diagnosis reads the codes (which often point to a calibration or an aim fault), checks the sensor mounting (is the bracket bent, is the sensor loose, is the lens dirty or damaged), verifies the ride height and the wheel alignment, and performs the calibration before condemning the sensor. The tradeoff is that the sensor is expensive and the calibration is labor, but jumping to the sensor is the common error. The disciplined technician calibrates before replacing.

### Clean and Inspect the Sensor Lenses and Mounting Surfaces

A dirty, wet, or iced camera or radar lens causes false alerts, disabled functions, and warning lamps, and the disciplined ADAS service cleans and inspects the lenses as a first step. The forward camera lens (behind the windshield, in the mirror area) is affected by a dirty or pitted windshield, a film, or a tint; the front radar lens (in the grille or bumper) is affected by dirt, mud, snow, ice, and a damaged bumper cover. The disciplined cleaning uses the OEM-recommended method (a soft cloth, no abrasives, no solvents that damage the lens) and inspects for pitting, cracking, or damage that requires replacement. The tradeoff is that cleaning is free and often curative, but it is skipped in favor of the calibration or the sensor.

## Common Traps

### Skipping the Calibration After a Windshield or Suspension Service — The windshield is replaced or the suspension is serviced, the ADAS is not calibrated, and the lane keeping and the AEB misread because the camera or the radar is now misaligned. The trap mechanism is that the sensor aim depends on the vehicle's geometry, and a geometry change misaligns it; the system still "works" but with wrong reference. The false signal is the system "not setting a code"; the harm is safety systems that misread the road. The disciplined technician calibrates after every geometry change.

### Replacing a Radar or Camera for a Ride-Height Misalignment — The ADAS warning lamp is on, the technician replaces the radar or the camera, and the lamp returns because the cause was the ride height (the vehicle sits low and the sensor aims low). The trap mechanism is that the sensor is aimed relative to the vehicle's level, and a low vehicle aims the sensor low; the new sensor has the same problem. The false signal is the code pointing at the sensor; the harm is an expensive sensor replaced for a ride-height issue. The disciplined technician verifies the ride height before the sensor.

### Setting Up the Static Targets by Estimation Instead of Measurement — The technician sets up the calibration targets "about right," runs the calibration, and the system accepts a wrong calibration because the targets were off. The trap mechanism is that the calibration is only as accurate as the target setup, and an estimated setup produces a wrong aim that the system stores as correct. The false signal is the calibration "completing successfully"; the harm is safety systems that misread. The disciplined technician measures the target positions to the OEM spec.

### Performing a Dynamic Calibration on the Wrong Road — The technician drives the vehicle for a dynamic calibration on a road with poor markings or heavy traffic, and the calibration does not complete or completes wrong because the conditions were not met. The trap mechanism is that the dynamic calibration depends on clear lane markings and specific conditions, and a wrong road prevents a valid calibration. The false signal is the drive "being done"; the harm is a calibration that did not complete or completed wrong. The disciplined technician chooses the right road and conditions and verifies completion.

### Condemning the Sensor for a Dirty or Damaged Lens — The ADAS faults, the technician condemns the sensor, and the cause was a dirty or iced lens. The trap mechanism is that a dirty lens blocks the sensor's view and mimics a sensor fault. The false signal is the code pointing at the sensor; the harm is an unnecessary sensor replacement. The disciplined technician cleans and inspects the lens first.

## Self-Check

- Did I check the OEM calibration requirements and include the calibration after every geometry-changing service (windshield, collision, suspension, alignment, sensor or module replacement)?
- For a static calibration, did I use the correct OEM targets, measure the target positions to the spec, verify the vehicle is level with correct tire pressure and fuel, and clear the bay of reflective objects?
- For a dynamic calibration, did I choose a road with clear markings and light traffic, follow the OEM drive sequence, and verify the calibration completed?
- Did I verify the ride height is within spec and the vehicle is level before any camera or radar calibration?
- For an ADAS fault, did I diagnose in order (calibration and alignment, sensor mounting, module and wiring) and calibrate before replacing the sensor?
- Did I clean and inspect the camera and radar lenses (and the windshield area) before condemning a sensor?
- Did I check the sensor mounting brackets for bending or looseness after a collision or a service?
- Did I verify the wheel alignment and the steering angle sensor zero are correct before the ADAS calibration?
- Did I road-test after the calibration and verify the ADAS functions (lane keep, adaptive cruise, AEB, blind spot) operate without false alerts or warnings under real conditions?
