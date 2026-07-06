---
name: lightning-grounding-and-potential-equalization.md
description: Use when the agent is bonding a lightning protection system to the building grounding electrode system, equalizing potential to prevent side-flash, calculating separation distance per NFPA 780, and ensuring that a strike does not arc between the LPS and adjacent building metal or wiring.
---

# Lightning Grounding and Potential Equalization

A lightning protection system that captures a strike and routes it down a conductor has solved only half the problem. The other half — and the half that destroys equipment and starts fires — is what happens to the enormous voltage that appears on the down conductor during the strike. If the LPS ground is at tens of thousands of volts while the building's electrical ground, structural steel, and wiring sit near earth, the voltage difference will arc across any gap: through walls, through wiring insulation, through equipment. The judgment problem is that the protection does not come from giving the lightning a path to earth, but from making sure every conductive system in the building rises and falls together so no difference can drive an arc. Agents miss the issues because the down conductor and ground rod look complete, while the invisible potential differences that cause side-flash are never analyzed.

## Core Rules

### Bond the LPS Ground to the Building Grounding Electrode System Per NFPA 780

NFPA 780 requires the lightning protection system's grounding electrodes to be bonded to the building's main grounding electrode system — the electrical service ground, the metallic water pipe, the concrete-encased electrode, and the building steel. The purpose is potential equalization: during a strike, bonding forces the LPS ground and the electrical ground to the same potential so no voltage difference can drive a side-flash between them. The defense is to bond every electrode system together with conductors sized per 780, to treat the bond as the single most important connection in the LPS, and never to isolate the LPS ground in the mistaken belief that isolation keeps lightning out of the electrical system.

### Equalize Potential by Bonding All Major Metallic Systems to a Common Reference

Potential equalization extends beyond the electrodes to every major metallic system in the building: structural steel, metallic piping, metallic ducts, cable trays, and the electrical equipment grounding system. During a strike, the magnetic field from the down conductor induces voltages in all nearby metal, and any metal left at a different potential becomes a side-flash target. The defense is to bond major metallic systems to the common grounding reference at defined points, especially where they enter the building and where they run parallel to down conductors, so that induced and conducted voltages equalize rather than arc.

### Calculate and Maintain the Separation Distance to Prevent Side-Flash

Where a down conductor runs near metallic material that cannot or should not be bonded, NFPA 780 requires a minimum separation distance so the strike voltage cannot arc across the gap. The separation distance depends on the strike current, the surge impedance of the down conductor, and the insulating medium (air, masonry, insulation). The defense is to calculate the required separation distance per 780 for every location where a down conductor parallels unbonded metal, to maintain that distance in the physical installation, or to bond the metal where bonding is the safer option, because a side-flash through the building is the precise damage the LPS exists to prevent.

### Distribute Down Conductors and Ground Electrodes to Reduce Voltage Rise

Concentrating the lightning current into a single down conductor and a single ground electrode produces the highest possible voltage rise at that point. Distributing the current across multiple down conductors spaced around the building, each tied to ground electrodes, splits the current and lowers the voltage at any one point, reducing both the side-flash risk and the ground potential rise. The defense is to use the number of down conductors 780 requires for the building perimeter, to space them evenly, to provide each with an adequate ground electrode, and to interconnect the electrodes into a ring or grid so the ground potential rises uniformly rather than spiking at one point.

### Coordinate the LPS Grounding With the Facility Surge Protection

The bonding between the LPS and the electrical ground means that lightning current entering the LPS will also raise the electrical ground reference, driving surge current into the electrical system through the bond. This is expected and is why surge protective devices must be installed at the service entrance and at sensitive panels: they clamp the surge that the equalization bond intentionally allows into the electrical ground. The defense is to treat LPS bonding and SPD installation as a coordinated system — bond per 780 to prevent side-flash, and install SPDs per Article 285 to clamp the resulting surge — because bonding without SPDs lets the surge roam the wiring, and SPDs without bonding invite side-flash.

### Use a Ground Ring or Counterpoise to Spread the Dissipation

