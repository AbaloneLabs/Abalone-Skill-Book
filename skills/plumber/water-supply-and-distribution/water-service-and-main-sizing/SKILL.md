---
name: water-service-and-main-sizing.md
description: Use when the agent is sizing a building water service or main, calculating demand using Hunter's curve or water supply fixture units, selecting pipe material for underground service, or determining meter and main capacity for residential or commercial supply.
---

# Water Service and Main Sizing

Water service and main sizing determines whether every fixture in a building receives adequate flow and pressure under peak demand, and the consequences of undersizing are not merely inconvenience — chronic low pressure damages appliance performance, causes nuisance complaints, and in fire-sprinklered buildings can compromise life-safety flow. The judgment problem is that peak demand is not the sum of all fixtures running simultaneously; it is a statistical peak estimated from fixture units, and the correct estimate depends on building type, fixture mix, and the probability of simultaneous use. A plumber who sizes by "add up all the fixture flows" produces a wildly oversized and expensive main, while one who sizes by gut feel produces a main that starves the top floor during morning shower rush. This skill covers the decisions that determine whether a water service will deliver adequate flow and pressure under realistic peak demand.

## Core Rules

### Size With Water Supply Fixture Units and Hunter's Curve, Not by Summing Fixture Flows

The modern method for estimating peak demand in plumbing systems is the water supply fixture unit (WSFU) system codified in the IPC and UPC. Each fixture is assigned a WSFU value based on its flow rate and frequency of use; the total WSFU for the building is converted to peak flow in gallons per minute using Hunter's curve (or the updated modified Hunter methods in current codes). The trap is summing the individual fixture flow rates — a building with twenty fixtures each flowing 2 gpm does not have a 40 gpm peak demand, because the fixtures never all run at once; the statistical peak is far lower, typically 15 to 25 gpm. Sizing to the summed flow produces an oversized, expensive main that delivers excessive velocity and pressure at low demand. The disciplined approach is to total the WSFU, apply the correct curve for the building type (residential, commercial, institutional), and convert to gpm for pipe sizing.

### Account for Static Pressure, Not Just Flow

Flow is only half the sizing problem; the other half is pressure. The available pressure at the topmost fixture is the static pressure at the meter minus the friction loss in the piping, the elevation loss (approximately 0.433 psi per foot of rise), and the residual pressure required at the fixture (typically 15 to 20 psi minimum for most fixtures, more for some appliances). A main sized for flow but ignoring elevation and friction will deliver adequate flow at the meter and inadequate pressure at the top floor. The disciplined method is to calculate the residual pressure at the most remote fixture: start with the static pressure at the meter, subtract elevation loss to the highest fixture, subtract friction loss through the piping at peak flow, and confirm the residual meets the fixture minimum. If the residual is inadequate, the pipe must be upsized to reduce friction loss, or a booster pump system is required.

### Select Service Pipe Material for Underground Durability

Underground water service pipe must resist soil corrosion, groundwater, and physical damage, and the material choice is governed by code, soil conditions, and local utility requirements. Common materials include copper (Type K for underground, with factory-applied sleeving where soils are aggressive), PE and PEX (for service lines, with specific depth and tracer wire requirements), PVC (CTS or IPS, for cold water service), and ductile iron (for large commercial mains). The trap is using a material not rated for underground service, or installing copper without protection in corrosive soil — acidic or cinder-fill soils will perforate bare copper within years. The disciplined rule is to verify the material is approved for underground water service by the code and local utility, to protect copper in aggressive soils, to install tracer wire on non-metallic service lines so they can be located, and to bury below the local frost line with the required cover.

### Verify Meter and Utility Capacity Before Sizing the Building Main

The building main can only deliver what the meter and utility service can supply. Before sizing the building main, confirm the meter size and capacity rating with the water utility, and confirm the static and residual pressure available at the meter under peak neighborhood demand. A building main sized for 50 gpm is useless if the meter is a 5/8-inch residential unit rated for 20 gpm. The trap is sizing the building main in isolation from the utility service, producing a main that is internally adequate but starved by an undersized meter or a low-pressure utility main. The disciplined approach is to obtain the meter capacity and available pressure from the utility in writing before sizing, and to upsize the meter if the building demand exceeds its rating.

### Provide for Future Expansion and Fire Flow Where Applicable

Water mains should be sized with reasonable margin for future fixture additions and, in sprinklered buildings, for fire flow. The trap is sizing exactly to current demand, leaving no capacity for a future bathroom, irrigation system, or fixture upgrade, which then requires a costly main replacement. The disciplined rule is to size with a modest margin (typically one pipe size above the calculated minimum for the main), and to coordinate with the fire protection designer for sprinklered buildings so the domestic main and fire main (or combined main) meet both demands. For combined domestic-fire mains, the sizing must account for the fire flow in addition to the domestic peak, and the backflow preventer and meter must be rated for the combined flow.

