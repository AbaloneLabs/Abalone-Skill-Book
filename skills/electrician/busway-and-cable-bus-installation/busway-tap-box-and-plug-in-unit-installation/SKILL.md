---
name: busway-tap-box-and-plug-in-unit-installation.md
description: Use when the agent is installing busway tap boxes and plug-in units, connecting branch loads through plug-in breakers or fused disconnects, sizing tap boxes, grounding at tap points, and routing load-side conductors safely.
---

# Busway Tap Box and Plug-In Unit Installation

Plug-in units and tap boxes are where busway stops being a sealed power highway and becomes a set of exposed tap points that workers interact with repeatedly. The judgment problem is that each tap is a manual connection to an energized bus, made by a person, and the safety of the entire system depends on the tap hardware, the grounding, the load-side protection, and the routing being correct at every single tap. Agents tend to treat a plug-in unit as a convenient receptacle, when it is in fact a fault-current connection point that must be rated, grounded, protected, and routed with the same discipline as a switchboard termination. A tap that is under-rated, ungrounded, or feeds an unprotected conductor is a latent arc-flash and fire hazard that can sit unnoticed until the tap is loaded or faulted.

## Core Rules

### Select Plug-In Units Rated for the Busway and the Available Fault Current
Every plug-in unit (breaker, fused disconnect, or plain tap box) must be rated for the busway's voltage, continuous current, and short-circuit withstand at the tap location. The available fault current at a tap near the source end of the busway can be nearly as high as at the busway main, so the plug-in device's interrupting rating must equal that value. A common error is to select a standard breaker that meets the load current but not the fault current; under fault, the device ruptures and propagates the arc. Verify the interrupting rating of every plug-in device against the calculated fault current at its position on the run.

### Use the Correct Plug-In Mechanism and Never Defeat the Shutter
Plug-in units engage the busway bars through spring fingers or a knife-style stab that mates with the busway opening, and the busway opening is normally covered by a shutter that opens only when the correct plug-in unit is inserted. Use only the manufacturer's listed plug-in units for that busway; do not modify the stab, shim it, or force an incompatible unit. Never prop or defeat the shutter to insert a non-listed device or to inspect the bus, because the shutter is the barrier that prevents accidental contact with the energized bus when no unit is installed. A defeated shutter turns every unused opening into an exposed live-part hazard.

### Ground the Tap Unit to the Busway Enclosure and the Equipment Ground
Every plug-in unit and tap box must bond to the busway enclosure and to the equipment ground conductor so that a ground fault in the load-side circuit has a low-impedance return path that operates the protection. The bond is typically through the plug-in stab frame and through a dedicated equipment ground conductor in the load-side cable. Verify the bond continuity, because a tap unit that relies on a painted or oxidized enclosure contact alone may have a high-impedance ground path that fails to clear a fault. Do not rely on the busway casing as the sole ground path for the load-side fault; provide the equipment ground conductor.

### Protect the Load-Side Conductor at the Tap
The conductor leaving the tap box must be protected against short circuit and overload by the tap device (breaker or fuse) sized to the conductor ampacity, and the conductor must be rated for the available fault current for the time it takes the device to clear. Tap conductors are subject to NEC Article 240.21 tap rules, which permit short tap conductors under specific conditions but require that the tap protection be coordinated with the conductor and the upstream protection. Do not run an unprotected conductor from the tap to a remote panel; the protection must be at the tap point, because an unprotected tap conductor that faults will not clear until the busway main trips, taking the whole run offline.

### Size the Tap Box for the Conductor and the Heat Dissipation
The tap box must be large enough to land the load-side conductors with the required bending space, to dissipate the heat from the tap device, and to allow the door to close over the conductors without pinching insulation. An undersized tap box forces tight bends that damage insulation, crowds the device so it overheats, and makes the terminations inaccessible for torque verification or thermography. Size the tap box to the conductor size and the device rating, and verify that the conductor entry is compatible with the cable or raceway method, because retrofitting a too-small tap box is difficult and often requires replacing the unit.

