---
name: gis-data-model-and-projection-selection.md
description: Use when the agent is selecting a GIS data model for civil work, choosing between vector and raster representations, selecting a horizontal or vertical datum and coordinate system such as NAD83 or NAVD88, choosing a map projection such as UTM or State Plane, defining topology rules, or setting scale and accuracy standards for a geospatial dataset used in civil engineering and surveying.
---

# GIS Data Model and Projection Selection

A GIS data model and a coordinate system are the foundations of any geospatial civil dataset, and the judgment problem is that they look like setup choices but determine what the data can represent, how accurately it locates features, how it can be analyzed, and whether it can be combined with other data without distortion or error. Agents tend to pick a default coordinate system and a familiar data type and miss that a vector model captures discrete parcels and pipes while a raster model captures continuous terrain, that a wrong projection distorts distances and areas across a project, that mixing datums shifts every feature by feet or meters, and that topology rules are what make a utility network analyzable rather than just drawable. The harm this skill prevents is a dataset that mislocates infrastructure, an analysis that measures distance in a distorted projection, layers that do not align because they use different datums, and a model that cannot support the civil decisions it was built for. This skill covers vector and raster data models, datums and coordinate systems, projections, topology, and scale and accuracy standards. It is a geospatial-engineering discipline, and the agent must confirm the datum, the projection, and the accuracy class before relying on any GIS layer for design or decision.

## Core Rules

### Match the Vector or Raster Data Model to the Phenomenon

A GIS data model represents reality in one of two principal ways, and the choice determines what the data captures well. A vector model represents discrete features as points (valves, manholes, survey monuments), lines (pipes, roads, property lines), and polygons (parcels, basins, zoning), and it is suited to infrastructure, boundaries, and networks where identity and connectivity matter. A raster model represents continuous surfaces as a grid of cells with values (elevation, slope, land cover, rainfall), and it is suited to terrain, hydrology, and phenomena that vary continuously across space. Choose the model by the phenomenon and the analysis: parcels and utilities as vector, terrain and watersheds as raster, and often both, with a digital elevation model (DEM) supporting vector drainage design. The disciplined rule is to recognize that the model constrains the analysis, because network tracing requires vector topology, and surface analysis requires raster, and choosing the wrong model forces costly conversion or blocks the analysis. The trap is forcing a continuous phenomenon into vector or a discrete network into raster, losing the structure the analysis needs.

### Select the Datum and Coordinate System and Document It

Every geospatial dataset is referenced to a datum and a coordinate system, and these must be defined and consistent across all layers. The horizontal datum defines the model of the earth's surface (NAD83 and its realizations such as NAD83(2011), or the newer NAD83(2011) epoch and the upcoming NATRF2020), and the vertical datum defines elevations (NAVD88 via GEOID models, or the newer NAPGD2020), and mixing datums shifts features by amounts that matter in civil work. Choose a coordinate system: geographic coordinates (latitude and longitude in degrees) preserve position but distort distance and area for measurement, and projected coordinates (state plane, UTM, or a local coordinate system) flatten the earth to a plane for measurement in linear units. The disciplined rule is to define the datum and projection in the dataset's metadata, to transform all layers to a common system before analysis, and to document the transformation used, because an undefined or assumed coordinate system is a latent error. The trap is overlaying layers in different datums or projections without transformation, so features appear to align on screen but are shifted by feet, producing mislocated design and construction.

### Choose a Projection That Controls Distortion for the Project's Extent

A map projection flattens the earth to a plane, and every projection distorts something: distance, area, shape, or direction. Choose the projection by the project's extent and the property that must be preserved. For a local or state project, a State Plane Coordinate System zone minimizes distortion across its designed extent and is the standard for surveying and engineering, preserving distance and area within the zone. For a regional or continental project, UTM zones (six degrees wide) provide a consistent system but with greater distortion at zone edges. For area-critical analysis such as watershed or parcel area, choose an equal-area projection so areas are preserved, and for distance-critical analysis such as pipeline length, choose an equidistant projection or measure on the ellipsoid. The disciplined rule is to know which property each projection preserves and distorts, to keep large or linear projects within a single zone or to account for zone-boundary issues, and to measure in the projected coordinate system only when its distortion is acceptable for the tolerance. The trap is measuring distances or areas in a geographic coordinate system or in a projection whose distortion exceeds the project's tolerance, producing lengths and areas that are wrong by percent-level margins.

