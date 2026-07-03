---
name: ev-charging-and-hv-battery-diagnosis.md
description: Use when the agent is diagnosing electric vehicle charging faults, onboard charger and DC fast charge issues, high-voltage battery state of health, cell imbalance, range degradation, or evaluating battery thermal management and contactor concerns.
---

# EV Charging and HV Battery Diagnosis

Electric vehicle charging and battery health are the defining service concerns of the EV era, and they require a fundamentally different diagnostic mindset from internal combustion vehicles. The judgment problem is that a "won't charge" or "short range" complaint can originate in the charging equipment (EVSE or DC fast charger), the vehicle's onboard charger (OBC), the charge port, the high-voltage battery, the battery thermal management system, or the software and communication protocols that govern charging—and the customer almost always blames the battery first. The technician must distinguish charging infrastructure faults from vehicle faults, must interpret battery state-of-health data that is not as straightforward as it appears, and must understand that battery degradation is gradual and predictable while sudden range loss usually points to a different fault.

## Core Rules

### Distinguishing AC, DC, and Level Considerations

EV charging happens at three levels with different equipment and vehicle-side components. Level 1 (120V AC, ~1.4 kW) and Level 2 (240V AC, 3.3-19.2 kW) use the vehicle's onboard charger (OBC) to convert AC to DC and charge the battery. DC fast charging (Level 3, 50-350+ kW) bypasses the OBC and delivers DC directly to the battery through a separate charge port and contactor path, controlled by the battery management system (BMS). A "won't charge" complaint must first be localized to the charging level: if the vehicle charges on DC fast but not on AC Level 2, the fault is in the OBC, the AC charge port, or the pilot/proximity communication; if it charges on AC but not DC fast, the fault is in the DC charge port, the DC contactors, or the fast-charge communication. Always ask the customer which charging method fails and test with a known-good EVSE before condemning vehicle components, because a faulty home EVSE or a tripped ground-fault circuit is a common cause of "vehicle won't charge" that requires no vehicle repair.

### Onboard Charger and Charge Port Diagnosis

The onboard charger (OBC) is a high-power electronic module that converts AC to DC, manages the charge profile, and communicates with the EVSE via the pilot signal (a 1 kHz PWM signal that indicates current capacity) and the proximity signal (which detects plug insertion). OBC faults produce "charge fault" or "unable to charge" messages, often with codes for input voltage, ground monitoring, temperature, or communication. Diagnose with a scan tool that reads the OBC data: input voltage, output current, temperature, and pilot signal status. A common OBC fault is ground isolation monitoring failure, where the OBC detects that the vehicle chassis is not properly isolated from the grid and refuses to charge for safety; this can be caused by moisture in the charge port, a damaged charge cable, or an actual isolation fault in the HV system. The charge port itself can fail from worn or bent pins, water intrusion, or overheating from a poor connection; inspect the port for discoloration, pin damage, and debris, and check the temperature sensors in the port. Always test with a second, known-good charge cable before condemning the OBC.

### Battery State of Health Versus State of Charge

Battery state of charge (SOC) is the current energy level, analogous to a fuel gauge; battery state of health (SOH) is the battery's capacity relative to when it was new, expressed as a percentage. SOH degrades gradually over time and cycles—typically 1-2 percent per year for most EVs, faster in hot climates and with frequent DC fast charging—and a vehicle at 90 percent SOH has 10 percent less range than new, which is normal aging, not a fault. Sudden range loss, however, almost always points to a fault: a failed cell module that the BMS has isolated, a thermal management problem causing the BMS to limit charge/discharge to protect the battery, or a software issue with the range estimate. Diagnose range complaints by reading the BMS data: SOC, SOH, cell voltage spread (the difference between the highest and lowest cell), cell temperature spread, and any DTCs. A large cell voltage spread (typically above 100-200 mV, per the manufacturer) indicates cell imbalance or a failing cell module, and may require module replacement rather than the full pack. A battery that has degraded gradually to, say, 85 percent SOH is not faulty; it is aged, and the customer should be informed that this is expected.

### Cell Imbalance and Module-Level Faults

A high-voltage battery pack is made of many cells (often hundreds) grouped into modules, and the cells are managed by the BMS which monitors each cell's voltage and temperature. Over time, cells age at different rates, and the weakest cell limits the pack: during discharge, the weakest cell reaches minimum voltage first and the BMS shuts down the pack to protect it; during charge, the strongest cell reaches maximum voltage first and the BMS stops charging. This is cell imbalance, and it manifests as reduced usable capacity (lower range) even though the nominal pack capacity is unchanged. The BMS data shows the cell voltage spread; a large spread that persists after a balance charge (which the BMS performs slowly during idle periods) indicates a failing cell or module. Some packs can be serviced at the module level—replacing the failed module rather than the full pack—but this requires the manufacturer's procedure, calibration, and often software to re-learn the pack configuration. Never attempt to service a pack without the HV disable procedure and the manufacturer's module-level service procedure.

### Thermal Management System Diagnosis

Battery temperature is critical to performance, longevity, and safety, and EVs use active thermal management—liquid cooling, air cooling, or refrigerant-based cooling—to keep the battery in its optimal range (typically 20-40 degrees C). A thermal management fault causes the BMS to limit charge rate (especially DC fast charge, which generates heat) and discharge rate, producing a "reduced power" or "charge slowed" message and reduced range. Diagnose by reading the battery temperature data: inlet and outlet coolant temperature, cell temperature spread, and any thermal codes. Common faults include a failed coolant pump, a stuck thermostat or diverter valve, a clogged cooling plate, low coolant, or a failed chiller (on refrigerant-based systems). A battery that overheats during DC fast charge but not during AC charge points to a thermal management limitation, not a battery fault. Always verify the thermal management system is functioning before condemning the battery, because a hot battery that is limited by the BMS is protecting itself, not failing.

