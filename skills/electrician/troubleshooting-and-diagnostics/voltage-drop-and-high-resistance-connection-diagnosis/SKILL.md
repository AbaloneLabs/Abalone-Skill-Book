---
name: voltage-drop-and-high-resistance-connection-diagnosis.md
description: Use when the agent is diagnosing dim or flickering lights, low voltage at receptacles, overheating connections, neutral problems causing overvoltage, or locating loose and corroded splices and terminals that cause voltage drop under load.
---

# Voltage Drop and High-Resistance Connection Diagnosis

The most common and most dangerous electrical faults are not dead shorts or open circuits — they are high-resistance connections that pass voltage at no load but drop it under load, heat up, and slowly degrade until they ignite. A loose wire nut, a corroded splice, a backstabbed receptacle that has loosened over years, a terminal screw that was never torqued correctly: all of these create a connection that reads full voltage with a meter but collapses when current flows. The judgment problem is that these faults are invisible to a simple voltage check, they produce symptoms (dim lights, intermittent operation, flickering) that are easy to dismiss, and the high-resistance connection heats progressively until it ignites the surrounding material. An electrician who measures voltage unloaded and declares the circuit good has missed the exact fault most likely to cause a fire. This skill covers how to find high-resistance connections, diagnose neutral faults that cause dangerous overvoltage, and distinguish a conductor-length voltage drop from a connection voltage drop.

## Core Rules

### Measure Voltage Drop Under Load, Not Just Voltage at the Receptacle

A high-resistance connection is invisible to an unloaded voltage measurement because the high-impedance meter draws essentially no current, and the voltage drop across the connection (which equals current times resistance) is near zero. The receptacle reads 120 volts with nothing plugged in. But when a load drawing 10 amps is connected, the high-resistance connection drops several volts, and the load receives only 110 volts or less. The diagnostic technique is to measure voltage at the receptacle with a known load connected, and to compare it to the unloaded voltage. A drop of more than a few volts under load indicates a high-resistance connection somewhere in the circuit. The trap is measuring only unloaded voltage and concluding the circuit is healthy. The defense is to always measure under load, using a test load that draws significant current (a 1500-watt heater or similar), and to investigate any voltage drop exceeding about 3 to 5 percent.

### Distinguish Conductor-Length Voltage Drop From Connection Voltage Drop

Voltage drop occurs in two ways: the unavoidable resistance of a long conductor, and the abnormal resistance of a poor connection. A long circuit with correctly sized conductors may drop 4 or 5 volts from source to load — this is a design issue, addressed by upsizing the conductors. A short circuit with a loose connection may drop 10 or 15 volts across that single connection — this is a fault, addressed by repairing the connection. The diagnostic distinction is where the voltage is lost. If the voltage drops gradually along the conductor length, it is conductor drop. If the voltage is normal at one point and drops sharply at the next, the drop is concentrated at a connection between those points. The trap is assuming all voltage drop is conductor length and recommending an expensive conductor upsize when the actual problem is a loose splice. The defense is to measure voltage at multiple points along the circuit and locate where the drop is concentrated, distinguishing distributed conductor drop from concentrated connection drop.

### Locate the Faulty Connection by Measuring Voltage Drop Across Each Connection

To find the specific high-resistance connection, measure the voltage drop across each connection (splice, terminal, device) while the circuit is under load. Place one meter probe on the line side of the connection and the other on the load side, with current flowing. A good connection drops near zero volts (millivolts). A high-resistance connection drops a volt or more, and the defective connection is immediately identified by the reading. This technique works on wire nuts, terminal screws, splices, breaker terminals, and bus connections. The trap is measuring voltage to ground or to neutral at each point, which can be misleading because the neutral path may also have drop, instead of measuring directly across each connection. The defense is to measure directly across each connection under load, and to investigate any connection that drops more than a fraction of a volt.

### Diagnose Neutral Faults That Cause Overvoltage on One Leg and Undervoltage on the Other

A lost or high-resistance neutral on a split-phase (120/240) or three-phase wye system causes dangerous voltage redistribution. On a 120/240 residential service, if the neutral is lost or high-resistance between the transformer and the service, the two 120-volt legs are no longer held at 120 volts each — instead, the voltage divides based on the load imbalance. The lightly loaded leg rises toward 240 volts (destroying electronics and appliances), while the heavily loaded leg drops toward zero. The symptom is lights getting brighter on one side of the house and dimmer on the other, often when a large load turns on or off. The trap is treating this as a low-voltage problem on the dim side and missing the overvoltage on the bright side, which is the more dangerous condition. The defense is to measure both legs to neutral simultaneously when the symptom appears, to recognize that a shifting neutral causes opposite voltage changes on the two legs, and to locate the neutral fault (which may be at the service, in the meter base, at the transformer, or in a subpanel) before the overvoltage destroys connected equipment.

### Use Thermal Inspection to Find Heating Connections Before They Fail

A high-resistance connection dissipates power as heat (power equals current squared times resistance), and the heat is proportional to both the resistance and the square of the current. A connection that drops 2 volts at 15 amps dissipates 30 watts — enough to raise the connection temperature significantly and, over time, to discolor, oxidize, and ignite adjacent material. A thermal imaging camera or an infrared thermometer can detect connections that are running hot before they fail, by scanning a panel or junction box for temperature anomalies. The trap is relying on visual inspection alone, because a heating connection inside a closed wire nut or behind a device shows no visible sign until it has already caused damage. The defense is to use thermal inspection on loaded panels and junctions, to investigate any connection running significantly hotter than its neighbors, and to recognize that a hot connection is a connection in the process of failing.

