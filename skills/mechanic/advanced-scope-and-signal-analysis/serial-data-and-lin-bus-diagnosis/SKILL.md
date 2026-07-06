---
name: serial-data-and-lin-bus-diagnosis.md
description: Use when the agent is diagnosing LIN bus sub-network faults, single-wire serial data communication, local interconnect network module or sensor faults, master-slave LIN topology issues, wakeup or sleep current faults, or evaluating LIN signal integrity, checksum errors, and module sleep behavior on vehicle networks.
---

# Serial Data and LIN Bus Diagnosis

The LIN (Local Interconnect Network) bus is the low-cost, single-wire serial network that connects sub-networks of sensors and actuators — a master module talking to one or more slave devices over a single wire — and it is the network that carries the window, wiper, seat, door, climate, and lighting commands that a customer notices every day. The judgment problem is that a LIN fault (a window that does not work, a wiper that stays parked, a seat that will not adjust, a climate flap that does not move) looks like a component failure, but the component is a slave on a LIN bus whose master or whose wire is the real fault, and because the LIN bus is single-wire, a single short, open, or corroded splice takes down every slave on that branch. A technician who replaces a window motor for a LIN wire fault, or a wiper motor for a master-module fault, hands back a vehicle with the same dead function. This skill covers the disciplined diagnosis of LIN bus and single-wire serial data faults.

## Core Rules

### Understand the LIN Topology: One Master, One Wire, and the Slaves That Depend on It

The disciplined LIN diagnosis starts with the topology: a LIN bus has exactly one master (the module that initiates communication and supplies the bus pull-up) and one or more slaves (the sensors or actuators), all connected on a single wire. The master polls each slave by its address, the slave responds, and the cycle repeats. This means a fault on the single wire or in the master disables every slave on that bus, while a fault in one slave may or may not disable the others (a shorted slave pulls the bus down for all; an open slave simply does not respond). The disciplined technician reads the OEM wiring diagram to identify the master, the slaves, and the wire path (and any splices), because the pattern of which slaves work and which do not localizes the fault to the master, the wire, or a specific slave. The tradeoff is that the topology must be read from the diagram, but diagnosing a LIN bus without the map is guessing which wire is the fault.

### Read the LIN Signal on a Scope: The Square Wave and the Pull-Up

