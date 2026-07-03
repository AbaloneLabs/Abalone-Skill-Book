---
name: occupancy_and_construction_type_determination.md
description: Use when the agent is classifying a building's occupancy group under the IBC, selecting the construction type, evaluating mixed-occupancy conditions and separations, or establishing the foundational code decisions that govern height, area, fire protection, and egress for the entire project.
---

# Occupancy And Construction Type Determination

Occupancy classification and construction type are the two foundational decisions in building code analysis, because together they determine everything that follows: allowable height and area, required fire-resistance ratings, permitted materials, egress capacity, fire protection features, and the scope of sprinkler requirements. Get these wrong and the entire code analysis cascades into error, producing a design that cannot be permitted or that requires costly redesign late in documentation. Agents often treat occupancy classification as a labeling exercise and miss that the IBC's occupancy groups turn on the intended use and the nature of the occupants — their familiarity with the building, their ability to self-evacuate, their density — and that a small misclassification can change the construction type required and with it the structural and envelope systems. The architect owns this determination because it is a design judgment about how the building will be used, even though the structural engineer and fire protection consultant contribute to its consequences. The goal is a defensible classification and construction-type selection, documented on the drawings, that the authority having jurisdiction will accept on the first review.

## Core Rules

### Classify Occupancy By Actual Use And Occupant Characteristics

The IBC organizes occupancies into ten main groups — Assembly, Business, Educational, Factory and Industrial, High-Hazard, Institutional, Mercantile, Residential, Storage, and Utility — each with subgroups, and the classification turns on the use and the occupants, not on the building's name or the owner's preference. Read the IBC definitions literally: an Assembly occupancy is one where persons gather for purposes such as civic, social, or religious functions, and the threshold for assembly treatment depends on occupant load, with spaces under a defined count sometimes classified with their host occupancy. Evaluate every space individually, because a building is often mixed-occupancy, with a restaurant (Assembly) in an office building (Business) and storage rooms (Storage) throughout. Document the basis for each classification, because the AHJ will ask, and an undocumented classification is an invitation to a comment. When a use is genuinely ambiguous, request an AHJ interpretation early, because a late reclassification can invalidate an entire design.

### Apply The Occupant Load Calculation Correctly

Occupant load drives egress capacity, plumbing fixture counts, and sometimes occupancy classification itself, and it is calculated from the IBC's occupant load factor table based on the use of each space, not on the actual expected population. Use the factor that matches the space's function — unconcentrated assembly at five net square feet per person differs sharply from concentrated assembly at seven net — and calculate gross versus net area correctly, because using gross where net is required or vice versa produces a wrong load that propagates into egress and plumbing. Sum the loads by floor and by occupancy, and recognize that the calculated occupant load may exceed the realistic population, which is intentional: the code sizes egress for design loads, not actual ones. An occupant load that drives the design must be shown on the life-safety plan with the factors used, because the AHJ verifies it.

### Select Construction Type As A Risk And Cost Decision

Construction type — Type I through Type V, with subtypes A and B — sets the permitted combustibility and the required fire-resistance of structural elements and is the primary determinant of allowable height and area. Type I and II are noncombustible with high ratings and permit the largest buildings but at the highest cost; Type V permits combustible framing and is the most economical but the most restricted in height and area. The selection is not free; it is constrained by the occupancy and the desired building size, and it is a cost decision because moving from Type V-B to Type II-A can fundamentally change the structural system. Select the least restrictive type that satisfies the height and area needs, because over-selecting imposes unnecessary cost, but never select a type that the occupancy and size do not permit. Document the construction type on the drawings and in the code analysis, because every downstream code provision depends on it.

### Resolve Mixed-Occupancy Conditions Deliberately

Most non-trivial buildings contain multiple occupancies, and the IBC offers three methods to handle them: fully separated, where each occupancy is separated by fire barriers rated per the table and each is treated independently for height and area; non-separated, where the most restrictive occupancy governs the entire building; and accessory, where a minor occupancy incidental to the main one is treated with it. The choice is a design and cost decision: separation allows larger aggregate area but requires rated barriers and protected openings; non-separation is simpler but constrains the whole building to the strictest limits. Evaluate each mixed-occupancy condition explicitly, document the method chosen, and confirm that the chosen method is consistently applied through the height and area calculation and the fire-rated assembly details. A building that uses separation in the area calculation but shows no rated barriers on the plans is internally inconsistent and will not pass review.

### Integrate Sprinkler Status Into The Classification Logic

The presence of an NFPA 13 automatic sprinkler system is not merely a fire protection feature; it is a code variable that changes occupancy treatment, height and area allowances, and construction-type requirements. Some occupancies require sprinklers regardless of size, others are incentivized by the increased allowances sprinklers provide, and a sprinkler can reduce the required fire-resistance of certain elements or permit an additional story. Decide the sprinkler strategy early — full NFPA 13, NFPA 13R for residential, or none — because it is integral to the classification and cannot be added later to rescue a non-compliant design. Coordinate the sprinkler decision with the owner, because it has first-cost and maintenance implications, and document it as an assumption in the code analysis so the AHJ and the team share the basis.

### Document The Determination Defensibly

Occupancy and construction type are legal determinations that the AHJ will scrutinize, and they must be presented, not merely decided. Produce a code analysis sheet or narrative that lists each occupancy, its basis, the construction type selected, the sprinkler status, and the resulting height and area check, with code section references. Show mixed-occupancy methods, fire-rated assembly locations, and the occupant load calculations. A determination that lives only in the architect's head or in an email is not defensible; the determination must be on the drawings and in a written analysis that accompanies the permit set. When the AHJ questions the classification, the documentation is what converts a dispute into a discussion.

