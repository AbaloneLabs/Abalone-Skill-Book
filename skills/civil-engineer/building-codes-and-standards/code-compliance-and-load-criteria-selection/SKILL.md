---
name: code-compliance-and-load-criteria-selection.md
description: Use when the agent is selecting the governing building code, identifying the applicable loads and load combinations, establishing occupancy and risk categories, or determining seismic, wind, snow, and flood criteria for a structure. Applies before any structural calculation, while resolving jurisdictional amendments and local code modifications, and when reviewing which edition of ASCE 7, the IBC, or material standards governs the design.
---

# Code Compliance and Load Criteria Selection

Code compliance and load criteria selection is the foundational step that establishes the rules the structure must satisfy and the loads it must resist, and every subsequent calculation inherits the choices made here. The governing code is not a single document: it is the adopted edition of the building code (the IBC or a regional equivalent), the referenced loads standard (ASCE 7 or equivalent), the material design standards (ACI 318, AISC 360, AISI, AWC, TMS), and the jurisdictional amendments that modify each, and the engineer must identify the exact set that applies to the project before computing a single load. The harm this skill prevents is a design built to the wrong code edition, to loads that do not match the site, or to a risk category that underestimates the consequence, any of which can leave the structure underdesigned against the governing event and create a compliance and a safety defect that propagates through the entire design. Because the code selection governs every load and every factor, it must be deliberate, documented, and reviewed before the calculations begin.

## Core Rules

### Identify the Governing Code, Edition, and Amendments Before Any Calculation

The first act of code compliance is to identify the exact governing documents: the building code adopted by the authority having jurisdiction (AHJ), at the adopted edition, with all local and state amendments; the referenced loads standard at the edition adopted by the code; and the material design standards at the editions adopted by reference. Confirm that the adopted edition is the current one for the permit application date, because codes update on cycles and the transition between editions changes the loads and the criteria. Identify and read the jurisdictional amendments, because state and local amendments can modify the wind speed, the seismic criteria, the snow load, the flood elevation, the occupancy classification, and the special inspection requirements, and a design that uses the unamended national code can be noncompliant in the jurisdiction. Document the governing code set in the design basis, so that every reviewer and every future investigation can verify the basis.

### Establish the Risk Category and the Consequence Classification

The risk category (occupancy category in some codes) classifies the structure by the consequence of its failure, from Category I (low hazard to life, such as agricultural storage) through Category II (default, ordinary buildings) and Category III (substantial hazard, such as schools, assembly, water treatment) to Category IV (essential facilities, such as hospitals, fire stations, emergency operations). The risk category governs the importance factors for wind, snow, and seismic (with higher factors for higher categories), the seismic design category thresholds, and the load combinations and the structural observation and special inspection requirements. Select the risk category against the actual use and the consequence, not by default, because a building that houses a school, a public assembly, or an essential function carries a higher category and higher loads, and a default Category II on a Category III or IV building underdesigns the structure for its consequence.

### Establish the Site-Specific Load Criteria

For each load, establish the site-specific criteria from the code and the referenced standard. Dead load from the actual materials and construction. Live load from the occupancy and the use table, with the reduction permitted only where the code allows and where the element qualifies. Snow load from the ground snow map, the exposure, the thermal, and the importance factors, with the site-specific value where the jurisdiction provides one. Wind load from the wind speed map (or the site-specific value), the exposure category, the enclosure classification, the topographic factor, and the directionality, with the gust-effect factor for flexible buildings. Seismic load from the mapped or site-specific spectral acceleration, the site class, the site coefficient, the risk category, the seismic design category, and the structural system and the response modification coefficient. Flood load from the flood insurance rate map and the design flood elevation, with the freeboard. Confirm that each criterion uses the site-specific value, not a regional default, because the wind, the snow, the seismic, and the flood vary across the region and the default can be far from the site.

### Resolve Conflicts and the More-Stringent Requirement

Where the codes, the standards, or the amendments conflict, the more stringent requirement governs, unless the conflict is a true inconsistency that the AHJ must resolve. Identify the conflict, evaluate the basis of each requirement, and apply the more conservative, because the code sets the minimum and the more stringent provides the safety margin. For the load combinations, use the strength and the allowable stress combinations from the governing standard (ASCE 7), and confirm that the material design uses the matching resistance factors or allowable stresses. Do not cherry-pick the less stringent provision across codes, because the design must be consistent within the governing code set, and a mixed-code design is noncompliant even if each individual provision is met in its own code.

