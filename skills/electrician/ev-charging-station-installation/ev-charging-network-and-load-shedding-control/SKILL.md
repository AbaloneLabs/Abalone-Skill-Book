---
name: ev-charging-network-and-load-shedding-control.md
description: Use when the agent is networking EV chargers for managed charging, wiring network controllers and communication gateways, implementing load shedding and demand response, integrating chargers with a building energy management system, or configuring OCPP and peak shaving with battery storage.
---

# EV Charging Network and Load Shedding Control

A single EV charger is an electrical appliance; a network of EV chargers is a control system. The judgment problem is that the value of managed charging, demand response, and peak shaving depends entirely on the reliability of the communication path between the chargers, the network controller, and the building energy management system (BEMS), and that path is often the weakest link in the installation. An electrician who pulls the power circuits correctly but treats the network as "the IT guy's problem" will deliver a system where the load management cap fails silently, the demand response signal never arrives, and the chargers run unmanaged exactly when management matters most. Worse, the failure modes are invisible: the chargers still charge, the lights on the controller still blink, and the owner only discovers the problem when the demand bill arrives or the feeder trips. This skill covers the networking topology, the OCPP protocol and its implications, the load shedding and demand response integration, and the commissioning that proves the control system actually works under every failure mode.

## Core Rules

### Design the Communication Topology for Reliability, Not Just Connectivity

Managed charging depends on a communication path from each charger to a central controller (the charging station management system, or CSMS), and that path can be Ethernet, Wi-Fi, cellular, or power line carrier. The topology must be designed for the reliability the load management cap requires: if the feeder is sized assuming the controller caps aggregate demand, then a controller that loses its connection and fails open defeats the sizing. The trap is choosing the cheapest communication medium (often Wi-Fi or cellular) without considering that a lost connection means unmanaged full-power charging on an undersized feeder. The defense is to prefer wired Ethernet or fiber for chargers whose load management is structurally relied upon, to provide redundant paths for critical sites, and to specify a fail-safe behavior (cap to a safe default, not release to full power) when the controller connection is lost.

### Understand OCPP and Its Impact on Interoperability and Security

The Open Charge Point Protocol (OCPP) is the open standard for communication between chargers and a management system, and version matters: OCPP 1.6 is widely deployed, OCPP 2.0.1 adds improved security and device management, and a charger and CSMS must speak the same version and profile to interoperate. OCPP also carries security implications, because a charger that accepts remote commands is a networked control device that must be authenticated, encrypted, and segmented from the corporate network. The trap is mixing charger and CSMS vendors assuming OCPP guarantees compatibility, then discovering the profiles or optional features differ, or deploying OCPP over an unencrypted connection that exposes the chargers to unauthorized command. The defense is to verify OCPP version and profile compatibility before procurement, to require OCPP security profiles (TLS, certificate authentication) for any internet-connected deployment, and to segment the charger network from other building systems.

### Wire the Network Controller and Gateway With the Same Discipline as Power

The network controller, cellular gateway, or local controller that commands the chargers is a piece of electrical equipment that must be powered, protected, and wired to the same standard as the rest of the installation. It needs a dedicated, protected circuit, a grounded enclosure, surge protection on its communication ports, and a physical installation that survives the parking environment (moisture, temperature, vibration). The trap is mounting the controller in an unsecured, unpowered, or unprotected location, so it loses power, loses its connection, or fails on a surge. The defense is to treat the controller as a critical electrical device, provide it a dedicated circuit and surge protection, mount it in a listed NEMA-rated enclosure appropriate to the environment, and document its address and configuration.

### Implement Load Shedding With a Defined Priority and Fail-Safe Behavior

Load shedding reduces or cuts charger power in response to a building demand signal, a utility demand response event, or a feeder overload, and it must do so in a defined priority order with a known fail-safe. Priority matters because shedding a charger serving an emergency vehicle is different from shedding a charger in a public lot. Fail-safe matters because if the load shed signal is lost, the system must either hold the last shed state (safe for the feeder) or revert to a configured default, never release all chargers to full power. The trap is configuring load shedding without a priority scheme or a fail-safe, so a lost signal releases all chargers onto an overloaded feeder. The defense is to define the shed priority per charger, to configure the fail-safe to hold or cap rather than release, and to test the behavior by physically disconnecting the control signal during commissioning.

### Integrate Demand Response and Peak Shaving With the Building Energy Management System

Demand response and peak shaving connect the charger network to the building's broader energy picture: the BEMS or a utility program signals a demand event, the charger controller reduces aggregate charging power, and the building avoids a demand peak or earns a demand response credit. On sites with battery storage, the battery can discharge to shave the peak while chargers continue at full power. The integration requires the charger controller to accept an external power limit signal (often via Modbus, BACnet, or a REST API) and to act on it reliably. The trap is installing chargers and a BEMS that nominally support integration but never testing the end-to-end signal path, so the first real demand event reveals a mapping error or a timeout. The defense is to define the integration protocol and points list during design, wire and configure the interface, and commission it by simulating a demand event and verifying the charger aggregate responds.

### Verify Charger-to-BMS Communication and the Aggregate Cap End to End

The single most important commissioning step for a managed charging system is to prove that the aggregate power cap actually holds under real conditions. This means connecting all chargers, commanding them to charge, raising the cap, lowering the cap, and measuring the aggregate response with a power meter. The trap is commissioning only the individual chargers and assuming the management layer works, then discovering at the first peak that the cap does not hold because of a configuration error or a communication timeout. The defense is to commission the system as a system, measure the aggregate power with an independent meter while varying the cap, and document the response time and accuracy of the management function.

