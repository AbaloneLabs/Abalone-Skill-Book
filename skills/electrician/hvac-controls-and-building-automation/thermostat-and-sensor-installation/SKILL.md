---
name: thermostat-and-sensor-installation.md
description: Use when the agent is selecting locations for thermostats and HVAC sensors, choosing sensor types such as RTD, thermistor, humidity, CO2, or pressure, wiring and calibrating sensors against a reference, or avoiding stratification and dead spots in air and water measurement.
---

# Thermostat and Sensor Installation

A thermostat or sensor is the eyes of the control system, and if it is installed in the wrong place, wired to the wrong input, or left uncalibrated, the entire building runs blind no matter how sophisticated the controller. The judgment problem is that sensors are small, cheap, and easy to mount, which invites electricians to put them wherever the wall is open and the wire is short, when in fact the location determines whether the sensor reads the condition it is supposed to control. A thermostat above a copier, a CO2 sensor in a return duct instead of the occupied space, a discharge air sensor in a stratified airstream, or a humidity sensor in direct sunlight will all read something, and the controller will dutifully act on that something, driving the building to the wrong condition. This skill covers the placement, selection, wiring, and calibration decisions that determine whether sensors tell the controller the truth.

## Core Rules

### Locate Sensors Where They Represent the Controlled Condition

A sensor's only job is to represent the condition the controller is trying to control, and that means it must be in a location where the measured variable matches the condition of interest. A room thermostat must be on an interior wall at typical breathing height, about five feet above the floor, away from supply diffusers that blow conditioned air across it, away from exterior doors that swing outside air past it, away from windows that radiate sun on it, and away from heat-generating equipment. A return air sensor must be in the mixed return airstream, not in a pocket of still air. A duct sensor must be in the moving air, not in a stratified layer along the duct wall. The location is chosen for representation, then the wiring is planned to reach it.

The trap is mounting the sensor where the wire is easy. The defense is to choose the location for representativeness first, at five feet AFF on an interior wall away from heat and air sources, and then to plan the wiring to reach it.

### Choose the Sensor Type for the Range, Accuracy, and Environment

Different measurement needs call for different sensor technologies, and the choice depends on the range, the accuracy, and the environment. Temperature sensors are most commonly thermistors or RTDs, with thermistors low-cost and high-sensitivity over a narrow range and RTDs more accurate and stable over a wider range; humidity sensors use capacitive elements that drift and need calibration; CO2 sensors use non-dispersive infrared that is accurate but costly and also drifts; pressure sensors use piezo or capacitive elements sized to the pressure range; and flow sensors vary widely by fluid and pipe size. Each type has a specific wiring requirement and a specific accuracy and drift characteristic, and the wrong type for the application produces readings that are either out of range or too inaccurate to control from.

The trap is using the cheapest sensor that fits the input. The defense is to choose the sensor type for the range, accuracy, and environment, to confirm its wiring and signal type, and to plan for the calibration its drift characteristic requires.

### Wire Sensors for Signal Integrity and Noise Immunity

Sensor wiring carries low-level signals that are vulnerable to noise, and the wiring method must match the signal type. Resistance-based sensors, thermistors and RTDs, use shielded cable with the shield grounded at one end, and three- or four-wire connections for RTDs over distance to cancel lead resistance. Current-loop sensors, 4-20mA, use a dedicated pair and are robust over distance. Voltage sensors, 0-10V, should be kept short or converted to current. All sensor wiring must be separated from line voltage and from variable-frequency drive output, which is a severe noise source, and from communication buses that can crosstalk. A sensor run bundled with the fan motor leads will read the motor's noise as signal, and the controller will chase that noise.

The trap is pulling sensor wiring alongside power to save a conduit. The defense is to use shielded cable for resistance sensors, to separate all sensor wiring from line voltage and VFD output, and to ground shields at one end only.

### Calibrate Sensors Against a Traceable Reference

