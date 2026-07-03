---
name: open-channel-and-floodplain-hydraulics.md
description: Use when the agent is designing or analysing open channels, computing water surface profiles by standard step method, determining floodplain boundaries and floodways, or evaluating channel lining, velocity, and scour for natural or engineered waterways. Applies before approving a channel modification or floodplain encroachment, while running HEC-RAS or equivalent models, and when reviewing flood insurance studies, bridge backwater, and stream-restoration hydraulics.
---

# Open-Channel and Floodplain Hydraulics

Open-channel hydraulics is the engineering of how water moves under gravity through a natural or constructed channel and across its floodplain, and it is the analysis that defines the water surface elevation that a levee must hold, the floodplain boundary that regulates development, and the velocity that a channel lining must withstand. The water surface profile is the output that drives flood mapping, bridge clearance, levee design, and flood insurance, and it depends on the channel geometry, roughness, slope, flow, and downstream conditions, all of which carry real uncertainty. The harm this skill prevents is a flood profile that is too low because the model was undercalibrated or the roughness was optimistically low, leading to a levee or a development that is overtopped at the design storm, or a channel lining that scours because the velocity was underestimated. Because floodplain decisions bind property and protect life, the hydraulics must be conservative, calibrated, and defensible.

## Core Rules

### Establish the Design Flow, Boundary Conditions, and Model Extent

The hydraulic model begins with the design flow (from the hydrologic analysis) at each cross section, the downstream boundary condition (a known water surface, a normal-depth slope, or a rating curve), and the model extent, which must extend far enough upstream and downstream that the project's effect is captured and the boundary conditions do not distort the result at the project site. Confirm that the model starts downstream of any tidal, confluence, or control-section influence and extends upstream beyond the backwater limit of the project. A model whose boundary condition is too close to the project site will produce a water surface that reflects the assumed boundary rather than the real hydraulics.

### Represent Channel and Floodplain Geometry Accurately

The cross sections are the geometry that defines the conveyance, and they must represent the real channel and floodplain. Survey cross sections perpendicular to the flow at intervals appropriate to the channel slope and the change in geometry, with closer spacing at bridges, culverts, bends, and transitions. Extend each cross section across the full floodplain to the highest water surface of interest, because a section that is too short cuts off storage and conveyance and produces an artificially high water surface. Account for channel overbank roughness differences, ineffective flow areas (where water is stored but not conveyed, such as behind a levee or in a pocket), and levees or roads that block flow. A model with sparse, truncated, or mislocated cross sections produces a profile that is precise to many decimal places and wrong in the value that matters.

### Select Roughness Values Conservatively and With Justification

Manning's roughness coefficient (n) is the single most sensitive input in a one-dimensional water surface profile, and it must be selected for the actual channel and floodplain condition, with documented justification. Use published references (photographs, tables, and verified calibrations) to select n for the channel (accounting for bed material, vegetation, irregularity, alignment, and obstructions) and for the overbank (accounting for vegetation, land use, and roughness). In the design case, use a roughness that represents the channel at its likely future, rougher condition (more vegetation, more debris), because channels mature and roughen over the design life. A roughness selected for the freshly constructed, smooth condition will underpredict the water surface as the channel weathers.

### Compute the Water Surface Profile by the Standard Step Method

For steady, gradually varied flow, compute the water surface profile by the standard step method (the method used in HEC-RAS and equivalent models), solving the energy equation between cross sections with friction and expansion or contraction losses. Confirm that the profile converges (the solution is not oscillating between subcritical and supercritical flow), that the flow regime is correctly identified, and that critical-section checks are performed where the flow may transition. For unsteady flow (where the hydrograph shape and routing matter, such as dam-break, tide, or rapidly changing flow), use the unsteady solver, because the steady profile does not capture the peak attenuation or the arrival time that govern the real flood.

### Calibrate and Validate the Model Against Observed High Water

Where observed high-water marks, gage records, or a published flood insurance study exist, calibrate the model by adjusting roughness and geometry within defensible ranges to reproduce the observed water surface, and validate against a different event. A model that cannot reproduce an observed flood when its inputs are within reasonable ranges has a geometry or roughness error that must be found before the model is used for design. A model that has never been calibrated to the real channel carries the full uncertainty of its assumed inputs, and a 20 percent error in roughness can translate to a significant error in water surface and floodplain boundary.

### Evaluate Bridges, Culverts, and Other Hydraulic Structures In-Channel

Bridges, culverts, and weirs constrict the flow and create backwater, and the model must represent them with the appropriate internal hydraulic (the bridge or culvert routines in HEC-RAS, with the correct deck low-chord, pier geometry, culvert barrel, and entrance and exit loss coefficients). Confirm that the contraction and expansion zones are represented by cross sections at the correct locations, that the bridge does not pressurise or overtop unexpectedly, and that the scour at piers and abutments is evaluated against the computed velocity. A bridge that is modelled as a simple constriction rather than with the bridge hydraulics will mispredict the backwater and the velocity at the foundations.

