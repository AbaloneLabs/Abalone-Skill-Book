---
name: medium-voltage-transformer-installation-and-protection.md
description: Use when the agent is installing or commissioning medium-voltage transformers (pad-mount, dry-type, or substation), sizing primary and secondary protection, selecting impedance, coordinating primary fuses, choosing neutral grounding method, specifying temperature monitoring, or deciding between oil-filled and dry-type construction.
---

# Medium-Voltage Transformer Installation and Protection

A medium-voltage transformer is the point where the utility or distribution voltage is converted to the facility utilization voltage, and its installation, protection, and grounding determine whether the downstream system is safe, reliable, and able to survive faults. The judgment problem is that transformer selection is often reduced to kVA and voltage ratio, when the decisions that actually govern performance and survival — impedance, primary fuse coordination, neutral grounding, temperature class, and oil versus dry construction — are made implicitly or not at all. An agent that sizes the transformer to the connected load, picks a standard fuse, and grounds the neutral solidly because that is the default will produce an installation that may nuisance-trip, fail to clear faults, produce destructive arc-flash on the secondary, or overheat in its actual duty cycle, and the defects are hidden until a fault or overload exposes them.

## Core Rules

### Size the Transformer for the Actual Load Cycle and Future Growth, Not the Connected Load

Transformer kVA sizing must account for the actual demand (not the sum of nameplate loads), the load diversity, the duty cycle, harmonic content from nonlinear loads, and planned future growth. The defense is to perform a load calculation based on the actual demand factor and diversity, derate the transformer for harmonic-rich loads (K-factor or measured harmonic derating), add a growth margin appropriate to the facility plan, and verify the resulting kVA keeps the transformer within its temperature rating under the actual load profile. Sizing to connected load wastes capital; undersizing causes overheating and shortened insulation life.

### Select Impedance Deliberately Because It Governs Fault Current and Voltage Regulation

Transformer impedance (typically 4 to 8 percent for MV distribution transformers) is the percentage of rated voltage that, applied to the primary with the secondary shorted, produces rated current. Lower impedance means higher available fault current on the secondary (requiring higher-rated downstream equipment) but better voltage regulation under load; higher impedance means lower fault current (easier on downstream equipment) but worse regulation and more voltage sag on motor starting. The defense is to calculate the secondary available fault current from the impedance and the source, verify the downstream equipment bracing and interrupting ratings exceed it, and select an impedance that balances fault current against regulation for the specific load mix.

### Coordinate the Primary Fuse With the Transformer Inrush and the Secondary Protection

The primary fuse (or breaker) must tolerate the transformer inrush (typically 8 to 12 times full-load current for 0.1 seconds at energization) without operating, protect the transformer against internal faults, and coordinate with the secondary main device so secondary faults are cleared by the secondary device, not the primary fuse. The defense is to plot the transformer inrush point, the transformer damage curve (per ANSI/IEEE C57.12), the primary fuse curve, and the secondary main curve on a single time-current chart, select a fuse that sits to the right of inrush and to the left of the damage curve, and verify the primary and secondary curves do not overlap. A fuse too small blows on inrush; a fuse too large fails to protect the transformer or miscoordinates with the secondary.

### Choose the Neutral Grounding Method for the System Requirements

The secondary neutral can be solidly grounded (common for 480/277V and 208/120V systems), resistance-grounded (low-resistance for industrial, high-resistance for continuity-critical), or ungrounded. Solid grounding provides a stable reference and rapid fault clearing but produces high single-line-to-ground fault current. Low-resistance grounding limits ground fault current to a controlled value (200 to 1000 A) for selective coordination. High-resistance grounding limits current to a few amps, allowing continued operation with a single ground fault while alarming, ideal for continuous-process plants. The defense is to select the method based on the system's need for fault clearing speed, continuity, and equipment protection, and to size the grounding resistor and monitor accordingly.

### Specify Temperature Monitoring and Cooling Appropriate to the Criticality

Transformers fail by insulation aging from sustained overtemperature, and the winding hottest-spot temperature, not the top-oil temperature, is the controlling variable. The defense is to specify winding temperature simulation or direct fiber-optic hotspot measurement on critical transformers, set the cooling stage activation and alarm trips per the temperature class, and verify the cooling fans and forced-air systems operate. For oil-filled units, specify oil temperature and level alarms and a sudden-pressure relay; for dry-type, specify the temperature sensors and the IP rating for the environment.

### Choose Oil-Filled or Dry-Type for the Environment and Installation

Oil-filled transformers (mineral oil or less-flammable fluids like FR3) are more efficient, more compact, and better for outdoor and high-kVA applications, but require containment for oil leaks, fire barriers, and are restricted indoors. Dry-type transformers (cast resin or VPI) are safer indoors, require no containment, but are larger, less efficient, and limited in kVA and voltage. The defense is to select oil-filled for outdoor substation and high-kVA duty with proper containment, dry-type for indoor and occupied-space installations, and less-flammable fluid where indoor oil-type is permitted with safeguards. Never install a mineral-oil transformer indoors without the required fire-rated vault.

