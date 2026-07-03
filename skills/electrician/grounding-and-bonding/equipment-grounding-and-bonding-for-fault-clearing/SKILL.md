---
name: equipment-grounding-and-bonding-for-fault-clearing.md
description: Use when the agent is installing equipment grounding conductors, sizing bonding jumpers, ensuring fault current paths, verifying continuity of grounding systems, or troubleshooting breakers that fail to trip during ground faults.
---

# Equipment Grounding and Bonding for Fault Clearing

When a live conductor contacts a metal enclosure — a frayed wire touches the case of a washing machine, a loose terminal arcs to a panel — the enclosure becomes energized at line voltage. The only thing preventing the next person who touches it from being electrocuted is the equipment grounding conductor (EGC), which carries the fault current back to the source, allowing the breaker to trip and remove the voltage. If the EGC is missing, broken, improperly terminated, or too high impedance, the breaker does not trip, the enclosure stays energized, and the fault becomes a latent electrocution hazard. The judgment problem is that the EGC is invisible in normal operation — it carries no current, does nothing apparent, and seems like an unnecessary complication — until the moment a fault occurs, when it is the only thing between a person and death. An electrician who treats grounding as a code formality rather than a life-safety system will install EGCs that exist on paper but fail when needed. This skill covers how to install and maintain the EGC and bonding network so that faults are cleared instantly and reliably.

## Core Rules

### Install the EGC in Every Raceway and Cable Assembly

The equipment grounding conductor must be installed in every raceway and cable assembly, running with the circuit conductors from the source to the furthest outlet. The EGC provides the metallic fault path that allows a breaker to trip when a fault energizes an enclosure. Without it, a fault has no low-impedance path back to the source, and the breaker cannot detect the overcurrent. The EGC may be a wire (bare or green insulated copper or aluminum), the raceway itself (metal conduit approved as an EGC), or a combination. The trap is relying on a raceway as the EGC without verifying its continuity — a loose coupling, a section of nonmetallic conduit, or a corroded fitting can interrupt the path, and the fault will not clear. The defense is to either install a wire-type EGC in every raceway or to verify the continuity of the metallic raceway with testing, recognizing that mechanical damage and corrosion can defeat a raceway-based EGC over time.

### Size the EGC Based on the Overcurrent Protection, Not the Load

The EGC is sized from NEC Table 250.122 based on the rating of the overcurrent protection device protecting the circuit, not on the load current. A 20-amp breaker requires a 12 AWG EGC; a 60-amp breaker requires a 10 AWG EGC; a 100-amp breaker requires an 8 AWG EGC. The sizing is based on the fault current that the EGC must carry — a larger breaker can supply more fault current, and the EGC must be large enough to carry that current without fusing (melting) before the breaker trips. The trap is sizing the EGC based on the actual load (which may be less than the breaker rating) or using a smaller EGC because "it's just a ground." The defense is to always size the EGC from Table 250.122 using the breaker or fuse rating, and to upsize the EGC whenever the circuit conductors are upsized (for voltage drop or continuous loads), per NEC 250.122(B).

### Bond Enclosures and Raceways to Ensure Electrical Continuity

Every metal enclosure — panelboard cabinets, junction boxes, device boxes, motor controllers, luminaires — must be bonded to the EGC so that a fault energizing the enclosure is immediately cleared. Bonding is achieved through the EGC connection at each enclosure, through listed fittings that provide grounding continuity (locknuts, bushings, grounding clips), and through the mechanical connection of the raceway to the enclosure. The trap is relying on a locknut or fitting that is not listed for grounding, or that is loose, corroded, or painted, creating a high-impedance connection that may not carry fault current. The defense is to use listed grounding fittings, to verify that painted or corroded surfaces are cleaned before bonding, and to install bonding locknuts or bushings where required (particularly on concentric or eccentric knockouts, which can create high-impedance connections).

### Upsize the EGC When Upsizing Circuit Conductors for Voltage Drop

NEC 250.122(B) requires that the EGC be upsized proportionally when the ungrounded circuit conductors are upsized for voltage drop. The principle is that the EGC's impedance must remain low relative to the circuit conductors so that fault current is high enough to trip the breaker quickly. If the circuit conductors are upsized (reducing their resistance and impedance) but the EGC is left at its original size, the EGC's impedance becomes a larger fraction of the total fault loop, reducing the fault current and potentially preventing the breaker from tripping within the required time. The trap is upsizing circuit conductors for voltage drop (a common practice on long runs) and leaving the EGC at the table size, which creates a fault loop with inadequate current to trip the breaker. The defense is to upsize the EGC proportionally — if the circuit conductors are doubled in size, the EGC should be approximately doubled as well.

### Verify Continuity of the Grounding System With Testing

The grounding system's effectiveness cannot be determined by visual inspection alone — a broken EGC inside a conduit, a loose connection inside a junction box, or a corroded bond may not be visible. Continuity testing (using a low-impedance tester, not a simple continuity beeper) verifies that the fault path is intact from each enclosure back to the source. The test should measure the impedance of the EGC path, not just the presence of continuity, because a high-impedance path may carry enough current to beep a tester but not enough to trip a breaker. The trap is assuming that because the EGC was installed correctly, it remains intact — vibration, thermal cycling, corrosion, and subsequent modifications can all degrade the path over time. The defense is to test the grounding system at installation and periodically, particularly in industrial environments where vibration and corrosion are prevalent.

