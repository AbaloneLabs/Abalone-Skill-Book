---
name: sensor-and-actuator-selection-for-bas.md
description: Use when the agent is selecting sensors such as temperature, humidity, CO2, pressure, and flow and actuators such as valves, dampers, and VFDs for building automation control loops, covering sensor placement, thermowell installation, averaging sensors, actuator fail positions, and control loop tuning considerations.
---

# Sensor and Actuator Selection for BAS

A building automation control loop is only as good as the measurement that feeds it and the actuator that moves in response. The judgment problem is that sensors and actuators are often chosen from a catalog by fitting the range and the signal type, which ignores the conditions that determine whether the device actually measures or moves correctly in service: where a sensor is placed, how it is mounted, whether it represents the medium it claims to, what an actuator does on power or air loss, and whether the loop can be tuned with the components selected. When these are missed, the loop either never settles, hunts, or fails to a wrong position, and the building underperforms in ways that are blamed on the controls logic rather than on the component selection and installation that actually caused it. This skill covers the selection and placement of sensors, the installation practices that protect measurement integrity, the actuator choices that govern fail behavior, and the tuning implications that tie selection to loop performance.

## Core Rules

### Match the Sensor Type and Range to the Medium and the Expected Span

Each measurement has a correct sensor family and a correct range. Air temperature is usually a thermistor or RTD; fluid temperature may use an immersed RTD in a thermowell; humidity needs a capacitive element rated for the operating range; CO2 uses nondispersive infrared (NDIR); pressure uses a piezoresistive transducer rated for the static pressure and the span; flow uses a differential pressure, magnetic, ultrasonic, or vortex element chosen by fluid, pipe size, and turndown. The range must bracket the expected operating span with margin but not so much that resolution is lost: a 0 to 1000 ppm CO2 sensor is right for an occupied space, a 0 to 5 percent sensor is not. Selecting a sensor by signal type alone, ignoring the measurement principle and the span, yields a device that reads but does not resolve the condition that matters.

### Place Sensors Where They Represent the Controlled Condition

Sensor placement determines whether the measurement represents the condition the loop is trying to control. A discharge air temperature sensor must be far enough downstream of a coil to read fully mixed air, not in a stratified lane. A space temperature sensor must be on an interior wall at occupant height, away from supply diffusers, return grilles, windows, and heat-generating equipment. A duct humidity sensor must be in a straight duct section where air is well mixed, not immediately downstream of a humidifier. A pressure sensor's impulse lines must be routed to avoid condensate traps and to read the intended tap. Placement is a field decision that follows physical reasoning about mixing, stratification, and interference, and it is documented so that a future technician understands why a sensor sits where it does.

### Use Thermowells and Averaging Sensors Where the Medium Demands

A temperature sensor in a pipe or duct must often be installed in a thermowell, which protects the element, allows withdrawal without draining the system, and provides the thermal contact needed for an accurate reading. A bare thermistor taped to a pipe wall does not measure the fluid temperature; it measures a blend of fluid, pipe, and ambient. In large ducts and coiling coils, a single point sensor can miss the true average condition because of stratification, so an averaging sensor, a long capillary or averaging bulb that spans the duct cross-section, is used to capture the mixed temperature. Selecting a single point sensor where an averaging sensor is needed yields a loop that controls to a reading that does not represent the coil's actual performance.

### Specify Actuator Fail Position to Match the Safety Consequence

Every pneumatic or electric actuator has a defined behavior on loss of its motive power or control signal: it fails in place, fails open, or fails closed. The fail position must be chosen by the safety and process consequence of loss, not by convenience. A heating valve in a freeze-protection loop should fail open to keep water moving and prevent freezing; a cooling valve may fail closed to avoid overcooling; a dampact on an outdoor air intake may fail closed to protect the coil in winter. An actuator that fails the wrong way turns a power or air loss into property damage or unsafe conditions. The fail position is a documented selection, verified during commissioning by actually removing power or air and observing the actuator's response.

### Size Actuators and Valves or Dampers for the Authority the Loop Needs

An actuator must have enough torque or thrust to move its load against the maximum differential pressure, and the valve or damper it drives must have the authority and characteristic the loop requires. A valve sized too large will operate near closed and hunt; a valve sized too small will never deliver full capacity. A damper actuator undersized for the static pressure will stall or slip. A control valve characteristic (linear, equal percentage, or quick opening) must match the heat transfer characteristic of the coil so that the loop gain stays roughly constant across the stroke. Selection is a coupled decision across actuator, valve or damper, and coil, and it is verified against the design pressure drop and flow.

