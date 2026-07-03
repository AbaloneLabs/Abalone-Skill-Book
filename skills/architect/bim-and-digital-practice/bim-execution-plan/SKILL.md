---
name: bim_execution_plan.md
description: Use when the agent is drafting or reviewing a BIM Execution Plan (BEP), defining Level of Development, assigning model authorship, setting coordinate systems and naming conventions, or establishing the digital delivery rules that govern a project's shared model.
---

# BIM Execution Plan

A BIM Execution Plan is the contract for how a project's shared digital model will be produced, exchanged, and trusted. It fixes the rules before anyone models, so that discipline models integrate, data survives handoff, and the deliverable the owner receives can actually be used for operations. Agents often treat the BEP as a template to be filled in and miss that its real value is forcing decisions about authorship, detail, and data that, if left unresolved, produce a federated model that looks complete but is unusable for coordination, quantity takeoff, or facilities management. The architect frequently owns the pre-contract BEP and must defend its provisions through design and into construction, because the BEP is where the owner's information requirements meet the team's modeling practices. The goal is a BEP that is specific to this project, agreed by every discipline, and enforceable as a delivery requirement rather than a suggestion.

## Core Rules

### Start From The Owner's Information Requirements, Not A Generic Template

A BEP exists to satisfy an Employer's or Owner's Information Requirement, so begin by obtaining that document and extracting the data the owner actually needs at handover: asset registers, warranty data, equipment parameters, spatial data, and the required model uses such as coordination, quantity extraction, or as-built records. A BEP written without the owner's requirements in view produces a model that is internally elegant but operationally useless. If the owner has not stated requirements, the architect must elicit them, because a defaulted BEP hands the owner a model they cannot use. Map each required deliverable to a model use, a level of information need, and a responsible author, so the BEP is traceable from requirement to outcome.

### Define Level Of Development And Information Need Explicitly

Level of Development (LOD) defines how reliably an element may be relied upon, and it must be specified per element category and per project milestone, not as a single project-wide value. Distinguish LOD from Level of Information Need, which addresses the data attributes attached to each element, because a structural column modeled to LOD 350 without the required attribute data is non-compliant even though it looks detailed. Specify which elements must carry parameters — fire rating, manufacturer, U-value, OmniClass or Uniclass code — and at which milestone, because data is as much a deliverable as geometry. State what "not-modeled" or generic content means for each category, so that placeholders are not mistaken for resolved design.

### Assign Model Authorship And Avoid Overlapping Responsibility

Every model element must have exactly one author at any milestone, and the BEP must record that authorship in a matrix that disciplines can reference. The architect authors walls, doors, and many finishes; the structural engineer authors the primary load-bearing framing; the MEP engineer authors ducts, pipes, and equipment. The trap is overlap: when two disciplines both model a shaft wall or a slab edge, the federated model carries conflicts that no one owns. Resolve contested elements — elevator pits, curbs, shaft enclosures, fireproofing — explicitly in the matrix, and assign secondary authors only for reference information that the primary author will incorporate. Authorship determines liability for accuracy, so it must be unambiguous.

### Establish Coordinate Systems, Datums, And Shared Positioning

A federated model only coordinates if every discipline model shares the same coordinate origin, orientation, and elevation datum. Specify the project base point, the survey point, the true north rotation, and the elevation datum, and require that all models be exported and imported against these references without internal repositioning. State the units, the grid system, and the level naming convention, because a model set in meters against a model set in feet, or a model that uses arbitrary elevations against one using sea-level datums, will not federate cleanly. Require a coordinate verification step at each exchange, because a silent shift of origin propagates as hundreds of clashes.

### Set Naming Conventions, File Structures, And Exchange Formats

Naming conventions must be defined for models, views, sheets, families, materials, and parameters, and they must be machine-readable and consistent across disciplines so that downstream automation can parse them. Specify the exchange format and version — IFC 2x3, IFC 4, native RVT, DWG with a defined coordinate attachment — and the model view definitions (MVDs) to be used, because an IFC exported without the correct MVD drops the data the owner needs. Define the common data environment (CDE) workflow: where files live, how they are named by status (WIP, shared, published, archived), and who has authority to publish. A BEP without a CDE workflow produces version chaos within weeks.

### Define Clash Detection, Coordination, And Review Cadence

The BEP must state how coordination happens: which model is the federated master, who runs clash detection, on what cadence, and how clashes are triaged and resolved. Specify clash categories — hard clashes, clearance clashes, and workflow or sequence clashes — and the rules for what counts as a valid clash versus an accepted tolerance. Require that resolved clashes be re-checked, because a fix in one model can introduce a new clash in another. Set a coordination meeting cadence and define the decision authority for disputes, because coordination stalls when no one can rule on a contested routing.