Sensors drift, and a sensor that was accurate at installation will not stay accurate, so calibration against a traceable reference is part of commissioning and of ongoing maintenance. Calibration means comparing the sensor's reading to a known reference, a certified thermometer, a calibrated humidity standard, a known gas concentration, and adjusting the sensor or the controller to agree. The calibration must be done with the sensor in its operating condition, allowing time for thermal equilibrium, because a sensor pulled out of the duct to be checked reads a different condition than it does in service. Calibration records document the as-found and as-left values and establish the baseline for future drift tracking.

The trap is trusting the sensor's factory calibration forever. The defense is to calibrate each sensor against a traceable reference at commissioning, to record as-found and as-left, and to schedule recalibration based on the sensor's drift characteristic.

### Avoid Stratification and Dead Spots in Air and Water Measurement

Fluids in ducts and pipes are rarely uniform, and a sensor placed in a stratified layer or a dead spot reads a condition that does not represent the bulk flow. In a duct, air stratifies by temperature, with warmer air rising and cooler air sinking, and a sensor on the duct wall or in a corner reads the stratified layer rather than the mixed average; the cure is to place the sensor in the moving air downstream of a mix, or to use an averaging element that spans the duct. In a pipe, water can stratify or stagnate in tees and takeoffs, and a sensor in a dead leg reads stagnant water rather than the flowing stream. The placement must seek the well-mixed, representative location, and avoid the stagnant pocket.

The trap is mounting the sensor in the nearest tap or on the nearest duct face. The defense is to place air sensors in the moving, mixed airstream, to use averaging elements where stratification is expected, and to mount water sensors in the flowing pipe, not in dead legs.

### Mount at the Correct Height and Orientation for the Measurement

The height and orientation of a sensor affect what it measures, and each sensor type has a specified mounting that must be followed. A room thermostat mounts at roughly five feet above the floor to represent the occupied breathing zone, not at seven feet where the air is warmer and unoccupied. A wall-mounted CO2 sensor mounts in the occupied space, not in the return duct, because the goal is the concentration people breathe. Humidity sensors often have a preferred orientation to prevent condensation pooling. Duct sensors must insert to the correct depth to reach the moving air. The mounting detail is not cosmetic; it determines whether the sensor reads the intended condition.

The trap is mounting at whatever height the junction box landed. The defense is to mount each sensor at the specified height and orientation for its measurement, five feet AFF for occupied-zone sensors, and to verify the insertion depth for duct and pipe sensors.

### Coordinate with the Mechanical Design and the Sequence of Operations

Sensors exist to serve a sequence of operations, and their type, location, and range must be coordinated with the mechanical design and the written sequence. A sequence that controls to discharge air temperature needs a discharge air sensor, not just a return air sensor; a sequence that resets supply air temperature based on outdoor air needs an outdoor air sensor that is sun-shielded and representative; a CO2-based demand-controlled ventilation sequence needs CO2 sensors in the occupied spaces, not just one in the return. The sensor list must be read against the sequence of operations, and any gap, a sensor the sequence calls for but the design omits, or a sensor placed where the sequence cannot use it, must be resolved before installation.

The trap is installing the sensors on the plan without reading the sequence. The defense is to cross-check the sensor list against the sequence of operations, to confirm each sensor's type, location, and range supports its role in the sequence, and to flag gaps before installation.

## Common Traps

### Thermostat on an Exterior Wall or Above a Heat Source

The installer mounts the thermostat on the nearest open wall, which happens to be an exterior wall above a heat-generating copier. The mechanism of the trap is that the exterior wall and the copier both bias the thermostat's reading away from the room's true condition, so the controller heats or cools based on a temperature that has nothing to do with occupant comfort, and the space is never comfortable even though the thermostat reports it is satisfied. The false signal is that the thermostat reads a plausible temperature, which proves it works but not that it represents the room. The harm is a chronically uncomfortable space and complaints that the controls contractor cannot resolve because the sensor location is the problem. The defense is to mount thermostats at five feet AFF on interior walls away from supply diffusers, doors, windows, and heat sources.

### CO2 Sensor in the Return Duct Instead of the Occupied Space