## Common Traps

### Sizing to Connected Load and Overheating Under Actual Demand

The transformer is sized to the sum of all connected loads, which is far above the actual demand, so it appears conservatively large — but in a different failure mode, the load is sized without diversity and the transformer is undersized for the actual simultaneous load. The mechanism of the failure is that the winding hottest-spot temperature exceeds the insulation class rating continuously, the insulation ages at an accelerated rate (insulation life roughly halves for every 6 to 10 C above rating), and the transformer fails prematurely. The false signal is that the transformer "is carrying the load" without tripping, implying adequacy, when the insulation is being consumed. The harm is a transformer that fails years before its design life, often at peak load.

### Selecting Low Impedance and Exceeding Downstream Equipment Ratings

A low-impedance transformer (for better regulation) is specified without recalculating the secondary fault current, and the downstream switchboard bracing and breaker interrupting ratings are below the new available fault current. The mechanism of the failure is that a downstream fault produces current exceeding the breaker's interrupting rating, the breaker fails to clear or ruptures, and the fault burns until the line melts or the upstream device operates. The false signal is that the breaker "is rated for the system voltage," implying it can interrupt, when the interrupting rating is exceeded. The harm is catastrophic breaker failure and arc flash on a fault that should have been cleared.

### Picking a Primary Fuse Too Small and Blowing on Inrush

A fuse is selected close to the full-load current without checking the inrush point, and the transformer energizes, draws 10 times full load for a tenth of a second, and the fuse blows. The mechanism of the failure is that the fuse's minimum-melting curve is to the left of the inrush point, so the fuse element melts on every energization. The false signal is that the fuse "is protecting the transformer," when it cannot survive normal energization. The harm is nuisance fuse operations on every start, often misdiagnosed as a transformer fault.

### Solidly Grounding Where Resistance Grounding Is Required for Continuity

A continuous-process plant is solidly grounded by default, and the first ground fault trips the feeder and shuts down the process. The mechanism of the failure is that solid grounding produces high ground-fault current that the protection must clear immediately, forcing an outage for every ground fault. The false signal is that solid grounding "is standard and safe," when high-resistance grounding would have allowed continued operation with an alarm. The harm is unnecessary process outages and lost production from faults that could have been managed.

### Omitting Winding Hotspot Monitoring on a Critical Transformer

A critical transformer has only top-oil temperature indication, and the winding hotspot is not monitored. The mechanism of the failure is that the hotspot can run 10 to 20 C above the top-oil temperature under load, and without hotspot indication the overload protection cannot respond to the actual insulation temperature, so the transformer overheats silently and ages. The false signal is that the oil temperature "is normal," implying the transformer is fine, when the windings are over temperature. The harm is insulation failure from sustained hotspot overtemperature that the oil gauge never reflected.

### Installing a Mineral-Oil Transformer Indoors Without a Vault

An oil-filled transformer is placed in a mechanical room without a fire-rated vault or oil containment. The mechanism of the failure is that a mineral-oil transformer is a significant fire load, and code requires a fire-rated vault with oil containment and proper ventilation for indoor installation. The false signal is that the transformer "is in an electrical room," implying compliance, when the vault and containment requirements were never met. The harm is a code violation, an uninsurable fire risk, and an oil spill that contaminates the building if the tank ruptures.

## Self-Check

- Did I size the transformer kVA from the actual demand and diversity (not connected load), derate for harmonics, add appropriate growth margin, and verify temperature rating under the actual load profile?
- Did I select the impedance deliberately, calculate the secondary available fault current, and verify all downstream equipment bracing and interrupting ratings exceed it?
- Did I plot the inrush point, transformer damage curve, primary fuse, and secondary main on a time-current chart, and confirm the fuse tolerates inrush, protects the transformer, and coordinates with the secondary?
- Did I select the neutral grounding method (solid, low-resistance, high-resistance) based on the system's need for clearing speed, continuity, and equipment protection, and size the resistor and monitor accordingly?
- Did I specify winding hotspot temperature monitoring (simulation or direct fiber) on critical transformers, and set cooling, alarm, and trip thresholds per the insulation class?
- Did I choose oil-filled or dry-type construction appropriate to the environment, with oil containment and fire-rated vaulting where mineral-oil units are indoors, and the correct fluid type for the application?
- Did I specify oil temperature and level alarms and a sudden-pressure relay for oil-filled units, and verify the cooling fans and forced-air systems operate during commissioning?
- Is the transformer installation documented with the sizing basis, impedance, fuse coordination study, grounding method, and temperature monitoring, so the design decisions are traceable and reviewable?
