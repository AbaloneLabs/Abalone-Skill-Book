---
name: boost-pump-and-pressure-tank-system-design.md
description: Use when the agent is designing or sizing a booster pump and pressure tank system for low municipal or well pressure, selecting variable-speed versus constant-speed pumping, sizing GPM and total dynamic head, setting pressure tank precharge, configuring jockey or pilot arrangements, or preventing short-cycling, motor burnout, water hammer, and suction cavitation.
---

# Boost Pump and Pressure Tank System Design

A booster pump system exists because the available supply pressure is too low to serve the building, and designing it wrong produces every failure mode at once: burnout from short-cycling, water hammer from sudden stops, inadequate flow at the top floor, and cavitation that erodes the impeller in weeks. The judgment problem is that the pump is the visible component, so agents size the pump and treat the pressure tank, the precharge, and the suction conditions as afterthoughts — but the tank and precharge are what prevent the pump from short-cycling to death, and the suction conditions (NPSH, available pressure) determine whether the pump cavitates. Agents also default to a single constant-speed pump where a variable-speed or a jockey arrangement would serve low demand without cycling, and they undersize the tank to save space, guaranteeing short runs and motor failure. This skill covers when boosting is needed, pump and tank sizing, precharge, drive selection, jockey arrangements, short-cycling prevention, and NPSH and cavitation.

## Core Rules

### Confirm Boosting Is Actually Needed and Quantify the Deficit

Boosting is justified only when the available supply cannot meet the building's required pressure and flow at the most remote fixture. Quantify the deficit: measure the static and dynamic supply pressure at the building entry, calculate the required pressure at the top fixture (fixture pressure, typically 20 to 30 psi residual, plus elevation loss at 0.433 psi per foot, plus friction loss through piping at design flow), and compare. If supply minus losses is below the required residual at peak flow, boosting is needed; if supply is adequate but distribution piping is undersized, the fix is repiping, not a pump. The trap is installing a booster to mask a distribution restriction or a supply-side problem that should be fixed at the source. The disciplined rule is to measure the supply, calculate the demand and losses, and boost only the real deficit.

### Size the Pump by GPM and Total Dynamic Head

The pump must deliver the building's peak flow (design GPM, from fixture unit conversion or the probable simultaneous demand) against the total dynamic head (TDH): the elevation lift, the friction loss through piping and fittings at design flow, the residual pressure required at the top fixture, minus any residual supply pressure available. Select a pump whose curve passes through the design duty point (GPM at TDH) and stays within the pump's preferred operating range — a pump run far right of best efficiency point (BEP) cavitates and overloads the motor, one run far left recirculates and overheats. For multi-pump systems, stage the pumps so each runs near BEP across the demand range. The trap is selecting by horsepower or pipe size. The disciplined rule is to compute the duty point and select from the pump curve.

### Size the Pressure Tank and Set Precharge to Prevent Short-Cycling

The pressure tank stores a drawdown volume that lets the pump rest between runs; without enough drawdown, the pump short-cycles (starts and stops every few seconds), burning the motor, the capacitor, and the contacts. Tank size is set by the pump GPM, the pressure switch differential (commonly 30/50 or 40/60 psi, a 20 psi cut-in to cut-out spread), and a minimum run time (typically 1 minute for motors up to 1.5 HP, longer for larger) to let the motor cool between starts. The tank precharge (air charge with no water in the system) must be set 2 to 4 psi below the cut-in pressure so water enters the tank before the pump starts. The trap is undersizing the tank or leaving the factory precharge. The disciplined rule is to size the tank for the minimum run time and set the precharge 2 to 4 psi below cut-in.

### Choose Variable-Speed, Constant-Speed, or Jockey Arrangement by Demand Profile

A constant-speed pump with a pressure switch and tank is simple and durable for steady demand, but short-cycles on low demand unless the tank is large. A variable-speed (variable-frequency drive, VFD) pump adjusts speed to match demand, holding near-constant pressure without large drawdown, which suits buildings with wide demand swings — but VFDs add cost, complexity, and harmonic/heat considerations, and still need a small tank or hydropneumatic vessel to absorb transients. A jockey (pilot) pump arrangement uses a small pump for low demand and a larger pump for peak, extending motor life. The trap is defaulting to one constant-speed pump for a variable demand. The disciplined rule is to match the drive and staging to the demand profile, and to provide tank or vessel capacity for transients on any system.

### Protect Against NPSH Shortfall and Suction Cavitation