### Recognize the Symptoms That Point to a High-Resistance Connection

Certain symptoms are classic indicators of a high-resistance connection: lights that dim when another load turns on, a receptacle where loads run slow or hot, intermittent operation of a device, a burning smell or discoloration at a receptacle or switch, a breaker that trips only under heavy load. These symptoms all indicate that voltage is being lost somewhere in the circuit under load, and the location of the loss is a high-resistance connection. The trap is dismissing dimming lights as "normal" or attributing intermittent operation to the load itself rather than the supply. The defense is to treat any load-dependent voltage symptom as a high-resistance connection until proven otherwise, to measure under load, and to trace the connection responsible.

## Common Traps

### Measuring Unloaded Voltage and Declaring the Circuit Good

An electrician measures 120 volts at a receptacle with a high-impedance meter and nothing plugged in, and reports the circuit is healthy. The customer plugs in a vacuum and the lights dim. The trap is that the unloaded measurement showed full voltage because the meter draws no current, and the high-resistance connection (a loose backstab at an upstream receptacle) dropped no voltage at zero current. The mechanism of harm is that the fault is missed, the connection continues to heat and degrade each time a load is used, and it eventually ignites. The false signal is the full voltage reading. The defense is to measure with a load connected, to compare loaded and unloaded voltage, and to never declare a circuit good based on an unloaded measurement alone.

### Assuming Voltage Drop Is Conductor Length When It Is a Connection

An electrician measures 108 volts at a receptacle under load on a long circuit and recommends upsizing the conductors from 12 AWG to 10 AWG to address voltage drop. But the actual cause is a loose wire nut in a junction box halfway along the circuit, which is dropping 8 of the 12 lost volts. The trap is that voltage drop was present and the conductor length seemed like the obvious cause, so the connection was never investigated. The mechanism of harm is that the expensive conductor upsize does not fix the fault (the loose connection continues to heat and degrade), and the customer pays for a repair that does not resolve the symptom. The false signal is that the circuit is long and voltage drop is expected. The defense is to measure voltage at multiple points along the circuit to locate where the drop is concentrated, and to distinguish a sharp drop at a connection from a gradual drop along the conductor.

### Missing a Lost Neutral and Diagnosing Only the Low-Voltage Side

A customer reports that the lights dim when the refrigerator starts. The electrician measures 95 volts on the affected leg and concludes there is a low-voltage problem, recommending a new breaker or panel. But the electrician never measured the other leg, which is reading 145 volts and is destroying the customer's electronics. The trap is that the dimming leg drew attention and the overvoltage leg was never checked. The mechanism of harm is that the neutral fault goes undiagnosed, the overvoltage continues to destroy connected equipment, and the recommended repair does not address the actual cause. The false signal is the obvious low-voltage symptom on one leg. The defense is to measure both legs to neutral simultaneously whenever a voltage symptom appears, to recognize that opposite voltage changes on the two legs indicate a neutral fault, and to locate and repair the neutral before the overvoltage causes a fire or equipment destruction.

### Relying on Visual Inspection and Missing a Heating Connection Inside a Closed Device

An electrician inspects a panel and a series of receptacles, looking for discoloration or burning. Everything looks clean, so the circuit is declared safe. But inside a wire nut at a hidden splice, a connection is running at 90 degrees Celsius under load, slowly oxidizing and degrading the insulation. The trap is that visual inspection cannot see inside closed connections, and the heating connection shows no external sign until it has already charred the wire nut or ignited the wall. The mechanism of harm is that the fault progresses undetected until it fails catastrophically. The false signal is that everything looked clean on visual inspection. The defense is to use thermal inspection (an infrared camera or thermometer) on loaded connections, to measure voltage drop across connections under load, and to recognize that a connection can be dangerously hot with no visible external evidence.

### Dismissing Dimming Lights as Normal Rather Than Diagnosing the Cause

A customer reports that the kitchen lights dim when the microwave runs. The electrician tells the customer this is normal for an older home and does no further testing. The trap is that dimming under load is a classic symptom of a high-resistance connection, and dismissing it as normal allows the connection to continue degrading toward ignition. The mechanism of harm is that the loose connection (often a backstabbed receptacle or an old wire nut) heats each time the load cycles, the resistance increases over time in a runaway process, and the connection eventually ignites. The false signal is that dimming is common and therefore seems benign. The defense is to treat any load-dependent dimming as a diagnostic signal, to measure voltage drop under load, and to locate and repair the high-resistance connection rather than dismissing the symptom.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I measure voltage at the receptacle or device with a significant load connected, and compare it to the unloaded voltage, rather than relying on an unloaded measurement?
- Did I measure voltage at multiple points along the circuit to distinguish a gradual conductor-length voltage drop from a sharp drop concentrated at a connection?
- Did I measure voltage drop directly across each suspect connection (line side to load side) under load, and investigate any connection dropping more than a fraction of a volt?
- If the symptom involves voltage changes on a split-phase or three-phase system, did I measure all legs to neutral simultaneously to detect a neutral fault causing overvoltage on one leg?
- Did I use thermal inspection (infrared camera or thermometer) on loaded panels and junctions to find connections running hotter than their neighbors?
- Did I treat load-dependent dimming, intermittent operation, or low voltage as a high-resistance connection symptom until proven otherwise, rather than dismissing it as normal?
- Did I confirm the repaired connection by re-measuring voltage drop under load after the repair, verifying the drop is now negligible?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