### Document the Design Basis and the Criteria

The design basis memorandum documents the governing code set, the risk category, the site-specific load criteria, the material design standards, the analysis method, the structural system, and the special inspection and structural observation requirements. The design basis is the reference for the entire project, the basis of the plan review, and the record in any future investigation. A design without a documented basis is a design whose criteria cannot be verified, and any future question (a plan review comment, a change order, a defect investigation) must reconstruct the criteria from the calculations, with the risk of error. Document the basis before the calculations, and update it when the criteria change.

### Address the Permit, the AHJ, and the Special Inspection

Code compliance is not complete with the calculations; it requires the permit, the plan review, and the special inspection. Confirm that the design is submitted to the AHJ with the required documentation, that the plan review comments are resolved and the design revised to address them, and that the special inspection and structural observation program (for the structural elements, the connections, the materials, and the seismic systems) is established and the special inspector engaged. A design that meets the code on paper but is not inspected as required is noncompliant in the field, because the special inspection is the verification that the as-built structure matches the designed structure.

## Common Traps

### The Wrong Code Edition

The design uses the previous edition of the code or the loads standard, because the templates and the software are set up for it, while the AHJ has adopted the current edition. The trap is that the design is internally consistent, while it does not comply with the adopted edition, which may have higher wind speeds, revised seismic criteria, or new requirements that the previous edition did not have. The false signal is the completed, code-referenced design; the harm is a plan review rejection, a redesign, or, worse, a permitted design that does not meet the current criteria.

### The Default Risk Category

A school, a public assembly building, or a water treatment plant is designed at the default Category II, because the use was not specifically evaluated, while the actual use requires Category III or IV. The trap is that the design meets Category II loads, while the consequence of failure is higher and the code requires higher importance factors, seismic criteria, and special provisions for the actual category. The false signal is the compliant Category II design; the harm is a structure underdesigned for its consequence, with lower seismic and wind capacity than the code requires for the actual use.

### The Regional Default For A Site-Specific Load

The wind speed, the ground snow, or the seismic acceleration is taken from a regional default or a neighbouring project, while the site-specific value (from the map, the AHJ, or the site-specific study) is different. The trap is that the load is defensible-looking, while the site-specific value is higher (or, less commonly, lower), and the design does not match the site. The false signal is the cited load value; the harm is a structure designed for a load that the site exceeds, with the underdesign surfacing only in the design event.

### The Amendment Not Read

The design uses the national code without the jurisdictional amendments, because the national code is the familiar reference. The trap is that the design complies with the national code, while the amendments modify the wind, the snow, the seismic, the flood, or the special inspection for the jurisdiction, and the design does not meet the amended criteria. The false signal is the national-code compliance; the harm is a design noncompliant in the jurisdiction, with plan review comments, redesign, or field corrections.

### The Less Stringent Provision Across Codes

The design uses the less stringent provision from one code where another code in the governing set is more stringent, on the reasoning that each provision is met in its own code. The trap is that each provision is individually compliant, while the governing code set requires the more stringent where they conflict, and the mixed-code design is noncompliant. The false signal is the citation of compliant provisions; the harm is a design that meets a lower standard than the governing code requires, because the conflict was resolved toward the less stringent.

### The Design Without The Documented Basis

The design proceeds without a documented design basis, and the criteria are scattered across the calculations and the correspondence. The trap is that the design is complete, while the criteria cannot be verified without reconstruction, and a plan review comment, a change, or a future investigation cannot confirm the basis. The false signal is the completed calculations; the harm is a design whose compliance cannot be demonstrated, because the basis was never documented at the start.

## Self-Check

- Is the governing code, the adopted edition, and all jurisdictional amendments identified and documented before any calculation?
- Is the risk category selected against the actual use and consequence, with the importance factors, seismic criteria, and special provisions matching the category?
- Are the site-specific load criteria (dead, live, snow, wind, seismic, flood) established from the code and the site-specific values, not regional defaults?
- Are conflicts between codes, standards, and amendments resolved toward the more stringent, with the load combinations and the material resistance consistent within the governing set?
- Is the design basis documented in a memorandum before the calculations, and updated when the criteria change?
- Are the permit submission, the plan review, and the special inspection and structural observation program established, with the special inspector engaged for the structural and seismic elements?
- Are the material design standards (ACI, AISC, AISI, AWC, TMS) at the adopted editions, with the matching resistance factors and allowable stresses?
- Has the code set and the criteria been peer-reviewed or plan-reviewed before the design is finalised?
