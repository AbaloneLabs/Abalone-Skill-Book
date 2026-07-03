---
name: ev-charging-load-management-and-demand-control.md
description: Use when the agent is sizing service or feeder capacity for multiple EV chargers, applying NEC 625.42 demand factors and automatic load management, designing load shedding or dynamic load control for EVSE, or determining whether an existing panel can support added charging loads without a service upgrade.
---

# EV Charging Load Management and Demand Control

A single EV charger is a large continuous load. Two or more chargers on the same service are a problem, because each one runs for hours at full current and they tend to run at the same time — every car in a garage plugs in around 6 PM and charges until midnight. If an electrician sizes the service by adding every charger at its full 125% continuous rating, the service must be enormous and the owner pays for capacity that statistics say will never be used simultaneously. If the electrician ignores the coincidence and undersizes, the main breaker trips every night. The judgment problem is that NEC 625.42 provides two legitimate tools — demand factors for multiple EVSE and automatic load management systems (ALMS) — but both have specific conditions, limits, and documentation requirements that are easy to apply wrong. An electrician who does not understand when the demand factor applies, how an ALMS changes the service calculation, and how dynamic load control interacts with the panel and the utility will either overbuild the service at great cost or underbuild it and create nightly tripping. This skill covers how to evaluate panel and service capacity for EV charging, when and how to apply demand reduction, and how to integrate load management so the system is both code-compliant and functional.

## Core Rules

### Recognize That Multiple EVSE Loads Coincide, Then Apply the Right Reduction

When multiple EVSE are supplied from a single service or feeder, the connected load is the sum of all chargers at their full rating, but the demand load — what the system must actually support — can be reduced. NEC 625.42 permits demand factors for multiple EVSE under specific conditions, reflecting that not all vehicles charge at full rate simultaneously and that managed charging can stagger or limit the load. The applicable reduction depends on the number of chargers and whether an automatic load management system is in place. Without an ALMS, a demand factor may still apply to multiple EVSE on a feeder, but the reduction is modest and the calculation must follow the specific table or rule in effect. The key insight is that the demand factor is a statistical allowance for coincidence, not permission to ignore capacity.

The trap is either applying no reduction and forcing an unnecessary service upgrade, or applying an aggressive reduction without the ALMS or documentation that justifies it. The defense is to determine the number of EVSE, check whether the demand-factor conditions are met, apply only the reduction the code permits for that configuration, and document the basis so the inspector can verify it.

### Understand When an Automatic Load Management System Changes the Calculation

An automatic load management system (ALMS) actively monitors the service or feeder load and dynamically allocates power among connected EVSE so that the total charging load never exceeds a set limit. NEC 625.42 permits the maximum EV charging load on a service or feeder equipped with an ALMS to be the system's configured limit, not the sum of the connected charger ratings. This is the mechanism that lets a building add many chargers without a service upgrade: the ALMS caps the total charging demand and shares it among vehicles, so ten 40-amp chargers might be limited to a combined 80 amps, distributed dynamically as vehicles connect and disconnect. The ALMS must be listed, it must limit the total charging load reliably, and each EVSE circuit must still be sized for its individual rating so that any single charger can deliver its full power when the system allocates it.

The trap is treating an ALMS as permission to undersize the individual branch circuits or the feeder below the system limit. The defense is to size each EVSE branch circuit at 125% of that EVSE's rating, to size the feeder or service to the ALMS configured limit plus all non-EV loads, and to confirm the ALMS is listed and installed per its listing.

### Evaluate the Existing Panel Before Adding Any Charging Load

Before specifying chargers, the electrician must determine whether the existing service and panel have capacity. This means performing or obtaining a load calculation for the building as it stands, then adding the EV charging load (after any permitted demand factor or ALMS reduction) and comparing the total to the service and feeder ratings. A panel that is already near its capacity cannot accept even one Level 2 charger without a load management strategy or a service upgrade. The 120% backfeed rule that applies to solar does not apply to EV charging — EV charging is a load, not a source — so the only ways to add charging capacity are to free up existing capacity, manage the load, or upgrade the service.

