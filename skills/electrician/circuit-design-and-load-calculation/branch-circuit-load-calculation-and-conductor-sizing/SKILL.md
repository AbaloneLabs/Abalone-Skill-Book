---
name: branch-circuit-load-calculation-and-conductor-sizing.md
description: Use when the agent is calculating branch-circuit loads, sizing conductors for continuous and noncontinuous loads, applying demand factors, selecting wire gauge for ampacity and voltage drop, or determining overcurrent protection ratings for new or modified circuits.
---

# Branch-Circuit Load Calculation and Conductor Sizing

Every conductor, breaker, and receptacle in an electrical system is sized to carry a specific amount of current safely. Undersizing a conductor causes it to overheat, which degrades the insulation, melts the jacket, and ignites the surrounding material — wire insulation, wall studs, or the combustible dust in a raceway. Oversizing wastes money and makes installation difficult. The judgment problem is that load calculation is not simple addition: continuous loads are counted at 125%, demand factors reduce certain loads based on usage patterns, motor loads require additional capacity for starting current, and conductor ampacity must be derated for ambient temperature and the number of current-carrying conductors in a raceway. An electrician who sizes conductors by "rules of thumb" or by copying a similar circuit will eventually install a conductor that cannot carry the actual load, and the failure will be hidden inside the walls until it ignites. This skill covers how to calculate loads accurately, apply the code requirements for continuous and noncontinuous loads, and size conductors for both ampacity and voltage drop.

## Core Rules

### Calculate the Connected Load Before Sizing Anything

The starting point of every circuit design is the connected load — the sum of all loads that will be connected to the circuit, expressed in volt-amperes (VA) or watts. For general-purpose receptacle circuits, the NEC specifies a minimum VA per receptacle or per square foot. For specific appliances, the nameplate rating is used. For lighting, the connected wattage of all fixtures is summed. The connected load is the raw input; it is then adjusted by demand factors, continuous-load multipliers, and code minimums to arrive at the design load that sizes the conductor and overcurrent protection. The trap is estimating the connected load from memory or from a "typical" circuit rather than calculating it for the specific installation — the actual loads may be higher than the estimate, and the conductor will be undersized from the start.

### Apply the 125% Factor to Continuous Loads

A continuous load is defined as a load where the maximum current is expected to continue for three hours or more. Examples include commercial lighting, refrigeration, data center loads, and electric heating. For continuous loads, the branch-circuit conductors and overcurrent protection must be sized at 125% of the load, not 100%. This means a 16-amp continuous load requires a conductor and breaker rated for at least 20 amps (16 × 1.25 = 20). The 25% margin accounts for the thermal buildup that occurs in conductors carrying sustained current — a conductor at its full ampacity rating for three hours will reach its temperature limit, and the margin prevents the insulation from operating at its thermal ceiling. The trap is treating all loads as noncontinuous by default, which undersizes the conductor for any load that actually runs continuously. The defense is to identify continuous loads explicitly during the design and to apply the 125% factor to them specifically.

### Size the Overcurrent Protection to Protect the Conductor

The overcurrent protection device (breaker or fuse) exists to protect the conductor from overheating due to overload or short circuit. The breaker rating must not exceed the ampacity of the conductor it protects — a 20-amp breaker on a 14 AWG conductor (rated 15 amps) will allow the conductor to carry 20 amps indefinitely, overheating it without tripping the breaker. The general rule is that the breaker rating matches the conductor ampacity: 15-amp breaker on 14 AWG, 20-amp on 12 AWG, 30-amp on 10 AWG. There are exceptions for motor circuits and specific applications, but the principle holds: the breaker protects the wire, and the wire must be able to carry what the breaker will allow. The trap is upsizing a breaker to stop nuisance tripping without upsizing the conductor — the tripping was telling the electrician that the load exceeds the conductor's capacity, and silencing the alarm does not fix the problem, it hides it until the wire ignites.

### Derate for Ambient Temperature and Conduit Fill

Conductor ampacity, as listed in NEC Table 310.16, is based on specific conditions: an ambient temperature of 30°C (86°F) and no more than three current-carrying conductors in a raceway or cable. When the actual conditions differ, the ampacity must be derated. In high ambient temperatures (attics, boiler rooms, direct sun exposure), the correction factors from the table reduce the ampacity. When more than three current-carrying conductors share a raceway, the mutual heating reduces each conductor's capacity, and the adjustment factors from Table 310.15(B)(3)(a) apply. These deratings can be significant — a conductor that carries 20 amps under standard conditions may be reduced to 15 amps or less after derating, requiring upsizing the wire gauge. The trap is selecting the conductor from the table without checking whether derating applies, and installing a conductor that is adequate under standard conditions but inadequate under the actual installation conditions. The defense is to always check the ambient temperature and conduit fill before finalizing the conductor size.

### Check Voltage Drop on Long Runs

Conductor ampacity ensures the wire will not overheat, but it does not ensure the load will receive adequate voltage. On long runs, the resistance of the conductor causes a voltage drop that reduces the voltage at the load. Excessive voltage drop causes motors to run hot and inefficiently, lights to dim, and electronics to malfunction. The general recommendation is to limit voltage drop to 3% on branch circuits and 5% total (feeder plus branch). Voltage drop is calculated from the conductor material (copper or aluminum), the cross-sectional area (AWG), the circuit length, and the load current. A conductor that is adequate for ampacity may need to be upsized one or two gauges to meet the voltage drop recommendation on a long run. The trap is sizing only for ampacity and ignoring voltage drop, which results in a circuit that is safe from fire but functionally inadequate for the load.

