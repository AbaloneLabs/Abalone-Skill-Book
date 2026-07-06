---
name: water-treatment-process-design.md
description: Use when the agent is designing or reviewing a surface water or groundwater treatment process train, selecting coagulant doses from jar testing, sizing flocculation and sedimentation basins, specifying granular media filters, computing disinfection CT for Giardia and viruses, or evaluating disinfection by-product formation under the Surface Water Treatment Rule and Stage 2 DBP rules.
---

# Water Treatment Process Design

Water treatment process design is the engineering of a train of physical, chemical, and biological unit processes that must reliably remove turbidity, pathogens, and regulated contaminants from a raw source so that the finished water meets drinking water standards at every moment, including the storm event that spikes raw water turbidity by an order of magnitude. The train is not a sequence of independent boxes; each unit conditions the water for the next, and the failure or overload of one stage cascades through the others. The harm this skill prevents is a plant that meets the turbidity limit on a clear day but passes Giardia cysts during the spring runoff, a disinfection design that computes CT at design flow but not at the cold-water condition where inactivation collapses, or a coagulation strategy tuned to one raw water quality that fails when the source turns over. Because the Surface Water Treatment Rule and the disinfection by-product rules are health-based and enforceable, and because treatment breakthrough is a direct public-health failure, the design must be defensible against the worst raw water condition, not the average.

## Core Rules

### Establish the Raw Water Characterisation and Regulatory Drivers First

Before selecting any unit process, characterise the source across its full range: historical turbidity minima and maxima including storm spikes, temperature seasonal range (cold water governs disinfection and settling), alkalinity (governs coagulant dose and pH adjustment), total organic carbon and UV-254 (the precursors of disinfection by-products), and the pathogen log removal credit required under the Surface Water Treatment Rule for Giardia cysts, viruses, and Cryptosporidium oocysts. Identify the applicable rules: the Long Term 2 Enhanced Surface Water Treatment Rule (LT2ESWTR) bin classification that dictates additional Cryptosporidium removal or inactivation, the Stage 2 Disinfection By-Products Rule locational running annual average limits for total trihalomethanes and haloacetic acids, and the Lead and Copper Rule. The regulatory log-removal requirement and the DBP precursor load together define whether conventional treatment suffices or whether enhanced coagulation, ozone, GAC, or membranes are required. Without this characterisation the process selection is arbitrary.

### Base Coagulation, Flocculation, and Sedimentation on Jar Testing and Design Loading

Coagulant selection (alum, ferric salts, polyaluminium chloride, or polymers) and dose must come from bench-scale jar testing across the seasonal range of raw water temperature and turbidity, not from a textbook value, because the optimal dose and pH shift with TOC, alkalinity, and temperature. Verify enhanced coagulation where the DBP rule requires TOC removal: the Step 1 TOC removal percentages by alkalinity, and the Step 2 alternative if jar testing supports it. Size rapid mix for a short, intense G value and residence time that disperses the coagulant before the floc forms. Size flocculation as tapered (declining G) basins in series, with the detention time and G value producing a settleable floc without shear-breakage, and provide variable-speed mixers so the operator can tune to the raw water. Size sedimentation for the surface overflow rate and detention time per Ten States Standards, with the rate reflecting the floc settleability; for high-rate applications use tube or plate settlers with the manufacturer loading verified. Confirm that the sedimentation effluent turbidity feeding the filters is low enough that the filters are not overloaded.

### Design Filtration for Turbidity, Pathogen Removal Credit, and Filter Run Length

Granular media filters are the particle-removal barrier that earns the Surface Water Treatment Rule removal credit, and their design must satisfy the 0.3 NTU combined filter effluent turbidity limit in 95 percent of monthly readings and never exceed 1 NTU, with individual filter monitoring and reporting. Specify the media (dual or mixed media with anthracite over sand, or deep-bed monomedium) and the effective size, uniformity coefficient, and depth that produce the required removal and run length. Set the filtration rate (commonly 5 to 12 m/h conventional, higher for deep-bed) based on the solids loading from sedimentation, and verify that the filter run length between backwashes is adequate and that the backwash rate, duration, and auxiliary scour (surface wash or air scour) adequately clean the media without media loss. Confirm the filter-to-waste provision and the slow-start or the rate-of-change control that prevents the initial turbidity spike from reaching the clearwell. Remember that Cryptosporidium removal credit depends on the filter being well-operated, so a design that invites poor backwashing forfeits the credit.