### Define Topology to Make Networks and Adjacencies Analyzable

Topology is the set of rules that define spatial relationships (connectivity, adjacency, containment) and that make a dataset analyzable rather than merely drawable, and it is essential for utility networks, parcels, and hydrology. Define connectivity rules so that pipes connect to nodes, that valves sit on pipes, and that the network can be traced from source to outlet; define adjacency rules so that parcels share boundaries without gaps or overlaps; and define containment rules so that structures contain the features within them. Enforce topology through validation that flags errors (dangles, overlaps, gaps, multipart errors) and through editing that resolves them, because a utility network with topology errors cannot be traced reliably and a parcel fabric with gaps and overlaps cannot support ownership. The disciplined rule is to build topology for any network or fabric that will be analyzed, to validate it, and to treat topology errors as defects to fix, not cosmetic issues. The trap is a "drawn" network of lines that looks connected but has tiny gaps and overshoots, so a trace fails or follows the wrong path, and the analysis is unreliable.

### Set Scale and Accuracy Standards Matched to the Decision

Every dataset has a scale (the level of detail it represents) and an accuracy (how close its positions are to truth), and these must be matched to the decision the data supports. A dataset compiled at 1:24,000 has a National Map Accuracy Standard positional accuracy of roughly 40 feet, suitable for planning but not for design, while survey-grade GIS data at centimeter accuracy supports design and staking. Define the required accuracy class for the application (control survey, design base map, planning, inventory) and select or compile data to that class, and document the accuracy in the metadata so users know the limits. The disciplined rule is to never use a dataset at a scale or accuracy finer than its source supports, because zooming in does not add accuracy, and to flag when a planning-scale layer is being used for a design decision. The trap is using a coarse, planning-scale layer (such as a national hydrography dataset or a USDA soils map) for a design decision that needs survey-grade accuracy, producing locations and boundaries that are wrong by tens of feet.

## Common Traps

### Mixing Datums or Projections Without Transformation

Layers in different datums (NAD83 vs. WGS84) or projections are overlaid without transformation, and they appear to align on screen but are shifted. The mechanism is that the same coordinates mean different positions under different datums. The false signal is visual alignment. The harm is features mislocated by feet or meters, with design and construction placed in the wrong position.

### Measuring in a Distorted or Geographic Coordinate System

Distances or areas are measured in latitude-longitude degrees or in a projection whose distortion exceeds the tolerance. The mechanism is that geographic coordinates are angular and projected coordinates distort by location. The false signal is a measured number on screen. The harm is lengths and areas wrong by percent-level margins, invalidating quantities and designs.

### Forcing the Wrong Data Model and Losing Structure

A continuous terrain is forced into vector contours, or a utility network is stored as unstructured raster or loose lines, losing the structure the analysis needs. The mechanism is that the data model constrains the possible analysis. The false signal is a dataset that displays. The harm is blocked network tracing, failed surface analysis, or costly conversion to rebuild the correct model.

### The Drawn Network With Hidden Topology Errors

A utility network is drawn as lines that look connected but have dangles, overshoots, and gaps, and a trace fails or follows the wrong path. The mechanism is that visual connectivity is not topological connectivity. The false signal is a connected-looking map. The harm is an unreliable network analysis that misidentifies flow paths, service areas, or isolation.

## Self-Check

- Did I choose the data model (vector for discrete parcels, pipes, and boundaries; raster for continuous terrain and surfaces) based on the phenomenon and the required analysis, rather than convenience?
- Is every layer referenced to a defined horizontal datum (e.g., NAD83 realization) and vertical datum (e.g., NAVD88 via a GEOID), and are all layers transformed to a common system before overlay and analysis?
- Is the projection (State Plane, UTM, or other) selected to control distortion for the project's extent, preserving the property (distance, area) the analysis needs, and is measurement done only where distortion is within tolerance?
- For any network, parcel fabric, or hydrologic model, did I define and validate topology rules (connectivity, adjacency, containment), resolve dangles, overlaps, and gaps, and treat topology errors as defects?
- Is the dataset's scale and accuracy class (control survey, design base map, planning, inventory) matched to the decision it supports, documented in metadata, and have I avoided using a planning-scale layer for a design-grade decision?
- Have I documented the datum, projection, transformation method, accuracy class, and source in the metadata so that the dataset's limits are knowable to every user?
