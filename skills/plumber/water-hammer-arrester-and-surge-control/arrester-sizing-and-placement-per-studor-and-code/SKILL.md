---
name: arrester-sizing-and-placement-per-studor-and-code.md
description: Use when the agent is sizing or placing a water hammer arrester, selecting between mechanical and air-chamber types, applying PDI-WH201 fixture-unit and shock-pressure tables, locating arresters near quick-closing valves, planning multiple arresters for long branch runs, or interpreting IPC 604.9 and manufacturer placement guidance.
---

# Arrester Sizing and Placement per Studor and Code

A water hammer arrester is a sealed pressure vessel that absorbs a transient by compressing a gas charge separated from the water, and whether it actually stops the hammer depends entirely on matching its size to the surge energy and placing it where the surge can reach it before reflecting. The judgment problem is that an arrester is often chosen by pipe size or convenience, installed at the water heater far from the trigger, or replaced with a short capped pipe "air chamber" that waterlogs within months, and each of these errors produces a system that looks protected but is not. Agents miss the failure because a wrong arrester still dampens the worst of the bang initially, so the symptom partially improves and the installer moves on, while the residual transient continues to fatigue fittings. This skill covers sizing per PDI-WH201 and manufacturer tables, placement within the effective distance of the trigger, the multiple-arrester decision for long runs, and the mechanical-versus-air-chamber choice, to prevent the recurring hammer and waterlogged-chamber failures that follow undersized or mislocated devices.

## Core Rules

### Size the Arrester From Fixture Units and Shock Pressure, Not From Pipe Diameter Alone

Arrester capacity is a function of the surge energy it must absorb, which depends on the flow rate, the pipe length between the trigger and the source, and the pipe size, and the standard sizing method is PDI-WH201 (and the equivalent ASSE 1710 manufacturer tables) that express required arrester size in fixture units and shock-pressure categories. The workflow is to determine the fixture-unit load of the quick-closing device (a washing machine, a dishwasher, an ice maker each carry defined values), measure or estimate the developed length of pipe from the device back to the source or main, identify the pipe size, and read the required arrester size from the table. A device sized only to match the pipe thread will almost always be undersized for a long branch, because surge energy grows with pipe length. The discipline is to treat the pipe length and the fixture-unit load as the primary inputs, to select a size from the PDI-WH201 or manufacturer table that covers both, and to round up rather than down when the load falls between sizes.

### Place the Arrester Within the Effective Distance of the Trigger, Ideally Within 6 Feet

An arrester only absorbs a surge if the surge reaches it before it reflects off a termination and amplifies, and because the transient travels at the acoustic velocity of water (roughly 4000 to 5000 ft/s), distance is decisive. Manufacturer guidance and field practice place the arrester as close as practical to the quick-closing device, ideally within about 6 feet, and between the device and the source of the shock. For a fixture like a washing machine, the arrester goes on the branch feeding the box; for a dishwasher or ice maker, on the branch feeding the appliance. The worst placement is at the water heater or the main, which is typically the farthest point from every fixture trigger; the surge reflects and amplifies before it arrives, and the arrester sees only a fraction of the peak. The discipline is to locate each arrester at or near its specific trigger, never at a central "convenient" point, and to confirm the device sits between the trigger and the source.

### Use Multiple Arresters for Long Branch Runs and at the End of Dead-End Branches

A single arrester cannot protect a long branch with multiple quick-closing fixtures, because the surge from the farthest fixture decays and reflects before reaching a centrally located device. For branch runs exceeding roughly 20 feet to the first fixture, or for branches feeding several appliances (a laundry group plus a nearby bathroom, for example), the design is multiple arresters: one at or near each quick-closing fixture, and where the branch is a long dead-end, an arrester at the end of the run where the reflected surge is strongest. IPC 604.9 and equivalent UPC provisions require arresters on quick-closing valves, and the field interpretation for long or multi-fixture branches is to treat each trigger as its own protection point. The discipline is to map every quick-closing fixture on the branch, measure the developed length to each, and specify an arrester for each trigger plus an end-of-branch unit where the run is long, rather than assuming one central arrester covers the branch.

### Specify a Mechanical Arrester and Never Rely on a Plain Pipe Air Chamber

A plain capped pipe used as an "air chamber" has no barrier between the air and the water, and the air dissolves into the pressurized water over weeks to months until the chamber is full of water and provides no cushion at all, at which point the hammer returns as if nothing were installed. A mechanical arrester (a sealed chamber with a diaphragm or piston separating a gas charge from the water) does not waterlog, because the gas cannot dissolve across the barrier, and it retains its cushion for the life of the seal. The discipline is to specify only mechanical (diaphragm or piston) arresters for any permanent installation, to size and place them per the tables, and to treat any existing air chamber as a failed device that must be replaced, not recharged. Where an air chamber must be retained temporarily, it should be drained and re-aerated on a schedule, but this is a maintenance burden that mechanical arresters eliminate.

### Confirm Accessibility, Orientation, and Code Documentation for Every Arrester

