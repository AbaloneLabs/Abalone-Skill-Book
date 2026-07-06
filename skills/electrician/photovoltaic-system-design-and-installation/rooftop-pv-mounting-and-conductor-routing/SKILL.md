---
name: rooftop-pv-mounting-and-conductor-routing.md
description: Use when the agent is planning rooftop PV module mounting, routing DC conductors from arrays to combiner boxes, and specifying rapid shutdown, covering ballasted versus attached mounting, conductor routing in sunlight, UV-rated cable, rapid shutdown per NEC 690.12, and roof penetrations and flashing.
---

# Rooftop PV Mounting and Conductor Routing

A rooftop PV array is a structure, an electrical system, and a weatherproofing problem all at once, sitting fully exposed to sun, wind, rain, ice, and UV for decades. The judgment problem is that mounting and conductor routing are often treated as a layout exercise of fitting modules onto a roof, which ignores the structural and weatherproofing consequences of how the array is held down, the thermal and UV exposure of the conductors that run under and across it, and the life-safety requirement that the DC energy be controllable from outside the array. When these are missed, roofs leak, conductors overheat or embrittle, and first responders cannot safely de-energize the array. This skill covers the mounting methods, the conductor routing and material rules, the rapid shutdown requirement, and the penetration and flashing practices that keep a rooftop PV system sound for its service life.

## Core Rules

### Choose Ballasted Versus Attached Mounting by Structural Capacity and Wind Exposure

Rooftop mounting is either ballasted, where the array is held down by weight, or attached, where the racking is mechanically fastened through the roof to the structure. The choice is driven by the roof's structural capacity, the wind exposure category, the roof membrane type, and the building's seismic design category. A ballasted system avoids penetrations but adds significant dead load that the roof must be able to carry, and it can shift or lift under wind uplift if the ballast is insufficient. An attached system transfers loads directly to the structure but requires penetrations that must be flashed and maintained. The structural analysis, signed where required, must confirm the roof can carry the chosen system under dead, wind, snow, and seismic loads, and the attachment or ballast layout must follow the engineering, not the installer's preference.

### Route Conductors Out of Direct Sunlight and Away From Heat

PV conductors on a rooftop operate in a severe thermal environment: ambient heat, radiated heat from the roof surface and the modules, and direct sunlight. Ampacity must be derated for the rooftop ambient and for conduit fill, and conductors should be routed under the modules or in shaded raceway where possible to reduce temperature. Running exposed cables across the top of modules or across open roof in full sun raises the conductor temperature, reduces ampacity, and accelerates insulation aging. The routing should follow the racking, stay under the array shadow, and avoid long exposed runs. Temperature derating is not optional; it is the difference between a conductor that carries its current safely and one that overheats over years.

### Use Only PV-Listed, UV-Rated Cable and Listed Components in the Sun

Any conductor or component exposed to sunlight must be listed for wet locations and sunlight-resistant, which for PV source circuits means USE-2 or PV Wire listed for the purpose. Using THHN or other indoor-rated insulation in sunlight causes the insulation to embrittle, crack, and fail over a few years, creating ground faults and fire risk. Connectors must be the manufacturer's listed type, mateable only with their own kind, and not mixed between brands, because mismatched MC-style connectors overheat and arc. The material specification is enforced at procurement and verified at inspection, because the failure mode of unlisted materials is slow, hidden, and dangerous.

### Implement Rapid Shutdown Per NEC 690.12 With Controlled Boundaries

NEC 690.12 requires that conductors outside the array boundary be de-energized to safe levels within a set time and distance, so that first responders can work on or near the building without exposure to live PV voltage. This is achieved by module-level power electronics (optimizers or microinverters), by string-level rapid shutdown devices, or by inverter-controlled shutdown, each with defined boundaries and a clearly marked initiation means. The controlled boundary must be understood and documented: where the array ends and the controlled conductor begins, where the shutdown device sits, and how the system is marked. A system without a functioning, marked rapid shutdown violates code and endangers responders, and the requirement has tightened across code cycles, so the applicable edition must be confirmed.

### Flash and Seal Every Penetration for the Roof's Service Life

Every roof penetration for an attached mount, a conduit, or a junction box is a potential leak, and it must be flashed and sealed to last as long as the roof, not just to pass the day of inspection. This means using the roofing manufacturer's approved flashing method, compatible sealants, and where possible, avoiding penetrations entirely by routing under the array and exiting at the perimeter. Penetrations through membrane roofs are flashed with the membrane system; through metal roofs, with purpose-made flashings that accommodate thermal movement. A penetration sealed with incompatible or short-life sealant will leak within years, and the leak is hidden under the array where it rots the deck before it is seen. Penetrations are minimized, flashed to the roof system, and documented.

