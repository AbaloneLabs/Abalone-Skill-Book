---
name: motor-starting-methods-and-overload-protection.md
description: Use when the agent is selecting a motor starting method, sizing overload protection, applying NEC Article 430 rules, choosing between across-the-line starters, soft starters, or VFDs, or troubleshooting motor starting and protection problems.
---

# Motor Starting Methods and Overload Protection

Motors are the single largest load class in most industrial facilities, and they are also the load class most prone to being misapplied, under-protected, or damaged by the very equipment meant to control them. The judgment problem in motor work is that starting method, overload protection, short-circuit protection, conductor sizing, and the driven load are not independent decisions — they are a coupled system, and an error in one ripples through the others. An electrician who picks a starter based on convenience, sizes an overload by matching the motor nameplate without reading the service factor, or ignores the difference between running overload and short-circuit protection will eventually burn out motors, trip breakers unpredictably, or create a fire. NEC Article 430 governs this work, and it is one of the most interdependent articles in the Code — nearly every rule cross-references another. This skill covers the decisions that determine whether a motor starts cleanly, runs protected, and is installed to Code.

## Core Rules

### Choose the Starting Method Based on the Load and the Supply, Not on Convenience

Across-the-line (full-voltage) starting applies the entire line voltage to the motor terminals at once. It is the simplest, cheapest, and most reliable method, and it is correct for most small motors on a stiff supply. But it draws roughly six times full-load current during the inrush period, and it produces a torque spike that can damage the driven equipment or the coupling. The decision to use a reduced-voltage method — soft starter, autotransformer, primary resistor/reactor, wye-delta, part-winding, or VFD — is driven by two constraints: the supply's ability to tolerate the inrush without an unacceptable voltage sag, and the mechanical load's ability to tolerate the torque transient. The trap is defaulting to across-the-line because it is cheap and then discovering that the starting current trips the upstream breaker, causes lights to flicker across the facility, or shears a shaft coupling on a high-inertia load. The defense is to evaluate the available short-circuit current, the transformer and conductor impedance, the load inertia, and the utility or facility voltage-drop limits before committing to a method.

### Size the Overload to the Motor's Actual Full-Load Current, With the Service Factor in Mind

Overload protection (Part III of Article 430) protects the motor from sustained overcurrent that would overheat and destroy the windings. The overload is sized as a percentage of the motor's full-load current (FLC) as shown on the nameplate — not the FLC table values in 430.6, which are used for conductor and short-circuit sizing. For a motor with a service factor of 1.15 or greater, the overload is typically set at 125% of nameplate FLC; for a service factor of 1.0, it is set at 115%. The trap is two-fold: using the table FLC instead of the nameplate FLC for the overload setting (the tables are conservative and will under-protect), or ignoring the service factor and applying a blanket 125% to a 1.0 SF motor, which allows the motor to run hot enough to shorten its insulation life dramatically. The defense is to read the nameplate, confirm the service factor and temperature rise, and set the overload class (10, 20, or 30) to match the motor's starting characteristic — a class 10 overload on a high-inertia load will nuisance-trip on every start.

### Separate Running Overload Protection From Short-Circuit and Ground-Fault Protection

This is the single most misunderstood distinction in Article 430. Running overload protection (Part III) responds to sustained overcurrent above the motor's rating and protects the motor. Short-circuit and ground-fault protection (Part IV) responds to fault current and protects the conductors and the equipment. They are sized completely differently: the overload is a percentage of nameplate FLC, while the short-circuit device is sized from the FLC tables and may be as high as 250%, 400%, 800%, or even 1100% of FLC depending on the device type per Table 430.52. The trap is conflating the two — assuming a properly sized circuit breaker also provides adequate overload protection, or assuming an overload block protects against a bolted fault. A breaker sized at 250% of FLC for short-circuit protection will not trip on a 130% overload soon enough to save the motor, and an overload block will not clear a phase-to-phase fault. The defense is to install both, understand which failure each addresses, and verify the coordination so the device closest to the fault operates first.

### Size Conductors From the Table FLC, Not the Nameplate FLC

NEC 430.6 requires that for most motor applications, the conductor ampacity, the short-circuit device, and the disconnect rating be based on the FLC values in Tables 430.247 through 430.250 — not the nameplate. The tables are intentionally conservative to account for motor replacement with a unit of the same horsepower but different efficiency. The conductor must carry at least 125% of the table FLC per 430.22. The trap is sizing the conductor from the nameplate, which is lower for a high-efficiency motor, and then discovering that when the motor is later replaced with a standard-efficiency unit of the same horsepower, the conductor is now undersized and overheating. The defense is to always use the table value for the installation, and reserve the nameplate value only for the overload setting where 430.6(A)(2) explicitly requires it.

### Respect the Two-Motor and Multimotor Feeder Rules

When a feeder supplies more than one motor, the feeder conductor is sized at 125% of the largest motor's FLC plus the sum of the FLCs of all other motors on the feeder per 430.24, and the feeder short-circuit device is sized from the largest motor's short-circuit rating plus the sum of the full-load currents of the other motors. The trap is treating each motor branch circuit independently and sizing the feeder as if only the largest motor ever runs — in a process where multiple motors start and run simultaneously, the feeder is overloaded. The defense is to enumerate all motors on the feeder, identify the duty cycle and simultaneous operation, and size the feeder for the realistic combination, not the single largest.

