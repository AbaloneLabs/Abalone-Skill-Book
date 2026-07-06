---
name: tire-pressure-monitoring-system-tpms-service.md
description: Use when the agent is servicing tire pressure monitoring systems, replacing TPMS sensors, diagnosing TPMS warning lights that stay on or flash, relearning sensor positions after tire rotation, programming auto-learn versus stationary sensors, or deciding whether a TPMS fault is a dead sensor, a registration issue, or a module problem.
---

# Tire Pressure Monitoring System (TPMS) Service

TPMS is a safety system mandated by law, and it is also one of the most common sources of dashboard warning lights and customer confusion. The judgment problem is that TPMS involves three distinct layers — the battery-powered wheel sensors, the vehicle's receiver/learning logic, and the module that interprets the data and lights the lamp — and a symptom like "TPMS light on" can originate at any layer. A technician who replaces a sensor every time the light is on, or who assumes a flashing light means the same thing as a steady light, will misdiagnose the fault and send customers home with lights still on or sensors that will not register. This skill covers the disciplined diagnosis and service of TPMS, with emphasis on distinguishing sensor faults from system faults and on the relearn and programming procedures that make a TPMS service actually clear the light.

## Core Rules

### Distinguish a Steady TPMS Light From a Flashing TPMS Light

The behavior of the TPMS lamp is itself diagnostic, and the two modes indicate different faults:

- **Steady illumination** — one or more tires are below the calibrated low-pressure threshold (typically 25% below the placard pressure). This is a pressure fault, and the response is to check and correct tire pressures, then drive the vehicle to let the system update. If the light remains after pressures are correct, a sensor may be reporting an erroneous low pressure or the system needs a relearn.
- **Flashing for 60-90 seconds at startup, then steady** — a system malfunction. The module has detected a fault: a dead sensor, a sensor not learned, a receiver fault, or a module internal fault. This is not a pressure problem; correcting tire pressure will not clear it. Diagnosis requires reading TPMS codes and checking sensor status.

The disciplined technician observes the lamp behavior before doing anything else, because treating a system-malfunction (flashing) light as a low-pressure (steady) light wastes time checking pressures that are already correct, and treating a low-pressure light as a malfunction leads to unnecessary sensor replacement. The two modes call for different procedures.

### Read TPMS Codes and Sensor Data Before Replacing Anything

The TPMS module stores codes for specific sensor faults (no signal, out-of-range, battery low) and system faults (receiver, module, circuit). Before replacing a sensor, read the codes and the sensor status in live data: which sensors are reporting, which are not, what pressures and temperatures they report, and whether each sensor's ID is learned to its wheel position. A sensor that reports a pressure is alive and transmitting; a sensor that shows no data may be dead, unlearned, or out of receiver range. The disciplined technician uses this data to identify the specific failed sensor or the system fault, rather than replacing sensors by guess. A "left-front sensor no signal" code points to one wheel; a "system malfunction" with all sensors reporting points to the receiver or module, not the sensors.

### Know the Three TPMS Sensor Types and Their Service Procedures

TPMS sensors fall into three categories, and the service procedure differs for each:

- **Direct, fixed-ID sensors (factory and many OEM-replacement)** — each sensor has a unique ID programmed at manufacture. When installed, the vehicle must learn the sensor ID to the wheel position via a relearn procedure (auto-learn by driving, stationary relearn with a tool, or a TPMS-tool-triggered relearn).
- **Programmable sensors (universal, "EZ-sensor")** — a blank sensor that is programmed by a TPMS tool to emulate a specific OEM sensor's protocol and ID. The technician selects the vehicle, the tool programs the sensor, and the sensor is then installed and learned like an OEM sensor. Useful for stocking one sensor for many vehicles, but the programming step must be done correctly or the sensor will not communicate.
- **Auto-learn / self-learning sensors** — on some vehicles, after driving above a threshold speed for a set time, the system automatically learns new sensor IDs and positions without a manual relearn. Still requires the sensors to be the correct protocol for the vehicle.

The disciplined technician identifies the sensor type from the vehicle and the service information, and follows the matching procedure — fixed-ID requires a relearn, programmable requires programming then a relearn, auto-learn requires the drive cycle. Assuming all sensors are the same and skipping the relearn is a primary cause of TPMS lights that remain on after sensor replacement.

### Perform the Correct Relearn Procedure After Tire Rotation or Sensor Replacement

When tires are rotated, the system's wheel-position mapping is wrong (it thinks the left-front sensor is still at the left-front when it is now at the right-rear), and the reported pressures no longer match the actual positions — which matters for systems that display individual pressures and for systems that use wheel-position data for other functions. When a sensor is replaced, the new sensor's ID is unknown to the module until learned. The relearn procedure tells the module which sensor ID is at which wheel:

- **Auto-learn (drive cycle)** — inflate to placard pressure, drive above the threshold speed (often 15-20+ mph) for the specified time. The system learns IDs and positions automatically. Works on some vehicles, not all.
- **Stationary / TPMS-tool relearn** — put the system in relearn mode (via the dash button, a scan-tool command, or a TPMS tool), then trigger each sensor in the specified wheel order (typically left-front, right-front, right-rear, left-rear) with the TPMS tool. The horn chirps confirm each sensor is learned.
- **Manual (pressure-release) relearn** — older systems: put in relearn mode, then deflate each tire in sequence (the pressure drop triggers the sensor to transmit and the system learns it). Time-consuming and less common on modern vehicles.

The disciplined technician performs the relearn specified for the vehicle, in the correct wheel order, and confirms all four sensors are learned (four horn chirps or the scan-tool data shows all positions) before returning the vehicle.