### Understand Why Separately Derived Systems Need Their Own Bonding

A separately derived system — a transformer, generator, or UPS that creates a new source of power — requires its own system bonding jumper connecting the neutral to the grounding system at the source. This bonding jumper serves the same function as the main bonding jumper at the service: it provides the fault-clearing path from the EGC on the secondary side back to the source. Without it, a fault on the secondary side has no path back to the transformer, and the breaker on the secondary cannot trip. The trap is treating a separately derived system as an extension of the service grounding, without installing the system bonding jumper at the transformer or generator. The defense is to identify every separately derived system and to install and size the system bonding jumper correctly, ensuring that the secondary side has an independent fault-clearing path.

## Common Traps

### Relying on Conduit as the EGC Without Verifying Continuity

An electrician installs metal conduit and relies on it as the equipment grounding conductor, without installing a wire-type EGC. The conduit appears continuous, but a loose coupling, a section of flexible conduit (which is often not approved as an EGC), or a corroded fitting creates a high-impedance gap. A fault energizes an enclosure downstream of the gap, and the breaker does not trip because the fault path is interrupted. The trap is that the conduit looks continuous and the installation appears correct, but the electrical continuity is not verified. The defense is to either install a wire-type EGC in every raceway (the safest approach) or to perform impedance testing on the metallic raceway system to verify that it can carry fault current. Flexible conduit (greenfield, LFMC) has specific limitations as an EGC and often requires an internal wire-type EGC.

### Leaving the EGC at Table Size After Upsizing Conductors

An electrician upsizes branch-circuit conductors from 12 AWG to 8 AWG to address voltage drop on a long run, but leaves the EGC at 12 AWG (the table size for a 20-amp breaker). The fault loop impedance is now dominated by the EGC, and the fault current may be too low to trip the breaker quickly. The breaker may eventually trip (magnetic trip) or may not trip at all (thermal trip at low current), leaving the enclosure energized for an extended period. The trap is that the upsizing was for voltage drop (a functional concern), and the EGC requirement (a safety concern) was overlooked. The defense is to apply NEC 250.122(B) whenever upsizing conductors, proportionally increasing the EGC to maintain the fault-loop impedance ratio.

### Using Non-Listed Fittings for Grounding

An electrician uses a standard locknut to connect a metal raceway to a panel, relying on the locknut for grounding continuity. The locknut is not listed for grounding, and the connection's impedance depends on the tightness of the locknut, the cleanliness of the threads, and the absence of paint or corrosion. Over time, the connection degrades, and the impedance increases until the fault path is ineffective. The trap is that standard fittings look like they provide grounding, and in many cases they do, but they are not tested or listed for the purpose. The defense is to use listed grounding fittings (grounding locknuts, grounding bushings, bonding bushings) at every enclosure connection, particularly on concentric or eccentric knockouts, which are specifically problematic for grounding continuity.

### Omitting the EGC Because "It's a Plastic Box"

An electrician installs a circuit using nonmetallic boxes and cable, and omits the equipment grounding conductor because "there's no metal to energize." But the devices (receptacles, switches) have metal yokes that can become energized by a fault, and appliances connected to the receptacles have metal cases that rely on the EGC. Without the EGC, a fault in an appliance has no path back to the source, and the breaker does not trip. The trap is focusing on the box (which is nonmetallic and cannot be energized) and forgetting the connected equipment (which is metallic and relies on the EGC). The defense is to install the EGC in every circuit regardless of the box material, connecting it to the grounding terminal of every device and to any metal enclosure or raceway.

### Creating Parallel Neutral Paths Through the Grounding System

An electrician bonds the neutral and ground at a subpanel (downstream of the main service), creating a parallel path for neutral current through the equipment grounding conductor. Neutral current now flows on both the neutral conductor and the EGC, energizing metal enclosures at a small but measurable voltage and creating electromagnetic interference. The trap is that the neutral-ground bond at the subpanel seems harmless (it "can't hurt to have an extra ground"), but it defeats the separation of neutral and grounding that the code requires downstream of the main bonding jumper. The defense is to maintain neutral-ground isolation at all panels and enclosures downstream of the service, connecting the neutral and grounding only at the main service or at separately derived systems.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Is an equipment grounding conductor installed in every raceway and cable assembly, running with the circuit conductors from the source to the furthest outlet?
- Is the EGC sized from NEC Table 250.122 based on the overcurrent protection rating, and if circuit conductors were upsized for voltage drop, was the EGC upsized proportionally per NEC 250.122(B)?
- Are all metal enclosures bonded to the EGC using listed grounding fittings, with painted or corroded surfaces cleaned before bonding?
- If I am relying on metal conduit as the EGC, have I verified its continuity with impedance testing, and have I accounted for sections of flexible or nonmetallic conduit that may interrupt the path?
- Have I tested the grounding system with a low-impedance tester (not a simple continuity beeper) to verify that the fault path can carry sufficient current to trip the breaker?
- For separately derived systems (transformers, generators), is a system bonding jumper installed and sized to provide the secondary-side fault-clearing path?
- Am I maintaining neutral-ground isolation at all subpanels and downstream enclosures, with no parallel neutral paths through the grounding system?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
