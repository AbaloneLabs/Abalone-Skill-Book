---
name: spatial-analysis-and-overlay-for-civil.md
description: Use when the agent is performing spatial analysis for civil engineering, running buffer and overlay operations, conducting network and suitability analysis, delineating watersheds from DEM or LiDAR, computing viewsheds, executing spatial joins, or evaluating how input data error propagates through geoprocessing results.
---

# Spatial Analysis and Overlay for Civil

Spatial analysis is where GIS turns data into decisions, and the judgment problem is that the operations (buffer, overlay, network trace, watershed delineation) look like clean algorithmic steps but each carries assumptions about scale, error, and topology that determine whether the result is defensible or misleading. Agents tend to run a buffer or an overlay and accept the output, and miss that a buffer distance applied to a coarse dataset captures the wrong area, that an overlay of mismatched scales compounds error into the intersection, that a watershed delineated from a coarse DEM follows the wrong flow path, and that error propagates multiplicatively through chained operations until the result is precise-looking but wrong. The harm this skill prevents is a suitability map that selects the wrong sites, a buffer that under- or over-protects a corridor, a watershed boundary that misallocates drainage, and a decision presented as analysis when it rests on unexamined error. This skill covers buffer and overlay, network and suitability analysis, watershed delineation from DEM and LiDAR, viewshed, spatial joins, and error propagation. It is an analytical-engineering discipline, and the agent must evaluate the input fitness, the operation assumptions, and the propagated error before presenting any spatial result as a basis for design or decision.

## Core Rules

### Apply Buffer and Overlay With Awareness of Scale and Boundary Effects

A buffer creates a zone of a stated distance around a feature, and an overlay combines layers (union, intersect, identity) to produce new geometries and attributes, and both are sensitive to the input scale and to boundary artifacts. Apply a buffer distance matched to the phenomenon and the input accuracy: a regulatory setback buffer applied to a coarse road centerline captures a different area than one applied to a survey-grade centerline, and the buffer width should exceed the positional uncertainty or the buffer is thinner than the error. In overlay, recognize that intersecting layers of different scales compounds the error of the least accurate layer into the output, and that slivers (tiny polygons along boundaries) arise from misaligned layers and must be resolved, not counted as real features. The disciplined rule is to document the buffer basis (regulatory, analytical, or heuristic), to use the most accurate input available, and to clean slivers and resolve boundary mismatches before interpreting the overlay. The trap is an overlay of a coarse soils layer and a fine parcel layer that produces thousands of sliver polygons treated as real units, inflating counts and misallocating areas.

### Conduct Network and Suitability Analysis on Validated Topology

Network analysis (shortest path, service area, flow tracing) and suitability analysis (weighted overlay of factors to rank sites) depend on a validated network or a defensible weighting scheme. For network analysis, confirm the topology (connectivity, one-way restrictions, turn restrictions, impedance such as length or travel time) is complete and correct, because a network with gaps or wrong impedances produces paths that do not exist or that violate constraints. For suitability analysis, define the factors (slope, soils, distance to infrastructure, environmental constraints), standardize them to comparable scales, assign weights that sum to a defined total, and document the rationale, because the weights are judgments that drive the result and must be transparent and sensitivity-tested. The disciplined rule is to validate the network before tracing, to document the suitability factors and weights, and to run a sensitivity analysis to see how the result changes when weights shift, because a suitability map that flips under small weight changes is not robust. The trap is a suitability analysis with undocumented or arbitrary weights presented as objective, hiding the judgment that actually drives the site selection.

### Delineate Watersheds and Drainage From DEM or LiDAR With Hydro-Conditioning

Watershed delineation and flow-direction analysis derive drainage networks and basins from a DEM, and the quality depends on the DEM resolution and on hydro-conditioning that removes spurious sinks. Use a DEM of resolution matched to the drainage features: LiDAR-derived DEMs (meter or sub-meter) resolve fine channels and culverts that a 10-meter or 30-meter DEM smooths away, and the choice changes the delineated watershed and the drainage network. Hydro-condition the DEM by filling true sinks only where they are not real depressions, by breaching road embankments and culverts where flow is known to pass, and by enforcing known stream lines, because an unconditioned DEM routes flow into artificial pits and produces watersheds cut off at roads. The disciplined rule is to choose the DEM resolution for the drainage scale, to hydro-condition with knowledge of the real drainage (culverts, bridges, depressions), and to validate the delineated network against observed streams and drainage structures. The trap is delineating a watershed from a coarse, unconditioned DEM that routes flow the wrong way at a road or a culvert, misallocating drainage area and corrupting a hydrologic model.

