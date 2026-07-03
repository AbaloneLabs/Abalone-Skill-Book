---
name: evse-installation-and-nec-article-625-compliance.md
description: Use when the agent is installing electric vehicle supply equipment (EVSE), selecting Level 1/2/3 charger circuits, applying NEC Article 625 wiring and disconnect rules, sizing branch circuits for continuous EV charging loads, or placing disconnects, GFCI, and ventilation for residential or commercial EV chargers.
---

# EVSE Installation and NEC Article 625 Compliance

An EV charger is not a receptacle with a cord on it. It is a continuous, high-amperage load that may run at full current for many hours, communicates with the vehicle over a pilot signal before any power flows, and carries its own ground-fault personnel protection that interacts with any upstream protection the electrician adds. NEC Article 625 exists precisely because ordinary branch-circuit rules do not capture these behaviors, and applying generic receptacle logic to an EVSE produces the three most common failures: undersized circuits that overheat during multi-hour charging, nuisance tripping from competing ground-fault devices, and missing or misplaced disconnects that leave a live charger no one can safely isolate. The judgment problem is that an EVSE looks simple — a box on the wall, a cable, a car — but the code treats it as a dedicated, continuous, listed appliance with specific requirements for circuit sizing, disconnecting means, fault protection, and ventilation. An electrician who wires it like a dryer outlet will pass a casual look but fail inspection and create real fire and shock hazards. This skill covers the decisions that determine whether an EVSE installation is safe, code-compliant, and serviceable.

## Core Rules

### Treat Every EVSE Branch Circuit as a Continuous Load at 125%

EV charging is the textbook continuous load: a vehicle plugged in overnight draws its full rated current for three hours or more. NEC Article 625 and the general continuous-load rule require the branch-circuit conductors and overcurrent protection to be sized at 125% of the EVSE's maximum rated load. A 40-amp Level 2 EVSE is therefore not a 40-amp circuit — it requires conductors and a breaker rated for at least 50 amps (40 × 1.25 = 50). This single rule is violated more often than any other in EVSE work, because the EVSE nameplate says "40A" and the electrician runs a 40-amp circuit to match. The circuit then carries 40 amps continuously through conductors and a breaker sized for a noncontinuous 40 amps, and the thermal margin that the 125% factor provides is gone.

The trap is reading the EVSE rating as the circuit rating. The defense is to take the EVSE's maximum rated current, multiply by 1.25, and size the conductor, breaker, and receptacle (if any) to that value, while also confirming the conductor ampacity survives any temperature or conduit-fill derating applied on top.

### Distinguish the Charging Levels and Wire Each by Its Real Demand

The three charging levels impose fundamentally different electrical demands, and conflating them drives most sizing errors. Level 1 charging uses a standard 120-volt single-phase receptacle at 12 to 16 amps and adds negligible load to a service; it is essentially a household appliance circuit. Level 2 charging uses 208 or 240 volts at 16 to 80 amps and is a dedicated, high-amperage continuous load that often dominates a residential service calculation. Level 3 — DC fast charging (DCFC) — bypasses the vehicle's onboard charger entirely and supplies high-voltage DC directly from an off-board charger at 50 to 350+ kW; it is not a branch circuit at all but a utility-coordinated power installation that typically requires medium-voltage service, a dedicated transformer, and engineering beyond Article 625's branch-circuit scope.

The trap is treating a Level 2 charger like a large appliance or treating a DCFC site like a big Level 2 installation. The defense is to identify the level first, apply the continuous-load and dedicated-circuit rules to Level 2 work, and recognize that DCFC requires utility coordination, transformer sizing, and a design that exceeds ordinary branch-circuit practice.

### Place the Disconnecting Means Where It Is Accessible and In Sight

NEC 625.43 requires a disconnecting means for EVSE, and for EVSE rated more than 60 amps or more than 150 volts to ground, that disconnect must be in a readily accessible location within sight of the EVSE. The disconnect allows the charger to be isolated for servicing without de-energizing the entire panel and without exposing a technician to live terminals. For lower-rated EVSE, the circuit breaker may serve as the disconnect if it is lockable, but the within-sight requirement for high-amperage units means a breaker in a basement panel does not satisfy the rule for a charger mounted in a detached garage.

