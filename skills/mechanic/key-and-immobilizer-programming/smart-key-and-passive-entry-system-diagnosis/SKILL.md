---
name: smart-key-and-passive-entry-system-diagnosis.md
description: Use when the agent is diagnosing push-button start faults, smart key not detected, passive entry and lock faults, keyless go system issues, door handle touch sensor faults, or deciding whether a smart key fault is the key fob, the vehicle antenna, the entry handle, the keyless module, or a low-voltage condition.
---

# Smart Key and Passive Entry System Diagnosis

A smart key (passive entry, push-button start, keyless go) system allows the driver to enter and start the vehicle with the key fob in their pocket, by using multiple low-frequency antennas around the vehicle that wake the key and verify its presence. The judgment problem is that a "key not detected" or "passive entry not working" complaint can be the key fob (a dead battery, a failed transmitter), a vehicle antenna, the door handle touch sensor, the keyless module, a low vehicle battery, or interference — and the system's behavior depends on which antennas and which path are involved. A technician who replaces the key fob for a faulty door handle antenna, or who condemns the keyless module for a low vehicle battery, hands back a vehicle with the same fault. This skill covers the disciplined isolation of smart key and passive entry faults.

## Core Rules

### Test With a Known-Good Key Fob Before Condemning the Vehicle

The disciplined smart key diagnosis begins by testing with a known-good, programmed key fob, because the key fob (with its internal battery, its low-frequency receiver, and its RF transmitter) is the most common cause of smart key faults, and a faulty fob mimics a vehicle fault. The disciplined technician tests the suspect fob's battery (replaces it with a fresh battery of the correct type), tests the fob's RF transmission (with the scan tool's key ID reception or a known-good reception), and tests the system with a second, known-good programmed fob. If the system works with the second fob, the first fob is the fault; if the system fails with both fobs, the vehicle is at fault. The tradeoff is that this test requires a second programmed key, but condemning the vehicle for a faulty fob is a frequent error.

### Isolate the Fault by the Location and the Function That Fails

The disciplined diagnosis isolates the fault by which location and which function fails, because the smart key system uses multiple antennas and multiple paths, and a localized fault points to a specific antenna or handle. The disciplined technician tests the passive entry at every door handle (if one handle does not wake the system but the others do, the fault is that handle's touch sensor or its antenna), tests the passive entry at the trunk or liftgate (a separate antenna), and tests the push-button start (which uses an interior antenna). A fault at one location points to that location's antenna or sensor; a fault at all locations points to the keyless module, the key fob, or a system-level condition. The tradeoff is that this testing takes walking around the vehicle, but it isolates the fault to the correct component.

### Evaluate the Vehicle's Low-Frequency Antennas and the Door Handle Sensors

The vehicle's low-frequency (LF) antennas (in the door handles, the trunk, the interior, and sometimes the rear bumper) wake the key fob and verify its location, and their faults disable the passive entry at their location. The disciplined diagnosis reads the keyless module's antenna status and DTCs (an open or shorted antenna sets a code), checks the antenna's resistance and continuity (per the OEM spec), and inspects the door handle touch sensors (the capacitive or the switch sensors in the handle) for damage, water intrusion, and disconnection. A door handle that has been in a collision or has water intrusion often has a damaged sensor or antenna. The tradeoff is that the antenna and sensor checks require the wiring diagram and trim removal, but condemning the keyless module for a door handle sensor is a frequent error.

### Check the Vehicle Battery Voltage and the Keyless Module's Power and Ground

The smart key system is sensitive to low voltage, because the LF antennas and the keyless module require full voltage to transmit and to process the key's response, and a low vehicle battery (from a draw, a failing battery, or a recent discharge) causes intermittent "key not detected" faults, especially after the vehicle has been sitting. The disciplined diagnosis checks the vehicle battery's voltage and health (a battery below 12.2V or with high internal resistance causes keyless faults), the keyless module's power and ground at its connector, and the system's behavior after a full charge. A smart key fault that appears after the vehicle sits and disappears after a drive points to a low battery. The tradeoff is that the battery check is quick, but condemning the keyless module for a low battery is a frequent error.

### Verify Interference and External RF Sources Are Not the Cause

The smart key system operates on low-frequency and RF signals that can be interfered with by external sources (a cellular tower, a radio transmitter, a nearby vehicle's keyless system, an aftermarket accessory with a strong RF emission, or even a USB charger in the cabin), and the interference causes intermittent "key not detected" faults in specific locations. The disciplined diagnosis asks the customer about the location and the pattern of the fault (does it happen in a specific parking spot, at home, at work?), tests the system in a different location, and checks for aftermarket accessories that emit RF. The tradeoff is that the interference check requires questioning and relocation testing, but condemning a component for an interference fault is a needless expense.

## Common Traps

### Replacing the Key Fob for a Door Handle Antenna or Sensor Fault — The passive entry does not work at one door, the key fob is blamed, and the cause is the door handle's touch sensor or its LF antenna. The trap mechanism is that the fob is the common guess, and the localized fault is not isolated. The false signal is the passive entry not working; the harm is a needless fob and programming. The disciplined technician isolates the fault by location.

### Condemning the Keyless Module for a Low Vehicle Battery — The "key not detected" appears after the vehicle sits, the keyless module is blamed, and the cause is a low vehicle battery that cannot power the LF antennas. The trap mechanism is that the low voltage causes the intermittent fault, and the battery is not checked. The false signal is the intermittent "key not detected"; the harm is a needless module. The disciplined technician checks the vehicle battery.

### Replacing the Push-Button Start for an Interior Antenna Fault — The push-button start shows "key not detected," the start button is blamed, and the cause is the interior LF antenna that verifies the key's presence in the cabin. The trap mechanism is that the interior antenna's fault prevents the key detection, and the antenna is not checked. The false signal is the "not detected" at the start button; the harm is a needless start button. The disciplined technician checks the interior antenna.

### Missing Water Intrusion in a Door Handle Sensor — The passive entry fails after rain or a car wash, the keyless module is blamed, and the cause is water intrusion into the door handle's touch sensor. The trap mechanism is that the water intrusion causes the intermittent fault, and the handle is not inspected. The false signal is the intermittent failure; the harm is a needless module. The disciplined technician inspects the handle sensor for water.

### Diagnosing a Hardware Fault for an Interference Issue — The smart key fails in a specific location, the keyless module is blamed, and the cause is RF interference from an external source. The trap mechanism is that the interference mimics a hardware fault, and the location is not considered. The false signal is the fault in that location; the harm is a needless diagnosis. The disciplined technician tests in a different location and asks about the pattern.

## Self-Check

- Did I test the system with a known-good, programmed key fob and a fresh fob battery before condemning the vehicle?
- Did I isolate the fault by testing the passive entry at every door handle, the trunk, and the push-button start?
- Did I read the keyless module's antenna status and DTCs, and check the antenna resistance and continuity?
- Did I inspect the door handle touch sensors for damage, water intrusion, and disconnection?
- Did I check the vehicle battery voltage and health and the keyless module's power and ground?
- Did I ask the customer about the location and pattern of the fault, and test in a different location to rule out interference?
- After the repair, did I verify the passive entry, the push-button start, and the passive locking at all locations?
- Did I document the isolation tests, the component findings, and the repair on the repair order?
