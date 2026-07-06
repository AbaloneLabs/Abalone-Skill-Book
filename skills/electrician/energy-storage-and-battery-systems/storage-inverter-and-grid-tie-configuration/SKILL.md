---
name: storage-inverter-and-grid-tie-configuration.md
description: Use when the agent is configuring storage inverters, setting grid-tie and islanding parameters, and programming charge and discharge profiles, covering grid-support modes, frequency-watt and volt-var, islanding detection, anti-islanding, utility interconnection agreement settings, and PCS commissioning.
---

# Storage Inverter and Grid-Tie Configuration

A storage inverter, or power conversion system (PCS), is the device that ties a battery to the grid or to a load, converting DC to AC and back, and it is also a grid-support and protection device whose settings determine how the storage system behaves under normal and abnormal grid conditions. The judgment problem is that configuration looks like entering nameplate values and enabling the system, which hides the grid-support functions that the utility and the interconnection agreement require, the anti-islanding protection that protects line workers and the public, and the charge and discharge profiles that determine whether the system delivers its economic and resilience purpose without abusing the battery. When these are missed, the system either trips off when the grid needs support, fails to disconnect when the grid is down, or runs a profile that ages the battery prematurely. This skill covers the grid-tie and islanding parameters, the grid-support modes, the charge and discharge programming, the utility interconnection requirements, and the commissioning that confirms it all.

## Core Rules

### Configure the Interconnection Settings to the Utility Agreement

Every grid-tied storage system operates under a utility interconnection agreement that specifies the points of common coupling, the protection settings, the power quality limits, and the conditions for connection and disconnection. The PCS must be configured to these settings exactly: the over and under voltage and frequency trip points and times, the reconnect delay, the power factor and harmonic limits, and any export limit or power control requirement. These are not defaults; they are contract and code values that vary by utility and by the size and type of interconnection. Configuring to the PCS defaults rather than to the agreement produces a system that either trips nuisance or, worse, operates out of compliance and endangers the grid and the line workers.

### Enable and Tune the Required Grid-Support Functions

Modern grids increasingly require storage and inverter-based resources to provide grid support, governed by standards such as IEEE 1547 and the local utility rule. The key functions are volt-var (the PCS injects or absorbs reactive power based on local voltage), frequency-watt (the PCS reduces real power output when frequency rises, or increases it when frequency falls), and volt-watt (the PCS reduces output when voltage is high). Each function has a curve with defined deadbands, slopes, and response times, set per the interconnection requirements. Enabling the wrong curve, leaving a function disabled, or setting a response time too slow means the system fails to support the grid when asked and may fail compliance testing. The functions are configured per the requirement and verified by injection during commissioning.

### Set Anti-Islanding Protection to Detect and Disconnect From a Downed Grid

When the grid loses power, a grid-tied storage inverter must detect the loss and disconnect, so that it does not energize a section of the grid that line workers believe is dead, a condition called islanding. PCS units use active anti-islanding methods, such as frequency or impedance injection, that detect the grid loss within the required time, plus passive voltage and frequency trip windows. The anti-islanding settings must be enabled and set to the required trip times, and the function must be tested. A PCS that continues to export onto a de-energized grid is a lethal hazard to line workers and the public, and it is the single most safety-critical setting in the configuration.

### Program Charge and Discharge Profiles for Purpose and Battery Health

The charge and discharge profile defines when the storage system charges from the grid or PV, when it discharges to the load or grid, and at what power and depth, and it must be programmed for the system's purpose (peak shaving, demand charge management, backup reserve, arbitrage) while respecting the battery's limits. The profile must observe the manufacturer's charge and discharge current limits, the state-of-charge window that preserves life (often avoiding full charge and full discharge), the temperature limits, and the reserved capacity for backup if the system serves resilience. A profile that cycles the battery too deep, too fast, or too often ages it prematurely and voids warranty. The profile is a coupled optimization of economics and battery health, not a simple schedule.

### Define the Islanding and Backup Transition Behavior

For systems that serve backup loads, the PCS must transition cleanly between grid-tied and islanded operation, isolating from the grid and forming a stable island for the backed-up loads within the required transfer time. The transition behavior, the loads on the backup side, the phase and voltage matching on reconnect, and the behavior of PV and other sources during the transition must all be defined and tested. A backup system that fails to transfer, drops loads on transition, or cannot synchronize on reconnect does not deliver its resilience purpose. The islanding transition is commissioned by simulating grid loss and restoration.

### Coordinate With PV, Generators, and Other On-Site Sources

