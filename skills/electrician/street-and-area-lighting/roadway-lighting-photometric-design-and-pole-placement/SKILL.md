---
name: roadway-lighting-photometric-design-and-pole-placement.md
description: Use when the agent is designing roadway or area lighting layouts, selecting luminaire distribution types (Type II through V), placing poles for uniformity, interpreting IES photometric files, evaluating light trespass and uniformity ratios, or planning LED retrofits against existing HID photometry.
---

# Roadway Lighting Photometric Design and Pole Placement

Roadway and area lighting design is a photometric problem, not a brightness problem: the goal is to deliver a specified maintained illuminance with controlled uniformity across the pavement, while keeping light out of adjacent properties and the night sky. The judgment problem is that lighting layout is often reduced to spacing poles at a fixed interval and mounting a familiar wattage, when the actual performance depends on the luminaire's distribution type, mounting height, pole spacing, overhang, tilt, lumen maintenance, and the geometry of the road. An agent that ignores photometric analysis will produce a layout with bright pools under each luminaire and dark gaps between them, light spilling into bedroom windows, or a retrofit that draws fewer watts but fails the uniformity requirement because the LED distribution does not match the HID it replaced.

## Core Rules

### Select the Distribution Type From the Road Geometry and Mounting Height

IES distribution types describe the lateral light pattern: Type II is narrow (narrow roads, walkways), Type III is medium (typical roadside mounting, the most common roadway type), Type IV is wide (wide roadways, parking lot perimeters), and Type V is symmetric (center-mounted area lighting, intersections). The defense is to match the distribution type to the road width and the pole position (roadside versus median versus center), verify the distribution covers the full road width without excessive far-side spill, and avoid using a Type V symmetric luminaire on a roadside pole where it wastes half its light on the far side and behind the pole.

### Set the Mounting Height and Spacing to the Distribution and Uniformity Requirement

The ratio of pole spacing to mounting height (S/MH) is the primary driver of uniformity: a higher ratio stretches the light but creates dark gaps; a lower ratio improves uniformity but requires more poles and luminaires. Typical roadway S/MH ratios range from 4 to 6 for staggered layouts and 3 to 5 for one-side layouts, depending on distribution type and required uniformity. The defense is to run a point-by-point photometric calculation with the actual luminaire IES file, mounting height, and spacing, verify the uniformity ratio (average-to-minimum and maximum-to-minimum) meets the IES or agency specification, and adjust spacing or height until the calculation passes. Never space poles by rule of thumb without a calculation.

### Use the Actual IES Photometric File, Not a Generic Wattage Assumption

Every luminaire has a unique photometric distribution captured in its IES (.ies) file, which specifies candela values across all vertical and horizontal angles. The defense is to obtain the IES file for the exact luminaire, optics, lumen package, and color temperature being specified, run the layout in photometric software, and verify the results against the design criteria. Substituting a "similar" luminaire without re-running the photometrics invalidates the layout, because two luminaires with the same lumen output can have radically different distributions and produce completely different uniformity.

### Apply Light Loss Factors and Verify Maintained Illuminance

The design must deliver the required illuminance at end-of-life, not at initial installation, so light loss factors (LLF) must be applied: lamp lumen depreciation (LLD), luminaire dirt depreciation (LDD), and for LEDs, the L70 or rated lumen maintenance at the design life. The defense is to apply the appropriate maintenance factors for the environment (clean, moderate, or dirty), use the LED manufacturer's rated lumen maintenance at the design life (often 50,000 or 100,000 hours), and verify the maintained average illuminance meets the specification. Designing to initial lumens over-lights the area initially and under-lights it as the system ages.

### Control Light Trespass and Glare at Property Lines and Roadways

Light trespass (unwanted illumination of adjacent properties) and glare (direct view of the light source into drivers' or residents' eyes) are regulated and objectionable. The defense is to select luminaires with house-side shields or back-light control where the pole is near a property line, specify cutoff or full-cutoff optics that limit light above horizontal, verify the vertical illuminance at the property line meets the local or LEED limit, and choose a color temperature and output that minimizes ecological disruption. LED retrofits often increase glare because the source is a point array, so optical control matters more than raw lumen output.

### Validate LED Retrofits Against the Original HID Photometry, Not Just Wattage

Replacing an HID luminaire with an LED of "equivalent wattage" frequently fails because LED luminaires direct light more efficiently and have different distributions, so the same wattage can over-light or under-light the road and change the uniformity. The defense is to run a photometric comparison of the existing HID layout and the proposed LED, match the maintained illuminance and uniformity (not the wattage), verify the LED distribution type matches the road geometry, and account for the LED's different lumen maintenance curve. A wattage-equivalent swap is not a photometric equivalent.

