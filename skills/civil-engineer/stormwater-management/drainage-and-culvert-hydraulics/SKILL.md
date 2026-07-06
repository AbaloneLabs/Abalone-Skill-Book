---
name: drainage-and-culvert-hydraulics.md
description: Use when the agent is sizing a roadway or railway culvert, performing FHWA HEC-style inlet and outlet control analysis, computing headwater and tailwater, selecting Manning n, checking scour and energy dissipation, or verifying that a culvert passes the design storm without overtopping the roadway.
---

# Drainage and Culvert Hydraulics

Drainage and culvert hydraulics is the engineering of an enclosed conduit that passes a stream or roadside drainage under a roadway, railway, or embankment, so that the design storm flows through without overtopping the roadway and without scouring the channel or the embankment at the outlet. A culvert is governed by two distinct flow regimes, inlet control where the entrance geometry limits the capacity and outlet control where the barrel friction and tailwater limit it, and the controlling regime is whichever produces the higher headwater for the given design flow. The harm this skill prevents is a culvert sized for the average year that overtops the road at the design storm, a barrel sized by diameter alone that drowns the inlet under high tailwater, or an outlet that passes the flow but scours an unlined channel into a headcut that migrates upstream and undermines the culvert. Because culverts under roads are life-safety infrastructure and their failure cuts access and endangers travelers, the design must be defensible against the design storm, the tailwater condition, and the scour that follows.

## Core Rules

### Establish the Hydrology, the Design Storm, and the Allowable Headwater

The culvert design begins with the design flow from the contributing watershed, computed by the approved hydrologic method (the rational method for small urban watersheds or the NRCS TR-55 method for larger rural basins), with the drainage area, runoff coefficient or curve number, and time of concentration reflecting the actual contributing conditions. Establish the design storm and the check storm the culvert must pass, commonly the 25- or 50-year event for the design and the 100-year event for the check, with the requirement that the design storm not overtop the roadway and the check storm be passed without catastrophic failure. Establish the allowable headwater, typically limited to a fraction of the culvert height or to the roadway subgrade elevation with freeboard, because the headwater is the pool that forms upstream and the measure of whether the road overtops. These three define the sizing target; without them the culvert is sized arbitrarily.

### Analyse Both Inlet and Outlet Control and Use the Controlling Headwater

Compute the headwater for the design flow under both inlet control and outlet control, and use the higher of the two as the governing headwater, because the culvert operates in whichever regime produces the greater upstream depth. For inlet control, use the FHWA inlet control nomographs or equations for the actual entrance type (projecting, mitered, headwall with or without wingwalls, and bevelled edge), because the entrance geometry and edge condition govern the capacity and a square-edged entrance passes far less than a bevelled one. For outlet control, sum the entrance loss, the barrel friction loss computed by Manning's equation with the appropriate roughness for the material, and the velocity head loss, and add the tailwater depth to obtain the headwater. Use the actual tailwater from the downstream channel or the critical depth in the barrel, whichever is higher, because a high tailwater drowns the outlet and shifts the culvert into outlet control. A culvert analysed under only one regime may be undersized by the other.

### Select the Barrel Material, Shape, Slope, and Manning n

Select the culvert material and shape suited to the site: concrete pipe or box for durability and structural cover, corrugated metal or HDPE for economy and flexibility, and arch or oval shapes where vertical clearance is limited. Select the Manning n appropriate to the material and corrugation, because the roughness governs the outlet-control capacity and the wrong n understates or overstates the headwater. Set the barrel slope to match the natural channel where possible to avoid deposition or scour, and confirm that the slope produces a velocity at the outlet that the downstream channel can withstand. Confirm the minimum cover over the culvert for the traffic loading and the structural class, because inadequate cover crushes the barrel under the roadway load. Provide multiple barrels only where the hydraulics and the debris and ice conditions support them, because a multi-barrel installation can accumulate debris at one barrel and lose capacity.

### Provide Outlet Protection, Energy Dissipation, and Scour Countermeasures