### Compute Disinfection CT and Control Disinfection By-Products

Disinfection inactivation is credited by the CT concept, the product of the residual disinfectant concentration and the contact time, where the contact time is the T10 (the time for ten percent of a tracer to pass) at peak hourly flow, not the theoretical residence time. Compute the required CT for the target log inactivation of Giardia and viruses using the EPA CT tables for the actual disinfectant (chlorine, chloramines, chlorine dioxide, ozone, UV), at the actual worst-case cold water temperature and pH, because chlorine inactivation drops sharply as temperature falls and pH rises. Verify that the contact basin and the clearwell provide the required T10 at peak flow with the baffle factor, and provide redundant contact volume. Balance disinfection against the DBP rules: the chlorine dose and contact time that inactivate pathogens also react with TOC to form trihalomethanes and haloacetic acids, so the design must achieve the required CT without forming DBPs above the locational running annual average. Where chlorine alone cannot satisfy both, consider chloramination for distribution residual, ozone or UV for primary inactivation, or enhanced coagulation and GAC for precursor removal.

## Common Traps

### The CT Computed at Design Flow and Warm Temperature

The disinfection CT is calculated at the design flow and the average or warm temperature, and the contact basin is sized on that basis. The trap is that the calculation is correct for the easy condition, while inactivation drops sharply at the cold winter temperature and the higher pH, and at peak hourly flow the T10 falls, so the plant fails the required log removal exactly when source risk may be highest. The false signal is the compliant CT on paper; the harm is a Giardia breakthrough risk that the operator never sees because the worst case was never checked.

### The Coagulant Dose From a Textbook

A single coagulant and dose is specified from a reference value without jar testing across the seasonal raw water range. The trap is that the dose looks reasonable, while the optimal dose and pH shift with every storm, turnover, and TOC change, and a fixed dose underdoses during high-turbidity events (passing turbidity and pathogens to the filters) and overdoses during low-turbidity periods (wasting chemical and raising residuals). The false signal is the specified dose; the harm is a plant that cannot be tuned to its source and that periodically fails turbidity or TOC removal.

### The Filter Designed Without Backwash Verification

The filter media and rate are specified but the backwash system is treated as an afterthought, with a backwash rate and auxiliary scour inadequate to fluidise and clean the media. The trap is that the filter produces good effluent when new, while inadequate backwash allows mudballs and fissures to develop, the run length collapses, and turbidity breakthrough occurs long before the terminal headloss. The false signal is the rated filtration capacity; the harm is a filter that fails its pathogen removal credit in service because it was never properly cleaned.

### The Disinfection That Ignores DBP Formation

The chlorine dose and contact time are maximised to satisfy the CT for pathogen inactivation, with no parallel check of DBP formation against the Stage 2 locational running annual average. The trap is that the disinfection satisfies the microbiological rule, while the same chlorine reacting with the unremoved TOC forms trihalomethanes and haloacetic acids that exceed the DBP limit at the distant distribution tap where residence time is longest. The false signal is the compliant CT; the harm is a DBP violation that forces an expensive retroactive change to precursor removal or alternative disinfection.

## Self-Check

- Has the raw water been characterised across its seasonal range of turbidity, temperature, alkalinity, and TOC, and have the Surface Water Treatment Rule log-removal and LT2ESWTR Cryptosporidium requirements been established?
- Are coagulant type and dose based on jar testing across the raw water range, with enhanced coagulation verified where the DBP rule requires TOC removal?
- Are flocculation basins tapered with variable-speed mixers, and is sedimentation sized for the surface overflow rate and detention time that the floc settleability supports?
- Does the filter design satisfy the 0.3 NTU 95th-percentile and 1 NTU maximum limits, with media, rate, backwash rate, auxiliary scour, and filter-to-waste verified for run length and clean media?
- Is the disinfection CT computed at peak hourly flow, cold water temperature, and actual pH using T10 with the baffle factor, and does it meet the required Giardia and virus log inactivation?
- Is the disinfection balanced against the Stage 2 DBP locational running annual average, with precursor removal or alternative disinfection where chlorine alone cannot satisfy both rules?