### Replace Sensors Proactively Based on Battery Life, but Confirm Before Replacing

TPMS sensors are battery-powered, and the batteries are not separately replaceable — when the battery dies, the sensor is replaced. Battery life is typically 5-10 years depending on use and temperature. A sensor approaching end of life may transmit intermittently before failing, producing a TPMS light that comes and goes. The disciplined technician, when one sensor's battery has failed, advises the customer that the other sensors of the same age are likely near failure too — replacing all four avoids a near-term return for the next sensor to die. However, each replacement should be confirmed dead (no signal in live data, low-battery code) before replacement, not assumed.

### Use Correct Torque on Sensor Valve Cores and Sensors

TPMS sensors mount through the valve hole and are held by a nut torqued to a specific low value (typically 35-80 in-lbs, not ft-lbs). Over-torquing cracks the sensor body or the aluminum wheel; under-torquing allows leaks. The valve core is a special nickel-plated core (not brass, which corrodes in the TPMS sensor's aluminum stem) and is torqued, not just snugged. The disciplined technician uses a calibrated inch-pound torque wrench on the sensor nut and a valve-core torque tool on the core, because "finger tight" and "good and snug" produce both leaks and broken sensors.

## Common Traps

### Replacing a Sensor for a Steady (Low-Pressure) Light — The TPMS light is on steady, the technician assumes a bad sensor, and replaces one or more sensors. The trap mechanism is that a steady light indicates low tire pressure, not a sensor fault; the sensors are working correctly and reporting the actual low pressure. The false signal is "the light is on, so a sensor must be bad." The harm is that the customer pays for sensors that were functioning, the light remains on because the actual tire is still low (or has a leak), and the shop has misdiagnosed a simple pressure issue as a sensor failure. The disciplined technician checks and corrects tire pressures first, observes whether the light clears after a drive, and reads sensor data to confirm the sensors are reporting before replacing anything.

### Treating a Flashing (System Malfunction) Light as a Low-Pressure Light — The TPMS light flashes at startup then goes steady, the technician checks tire pressures, finds them correct, and tells the customer the system is fine. The trap mechanism is that the flashing-then-steady pattern indicates a system malfunction (dead sensor, unlearned sensor, receiver fault), not a pressure problem; correcting pressures cannot clear it. The false signal is that pressures are correct, interpreted as "no problem." The harm is that the customer drives with a non-functional TPMS system — a safety-system failure and, in jurisdictions where TPMS function is inspected, a vehicle that will not pass inspection. The disciplined technician recognizes the flashing pattern as a malfunction, reads the TPMS codes, and diagnoses the system fault.

### Skipping the Relearn After Sensor Replacement or Rotation — The technician replaces a sensor or rotates the tires and returns the vehicle without performing the relearn. The trap mechanism is that the module does not know the new sensor's ID (after replacement) or the new wheel positions (after rotation), so it cannot display correct pressures by position and may flag a fault for the unlearned sensor. The false signal is that the TPMS light is off when the car leaves (the system may suppress the light temporarily while searching for sensors). The harm is that the light comes on within a drive cycle, the customer returns, and the shop must redo the job — or worse, the system reports incorrect pressures by position, misleading the customer about which tire is low. The disciplined technician always performs the specified relearn after any sensor replacement or rotation and confirms all positions are learned.

### Installing a Programmable Sensor Without Programming It First — The technician stocks universal programmable sensors, installs one to replace a failed OEM sensor, and skips the programming step. The trap mechanism is that an un-programmed universal sensor transmits no protocol or the wrong protocol, so the vehicle's receiver cannot read it and the module flags it as missing. The false signal is that the sensor is physically installed and the valve stem looks correct. The harm is that the TPMS light remains on, the customer returns, and the technician may condemn the sensor and replace it again — when the original sensor was fine but never programmed. The disciplined technician programs every universal sensor to the specific vehicle protocol and ID before installation, using the TPMS tool's vehicle-selection process.

### Over-Torquing or Under-Torquing the Sensor Nut and Causing Leaks or Cracks — The technician installs a TPMS sensor and tightens the nut "by feel" or with a foot-pound wrench. The trap mechanism is that TPMS sensor nuts require inch-pound torque (35-80 in-lbs); by-feel tightening almost always over-torques, cracking the sensor body or the aluminum wheel seat, and a foot-pound wrench cannot dial low enough. The false signal is that the nut feels tight and the tire holds initial pressure. The harm is a slow leak that appears over days (under-torque) or a cracked sensor that fails and leaks (over-torque), requiring rework and sensor replacement. The disciplined technician uses a calibrated inch-pound torque wrench and a valve-core torque tool, because TPMS sensor torque is a precision spec, not a "snug" judgment.

## Self-Check

- Did I observe whether the TPMS light is steady (low pressure) or flashing-then-steady (system malfunction) before diagnosing?
- For a steady light, did I check and correct all tire pressures to placard and drive the vehicle before assuming a sensor fault?
- For a flashing light, did I read TPMS codes and sensor live data to identify the specific failed sensor or system fault?
- Did I identify the sensor type (fixed-ID, programmable, auto-learn) and follow the matching service procedure?
- For a programmable universal sensor, did I program it to the vehicle's protocol and ID before installation?
- After sensor replacement or tire rotation, did I perform the specified relearn procedure in the correct wheel order and confirm all positions learned?
- Did I use a calibrated inch-pound torque wrench on the sensor nut and a torque tool on the nickel-plated valve core?
- Did I advise the customer about the remaining sensors' battery life when one sensor has failed, and confirm each replaced sensor was actually dead before replacement?
