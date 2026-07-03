---
name: parametric_design_and_computational_workflow.md
description: Use when the agent is building parametric or computational design models, authoring scripts or graphs to generate and evaluate design options, setting up design automation for massing and paneling studies, or establishing a reusable computational workflow across a project team.
---

# Parametric Design And Computational Workflow

Parametric and computational design replaces fixed geometry with rule-based models whose outputs change as inputs change, letting the architect explore thousands of options, optimize against measurable criteria, and maintain consistency across complex geometries. The power is real, but it concentrates design decisions into the logic of the script, and a script that is opaque, unverified, or built on wrong assumptions produces confident-looking wrong answers at scale. Agents often treat computational design as a visualization tool and miss that every graph or script is a set of encoded design decisions that must be reviewed, documented, and validated like any other design artifact. The architect owns the computational workflow because it encodes design intent, and the goal is a workflow that is transparent, reproducible, and tied to verifiable design outcomes rather than a black box that produces seductive forms.

## Core Rules

### Define The Design Question Before Building The Graph

Computational design is powerful precisely because it can generate and evaluate many options, so the first step is to state the question the model must answer: maximize developable area within a zoning envelope, minimize surface area for a target volume, optimize panel sizes against a manufacturer's stock, or daylight autonomy across a floor plate. A graph built without a clear question produces outputs that cannot be evaluated, because nothing defines a good result. Write the objective, the constraints, and the evaluation metrics before modeling anything, and revisit them as the model develops, because the question often sharpens once options emerge. The architect must own the question because it is a design judgment, even when a computational designer builds the graph.

### Encode Constraints From Code, Zoning, And Program Explicitly

The script must encode the real constraints — setbacks, height limits, FAR, occupancy, daylight requirements, structural bay sizes — as explicit parameters, sourced from the controlling code and ordinance, not approximated from memory. A graph that uses a rounded setback or an assumed FAR produces a building that looks compliant but is not, and because the script scales the error across thousands of iterations, the wrong assumption compounds. Document the source of every constraint parameter with a code or ordinance citation, and require that constraint changes in the project trigger a script update. Treat the constraint block of the graph as a controlled input that must be verified like a code analysis sheet.

### Make The Logic Transparent And Readable

A computational graph that only its author can read is a liability: when the author leaves, the workflow dies, and even while they remain, no one can verify the logic. Structure the graph with named groups, clear node labels, and a layout that reads left to right from inputs to outputs, and accompany it with documentation that explains the logic, the assumptions, and the dependencies. Prefer readable, commented code in scripted nodes over dense one-liners, because the team must be able to audit the logic. The architect must require this transparency, because an opaque workflow cannot be reviewed for design integrity.

### Validate Outputs Against Hand Calculations And Known Cases

Computational outputs must be validated against hand calculations and known reference cases before they are trusted, because scripts contain logic errors, unit errors, and silent failures that produce plausible but wrong results. For a paneling study, check the script's panel count against a manual count of a sample region; for a daylight analysis, check the script's results against a published reference or a simulation in a different tool. Treat the first confident-looking output as a hypothesis to be tested, not a result to be presented. Validation must be documented so reviewers can see that the workflow was checked, not merely run.

### Manage Data Flow, Units, And Tolerance Deliberately

Computational models move data between geometry, lists, and parameters, and silent mismatches — a list that shifts order, a unit that converts incorrectly, a tolerance that lets nearly-coincident points diverge — produce outputs that look right but are wrong. Standardize units across the graph, manage list structure explicitly with documented indexing, and apply tolerance controls consistently so that geometric operations are deterministic. Watch for operations that depend on point identity, because two points that should be identical may differ by a tiny amount and break a downstream operation. Data hygiene is not glamorous, but it is the difference between a reproducible workflow and one that gives different answers each run.

### Tie Optimization To Measurable, Agreed Criteria

When the workflow optimizes, the objective function must reflect criteria the team and the client have agreed matter — energy use, daylight, structural mass, cost, view — weighted as the project actually values them. An objective function chosen by the computational designer in isolation optimizes for the wrong thing, producing a building that scores well on a metric no one cared about. Make the objective function explicit, document the weights, and show the client the tradeoffs, because optimization is inherently about tradeoffs and hiding them produces a result no one owns. The architect must curate the criteria, because they are design values, not technical settings.

### Version, Document, And Hand Off The Workflow