## Common Traps

### Summing Fixture Flows Instead of Using Fixture Units

A plumber adds up the flow rates of every fixture in the building and sizes the main to that total, producing a main sized for, say, 60 gpm when the actual peak demand is 25 gpm. The trap is that the method feels conservative — "more capacity is safer" — but it produces excessive velocity at low flow, which causes water hammer, erosion, and noise, and it wastes money on oversized pipe. The mechanism is that fixtures never run simultaneously at full flow, and the statistical peak is far below the sum. The false signal is that the main "works," which it does, expensively. The harm is unnecessary cost and operational problems from excessive velocity. The defense is to always use the WSFU method and Hunter's curve, not summed flows.

### Ignoring Elevation Loss in Multistory Buildings

A plumber sizes a riser for flow and friction but forgets to subtract the elevation loss to the top floor, and the top-floor fixtures have inadequate pressure. The trap is that elevation loss is invisible in the flow calculation and easy to overlook, but at 0.433 psi per foot it dominates the pressure budget in tall buildings — a 50-foot rise costs 22 psi, which can be most of the available static pressure. The mechanism is gravity acting on the water column. The false signal is that the lower floors work fine. The harm is chronic low pressure on upper floors, which is difficult and expensive to fix after the building is finished. The defense is to always calculate residual pressure at the most remote (highest) fixture, including elevation loss.

### Using Copper Without Protection in Corrosive Soil

A plumber installs a copper service line in cinder fill or acidic soil without sleeving or protective wrap, and the copper perforates within a few years. The trap is that copper is the traditional service material and "always works," which is true in neutral soil but not in aggressive soil. The mechanism is galvanic and chemical attack on the copper from soil contaminants. The false signal is that the copper passes the pressure test at installation. The harm is premature failure and a costly service replacement. The defense is to test or assess soil conditions, to sleeve or wrap copper in aggressive soils, or to select a non-corrosive material such as PE or PVC for service lines in known aggressive soils.

### Sizing the Building Main Without Confirming Meter Capacity

A plumber sizes a building main for 40 gpm and installs it, but the water meter is a 5/8-inch unit rated for 20 gpm, and the building never achieves the designed flow. The trap is that the main is internally adequate but the meter is the bottleneck, and the meter is the utility's responsibility, not the plumber's, so it is easy to overlook. The mechanism is that the meter restricts flow regardless of the downstream pipe size. The false signal is that the main is sized correctly per the calculation. The harm is a building that cannot achieve design flow, requiring a meter upgrade negotiated with the utility after the fact. The defense is to always confirm meter size and capacity with the utility before sizing the building main.

### Forgetting Tracer Wire on Non-Metallic Service Lines

A plumber installs a PE or PEX service line without tracer wire, and years later the line cannot be located for repair or utility marking. The trap is that tracer wire is invisible to the building occupant and adds a small cost and step that is easy to skip. The mechanism is that non-metallic pipe is invisible to electromagnetic locators without a tracer wire. The false signal is that the installation passes inspection (in jurisdictions that do not enforce tracer wire). The harm is that future excavation cannot avoid the line and future repairs cannot locate it, leading to damage and costly exploratory excavation. The defense is to always install tracer wire on non-metallic service lines and to bring it to an accessible point at the building.

## Self-Check

- Did I size the water service and main using water supply fixture units (WSFU) and Hunter's curve or the modified Hunter method, rather than summing individual fixture flow rates?
- Did I calculate the residual pressure at the most remote (highest, farthest) fixture, accounting for static pressure at the meter, elevation loss, friction loss at peak flow, and the fixture minimum pressure requirement?
- Did I confirm the meter size and capacity rating with the water utility in writing before sizing the building main, and is the meter adequate for the building peak demand?
- Did I select a service pipe material approved for underground water service by the code and local utility, appropriate to the soil conditions, with copper protected (sleeved or wrapped) in aggressive soils?
- Did I install tracer wire on any non-metallic service line, routed to an accessible point at the building?
- Is the service buried below the local frost line with the required minimum cover, and is the trench bedding appropriate for the pipe material?
- Did I size with a reasonable margin for future fixture additions, and did I coordinate with the fire protection designer for combined domestic-fire mains so both demands are met?
- For combined mains, does the backflow preventer and meter meet the combined domestic and fire flow requirements?
- Did I document the WSFU calculation, pressure budget, meter capacity confirmation, and material selection so the design basis is verifiable?
- If residual pressure at the topmost fixture is inadequate, did I specify a booster pump system rather than leaving the deficiency unaddressed?