### Account for Tuning Implications in Component Selection

Loop tuning is not independent of component selection. A slow actuator, a sensor with lag from a thick thermowell, or an oversized valve each change the dynamics the controller must handle and can make a loop untunable. A loop with significant transport delay, such as a long duct between a coil and the discharge sensor, is harder to stabilize and may need a cascade arrangement. Selecting components without considering their dynamic effect on the loop produces a system that no set of tuning gains can fix, because the problem is physical, not algorithmic. The selection should favor fast, well-placed sensors and properly sized actuators so that tuning has room to work.

### Document Selection Rationale and Maintain a Spare and Calibration Strategy

Each sensor and actuator selection should be documented with its rationale, its range, its fail position, and its calibration interval, so that the device can be maintained and replaced correctly over the building's life. Critical sensors should have spares identified, and sensors that drift, such as CO2 and humidity, need a calibration or replacement schedule. A system built without this documentation degrades silently, because no one knows when a sensor was last checked or what the right replacement is.

## Common Traps

### Discharge Sensor Too Close to the Coil

The installer mounts the discharge air temperature sensor immediately downstream of the cooling coil where the air is not yet mixed. The mechanism is that the air leaving a coil is stratified, and a sensor in one lane reads a local temperature that does not represent the average discharge, so the loop controls to a biased value. The false signal is that the sensor reads a plausible temperature. The harm is a loop that never settles, overcools or undercools, and wastes energy while the space never reaches setpoint.

### Bare Thermistor on a Pipe Wall

The installer tapes a thermistor to the outside of a chilled water pipe and wraps it, instead of using an immersed element in a thermowell. The mechanism is that the bare element reads a blend of fluid, pipe wall, and ambient, lagging and offset from the true fluid temperature. The false signal is that the reading tracks the fluid roughly and looks responsive. The harm is a reset or control sequence built on a biased measurement that never delivers the intended supply temperature.

### Single Point Sensor Where an Averaging Sensor Is Needed

The designer uses a single point duct sensor across a large cooling coil discharge where stratification is significant. The mechanism is that the single point samples one lane of stratified air, so the reading swings with airflow changes and misses the average. The false signal is that the sensor reads a temperature that looks reasonable. The harm is a discharge control that hunts and a coil whose true performance is hidden, leading to comfort complaints and energy waste.

### Actuator Fails the Wrong Way

The actuator on a heating coil valve is selected to fail closed by default, in a freeze-prone climate. The mechanism is that on a power or air loss the valve closes, stopping flow through the coil just when freeze protection matters most. The false signal is that the actuator works correctly under normal power. The harm is frozen coils and water damage during the very outage the fail position was meant to protect against.

### Oversized Valve Operating Near Closed

The valve is selected one or two sizes too large for the flow, so it operates in the bottom few percent of its stroke. The mechanism is that an oversized valve has excessive gain near closed, so tiny movements produce large flow changes, and the loop hunts or oscillates. The false signal is that the valve can reach full flow. The harm is a loop that never stabilizes, actuator wear, and poor temperature control.

### CO2 Sensor Never Recalibrated

A CO2 sensor is installed and trusted for years without calibration. The mechanism is that NDIR sensors drift over time, so the reading shifts away from the true concentration, and the demand-controlled ventilation commands the wrong outdoor air quantity. The false signal is that the sensor continues to output a plausible ppm value. The harm is chronic over-ventilation that wastes energy or under-ventilation that harms indoor air quality, neither of which is obvious to the operator.

## Self-Check

- Did I select each sensor by measurement principle and span appropriate to the medium and the expected operating range, not by signal type alone?
- Did I place each sensor where it represents the controlled condition, away from stratification, drafts, and heat sources, and document the placement rationale?
- Did I use thermowells for immersed temperature measurements and averaging sensors where duct or coil stratification would mislead a single point sensor?
- Did I specify each actuator's fail position to match the safety and process consequence of power or air loss, and verify it by actually removing motive power during commissioning?
- Did I size actuators and their valves or dampers for the required authority and characteristic, checking torque against maximum differential pressure and valve characteristic against coil behavior?
- Did I consider the dynamic effect of each component on loop tunability, favoring fast sensors, well-placed measurements, and properly sized actuators, and using cascade where transport delay is large?
- Did I document each selection's range, fail position, calibration interval, and spare, and establish a calibration or replacement schedule for drifting sensors such as CO2 and humidity?
- Does the output stay within the agent's scope, deferring licensed judgment, final authority, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
