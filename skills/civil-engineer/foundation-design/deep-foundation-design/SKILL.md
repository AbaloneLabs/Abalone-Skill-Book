---
name: deep-foundation-design.md
description: Use when the agent is designing driven piles or drilled shafts, computing axial capacity by static analysis or dynamic testing, evaluating pile group action, or resolving lateral capacity, downdrag, and setup on deep foundation systems. Applies before specifying pile type, length, and capacity, while verifying single-pile and group capacity against structural loads, and when reviewing pile driving, wave equation, and static load test results.
---

# Deep Foundation Design

Deep foundation design is the engineering of piles or drilled shafts that transfer structural load through weak or compressible near-surface soil to a competent bearing stratum at depth, and it is the foundation choice when shallow foundations cannot meet the bearing or settlement limits. A deep foundation is the most geotechnically uncertain of structural elements: its capacity depends on soil or rock that cannot be directly seen, its installation changes the ground around it, and its performance under load is verified by indirect methods. The harm this skill prevents is a pile that does not develop its assumed capacity, a pile group that settles as a block because group effects were ignored, or a pile that is damaged by driving or by downdrag from settling soil, any of which can leave the structure under-supported and require costly, intrusive remediation. Because the capacity is hidden and the installation is irreversible, the design must be conservative, the installation must be controlled, and the capacity must be verified by testing rather than assumed.

## Core Rules

### Base the Design on a Deep Investigation and Clear Load Transfer Mechanism

The geotechnical investigation for deep foundations must extend below the anticipated founding depth, with sufficient borings or soundings to characterise the variability of the bearing stratum across the footprint. Establish the load transfer mechanism: end-bearing on a hard stratum, shaft friction along the embedded length, or a combination, and confirm that the stratum that provides the capacity is continuous and of adequate thickness, not a thin lens. Identify compressible layers above and below the bearing stratum that could cause negative skin friction (downdrag) or settlement of the bearing layer under the group load. A pile design based on an investigation that stops at the assumed tip elevation has not verified the stratum it depends on.

### Compute Axial Capacity by Static Analysis and Verify by Testing

Compute the ultimate axial capacity by static analysis, summing the shaft friction (from the soil or rock shear strength and the effective stress, by alpha, beta, or lambda methods for cohesive soil, and by effective stress or direct CPT methods for cohesionless soil) and the end-bearing (from the bearing stratum strength). Apply the appropriate factor of safety (commonly between 2.0 and 3.0 for static analysis alone, reduced toward 1.5 to 2.0 when verified by load testing, per the governing code). Verify the capacity by the appropriate method for the project scale and risk: a static load test (the most reliable, required by many codes for major projects), a dynamic load test by PDA during driving, or wave-equation driving analysis with established pile set criteria. A pile capacity computed by static analysis alone, without verification, is an estimate with the full uncertainty of the soil strength data.

### Evaluate Pile Group Capacity and Settlement

Piles are almost always installed in groups, and the group does not behave as the sum of the individual piles. Check the group capacity for block failure (where the group fails as a block along a perimeter of soil, rather than as individual piles), with the group efficiency factor appropriate to the soil type and spacing. Critically, check the group settlement, because the stress bulb of a group extends far deeper than that of a single pile, and the group can settle by consolidation of a compressible layer that a single pile would not affect. Confirm that the group settlement under the design load is within the tolerable value for the structure, and that the differential settlement between groups is within the angular distortion limit. A group designed for the single-pile capacity, multiplied by the number of piles, without group effects, can settle as a block far beyond the single-pile prediction.

### Evaluate Lateral Capacity and Pile-Head Fixity

Piles under lateral load (from wind, seismic, soil pressure, or vessel impact) develop their capacity by the lateral resistance of the soil along the embedded length, analysed by the p-y method (for soil-structure interaction) or the Broms method (for simplified capacity). Confirm that the lateral deflection at the pile head under the design load is within the tolerable value for the structure, that the moment and shear in the pile under the lateral load are within the structural capacity, and that the pile-head fixity (fixed or free) assumed in the analysis matches the detail provided by the cap. For pile groups under lateral load, account for the row-spacing and shadowing effects that reduce the resistance of trailing rows. A pile that meets the axial capacity but deflects laterally beyond the structure's tolerance has failed its service requirement.

### Address Downdrag, Setup, and Installation Effects

Downdrag occurs when the soil around the pile settles (by consolidation under fill or dewatering) and drags the pile downward, adding a load that the pile must carry in addition to the structural load. Identify the downdrag condition, compute the drag load, and either add it to the design load or provide a slip layer or bitumen coating to reduce it. Setup (the increase in capacity over time as cohesionless soil densifies and cohesive soil remoulds and regains strength) can increase capacity by 30 to 100 percent over the initial drive, and the design and the driving criteria must account for it, especially when re-driving or restriking. Installation effects, including heave of surrounding soil during driving, lateral displacement of adjacent piles, and relaxation of the bearing layer, must be monitored and addressed. A pile design that ignores downdrag, setup, or installation effects is a design that will behave differently in the field than on paper.

### Specify the Pile Type, Material, and Installation Tolerance