### Contactor and High-Voltage Connection Faults

The HV battery is connected to the vehicle through two main contactors (positive and negative) that close to energize the system and open to isolate it. Contactor faults produce "check EV system," "power reduced," or "pull over safely" messages, often with codes for contactor weld, contactor open, or high circuit resistance. A welded contactor (the contacts have arced and fused closed) means the system cannot be isolated by the BMS, which is a safety fault; a contactor that will not close means the vehicle will not drive. Diagnose with a scan tool that reads the contactor status and the HV circuit resistance, and follow the manufacturer's procedure, which may involve a contactor diagnostic or a resistance measurement at the service disconnect. High-resistance connections within the pack or at the bus bars produce heat and voltage drop, leading to power limitation and potential thermal damage; these are often detected by the BMS as a high-resistance fault and may require pack inspection. Always perform the HV disable and verify zero voltage before any contactor or pack inspection.

## Common Traps

### Blaming the Vehicle for a Charging Infrastructure Fault

The most common trap is diagnosing a vehicle charging fault when the EVSE or electrical circuit is the problem. The mechanism is that a faulty home EVSE, a tripped GFCI, a loose receptacle, an undersized circuit, or a damaged public charger all prevent charging and the customer reports "the car won't charge," directing attention to the vehicle. The false signal is the vehicle displaying a charge error, which implicates the vehicle. The harm is that the technician spends hours diagnosing the OBC and charge port while the real fault is in the customer's garage wiring or the EVSE, the customer pays for unnecessary diagnosis, and the problem recurs at home. Always test the vehicle with a known-good EVSE on a verified circuit before condemning vehicle components, and ask the customer about their home charging setup.

### Confusing Gradual Degradation With a Fault

A second trap is diagnosing a battery fault when the vehicle is experiencing normal age-related degradation. The mechanism is that all lithium batteries lose capacity over time and cycles, and a vehicle at 80-90 percent of original SOH has proportionally less range, which the customer perceives as a problem. The false signal is the reduced range and the lower SOH number, which look like a fault. The harm is that the technician tests the battery, finds no fault, but the customer is dissatisfied because they expected a repair, or worse, the technician recommends an unnecessary battery replacement. Always compare the SOH to the expected degradation curve for the vehicle's age and climate, read the cell voltage spread to confirm pack health, and communicate to the customer that gradual degradation is normal and not a fault.

### Condemning the Battery for a Thermal Management Limitation

A third trap is condemning the battery when the thermal management system is limiting charge or discharge to protect it. The mechanism is that a failed coolant pump, low coolant, or a stuck valve causes the battery to run hot, and the BMS reduces power and charge rate to keep the cells in safe limits, producing "reduced power" and slow charging that look like a battery fault. The false signal is the power limitation and the battery-related warning message. The harm is that the technician recommends a battery replacement while the real fault—a coolant pump or valve—is a fraction of the cost, and the new battery would also be limited by the same thermal fault. Always verify the thermal management system (coolant pump, level, flow, valve operation, chiller) before condemning the battery.

### Diagnosing a Cell Imbalance Without Performing a Balance Cycle

A fourth trap is condemning a cell or module for a voltage spread that would resolve with a balance charge. The mechanism is that the BMS balances cells slowly over time during idle and at the top of charge, and a vehicle that has not been fully charged recently may show a temporary voltage spread that is not a cell fault. The false signal is the voltage spread on the scan tool, which looks like a failing cell. The harm is that the technician recommends module or pack replacement for a spread that would have resolved with a full charge and an idle period for balancing. Always perform a full charge and allow the BMS to balance before evaluating a cell voltage spread, and re-check the spread after the balance cycle.

### Servicing the Pack Without Proper HV Disable and Verification

A fifth trap is opening a battery pack or contacting HV components without performing the full disable and zero-voltage verification. The mechanism is that the pack stores lethal voltage at all times (unlike the vehicle bus, which is isolated by contactors when off), and the cells are directly connected to the bus bars inside the pack, so any contact is across the full pack voltage. The false signal is the vehicle being off and the contactors being open, which does not isolate the pack internally. The harm is lethal electrocution from direct cell contact, or an arc flash from shorting a tool across bus bars. Always perform the full HV disable, remove the service disconnect, verify zero voltage at the pack terminals, and wear Class 0 gloves before opening or contacting any pack internal component.

## Self-Check

- Did I localize the charging fault to AC Level 1/2, DC fast charge, or all methods before diagnosing components?
- Did I test the vehicle with a known-good EVSE on a verified circuit before condemning the OBC or charge port?
- Did I read the OBC data (input voltage, output current, temperature, pilot status) with a capable scan tool?
- Did I distinguish state of charge from state of health and compare SOH to expected age-related degradation?
- Did I read the cell voltage spread and confirm it persists after a full charge and balance cycle before condemning a cell or module?
- Did I verify the battery thermal management system (coolant pump, level, flow, valves, chiller) before condemning the battery?
- Did I check the charge port for pin damage, discoloration, water intrusion, and temperature sensor operation?
- Did I read the contactor status and HV circuit resistance for "check EV system" or power reduction complaints?
- Did I perform the full HV disable and verify zero voltage before any pack or contactor inspection?
- Did I wear Class 0 gloves and use HV-rated tools when working near or inside the battery pack?
