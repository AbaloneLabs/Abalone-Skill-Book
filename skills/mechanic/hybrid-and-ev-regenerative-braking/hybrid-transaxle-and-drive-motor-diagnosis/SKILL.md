---
name: hybrid-transaxle-and-drive-motor-diagnosis.md
description: Use when the agent is diagnosing hybrid transaxle or EV drive motor faults, motor whine and noise, inverter and power electronics faults, motor resolver and position sensor faults, hybrid transmission shudder, or deciding whether a drivability or noise fault is the motor, the inverter, the transaxle gears, or the hybrid control system.
---

# Hybrid Transaxle and Drive Motor Diagnosis

A hybrid transaxle or an EV drive unit integrates one or more electric motors, a planetary or fixed-gear power path, and the power electronics (inverter and converter) into a single drivetrain, and its faults span the electrical, the mechanical, and the control domains — a motor whine can be a bearing, a stator, or an inverter switching fault; a shudder can be a clutch, a motor resolver, or a control issue. The judgment problem is that the components are deeply integrated and expensive, and a fault in one domain mimics a fault in another, so condemning the motor for an inverter fault, or the transaxle for a motor fault, is a multi-thousand-dollar error. A technician who replaces the drive unit for an inverter fault, or who condemns the motor for a gear noise, hands back a vehicle with the same complaint. This skill covers the disciplined isolation of hybrid transaxle and drive motor faults.

## Core Rules

### Classify the Fault as Mechanical, Electrical, or Control Before Condemning a Component

The disciplined hybrid drivetrain diagnosis classifies the fault by domain. A mechanical fault (a whine, a growl, a gear noise that changes with vehicle speed and is present in all modes) points to the transaxle's gears, bearings, or the differential. An electrical fault (a motor whine that changes with motor speed and load, a loss of power, a shudder on electric drive) points to the motor, the inverter, or the resolver. A control fault (an inconsistent behavior, a mode-switching issue, a derate, a specific DTC in the hybrid controller) points to the hybrid control system, the sensors, or the software. The disciplined technician uses the symptom's behavior (speed-dependent, load-dependent, mode-dependent) and the DTCs to classify the fault before condemning a component. The tradeoff is that this classification takes analysis and data, but it is the difference between the right and the wrong multi-thousand-dollar part.

### Use Motor Speed, Torque, and Resolver Data to Isolate Motor and Inverter Faults

The disciplined diagnosis reads the motor data from the hybrid controller: the commanded motor torque, the actual motor torque, the motor speed (from the resolver), the inverter's phase current and voltage, the inverter temperature, and the motor temperature. A motor that does not produce the commanded torque (a commanded torque with no actual torque, or a lower actual torque) points to the motor, the inverter, or the power supply. A resolver fault (a motor speed reading that is erratic, drops out, or disagrees with the vehicle speed) points to the resolver or its wiring, and the hybrid controller cannot control the motor without the resolver's position feedback. An inverter fault (a phase current imbalance, an over-temperature, an insulation fault) points to the inverter or the power electronics. The tradeoff is that the data reading requires a capable scan tool, but it isolates the fault to the motor, the inverter, or the resolver.

### Distinguish Motor and Gear Noise by the Speed Reference and the Mode