The LIN bus is a single-wire, single-ended signal, and the disciplined scope capture shows its health. A healthy LIN signal is a square wave swinging between ground (the dominant bit, near 0 V, driven by the transmitting node) and battery voltage (the recessive bit, near 12 V, pulled up by the master's resistor). The disciplined interpretation: a bus stuck low (near 0 V continuously) indicates a shorted slave or a short to ground; a bus stuck high (near 12 V continuously) indicates an open wire or a dead master (no dominant bits); a bus with reduced amplitude or rounded edges indicates a high-resistance connection or a loaded bus; a bus with a normal-looking waveform but no communication indicates a master that is not polling or a slave that is not responding (a protocol fault, read with a scan tool). The tradeoff is that the scope shows the electrical health but not the protocol content, so the disciplined technician pairs the scope with the scan tool's LIN diagnostic (which reports which slaves respond and which do not).

### Use the Pattern of Working and Non-Working Slaves to Localize the Fault

The most powerful LIN diagnostic is the pattern of which functions work and which do not, because the topology maps the fault. If no slave on a LIN bus works, the fault is the master (not polling, no power, no ground), the master's pull-up, or the main wire (open or shorted before the first splice). If some slaves work and some do not, the fault is in the branch or the slave that does not respond (an open to that slave, a shorted slave pulling down its branch, or a failed slave). The disciplined technician tests each function on the bus (each window, each wiper mode, each seat direction), maps the results to the topology, and identifies the common point of failure. The tradeoff is that this functional testing takes time, but it localizes the fault before any wire is cut.

### Verify Power, Ground, and the Master Before Condemning a Slave

A slave that "does not respond" is often not failed — it is missing power or ground, or its master is not polling. The disciplined diagnosis verifies the slave's power and ground at its connector (under load, with a voltage drop test), verifies the master is powered and communicating (the scan tool reaches the master, the master reports its LIN status), and verifies the LIN wire continuity from the master to the slave before condemning the slave. A common fault is a master that has lost power or ground (a blown fuse, a corroded ground) and is not polling any slave, which looks like every slave failed. The tradeoff is that verifying the master and the feeds takes minutes, but condemning a slave for a dead master is a costly and avoidable error.

### Diagnose Sleep and Wakeup Faults That Cause Battery Drain

LIN buses go to sleep when the vehicle is off to save power, and a slave that does not sleep (a window motor that stays awake, a climate flap that keeps polling) causes a parasitic battery drain. The disciplined sleep diagnosis uses a current draw test (the vehicle asleep, the draw measured, then LIN buses woken one at a time by touching a function), and a scope on the LIN bus (a bus that keeps toggling when the vehicle should be asleep indicates a slave or a master that does not sleep). The tradeoff is that sleep faults are intermittent and time-dependent, but a scope on the bus during the sleep transition reveals the node that stays awake.

## Common Traps

### Replacing a Slave (Window Motor, Wiper Motor) for a LIN Wire Fault — The window does not work, the technician replaces the motor, and it still does not work because the LIN wire to the motor is open or shorted. The trap mechanism is that the slave is the visible component and the wire is hidden, and a single-wire bus fault disables the slave. The false signal is the function not working pointing at the actuator; the harm is an unnecessary motor. The disciplined technician scopes the LIN signal and verifies the wire before the slave.

### Condemning Every Slave for a Dead Master — Multiple LIN slaves stop working, the technician replaces them one by one, and none fix the fault because the master (the door module, the BCM) has lost power or ground and is not polling. The trap mechanism is that a dead master disables every slave, and the slaves look failed. The false signal is the slaves not responding; the harm is multiple unnecessary replacements. The disciplined technician verifies the master's power, ground, and communication before the slaves.

### Treating a Shorted Slave as a Total Bus Failure — The entire LIN bus goes down, the technician assumes a wire fault, and the cause is a single shorted slave pulling the bus low. The trap mechanism is that one shorted slave grounds the single wire and disables every other slave. The false signal is the bus-wide failure; the harm is a misdirected wire search. The disciplined technician isolates the shorted slave by disconnecting slaves one at a time and rechecking the bus.

### Ignoring a Corroded Splice and Condemning the Wire Run — A LIN branch stops working, the technician replaces the wire run, and the fault persists because a splice in the harness corroded. The trap mechanism is that LIN buses use splices to fan out to multiple slaves, and a corroded splice opens the branch. The false signal is the branch not communicating pointing at the wire; the harm is a replaced wire that did not address the splice. The disciplined technician locates and inspects the splices from the diagram.

### Diagnosing a Parasitic Drain Without Checking the LIN Sleep State — The battery drains overnight, the technician tests the alternator and the battery, and the real cause is a LIN slave that never sleeps. The trap mechanism is that LIN nodes must sleep to stop drawing current, and a non-sleeping node drains the battery. The false signal is the "good battery"; the harm is a recurring drain. The disciplined technician scopes the LIN bus during the sleep transition.

## Self-Check

- Did I read the OEM wiring diagram to identify the LIN master, the slaves, the single wire, and any splices before diagnosing?
- Did I scope the LIN signal (square wave, 0 to 12 V) and interpret a stuck-low, stuck-high, or reduced-amplitude bus?
- Did I use the pattern of working and non-working slaves to localize the fault to the master, the wire, a splice, or a slave?
- For a non-responding slave, did I verify the slave's power and ground under load and the master's power, ground, and communication before the slave?
- For a bus-wide failure, did I isolate a shorted slave by disconnecting slaves one at a time?
- Did I locate and inspect any splices in the LIN branch from the diagram?
- For a parasitic drain, did I scope the LIN bus during the sleep transition to find a non-sleeping node?
- Did I verify the repair by confirming all LIN functions work, no communication codes remain, and the bus sleeps correctly when the vehicle is off?
