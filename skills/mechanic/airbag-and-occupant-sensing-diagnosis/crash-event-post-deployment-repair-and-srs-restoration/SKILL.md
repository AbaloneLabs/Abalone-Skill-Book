---
name: crash-event-post-deployment-repair-and-srs-restoration.md
description: Use when the agent is restoring a vehicle after airbag deployment, replacing deployed airbags and seat belt pretensioners, clearing crash data or evaluating SRS module replacement, diagnosing crash sensors and impact sensors after a collision, or verifying SRS readiness and system arming after collision repair.
---

# Crash Event Post-Deployment Repair and SRS Restoration

Restoring a vehicle after an airbag deployment is fundamentally different from diagnosing an SRS warning lamp, because a deployment is a system-wide event that fires multiple pyrotechnic devices, stresses the wiring and the mounts, and leaves the SRS module in a crash-recorded state that often requires replacement or crash-data clearing. The judgment problem is that a post-deployment restoration must replace every deployed device, verify every circuit that the crash stressed, and restore the SRS module to a state that arms and deploys correctly in the next crash — and a single missed pretensioner, a reused stressed connector, or a module left in a crash state hands back a vehicle whose airbags will not fire when needed. A technician who replaces the visible airbags and clears the codes, without checking the seat belt pretensioners, the impact sensors, and the module's crash state, returns a vehicle that is not actually protected. This skill covers the disciplined restoration of the SRS after a crash event.

## Core Rules

### Inventory Every Pyrotechnic Device That Deployed and Every Device That Did Not

The disciplined post-deployment restoration begins with a complete inventory of every pyrotechnic device on the vehicle, because a crash deploys some and not others, and every deployed device must be replaced while every non-deployed device must be verified. The inventory covers: the driver airbag (steering wheel), the passenger airbag (dash), the knee airbags, the side airbags (seat or door), the curtain airbags (head), the seat belt pretensioners (front and rear, often forgotten), the seat cushion airbags (active protection on some vehicles), and the seatbelt load limiters. The disciplined technician cross-references the OEM deployment logic for the crash type (a frontal deployment fires the front bags and the front pretensioners; a side deployment fires the side and curtain bags and may fire the pretensioners) to predict which devices fired, then physically verifies each device (a deployed airbag is open and spent; a deployed pretensioner is locked and shortened). The tradeoff is that the inventory is thorough and slow, but a missed pretensioner leaves a seat belt that does not lock in the next crash.

### Evaluate the SRS Module: Replace or Clear the Crash Data

The SRS module records the crash event (the deployment command, the crash data, the sensor readings), and many OEMs require the module to be replaced after a deployment, while some allow the crash data to be cleared by a specialized tool or an OEM procedure. The disciplined restoration follows the OEM requirement: if the OEM requires replacement, the module is replaced and programmed; if the OEM allows clearing, the crash data is cleared with the OEM-approved tool and the module is verified to arm and self-test. A module left in a crash state (the data not cleared, the module not replaced) will not arm and will not deploy in the next crash, and it sets a permanent lamp. The tradeoff is that a replacement or a crash-data clear is mandatory, and a module that "still works" after a deployment is not a module that will fire next time.

### Inspect Every Impact Sensor, Crash Sensor, and Mounting for Damage

The crash that deployed the airbags also stressed the impact sensors, the crash sensors, and their mounts, and the disciplined restoration inspects every sensor in the impact zone. A side-impact sensor in a struck door may be damaged even if it did not trigger the deployment; a front impact sensor in a struck bumper may be cracked or its bracket bent; a satellite sensor in the B-pillar may have its wiring stressed. The disciplined inspection checks each sensor for physical damage, checks the bracket and the mounting, and checks the wiring harness in the impact zone for pulled, pinched, or cut wires. A damaged sensor reads wrong and either fails to deploy in the next crash or deploys falsely. The tradeoff is that the sensor inspection is detailed, but a damaged sensor left in place is a latent safety failure.

### Verify the SRS Wiring and Connectors in and Near the Impact Zone

