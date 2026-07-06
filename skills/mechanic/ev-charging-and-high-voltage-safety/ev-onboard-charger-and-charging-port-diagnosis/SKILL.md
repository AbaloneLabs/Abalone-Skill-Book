---
name: ev-onboard-charger-and-charging-port-diagnosis.md
description: Use when the agent is diagnosing EV or plug-in hybrid AC charging faults, DC fast charging faults, charging port faults, onboard charger (OBC) DTCs, pilot and proximity signal faults, charge inlet overheating, or deciding whether a charging fault is the vehicle, the charger, the cable, or the charging station.
---

# EV Onboard Charger and Charging Port Diagnosis

An electric or plug-in hybrid vehicle charges through a charging port and an onboard charger (OBC) that converts AC to DC for the battery, and the DC fast charge path that bypasses the OBC and feeds the battery directly, and a charging fault can be in the vehicle, the cable, the charging station, or the communication between them. The judgment problem is that a "won't charge" complaint is ambiguous, and the source can be a damaged charging port, a failed OBC, a faulty DC fast charge contactor, a communication fault on the pilot or proximity signal, or an external charger or station — and the customer's experience does not distinguish them. A technician who condemns the OBC for a damaged cable, or who replaces the charging port for a station fault, hands back a vehicle that still will not charge. This skill covers the disciplined isolation of charging faults across the vehicle, the cable, and the station.

## Core Rules

### Reproduce the Fault With a Known-Good Charger and Cable Before Condemning the Vehicle

The disciplined charging diagnosis begins by reproducing the fault with a known-good charger and cable, because the most common cause of a "won't charge" complaint is an external charger, a damaged cable, a faulty wall outlet, or a charging station fault, not the vehicle. The disciplined technician tests the vehicle with a known-good Level 1 (120V) charger, a known-good Level 2 (240V) charger or EVSE, and, where available, a known-good DC fast charger, and confirms whether the fault follows the vehicle (the vehicle is at fault) or the charger (the external equipment is at fault). The technician also checks the wall outlet's wiring (a miswired or loose outlet causes charging faults) and the charging station's status. The tradeoff is that this testing requires known-good equipment, but condemning the vehicle for a bad charger is a frequent error.

### Inspect the Charging Port for Damage, Debris, and Overheating Signs

The charging port is the physical interface, and its damage is a common and visible cause of charging faults. The disciplined inspection checks every pin in the port (the AC pins, the DC fast charge pins, the pilot and proximity signal pins, the ground) for bending, burning, discoloration from overheating, corrosion, and debris (dirt, water, insect nests). A pin with discoloration or burning indicates a high-resistance connection that overheated, often from a worn or damaged connector on the cable, and the port and the cable connector must be replaced. A bent or corroded pin causes a poor connection and a charging fault. The technician verifies the port's locking mechanism engages (the vehicle locks the connector during charging for safety). The tradeoff is that the inspection is visual, but a damaged port is a frequent and easily missed cause of charging faults.

### Read the OBC and DC Fast Charge DTCs and the Communication Signals