Where the site has PV, a generator, or other sources alongside storage, the PCS must be coordinated with them: the charge from PV must be managed so excess PV charges the battery before export, the generator must not fight the inverter in island mode, and the controls must sequence the sources for stability and economy. Coordination is a system-level control design that defines which source leads, which follows, and how they hand off, and it is tested under the transitions the site will experience. Uncoordinated sources oscillate, trip, or fail to serve the load.

### Commission by Exercising Every Mode and Failure Path

Commissioning is not confirming that the system turns on; it is exercising every operating mode and failure path: grid-tied charge and discharge, each grid-support function by injection, anti-islanding by grid disconnect, island transition and reconnect, and the alarms and shutdowns. Each test is documented against the required response, and the settings are locked and recorded. A system commissioned only at the nominal operating point will pass on day one and fail in the first real event, because the conditions that matter were never exercised.

## Common Traps

### PCS Left on Default Settings Instead of the Interconnection Agreement

The installer enables the PCS with factory defaults for voltage, frequency, and reconnect, ignoring the utility agreement. The mechanism is that the defaults differ from the required settings, so the system trips at the wrong thresholds or reconnects too quickly, out of compliance with the interconnection. The false signal is that the system connects and exports. The harm is nuisance trips, utility non-compliance, and potential disconnection of the site by the utility.

### Grid-Support Function Disabled or Wrong Curve

The installer leaves volt-var or frequency-watt disabled, or sets a curve that does not match the requirement, because the system runs without them. The mechanism is that the grid then does not receive the support the interconnection requires, and the system may fail compliance testing or grid events. The false signal is that the PCS produces real power normally. The harm is non-compliance, possible curtailment, and a system that fails its grid role when the grid needs it.

### Anti-Islanding Disabled or Too Slow

The installer disables active anti-islanding to avoid nuisance trips, or sets the trip time too long. The mechanism is that on a grid loss the PCS continues to energize the line, creating an island that line workers may contact believing it is dead. The false signal is that the PCS runs stably on the grid. The harm is a lethal shock hazard to workers and the public, and a system that violates the most basic interconnection safety requirement.

### Profile That Deep-Cycles the Battery to Early Aging

The operator programs a peak-shaving profile that cycles the battery from full to empty daily to maximize savings, ignoring the state-of-charge window that preserves life. The mechanism is that deep, frequent cycling accelerates capacity loss and resistance growth, so the battery degrades far faster than its warranted life. The false signal is that the profile delivers strong economic returns early. The harm is premature capacity loss, warranty voiding, and a battery that must be replaced years early.

### Backup Transfer That Drops Loads or Fails to Form an Island

The backup system is not tested for the grid-loss transition, so it fails to form a stable island or drops the backed-up loads on transfer. The mechanism is that the transition logic, load sizing, or source sequencing is wrong, so the PCS trips or the loads fall during the very outage they were installed to survive. The false signal is that the PCS operates in grid-tied mode. The harm is a backup system that does not back up, discovered at the moment it is needed.

### Uncoordinated PV and Storage Oscillates or Trips

PV and storage are installed without coordinated control, so both regulate the island or the export independently. The mechanism is that the two sources fight each other, causing voltage or frequency swings that trip one or both, and the load is unserved. The false signal is that each source works alone. The harm is an unstable system that trips repeatedly and cannot serve the load in island or high-PV conditions.

## Self-Check

- Did I configure the PCS voltage, frequency, reconnect delay, power factor, harmonic, and export settings to the utility interconnection agreement exactly, rather than to factory defaults?
- Did I enable and tune the required grid-support functions (volt-var, frequency-watt, volt-watt) to the correct curves, deadbands, slopes, and response times per IEEE 1547 and the utility rule?
- Did I enable and set active anti-islanding protection to the required trip time, and verify it by grid-disconnect testing, so the PCS cannot energize a downed grid?
- Did I program charge and discharge profiles for the system's purpose while observing the manufacturer's current, state-of-charge, temperature, and cycle limits and reserving backup capacity?
- Did I define and test the islanding and backup transition, including isolation, island formation, load pickup, phase and voltage matching on reconnect, and the behavior of PV and other sources?
- Did I coordinate the PCS with PV, generators, and other on-site sources so that sources sequence and hand off without oscillation or tripping?
- Did I commission the system by exercising every operating mode and failure path, including grid-tied charge and discharge, each grid-support function by injection, anti-islanding, island transition, and alarms, documenting each against the required response?
- Does the output stay within the agent's scope, deferring licensed engineering, stamped design, utility and AHJ approval, and specialist commissioning to the qualified person where the question exceeds the agent's competence?
