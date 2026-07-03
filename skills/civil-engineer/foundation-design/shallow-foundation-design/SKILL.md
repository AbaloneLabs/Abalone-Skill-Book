---
name: shallow-foundation-design.md
description: Use when the agent is designing spread footings, mats, or combined footings, computing bearing capacity and settlement, selecting allowable bearing pressure, or evaluating eccentric and moment loading on soil-supported foundations. Applies before fixing footing dimensions, while verifying service and strength limit states against geotechnical and structural capacity, and when reviewing bearing on layered or soft ground, expansive soil, or frost-susceptible sites.
---

# Shallow Foundation Design

Shallow foundation design is the engineering of the interface between a structure and the soil that supports it, where every column load, every wall reaction, and every overturning moment must be transferred to the ground without exceeding the soil's strength and without settling the structure beyond what it can tolerate. The foundation is the one structural element whose supporting medium is natural, variable, and only knowable through sampling, and the difference between a foundation that performs and one that fails is almost never the concrete: it is the soil capacity assumed, the settlement predicted, and the ground conditions actually encountered. The harm this skill prevents is a footing that punches into soft ground, a mat that differentially settles and tilts the frame, or a foundation that heaves under expansive soil or frost, any of which can crack the superstructure, jam doors, rack cladding, and, in the worst cases, trigger progressive structural distress. Because the soil is hidden and the foundation is buried, the design must be conservative against the uncertainty that cannot be eliminated.

## Core Rules

### Base the Design on a Site-Specific Geotechnical Investigation

No shallow foundation design is defensible without a site-specific geotechnical report that establishes the soil profile, the groundwater condition, the shear-strength or penetration-test data, the allowable bearing pressure, and the settlement estimate. Confirm that the borings or soundings extend below the zone of foundation influence (commonly to a depth of 1.5 to 2 times the footing width), that the spacing suits the project scale and the soil variability, and that the report addresses the specific foundation type and loading. Do not use "allowable bearing pressure" values from neighbouring sites, from code prescriptive tables for residential construction, or from regional assumptions, because the soil under a given footing is specific to that footing. Where the report gives a range or a contingent value, design for the conservative end and confirm the condition in the field during excavation.

### Verify Bearing Capacity at the Strength Limit State

Compute the ultimate bearing capacity by the appropriate method (Terzaghi, Meyerhof, Hansen, or Veseli, or a direct correlation from in-situ testing such as the standard penetration test or cone penetration test), using the soil shear strength, the footing width and depth, the groundwater condition, and the load inclination and eccentricity. Apply the factor of safety appropriate to the project and the reliability of the strength data (commonly 3.0 for static loading with conventional soil data, and higher where the data are sparse or the consequences are high), and confirm that the design bearing pressure under the worst load combination is within the allowable. For footings under moment or eccentric load, check the bearing pressure under the reduced effective area, because the load that concentrates at the edge can exceed the capacity even when the average pressure is acceptable. Confirm that the bearing layer is not underlain by a weaker layer within the stress influence zone, because bearing failure can occur by punching through the firm crust into the soft layer below.

### Verify Settlement at the Service Limit State

Settlement, not bearing capacity, governs most shallow foundation designs on competent soil, and it must be checked at both the total and the differential level. Compute the immediate (elastic) settlement, the primary consolidation settlement (for cohesive soils), and the secondary (creep) settlement, using the soil compressibility from the geotechnical data and the stress increase from the load, distributed by Boussinesq or equivalent. Confirm that the total settlement of each footing is within the tolerable value for the structure, and critically that the differential settlement between adjacent footings is within the angular distortion limit (commonly 1/500 to 1/300 for conventional framing, and tighter for sensitive framing or cladding). A foundation that meets the bearing capacity but exceeds the differential settlement limit will crack the superstructure, rack the cladding, and jam the doors, all of which are service failures that the bearing check cannot catch.

### Account for Eccentricity, Moment, and Overturning

Foundations under moment (from wind, seismic, or eccentric columns) must be checked for the bearing pressure distribution under the combined axial and moment load, the potential for partial uplift (loss of contact) under the overturning, and the sliding and overturning stability under the lateral load. Confirm that the resultant of the load falls within the middle third of the footing (for a triangular-free bearing distribution) or within the footing footprint (with the reduced effective area method), and that the factor of safety against overturning and sliding meets the project criteria. For footings under lateral load, confirm that the passive resistance of the soil in front of the footing and the friction on the base, suitably factored, resist the design load, and that the footing is embedded deep enough to mobilise the assumed resistance. A footing that is stable under gravity but tips or slides under the lateral case has failed, even if its bearing pressure is within limits.

### Address Groundwater, Frost, and Expansive Soil Conditions

Groundwater reduces effective stress and bearing capacity and can buoy the footing or flood the excavation; confirm the design bearing pressure uses effective (submerged) unit weights where the groundwater is within the influence zone, and that the foundation drainage prevents hydrostatic uplift or water intrusion. Frost depth governs the minimum footing embedment in cold climates, because frost heave can lift a shallow footing and thaw can drop it, and the footing must extend below the frost line or be founded on non-frost-susceptible material. Expansive soils, identified in the geotechnical report, can heave or shrink with moisture change and require special provisions: increased embedment, moisture control, soil replacement, or a switch to deep foundations. A foundation designed for bearing and settlement but ignoring frost or expansive soil will heave or settle seasonally, and the superstructure will reflect every cycle.

