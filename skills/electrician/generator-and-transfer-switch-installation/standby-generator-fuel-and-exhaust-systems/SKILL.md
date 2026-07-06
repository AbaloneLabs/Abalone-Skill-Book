---
name: standby-generator-fuel-and-exhaust-systems.md
description: Use when the agent is planning standby generator fuel supply for diesel, natural gas, or propane systems, sizing fuel piping and day tanks, specifying exhaust sizing and clearance, or verifying code-required runtime fuel storage for NFPA 110 Class and Level compliance.
---

# Standby Generator Fuel and Exhaust Systems

A standby generator that produces no power when called is worse than no generator at all, because the building relied on it. The two failure modes that most often silence an otherwise healthy engine are fuel starvation and exhaust restriction, and both are integration problems that fall to the installing electrician or generator contractor to get right even though they sit outside the electrical trade's comfort zone. The judgment problem is that fuel and exhaust are mechanical systems governed by mechanical codes (NFPA 30, NFPA 37, NFPA 110, the International Fuel Gas Code), but their sizing and installation determine whether the electrical system they serve actually performs. An electrician who treats fuel and exhaust as "the generator comes with it" will deliver a unit that runs out of fuel, that cannot hold gas pressure, or whose exhaust chokes on its own backpressure. This skill covers diesel tank and pipe sizing, natural gas pressure requirements, exhaust sizing and clearance, and the code-required runtime that defines a compliant installation.

## Core Rules

### Size the Fuel Supply for the Code-Required Runtime at Full Load

NFPA 110 requires Level 1 systems to have fuel for the Class duration, which for a Class 96 system means 96 hours of runtime at the full load the generator serves, and many authorities and owners specify a minimum 24 or 96 hour supply. The fuel quantity must be calculated at the generator's full-load consumption rate from its data sheet, not at a optimistic partial-load rate, because the worst-case outage coincides with full building load. The trap is sizing the tank to the physical space available or to a nominal "one day" estimate, so the generator runs dry before the outage ends. The defense is to obtain the full-load fuel consumption (typically gallons per hour or liters per hour) from the data sheet, multiply by the required runtime, add a margin for the unusable fuel at the bottom of the tank, and select a tank that meets or exceeds the result.

### Design the Diesel System With Day Tank or Sub-Base Tank Logistics

Diesel generators store fuel either in a sub-base tank integral to the generator skid (typically 24 to 72 hours) or in a remote bulk tank with a day tank that receives fuel via a transfer pump from the bulk supply. Sub-base tanks are simple but limited in capacity; remote bulk and day tank systems extend runtime but add pumps, level controls, and the failure modes that come with them. The day tank must be sized to ride through pump cycles, the pump controls must be reliable, and the system must alarm on low level, pump failure, and overfill. The trap is installing a day tank system with marginal pump capacity or no alarms, so a pump failure or a leak silently drains the day tank and the generator stops. The defense is to size the day tank and pump for the consumption rate, install redundant pumps and level controls for Level 1 systems, and wire alarms to a monitored location.

### Size Natural Gas Piping for the Required Pressure and Flow at Full Load

Natural gas generators consume large volumes of gas at full load, and the pipe must deliver that volume at the pressure the generator's regulator requires, accounting for the pressure drop along the run and through meters and regulators. A 200 kW generator may demand 2000 cubic feet per hour at a required inlet pressure of 5 to 14 inches water column, and an undersized pipe or a long run will drop the pressure below the generator's minimum, causing it to shut down on low fuel pressure. The trap is sizing the gas pipe like a building heating load without considering the generator's high instantaneous demand and low pressure tolerance. The defense is to obtain the generator's full-load gas consumption and required inlet pressure, calculate the pipe size and pressure drop per the Fuel Gas Code for the actual run length and fittings, and verify the delivered pressure at the generator inlet under full load.

### Verify Natural Gas Supply Pressure and Regulation for Generator Service

Beyond the pipe sizing, the gas service itself must supply the required pressure and flow, which may require a dedicated meter and regulator sized for the generator. Utility gas services are often sized for building heat at low pressure, and adding a generator can exceed the service's capacity or drop the pressure for other loads. The trap is connecting the generator to an existing service without confirming the utility can supply the additional demand, so the generator starts, loads up, and shuts down on low pressure. The defense is to coordinate with the gas utility early, confirm the service pressure and capacity, install a dedicated regulator and meter if required, and verify the pressure holds at the generator inlet under full load.

### Size the Exhaust System for Backpressure and Clearance

Generator exhaust must be routed to the outdoors with a pipe sized to keep the engine backpressure within the manufacturer's limit, which is typically a few inches of mercury or water column. An undersized or excessively long exhaust with many fittings raises backpressure, which reduces engine power, raises exhaust temperature, and can shut the engine down on overtemperature. The exhaust outlet must also clear building openings, intakes, and combustible construction by the distances NFPA 37 and the manufacturer require, and the pipe must be supported and expansion-jointed to handle thermal growth. The trap is running a long, small-diameter exhaust with many elbows to reach a convenient wall, raising backpressure beyond the limit. The defense is to size the exhaust per the manufacturer's method for the run length and fittings, verify the calculated backpressure is within the limit, route the outlet with the required clearances, and design the supports and expansion joints for thermal growth.

### Maintain Exhaust Clearances to Combustibles and Building Openings

The exhaust pipe and muffler operate at several hundred degrees, and NFPA 37 requires specific clearance to combustible construction, and the exhaust outlet must be located away from building intakes, windows, and adjacent properties. The trap is routing the exhaust near combustible siding or under an intake louver, creating a fire risk or drawing exhaust back into the building. The defense is to verify the clearance to combustibles per NFPA 37 and the manufacturer, use listed exhaust insulation or clearance-reducing thimbles where space is tight, and locate the outlet to avoid re-entrainment into the building or nuisance to neighbors.

