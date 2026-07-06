---
name: mat-and-raft-foundation-design.md
description: Use when the agent is designing mat (raft) foundations, analyzing soil-structure interaction, determining mat thickness and reinforcement, evaluating differential settlement and angular distortion, selecting the analysis method (rigid, finite element, modulus of subgrade reaction), handling variable soil conditions or compressible layers beneath the mat, or coordinating mat design with column loads and superstructure stiffness.
---

# Mat and Raft Foundation Design

Mat foundation design is a domain where the foundation and the soil are a single interacting system, where the load distribution to the soil depends on the relative stiffness of the mat, the superstructure, and the ground, and where the governing design check is often not the bearing capacity but the differential settlement that distorts the superstructure, or the shear and moment in the mat that the soil pressure distribution produces. The recurring failure is to analyze the mat as a rigid body on a uniform soil pressure — which hides the differential settlement and the mat internal forces — or to use a modulus of subgrade reaction without recognizing that it is not a soil property but a spring constant dependent on the mat size and the loading, and to design the mat thickness and reinforcement from an analysis that does not capture the soil-structure interaction. The engineer's job is to model the mat-soil interaction with a method that captures the stiffness, to evaluate both the bearing capacity and the differential settlement against the superstructure's tolerance, and to design the mat as a structural element carrying the soil pressure distribution that the interaction produces. This skill covers the judgment used to design mat foundations, with the overriding principle that the mat and the soil are one system and the differential settlement, not the average bearing pressure, usually governs.

## Core Rules

### Model the Soil-Structure Interaction With an Appropriate Method

The mat's response depends on the relative stiffness of the mat and the soil, which sets whether the mat behaves as rigid (dishing the soil pressure to the edges), flexible (concentrating the pressure under the loads), or intermediate. Select the analysis method against the relative stiffness: a rigid method (combined with a settlement analysis) for thick, stiff mats on uniform soil; a finite element method with a soil model (modulus of subgrade reaction, or an elastic continuum, or a nonlinear soil model) for mats of intermediate stiffness, variable soil, or irregular loading. The finite element method captures the soil-structure interaction that the rigid method hides, and it is the appropriate method for most mat designs of meaningful complexity. The method must capture the interaction, not assume it away.

### Recognize That the Modulus of Subgrade Reaction Is Not a Soil Property

The modulus of subgrade reaction (k) is the ratio of pressure to settlement at a point, and it is commonly used as the soil spring in a finite element mat analysis. But k is not a fundamental soil property — it depends on the size and shape of the loaded area, the soil profile, and the loading, so a k value from a plate load test or a table cannot be applied directly to a large mat without adjustment. Determine k from the soil's elastic modulus and the mat dimensions, or from the settlement analysis of the loaded area, and recognize that using a single, unadjusted k value can misrepresent the soil's stiffness and the resulting mat forces. The subgrade reaction is a modeling tool, and its limitations must be understood when interpreting the analysis.

### Evaluate Differential Settlement and Angular Distortion Against the Superstructure Tolerance

The average settlement of a mat is rarely the governing check; the differential settlement between columns, and the resulting angular distortion (the slope of the mat between supports), is what distresses the superstructure, damages finishes, cracks partitions, and impairs sensitive equipment. Calculate the differential settlement from the soil-structure interaction analysis, express it as an angular distortion, and compare it to the tolerable limits for the structural system and the use (commonly 1/500 to 1/1000 for conventional buildings, tighter for sensitive equipment or plain masonry). If the differential settlement exceeds the tolerance, increase the mat stiffness (thickness), improve the soil (ground improvement), or use a partial or full pile-supported mat. Differential settlement, not average settlement or bearing capacity, is the usual governing check.

### Design the Mat Thickness for Shear, Punching Shear, and Serviceability

The mat thickness is set by the one-way shear at the column lines, the two-way (punching) shear at the columns, the serviceability (deflection and crack control), and the construction sequencing (concrete placement, heat of hydration), not by flexure alone. Check the punching shear at the columns with the critical perimeter per the code (ACI 318), accounting for the column size and the mat depth, and confirm the mat is thick enough to resist the column reaction without shear reinforcement, because adding shear reinforcement to a mat is complex and usually avoided by increasing the thickness. Check the one-way shear and the flexure, and confirm the thickness also limits the heat of hydration and the shrinkage cracking during construction. The thickness is governed by shear and constructability as often as by flexure.

### Design the Reinforcement for the Moment Distribution From the Interaction Analysis

The mat reinforcement is designed for the moment distribution that the soil-structure interaction analysis produces, which can include both positive and negative moments across the mat, concentrations at the columns, and the effects of variable soil stiffness and loading. Provide the reinforcement (top and bottom, in both directions) for the maximum moments, with due attention to the development lengths, the lap splices, and the detailing at the columns and the edges. Do not design the reinforcement from a uniform pressure assumption, because the actual pressure distribution — concentrated under the loads for flexible mats, at the edges for rigid mats — produces different and often larger moments. The reinforcement must match the interaction analysis.

### Account for Variable Soil Conditions and Compressible Layers