### Define the Floodway to Preserve Floodplain Conveyance

The floodway is the channel and adjacent floodplain that must be kept open to carry the design flood without raising the water surface more than the regulatory allowance (commonly 0.3 m). The floodway is defined by encroaching the floodplain from both sides until the allowable rise is reached, and it is the zone in which development is restricted. Confirm that the floodway is computed against the design flood (commonly the 100-year event), that the encroachment does not raise the water surface above the allowable at any cross section, and that the floodway is consistent with the regulatory flood insurance study. A floodway narrowed to accommodate development reduces conveyance and raises the base flood elevation, transferring flood risk to other properties.

### Address Velocity, Lining, and Scour

The water surface is only half the design; the velocity determines whether the channel erodes. Compute the velocity at each cross section and compare to the permissible velocity for the channel material (soil, grass lining, riprap, or concrete). Where the velocity exceeds the permissible, design a lining or grade control, sized by the tractive force or the permissible shear stress, not by rule-of-thumb rock size. At structures and bends, evaluate local scour (pier, abutment, and bend scour) against the computed velocity and provide bed and bank protection sized for the design event. A channel whose water surface is controlled but whose velocity is not will scour its bed and banks, undermining the lining and the structures.

## Common Traps

### The Roughness Selected For The New Channel

A channel is constructed and the designer uses the smooth, freshly graded roughness to compute the design water surface, because that is the condition at handover. The trap is that the profile is correct for the day of completion, while the channel will vegetate, accumulate debris, and roughen over its design life, raising the real water surface above the design value. The false signal is the defensible, as-built roughness; the harm is a channel or levee that is overtopped at the design storm years after construction, because the roughness matured and the profile rose.

### The Cross Sections Too Far Apart

A model uses widely spaced cross sections, especially across a reach of varying geometry, because survey was minimised. The trap is that the model runs and produces a smooth profile, while the missed cross sections hide real expansions, contractions, and channel bends that the energy equation must evaluate, and the profile is wrong at the project site even if it looks reasonable in the output. The false signal is the smooth, converged profile; the harm is a water surface that does not represent the real channel, with floodplain or levee decisions made on a profile that missed the controlling geometry.

### The Uncalibrated Model On A Gaged Reach

A model is built for a reach with a stream gage and observed high-water marks, but the model is not calibrated, and the assumed roughness produces a water surface that disagrees with the observation. The trap is that the model is precise and defensible-looking, while the disagreement with the observed flood reveals a geometry or roughness error that the calibration would have found. The false signal is the completed, documented model; the harm is a flood profile used for levee or floodplain decisions that the observed data already contradicted.

### The Bridge Modelled As A Constriction

A bridge is represented in the model as a simple narrowed cross section rather than with the bridge hydraulics, because the bridge routine is more effort to set up. The trap is that the model produces a backwater, while the real bridge with its deck, piers, and possible overtopping or pressurisation produces a different and usually larger backwater, and the scour at the piers is not evaluated. The false signal is the backwater that the simple model predicts; the harm is a bridge whose real backwater floods upstream property or whose pier scour undermines the foundation at the design event.

### The Floodway Narrowed For Development

A floodway is encroached to the regulatory limit to allow development on the floodplain fringe, and the resulting rise in the base flood elevation is accepted up to the allowable. The trap is that the encroachment complies with the rule, while it has transferred the flood risk to other properties in the reach, whose base flood elevation now sits at the raised regulatory level rather than the natural level. The false signal is the compliant floodway; the harm is upstream or downstream property that now floods at the design storm where it did not before, with the cause hidden in the regulatory tolerance.

### The Velocity That Scours The Lining

A channel lining is sized for the design flow and depth, but the velocity, not the depth, governs the lining stability, and the velocity at the design event exceeds the permissible shear for the specified lining. The trap is that the channel carries the flow on paper, while the lining is torn out and the bed and banks scour at the first design storm, because the lining was sized by flow and not by tractive stress. The false signal is the adequate capacity; the harm is channel failure that undermines adjacent structures and requires reconstruction under emergency conditions.

## Self-Check

- Are the design flow, downstream boundary condition, and model extent established so that the boundary does not distort the profile at the project site?
- Are cross sections surveyed perpendicular to flow at appropriate spacing, extended across the full floodplain, with ineffective flow and levees represented?
- Are Manning's n values selected for the actual and likely future channel condition, with documented justification and a conservative (rougher) design value?
- Is the profile computed by the standard step method (steady) or the unsteady solver (where routing matters), with flow regime and convergence checked?
- Is the model calibrated or validated against observed high water or gage data, and are disagreements investigated before the model is used for design?
- Are bridges, culverts, and weirs modelled with the appropriate internal hydraulics, with backwater, overtopping, and pier and abutment scour evaluated?
- Is the floodway defined against the design flood, with the encroachment rise within the regulatory allowance at every cross section?
- Is the channel velocity compared to the permissible velocity or shear for the lining, with lining or grade control designed where the velocity exceeds the limit?
