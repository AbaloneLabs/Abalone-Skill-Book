---
name: recirculation-temperature-control-and-timer-strategy.md
description: Use when the agent is setting a hot water recirculation return target temperature (commonly 105 to 110°F), placing and setting an aquastat differential, programming timer schedules to occupancy, configuring demand-pump activation logic, balancing loops to prevent short-loop hogging, or managing scald risk from an elevated storage setpoint where scald, energy waste, and uneven delivery are the risks.
---

# Recirculation Temperature Control and Timer Strategy

A recirculation system's temperature and schedule settings decide whether it delivers safe, even hot water efficiently or whether it scalds the occupant, wastes energy around the clock, or heats one fixture while starving another. The judgment problem is that the return temperature target, the aquastat differential, the timer schedule, and the demand logic all interact: a high storage setpoint combined with a poorly placed aquastat can deliver scalding water, a timer that ignores real occupancy runs the pump all night, and an unbalanced multi-loop system lets the short loop steal the flow. Agents miss this because "hot water comes out fast" at commissioning, masking that the storage tank is at 140°F with no tempering valve, the aquastat is on the wrong pipe, or the timer is set to the factory default. This skill covers the temperature, control, schedule, and balancing decisions that make a recirculation system safe, efficient, and even.

## Core Rules

### Set the Return Target Temperature for Delivery, Not for Storage

The return temperature is the temperature the loop is maintained at, and it should be set so the farthest fixture delivers useful hot water quickly — commonly 105 to 110°F at the return. Setting it higher (120°F plus) wastes energy through greater pipe loss and increases scald risk at the tap; setting it lower (under 100°F) risks the far fixture delivering only lukewarm water and can encourage Legionella growth in the warm loop. The disciplined rule is to set the return target to 105 to 110°F, to verify the farthest fixture reaches roughly 110 to 115°F within a few seconds of opening, and to keep the storage setpoint (often 120 to 140°F for Legionella control) separate from the delivery temperature by means of a tempering (mixing) valve between the heater and the loop. Never use an elevated storage setpoint to "fix" a cold far fixture without a tempering valve.

### Place the Aquastat Correctly and Set a Realistic Differential

The aquastat (return temperature switch) must be strapped to the return pipe at the water heater, insulated so it reads the pipe and not the room, and set with a differential that avoids short-cycling. A typical setting is cut-in at 100°F and cut-out at 110°F (a 10°F differential), which lets the loop cool 10 degrees before the pump restarts. A differential that is too narrow (2°F) short-cycles the pump and shortens its life; a differential that is too wide (30°F) lets the loop go cold between cycles. The trap is mounting the aquastat on the hot supply or in the open air, where it never reads the return temperature. The disciplined rule is to mount the aquastat on the insulated return pipe at the heater, set a 8 to 12°F differential, and verify the pump cycles on and off as the loop cools and recovers.

### Program Timer Schedules to Actual Occupancy, Not to the Factory Default

A timer restricts pump operation to hours when hot water is actually wanted, saving the energy of running all night. The schedule should reflect the building's real occupancy — for a residence, mornings (roughly 5 to 9 AM) and evenings (roughly 5 to 11 PM); for an office, business hours; for a hotel, extended morning and evening peaks. The trap is leaving the timer at a factory default (often "always on") or setting it to 24 hours "to be safe." The disciplined rule is to set the schedule to the observed occupancy pattern, to pair the timer with an aquastat so even during occupied hours the pump only runs when the loop has cooled, and to include a manual or demand override for off-schedule use.

### Configure Demand-Pump Activation Logic for True On-Demand Delivery

A demand system (button, motion sensor, or occupancy sensor at the fixture) runs the pump only when hot water is actually wanted, giving the lowest energy use and longest pump life. The activation logic must run the pump long enough to bring hot water to the fixture (commonly 10 to 30 seconds, or until a return temperature sensor confirms arrival) and then stop, so the pump does not run continuously. The trap is a demand system that runs the pump for a fixed long period or that lacks a temperature-based shutoff, wasting the energy the demand strategy was meant to save. The disciplined rule is to use a temperature-based shutoff (a sensor that stops the pump when hot water reaches the fixture) or a tuned fixed run-time, and to verify the wait at the fixture is under a few seconds after activation.

### Balance Multi-Loop Systems and Manage Scald Risk From Elevated Setpoints

