---
name: ddc-controller-point-mapping-and-wiring.md
description: Use when the agent is wiring DDC controllers, mapping analog and digital I/O points, and connecting sensors and actuators to building automation systems, covering point types, 0-10V versus 4-20mA signals, relay and triac outputs, and point-to-wire mapping documentation.
---

# DDC Controller Point Mapping and Wiring

Direct digital control (DDC) controllers are the field computers that read sensors, command actuators, and execute the sequences of operation that keep a building comfortable and efficient. The judgment problem is that landing points looks like routine low-voltage terminal work, which invites installers to treat each input or output as interchangeable, to choose whatever point is free, and to treat the point-to-wire map as paperwork filled in at the end. In reality every point is typed, every signal has an electrical signature, and every wire must map to a defined software object that the sequence depends on. When the mapping is wrong, the controller reads the wrong sensor, commands the wrong actuator, or drives a loop that never settles, and the building underperforms for years because the defect is hidden inside plausible-looking data. This skill covers the point types, the analog and digital signal families, the output hardware options, and the documentation that makes a DDC installation commissionable and maintainable.

## Core Rules

### Classify Every Point by Type Before Landing Any Wire

DDC points fall into four families: analog inputs (AI), analog outputs (AO), digital inputs (DI), and digital outputs (DO). Analog inputs accept variable signals such as thermistor resistance, RTD, 0-10V, or 4-20mA. Analog outputs command variable signals to actuators. Digital inputs read discrete states from dry contacts or voltage. Digital outputs switch discrete loads through relay or triac hardware. The first act of mapping is to confirm, from each device datasheet, what family and signal the device produces or accepts, and to reserve a controller point of the matching type. A point is never landed on whatever terminal is free; it is landed on the terminal whose configured type matches the device. The controller point database must mirror the physical wiring exactly, because the sequence of operations addresses points by that database.

### Choose 0-10V Versus 4-20mA by Distance and Noise Environment

The two dominant analog signal families trade off simplicity against robustness. A 0-10V signal is easy to generate and read, but it is a voltage source, so it suffers voltage drop on long runs and picks up induced noise when run near power wiring. A 4-20mA current loop is a current source, which rejects voltage drop over distance and is far more immune to induced noise, at the cost of a dedicated pair and a loop power arrangement. The decision rule is to prefer 4-20mA for runs beyond roughly 30 meters, for runs that share raceway with power, and for any signal that crosses between buildings, and to reserve 0-10V for short, clean runs inside a single equipment room. A 0-10V signal run 100 meters across a plant will read offset and noisy, and the trap is that it still reads a value, so the defect is not obvious.

### Match Output Hardware to the Load Being Switched

Digital outputs come as relay outputs or triac outputs, and the choice is dictated by the load. Relay outputs are dry contacts that can switch alternating or direct current, resistive or inductive loads, and they are rated in amperes with a stated pilot duty for inductive coils. Triac outputs are solid-state AC switches suited to low-current resistive loads and to some actuator drive signals, but they leak a small current when off and cannot switch DC. Driving a large motor starter coil from a triac output, or switching a DC load from a triac, is a mismatch that either fails to switch or damages the output. Each DO must be checked against the load's voltage, current, and inrush, and the output type must be confirmed before the load is wired. Where the controller output cannot meet the load, an interposing relay is added rather than overloading the point.

### Provide Correct Power, Reference, and Shielding for Each Signal Family

Each signal family has wiring rules that protect signal integrity. Thermistor and RTD inputs are low-level resistance signals and must use shielded cable with the shield grounded at one end only, to prevent ground loops while blocking induced noise. Current loops need a dedicated pair with loop power supplied at one end and the shield grounded per the manufacturer's scheme. Voltage signals should be kept short and separated from power. Digital dry-contact inputs need a shared reference and often a wetting current so that the contact is reliably read. Powering a 4-20mA two-wire transmitter without loop power, or grounding a thermistor shield at both ends, are errors that produce readings which look plausible but are wrong. The wiring method follows the signal family, not a single generic practice.

### Keep Signal, Communication, and Power Wiring Separated

A DDC cabinet carries three classes of wiring: sensor and actuator signals, communication buses such as BACnet MS/TP, and controller and actuator power. These must be separated to prevent crosstalk and induced noise. The practice is to use separate raceways where practical, listed barriers where circuits share an enclosure, and maintained spacing where cables run parallel. Bundling an RS-485 bus with the 24V actuator supply in one conduit invites intermittent communication errors that resist diagnosis, because the defect appears only under specific load and noise conditions. Separation is a design rule applied at the planning stage, not a cleanup done after the wiring is in.