A mechanical arrester is a wearing component whose internal seal or charge will eventually fail, and a failed arrester hidden behind a finished wall is undetectable until the hammer returns or the seal leaks. The discipline is to install every arrester in an accessible location (in a cabinet, behind an access panel, or in a mechanical room), in the orientation permitted by the manufacturer (most piston and diaphragm types allow any orientation but some specify vertical), and with the installation documented for inspection and maintenance. IPC 604.9 and local amendments may require arresters on all quick-closing valves, and the inspection record should note each arrester's location, size, and the trigger it serves. Concealing an arrester without access is a latent defect: the device cannot be tested or replaced, and the next occupant inherits an unprotected system.

## Common Traps

### Sizing the Arrester to the Pipe Thread Instead of the Surge Energy

A plumber matches the arrester to the pipe size (a 1/2-inch arrester on a 1/2-inch line) without consulting the fixture-unit and length tables, and on a 40-foot branch feeding a washing machine the arrester is far too small. The trap is that pipe size is the easiest input and the least relevant to capacity. The mechanism is that surge energy scales with pipe length and flow, so a small arrester on a long branch saturates immediately. The false signal is that "an arrester rated for this pipe is installed." The harm is recurring hammer and a device that absorbs nothing. The defense is to size from PDI-WH201 or the manufacturer table using fixture units and developed length, rounding up between sizes.

### Mounting the Arrester at the Water Heater or Main, Far From Every Trigger

The plumber installs one arrester on the cold inlet of the water heater because it is accessible and central, but every fixture trigger is 30 to 80 feet away, and the hammer continues. The trap is that a central location feels thorough but is acoustically and hydraulically the worst seat in the house. The mechanism is that the transient reflects off terminations and amplifies before traveling back to the heater. The false signal is that "an arrester is installed centrally." The harm is a mislocated device and continued fitting fatigue. The defense is to place the arrester within roughly 6 feet of each quick-closing trigger, between the trigger and the source.

### Leaving a Plain Air Chamber in Place Because It "Still Looks Fine"

An existing installation has capped copper air chambers at the laundry box, and the plumber leaves them, assuming they work, but they waterlogged years ago and the hammer has been recurring. The trap is that an air chamber looks identical whether it is full of air or full of water. The mechanism is that the air dissolves into the pressurized water across the open interface. The false signal is that "there is an air chamber, so there is protection." The harm is a system that appears protected but is not. The defense is to treat any plain air chamber as a failed device, replace it with a mechanical arrester, and never assume an existing chamber still holds air.

### Using One Arrester for a Multi-Fixture Long Branch

A long branch feeds a washing machine, a dishwasher, and an ice maker, and the plumber installs a single arrester at the laundry box, expecting it to cover the other two fixtures. The trap is that the other fixtures' surges reflect and decay before reaching the single arrester. The mechanism is that each trigger generates its own transient at its own location. The false signal is that "the branch has an arrester." The harm is continued hammer from the uncovered fixtures. The defense is to map every quick-closing fixture on the branch, specify an arrester for each trigger, and add an end-of-branch unit where the run is long.

### Concealing the Arrester Behind a Finished Wall With No Access

The plumber installs a mechanical arrester inside a wall and finishes over it to keep the installation invisible, and years later the seal fails and leaks into the wall cavity, or the hammer returns and the device cannot be serviced. The trap is that a clean aesthetic hides a wearing component. The mechanism is that arrester seals and charges have a finite life. The false signal is that "the wall looks finished and complete." The harm is undetected failure and costly access demolition. The defense is to install every arrester in an accessible location with an access panel, in the manufacturer-permitted orientation, and to document the location for inspection and future replacement.

## Self-Check

- Did I size the arrester from the PDI-WH201 or ASSE 1710 / manufacturer table using the fixture-unit load of the trigger and the developed pipe length back to the source, rounding up between sizes?
- Did I place the arrester within roughly 6 feet of the quick-closing trigger, between the trigger and the source of the shock, rather than at the water heater or main?
- For a long or multi-fixture branch, did I map every quick-closing fixture and specify an arrester for each trigger, plus an end-of-branch unit where the run exceeds roughly 20 feet?
- Did I specify a mechanical arrester (diaphragm or piston with a sealed gas charge) and replace any existing plain pipe air chamber that will waterlog?
- Did I confirm the arrester is installed in an accessible location with an access panel, in the orientation permitted by the manufacturer, rather than concealed behind a finished wall?
- Did I verify the arrester size covers the specific trigger's flow (washing machine, dishwasher, ice maker, sensor faucet) and not merely the pipe diameter?
- Did I document each arrester's location, size, the trigger it serves, and the branch length, consistent with IPC 604.9 and any local amendment requirements?
- Did I confirm that the static pressure is at or below 80 psi before relying on the arrester, since overpressure amplifies the transient the arrester must absorb?
- Did I verify after installation that the hammer is eliminated under the exact triggering operation, not merely under a static pressure test?
- Did I confirm the arrester is rated for the application (residential vs commercial, and the operating temperature and pressure range) and listed to ASSE 1710 or equivalent?