A single ground rod dissipates lightning current into a small volume of soil, producing a high local ground potential rise that can exceed the insulation of nearby cables and arc to them. A ground ring (a buried conductor encircling the building) or a counterpoise spreads the dissipation over a larger area, lowering the potential rise and equalizing the ground potential around the perimeter. The defense is to install a ring or counterpoise per 780 for structures where the soil, the size, or the sensitivity warrants it, to bond each down conductor to the ring, and to verify that the grounding electrode system meets the resistance 780 targets for the installation.

## Common Traps

### Isolating the LPS Ground to "Keep Lightning Out of the Electrical System"

The installer leaves the LPS ground separate from the electrical ground, believing isolation protects the wiring. The false signal is that the systems are separate and therefore safe from each other. The mechanism of failure is that during a strike the LPS ground rises to a huge voltage while the electrical ground stays near earth, and the difference arcs from the LPS into the wiring, structural steel, or piping. The harm is exactly the side-flash, equipment destruction, and fire that bonding per 780 prevents.

### Bonding Only the Electrodes and Ignoring Nearby Metallic Systems

The installer bonds the LPS ground rod to the electrical ground rod and stops there, leaving structural steel, piping, and ducts unbonded. The false signal is that the electrode bond satisfies the requirement. The mechanism of failure is that the down conductor's magnetic field induces voltage in all nearby metal, and any unbonded metal at a different potential becomes a side-flash target. The harm is arcs to building steel, piping, or ducts that the electrode bond never addressed.

### Routing a Down Conductor Tight Against Unbonded Metal With No Separation

The installer runs a down conductor in contact with or inches from a metal downspout or conduit that is not bonded, with no calculated separation. The false signal is that the run is tidy and out of the way. The mechanism of failure is that the strike voltage on the down conductor arcs across the small gap to the parallel metal, sending current into the building envelope. The harm is a side-flash into structure or wiring because the required separation distance was never maintained or the metal never bonded.

### Concentrating All Down Conductors Into One Corner Ground

The installer routes all down conductors to a single ground electrode at one corner to simplify the installation. The false signal is that there is a low-resistance path to earth. The mechanism of failure is that the full strike current concentrates at one point, producing an extreme voltage rise that arcs to nearby metal and raises the ground potential far above the rest of the building. The harm is a high-voltage hot zone at one corner that side-flashes into adjacent structure and wiring.

### Installing SPDs Without Bonding the LPS, or Bonding Without SPDs

The installer puts surge protection on the panels but does not bond the LPS, or bonds the LPS but installs no SPDs. The false signal is that one measure alone addresses the surge. The mechanism of failure is that without bonding, the LPS voltage side-flashes into the wiring regardless of the SPDs, and without SPDs, the bonding that equalizes potential lets surge current roam the electrical system unclamped. The harm is either side-flash destruction or surge propagation, because the two measures must work together.

### Assuming a Single Ground Rod Provides Adequate Dissipation

The installer drives one ground rod for the LPS and assumes it is sufficient because it tests at a low resistance. The false signal is that the resistance reading looks acceptable. The mechanism of failure is that lightning current in a single rod produces a high local potential rise that arcs to nearby cables and structure, regardless of the steady-state resistance. The harm is side-flash and ground-potential damage from a strike that the single rod could not spread out, which a ring or counterpoise would have mitigated.

## Self-Check

- Did I bond the LPS ground electrodes to the building grounding electrode system with conductors sized per NFPA 780?
- Did I equalize potential by bonding all major metallic systems — structural steel, piping, ducts, cable trays — to the common grounding reference?
- Did I calculate and maintain the required separation distance wherever a down conductor parallels unbonded metal, or bond the metal where bonding is safer?
- Did I distribute down conductors and ground electrodes around the building to split current and reduce voltage rise, rather than concentrating at one point?
- Did I coordinate LPS bonding with facility SPD installation so that side-flash is prevented and the resulting surge is clamped?
- Did I install a ground ring or counterpoise where the structure size, soil, or sensitivity warrants it, and bond each down conductor to it?
- Did I verify the grounding electrode system meets the resistance target of 780 for the installation, recognizing that resistance alone does not guarantee safe dissipation?
- Is the bonding and grounding design documented so that future additions of metal or wiring can be checked against the equalization model?