The disciplined noise diagnosis distinguishes a motor noise (electromagnetic whine from the motor's switching, which changes with the motor speed and the load, and is often present on electric drive but not on engine-only drive) from a gear or bearing noise (a mechanical whine or growl that changes with the vehicle speed and is present in all modes). The disciplined technician road-tests in different modes (electric-only, engine-only, combined) and at different speeds, and notes whether the noise changes with the motor speed (motor or inverter) or the vehicle speed (gears or bearings). A stethoscope or a chassis-ear on the transaxle housing and the inverter can further isolate the source. The tradeoff is that the mode testing takes a road test, but condemning the drive unit for a motor whine that is a normal characteristic, or for an inverter fault, is a frequent error.

### Check the Inverter's Thermal Management and the Coolant System

The inverter and the motor are liquid-cooled, and their thermal management (the dedicated coolant loop, the pump, the radiator, the coolant condition) is critical to their operation, and a thermal fault derates or disables the drivetrain. The disciplined diagnosis checks the inverter and motor temperatures (from the scan tool), the coolant level and condition in the motor/inverter loop (separate from the engine coolant loop), the coolant pump's operation, and the radiator and the hoses for restriction. An inverter that derates at a specific temperature points to a thermal management fault (low coolant, a failed pump, a restricted radiator), not an inverter fault. The tradeoff is that the thermal check requires the scan tool and the coolant inspection, but condemning the inverter for a coolant pump fault is a needless expense.

### Evaluate the Transaxle Fluid and the Mechanical Components During Any Drivetrain Service

The hybrid transaxle has its own fluid (often a specific ATF, not the engine oil), and the fluid condition and level are critical to the gears, the bearings, and the wet clutches (where used). The disciplined service checks the fluid level and condition (a dark, burnt, or metallic fluid indicates gear or bearing wear; a fluid with water or coolant indicates a heat exchanger leak), and during any drivetrain service, inspects the fluid and the magnetic drain plug for debris. A transaxle with metallic debris in the fluid has mechanical damage and must be evaluated for rebuild or replacement. The tradeoff is that the fluid check is quick, but ignoring the fluid condition leads to mechanical failures that destroy the unit.

## Common Traps

### Condemning the Drive Unit for an Inverter or Power Electronics Fault — A loss of power or a motor fault code sets, the drive unit (motor and transaxle) is blamed, and the cause is the inverter or the power electronics, which are separate or serviceable. The trap mechanism is that the inverter and the motor are integrated in the diagnosis, and the inverter is not isolated. The false signal is the motor fault code; the harm is a needless drive unit. The disciplined technician reads the inverter data and isolates the fault.

### Condemning the Motor for a Resolver or Position Sensor Fault — The motor runs erratically or not at all, the motor is blamed, and the cause is the resolver (the position sensor) or its wiring, which the hybrid controller needs to commutate the motor. The trap mechanism is that the resolver's fault disables the motor's control, and the motor is fine. The false signal is the motor not running; the harm is a needless motor. The disciplined technician checks the resolver data and wiring.

### Treating a Normal Motor Whine as a Fault — The customer reports a whine on electric drive, the drive unit is diagnosed, and the whine is a normal electromagnetic characteristic of the motor's switching, not a fault. The trap mechanism is that the motor whine is a characteristic of EV and hybrid motors, and it is not a fault. The false signal is the customer's complaint; the harm is a needless diagnosis. The disciplined technician distinguishes the normal whine from a fault by the frequency and the comparison to a known-good vehicle.

### Condemning the Inverter for a Thermal Management Fault — The drivetrain derates, the inverter is blamed, and the cause is a failed coolant pump or a low coolant level in the inverter loop, causing the inverter to overheat and derate. The trap mechanism is that the thermal fault causes the derate, and the thermal management is not checked. The false signal is the derate pointing at the inverter; the harm is a needless inverter. The disciplined technician checks the inverter temperature and the coolant system.

### Ignoring the Transaxle Fluid Condition and Missing Mechanical Damage — The transaxle is noisy, the fluid is not checked, and the cause is gear or bearing wear indicated by metallic fluid, which is overlooked. The trap mechanism is that the fluid condition reveals the mechanical damage, and the fluid is not checked. The false signal is the noise being "electrical"; the harm is a missed mechanical failure. The disciplined technician checks the fluid and the magnetic plug.

## Self-Check

- Did I classify the fault as mechanical, electrical, or control before condemning a component?
- Did I read the motor torque, speed, resolver, and inverter phase current and temperature data?
- Did I distinguish a motor noise (motor-speed and load dependent) from a gear noise (vehicle-speed dependent) by testing in different modes?
- Did I check the inverter and motor temperatures and the coolant loop (level, pump, radiator)?
- Did I check the transaxle fluid level and condition and the magnetic drain plug for debris?
- For a resolver or position sensor fault, did I check the sensor data and the wiring before the motor?
- After the repair, did I verify the drivetrain operates in all modes (electric, engine, combined) with no derate, no noise, and no codes?
- Did I document the data readings, the fault classification, and the repair on the repair order?
