---
name: water-distribution-and-pipeline-hydraulics.md
description: Use when the agent is designing water distribution or pressure pipeline systems, sizing mains and pumps, computing headloss by Hazen-Williams or Darcy-Weisbach, setting pressure zones, or evaluating fire flow, surge, and water-age in municipal or industrial networks. Applies before approving a pipe network layout, while running extended-period simulation, and when reviewing pressure, storage, and reliability under peak demand and contingency conditions.
---

# Water Distribution and Pipeline Hydraulics

Water distribution design is the engineering of a pressurised network that must deliver adequate flow and pressure to every service connection, at every hour, under peak demand and under the failure of any single component, for the life of the system. The network must satisfy three simultaneous demands: the domestic, commercial, and industrial demand at peak hour; the fire-flow demand at the required flow and residual pressure; and the contingency demand when a main is taken out for repair. The harm this skill prevents is a network that meets average demand but collapses under peak or fire flow, that contaminates or loses water through low-pressure or stagnant zones, or that ruptures from pressure surge because the transient was never analysed. Because water systems are essential services and buried infrastructure, the design must be robust against the failure modes that the public never sees but the operator lives with.

## Core Rules

### Establish Demand, Peaking Factors, and Fire Flow Requirements

The design begins with the average day demand, projected over the design life, distributed across the service area by land use and meter data. Apply peaking factors to obtain the maximum day and peak hour demands, using recognised criteria or local data, because the network must serve the peak, not the average. Establish the fire-flow requirement for each pressure zone and hydrant district, from the fire code or the insurance services office schedule, including the required flow (commonly 1,900 to 12,000 L/min depending on building type and separation) and the minimum residual pressure (commonly 140 kPa at the hydrant under maximum day plus fire flow). These three demands (peak hour, maximum day plus fire, and contingency) are the load cases the network must satisfy simultaneously with adequate residual pressure at every node.

### Configure Pressure Zones, Storage, and Pumping for Reliability

Lay out the system in pressure zones that maintain the service pressure within the acceptable range (commonly 280 to 550 kPa at the service connection, with tighter limits for low-rise and high-rise service) across the topographic range of the zone. Provide storage (elevated tanks or ground storage with booster pumping) sized for equalisation (to buffer the peak above the average supply rate), fire storage (to supply the fire flow for the required duration), and emergency storage (to cover supply or pump failure). Size pumping capacity with redundancy, so that the largest pump can fail and the remaining pumps still meet the maximum day demand, and provide standby power so that the station operates through a power outage. A system without storage or redundancy fails the first time a pump trips or a main breaks.

### Size Mains by Headloss, Velocity, and Pressure

Size each main so that the headloss under peak flow keeps the residual pressure at every node within the acceptable range, with velocity commonly limited to 1 to 2 m/s under peak hour to limit headloss and surge, and higher only in large transmissions mains where the analysis supports it. Compute headloss by the appropriate method: Hazen-Williams for typical municipal water mains (with C values appropriate to the pipe material and age, not the new-pipe value), or Darcy-Weisbach where the fluid, temperature, or Reynolds number warrants the more rigorous treatment. Confirm that the residual pressure at the most hydraulically remote node meets the minimum under each load case, and that the maximum pressure at the low-elevation nodes does not exceed the service or pipe rating. A network sized by velocity alone, without headloss and residual-pressure verification, will have zones that fall below the minimum pressure at the worst node.

### Model the Network Under Extended-Period and Contingency Conditions

Build a network hydraulic model (EPANET or equivalent) and run it under steady-state peak hour, steady-state maximum day plus fire flow at each hydrant district in turn, and extended-period simulation over the maximum day to verify that the tank levels cycle correctly and recover. Run the contingency cases: the loss of the largest pump, the loss of the largest transmission main, and the loss of a storage facility, to confirm that the system maintains service under each single failure. Identify and resolve low-pressure nodes, dead ends with high water age, and zones where the storage does not recover over the daily cycle. A model that has only been run at peak hour has not been tested against the fire-flow or the failure cases that define the system's real reliability.

### Analyse Pressure Transient (Surge) Conditions

Pressure transients, caused by pump start or stop, valve closure, or rapid demand change, can raise or lower the pressure far beyond the steady-state range and rupture or collapse pipes. Analyse the system for surge by the method of characteristics or a recognised surge model, with the actual pump inertia, valve closure times, and pipe profiles, and provide protection (surge tanks, air chambers, slow-closing valves, air-release and air-vacuum valves at high points) where the transient exceeds the pipe rating or drops below vapour pressure. A system designed only for steady state will fail at the first pump trip or power outage, because the transient, not the steady pressure, governs the worst-case pipe loading.

### Control Water Age and Water Quality

