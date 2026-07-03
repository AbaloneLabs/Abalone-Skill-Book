---
name: srs-airbag-system-service-and-diagnosis.md
description: Use when the agent is diagnosing SRS airbag warning lights, reading airbag module codes, servicing steering wheel clockspring, evaluating seat belt pretensioner and occupant detection circuits, or performing safe airbag deployment and module replacement.
---

# SRS Airbag System Service and Diagnosis

The Supplemental Restraint System (SRS) is the highest-stakes electrical system on a vehicle because its components are explosive, its diagnostics carry risk of accidental deployment, and its failures have direct life-safety consequences. The judgment problem is that an SRS warning light can be set by dozens of causes—a failed clockspring, an open seat belt pretensioner, a corrupted occupant classification sensor, a low battery voltage event, or a prior crash that deployed a bag—and the codes do not always point clearly to the failed component. The technician must approach every SRS job with disciplined safety procedures, must interpret codes in context of the vehicle's crash history, and must never take shortcuts with a system designed to detonate. A misdiagnosed or improperly repaired SRS can fail to deploy in a crash, deploy unexpectedly in the shop, or leave the warning light on, all of which are unacceptable outcomes.

## Core Rules

### Safety Procedures Before Any SRS Work

Before connecting a scan tool, probing a connector, or disconnecting any SRS component, disable the system to prevent accidental deployment. The universal procedure is: turn the ignition off, remove the key, disconnect the negative battery cable, and wait the manufacturer-specified time (typically 2 to 10 minutes) for the backup power capacitors in the airbag control module to discharge. This wait is non-negotiable because the module stores enough energy to fire the bags even after battery disconnect, and the capacitors must drain. Never probe the wires to an airbag or pretensioner with a test light or a multimeter that sources current, because even small currents can trigger the squib. Always carry a deployed or undeployed airbag module with the trim cover facing away from your body, store modules on a flat surface trim-up (never stacked), and never test an airbag module with resistance measurement—there is no safe bench test for a live squib. When disposing of an undeployed module, follow the manufacturer's deployment procedure, never simply discard it.

### Reading and Interpreting SRS Codes

SRS codes are manufacturer-specific in numbering and meaning, and they require a scan tool with the correct software; generic OBD-II readers will not read airbag codes on most vehicles. Read all codes, both active and stored, and note whether each is current, history, or pending. A code for "driver airbag circuit open" most commonly points to the clockspring, which is the flexible ribbon connector between the steering wheel and the column that breaks from steering rotation over millions of cycles. A code for "seat belt pretensioner circuit" points to the pyrotechnic tensioner at the seat belt buckle or retractor, which fires in a crash to remove slack. A code for "occupant classification system" points to the weight-sensing system in the passenger seat that decides whether to arm or suppress the passenger airbag. Always check for multiple related codes, because a single event—like a low battery during cranking—can set codes across several SRS components that are all healthy, and clearing the codes after restoring battery voltage resolves the issue. Never clear codes without recording them first.

### Clockspring Diagnosis and Replacement

The clockspring is the most common SRS failure because it flexes with every steering input and eventually fatigues the ribbon. Symptoms include an SRS light with a driver airbag circuit code, an inoperative horn, and inoperative steering wheel controls (cruise, audio, etc.), because all of these circuits pass through the clockspring. Diagnose by checking continuity through the clockspring for the airbag circuit and the horn circuit; an open on either confirms the failure. When replacing the clockspring, the steering wheel and column must be locked in the straight-ahead position before removal to prevent the new clockspring from being installed off-center, which would break it on the first full steering lock. The new clockspring may come with a locking pin that holds the rotor at center; leave the pin in until the clockspring is installed, then remove it. Always follow the manufacturer's torque specification for the steering wheel nut and the clockspring mounting screws, and perform any required steering angle sensor or SRS calibration after replacement.

### Occupant Classification and Seat Sensor Service

Modern vehicles use an occupant classification system (OCS) to detect whether the passenger seat is occupied and by what size occupant, and to decide whether to deploy the passenger airbag. The system uses either a pressure-sensitive bladder with a mat sensor, a strain-gauge weight sensor in the seat rails, or a more advanced system with multiple sensors and a classification module. Failures produce an SRS light and a "passenger airbag off" indicator that behaves incorrectly—staying on with an adult in the seat, or turning off with an empty seat. Diagnose with a scan tool that reads the OCS module, checking the weight reading, the classification result, and the sensor voltages. Many systems require a zero-weight calibration after seat service or sensor replacement, performed with the seat empty and level; skipping this causes misclassification. Be aware that aftermarket seat covers, seat heaters, or objects on the seat can interfere with the OCS, and that spilling liquid on a mat sensor can damage it. Never bypass or defeat the OCS, because an incorrectly classified occupant may have the airbag suppressed in a crash.

### Module Replacement, Crash Data, and Deployment Loops

After a crash in which any airbag or pretensioner deployed, the SRS control module must be replaced on most vehicles because it fires one-time pyrotechnic loops and stores crash data that cannot be cleared. Some manufacturers allow the module to be reused after a minor crash if no deployment occurred and the codes clear; always check the service information for the deployment threshold. When replacing the module, transfer the crash data if required for legal or insurance purposes, and install the new module with the correct torque and orientation. All deployed components—the airbag, the pretensioner, the module, and often the clockspring and the seat belt buckle—must be replaced, not repaired. Inspect the mounting points and the wiring harness for damage from the deployment forces; a bent mounting bracket can misalign a new sensor and cause future malfunction. After reassembly, clear the codes, perform any required calibration, and verify the SRS light illuminates at key-on and extinguishes after the bulb check with no remaining codes.

