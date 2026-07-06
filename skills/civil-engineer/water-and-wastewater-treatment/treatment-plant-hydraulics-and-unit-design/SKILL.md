---
name: treatment-plant-hydraulics-and-unit-design.md
description: Use when the agent is developing the hydraulic profile through a water or wastewater treatment plant, computing headloss across weirs, gates, filters, and clarifiers, sizing unit processes with redundancy, or verifying that peak flows pass without submergence, bypass, or overflow under n+1 operating conditions.
---

# Treatment Plant Hydraulics and Unit Design

Treatment plant hydraulics and unit design is the engineering of the flow path through a plant so that water or wastewater moves from the headworks to the effluent by gravity or controlled pumping at every flow rate, while each unit process is sized to perform its function and is provided with the redundancy that keeps the plant in service when a unit is taken offline. The hydraulic profile is the backbone of the plant: if the water levels are wrong, a downstream basin submerges an upstream filter, an overflow bypasses untreated flow around a process, or a pump station runs dry. The harm this skill prevents is a plant that treats the design flow on paper but floods the basins at the peak hour, a unit sizing that meets the average loading but fails the peak, or a redundancy scheme that is theoretical because the hydraulics cannot actually split flow to the remaining units when one is out of service. Because treatment plants are essential services and permit compliance depends on every unit performing at the peak, the hydraulic and unit design must be defensible against the peak flow with the worst unit out.

## Core Rules

### Establish the Design Flows, the Hydraulic Profile, and the Governing Downstream Condition

The hydraulics begin with the design flow cases: the average day, the maximum day or peak hour, the minimum flow, and the peak instantaneous flow that the plant must pass without bypass. Establish the downstream controlling water level, whether it is the receiving water at flood stage, the effluent channel, or the discharge pipe, because the entire hydraulic profile is built up from that tailwater and every upstream basin must clear it with freeboard. Compute the water surface profile from the downstream control backward through each unit, summing the headloss across weirs, flumes, gates, screens, filters, and basins, and confirm that at peak flow each basin has the required freeboard and no process is submerged or reversed. A profile computed only at average flow hides the submergence that occurs at the peak, and a profile built from the upstream end without the downstream control produces water levels that cannot actually discharge.

### Compute Headloss Across Each Unit and Account for All Losses

Headloss through a treatment plant accumulates across many small elements that are easy to overlook individually but collectively determine the profile. Account for the loss through bar screens and fine screens (which clog and raise the loss), Parshall or other flumes (with the submergence transition), weirs and launder orifices (with the submergence correction), gates and valves, static mixers, filter media and underdrains (clean and at terminal headloss), clarifier inlet and outlet structures, and the friction and minor losses in channels and pipes. Use the appropriate equation for each: the orifice and weir equations at the actual head, the friction loss by Manning or Darcy, and the manufacturer loss curves for proprietary units. Confirm that the filters are checked at terminal headloss, not clean media, because the headloss that governs the profile is the maximum that occurs just before backwash. A profile that omits the clogged screen or the dirty filter understates the upstream water level and invites flooding.

### Size Each Unit Process for the Loading and the Redundancy Requirement

Each unit process must be sized for the design loading and provided with redundancy so that the plant maintains treatment when a unit is out of service for maintenance or repair. Apply the Ten States Standards redundancy principle: for critical processes, provide capacity such that with the largest unit out of service the remaining units meet the design flow, commonly expressed as n+1 for pumps and multiple basins for filters and clarifiers. Size filters for the filtration rate with one filter out of backwash or service, clarifiers for the surface overflow and solids loading with one basin out, and aeration for the peak demand with the largest blower out. Verify that the hydraulic splitting structures (inlet flumes, weirs, or flow-splitting boxes) actually divide the flow equally among the operating units, because an unbalanced split overloads one unit and starves the others, defeating the redundancy.

### Provide Bypass, Overflow, and Drain-Down Provisions

Every basin and unit must have a controlled path for the flow that exceeds its capacity and for the water that must be removed for maintenance. Provide an overflow weir or bypass channel at each basin sized to pass the peak flow without flooding the basin or the plant, and route the bypass to a point that does not violate the permit (a bypass around a primary or secondary process that discharges untreated flow is a permit violation unless explicitly allowed). Provide drain-down and pump-out for each basin so that it can be dewatered for inspection and repair, with the dewatering pump and the receiving point defined. Confirm that the bypass and overflow paths do not create a cross-connection between treated and untreated water, and that the plant can be taken through a unit outage without losing the hydraulic profile or the treatment train.

### Design Pump Stations with Wet-Well Sizing, Redundancy, and Control

Where pumping lifts the flow between hydraulic grades or feeds the plant, size the wet well for the cycle time that avoids motor cycling and for the active volume that prevents deposition, and select the pumps with the firm capacity (the largest pump out of service) meeting the peak flow. Provide variable-speed or staged pumping to match the influent flow across the diurnal range without surging the downstream units, and confirm that the pump start and stop levels keep the upstream sewer from surcharging and the downstream flow within the capacity of the units. Provide standby power and a backup control so that the station operates through a power or instrument failure, because a lift station failure backs up the collection system and can flood upstream.

## Common Traps

### The Profile Computed at Average Flow

The hydraulic profile is developed at the average or design average flow, and the peak flow case is not checked. The trap is that the profile is adequate at the average, while at the peak hour the headloss across each element rises, the water levels back up, and an upstream basin submerges or a process is bypassed. The false signal is the adequate average-flow profile; the harm is a plant that floods or bypasses at the peak hour it was designed to pass, because the worst case was never verified.

### The Redundancy That Cannot Split Flow

Multiple basins or filters are provided for n+1 redundancy, but the flow-splitting structure does not divide the flow equally when one unit is out. The trap is that the redundancy exists on the layout, while the hydraulics overload the remaining operating units or starve them, so the redundancy is theoretical. The false signal is the redundant unit count; the harm is a plant that loses treatment when one unit is taken out because the flow cannot actually be balanced to the remaining units.

### The Headloss at Clean Instead of Dirty Filter

The hydraulic profile uses the clean-media filter headloss, and the dirty or terminal headloss is not checked. The trap is that the profile clears at the start of a filter run, while at terminal headloss just before backwash the loss is far higher and the upstream water level rises to flood the basin or reverse the flow. The false signal is the clean-filter profile; the harm is periodic flooding or filter upset at the end of each run, traced to the headloss value that was never checked at the governing condition.

### The Bypass That Discharges Untreated Flow

An overflow or bypass is provided to protect the basins from flooding, but it routes untreated or partially treated flow to the effluent or receiving water without permit coverage. The trap is that the bypass protects the plant hydraulically, while discharging through it is a permit violation and a water-quality failure. The false signal is the protected basin; the harm is an unpermitted discharge that triggers an enforcement action the first time the bypass is used in earnest.

## Self-Check

- Are the design flow cases (average, peak hour, peak instantaneous, minimum) established, and is the hydraulic profile built from the governing downstream tailwater backward through each unit?
- Is the headloss across every element (screens, flumes, weirs, gates, filters, clarifiers, channels) computed and summed, with filters checked at terminal headloss and screens at the clogged condition?
- Does each unit process meet the design loading with the largest unit out of service, and do the flow-splitting structures divide the flow equally among operating units?
- Are overflow and bypass paths provided and sized for peak flow, routed to a permitted point, and free of cross-connections between treated and untreated water?
- Are pump station wet wells sized for cycle time and active volume, with firm capacity, variable or staged control, standby power, and backup controls?
- At peak flow with the worst unit out, does every basin have the required freeboard, and is no process submerged, reversed, or bypassed?