Computational workflows evolve rapidly, and without version control the team cannot reproduce a result, trace why an option was chosen, or recover from a regression. Use version control for scripts and graphs, record the input sets that produced presented options, and document the dependencies — software versions, plugins, libraries — so the workflow can be re-run. At handoff, provide the workflow, its documentation, and the input sets that produced the issued design, because an owner or future team may need to re-evaluate the design against changed conditions. A workflow that cannot be reproduced is not a deliverable.

## Common Traps

### Optimizing For A Metric No One Agreed To Care About

The computational designer builds an objective function that minimizes surface-to-volume ratio because it is easy to compute, the optimization produces a form, and the team presents it without recognizing that the client valued daylight and views, which the objective ignored. The mechanism is that objective functions feel technical and the team defers to whoever builds the graph, and the false signal is that the result is optimized and therefore good. The harm is that the design optimizes the wrong objective, the client receives a building that fails their actual priorities, and the error is invisible because the metric looks impressive. The objective function must be curated by the architect and the client, with weights and tradeoffs documented.

### Trusting Outputs Without Validation

The script produces a daylight autonomy map or a panel layout, the team presents it, and a hidden unit error or list-shift has produced results that are off by a factor or that mislabel regions. The mechanism is that computational outputs look authoritative and the effort of validation feels redundant, and the false signal is that the software produced the result and so it is correct. The harm is that wrong results drive design decisions, construction documents are based on bad analysis, and the error is discovered late or never. Every computational output must be validated against hand calculations or reference cases before it informs design, and the validation must be documented.

### Encoding Wrong Code Or Zoning Constraints

The graph uses a setback of fifteen feet because the computational designer remembered it from another project, but the actual ordinance requires twenty, and the optimization produces a massing that breaches the buildable envelope. The mechanism is that constraint parameters feel like setup details and are approximated rather than sourced, and the false signal is that the graph ran and produced geometry. The harm is that the entire optimization is invalid, the design is non-compliant, and the error propagates because the script scaled the wrong assumption across every iteration. Constraint parameters must be sourced from the controlling code or ordinance with citations and verified like any code analysis.

### Building An Opaque Workflow That Only The Author Can Run

The graph is a tangle of unlabeled nodes, the logic lives only in the author's head, and when the author is unavailable the workflow cannot be run, modified, or audited. The mechanism is that documentation feels like overhead during the momentum of design and the author understands the graph, and the false signal is that the workflow works and so it is fine. The harm is that the workflow dies on the author's departure, the team cannot verify its logic, and the design cannot be re-evaluated against changed conditions. The graph must be structured, labeled, documented, and version-controlled so that any competent team member can read and re-run it.

### Letting The Tool Drive The Form Without Design Judgment

The team generates hundreds of options, picks the one that scores best on the objective function, and presents it as the design, without the architect exercising judgment on proportion, context, material, or experience. The mechanism is that computational output feels objective and design judgment feels subjective, and the false signal is that the optimized option is by definition the best. The harm is that the building is formally or experientially deficient in ways the metric did not capture, and the team has abdicated the architect's core responsibility. Computational options are inputs to design judgment, not substitutes for it, and the architect must curate, reject, and synthesize rather than simply select the top score.

### Failing To Reproduce Or Hand Off The Workflow

The presented design was produced by a specific version of the graph with specific inputs, but neither was recorded, so when the client asks to explore a variant the team cannot reproduce the result and must rebuild from scratch. The mechanism is that version control feels like software engineering overhead and the inputs feel ephemeral, and the false signal is that the design is captured in the drawings. The harm is that the design cannot be re-evaluated, the workflow is lost, and the team cannot respond to changes without starting over. The graph, its version, its dependencies, and the input sets that produced issued options must be recorded and handed off.

## Self-Check

- [ ] Is the design question — objective, constraints, and evaluation metrics — stated in writing before the graph is built, and owned by the architect?
- [ ] Are code, zoning, and program constraints encoded as explicit parameters sourced from the controlling documents, with citations, and updated when the project constraints change?
- [ ] Is the graph structured with named groups, labeled nodes, readable code, and documentation so that any competent team member can audit and re-run it?
- [ ] Have computational outputs been validated against hand calculations or known reference cases, with the validation documented?
- [ ] Are units, list structure, and tolerances managed deliberately so that the workflow is deterministic and reproducible?
- [ ] Does the objective function reflect criteria and weights agreed with the client, with tradeoffs documented rather than hidden?
- [ ] Are scripts under version control, with input sets recorded for every presented option and dependencies documented for handoff?
- [ ] Has the architect exercised design judgment over the computational outputs, rather than deferring to the top-scoring option?