### Seat Belt and Pretensioner Considerations

Seat belt pretensioners are pyrotechnic devices that fire in a crash to retract the belt and remove slack, and they are part of the SRS. After any deployment, the pretensioner and usually the entire seat belt retractor must be replaced, because the pyrotechnic charge is one-time. A seat belt that will not retract or extend freely after a crash may have a deployed pretensioner, even if the belt looks intact. Diagnose pretensioner circuit codes by checking resistance at the connector (typically 1 to 3 ohms) and by inspecting for the deployment indicator (a small flag or marker that changes position when fired). Never attempt to reset or repair a deployed pretensioner. On some vehicles, the seat belt buckle contains the pretensioner and the buckle switch that tells the SRS whether the belt is fastened; a failed buckle switch can set a code and disable the airbag system. Always replace the entire seat belt assembly as a unit after deployment, using OEM parts, because aftermarket belts may not have the correct load limiters or pretensioner calibration.

## Common Traps

### Not Waiting for Capacitor Discharge After Battery Disconnect

The most dangerous trap is beginning SRS work immediately after disconnecting the battery, without waiting for the module's backup capacitors to discharge. The mechanism is that the airbag module stores enough energy in capacitors to fire the bags for several minutes after battery disconnect, as a safety measure in case battery power is lost in a crash before deployment. The false signal is that the ignition is off and the battery is disconnected, so the system appears dead. The harm is that probing or disconnecting an airbag connector while the capacitors are charged can trigger deployment, causing serious injury or death from the explosive force of the bag deploying at close range in the shop. Always wait the full manufacturer-specified time, typically 2 to 10 minutes, after battery disconnect before touching any SRS component.

### Probing SRS Circuits With a Test Light

A second trap is using a test light or a current-sourcing multimeter to probe airbag or pretensioner circuits. The mechanism is that these tools can inject current into the squib circuit, and the airbag firing squib requires only a small current to ignite; a test light that draws 50-200 mA can easily exceed the firing threshold. The false signal is that the test light "should be fine" because it works on other circuits. The harm is accidental deployment of the airbag or pretensioner, with risk of injury, damage to the vehicle, and the cost of replacing the deployed components. Always use only a high-impedance digital multimeter (10 megohm input) on SRS circuits, never a test light, and never source current into a squib circuit.

### Clearing Codes Without Recording Them

A third trap is clearing SRS codes before recording them, which destroys the diagnostic history and makes intermittent faults much harder to find. The mechanism is that the SRS module stores current, history, and pending codes that together tell the story of when and how the fault occurred, and clearing wipes that context. The false signal is that clearing "resets" the system and the light goes off, which feels like progress. The harm is that if the code returns, the technician has lost the freeze frame and the related codes that pointed to the root cause, extending diagnosis time and risking a wrong repair. Always photograph or write down every code with its status before clearing anything, and only clear after the repair is complete.

### Reusing Seat Belts or Pretensioners After a Crash

A fourth trap is reusing a seat belt or pretensioner that was in a vehicle during a crash, even if it appears intact. The mechanism is that the pyrotechnic pretensioner may have fired internally without visible damage, or the load limiter may have stretched the belt webbing, and these one-time safety devices cannot protect in a second crash. The false signal is that the belt retracts and latches normally and looks undamaged. The harm is that in a subsequent crash, a stretched belt or spent pretensioner fails to restrain the occupant, with catastrophic injury consequences, and the shop that performed the post-crash repair bears liability. Always replace all seat belts and pretensioners in the vehicle after any crash that deployed any SRS component, using OEM parts.

### Skipping Calibration After Occupant Sensor or Module Replacement

A fifth trap is replacing an occupant classification sensor, a seat, or the SRS module without performing the required calibration. The mechanism is that the OCS must learn the empty-seat weight as a zero reference, and the module must learn the sensor characteristics, and without calibration the system misclassifies the occupant—suppressing the airbag for an adult or arming it for an empty seat or a child seat. The false signal is that the SRS light is off and no codes are set, so the system appears functional. The harm is that the passenger airbag may fail to deploy for an adult in a crash, or may deploy unnecessarily, both of which are life-safety failures. Always perform the factory zero-weight calibration after any seat or OCS service, and verify the "passenger airbag off" indicator behaves correctly with an adult and an empty seat.

## Self-Check

- Did I disconnect the battery and wait the full specified time (2-10 minutes) for capacitor discharge before touching any SRS component?
- Did I avoid using test lights or current-sourcing meters on any airbag or pretensioner circuit?
- Did I record all SRS codes with their status before clearing any?
- Did I diagnose clockspring failure by checking horn and steering wheel control function alongside the airbag circuit code?
- Did I lock the steering straight-ahead before removing and replacing the clockspring?
- Did I read the occupant classification system data with a capable scan tool and check weight and classification readings?
- Did I perform the zero-weight OCS calibration after any seat or sensor service?
- Did I replace all deployed components—airbag, pretensioner, module, clockspring, and seat belt—after a crash, using OEM parts?
- Did I inspect mounting points and wiring harnesses for deployment damage before installing new components?
- Did I verify the SRS light illuminates at key-on and extinguishes after the bulb check with no remaining codes?