### Apply Motor Full-Load and Starting Current Rules

Motor circuits require special consideration because motors draw a starting current (locked-rotor current) that is 5 to 8 times their full-load running current. The branch-circuit conductors for a motor are sized at 125% of the full-load current (not the starting current), but the overcurrent protection must handle the starting surge without tripping. This is why motor circuits use time-delay fuses or inverse-time breakers that tolerate the brief starting surge while still protecting against sustained overload. The motor's overload protection (separate from the branch-circuit overcurrent protection) is sized at the full-load current and protects the motor from overheating. The trap is treating a motor like a resistive load — sizing the breaker at the full-load current will cause nuisance tripping on start, and the response of upsizing the breaker without upsizing the conductor creates an overload hazard. The defense is to use the NEC motor tables (Article 430) for full-load currents, apply the 125% factor to the conductors, and select time-delay overcurrent protection rated for the starting surge.

## Common Traps

### Sizing by "What Worked Before" Instead of Calculating

An electrician sizes a circuit by copying a similar installation — "the last kitchen I wired used 12 AWG on a 20-amp breaker, so this one will too." The trap is that the loads may be different: the new kitchen may have more appliances, a continuous-load exhaust hood, or a longer run that requires voltage-drop upsizing. Copying without calculating transfers the assumptions of one installation to another where they may not hold. The defense is to calculate the connected load for every circuit, even if it resembles a previous one, because the code requirements and the actual loads are specific to the installation.

### Upsizing the Breaker to Stop Tripping

A 15-amp breaker on a 14 AWG circuit trips repeatedly because the load exceeds 15 amps. The electrician, tired of the call-backs or the customer complaints, replaces the 15-amp breaker with a 20-amp breaker. The tripping stops — because the breaker now allows 20 amps through a conductor rated for 15. The conductor overheats, the insulation degrades, and eventually the wire ignites the wall cavity. The trap is that the tripping was a correct response to an overload, and silencing it did not fix the overload, it removed the only protection the conductor had. The correct response is to investigate why the breaker is tripping — is the load too large for the circuit? Is there a fault? Is the load continuous and the circuit is undersized? — and to fix the cause, not the symptom. Upsizing the breaker without upsizing the conductor is one of the most common and most dangerous errors in electrical work.

### Forgetting Derating on Hot or Crowded Runs

A conductor is selected from the ampacity table at its full rating — 12 AWG copper at 20 amps. But the conductor runs through a 40°C attic and shares a conduit with five other current-carrying conductors. After ambient temperature correction (0.82 factor) and conduit fill adjustment (0.80 factor for six conductors), the adjusted ampacity is 20 × 0.82 × 0.80 = 13.1 amps. The conductor that was "20 amp" is now only rated for 13 amps, and a 20-amp breaker on this conductor will allow sustained overload. The trap is that the derating factors compound multiplicatively, and the reduction can be dramatic. The defense is to always identify the installation conditions (ambient temperature, conduit fill) before selecting the conductor, and to verify the adjusted ampacity against the breaker rating.

### Ignoring Continuous Loads in Commercial Work

A commercial lighting circuit is calculated at the connected wattage and sized at 100%. But commercial lighting is a continuous load — it runs for more than three hours — and the conductors and breaker must be sized at 125%. The circuit is undersized by 25%, and the conductors run hot for the entire operating period, degrading the insulation over months and years. The trap is that the undersizing is not immediately apparent — the breaker does not trip because the load is below the breaker rating, but the conductors are operating above their continuous-duty capacity. The defense is to identify continuous loads during design and to apply the 125% factor to conductors, breakers, and devices (switches and receptacles are also rated for continuous loads at 80% of their ampere rating).

### Using the Motor's Nameplate Full-Load Current for Conductor Sizing

An electrician reads the motor nameplate full-load current (FLC) and sizes the conductor at 125% of that value. But the NEC requires using the FLC from the NEC motor tables (Table 430.248 for AC, 430.250 for three-phase), not the nameplate, for conductor and overcurrent sizing. The nameplate FLC reflects the specific motor's efficiency and may be lower than the table value, which represents a worst-case motor of that rating. Using the nameplate value can undersize the conductor if the motor is later replaced with one of the same rating but lower efficiency. The trap is that the nameplate appears to be the authoritative source, but the code explicitly requires the table values for sizing. The defense is to always use the NEC motor tables for conductor and overcurrent sizing, and to use the nameplate FLC only for the separate overload protection.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I calculate the connected load for the specific installation, including all receptacles, lighting, and appliances, rather than copying a similar circuit?
- Did I identify which loads are continuous (three hours or more) and apply the 125% factor to their contribution to the conductor and overcurrent sizing?
- Is the overcurrent protection rating equal to or less than the conductor ampacity, and if I am tempted to upsize a breaker, have I first upsized the conductor or investigated the cause of tripping?
- Have I checked the ambient temperature and conduit fill conditions and applied the derating factors, verifying that the adjusted ampacity still meets the load?
- Have I calculated the voltage drop on long runs and upsized the conductor if the drop exceeds 3% on branch circuits or 5% total?
- For motor circuits, am I using the NEC table full-load current (not the nameplate) for conductor sizing, applying the 125% factor, and using time-delay overcurrent protection rated for the starting surge?
- Is the separate motor overload protection sized to the nameplate full-load current, distinct from the branch-circuit overcurrent protection?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