### Make The BEP Enforceable And Living

A BEP that is not referenced in the professional services agreements is a suggestion that contractors can ignore. Ensure the BEP is incorporated by reference into the owner-architect, owner-consultant, and owner-contractor agreements, and that the construction BEP is developed from the design BEP rather than replacing it. Treat the BEP as a living document with a change log, because modeling rules evolve as the design resolves and the team learns what works. Require that deviations from the BEP be documented and approved, because silent drift produces a model that no longer matches the contract.

## Common Traps

### Filling In A Generic Template Without Project-Specific Decisions

The team downloads a standard BEP, fills in the project name, and submits it, leaving LOD, authorship, and exchange rules at generic defaults. The mechanism is that templates feel sufficient and the project-specific decisions feel like they can be deferred, and the false signal is that the document is complete because all fields are populated. The harm is that disciplines model to their own assumptions, the federated model carries thousands of clashes and data gaps, and the owner receives a model that cannot be used for the intended purposes. Every section of the BEP must be examined against this project's model uses and resolved to a specific, defensible rule.

### Confusing Level Of Detail With Level Of Development

The team equates LOD with how detailed the geometry looks, so a wall is shown with sweeps and reveals early in design and labeled LOD 400 because it appears precise, when its dimensions, composition, and fire rating are still undecided. The mechanism is that "development" sounds visual and teams grade on appearance rather than on the reliability of the information, and the false signal is that a rich-looking object is a reliable object. The harm is that downstream users — contractors pricing, fabricators detailing, owners maintaining — rely on data that is not actually resolved, producing bad quantities, bad coordination, and bad asset registers. LOD must be graded on the dependability of the information, and geometry richness must be matched to confirmed design decisions.

### Leaving Authorship Of Contested Elements Unassigned

Shaft walls, slab edges, fireproofing, elevator enclosures, and curbs are modeled by multiple disciplines because each needs them for their own work, and the BEP never assigns a single author. The mechanism is that contested elements are not glamorous decisions and the matrix focuses on the obvious categories, and the false signal is that the matrix covers the major systems and so is complete. The harm is that the federated model carries duplicate geometry that generates false clashes, and when a real conflict arises no one is responsible for the resolution. Every element that more than one discipline touches must have a single primary author, with the others limited to reference copies.

### Neglecting The Data Side Of The Deliverable

The team focuses on geometry and treats parameters as optional, so the as-built model contains rich three-dimensional content but no manufacturer, warranty, or maintenance data, and the owner's facilities platform cannot populate its asset register. The mechanism is that data entry is tedious and invisible compared to modeling, and the false signal is that a model that looks complete is complete. The harm is that the owner's information requirement is not met, the model is useless for computerized maintenance management, and the team has delivered a deliverable the owner cannot operate from. Required parameters must be specified per element and per milestone, and parameter completeness must be verified at each submission like geometry completeness.

### Failing To Enforce The Common Data Environment

Files are emailed, dropped on shared drives, and named informally, so the team works from multiple versions and the federated model federates against stale discipline models. The mechanism is that a CDE feels like overhead and email feels immediate, and the false signal is that the file was sent and so it is current. The harm is that coordination runs against superseded models, clashes are reported against geometry that has already changed, and decisions are made on bad information. The CDE must be the single source of truth, with status-based naming, and discipline leads must publish rather than send.

## Self-Check

- [ ] Does the BEP derive from a documented owner information requirement, with each required deliverable mapped to a model use, author, and milestone?
- [ ] Is Level of Development specified per element category and per milestone, and is it graded on information reliability rather than visual detail, with required parameters identified?
- [ ] Does the authorship matrix assign exactly one primary author to every element, including contested elements like shaft walls, slab edges, and fireproofing?
- [ ] Are coordinate system, datum, units, grid, and level naming defined and required for every discipline model, with a verification step at each exchange?
- [ ] Are naming conventions, exchange formats, IFC MVDs, and the common data environment workflow specified, with file status-based naming enforced?
- [ ] Is the clash detection and coordination process defined — federated master, cadence, clash categories, triage authority, and re-check rules?
- [ ] Is the BEP incorporated by reference into the professional services and construction agreements, and maintained as a living document with a change log?
- [ ] Are required data parameters verified at each submission with the same rigor as geometry, so the as-built model supports the owner's facilities management needs?