## Common Traps

### Spacing Poles by Rule of Thumb Without a Photometric Calculation

The designer spaces poles at 150 feet because that is what was done on the last job, and mounts a 400W luminaire. The mechanism of the failure is that the actual uniformity depends on the distribution type, mounting height, and road width, and the rule-of-thumb spacing produces a maximum-to-minimum uniformity ratio that exceeds the specification, leaving dark gaps between poles that fail the agency's acceptance test. The false signal is that the road "looks lit" from a drive-through, when the point-by-point calculation would show noncompliant dark spots. The harm is a layout that fails inspection and requires added poles or luminaires after installation.

### Using a Symmetric Type V Luminaire on a Roadside Pole

A Type V (symmetric) luminaire is mounted on a roadside pole because it is in stock, throwing equal light in all directions. The mechanism of the failure is that half the lumens go behind the pole onto private property (light trespass) and across the road beyond the far lane, while the near lane is under-lit relative to a Type III designed for roadside mounting. The false signal is that the luminaire "is bright," implying coverage, when the distribution wastes light and creates trespass. The harm is noncompliant light trespass, wasted energy, and uneven pavement illumination.

### Designing to Initial Lumens and Ignoring Maintenance Factors

The layout is calculated at the luminaire's initial lumen output, and it meets the average illuminance on paper. The mechanism of the failure is that within the maintenance cycle the luminaire output drops due to dirt accumulation and lumen depreciation, and the maintained illuminance falls below the specification, producing a road that is under-lit at end of the maintenance period. The false signal is that the calculation "passed," implying compliance, when the maintenance factors were not applied. The harm is a road that meets the standard only when freshly cleaned and fails for most of its service life.

### Substituting a Similar Luminaire Without Re-Running the Photometrics

The specified luminaire is unavailable, so a "similar" one with the same lumen output is substituted, and the layout is built. The mechanism of the failure is that the substitute has a different distribution (a wider or narrower beam, a different cutoff), so the point-by-point illuminance pattern changes, the uniformity shifts, and the layout that passed with the specified luminaire fails with the substitute. The false signal is that "same lumens means same result," when distribution, not total lumens, drives the layout. The harm is a noncompliant layout discovered only at the acceptance photometric test.

### Matching Wattage in an LED Retrofit and Missing the Uniformity

A 400W HPS luminaire is replaced with a 150W LED "equivalent," and the average illuminance is similar, so the retrofit is declared successful. The mechanism of the failure is that the LED's tighter optical control and different distribution produce a different uniformity pattern — often brighter directly under the luminaire and darker between poles — so the average is maintained but the minimum drops and the uniformity ratio fails. The false signal is that the average illuminance "matches," implying equivalence, when the uniformity degraded. The harm is a road that meets the average but has dark gaps that fail the uniformity criterion.

### Ignoring Light Trespass at Property Lines and Causing Complaints

A parking-lot or roadway luminaire near a residential property line throws light into bedroom windows because no house-side shield or back-light control was specified. The mechanism of the failure is that the luminaire's distribution sends a portion of its output backward and sideways across the property line, exceeding the permitted vertical illuminance and generating complaints and code violations. The false signal is that the luminaire "is full cutoff," implying no trespass, when full cutoff only limits uplight, not backlight. The harm is regulatory complaints, required retrofit shielding, and neighbor conflict.

## Self-Check

- Did I select the IES distribution type (II, III, IV, or V) based on the road width and pole position, and verify it covers the full road width without excessive spill?
- Did I run a point-by-point photometric calculation with the actual luminaire IES file, mounting height, and spacing, and verify the average-to-minimum and maximum-to-minimum uniformity ratios meet the specification?
- Did I obtain and use the exact IES photometric file for the specified luminaire, optics, lumen package, and color temperature, rather than a generic or substitute file?
- Did I apply the appropriate light loss factors (LLD, LDD, and LED lumen maintenance at design life) and verify the maintained average illuminance meets the specification at end of the maintenance period?
- Did I control light trespass with house-side shields or back-light control where poles are near property lines, verify the vertical illuminance at the line meets the limit, and specify cutoff optics to limit uplight?
- For an LED retrofit, did I run a photometric comparison against the existing HID layout and match the maintained illuminance and uniformity, rather than matching wattage alone?
- Did I verify the layout meets the relevant IES recommended practice (RP-8 for roadways, RP-20 for parking) or the agency specification for the road class?
- Is the photometric layout documented with the luminaire schedule, IES file reference, calculation grid, uniformity results, and maintenance factors, so the design is traceable and verifiable?
