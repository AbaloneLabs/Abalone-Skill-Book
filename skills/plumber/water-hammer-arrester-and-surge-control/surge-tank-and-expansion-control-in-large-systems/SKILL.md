---
name: surge-tank-and-expansion-control-in-large-systems.md
description: Use when the agent is sizing a surge tank or hydropneumatic tank for a booster pump system or large building, selecting bladder versus diaphragm tanks, setting precharge pressure, controlling pump short-cycling, combining surge tanks with water hammer arresters, or protecting large-distribution mains from pump-start and pump-stop transients.
---

# Surge Tank and Expansion Control in Large Systems

In large buildings and booster-pump systems, the transients from pump starts and stops are an order of magnitude larger than any fixture hammer, and the protection strategy is a hydropneumatic surge or expansion tank sized for the system dynamics, not a residential arrester. The judgment problem is that the tank must do two jobs at once: absorb the start and stop surges to protect the mains and fittings, and store enough pressurized volume to prevent the pump from short-cycling on small demands. Agents miss the sizing because they treat the tank as a generic accessory, size it by rule of thumb, or set the precharge incorrectly, and the result is either a pump that starts and stops dozens of times an hour (destroying the motor and contactor) or a tank that is too small to absorb the stop surge and lets the check valve slam anyway. This skill covers surge and drawdown volume sizing, precharge setting, bladder-versus-diaphragm selection, placement near the pump discharge, and the combination of a tank with fixture-level arresters, to prevent pump short-cycling, main damage, and the premature tank failure that follows wrong precharge.

## Core Rules

### Size the Tank for Both Drawdown Volume and Surge Absorption, Using Pump Starts per Hour

A hydropneumatic tank in a booster system serves two loads that must both be satisfied: it must provide enough usable (drawdown) volume between pump cut-in and cut-out to keep the pump starts per hour within the motor manufacturer limit (commonly 6 to 10 starts per hour for small motors, fewer for large), and it must have enough gas volume to absorb the pressure rise when the pump stops and the forward-flowing water column decelerates. The drawdown volume is computed from the pump flow rate and the allowable cycle time (for example, a 50 gpm pump limited to 6 starts per hour needs roughly 5 minutes of run time per cycle, so roughly 250 gallons of drawdown, adjusted by the pressure band). The surge requirement is the gas volume needed to cushion the stop transient without the pressure exceeding the relief setting. The discipline is to compute both, take the larger, and select a tank (or bank of tanks) whose acceptance volume at the system pressure band meets it, rather than sizing by tank total volume or by rule of thumb.

### Set the Precharge 2 to 5 psi Below the Pump Cut-In Pressure

The precharge (the gas pressure in the tank with no water in it, measured at the Schrader valve with the system depressurized) determines how much water the tank accepts and releases across the pressure band, and a wrong precharge is the most common cause of both short-cycling and bladder failure. The correct precharge is 2 to 5 psi below the pump cut-in pressure, so the tank still holds a small water reserve at cut-in (protecting the bladder from flexing fully to the shell) and delivers its full acceptance across the band. If the precharge is too low, the tank accepts too much water and the usable drawdown shrinks, and the bladder overflexes; if the precharge is too high (at or above cut-in), the tank empties before cut-in and the pump short-cycles on every small draw. The discipline is to set and verify the precharge at commissioning, to recheck it annually (gas permeates through the bladder over time), and to document the cut-in, cut-out, and precharge values on the tank.

### Choose Bladder or Diaphragm by Size, Pressure, and Replaceability

Surge and expansion tanks come in two constructions: a bladder (a replaceable bag held in a steel shell) typical of larger tanks, and a diaphragm (a fixed membrane sealed to the shell) typical of smaller tanks. Bladder tanks are favored for larger volumes because the bladder can be replaced without replacing the shell, and they tolerate higher pressures and broader acceptance ranges; diaphragm tanks are simpler and cheaper for small residential expansion duty but the diaphragm is not separately replaceable and the whole tank is discarded on failure. The selection criteria are the required acceptance volume, the maximum system pressure (the tank must be rated above the relief valve setting, commonly 150 psi shells for commercial duty), the temperature (cold-water versus thermal expansion service), and the maintenance plan. The discipline is to match the construction to the service: replaceable bladder tanks for large or high-pressure booster service where the bladder will eventually be serviced, and diaphragm tanks only for small, low-maintenance expansion duty.

### Place the Tank Near the Pump Discharge and Upstream of the Check Valve Logic

The surge tank must see the pump's start and stop transients directly to absorb them, and that means it belongs on the discharge header close to the pump, between the pump and the system it protects, and arranged so the tank is not isolated from the pump by a closed check valve during the stop event. The typical arrangement is the tank connected to the discharge header downstream of the pump's check valve but upstream of the isolation and distribution, so the stop surge (the reversing column) is cushioned by the tank gas. Placement far from the pump, or on a long branch, reduces the tank's effectiveness because the transient reflects and amplifies in the intervening piping. The discipline is to mount the tank as close to the pump discharge as practical, with a dedicated isolation valve and drain for service, and to confirm the tank is in the flow path of the surge, not on a dead-end stub.