### Provide Clear Access, Pathways, and Setbacks for Safety and Maintenance

A rooftop array must leave documented pathways and setbacks for fire service access, for ventilation of roof-mounted equipment, and for maintenance. Codes and local fire ordinances specify setback distances from roof edges, ridges, hips, and valleys, and clear paths around skylights and HVAC. These are not optional layout conveniences; they are life-safety requirements that affect whether a fire can be fought. The layout must be checked against the applicable fire code, not just fitted to the available roof area, and access hatches and fall protection must be considered for the people who will maintain the system.

### Secure and Protect DC Home Runs and Combiner Locations

The DC conductors from the array to the combiner or inverter, the home runs, must be secured at intervals, protected from physical damage, and routed in listed raceway where exposed. Combiner boxes and junction boxes must be accessible, weatherproof, and located where they can be serviced and where their overcurrent and disconnect functions are reachable. Leaving home runs loose across the roof or tucking combiners into inaccessible corners creates both code violations and maintenance hazards. The routing and box placement are planned, not improvised.

## Common Traps

### Ballasted Array on an Overloaded Roof

The installer places a ballasted array without confirming the roof's live and dead load capacity, assuming the structure can take it. The mechanism is that a ballasted array adds significant point and distributed load, and an undersized or aged roof deflects or fails under the combined dead load, snow, and wind. The false signal is that the array sits level on day one. The harm is structural distress, membrane damage, and in the worst case partial roof collapse under snow or wind events.

### Exposed Cable in Full Sun Embrittles

The installer runs PV source cables across open roof and over module tops in full sun using cable that is not sunlight-rated, or runs listed cable in a way that bakes it. The mechanism is that UV and heat embrittle the insulation over a few years, leading to cracking, ground faults, and arc risk. The false signal is that the insulation looks intact at commissioning. The harm is latent failures years later, hidden under the array, that create fire and shock risk and are expensive to find.

### Mixed Connector Brands Overheat

The installer mates connectors from different manufacturers, or cuts off a connector and field-installs a different brand, because they look compatible. The mechanism is that MC-style connectors are listed only as mating pairs of the same brand, and mismatched contacts have different geometries and spring forces that run hot and arc. The false signal is that the connection passes a continuity test. The harm is overheating, melting, and arc faults at the connector, a leading cause of PV fires.

### Rapid Shutdown Boundary Undefined or Unmarked

The installer installs rapid shutdown devices but does not define or mark the controlled boundary or the initiation means. The mechanism is that responders cannot tell where the live conductors end or how to trigger shutdown, so the life-safety function is defeated by ambiguity. The false signal is that the devices are present and tested. The harm is a system that meets the letter on paper but fails the responder in an actual event, with potential legal and safety consequences.

### Incompatible Sealant on a Membrane Penetration

The installer flashes a conduit penetration with a generic sealant incompatible with the roof membrane. The mechanism is that incompatible materials do not bond or flex together, so the joint opens with thermal cycling and water tracks under the membrane. The false signal is that the penetration passes a hose test at handoff. The harm is a hidden leak under the array that rots the deck and saturates insulation before it is seen indoors.

### No Setbacks or Pathways for Fire Access

The installer fills the roof edge to edge with modules to maximize capacity, ignoring fire code setbacks and pathways. The mechanism is that the array blocks fire service access to the roof, to skylights, and to rooftop equipment, so a fire cannot be fought from above. The false signal is that the layout fits the roof and produces more. The harm is a code violation that fails inspection and, worse, a building where a roof fire cannot be controlled.

## Self-Check

- Did I choose ballasted versus attached mounting based on a structural analysis confirming the roof can carry the dead, wind, snow, and seismic loads, and follow the engineering in the layout?
- Did I route conductors under modules or in shade where possible, derate ampacity for rooftop ambient and conduit fill, and avoid long exposed runs in full sun?
- Did I specify only PV-listed, wet-location, sunlight-resistant cable (USE-2 or PV Wire) for source circuits, and use a single manufacturer's listed connectors throughout with no brand mixing?
- Did I implement rapid shutdown per the applicable NEC 690.12 edition, define and mark the controlled boundary, and provide a clearly marked initiation means?
- Did I flash and seal every penetration using the roofing manufacturer's approved method and compatible materials, and minimize penetrations by routing under the array where possible?
- Did I lay out the array with code-required setbacks, pathways, and access around roof edges, ridges, skylights, and equipment, and provide safe access and fall protection for maintenance?
- Did I secure DC home runs at intervals, protect them from physical damage, route exposed runs in listed raceway, and place combiner and junction boxes in accessible, weatherproof locations?
- Does the output stay within the agent's scope, deferring licensed structural engineering, stamped design, AHJ approval, and roofing system sign-off to the qualified person where the question exceeds the agent's competence?
