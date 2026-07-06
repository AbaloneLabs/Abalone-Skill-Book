---
name: pool-equipment-and-deck-box-wiring.md
description: Use when the agent is wiring pool equipment pads, junction boxes, and deck boxes for pumps, heaters, and underwater lights, applying NEC 680 receptacle distance rules, providing GFCI protection for pool equipment, sealing deck boxes, and bonding metallic equipment to the pool equipotential grid.
---

# Pool Equipment and Deck Box Wiring

The pool equipment pad is where several demanding electrical decisions converge at once: motors that draw locked-rotor starting current, heaters with electronic ignition and high-limit controls, timers and automation that switch inductive loads, and underwater light cords that must transition from a wet conduit to a dry junction box above the water line. NEC Article 680 governs nearly every one of these decisions, and the article is unforgiving because the consequences of error are in and around water. The judgment problem is that the equipment pad looks like ordinary outdoor wiring — a pump on a slab, a receptacle on a post — and an installer can apply generic outdoor rules and still produce a code violation and a shock hazard. Agents miss the issues because the individual pieces appear routine, while the article-specific requirements for distances, GFCI, sealing, and bonding are what actually make the installation safe and compliant.

## Core Rules

### Apply NEC 680 Receptacle Distance and GFCI Rules Strictly

Receptacles around pools are governed by measured horizontal distances from the inside wall of the pool. A receptacle must be located at least 6 feet from the pool wall, and receptacles between 6 and 20 feet must be GFCI protected; within 20 feet the same GFCI requirement effectively applies in practice through the combination of rules. Receptacles for pool pump motors require GFCI protection regardless of distance. The defense is to measure from the pool wall, not from the deck edge or the equipment pad, to apply GFCI to every required receptacle and to all pump motors, and to remember that the measurement is the horizontal distance to the water's edge structure, which is stricter than intuition suggests.

### Size the Pump Motor Branch Circuit for Locked-Rotor and Continuous Duty

A pool pump motor is a continuous-duty inductive load that draws many times its running current for the first fraction of a second at startup. The branch circuit conductor and breaker must be sized for the running load using the motor's nameplate and the NEC ampacity rules, but the overcurrent device must also tolerate the locked-rotor inrush without nuisance tripping. The defense is to use the motor's full-load amps and the inverse-time breaker rules, to verify the breaker holds through startup, and to recognize that an oversized breaker to stop nuisance tripping is a warning sign of a motor or wiring problem, not a license to uprate the protection.

### Bond All Metallic Pool Equipment to the Equipotential Grid

Every metallic component on the equipment pad — the pump motor housing, the heater, the metal conduit, the handrails, and any metallic piping — must be bonded into the pool equipotential grid per NEC 680.26. The pump motor's bonding lug is the most commonly missed connection. The defense is to run the required solid bonding conductor from the common bonding point to every listed metallic component on the pad, to use the manufacturer's bonding lug, and to verify that the grid is continuous so that no potential difference can exist between the water, the shell, and the equipment a person touches while wet.

### Seal Deck Boxes and Route Cords to Prevent Water Migration

A deck box or forming-shell junction box for an underwater light sits at or near deck level and is the transition point between the wet niche conduit and the dry wiring above. It must be listed for pool use, mounted at the correct height above the water line, equipped with a grounded terminal for the equipment grounding conductor, and sealed so that water from the conduit cannot enter the box. The defense is to use only listed pool deck boxes, to pot the niche conduit at the niche end, to route cords with a downward loop so water does not track along the cord into the box, and to verify the box gasket and cover are intact.

### Wire Pump Timers and Automation to Handle Inductive Switching

Pool pump timers and automation relays switch a motor, which is an inductive load that arcs and produces voltage spikes when interrupted. Mechanical timers need contacts rated for motor loads, and electronic automation outputs need surge suppression to protect the relay contacts and the controller. The defense is to confirm the timer or relay is rated for the motor's horsepower and full-load amps, to install surge suppression where the controller requires it, and to expect that contact pitting on a timer is a sign of undersized contacts or a missing suppressor, not normal wear.