In a multi-loop system (multiple returns, multiple zones), each return must have a balancing (circuit setter) valve so a short, low-head loop cannot hog the pump's flow while a long loop runs cold; balance by measured flow at each return. Separately, if the storage setpoint is elevated (140°F for Legionella control in hospitals, care homes, or large systems), the delivery to fixtures must pass through a tempering (mixing) valve set to a safe temperature (commonly 110 to 120°F, lower for pediatric or elderly care) so the elevated storage temperature never reaches the tap. The disciplined rule is to balance every return by measured flow and to install and verify a tempering valve between any elevated-setpoint heater and the fixtures, checking the mixed outlet temperature with a calibrated thermometer.

## Common Traps

### Setting the Return Target Too High and Scalding the Tap

A plumber sets the return target to 120 to 130°F to "make sure the far fixture is hot." The trap is that the loop now delivers scalding water. The mechanism is that delivery temperature tracks the loop temperature; the false signal is "the fixture is hot." The harm is scald injuries, especially to children and elderly occupants. The defense is to set the return target to 105 to 110°F and to use a tempering valve if storage is hotter.

### Using Elevated Storage Temperature Without a Tempering Valve

A plumber raises the storage setpoint to 140°F for Legionella control but installs no tempering valve. The trap is that 140°F water reaches the tap and scalds. The mechanism is that the loop delivers storage temperature when first drawn; the false signal is "the heater is set high for safety." The harm is scald injuries. The defense is to install and verify a tempering valve between any elevated-setpoint heater and the fixtures.

### Mounting the Aquastat on the Wrong Pipe or in Open Air

A plumber straps the aquastat to the hot supply or leaves it exposed to room air. The trap is that the aquastat never reads the return temperature and the pump short-cycles or never stops. The mechanism is that the sensor reads the wrong medium; the false signal is "the aquastat is installed." The harm is pump wear and poor control. The defense is to mount the aquastat on the insulated return pipe at the heater and set an 8 to 12°F differential.

### Leaving the Timer at the Factory Default or Always-On

A plumber commissions the system and leaves the timer at its factory default (often always on). The trap is that the pump runs 24 hours and the loop bleeds energy all night. The mechanism is that the default schedule ignores occupancy; the false signal is "the timer is set." The harm is high energy bills and short pump life. The defense is to program the schedule to observed occupancy and to pair it with an aquastat.

### Letting a Short Loop Hog Flow While a Long Loop Runs Cold

A plumber ties multiple returns together with no balancing valves. The trap is that the shortest loop takes most of the flow and the long loop runs cold. The mechanism is that flow distributes by hydraulic resistance; the false signal is "the near fixture is hot." The harm is cold delivery at the far fixture and callbacks. The defense is to install balancing valves on every return and to balance by measured flow.

## Self-Check

- Is the return target temperature set to 105 to 110°F, and does the farthest fixture reach roughly 110 to 115°F within a few seconds of opening?
- If the storage setpoint is elevated (120 to 140°F for Legionella control), is a tempering (mixing) valve installed and verified between the heater and all fixtures, with the mixed outlet checked at 110 to 120°F (lower for pediatric/elderly care)?
- Is the aquastat strapped to the insulated return pipe at the water heater (not the hot supply, not open air), with an 8 to 12°F differential (e.g., cut-in 100°F, cut-out 110°F)?
- Does the pump cycle on and off as the loop cools and recovers, without short-cycling on a too-narrow differential?
- Is the timer schedule programmed to the building's actual occupancy (e.g., 5 to 9 AM and 5 to 11 PM for a residence), not left at a factory default or always-on?
- Is the timer paired with an aquastat so the pump runs only during occupied hours and only when the loop has cooled?
- For a demand system, does the activation logic use a temperature-based shutoff or a tuned run-time (10 to 30 seconds) so the pump stops once hot water reaches the fixture?
- For a multi-loop system, is a balancing (circuit setter) valve installed on every return and set by measured flow so no short loop hogs the pump's capacity?
- Did I verify the delivery temperature at the farthest fixture with a calibrated thermometer and confirm it is useful (roughly 110 to 115°F) without being scalding?
- Did I document the return target, aquastat setting and location, timer schedule, demand logic, balancing valve settings, and tempering-valve setpoint so the control strategy is verifiable?