Long residence times in storage and dead-end mains degrade water quality: disinfectant residual decays, disinfection by-products form, and bacterial regrowth occurs. Design the network to minimise water age by locating storage to turn over daily, by avoiding oversized mains and dead ends, by providing looping where the system allows, and by specifying flushing hydrants or blow-offs where dead ends are unavoidable. For systems with multiple sources or treatment changes, model the blending and the disinfectant residual under extended-period simulation. A system that meets pressure and flow but delivers old, low-residual water fails its primary purpose of protecting public health.

### Specify Materials, Appurtenances, and Corrosion Protection

Select pipe materials suited to the pressure, soil, and water quality: ductile iron, PVC, HDPE, concrete pressure pipe, or steel, each with its own fittings, joint type, corrosion protection, and service life. Provide cathodic protection or polyethylene encasement for metallic pipe in corrosive soil, lining and coating suited to the water chemistry, and air-release and air-vacuum valves at high points to prevent air binding and vacuum collapse. Provide isolation valves to allow repair without shutting down large areas, and check valves and surge protection at pump stations. A system with the right hydraulics but the wrong material or no corrosion protection will fail by external or internal corrosion long before its design life.

## Common Traps

### The Hazen-Williams C For New Pipe

The headloss calculation uses the new-pipe C value (140 or higher for ductile iron), and the mains are sized on that basis. The trap is that the calculation is correct for the day of installation, while the pipe roughens with age, tuberculation, and deposits, and the real C falls to 100 or lower over the design life, raising the headloss and dropping the residual pressure below the minimum. The false signal is the defensible new-pipe C; the harm is a network whose pressure decays over the years until it fails the peak-hour or fire-flow case that it met at handover.

### The Peak-Hour-Only Model

The network model is run at peak hour and the pressures are adequate, and the model is declared complete. The trap is that the peak hour is the easiest load case, while the maximum-day-plus-fire-flow case and the contingency cases (largest pump or main out of service) are the cases that define reliability, and they have not been run. The false signal is the adequate peak-hour pressure; the harm is a system that meets the peak but collapses under fire flow or fails service when a single component is lost, the very conditions the storage and redundancy were meant to cover.

### The Surge Never Analysed

The system is designed for steady-state pressure and the pipe class is selected accordingly, with no transient analysis. The trap is that the steady pressure is the average condition, while the pump trip or the rapid valve closure produces a transient that can double the pressure or drop it to vapour, rupturing or collapsing the pipe. The false signal is the steady-state adequacy; the harm is a main that fails at the first power outage or pump trip, because the transient, not the steady state, governs the worst loading.

### The Dead End With Old Water

A main is extended to a new development as a dead end because looping is expensive, and no blow-off or flushing plan is provided. The trap is that the main delivers adequate flow and pressure, while the water at the end of the main stagnates, the disinfectant residual decays, and bacterial regrowth occurs, delivering water that fails the quality standard. The false signal is the adequate pressure; the harm is a public-health failure at the far end of the system, traced not to the hydraulics but to the water age that the dead end created.

### The Storage Sized For Equalisation Only

A tank is sized to equalise the peak above the average supply rate, and the fire and emergency components are omitted or understated. The trap is that the tank cycles correctly over the average day, while the fire-flow drawdown or the supply-failure duration empties the tank and the system loses pressure at the very moment it is most needed. The false signal is the daily cycle that the tank performs; the harm is a system that cannot sustain a fire flow or survive a supply outage for the required duration, because the storage was sized for one of its three functions.

### The Loop Without The Contingency Main

A looped main is provided for reliability, but the loop is sized so that, with one side out of service, the remaining side cannot carry the peak-hour or fire flow at adequate pressure. The trap is that the loop provides redundancy on paper, while the contingency capacity is half the required, because the loop was sized for the combined, not the single, condition. The false signal is the redundant layout; the harm is a system that loses service to a large area when one side of the loop fails, because the redundancy was apparent, not real.

## Self-Check

- Are the average, maximum day, and peak hour demands projected over the design life, and is the fire-flow requirement established by code for each zone?
- Are pressure zones configured to maintain service pressure across the topography, with storage sized for equalisation, fire, and emergency and pumping sized with redundancy and standby power?
- Are mains sized by headloss and residual pressure under each load case, with velocities within limits and Hazen-Williams C (or Darcy-Weisbach) values representing the aged, not the new, condition?
- Has the network model been run at peak hour, maximum day plus fire flow at each district, and extended-period over the maximum day, with tank cycling and recovery verified?
- Have the contingency cases (largest pump out, largest main out, storage out) been modelled, and do they maintain service under each single failure?
- Has a pressure transient (surge) analysis been performed, and is surge protection provided where the transient exceeds the pipe rating or drops to vapour pressure?
- Is water age controlled by storage turnover, looping, and flushing, and is the disinfectant residual maintained at the far nodes under extended-period simulation?
- Are pipe materials, corrosion protection, air valves, isolation valves, and pump-station check and surge valves specified for the actual soil, water, and pressure conditions?