The trap is counting free breaker spaces as free capacity, when a panel can have empty spaces and zero spare ampacity simultaneously. The defense is to perform a load calculation that accounts for the existing connected and demand load, add the EV charging demand, and verify the total stays within the service and feeder ratings — and to recognize that a panel "full" of existing load needs load management or an upgrade regardless of physical space.

### Distinguish Static Load Shedding From Dynamic Load Management

Load management strategies fall on a spectrum from simple to sophisticated, and the choice affects both the code calculation and the user experience. Static load shedding uses a contactor or relay to drop a non-essential load (such as a water heater or a second charger) when a priority load energizes — a simple interlock that guarantees the two loads never run together, allowing the noncoincident-load rule to apply. Dynamic load management uses an ALMS that continuously measures the service load and modulates charger output in real time, sharing available capacity among all connected vehicles. Static shedding is cheap and reliable but binary; dynamic management is flexible and maximizes charger utilization but requires listed equipment and communication wiring between the ALMS controller and each EVSE.

The trap is choosing static shedding where dynamic sharing is needed (leaving charger capacity unused) or specifying dynamic management where a simple interlock would suffice (overcomplicating and overspending). The defense is to match the strategy to the load profile: use noncoincident-load interlocks for mutually exclusive loads, and use an ALMS where multiple chargers must share a constrained service intelligently.

### Size the Feeder and Service to the Managed Limit Plus All Other Loads

When an ALMS or demand factor reduces the EV charging contribution, the feeder and service are sized to the reduced EV load plus all other building loads calculated normally. The reduction applies only to the EV charging portion; the rest of the building's load calculation is unaffected. The feeder conductors, the main breaker, and the service equipment must all carry the total — reduced EV load plus full building load — with the continuous-load margin applied where applicable. This means a building with a 200-amp service, 120 amps of existing demand, and an ALMS-limited EV load of 60 amps needs a service that can carry 180 amps, which the existing 200-amp service can support — but only because the ALMS caps the EV load. If the ALMS fails open (allowing full charger output), the service could be overloaded, so the ALMS must fail safe or the system must be designed so that even full charger output does not exceed the service rating.

The trap is sizing to the managed limit without considering what happens if the management system fails or is bypassed. The defense is to verify the ALMS fails in a current-limiting state, or to confirm that even unmanaged full output does not exceed the service rating, and to document the management dependency in the as-built records.

### Coordinate the Load Management System's Communication and Control Wiring

An ALMS or dynamic load management system depends on communication between a controller and each EVSE, and that communication infrastructure is part of the electrical installation. The controller must read the service or feeder load (via current transformers on the main feeders), communicate charge limits to each EVSE (via hardwired signal, Modbus, OCPP over Ethernet, or wireless), and the EVSE must respond by reducing its pilot-signal current allocation to the vehicle. If the communication fails, the system must default to a safe state — typically reduced or zero charging, not unlimited. The wiring for current sensors, control signals, and network connections must be installed per the ALMS manufacturer's instructions, and the routing must avoid interference from the high-current charging conductors.

The trap is treating the control wiring as an afterthought and discovering at commissioning that the current sensors are misplaced, the communication is unreliable, or the EVSE do not respond to the controller. The defense is to plan the sensor placement, control wiring, and network architecture during rough-in, to follow the manufacturer's installation instructions exactly, and to commission the system by verifying that each EVSE reduces output when the controller commands it.

## Common Traps

### Adding Every Charger at Full Rating and Forcing an Unnecessary Service Upgrade