### Route Load-Side Conductors With Support and Protection
The conductors leaving the tap box must be supported within the required distance of the box, routed in approved raceway or cable method, and protected from physical damage. Do not let the conductors hang from the tap box lugs, because the weight works the termination loose and the unsupported span is a code violation and a damage risk. Use the correct cable or raceway for the environment, transition to conduit or tray at the box, and seal the entry to maintain the enclosure rating. Route phase conductors together and avoid sharp bends that exceed the minimum bending radius for the conductor size.

### Apply the Tap Rules and Verify Coordination With Upstream Protection
Where the tap conductor is smaller than the busway ampacity, the NEC 240.21 tap rules apply, and the installation must meet the conditions of the specific tap rule used (such as the 10-foot or 25-foot tap rules for feeder taps). Verify that the tap conductor length, ampacity, and protection meet the rule, that the upstream busway protection will clear a fault at the tap within the conductor's short-circuit withstand, and that the tap device coordinates so that a load-side fault clears at the tap rather than at the busway main. A tap that violates the tap rules is both a code violation and a real hazard, because the conductor is not adequately protected.

## Common Traps

### Selecting a Plug-In Breaker by Load Current Only
The mechanism is that the plug-in breaker is chosen to match the load current without checking the interrupting rating. The false signal is that the breaker carries the load correctly. The harm is that under a fault the breaker's interrupting capacity is exceeded, it ruptures instead of clearing, and the arc propagates into the busway, because the available fault current at a tap near the source is nearly as high as at the main.

### Defeating the Busway Shutter to Insert a Non-Listed Device
The mechanism is that a non-listed device or a modified stab is forced into a busway opening with the shutter propped open. The false signal is that the device makes contact and the load runs. The harm is that the shutter barrier is disabled, leaving an exposed energized bus opening that endangers anyone near the busway, and the non-listed stab may overheat or fail to engage properly, creating both a shock and an arc hazard at the tap.

### Relying on the Enclosure Alone for Tap Grounding
The mechanism is that the tap unit ground path is assumed through the painted busway casing contact without a dedicated equipment ground conductor. The false signal is that the unit is bolted to the busway and therefore grounded. The harm is a high-impedance ground path through paint and oxidation that cannot carry fault current, so a load-side ground fault does not clear, the casing energizes, and the fault persists until the main trips or a person becomes the path.

### Running an Unprotected Tap Conductor to a Remote Panel
The mechanism is that a conductor leaves the tap box without a local breaker or fuse and runs to a remote panel for protection. The false signal is that the remote panel protects the circuit. The harm is that a fault on the unprotected tap conductor is not cleared by the remote panel; it must be cleared by the busway main, which takes the entire run offline and lets the fault persist for longer, because the protection is not at the tap point as the tap rules require.

### Undersizing the Tap Box and Crowding the Conductors
The mechanism is that a small tap box is used to save cost or space, forcing large conductors into tight bends and crowding the device. The false signal is that the conductors landed and the unit closed. The harm is insulation damage at the bends, device overheating from lack of air, and terminations that cannot be inspected or re-torqued, leading to premature failure at the tap, which is among the most common and most damaging busway failure points.

## Self-Check

- Is every plug-in unit rated for the busway voltage, continuous current, and the available fault current at its tap location?
- Are only manufacturer-listed plug-in units used, with shutters intact and no defeated or propped-open openings?
- Is the tap unit bonded to the busway enclosure and equipped with a dedicated equipment ground conductor in the load-side circuit?
- Is the load-side conductor protected at the tap by a breaker or fuse sized to the conductor ampacity?
- Does the tap installation comply with the applicable NEC 240.21 tap rule for conductor length, ampacity, and coordination?
- Is the tap box sized for the conductor bending space, heat dissipation, and access for torque and thermography?
- Are load-side conductors supported at the box, routed in an approved method, and protected from physical damage?
- Is the tap device coordinated so that a load-side fault clears at the tap rather than at the busway main?