Select the pile type (driven steel, driven precast concrete, drilled shaft, augercast, or others) suited to the soil, the load, the environment, and the installation constraints, with the material grade and the corrosion allowance appropriate to the service life. Specify the installation tolerance for position, plumbness, and tip elevation, and the driving criteria (final set per blow, hammer energy, or drilled-shaft concrete volume) that confirm the capacity. For driven piles, perform a wave-equation analysis to confirm that the hammer can drive the pile to the capacity without damage, and set the refusal criterion that prevents overdriving. For drilled shafts, specify the excavation method, the slurry condition, the concrete placement, and the cleanliness of the base, because a dirty base or a slurry-included defect can eliminate the end-bearing.

### Verify Capacity and Integrity During and After Installation

During installation, monitor the pile driving or shaft drilling against the criteria, and require dynamic testing or static load testing at the frequency the project and code require. After installation, perform integrity testing (low-strain sonic, cross-hole sonic, thermal, or gamma-gamma for drilled shafts) to detect defects, cracks, or necking, and disposition any defective pile by re-analysis, replacement, or supplementary piles. A pile that drives to the criteria but contains a defect has not developed its capacity, and the integrity test is the verification that the constructed pile matches the designed pile.

## Common Traps

### The Static Capacity Without Verification

The pile capacity is computed by static analysis and the design proceeds without a load test or dynamic verification, because the soil data "look adequate." The trap is that the static analysis carries the full uncertainty of the soil strength correlation, and the actual capacity can be half or double the computed value, especially in mixed or layered soil. The false signal is the defensible, computed capacity; the harm is a foundation that may be far under capacity, discovered only when the structure settles or when a load test is finally run during construction.

### The Group Designed As The Sum Of Single Piles

The pile group capacity is computed as the single-pile capacity multiplied by the number of piles, with no group efficiency or settlement check. The trap is that the arithmetic produces a defensible total, while the group can fail as a block or settle by consolidation of a deep layer that the single-pile analysis does not reach, and the group settles far more than the single pile. The false signal is the multiplied capacity; the harm is a foundation that meets the single-pile capacity on paper but settles as a block, cracking the structure.

### The Downdrag Not Accounted For

The site has fill or dewatering that will consolidate the upper soil, and the piles are designed for the structural load only, without the drag load from the settling soil. The trap is that the piles meet the structural capacity, while the settling soil adds a downward load that can exceed the structural load and push the pile into the bearing stratum or to failure. The false signal is the verified structural capacity; the harm is a foundation that settles or fails under a load the design never included, because the downdrag condition was not recognised.

### The Setup Ignored In Driving Criteria

A pile is driven to the design capacity on the day of installation, without accounting for the setup that will increase the capacity over the following weeks, and the pile "refuses" at a higher tip than necessary. The trap is that the pile meets the criteria, while the design could have used a shorter pile if the setup capacity had been accounted for, or, conversely, the pile is driven too hard to reach a capacity that setup would have provided, damaging the pile. The false signal is the achieved driving resistance; the harm is either an over-driven, damaged pile or an unnecessarily deep and expensive foundation.

### The Lateral Deflection Not Checked

A pile under lateral load is checked for capacity but not for deflection, and the pile "carries" the load. The trap is that the pile has the lateral capacity, while the deflection at the head exceeds the structure's tolerance, and the supported element moves laterally beyond what the connections or the cladding can accommodate. The false signal is the adequate lateral capacity; the harm is a service failure (cracked connections, jammed elements, displaced superstructure) that the capacity check could not catch.

### The Dirty Base On A Drilled Shaft

A drilled shaft is excavated to the bearing stratum, but the base is not cleaned of slurry, cuttings, or debris before the concrete is placed. The trap is that the shaft is drilled to the design depth and the concrete is placed, while the end-bearing sits on a soft layer of debris that compresses under load, eliminating the end-bearing capacity the design assumed. The false signal is the shaft at the design depth; the harm is a shaft that develops only the shaft friction and settles under the design load, because the end-bearing was lost to a dirty base.

## Self-Check

- Is the design based on an investigation that extends below the founding depth and verifies the continuity and thickness of the bearing stratum?
- Is the axial capacity computed by static analysis and verified by static load test, dynamic load test, or wave-equation analysis at the frequency the project and code require?
- Is the pile group checked for block failure (group efficiency) and group settlement, with the differential settlement within the structure's tolerance?
- Is the lateral capacity and deflection checked by the p-y or Broms method, with the moment and shear in the pile within the structural capacity and the head fixity matching the detail?
- Are downdrag, setup, and installation effects (heave, displacement, relaxation) identified and accounted for in the design and the driving criteria?
- Is the pile type, material, corrosion allowance, installation tolerance, and driving or drilling criterion specified, with the wave-equation analysis confirming the hammer and the refusal criterion?
- For drilled shafts, are the excavation, slurry, base cleanliness, and concrete placement specified, and is the base cleanliness verified before placement?
- Is integrity testing (low-strain, cross-hole, thermal, gamma-gamma) performed at the required frequency, and are defective piles dispositioned by re-analysis, replacement, or supplement?
