---
name: sewage-ejector-pump-selection-and-basin-sizing.md
description: Use when the agent is selecting a sewage ejector pump by horsepower and solids-handling capacity, sizing the basin volume and pump run time for motor cooling, choosing simplex versus duplex for redundancy, sequencing float switches, or venting the basin separately to prevent sewage overflow, motor burnout, and backup.
---

# Sewage Ejector Pump Selection and Basin Sizing

A sewage ejector pump lifts raw sewage from a below-grade basin to the gravity sewer, and selecting the pump or basin wrong produces sewage overflow into the building, motor burnout from short-cycling, or a single-point failure that floods when the one pump dies. The judgment problem is that the pump is chosen by horsepower alone (ignoring solids handling, total dynamic head, and the duty point on the curve), the basin is sized to fit the hole rather than to give the motor a proper run time, and the simplex-versus-duplex decision is driven by cost rather than by the consequence of failure. Agents also mis-sequence the floats so the pump short-cycles or the alarm comes too late, and they forget that the basin needs a separate vent because it is a sealed, air-bound vessel. This skill covers pump HP and solids handling, basin volume and run time, simplex versus duplex, float sequencing, and basin venting.

## Core Rules

### Select Pump Horsepower and Solids Handling for the Fixture Load and TDH

A sewage ejector must pass the solids the fixtures generate and lift the design flow against the total dynamic head (TDH). Solids handling is rated by the maximum sphere the pump can pass — a minimum of 2 inches is standard for residential and light-commercial sewage, and larger (3 inches or more) for higher-load applications; a pump that cannot pass the solids will jam and fail. Horsepower and the pump curve determine the flow at the TDH (the static lift from the basin low level to the discharge elevation, plus the friction through the discharge pipe and fittings at design flow). Select a pump whose curve delivers the design GPM (from fixture unit conversion) at the computed TDH, within its preferred operating range. The trap is selecting by HP alone. The disciplined rule is to confirm 2-inch minimum solids handling and select the pump from the duty point on the curve.

### Size the Basin Volume to Give the Motor a Minimum Run Time

The basin volume between the pump-off and pump-on levels (the active volume) determines how long the pump runs per cycle, and the run time must be long enough for the motor to cool between starts — typically a minimum of 1 minute for motors up to 1.5 HP and longer for larger motors, or per the manufacturer's maximum starts per hour. Too small an active volume short-cycles the pump (rapid start-stop), burning the motor, capacitor, and floats; too large a volume is wasteful of space but harmless. The basin also needs reserve volume above the pump-on level and below the inlet (the storage volume during a power or pump failure before sewage reaches the inlet and overflows). The trap is sizing the basin to the available hole. The disciplined rule is to size the active volume for the minimum run time and provide reserve storage for the expected outage.

### Choose Simplex or Duplex by the Consequence of Failure

A simplex system (one pump) is acceptable where a failure can be tolerated until repair (a rarely-used basement bath, a monitored system with stored alarm response). A duplex system (two pumps in one basin, alternating) is required where the consequence of failure is sewage overflow into occupied space, where the inflow can exceed one pump's capacity during peak events, or where redundancy is mandated by code for the fixture classification. Duplex systems alternate the lead pump to equalize wear, and the lag pump starts if the lead cannot keep up (high-level alarm). The trap is choosing simplex to save cost on a critical installation. The disciplined rule is to use duplex where overflow is unacceptable or inflow may exceed one pump, and to wire the alternation and high-level alarm.

### Sequence the Floats for Proper Pump Operation and Alarm

Float switch sequencing sets the pump behavior. In a simplex basin, three floats typically control: pump-off (lowest), pump-on (above off), and high-level alarm (above on). In a duplex basin, four floats: pump-off (lowest), lead pump-on, lag pump-on, and high-level alarm, with the lead-lag alternating. The vertical spacing between pump-off and pump-on sets the active volume (and thus run time); the spacing between pump-on and the alarm sets the reserve before overflow. Floats must be spaced so they do not tangle, mounted clear of the pump and inlet, and set so the alarm activates before sewage reaches the inlet (the flood rim). The trap is bunching the floats or setting the alarm too high. The disciplined rule is to space floats for the designed active and reserve volumes, mount them clear, and set the alarm below the inlet.

### Vent the Basin Separately (It Is a Sealed, Air-Bound Vessel)