### Address Propane System Tank Sizing and Vaporization in Cold Climates

Propane generators depend on the tank's ability to vaporize liquid propane into gas fast enough to meet the engine's demand, and vaporization rate drops with temperature and tank fill level. A tank that vaporizes adequately in summer may starve the generator in winter, and a nearly empty tank vaporizes less than a full one. The trap is sizing the propane tank to the liquid volume only, ignoring vaporization, so the generator runs out of gas in cold weather even with liquid remaining. The defense is to size the tank for the required vaporization rate at the design minimum temperature (often from a propane supplier's chart), consider a vaporizer for large loads or cold climates, and locate the tank for the required clearances.

## Common Traps

### Sizing the Fuel Tank to Space Instead of Runtime

The generator is installed with a sub-base tank that fits the skid, and the runtime is calculated after the fact to be 12 hours, far short of the NFPA 110 Class 96 requirement. The mechanism of the failure is that the tank was chosen for physical convenience rather than the runtime calculation, and the building discovers the shortfall only during an extended outage. The false signal is that the tank is full and the generator runs, which proves function for a short test but not compliance with the runtime requirement. The harm is a generator that stops mid-outage, leaving critical loads unpowered. The defense is to calculate the runtime first and select a tank (sub-base, day tank, or bulk) that meets it.

### Undersizing Natural Gas Pipe and Dropping Pressure Under Load

The gas pipe is sized for the building's heating load, and the generator is tee'd off the existing line. The mechanism of the failure is that the generator's full-load demand creates a pressure drop that pulls the inlet pressure below the generator's minimum, and the generator shuts down on low fuel pressure when it loads up. The false signal is that the generator starts and runs unloaded, which proves the gas path is open but not that it can sustain full load. The harm is a generator that cannot carry the building load. The defense is to size the gas pipe for the generator's full-load demand and verify the inlet pressure under load.

### Connecting to an Existing Gas Service Without Utility Coordination

The generator is connected to the building's existing gas service, and the utility is not consulted. The mechanism of the failure is that the existing service and meter are sized for the building's heat, and the added generator demand exceeds the service capacity or drops the pressure for other loads. The false signal is that gas flows at the generator, which proves connection but not adequate supply. The harm is low-pressure shutdowns and possible disruption to other gas equipment. The defense is to coordinate with the gas utility, confirm service capacity, and install a dedicated regulator and meter if required.

### Routing Exhaust With Excessive Backpressure

The exhaust is run a long distance with several elbows to reach a convenient wall, using a pipe size that "looked right." The mechanism of the failure is that the long run and fittings raise the backpressure beyond the manufacturer's limit, the engine loses power, exhaust temperature rises, and the engine may shut down on overtemperature or fail to carry load. The false signal is that exhaust flows out the tailpipe, which proves the path is open but not that backpressure is within limit. The harm is derated output, overheating, and potential engine damage. The defense is to calculate the backpressure per the manufacturer's method and increase the pipe size or shorten the run to stay within limit.

### Locating the Exhaust Outlet Near an Intake or Combustible Surface

The exhaust outlet is placed under a fresh-air intake louver or near combustible siding to shorten the run. The mechanism of the failure is that exhaust is drawn back into the building through the intake, contaminating occupied space, or the hot pipe ignites the combustible surface. The false signal is that the outlet is outdoors, which proves termination but not safe location. The harm is carbon monoxide in the building or a structural fire. The defense is to locate the outlet per NFPA 37 clearances to intakes and combustibles and to verify the separation distance.

### Sizing Propane Tank to Liquid Volume and Starving in Cold Weather

The propane tank is sized to the liquid volume needed for the runtime, ignoring vaporization. The mechanism of the failure is that in cold weather the tank cannot vaporize propane fast enough to meet the engine's demand, the vapor pressure drops, and the generator shuts down on low pressure even though liquid fuel remains. The false signal is that the tank gauge shows fuel, which proves liquid volume but not vaporization capacity. The harm is a generator that stops in the exact cold-weather outage it was meant to cover. The defense is to size the tank for vaporization at the design minimum temperature and to add a vaporizer where needed.

## Self-Check

- Did I calculate the required fuel runtime from the NFPA 110 Class duration and the generator's full-load consumption rate, and select a tank (sub-base, day tank, or bulk) that meets it with margin for unusable fuel?
- For a diesel day tank system, did I size the day tank and pump for the consumption rate, install redundant pumps and level controls for Level 1 systems, and wire alarms for low level, pump failure, and overfill to a monitored location?
- For a natural gas system, did I size the pipe for the generator's full-load demand and required inlet pressure per the Fuel Gas Code, and verify the delivered pressure at the generator inlet under full load?
- Did I coordinate with the gas utility to confirm the service can supply the additional generator demand, and install a dedicated regulator and meter if required?
- Did I size the exhaust per the manufacturer's method for the run length and fittings, verify the calculated backpressure is within the limit, and design supports and expansion joints for thermal growth?
- Did I locate the exhaust outlet per NFPA 37 clearances to combustibles and building intakes, and use listed insulation or thimbles where clearance is tight?
- For a propane system, did I size the tank for the vaporization rate at the design minimum temperature, consider a vaporizer for large loads or cold climates, and locate the tank for required clearances?
- Is the fuel and exhaust design documented with calculations, code references, and verification data so another practitioner can confirm compliance?