### Compute Viewsheds and Spatial Joins With Defined Inputs and Tolerances

A viewshed computes the area visible from one or more observer points over a surface, and it depends on the DEM resolution, the observer and target offsets (eye height, structure height), and the curvature and refraction corrections, all of which must be stated because they change the result materially. A spatial join transfers attributes between layers based on location (which parcels a pipeline crosses, which buffers contain a well), and it depends on the join rule (intersect, within, closest) and on a tolerance that accounts for positional error, because a join with zero tolerance misses features that are within the error band. The disciplined rule is to state the viewshed's observer offset, target offset, and corrections, and to use a DEM whose resolution supports the line-of-sight being analyzed, and for spatial joins, to choose the rule and tolerance deliberately and to check borderline cases. The trap is a viewshed computed with default offsets that do not match the actual observer or structure, over- or under-predicting visibility, or a spatial join with no tolerance that drops features lying just outside a boundary by a distance smaller than the data error.

### Evaluate Error Propagation Through Chained Geoprocessing

Every geoprocessing operation carries the error of its inputs and adds the error of the operation, and chained operations propagate error so that the final result can be far less certain than its precise appearance suggests. Quantify or at least bound the error: positional error in the inputs (from metadata accuracy class), error introduced by the operation (resampling in raster overlay, slivers in vector intersect, generalization), and the compounding when operations chain (a buffer of a buffer, an overlay of several layers). Present results with their uncertainty, using accuracy statements, confidence bands, or sensitivity analysis, rather than as deterministic outputs, because a suitability score of 0.84 built from coarse inputs is not more reliable than one of 0.6. The disciplined rule is to track the lineage of inputs and operations, to estimate the propagated error, and to communicate the result at a resolution consistent with its accuracy, never finer. The trap is a multi-step analysis whose output is presented to many decimal places or as a crisp boundary when the input error would move that boundary by a large distance, overstating the certainty of the decision.

## Common Traps

### The Buffer Thinner Than the Positional Error

A buffer of a distance smaller than the input's positional accuracy is applied, so the buffer zone is narrower than the error band. The mechanism is that positional uncertainty exceeds the buffer width. The false signal is a buffer drawn on the map. The harm is a setback or corridor that captures the wrong area, under- or over-protecting the feature it was meant to bound.

### The Overlay Compounding Coarse-Input Error

An overlay combines a coarse layer (soils, planning-scale hydrography) with a fine layer, and the error of the coarse layer propagates into the intersection. The mechanism is that the least accurate layer sets the accuracy of the output. The false signal is a detailed-looking intersection. The harm is areas and counts that are wrong at the scale of the coarse input, misallocating quantities and sites.

### The Watershed From a Coarse, Unconditioned DEM

A watershed is delineated from a 30-meter DEM without breaching roads or culverts, so flow routes into artificial pits or the wrong path. The mechanism is that coarse resolution and sinks misdirect flow accumulation. The false signal is a complete drainage network. The harm is a watershed boundary and drainage area that feed a hydrologic model the wrong inputs, corrupting peak-flow estimates.

### The Precise-Looking Result From Error-Compounding Steps

A chained analysis (buffer, overlay, reclassify, weight) produces a result shown to several decimals or as a crisp line, when the input error would shift the result substantially. The mechanism is that error propagates and compounds through operations. The false signal is the precise output number. The harm is a decision presented as analytically certain when the underlying uncertainty would change the conclusion, undermining the credibility of the analysis.

## Self-Check

- For every buffer, is the distance matched to the phenomenon and larger than the input's positional accuracy, and is the buffer basis (regulatory, analytical, heuristic) documented?
- For every overlay, are the input scales reconciled, slivers and boundary mismatches cleaned, and the error of the least accurate layer acknowledged in the output's accuracy?
- For network and suitability analysis, is the network topology validated (connectivity, restrictions, impedance), and are the suitability factors, standardization, and weights documented and sensitivity-tested?
- For watershed and drainage delineation, is the DEM resolution matched to the drainage scale, is the surface hydro-conditioned (sinks, culverts, road breaches, enforced streams), and is the network validated against observed drainage?
- For viewsheds and spatial joins, are the observer and target offsets, curvature and refraction corrections, and DEM resolution stated for the viewshed, and are the join rule and tolerance chosen deliberately for the spatial join?
- Have I tracked the lineage of inputs and operations, estimated the propagated error, and presented the result at a resolution consistent with its accuracy rather than overstating certainty?