A sewage ejector basin is a sealed vessel with a gasketed cover, and it must be vented independently to the atmosphere (or to the building vent system per code) so air can enter as the pump removes liquid and exit as liquid fills it. Without a vent, the basin is air-bound: the pump cannot draw, or the cover pressurizes and forces sewer gas past the gasket and into the space. The basin vent is separate from the fixture vent (the fixtures draining in have their own venting), and it must be sized per code (commonly a minimum 2-inch vent) and routed to a proper termination. The trap is omitting the basin vent or connecting it improperly. The disciplined rule is to vent the basin separately per code, with a properly sized and terminated vent, and to confirm the cover gasket is intact.

## Common Traps

### Selecting the Pump by Horsepower Alone

The plumber picks a 1/2 HP or 1 HP pump by feel, without checking solids handling or plotting the duty point. The trap is that the pump may not pass the solids or may run off its curve. The mechanism is that output depends on the curve intersection with system head and on impeller clearance, not on HP. The false signal is that "more HP means more lift." The harm is jamming, inadequate flow, or motor overload. The defense is to confirm 2-inch minimum solids handling and select the pump from the duty point (GPM at TDH) on the curve.

### Sizing the Basin to the Hole Instead of the Run Time

The plumber drops in a small basin because the excavation is tight, giving the pump a 15-second run time. The trap is that the pump short-cycles to death. The mechanism is that a small active volume empties before the motor cools. The false signal is that "it fits." The harm is motor, capacitor, and float burnout and frequent replacement. The defense is to size the active volume for the minimum run time (1 minute for motors up to 1.5 HP) and provide reserve storage for outages.

### Choosing Simplex for a Critical Installation to Save Cost

The plumber installs a single pump in a finished basement where overflow would flood living space. The trap is that the single pump is the only thing between sewage and the floor. The mechanism is that any single pump fails eventually, and inflow may exceed it during peak. The false signal is that "one pump is cheaper and fine." The harm is sewage overflow into occupied space on the first failure. The defense is to use duplex where overflow is unacceptable or inflow may exceed one pump, with alternation and a high-level alarm.

### Bunching the Floats or Setting the Alarm Too High

The plumber sets the floats close together or puts the alarm near the inlet. The trap is that the active volume is too small or the alarm comes after overflow. The mechanism is that float spacing sets the active and reserve volumes and the alarm threshold. The false signal is that "the floats are in." The harm is short-cycling or an alarm that fires too late to prevent a flood. The defense is to space floats for the designed active and reserve volumes, mount them clear of tangling, and set the alarm below the inlet.

### Omitting the Basin Vent and Making the Vessel Air-Bound

The plumber seals the basin with no vent, or connects the vent improperly. The trap is that the pump cannot draw or the cover pressurizes. The mechanism is that a sealed vessel with changing liquid level needs air exchange. The false signal is that "the cover is sealed, so it's done." The harm is pump failure to move sewage and sewer-gas leakage past the gasket into the space. The defense is to vent the basin separately per code, with a properly sized (commonly 2-inch minimum) and terminated vent, and to verify the cover gasket.

## Self-Check

- Did I select a pump rated to pass a minimum 2-inch sphere (or larger for the application), and plot the duty point (design GPM at computed TDH) on the manufacturer curve within the preferred operating range?
- Did I size the basin active volume (between pump-off and pump-on) to give the motor a minimum run time of 1 minute (or per manufacturer starts-per-hour), rather than fitting the available hole?
- Did I provide reserve storage volume between pump-on and the inlet to hold sewage during a power or pump failure before overflow?
- Did I choose simplex or duplex based on the consequence of failure (overflow into occupied space, peak inflow exceeding one pump, code requirement), not on cost alone?
- For a duplex system, did I wire lead-lag alternation and a high-level alarm, and confirm both pumps operate?
- Did I sequence and space the floats for the designed active and reserve volumes, mount them clear of tangling and the inlet, and set the high-level alarm below the basin inlet?
- Did I vent the basin separately per code with a properly sized (commonly 2-inch minimum) and terminated vent, so the vessel is not air-bound?
- Is the basin cover gasketed and sealed, with the electrical penetrations sealed, to prevent sewer-gas leakage into the space?
- Did I confirm the discharge piping is sized to the pump outlet and includes the required check valve, gate valve, and union for service?
- Did I document the pump model and duty point, basin dimensions and active/reserve volumes, float settings, simplex/duplex rationale, and vent arrangement in the installation record?
