---
name: intercom-and-entry-system-wiring.md
description: Use when the agent is installing audio or video intercoms, wiring multi-tenant apartment entry and directory systems, running shielded intercom cabling and risers, or integrating door stations with access control release relays and camera tie-ins.
---

# Intercom and Entry System Wiring

An intercom is one of the few low-voltage systems that an occupant touches every day, and it is also one of the most cross-connected systems on a door, tying together voice, video, directory, access release, and sometimes fire alarm and emergency call. The judgment problem is that intercom wiring looks like simple two-wire audio, which invites electricians to use unshielded cable, to bundle runs with other low-voltage systems, and to treat the door station as a standalone device rather than as the convergence point of several subsystems. The result is hum, crosstalk, garbled video, and door releases that do not fire when a tenant buzzes in. This skill covers the decisions that determine whether an intercom is intelligible, reliable, and properly integrated with the access and security systems around it.

## Core Rules

### Select the Intercom Architecture for the Building Type and Tenant Count

Intercom systems fall into broad architectures that suit different building types. A simple two-wire or four-wire audio intercom suits a single door to a single office. A video entry system with one master station and one door station suits a small professional building. A multi-tenant apartment system, with a directory, a door station, and dozens or hundreds of tenant stations, requires a central controller and a structured riser that distributes audio, video, and door-release signals across many floors. The architecture determines the cabling, the power distribution, and the integration points, and choosing the wrong architecture for the tenant count leads to a system that cannot scale or that is wildly overbuilt for a two-door office.

The trap is applying a familiar residential-style intercom to a multi-tenant building. The defense is to size the architecture to the tenant count and door count, to confirm the directory and call capacity, and to plan the riser and power distribution before pulling any cable.

### Use Shielded, Twisted Cable and Respect the Topology for Audio Quality

Audio intercom signals are low-level and vulnerable to hum and crosstalk, especially over the long risers typical of multi-tenant buildings. Shielded twisted pair is the minimum for audio runs, with the shield grounded at one end to prevent ground loops, and video runs require coax or twisted-pair with the specified impedance and adequate shielding. Daisy-chain topologies, where one cable loops through several tenant stations, are cheap but create a single point of failure and accumulate noise; star or homerun topologies to a central controller isolate faults and preserve signal quality. The cable type and topology must follow the manufacturer's specification, because intercom systems are tuned for a specific cable and deviation causes level and impedance problems that manifest as low volume or distortion.

The trap is pulling unshielded thermostat wire for audio because it is on the truck. The defense is to use the manufacturer-specified shielded cable, to homerun where the topology requires it, and to ground shields at one end only to avoid ground loops.

### Plan the Riser for Multi-Tenant Systems Before Pulling Cable

In a multi-tenant apartment or office building, the riser is the backbone that carries audio, video, power, and door-release signals from the central controller to every tenant station, and its design determines whether the system is maintainable. The riser must be sized for the final tenant count, not just the current one, with spare capacity for future stations, and it must follow a planned vertical and horizontal route through protected shafts and rated assemblies. Power for tenant stations may be distributed from the controller or from local power supplies on each floor, and the choice affects conductor sizing and voltage drop. A riser planned for today's count with no spare pairs is a system that cannot grow without a costly re-pull.

The trap is pulling exactly enough pairs for the current tenant count. The defense is to size the riser for the build-out capacity, to route it through protected shafts, and to document the route and spare capacity for future expansion.

### Integrate the Door Release Relay With Access Control, Not Around It

The door station's tenant-release button must operate the door lock, and the cleanest way to do this is through a dry contact into an access control input rather than by driving the lock directly from the intercom. Driving the lock directly bypasses the access control system's scheduling, door-forced monitoring, and fire alarm release, creating an unmonitored release path that can conflict with life-safety requirements. Landing the intercom release as a dry contact into the access controller lets the access system log the release, enforce schedules, and, critically, still honor the fire alarm override. The intercom and access systems should share the door but not share the lock power.

The trap is wiring the intercom release relay straight to the lock to save an interface. The defense is to land the intercom release as a dry contact into the access controller, to let the access system drive the lock, and to verify that fire alarm release still overrides the intercom release.

### Coordinate the Camera Tie-In With the Surveillance and Video Entry Systems

A video door station may be the only camera covering the entry, or it may duplicate a dedicated surveillance camera, and the relationship between them must be intentional. If the door station feeds a tenant monitor only, then a separate surveillance camera should record the entry for after-the-fact review. If the door station's video is also recorded, the feed must be brought into the NVR or VMS, and the frame rate and retention must meet the same investigative objectives as the dedicated cameras. Treating the door station video as a live-only convenience, with no recording, leaves the most important door in the building unrecorded.

The trap is treating the door station camera as a live monitor with no recording. The defense is to decide explicitly whether the door station video is recorded, to bring the feed into the VMS if it is, and to ensure a dedicated surveillance camera covers the entry if the door station is live-only.

### Provide Clean Power and Grounding for the Door and Master Stations

Intercom door stations and master stations are mixed-signal devices that combine audio, video, and relay outputs, and they are sensitive to power quality and grounding. Power should come from a listed supply matched to the system, not from a shared supply that also runs locks or access controllers, because lock inrush can collapse the voltage and reset the intercom mid-call. Grounding must follow the manufacturer's scheme, typically a single-point ground at the controller, to avoid ground loops that manifest as hum on the audio. Surge protection is essential at exterior door stations, where nearby strikes and ground potential shifts can damage the station and the controller it connects to.