### Recognize When To Defer To Specialists

The architect owns the occupancy and construction-type determination, but specific conditions require specialist input. High-hazard occupancies require a fire protection engineer for the detailed analysis the IBC demands; health care occupancies trigger CMS and NFPA 101 requirements that a code consultant may track more reliably; high-rise buildings trigger stacked structural and fire protection provisions that benefit from specialist coordination. Engage these specialists early, before the classification is finalized, because their input can change the determination. The architect's role is to integrate specialist input into a coherent code analysis, not to produce it in isolation when the conditions exceed general practice.

## Common Traps

### Misclassifying Assembly Spaces By Ignoring Occupant Load Thresholds

A restaurant, a conference room, or a lobby is classified as Business because it sits in an office building, when its occupant load exceeds the assembly threshold and it must be treated as Assembly. The mechanism is that the IBC's assembly threshold is occupant-load-based and teams classify by the host building's use rather than by each space's calculated load, and the false signal is that the space "feels" like part of the office. The harm is that the egress, plumbing, and fire protection are undersized for the actual load, the AHJ catches it in review, and the redesign ripples through the floor plan, the structural shafts, and the plumbing design late in documentation. Every gathering space must be evaluated against the assembly threshold by calculated occupant load, not by intuition or by the building's primary use.

### Choosing Construction Type Without Checking Height And Area First

The team selects Type V-B wood framing for economy, designs the building, and only then runs the height and area check, discovering that the occupancy and size require Type II-A or a sprinkler that was not planned. The mechanism is that construction type feels like a structural decision and is deferred to the structural engineer, who selects for cost without the code constraint, and the false signal is that a type was chosen and so the analysis seems complete. The harm is that the late discovery forces either a smaller building, a more expensive structural system, or a redesign that cascades through the documents. Construction type must be selected through an iterative check against the height and area tables at the start of design, because it is a code-driven decision with cost consequences, not a cost-driven decision with code afterthoughts.

### Inconsistent Mixed-Occupancy Treatment

The code analysis claims separated occupancies to maximize allowable area, but the drawings show no rated barriers between the occupancies, or the area calculation uses non-separated treatment while the details show separation. The mechanism is that the mixed-occupancy method is chosen in the narrative but never translated into the rated assembly requirements on the plans, and the false signal is that the analysis and the plans were both produced by the team. The harm is that the AHJ cannot verify the claimed separation, issues a comment requiring either rated barriers or a recalculation under the more restrictive method, and the correction changes the design. The mixed-occupancy method must be chosen once, documented, and consistently reflected in the area calculation, the rated assembly plan, and the detail callouts.

### Forgetting That Sprinkler Status Changes Everything

The team designs to unsprinklered allowances and only later adds a sprinkler to satisfy a different requirement, or designs to sprinklered allowances and then value-engineers the sprinkler out, without re-running the classification and height-area analysis. The mechanism is that sprinklers are treated as an additive feature rather than a code variable, and the false signal is that the sprinkler is a fire protection detail unrelated to the classification. The harm is that removing or adding the sprinkler changes the allowable height and area, the required ratings, and sometimes the occupancy treatment, invalidating a design that was compliant under the prior assumption. The sprinkler status must be fixed as a code input before the analysis and re-verified whenever it changes, because it is a load-bearing assumption in the entire code logic.

### Classifying By Building Name Or Owner Preference

An owner calls a building a "training center" and the team classifies it as Business, when its function — assembly of trainees for instruction — is Educational or Assembly depending on the occupant count and the nature of the program. The mechanism is that classification by label is faster than analysis against the definitions, and the false signal is that the owner's terminology seems authoritative. The harm is that the misclassification produces wrong egress, wrong ratings, and wrong plumbing, discovered at permit review, with redesign costs borne by the design team. Classification must be driven by the IBC definitions and the actual use, tested against the occupant load factors, and never by the marketing name or the owner's preference.

### Undocumented Determinations That Cannot Survive Review

The architect makes a sound classification decision but records it only in conversation or in a draft email, so when the AHJ questions it there is no defensible basis and the team must reconstruct the logic under pressure. The mechanism is that documentation feels like overhead during design and the determination seems settled once decided, and the false signal is that the decision was made and so it stands. The harm is that the AHJ's question becomes a re-analysis, the schedule slips, and a decision that was correct may be overturned for lack of documentation. Every occupancy and construction-type determination must be documented in a written code analysis with section references, on or accompanying the drawings, before permit submittal.

## Self-Check

- [ ] Has every space been classified against the IBC occupancy definitions based on actual use and occupant characteristics, not on building name or owner preference?
- [ ] Have assembly-eligible spaces been evaluated against the occupant-load threshold, with the calculation shown on the life-safety plan?
- [ ] Has construction type been selected through an iterative height-and-area check at the start of design, rather than chosen for cost and checked later?
- [ ] Has the mixed-occupancy method — separated, non-separated, or accessory — been chosen, documented, and consistently reflected in the area calculation, rated assembly plan, and details?
- [ ] Is the sprinkler status fixed as a code input, documented in the analysis, and re-verified whenever it changes?
- [ ] Does a written code analysis with section references accompany the permit set, documenting each occupancy, the construction type, and the basis for each determination?
- [ ] Have specialist inputs — fire protection, health care, high-rise — been integrated before the classification was finalized?
- [ ] Has the occupant load been calculated using the correct factor and gross-or-net basis for each space, and propagated into egress and plumbing?