### Combine the Surge Tank With Fixture Arresters and a Slow-Closing Check Valve

A surge tank at the pump does not protect the fixtures from their own quick-closing transients, and fixture arresters do not protect the mains from the pump stop transient; the two protections address different events and both are required in a large system. The complete protection strategy is a hydropneumatic tank (for pump start and stop surges and for drawdown), a slow-closing or spring-loaded check valve at the pump (to prevent the slam that generates the worst stop surge), fixture-level mechanical arresters at each quick-closing device (for fixture hammer), and, on larger systems, a surge-relief valve that opens to drain if a transient exceeds the relief setting. Relying on the tank alone leaves fixture hammer unaddressed; relying on arresters alone leaves the pump stop surge to slam the check and stress the mains. The discipline is to specify the full combination, sized for each load, and to verify each element at commissioning.

## Common Traps

### Sizing the Tank by Total Volume or Rule of Thumb and Short-Cycling the Pump

A plumber installs a "20-gallon" tank on a 60 gpm booster and assumes the rating means capacity, but the acceptance volume at the pressure band is only a few gallons, and the pump starts 30 times an hour. The trap is that tank total volume is not usable volume. The mechanism is that acceptance depends on the pressure band and the precharge, and a small acceptance empties and refills rapidly. The false signal is that "a tank is installed." The harm is motor and contactor failure from short-cycling. The defense is to compute drawdown from the pump flow and the starts-per-hour limit and to select a tank whose acceptance volume at the band meets it.

### Setting the Precharge at or Above Cut-In and Emptying the Tank

The precharge is set equal to the cut-in pressure, and the tank delivers no water at cut-in, so every small draw drops the pressure instantly and starts the pump. The trap is that the precharge looks plausible but is on the wrong side of cut-in. The mechanism is that a precharge at or above cut-in means the tank is empty at cut-in and provides no buffer. The false signal is that "the tank is pressurized." The harm is constant short-cycling and bladder overflex. The defense is to set the precharge 2 to 5 psi below cut-in, verify it at commissioning, and recheck it annually.

### Placing the Tank on a Long Branch Far From the Pump Discharge

The tank is installed in a mechanical room corner 40 feet of pipe from the pump, and the stop surge reflects and amplifies in the intervening run before reaching the tank, so the check still slams. The trap is that the tank is present but hydraulically remote. The mechanism is that the transient travels at acoustic velocity and the tank only cushions what reaches it promptly. The false signal is that "the surge tank is installed." The harm is continued check slam and main stress. The defense is to mount the tank close to the pump discharge, in the surge flow path, with a dedicated isolation valve.

### Relying on the Surge Tank Alone and Leaving Fixture Hammer Unaddressed

A booster system has a properly sized surge tank, but the quick-closing fixtures throughout the building still hammer because their transients never reach the tank, and fittings fatigue at the fixtures. The trap is that one protection is mistaken for complete protection. The mechanism is that the tank addresses pump transients, not fixture transients. The false signal is that "the system has surge control." The harm is localized fitting failure at fixtures. The defense is to combine the surge tank with mechanical arresters at each quick-closing fixture and a slow-closing check at the pump.

### Ignoring the Tank's Pressure and Temperature Rating for the Service

A diaphragm tank rated for 100 psi and residential thermal duty is installed on a 125 psi commercial booster, and the shell or diaphragm fails prematurely. The trap is that the tank looks adequate but is undersized for the pressure. The mechanism is that the shell and membrane ratings must exceed the system relief setting and operating temperature. The false signal is that "a tank of the right volume is installed." The harm is premature failure and potential rupture. The defense is to confirm the tank pressure rating exceeds the relief valve setting (commonly 150 psi shells for commercial duty) and that the temperature rating matches the service.

## Self-Check

- Did I compute the required drawdown volume from the pump flow rate and the allowable starts per hour (commonly 6 to 10 for small motors), and confirm the tank's acceptance volume at the pressure band meets it?
- Did I separately compute the gas volume needed to absorb the pump stop surge without exceeding the relief setting, and select the larger of the drawdown and surge requirements?
- Did I set and verify the precharge 2 to 5 psi below the pump cut-in pressure, with the system depressurized, and document the cut-in, cut-out, and precharge values on the tank?
- Did I choose a replaceable bladder tank for large or high-pressure booster service, and confirm the tank pressure rating exceeds the relief valve setting (commonly 150 psi shells for commercial duty)?
- Did I mount the tank close to the pump discharge, in the surge flow path and downstream of the pump check, rather than on a long branch or dead-end stub?
- Did I include a slow-closing or spring-loaded check valve at the pump discharge to prevent the slam that generates the worst stop surge?
- Did I specify mechanical arresters at each quick-closing fixture in addition to the surge tank, recognizing that the tank addresses pump transients and the arresters address fixture transients?
- On larger systems, did I include a surge-relief valve set to open above the normal transient peak and discharge to a drain?
- Did I provide a dedicated isolation valve and drain for tank service so the bladder or tank can be maintained without draining the system?
- Did I schedule annual precharge verification (gas permeates the bladder over time) and document the commissioning pressure band, precharge, and starts-per-hour count?