### Verify the Disconnect Means Is Rated and Located Correctly

A controller disconnect must be in sight of the motor (within 15 m / 50 ft and visible) or capable of being locked in the open position per 430.102. The disconnect must also have an ampere and horsepower rating suitable for the motor. The trap is using a general-purpose safety switch without a horsepower rating, or installing the disconnect out of sight of the motor and relying on a lockable mechanism that no one actually locks. The defense is to confirm the disconnect carries the proper HP rating for the voltage and that it is positioned so the worker can see the motor while standing at the disconnect.

## Common Traps

### Sizing the Overload From the FLC Table Instead of the Nameplate

The electrician reads the horsepower from the motor nameplate, looks up the full-load current in NEC Table 430.250, and sets the overload relay to 125% of that table value. The table value is intentionally conservative and is higher than the actual nameplate FLC of a modern high-efficiency motor, so the overload is set too high to protect the motor under a real sustained overload. The mechanism of the harm is insulation degradation: a motor running at a modest overload for hours does not trip, but the winding temperature rises into the range where insulation life is halved for every 10 degrees C of excess. The false signal is that the motor "runs fine" because it does not nuisance-trip, while it is quietly being cooked. The defense is to use the nameplate FLC exclusively for the overload setting and the table FLC exclusively for conductor and short-circuit sizing.

### Applying a Class 10 Overload to a High-Inertia Load

Class 10, 20, and 30 overload relays trip at different rates during a sustained overload, with class 10 being the fastest. A high-inertia load — a large fan, a centrifuge, a loaded conveyor — may require 15 to 30 seconds to reach full speed, and during that time the motor draws starting current. A class 10 overload, designed to trip within 10 seconds at six times its rating, will nuisance-trip on every start. The electrician, frustrated by the trips, either raises the overload setting beyond what protects the motor or disables the overload entirely. The false signal is that the overload is "too sensitive," when in fact it is the wrong class for the application. The harm is that the workaround defeats the protection the motor needs once running. The defense is to match the overload trip class to the motor's actual starting time, measured under load.

### Assuming a Properly Sized Breaker Provides Overload Protection

A motor branch circuit uses a circuit breaker or fuse sized at 250% to 800% of FLC for short-circuit protection, far above the motor's running current. The electrician installs only the breaker, reasoning that "the breaker protects the circuit." The mechanism of the failure is that the breaker's thermal element, if it has one, is calibrated for cable protection and will not respond to a 130% motor overload until the windings are already damaged. The false signal is that the breaker carries the motor's normal running current without tripping, suggesting it is correctly protecting the motor. The harm is that the motor burns out under a sustained overload that the breaker never sees as a fault. The defense is to install a separate overload device sized to the nameplate FLC, and to understand that the breaker and the overload protect different things.

### Ignoring the Service Factor When Setting the Overload

A motor nameplate reads 1.0 service factor, and the electrician sets the overload at 125% because "that's what we always do." A 1.0 SF motor is not designed to run continuously above its nameplate rating, and 125% allows it to run hot enough to destroy the insulation within months. The mechanism is that the overload percentage is supposed to track the service factor — 115% for 1.0 SF, 125% for 1.15 SF or greater — and applying the wrong percentage shifts the motor's thermal operating point into the damage zone. The false signal is that the motor runs and the customer does not complain, because insulation failure is gradual and cumulative. The harm is premature motor failure, often blamed on the motor manufacturer rather than the protection setting. The defense is to read the service factor on every nameplate and adjust the overload percentage accordingly.

### Using a Reduced-Voltage Starter Where the Supply Cannot Tolerate Across-the-Line, Without Checking the Mechanical Load

The electrician selects a soft starter to limit inrush because the utility or the facility engineer complained about voltage sag. The soft starter ramps the voltage, limiting the current, but also limiting the starting torque — which for a soft starter is roughly proportional to the square of the voltage reduction. A load requiring high breakaway torque — a positive-displacement pump, a loaded screw conveyor — may never accelerate to full speed, stalls in the ramp, and the soft starter trips on stall protection or the motor overheats from prolonged starting current. The false signal is that the soft starter "doesn't work," when in fact the load's torque requirement exceeds what the reduced-voltage method can deliver. The harm is repeated failed starts, motor damage, and a rework to install a method that can deliver the torque, such as a VFD with full torque at low speed. The defense is to obtain the load's speed-torque curve before selecting a reduced-voltage method.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I select the starting method based on the supply's tolerance for inrush and the load's torque and inertia requirements, rather than defaulting to the cheapest option?
- Did I size the overload relay from the motor nameplate full-load current (not the NEC table) and adjust the percentage for the service factor (115% for 1.0 SF, 125% for 1.15 SF or greater)?
- Did I size the branch-circuit conductor at 125% of the table FLC from 430.6, and the short-circuit device from Table 430.52, recognizing these are separate calculations from the overload?
- Did I install both running overload protection and short-circuit/ground-fault protection, and do I understand which failure mode each is designed to clear?
- Did I match the overload trip class (10, 20, or 30) to the motor's actual starting time under load, rather than assuming a default class?
- For a multimotor feeder, did I size the conductor and short-circuit device at 125% of the largest motor plus the sum of the others per 430.24, accounting for realistic simultaneous operation?
- Does the controller disconnect carry a horsepower rating suitable for the motor, and is it located within sight of the motor or lockable in accordance with 430.102?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