The trap is relying on the panel breaker as the disconnect for a high-amperage charger that is out of sight of the panel, leaving no compliant local isolation means. The defense is to install a listed disconnect switch within sight of any EVSE over 60 amps or 150 volts to ground, verify it is readily accessible (not behind a parked car or locked gate), and ensure it breaks all ungrounded conductors.

### Do Not Stack GFCI Protection on the EVSE's Internal Personnel Protection

Listed EVSE includes an internal personnel protection system — a charge cable interrupting device (CCID) that monitors for ground faults at roughly 20 mA, far more sensitive than a standard GFCI receptacle and tailored to the charging environment. This internal protection is the code-compliant ground-fault protection for the EVSE. Adding an upstream GFCI breaker creates two ground-fault detection systems on the same circuit, and they interact: the capacitive coupling of the long charging cable and the vehicle chassis causes leakage current that one or both devices may interpret as a fault, producing nuisance tripping that defeats the charger. Hardwired EVSE with internal CCID protection generally does not require, and should not receive, an additional GFCI breaker.

The trap is reflexively adding GFCI protection because the location sounds "damp" or because the electrician defaults to GFCI for any outdoor or garage circuit, then chasing phantom faults when the charger trips repeatedly. The defense is to confirm the EVSE is listed with internal personnel protection, to avoid stacking an upstream GFCI device on a hardwired listed EVSE, and to recognize that a GFCI receptacle is appropriate only for a portable plug-in Level 1 unit fed from a standard receptacle where no EVSE-internal protection exists upstream of the plug.

### Verify the Ventilation Requirement Marking and Interlock Where Needed

NEC 625.29 addresses ventilation for indoor EV charging, because certain battery technologies can evolve hydrogen or other gases during charging, and an enclosed space without ventilation can accumulate an explosive concentration. Modern EVSE for lithium-ion vehicles is typically marked "ventilation not required," reflecting that the sealed battery packs do not vent during normal charging. But the electrician cannot assume this — the marking must be verified on the installed unit, and where the EVSE is marked "ventilation required," the charger must be interlocked with a mechanical ventilation system so that charging cannot occur unless the ventilation is running. This interlock is a life-safety function: without it, a vehicle could charge in a sealed garage and fill the space with flammable gas.

The trap is assuming all modern EVs are sealed and skipping the ventilation check, or installing a "ventilation required" EVSE without the interlock. The defense is to read the EVSE marking, and where ventilation is required, wire the interlock so the EVSE is disabled when ventilation is off, and confirm the ventilation system is sized for the charging area.

### Use Listed Equipment and Listed Type EV Cable in the Charging Assembly

Article 625 requires the EVSE to be listed, and the charging cable must be a listed Type EV (or equivalent) cable rated for the environment, the voltage, and the flexing and weather exposure it will see. Ordinary SO or SJ cord is not acceptable for the charging lead because it is not rated for the cyclic flexing, temperature, and UV exposure of a cable handled daily and left outdoors. The coupling to the vehicle must be a listed connector matching the standard (J1772 for most AC, CCS or NACS for DCFC). For receptacle-connected portable units, the receptacle must be rated for the full circuit amperage and the environment.

The trap is substituting a general-purpose cord or connector for the listed charging cable, or using a receptacle underrated for the circuit. The defense is to install only listed EVSE with its original listed cable and connector, and to size any receptacle to the circuit rating and the environmental rating of the location.

## Common Traps

### Sizing the Circuit to the Nameplate Instead of 125% of It

An electrician reads "40A" on the EVSE and installs a 40-amp breaker on 8 AWG copper, sized exactly to the nameplate. The mechanism of the trap is that EV charging is a continuous load, and the continuous-load rule requires 125% sizing, so a 40-amp EVSE needs a 50-amp circuit. The false signal is that the nameplate rating and the breaker rating match, which looks correct but ignores the sustained-duration thermal margin the code requires. The harm is conductors operating at their ceiling for hours every night, insulation degradation, and breaker heating that shortens the device life and risks fire. The defense is to multiply the EVSE rated current by 1.25 and size the entire circuit to that product.

### Adding a GFCI Breaker to a Listed Hardwired EVSE and Creating Nuisance Trips