A booster pump on a suction source (a tank, a low-pressure main) must have adequate net positive suction head (NPSH): the available NPSH (from atmospheric pressure plus suction static head minus vapor pressure and suction friction) must exceed the pump's required NPSH (from the curve) by a margin, typically 2 to 3 feet, or the pump cavitates. Cavitation erodes the impeller, drops output, and destroys the pump in weeks. On a suction lift (pump above the water level), the available NPSH is small and cavitation risk is high; on a flooded suction (pump below the water level), risk is low. The trap is installing a booster on a marginal suction source without checking NPSH. The disciplined rule is to compute available NPSH, compare to the required NPSH at the duty point, and prefer flooded suction; if NPSH is marginal, lower the pump, enlarge suction piping, or select a lower-NPSH pump.

## Common Traps

### Installing a Booster to Mask a Distribution or Supply Restriction

The plumber installs a booster pump because pressure is low at the top floor, without measuring the supply or checking the distribution. The trap is that the real cause is an undersized riser or a weak supply. The mechanism is that the restriction is downstream or upstream of where a pump helps. The false signal is that "low pressure means add a pump." The harm is a pump that does not fix the problem and adds energy and failure modes. The defense is to measure supply and calculate demand and losses, and to fix restrictions before boosting.

### Selecting the Pump by Horsepower or Pipe Size, Not the Duty Point

The plumber picks a pump by HP or by matching the pipe size, without plotting the duty point on the curve. The trap is that the pump may run far off BEP. The mechanism is that output depends on the curve intersection with system head, not on HP rating. The false signal is that "a bigger HP pump moves more water." The harm is cavitation, motor overload, or recirculation overheating. The defense is to compute GPM and TDH and select a pump whose curve passes through the duty point within its preferred range.

### Undersizing the Pressure Tank or Leaving the Factory Precharge

The plumber installs a small tank or does not adjust the precharge to cut-in. The trap is that drawdown is too small or zero. The mechanism is that a tank with precharge at or above cut-in admits no water, and a too-small tank empties in seconds. The false signal is that "any tank stops cycling." The harm is short-cycling that burns the motor, capacitor, and contacts. The defense is to size the tank for the minimum run time and set the precharge 2 to 4 psi below cut-in with the system depressurized.

### Using One Constant-Speed Pump for a Wide Demand Profile

The plumber installs a single constant-speed pump for a building whose demand swings from a trickle to peak. The trap is that low demand forces constant short-cycling. The mechanism is that a fixed-speed pump cannot throttle to low flow without cycling or bypassing. The false signal is that "one pump is simpler." The harm is motor and contact burnout and poor pressure control. The defense is to match the drive and staging to the demand profile — VFD for wide swings, jockey plus main for staged demand, or a properly sized tank.

### Ignoring NPSH on a Suction Booster and Cavitation-Eroding the Impeller

The plumber installs a booster on a tank or low-pressure main without checking available NPSH against the pump's required NPSH. The trap is that the pump cavitates silently at first. The mechanism is that low suction head drops the liquid pressure below vapor pressure, forming bubbles that collapse on the impeller. The false signal is that "the pump is running." The harm is impeller erosion and output loss within weeks. The defense is to compute available NPSH, compare to required NPSH with margin, prefer flooded suction, and enlarge suction piping or reselect the pump if marginal.

## Self-Check

- Did I confirm boosting is needed by measuring static and dynamic supply pressure and calculating required residual pressure at the top fixture, rather than assuming low pressure means add a pump?
- Did I size the pump from the duty point (design GPM at computed TDH) plotted on the manufacturer curve, within the preferred operating range, rather than by horsepower or pipe size?
- Did I size the pressure tank to deliver the minimum pump run time (1 minute for motors up to 1.5 HP) at the chosen pressure-switch differential?
- Did I set the tank precharge to 2 to 4 psi below the cut-in pressure, with the system depressurized, and verify with a tire gauge?
- Did I choose constant-speed, variable-speed, or jockey-plus-main based on the building's demand profile, and provide tank or vessel capacity for transients on any system?
- Did I compute available NPSH and confirm it exceeds the pump's required NPSH at the duty point by 2 to 3 feet, or prefer a flooded suction?
- Is the suction piping at least as large as the pump inlet, short and direct, to minimize suction friction and cavitation risk?
- Did I include a check valve, isolation valves, and a pressure gauge on both suction and discharge for service and diagnosis?
- Did I provide for water hammer on quick-closing valves (an arrester or sufficient tank capacity), and confirm the system holds pressure with the pump off?
- Did I document the duty point, tank size and precharge, drive type, and NPSH margin in the design and service record?
