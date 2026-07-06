---
name: stormwater-detention-and-retention-design.md
description: Use when the agent is sizing a stormwater detention or retention basin, applying the NRCS TR-55 method or the rational method, selecting SCS curve numbers and design storms, routing an inflow hydrograph through storage, or designing multi-stage orifice and weir outlets to meet peak attenuation and water-quality release rates.
---

# Stormwater Detention and Retention Design

Stormwater detention and retention design is the engineering of a basin that stores the runoff a development generates and releases it at a rate and in a manner that does not worsen downstream flooding and that protects the receiving stream. Development replaces permeable vegetated ground with impervious surfaces, multiplying both the volume and the peak rate of runoff and shortening the time of concentration, so the basin must compensate by holding the difference between the post-development hydrograph and the allowable release. The harm this skill prevents is a basin that passes the design storm on paper but overtops at the real storm because the hydrograph was never routed, an outlet sized by peak subtraction that releases more than the pre-development rate, or a retention pond whose water-quality drawdown fails because the low-flow orifice clogs. Because stormwater permits are enforceable and downstream flooding is a liability, the basin must be defensible against the design storm, the emergency spillway condition, and the regulator who audits the routing.

## Core Rules

### Establish the Permit Release Rates and Design Storms Before Sizing

Before any basin sizing, establish the governing stormwater regulation and the release-rate and design-storm requirements it imposes. Most rules require that the post-development peak discharge not exceed the pre-development rate for one or more design storms, commonly the 1-, 2-, 10-, 25-, and 100-year events, and some additionally require control of the channel-forming or one-year storm to protect the receiving channel from erosion. Confirm the rainfall distribution and depth from the approved source (NRCS Type II or local intensity-duration-frequency data) and the design storm duration. Establish the allowable release rate for each controlled storm, because these rates are the constraints that the routing must satisfy. Without these rates and storms fixed and documented, the basin sizing is arbitrary and the design cannot be audited against the permit.

### Characterise Pre- and Post-Development Hydrology by the Approved Method

Compute the pre-development and post-development peak flows and hydrographs by the approved hydrologic method, with separate drainage areas, curve numbers or runoff coefficients, and times of concentration for each condition. For the NRCS TR-55 method, select the curve number from the actual hydrologic soil group and cover, and compute the time of concentration by the sheet, shallow concentrated, and channel flow segments, not a single assumed value. Characterise the pre-development condition realistically as the meadow or woods condition the rule specifies, not an inflated baseline that makes the post-development condition look compliant. Update the curve numbers and times of concentration for the post-development condition to reflect all proposed impervious area and the actual proposed cover. The required storage is the difference between the post-development inflow hydrograph and the allowable release, and it cannot be determined without both hydrographs computed correctly.

### Route the Inflow Hydrograph Through the Basin Storage

Size the basin by routing the post-development design-storm hydrograph through the proposed storage using the storage-indication or level-pool routing method, not by a simplified peak-flow subtraction. The routing accounts for the fact that the basin fills as it releases, that the outlet discharge varies with the rising and falling head, and that the storage available at each elevation determines the attenuation. Confirm that the routed peak outflow does not exceed the allowable release rate at each design storm, that the basin does not overtop at the largest design storm, and that the maximum water surface stays within the basin footprint with freeboard. Build the stage-storage curve from the actual basin grading and the stage-discharge curve from the outlet hydraulics, and iterate the basin size and outlet until the routing satisfies every storm. A basin sized by peak subtraction alone is undersized because it ignores the volume above the release rate accumulated over the storm duration.

### Design the Multi-Stage Outlet and the Emergency Spillway

Most basins must control multiple design storms with one outlet structure, achieved by a multi-stage arrangement: a low-flow or water-quality orifice at the base, an orifice or weir at the 1- or 2-year stage, a larger opening or weir at the 10-year stage, and the emergency spillway at the 100-year stage. Size each opening by the orifice or weir equation at the head appropriate to its stage, accounting for submergence where the tailwater rises, and confirm that the combined discharge at each design-storm peak does not exceed the allowable release for that storm. Verify that the low-flow orifice releases the water-quality volume over the required drawdown time (commonly 24 to 48 hours) to settle pollutants and avoid prolonged standing water. Provide an emergency spillway sized to pass the design storm or the storm beyond it safely if the principal outlet clogs, because outlet clogging is a common failure and the spillway is the last line of defence against catastrophic embankment failure or overtopping. Protect each orifice with a trash rack whose open area far exceeds the orifice area.

## Common Traps

### The Peak Subtraction That Ignores Routing

The required storage is computed as the simple difference between the post-development peak flow and the allowable release multiplied by the storm duration, without routing the hydrograph. The trap is that the arithmetic produces a storage volume that looks defensible, while the actual basin must store the entire rising hydrograph above the release rate, which is far larger than the peak-subtraction estimate. The false signal is the completed dimensioned basin; the harm is a basin that overtops at the design storm because it was undersized by a method that does not represent the physics of storage.

### The Inflated Pre-Development Baseline

The pre-development condition is characterised with a high curve number or an already-developed cover, so the pre-development peak is high and the required attenuation is small. The trap is that the analysis complies with the release-rate rule on paper, while the baseline is inflated and the post-development release actually exceeds the true pre-development rate, worsening downstream flooding. The false signal is the compliant-looking release rate; the harm is downstream property that floods more severely than before development, with the liability traced to the mischaracterised baseline.

### The Low-Flow Orifice That Clogs

The low-flow or water-quality orifice is small and is protected by a trash rack with insufficient open area, or by none at all. The trap is that the outlet passes the design flow when clean, while in service the orifice clogs with debris, sediment, or vegetation, and the basin fills to the next stage or to the spillway, releasing at a rate far above the permitted value or overtopping. The false signal is the hydraulic adequacy of the clean orifice; the harm is an uncontrolled basin that floods downstream or fails the embankment at a storm the design was supposed to pass.

### The Single-Storm Design That Misses the Water-Quality Volume

The basin is sized and routed for the 10- or 25-year peak attenuation, but the water-quality volume and its drawdown time are not separately checked. The trap is that the basin controls the large storm, while the small frequent storms that carry most of the pollutant load pass through too quickly for treatment, and the receiving stream receives the full pollutant load of the development. The false signal is the compliant peak control; the harm is water-quality permit noncompliance and the degradation of the receiving stream that the water-quality requirement was meant to prevent.

## Self-Check

- Are the governing permit release rates and design storms established and documented, including the rainfall source and the storms that must be controlled?
- Are the pre- and post-development hydrology computed separately by the approved method, with curve numbers and times of concentration reflecting the actual soil, cover, and proposed impervious area?
- Is the basin sized by routing the design-storm hydrograph through the storage, with the routed outflow verified against the allowable release at each storm and the maximum stage within freeboard?
- Is the outlet multi-stage, with each orifice or weir sized for its design storm, the water-quality orifice checked for drawdown time, and each protected by an adequately sized trash rack?
- Is an emergency spillway provided and sized for the design storm or beyond, protecting the embankment against overtopping if the principal outlet clogs?
- Is the downstream effect checked to the point where the site discharge no longer affects the receiving water, confirming the release does not synchronise with a downstream tributary peak?