### Build the Point-to-Wire Map as a Living Contract Document

The point-to-wire map, often expressed as the I/O point list or the points schedule, is the document that ties every field device to a controller point, a terminal number, a software object, a range, and a place in the sequence of operations. It is the contract between the installer, the controls contractor, and the commissioning agent. The map is built before installation, updated with every field change, and used as the basis for point-to-point checkout. A map that is drafted early and never reconciled with the field makes checkout impossible, because no one can confirm that a given wire lands on the point and maps to the object the document claims. The map is not paperwork; it is the verification backbone of the system.

### Plan Spare Points and a Consistent Naming Convention

A well-designed controller reserves spare points of each type for future expansion and for field corrections, because adding a sensor after the cabinet is full forces a redesign. The naming convention for points must be consistent, hierarchical, and human-readable, encoding the system, the equipment, the measurement, and the point type, so that a point name alone tells a technician what it is. Inconsistent or cryptic naming makes the system unmaintainable, because no one can find the right point in the database. Spare capacity and naming discipline are decided at design, not recovered later.

## Common Traps

### Landing a Point on the Wrong I/O Type

The installer lands a 0-10V sensor on an input configured for 4-20mA, or a thermistor on an analog output, because the terminal was free and the connector fit. The mechanism is that each I/O type expects a specific electrical signal, and a mismatch produces a reading that may look plausible but is wrong, or it damages the point. The false signal is that the point reads a value, which proves a connection but not a correct match. The harm is a controller acting on bad data, commanding actuators to wrong positions, and a building that never performs, traced only after weeks of nuisance.

### Using 0-10V on a Long Noisy Run

The installer pulls a 0-10V signal 80 meters across a plant in the same tray as motor power to save a converter. The mechanism is that the voltage signal suffers drop and picks up induced noise along the run, so the controller reads an offset and noisy value that drifts with the load on the adjacent power. The false signal is that the reading is plausible at the bench or on a quiet day. The harm is a control loop that never settles and an actuator that hunts, wasting energy and wearing the equipment.

### Driving an Inductive Load From a Triac Output

The installer wires a motor starter coil directly to a triac digital output rated for resistive loads. The mechanism is that the inductive coil's inrush and flyback exceed the triac's rating, and the off-state leakage may hold the coil partially energized, so the triac fails or the contactor chatters. The false signal is that the output switches the coil during a quick bench test. The harm is a failed output or a chattering contactor that damages the starter and interrupts the controlled equipment.

### Grounding a Shield at Both Ends

The installer grounds a thermistor cable shield at the controller and again at the sensor junction box to be safe. The mechanism is that the two ground points sit at different potentials, driving ground-loop current through the shield that induces noise into the low-level signal. The false signal is that the sensor reads cleanly when the building ground is quiet. The harm is a temperature input that wanders with ground activity, destabilizing the loop and resisting diagnosis.

### Letting the Point List Drift From the Field

The point list was drafted early, the field wiring changed during installation, and the list was never reconciled. The mechanism is that the list is the checkout basis, and a list that does not match the field makes point-to-point verification impossible. The false signal is that the list exists and looks complete. The harm is a commissioning process that cannot confirm the system and a building accepted with hidden mismatches that surface as operational problems.

### Sharing the Controller Supply With Actuators

The installer powers the controller and its actuators from one 24V supply to save a homerun. The mechanism is that actuators draw inrush when they move, collapsing the shared supply and resetting the controller mid-sequence. The false signal is that the controller runs fine when actuators are idle. The harm is a controller that resets whenever it commands a move, defeating the sequence and producing a system that never completes its actions.

## Self-Check

- Did I confirm each device's signal type from its datasheet and reserve a controller point of the matching family (AI, AO, DI, DO), keeping the point database in lockstep with the physical wiring?
- Did I choose 4-20mA for long, noisy, or between-building runs and reserve 0-10V for short clean runs, rather than defaulting to one signal family everywhere?
- Did I match each digital output hardware type (relay versus triac) to the load's voltage, current, and inductive character, adding interposing relays where the controller output cannot meet the load?
- Did I provide shielded cable for thermistor and RTD inputs with the shield grounded at one end only, dedicated pairs for current loops, and correct loop power for two-wire transmitters?
- Did I separate signal, communication, and power wiring into their own raceways or with listed barriers, and maintain spacing where parallel runs are unavoidable?
- Did I build the point-to-wire map before installation, keep it updated with every field change, and use it as the basis for point-to-point checkout?
- Did I reserve spare points of each type and apply a consistent, hierarchical, human-readable naming convention?
- Does the output stay within the agent's scope, deferring licensed judgment, final authority, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