### Provide the Structural Design of the Footing Itself

The concrete footing must be designed to span the soil pressure and to transfer the column or wall load to the ground without shearing or flexural failure. Confirm the footing thickness for one-way shear (at a distance d from the column face), punching (two-way) shear (at a distance d/2 from the column face), and flexure (at the column face section), with the reinforcement sized for the moment in each direction. For combined footings and mats, analyse the soil pressure as a beam or slab on an elastic foundation, with the soil modelled by the modulus of subgrade reaction, and design the reinforcement for the resulting moments and shears. A footing that meets the geotechnical capacity but is under-reinforced will crack or punch through under the load, transferring the failure from the soil to the concrete.

### Verify the As-Built Condition During Construction

The geotechnical design assumes a soil condition at the bearing level; the construction must deliver that condition. Require that the bearing soil be inspected by the geotechnical engineer before the footing is placed, that any disturbed, softened, or unexpected soil be removed and replaced or re-evaluated, and that the excavation not be left open to soften or swell before concrete placement. Confirm that the footing is founded at the design depth, that the groundwater is controlled, and that any unsuitable material encountered during excavation is referred to the geotechnical engineer for re-evaluation rather than built over. A design that is correct on paper but built on disturbed or softened soil has failed at the moment of construction, before any load is applied.

## Common Traps

### The Bearing Pressure From A Neighbouring Site

The designer uses an allowable bearing pressure from a nearby project or a code prescriptive table, because the site "looks similar" or the budget did not include borings. The trap is that the value is defensible-looking, while the soil under the actual footing may be softer, layered, or more variable than the borrowed value represents, and the footing may settle or punch into ground that the borrowed capacity said was adequate. The false signal is the cited, numeric value; the harm is a foundation that fails the soil it actually sits on, discovered only when the structure settles or cracks.

### The Capacity Check Without The Settlement Check

The footing is sized for the allowable bearing pressure and the bearing capacity is verified, but the settlement is not computed, because the soil "looks competent." The trap is that the bearing capacity is one of two limit states, and on many competent soils the settlement, not the capacity, governs the size, and a footing sized for capacity alone may settle beyond the tolerable differential. The false signal is the verified bearing capacity; the harm is a structure that does not fail in bearing but cracks and racks from differential settlement that was never checked.

### The Soft Layer Below The Bearing Stratum

The bearing capacity is computed for the soil at the footing base, and the capacity is adequate, but a softer layer lies within the stress influence zone below the footing. The trap is that the capacity at the base is correct, while the load spreads into the softer layer below, which can punch or consolidate under the stress, and the footing settles or fails by bearing through the crust into the weak layer. The false signal is the adequate bearing capacity at the base; the harm is settlement or bearing failure driven by a layer the surface capacity did not represent.

### The Eccentric Load On A Small Footing

A footing under a column with moment or eccentricity is sized for the average bearing pressure, and the average is within the allowable. The trap is that the eccentricity concentrates the pressure at one edge, where it can exceed the allowable, or lifts the other edge, reducing the effective area and raising the pressure further, while the average stays within limits. The false signal is the compliant average pressure; the harm is local bearing failure or overturning at the edge, where the real stress is highest, under a load the average said was acceptable.

### The Frost Heave On A Shallow Footing

A footing is placed at a depth that meets the structural requirement but is above the local frost depth, in a frost-susceptible soil. The trap is that the footing is structurally adequate, while the frost-susceptible soil beneath it heaves in winter and thaws in spring, lifting and dropping the footing and the structure above. The false signal is the structural adequacy of the footing; the harm is seasonal heave and settlement that cracks the superstructure, because the frost depth, not the structural depth, governs the embedment.

### The Built-On-Disturbed-Soil Foundation

The excavation reaches the design bearing stratum, but the soil is disturbed by equipment, softened by rain or groundwater, or loosened by over-excavation, and the footing is placed on the disturbed surface without re-evaluation. The trap is that the design assumed the undisturbed bearing capacity, while the actual bearing surface is weaker and more compressible than the design value, and the footing settles or fails on the disturbed layer. The false signal is the design bearing elevation reached; the harm is a foundation built on a condition the geotechnical report never described, because the construction disturbed the very soil the design depended on.

## Self-Check

- Is the design based on a site-specific geotechnical report with borings extending below the influence zone, addressing the foundation type and loading?
- Is the bearing capacity verified at the strength limit state with the appropriate factor of safety, accounting for footing dimensions, groundwater, eccentricity, and a weak layer below?
- Is the total and differential settlement verified at the service limit state, with the differential within the angular distortion limit for the structure?
- Are eccentric and moment loads checked for edge bearing pressure, partial uplift, and overturning and sliding stability under the lateral cases?
- Are groundwater, frost depth, and expansive soil conditions addressed, with embedment, drainage, or special provisions where required?
- Is the concrete footing designed for one-way shear, punching shear, and flexure, with reinforcement sized for the soil pressure distribution?
- For mats and combined footings, is the soil modelled as a beam or slab on the modulus of subgrade reaction, with moments and shears designed accordingly?
- Is the bearing soil inspected by the geotechnical engineer before placement, with disturbed or unexpected soil removed or re-evaluated rather than built over?