The disciplined diagnosis reads the DTCs from the OBC and the DC fast charge controller (where separate), and scopes the pilot signal (the PWM signal that communicates the charger's available current) and the proximity signal (the signal that detects the connector's insertion and the cable's current rating). A pilot signal that is absent or out of spec indicates an OBC or a port fault; a proximity signal that is wrong indicates a connector or a port fault. The technician also reads the OBC's input and output data (the AC input voltage and current, the DC output voltage and current, the temperature) to determine whether the OBC is receiving power and converting it. The tradeoff is that this diagnosis requires a scan tool and a scope, but it distinguishes an OBC fault from a port or a communication fault.

### Distinguish AC Charging Faults From DC Fast Charging Faults

AC charging (Level 1 and Level 2) routes through the OBC, which converts the AC to DC for the battery; DC fast charging bypasses the OBC and feeds the battery directly through the DC fast charge contactors and the charging controller. The disciplined diagnosis distinguishes the two paths, because a fault in one does not necessarily affect the other. A vehicle that charges on AC but not on DC fast charge has a DC fast charge-specific fault (the DC contactors, the fast charge controller, the fast charge pins, the communication). A vehicle that does not charge on AC but does on DC fast charge has an OBC or an AC-specific fault. The disciplined technician tests both paths to isolate the fault. The tradeoff is that testing both paths requires both types of chargers, but it isolates the fault to the correct subsystem.

### Evaluate Thermal Management of the OBC and the Battery During Charging

The OBC and the battery generate heat during charging, and the vehicle's thermal management (the OBC's coolant loop, the battery's coolant loop, the radiator and fans) must dissipate this heat or the vehicle derates or aborts the charge to protect the components. The disciplined diagnosis checks the OBC and battery temperatures during charging (with the scan tool), the coolant level and flow in the OBC and battery loops, the radiator and fan operation, and the coolant pump. A charge that aborts when the OBC or battery reaches a threshold temperature indicates a thermal management fault (low coolant, a failed pump, a restricted radiator, a failed fan), not an OBC fault. The tradeoff is that the thermal check requires charging the vehicle and watching the temperatures, but an OBC replaced for a coolant pump fault is a needless expense.

## Common Traps

### Condemning the OBC for a Faulty Wall Outlet or Charging Station — The vehicle will not charge, the OBC is blamed, and the fault continues because the wall outlet is miswired or the station is faulty. The trap mechanism is that the external equipment is the most common cause, and it is not tested with a known-good charger. The false signal is the vehicle "not charging"; the harm is a needless OBC. The disciplined technician tests with a known-good charger and checks the outlet wiring.

### Replacing the Charging Port for a Bent Signal Pin Without the Cable — The charging port is burned or bent, the port is replaced, and the fault returns because the cable connector that caused the damage is still in use. The trap mechanism is that a damaged cable connector causes the port damage, and the cable is not replaced. The false signal is the port being the visible damage; the harm is a repeat port failure. The disciplined technician replaces the port and the cable connector together.

### Condemning the OBC for a Thermal Management Fault — The charge aborts after a few minutes, the OBC is blamed, and the fault returns because the coolant pump or the fan is failed and the OBC overheated. The trap mechanism is that the OBC aborted the charge to protect itself from overheating, and the thermal management fault is not diagnosed. The false signal is the charge abort; the harm is a needless OBC. The disciplined technician checks the OBC and battery temperatures and the thermal management.

### Treating a DC Fast Charge Fault as an OBC Fault — The vehicle will not DC fast charge, the OBC is blamed, and the fault continues because the OBC is not in the DC fast charge path. The trap mechanism is that the DC fast charge bypasses the OBC, and the fault is in the DC contactors or the fast charge controller. The false signal is the vehicle "not charging"; the harm is a needless OBC. The disciplined technician distinguishes the AC and DC paths.

### Missing a Damaged or Debris-Filled Charging Port — The vehicle intermittently will not charge, the OBC is diagnosed, and the cause is corrosion or debris in the charging port causing a poor connection. The trap mechanism is that the port's damage is not inspected, and the intermittent fault is blamed on the OBC. The false signal is the intermittent nature; the harm is a needless OBC. The disciplined technician inspects the port for damage and debris.

## Self-Check

- Did I reproduce the fault with a known-good Level 1, Level 2, and DC fast charger before condemning the vehicle?
- Did I inspect every pin in the charging port for bending, burning, corrosion, and debris, and check the locking mechanism?
- Did I read the OBC and DC fast charge DTCs, and scope the pilot and proximity signals?
- Did I distinguish an AC charging fault (OBC path) from a DC fast charging fault (direct path)?
- Did I check the OBC and battery temperatures and the thermal management (coolant, pump, fan, radiator) during charging?
- For a damaged port, did I replace the port and the cable connector together?
- After the repair, did I verify AC and DC charging (where applicable) to full or to the expected level with no aborts?
- Did I document the test chargers used, the DTCs, the signal readings, and the repair on the repair order?