The soil beneath a mat is rarely uniform, and variable soil conditions — a soft pocket, a compressible layer, a varying bedrock depth, an old foundation — produce differential settlement and mat stresses that a uniform-soil analysis does not capture. Characterize the soil variability across the mat footprint from the site investigation, model the variable conditions in the analysis (variable modulus, variable layer thickness), and evaluate the differential settlement and the mat forces that result. A mat on variable soil behaves very differently from a mat on uniform soil, and ignoring the variability produces a design that does not match the as-built condition.

### Coordinate the Mat Design With the Superstructure Stiffness

The superstructure is not a set of independent column loads on the mat; it is a stiff structure that redistributes load as the mat settles, with stiff elements (shear walls, braced frames) attracting more load and flexible elements less. Include the superstructure stiffness in the soil-structure interaction analysis, or approximate it with equivalent column springs, because the load redistribution changes the mat forces and the settlement distribution. A mat designed with pinned column loads, ignoring the superstructure stiffness, can under-predict the moments under the stiff elements and over-predict them under the flexible ones, and the design does not match the real behavior.

## Common Traps

### Mat Analyzed as Rigid on a Uniform Soil Pressure

The engineer analyzes the mat as a rigid body on a uniform soil pressure, calculates the average bearing stress, and checks it against the allowable, without analyzing the soil-structure interaction or the differential settlement. The mechanism is that the rigid method is simple and the uniform pressure feels conservative. The false signal is a bearing check that "passes." The harm is that the differential settlement, which governs the superstructure distress, is not evaluated, and the mat internal forces, which depend on the actual pressure distribution, are not designed for. The interaction must be modeled.

### Modulus of Subgrade Reaction Used as a Fixed Soil Property

The engineer uses a single modulus of subgrade reaction (k) from a table or a plate load test, without adjusting for the mat size and the soil profile. The mechanism is that k is presented as a soil property. The false signal is a k value in the analysis. The harm is that the unadjusted k misrepresents the soil's stiffness — typically over-stiffening a large mat, which under-predicts the settlement and the moments — and the design is based on a spring constant that does not represent the real soil-structure interaction. The k must be determined for the mat dimensions and soil profile.

### Differential Settlement Not Checked Against the Superstructure Tolerance

The engineer checks the average settlement and the bearing capacity but does not check the differential settlement and the angular distortion against the superstructure's tolerance. The mechanism is that the average settlement and the bearing capacity feel like the design checks. The false signal is a settlement and a bearing check that "pass." The harm is that the differential settlement, which is what distresses the superstructure, exceeds the tolerable angular distortion, and the building cracks, tilts, or impairs equipment despite the average settlement being acceptable. The differential settlement must be checked.

### Mat Thickness Set by Flexure Alone, Shear and Punching Shear Ignored

The mat thickness is set by the flexural moment, without checking the one-way shear, the two-way punching shear at the columns, or the constructability. The mechanism is that flexure feels like the primary design. The false signal is a thickness that "works for moment." The harm is that the mat fails in punching shear at a heavily loaded column, or in one-way shear along a column line, because the thickness is too thin, and shear failure is sudden and catastrophic. The thickness must be set by shear and punching shear, with flexure and constructability confirmed.

### Reinforcement Designed From a Uniform Pressure Assumption

The mat reinforcement is designed for the moments from a uniform soil pressure, rather than from the interaction analysis. The mechanism is that the uniform pressure feels like the load. The false signal is a reinforcement design. The harm is that the actual pressure distribution — concentrated under the loads, or at the edges for a rigid mat — produces larger moments than the uniform assumption, and the mat is under-reinforced at the critical locations. The reinforcement must come from the interaction analysis.

### Variable Soil Conditions Modeled as Uniform

The engineer models the soil beneath the mat as uniform, despite the site investigation showing variable conditions, soft pockets, or compressible layers. The mechanism is that the uniform model is simpler and the variability feels like detail. The false signal is a completed analysis. The harm is that the variable soil produces differential settlement and mat stresses that the uniform model does not capture, and the mat cracks or the superstructure distresses over the variable zones. The variable conditions must be modeled.

### Superstructure Stiffness Ignored in the Interaction

The mat is analyzed with the column loads applied as pinned point loads, without the superstructure stiffness, so the load redistribution from the mat settlement is not captured. The mechanism is that the columns are the loads and the superstructure is a separate structure. The false signal is a mat analysis with column loads. The harm is that the stiff elements (shear walls, braced frames) attract more load than the pinned assumption gives them, the mat moments under those elements are under-designed, and the settlement distribution differs from the analysis. The superstructure stiffness must be included.

## Self-Check

- Is the soil-structure interaction modeled with an appropriate method (finite element for complex mats, rigid for simple uniform conditions), rather than assuming a rigid mat on uniform pressure?
- Is the modulus of subgrade reaction determined for the mat dimensions and the soil profile, rather than used as a fixed soil property from a table or plate test?
- Is the differential settlement and the angular distortion checked against the superstructure's tolerance, with mitigation (thicker mat, ground improvement, piles) provided if exceeded?
- Is the mat thickness set by one-way shear, two-way punching shear at the columns, and constructability, with flexure confirmed, rather than by flexure alone?
- Is the reinforcement designed for the moment distribution from the interaction analysis, with top and bottom steel in both directions and detailing at the columns and edges?
- Are variable soil conditions — soft pockets, compressible layers, varying bedrock — modeled in the analysis, rather than assumed uniform?
- Is the superstructure stiffness included in the soil-structure interaction (or approximated), so the load redistribution from the mat settlement is captured?