An outdoor Level 2 charger is hardwired, and the electrician installs a GFCI breaker upstream "for safety," then receives call-backs because the charger trips randomly, sometimes mid-charge. The mechanism of the trap is that the listed EVSE already has internal CCID ground-fault protection at 20 mA, and the added GFCI breaker's leakage threshold overlaps and conflicts with it, tripping on the normal capacitive leakage of the cable and vehicle chassis. The false signal is that GFCI is "safer," which is true for a receptacle but wrong when layered on a device that already provides equivalent or better protection. The harm is a charger that does not work and an owner who loses confidence in EV charging. The defense is to rely on the EVSE's listed internal protection for hardwired units and to reserve GFCI devices for portable plug-in units fed from standard receptacles.

### Using the Panel Breaker as the Disconnect for an Out-of-Sight Charger

A 70-amp Level 2 charger is mounted in a detached garage, fed from a breaker in the house panel 60 feet away and around a corner. The mechanism of the trap is that NEC 625.43 requires the disconnect for a charger over 60 amps to be within sight and readily accessible, and a remote panel breaker satisfies neither. The false signal is that the breaker isolates the circuit, which is electrically true but does not meet the code's requirement for a local, visible means of disconnection. The harm is a technician servicing the charger who cannot see the disconnect and cannot confirm the circuit is isolated, creating a shock hazard. The defense is to install a listed disconnect switch within sight of any EVSE over 60 amps or 150 volts to ground.

### Assuming Ventilation Is Never Required and Skipping the Interlock

An indoor EVSE is installed in a commercial parking garage, and the electrician assumes modern batteries are sealed and skips the ventilation check. The mechanism of the trap is that the EVSE marking determines the ventilation requirement, and a unit marked "ventilation required" installed without the interlock can charge a vehicle that vents gas into an enclosed, unventilated space. The false signal is that "EVs don't off-gas," which is generally true for lithium-ion but is a property of the battery, not a license to ignore the EVSE marking. The harm is explosive gas accumulation in an enclosed space. The defense is to read the marking on every indoor EVSE and wire the ventilation interlock where required.

### Treating a DC Fast Charger Like a Large Level 2 Unit

A site host wants a DC fast charger, and the electrician approaches it as a high-amperage branch circuit, planning to pull large conductors from the existing service. The mechanism of the trap is that DCFC at 50 kW and above typically requires a dedicated utility transformer, medium-voltage service, and a power draw that the existing service cannot support — it is not a branch-circuit installation. The false signal is that a charger is a charger, which obscures the order-of-magnitude difference in power and infrastructure between AC Level 2 and DC fast charging. The harm is a designed system that cannot be built on the existing service, wasted conduit and conductor, and a delayed project. The defense is to recognize DCFC as a utility-coordinated power installation and to engage the utility before designing the feed.

### Overlooking the Edge Case or Exception

The typical Level 2 residential installation is handled well, but the unusual case — a 48-amp hardwired unit on a 60-amp circuit in a wet location with a long run requiring voltage-drop upsizing — is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I size the branch-circuit conductors and overcurrent protection at 125% of the EVSE's maximum rated current, and does the conductor ampacity still meet that value after temperature and conduit-fill derating?
- Did I identify the charging level (Level 1, Level 2, or DCFC) and apply the correct rules — household receptacle circuit for Level 1, dedicated continuous-load circuit for Level 2, and utility-coordinated power design for DCFC?
- For any EVSE rated over 60 amps or over 150 volts to ground, did I install a listed disconnecting means that is readily accessible and within sight of the EVSE, rather than relying on a remote panel breaker?
- Did I confirm the EVSE is listed with internal personnel protection (CCID), and did I avoid stacking an upstream GFCI breaker on a hardwired listed unit that would cause nuisance tripping?
- Did I read the EVSE ventilation marking, and for any unit marked "ventilation required," did I wire the interlock so charging is disabled when ventilation is not running?
- Did I use listed EVSE with its original listed Type EV cable and connector, and is any receptacle rated for the full circuit amperage and the environmental conditions?
- For DC fast charging, did I recognize the utility-coordination and transformer requirements rather than treating it as a large branch circuit?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