### Coordinate Heater Control and Safety Limit Wiring

A pool heater has its own internal controls — ignition modules, high-limit aquastats, pressure switches that prove pump flow — and these must be wired so that the heater cannot fire unless the pump is running and the limits are satisfied. The interlock between pump operation and heater enable is a life-safety function that prevents firing a dry heat exchanger. The defense is to follow the heater manufacturer's wiring diagram exactly, to wire the pump-run proof and high-limit circuits in series with the heater enable, and never to bypass a safety limit to make the heater run.

## Common Traps

### Measuring Receptacle Distance From the Deck Instead of the Pool Wall

The installer measures the 6-foot or 20-foot distance from the deck edge or the equipment pad rather than from the inside wall of the pool. The false signal is that the measurement looks generous. The mechanism of failure is that the code measures from the pool wall, so a receptacle that appears 8 feet away by the installer's tape is actually inside the restricted zone. The harm is a code violation and a receptacle too close to the water, increasing shock risk.

### Omitting GFCI on the Pump Motor Circuit

The installer wires the pump motor on a standard breaker because the motor is "hardwired, not a receptacle." The false signal is that GFCI is associated with receptacles. The mechanism of failure is that NEC 680 requires GFCI for pool pump motors regardless of whether they are cord-and-plug or hardwired. The harm is a motor circuit with no fast-acting ground-fault protection, leaving a real shock path through a wet pump housing.

### Forgetting the Pump Motor Bonding Lug

The installer bonds the niche, ladder, and heater but leaves the pump motor's bonding lug unconnected because the motor is grounded through the equipment grounding conductor. The false signal is that the motor is grounded and therefore safe. The mechanism of failure is that grounding clears faults but does not equalize potential, so a wet person touching the motor and the pool water can bridge a voltage difference. The harm is a latent shock path on the very equipment most likely to be touched during maintenance.

### Leaving the Deck Box Unsealed or Below the Required Height

The installer mounts the deck box at grade for convenience or fails to seal the niche conduit, reasoning that the box is above the water. The false signal is that the box is dry when tested. The mechanism of failure is that water migrates up the unsealed conduit or floods the low box during rain and splash, carrying current into the box and corroding terminals. The harm is water in the box, GFCI tripping, corrosion, and a hidden shock and fire hazard.

### Bypassing a Heater Safety Limit to Restore Operation

The heater will not fire and the installer jumps the pressure switch or high-limit to get it running. The false signal is that the heater now works, suggesting the limit was faulty. The mechanism of failure is that the limit was likely doing its job — proving pump flow or preventing overheating — and bypassing it lets the heater fire dry or overheat. The harm is heat exchanger damage, fire, or a dangerous pressure buildup that the safety circuit was designed to prevent.

### Undersizing Timer Contacts for the Motor Load

The installer uses a general-purpose timer whose contacts are not horsepower-rated for the pump motor. The false signal is that the timer switches the motor successfully at first. The mechanism of failure is that the inductive inrush pits the contacts, which then weld or burn open over weeks. The harm is intermittent pump operation, welded contacts that keep the pump running, or an open circuit that strands the pool without filtration.

## Self-Check

- Did I measure all receptacle distances from the inside wall of the pool per NEC 680, and is every required receptacle and pump motor GFCI protected?
- Did I size the pump motor branch circuit for full-load amps and locked-rotor inrush, and is the breaker holding through startup without being oversized?
- Did I bond every metallic component on the equipment pad, including the pump motor bonding lug, to the pool equipotential grid?
- Did I use a listed pool deck box mounted at the correct height, with the niche conduit potted at the niche and cords looped downward to prevent water tracking?
- Did I confirm the pump timer or automation relay is horsepower-rated for the motor, with surge suppression where the controller requires it?
- Did I wire the heater controls per the manufacturer's diagram, with pump-run proof and high-limit circuits in series with the heater enable, and did I avoid bypassing any safety limit?
- Did I verify that no receptacle is within the prohibited distance of the pool and that all GFCI devices trip on test?
- Is the equipment pad wiring documented so that future service can confirm bonding, GFCI, and interlock integrity?