### Document the Control Architecture and the Failure Modes

A managed charging system's architecture, points lists, fail-safe behaviors, and commissioning results must be documented so that the next electrician, the BEMS integrator, and the owner can understand and maintain it. The trap is delivering a working system with no documentation, so a future configuration change breaks the load management cap without anyone realizing it. The defense is to produce a control architecture diagram, a points list for each integration, a fail-safe behavior table, and a commissioning report that records the measured aggregate response.

## Common Traps

### Choosing Cellular or Wi-Fi Without Considering Fail-Open Behavior

The installer uses cellular modems for charger communication because the lot has no wired infrastructure, and the feeder is sized assuming load management caps the aggregate. The mechanism of the failure is that cellular connections drop, and when the connection is lost the chargers revert to their default behavior, which on many units is full power, overloading the feeder that was sized to the managed load. The false signal is that the chargers communicate during commissioning, which proves the medium works in good conditions but not under connection loss. The harm is a feeder overload precisely when management is lost, with tripping or fire risk. The defense is to prefer wired communication where load management is structurally relied upon, or to configure chargers to fail to a safe capped default.

### Assuming OCPP Guarantees Interoperability Across Vendors

The owner buys chargers from one vendor and a management system from another, both claiming OCPP compliance, and assumes they will interoperate. The mechanism of the failure is that OCPP has versions (1.6, 2.0.1) and profiles, and vendors implement optional features differently, so the charger connects but certain commands (like a remote power limit) are not supported or behave differently. The false signal is that the charger appears online in the management system, which proves basic connectivity but not full functional compatibility. The harm is a system that monitors but cannot control, defeating load management and demand response. The defense is to verify OCPP version, profile, and specific feature support before procurement and to test the critical commands during commissioning.

### Exposing the Charger Network Without Security Profiles

The chargers are connected to the internet over plain OCPP with no TLS and no certificate authentication, because that was the default. The mechanism of the failure is that an unauthenticated networked control device can be commanded by anyone who reaches it, and chargers have been observed accepting remote start and configuration changes without authentication. The false signal is that the system works and is convenient, which proves functionality but ignores the attack surface. The harm is unauthorized control of charging infrastructure, potential feeder overload via malicious command, and a compromised device on the building network. The defense is to require OCPP security profiles with TLS and certificate authentication and to segment the charger network from other building systems.

### Mounting the Controller in an Unprotected Location

The network controller is zip-tied inside a charger pedestal or mounted in an unsealed box, powered from a shared convenience receptacle. The mechanism of the failure is that the controller loses power when the receptacle is switched off, or it fails on a surge, or moisture corrodes the connections, and the load management cap is lost. The false signal is that the controller is installed and lit up, which proves presence but not robustness. The harm is intermittent loss of management that is hard to diagnose and that defeats the feeder sizing at random times. The defense is to mount the controller in a listed NEMA-rated enclosure on a dedicated protected circuit with surge protection.

### Configuring Load Shedding Without a Fail-Safe Default

Load shedding is configured to cut charger power on a building demand signal, but no fail-safe is defined for when the signal is lost. The mechanism of the failure is that a lost signal is interpreted as no-shed, so all chargers return to full power during the exact period the feeder is most constrained. The false signal is that the shed works when tested with a healthy signal, which proves the command path but not the loss-of-signal behavior. The harm is a feeder overload or demand spike when the control signal fails. The defense is to define the fail-safe to hold the last shed state or revert to a safe capped default and to test the behavior by disconnecting the signal.

### Skipping End-to-End Aggregate Cap Verification

The chargers are commissioned individually, the management system shows them online, and the cap is set in software, but no one measures the aggregate power while varying the cap. The mechanism of the failure is that a configuration error, a communication timeout, or a units mismatch means the cap does not actually limit the aggregate, and the error is invisible until the first peak. The false signal is that the management system reports the cap is active, which proves configuration but not measured behavior. The harm is an unmanaged feeder that trips or overloads at the worst time. The defense is to measure the aggregate power with an independent meter while stepping the cap and to document the response.

## Self-Check

- Did I design the communication topology for the reliability the load management cap requires, preferring wired Ethernet or fiber where management is structurally relied upon, and did I specify a fail-safe (cap, not release) on connection loss?
- Did I verify OCPP version and profile compatibility between the chargers and the management system before procurement, and did I test the critical remote commands (power limit, remote start) during commissioning?
- Did I require OCPP security profiles (TLS, certificate authentication) for any internet-connected deployment, and did I segment the charger network from other building systems?
- Did I mount the network controller in a listed NEMA-rated enclosure on a dedicated protected circuit with surge protection, and did I document its address and configuration?
- Did I define the load shedding priority per charger and the fail-safe behavior (hold or cap, not release), and did I test the behavior by physically disconnecting the control signal?
- Did I define the demand response and peak shaving integration protocol and points list, wire and configure the BEMS interface, and commission it by simulating a demand event and verifying the aggregate charger response?
- Did I measure the aggregate charger power with an independent meter while stepping the cap, and did I document the response time and accuracy of the management function?
- Is the control architecture, points lists, fail-safe behaviors, and commissioning results documented so another practitioner can maintain the system?