The culvert outlet concentrates the flow at a velocity higher than the natural channel, and without protection it scours a plunge pool that undercuts the culvert and a headcut that migrates upstream. Size the outlet protection by the outlet velocity and the design flow: a riprap apron sized by the unit discharge and the tailwater, a stilling basin or energy dissipator where the velocity is high or the tailwater is low, or a FHWA-designed impact or baffled basin where the energy must be dissipated before the channel. Confirm that the riprap gradation and thickness are suited to the velocity and that the apron length and width prevent flank erosion. At the inlet, provide a headwall, wingwalls, and a bevelled or slope-tapered entrance to improve capacity and prevent inlet scour and seepage along the barrel. A culvert that passes the flow but lacks outlet protection fails the channel and the embankment within the first few storms.

### Check Roadway Overtopping, Flood Hazard, and Downstream Effects

For the design and check storms, confirm that the headwater does not overtop the roadway at the low point, and where overtopping of the check storm is allowed, analyse the roadway as a broad-crested weir to confirm the overtopping depth and the downstream scour are tolerable. Assess the flood hazard to upstream property from the headwater pool, because the culvert backs water up and the increased flood stage may inundate land or structures that the natural channel did not. Evaluate the downstream effects of concentrating the flow, because a culvert that passes the design storm but releases it at a higher rate or in a more concentrated plume can erode the downstream channel or worsen flooding at a downstream crossing. A culvert sized for the barrel alone, without the overtopping, hazard, and downstream checks, may pass the hydraulics but fail the public it is meant to serve.

## Common Traps

### The Single-Regime Analysis

The culvert is analysed under inlet control only, or under outlet control only, and the other regime is not checked. The trap is that the computed headwater is adequate for the analysed regime, while the unanalysed regime produces a higher headwater and the culvert overtops the road at the design storm. The false signal is the compliant headwater; the harm is a roadway overtopping at a storm the culvert was supposed to pass, because the controlling regime was never identified.

### The Tailwater Ignored

The outlet-control headwater is computed using the barrel geometry and friction without the actual downstream tailwater, or assuming a low tailwater. The trap is that the headwater is low on paper, while the real tailwater from the downstream channel or a converging stream drowns the outlet, raises the headwater, and overtops the road. The false signal is the computed headwater; the harm is a culvert that fails under the high-tailwater condition that recurs every flood season.

### The Outlet Without Protection

The culvert barrel is sized to pass the design flow but the outlet discharges to an unprotected channel at a velocity far above the natural condition. The trap is that the culvert passes the flow, while the concentrated jet scours a plunge pool, undercuts the barrel end, and initiates a headcut that migrates upstream and threatens the embankment. The false signal is the adequate barrel capacity; the harm is culvert failure by outlet scour within the first few storms, traced to the protection that was never designed.

### The Manning n for the Wrong Material

The headloss is computed with a Manning n that does not match the actual barrel material or corrugation, often a smooth-pipe value applied to corrugated metal. The trap is that the outlet-control headwater is understated, while the real roughness produces a higher friction loss and a higher headwater that overtops the road. The false signal is the computed headwater; the harm is a culvert undersized by a roughness value that misrepresented the barrel, a mistake invisible until the design storm arrives.

## Self-Check

- Is the design flow computed by the approved method for the actual contributing watershed, and are the design and check storms and the allowable headwater established?
- Are both inlet control and outlet control analysed, and is the higher headwater used as the governing value for the design and check storms?
- Is the actual tailwater from the downstream channel or converging stream used in the outlet-control analysis, with critical depth checked as the alternative?
- Is the barrel material, shape, slope, and Manning n appropriate, and is the minimum cover provided for the traffic loading and structural class?
- Is outlet protection or an energy dissipator sized for the outlet velocity and design flow, with inlet headwall, wingwalls, and seepage control provided?
- Does the headwater avoid roadway overtopping at the design storm, and are the upstream flood hazard and the downstream concentration effects checked?