An apartment building wants ten Level 2 chargers, each 40 amps. The electrician sizes the service at ten times 50 amps (the 125% value) plus the building load, requiring a massive service upgrade. The mechanism of the trap is that without applying the multiple-EVSE demand factor or installing an ALMS, the connected load is taken at full value, ignoring that the chargers will rarely all run at full rate simultaneously. The false signal is that the arithmetic is correct — the chargers are 40 amps each — but it ignores the statistical and managed coincidence that the code permits to reduce. The harm is an oversized, expensive service that the owner may decline, killing the project. The defense is to evaluate the demand factor and ALMS options that legitimately reduce the required capacity.

### Applying a Demand Factor Without Meeting Its Conditions

An electrician applies a generous demand factor to a group of EVSE to avoid a service upgrade, but the chargers are unmanaged and the conditions for the reduction are not met. The mechanism of the trap is that demand factors for multiple EVSE have prerequisites — often the presence of an ALMS or a specific minimum number of chargers — and applying the factor without meeting them produces an undersized service. The false signal is that a demand factor "exists" for EV charging, which is true but conditional. The harm is a service that trips under real-world simultaneous charging. The defense is to verify the specific conditions for any demand factor before applying it and to document compliance.

### Counting Free Breaker Spaces as Free Panel Capacity

A residential panel has six empty spaces, and the electrician assumes it can accept a 50-amp EV charger circuit. The mechanism of the trap is that physical space and electrical capacity are independent; the panel may already be loaded to its service rating with existing circuits, leaving no ampacity for a 40-amp continuous charger load. The false signal is the empty spaces, which permit the physical connection but not the loading. The harm is a service that trips when the charger runs alongside the existing load. The defense is to perform a load calculation before adding any EV charging load and to use load management or a service upgrade when capacity is insufficient.

### Sizing Individual Branch Circuits Down Because an ALMS Caps the Total

A building installs an ALMS that limits total EV charging to 80 amps, and the electrician runs undersized branch circuits to each charger reasoning that the ALMS will never let them draw full current. The mechanism of the trap is that the ALMS allocates the total dynamically, and any single charger may receive the full system limit when others are idle, so each branch circuit must be sized for the individual EVSE rating. The false signal is the system cap, which limits the total but not any individual circuit. The harm is an undersized branch circuit that overheats when the ALMS routes full power to one charger. The defense is to size every EVSE branch circuit at 125% of that EVSE's rating regardless of the ALMS total limit.

### Ignoring the Fail-Safe Behavior of the Load Management System

An ALMS is installed and the service is sized to the managed limit, but the system fails in a state that allows all chargers to run at full output, overloading the service. The mechanism of the trap is that load management systems must fail safe, and a failure that opens the throttle defeats the capacity assumption the service sizing relied on. The false signal is that the system works during commissioning, which proves the managed state but not the failure state. The harm is a service overload and main-breaker tripping when the controller fails. The defense is to verify the ALMS fails in a current-limiting state and to document the failure mode.

### Overlooking the Edge Case or Exception

The standard multi-charger installation with an ALMS is handled well, but the case of chargers on a shared feeder with mixed managed and unmanaged units, or a single high-amperage charger on an already-loaded service, is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- For multiple EVSE on one service or feeder, did I determine whether the demand-factor conditions are met and apply only the reduction the code permits for that configuration, with the basis documented?
- If an ALMS is used, did I size each individual EVSE branch circuit at 125% of its own rating, and size the feeder or service to the ALMS configured limit plus all non-EV loads?
- Did I perform a load calculation on the existing service before adding EV charging, and did I distinguish free physical breaker spaces from actual spare ampacity?
- Did I choose between static load shedding (for mutually exclusive loads) and dynamic load management (for shared capacity) based on the load profile rather than defaulting to one approach?
- Did I verify the ALMS or load management system fails in a current-limiting state, or confirm that even unmanaged full charger output does not exceed the service rating?
- Did I plan the current-sensor placement, control wiring, and communication architecture during rough-in, and commission the system by verifying each EVSE responds to the controller's charge-limit commands?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