The installer mounts the CO2 sensor in the return air duct to save wiring, reasoning that return air represents the space. The mechanism of the trap is that return air is a mixture from many spaces and from infiltration, so it dilutes the local concentration that the occupants actually breathe, and the demand-controlled ventilation sequence never opens the dampers as much as the real occupied-zone concentration warrants, under-ventilating the space. The false signal is that the sensor reads a CO2 value, which proves function but not representativeness. The harm is a space that is under-ventilated, with stale air and elevated CO2, precisely what the sensor was meant to prevent. The defense is to mount CO2 sensors in the occupied breathing zone where the sequence is meant to control.

### Unshielded Thermistor Run Beside the Fan Motor Leads

The installer pulls the thermistor wiring in the same conduit as the fan motor leads to save a run. The mechanism of the trap is that the motor leads induce voltage into the unshielded thermistor wiring, so the controller reads a temperature that jumps and drifts with the fan's operation, and the control loop hunts as it chases the noise. The false signal is that the sensor reads correctly when the fan is off, which is how it is often tested. The harm is a temperature input that wanders with electrical noise, causing the HVAC to cycle and never settle, and shortened equipment life from the hunting. The defense is to use shielded cable for resistance sensors and to separate sensor wiring from motor leads and VFD output.

### Sensor in a Duct Dead Spot or Stratified Layer

The installer mounts the duct temperature sensor on the nearest duct face, which happens to be in a slow, stratified layer along the bottom of the duct. The mechanism of the trap is that the air in a duct is rarely uniform, and a sensor in a stratified layer or a dead spot reads a temperature that does not represent the bulk flow, so the controller acts on a false condition. The false signal is that the sensor reads a plausible temperature, which proves it is in the duct but not that it is in the representative air. The harm is a control loop that never matches the real discharge or mixed air temperature, causing poor control and inefficient operation. The defense is to place duct sensors in the moving, mixed airstream and to use averaging elements where stratification is expected.

### Trusting Factory Calibration Without Field Verification

The installer powers up the sensors, sees plausible readings, and declares them calibrated. The mechanism of the trap is that sensors drift, and some arrive out of calibration from the factory or are shifted by the installation, and a plausible reading can still be several degrees or several percent off, enough to drive the control loop to the wrong condition. The false signal is that the reading looks reasonable, which proves the sensor is alive but not that it is accurate. The harm is a building that controls to a biased condition, with comfort and energy consequences that are invisible because the sensor reports satisfaction. The defense is to calibrate each sensor against a traceable reference at commissioning and to record as-found and as-left values.

### Mounting at the Wrong Height for the Measurement

The installer mounts the thermostat at seven feet because that is where the junction box landed, or the CO2 sensor at ceiling level because that was easiest. The mechanism of the trap is that the height determines what condition the sensor measures, and a thermostat at seven feet reads the warmer, unoccupied air above the breathing zone, while a CO2 sensor at the ceiling reads a concentration that differs from what occupants breathe. The false signal is that the sensor reads a value, which proves function but not representativeness. The harm is a control system that drives the building to the wrong condition because its sensors do not sample the occupied zone. The defense is to mount occupied-zone sensors at roughly five feet AFF and to follow the specified orientation for each sensor type.

## Self-Check

- Did I locate each sensor where it represents the controlled condition, mounting room thermostats at roughly five feet AFF on interior walls away from supply diffusers, doors, windows, and heat sources?
- Did I choose the sensor type for the range, accuracy, and environment, confirm its wiring and signal type, and plan for the calibration its drift characteristic requires?
- Did I use shielded cable for resistance sensors (thermistors and RTDs), separate all sensor wiring from line voltage and VFD output, and ground shields at one end only?
- Did I calibrate each sensor against a traceable reference at commissioning, record as-found and as-left values, and schedule recalibration based on the sensor's drift characteristic?
- Did I place air sensors in the moving, mixed airstream, use averaging elements where stratification is expected, and mount water sensors in the flowing pipe rather than dead legs?
- Did I mount each sensor at the specified height and orientation for its measurement, five feet AFF for occupied-zone sensors, with correct insertion depth for duct and pipe sensors?
- Did I cross-check the sensor list against the sequence of operations and confirm each sensor's type, location, and range supports its role in the sequence, flagging gaps before installation?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