The SRS wiring uses specific connectors (often yellow, with locking tabs and shorting bars that short the circuit when disconnected to prevent accidental deployment), and the disciplined restoration verifies every connector in and near the impact zone. A connector that was stressed in the crash may have a bent pin, a cracked housing, or a failed shorting bar, and a damaged connector causes an intermittent resistance fault that sets a lamp or prevents deployment. The disciplined technician inspects each connector visually, checks the pin fit, and performs a wiggle test on the harness. The tradeoff is that the connector inspection is meticulous, but a damaged connector is a common cause of a post-repair SRS lamp.

### Clear the Codes, Arm the System, and Verify the Readiness With a Drive Cycle

After the restoration, the disciplined technician clears the codes, verifies the lamp behavior (the bulb check on, off after the self-test), and confirms the system arms and is ready. Many SRS systems require a specific readiness sequence (a number of drive cycles, a specific speed or condition) to confirm the system is armed, and the disciplined technician follows it and verifies the readiness on the scan tool before returning the vehicle. A lamp that stays on is an unresolved fault; a lamp that is off but the system is not ready is a latent fault. The tradeoff is that the readiness verification takes a drive cycle, but returning a vehicle that is not armed is a safety failure.

## Common Traps

### Replacing the Visible Airbags and Missing the Seat Belt Pretensioners — The deployed airbags are replaced, the codes are cleared, and the seat belt pretensioners (which also fired) are left spent, so the seat belts do not lock in the next crash. The trap mechanism is that the pretensioners are pyrotechnic devices that fire in a crash, and they are less visible than the airbags. The false signal is the airbags "being replaced"; the harm is a seat belt system that does not protect. The disciplined technician inventories every pyrotechnic device and replaces every deployed one.

### Clearing the Codes on a Crash-State Module Without Replacing or Clearing the Crash Data — The SRS lamp is on after the repair, the technician clears the codes, the lamp goes out temporarily, and it returns because the module is still in a crash state. The trap mechanism is that the module records the crash and must be replaced or crash-data-cleared; clearing the codes does not clear the crash state. The false signal is the lamp going out after the clear; the harm is a vehicle returned with a non-arming SRS. The disciplined technician follows the OEM module replacement or crash-data-clear procedure.

### Reusing a Damaged Impact Sensor in the Impact Zone — The side airbag deployed, the side-impact sensor in the door is reused because it "looks OK," and the sensor is damaged and reads wrong. The trap mechanism is that the sensor was in the impact zone and stressed, and a damaged sensor misreads the next crash. The false signal is the sensor looking intact; the harm is a sensor that fails to deploy or deploys falsely. The disciplined technician inspects and replaces any sensor in the impact zone.

### Ignoring a Stressed SRS Connector and Returning With an Intermittent Lamp — The restoration is complete, but a connector in the impact zone has a bent pin, and the vehicle returns with an intermittent SRS lamp. The trap mechanism is that the crash stressed the connector, and a damaged connector causes an intermittent resistance fault. The false signal is the connector "looking seated"; the harm is a comeback. The disciplined technician inspects and verifies every connector in the impact zone.

### Declaring the System Ready Without the Readiness Drive Cycle — The lamp is off after the clear, the technician returns the vehicle, and the system is not actually armed because the readiness sequence was not completed. The trap mechanism is that the SRS requires a readiness confirmation, and a lamp that is off is not proof of an armed system. The false signal is the lamp being off; the harm is a vehicle returned with a non-armed SRS. The disciplined technician verifies the readiness on the scan tool.

## Self-Check

- Did I inventory every pyrotechnic device (all airbags, all pretensioners, load limiters) and replace every deployed one?
- Did I follow the OEM requirement for the SRS module (replace or crash-data-clear) and verify the crash state is resolved?
- Did I inspect every impact and crash sensor in and near the impact zone for damage, and replace any stressed sensor?
- Did I verify every SRS connector in the impact zone for pin fit, housing integrity, and shorting-bar function, and wiggle-test the harness?
- Did I clear the codes, verify the lamp behavior (bulb check on, off after self-test), and complete the OEM readiness sequence?
- Did I verify the system is armed and ready on the scan tool before returning the vehicle?
- Did I document every device replaced, the module status, and the readiness verification on the repair order?
- Did I confirm no active SRS codes and no crash-data faults remain after the readiness drive cycle?