The trap is powering the intercom from whatever supply is handy. The defense is to use a dedicated listed supply for the intercom, to follow the single-point grounding scheme, and to install surge protection at exterior door stations.

### Respect Listings and Environmental Ratings at the Door Station

The door station is installed at the building entrance, exposed to weather, temperature swings, and direct contact with the public, and its listing and rating must match. An exterior door station must carry an appropriate ingress rating for the environment, a vandal-resistant housing where it is reachable, and a listed enclosure when installed in a wet location. Interior door stations in ordinary environments have lighter requirements but still need to match the ambient temperature and humidity. A non-rated door station will fail from moisture infiltration or vandalism, taking the entire entry system offline at the one door that matters most.

The trap is using an interior-rated station at an exterior entrance. The defense is to match the door station's environmental and vandal ratings to the installed location and to use listed enclosures in wet locations.

## Common Traps

### Unshielded Cable on a Long Audio Riser

To save cost, the installer pulls unshielded thermostat wire for the audio riser in a multi-tenant building. The mechanism of the trap is that unshielded audio runs over long vertical distances act as antennas for the building's electrical noise, inducing hum and crosstalk that make conversation difficult, and the lack of twisting also defeats common-mode noise rejection. The false signal is that the system benches out clean on the workbench or on a short run, which never exercises the noise pickup of a real riser. The harm is a system where every call carries a persistent hum and where tenants stop using the intercom because they cannot understand visitors, defeating the system's purpose. The defense is to use the manufacturer-specified shielded twisted pair, to homerun where required, and to ground the shield at one end only.

### Daisy-Chained Tenant Stations Creating a Single Point of Failure

The installer loops one cable through a row of tenant stations to save riser capacity. The mechanism of the trap is that a daisy chain means every station downstream of a fault loses service when one connection fails, and the accumulated impedance of many stations on one run also degrades audio level at the end of the chain. The false signal is that all stations work at handover, when the chain is intact. The harm is a single bad connection that takes out a whole floor of intercom service and a difficult troubleshooting exercise to find the break. The defense is to homerun each station or each small group to the controller, to isolate faults, and to preserve signal level across the system.

### Intercom Release Wired Direct to the Lock, Bypassing Access Control

To avoid an interface, the installer wires the tenant-release relay straight to the electric strike. The mechanism of the trap is that this creates a second, unmonitored release path that bypasses the access controller's scheduling, logging, and fire alarm override, so a fire alarm signal that drops the access controller's lock output may not drop the intercom's parallel release path, leaving the door energized. The false signal is that the door releases when the tenant presses the button, which proves the relay works but not that the release is coordinated with life safety. The harm is a door that can be released by the intercom during a fire alarm, defeating the coordinated release, and an unlogged release path that defeats audit. The defense is to land the intercom release as a dry contact into the access controller and to let the access system drive the lock.

### Door Station Camera Treated as Live-Only With No Recording

The installer commissions the video door station, confirms a clean image on the tenant monitor, and leaves no recording in place. The mechanism of the trap is that the entry door is the highest-risk location in the building, and a live-only video feed provides no after-the-fact evidence of who entered or what occurred, so the one camera that matters most records nothing. The false signal is that the tenant can see the visitor clearly, which proves the live path but not the recording path. The harm is an incident at the entry with no recorded evidence, the precise gap the surveillance system was meant to fill. The defense is to decide explicitly whether the door station video is recorded, to bring it into the VMS if so, and to ensure a dedicated recorded camera covers the entry otherwise.

### Shared Power Supply Collapsing on Lock Inrush

The installer powers the intercom door station and the electric strike from the same supply to save a homerun. The mechanism of the trap is that the strike's inrush, several times its hold current, momentarily collapses the supply voltage, resetting the intercom door station mid-call and often cutting off the tenant's conversation exactly as they press the release. The false signal is that the intercom works when the lock is idle, which is how it is usually tested. The harm is a system that cuts out every time someone is buzzed in, a daily nuisance that tenants notice immediately. The defense is to power the intercom from a dedicated supply and to let the access system power the lock.

### Interior-Rated Door Station at an Exposed Entrance

The installer mounts an interior-rated door station at a building entrance exposed to rain and sun. The mechanism of the trap is that the interior housing lacks the ingress and UV resistance for the environment, so moisture infiltrates the station, corrodes the electronics, and UV degrades the lens and buttons, leading to failure within a season or two. The false signal is that the station works at installation, when the housing is new and dry. The harm is progressive degradation and eventual failure of the entry system at the one door that every visitor uses, plus the cost of replacement at height. The defense is to match the door station's environmental and vandal ratings to the installed location and to use a listed enclosure in wet locations.

## Self-Check

- Did I select the intercom architecture, audio, video, or multi-tenant directory, to match the building type and the final tenant count, with capacity for build-out?
- Did I use the manufacturer-specified shielded twisted pair for audio and the correct cable for video, homerun where the topology requires it, and ground shields at one end only to prevent ground loops?
- Did I size the riser for the build-out tenant count with spare capacity, route it through protected shafts, and document the route and spare pairs for future expansion?
- Did I land the tenant-release button as a dry contact into the access controller rather than driving the lock directly, so scheduling, logging, and fire alarm override remain intact?
- Did I decide explicitly whether the door station video is recorded, bring the feed into the VMS if it is, and ensure a dedicated recorded camera covers the entry if the door station is live-only?
- Did I power the intercom from a dedicated listed supply, follow the manufacturer's single-point grounding scheme, and install surge protection at exterior door stations?
- Did I match the door station's environmental ingress and vandal ratings to the installed location, using listed enclosures in wet locations?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
