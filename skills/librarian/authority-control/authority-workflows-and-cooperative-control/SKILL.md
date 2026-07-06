---
name: authority_workflows_and_cooperative_control.md
description: Use when the agent is setting up or managing authority control workflows, integrating authority work into the cataloging process, planning batch authority processing, coordinating with cooperative authority programs, or maintaining authority data quality over time.
---

# Authority Workflows And Cooperative Control

Authority control fails when it is treated as something a cataloger does once and forgets. Names drift, standards change, vendor records arrive with inconsistent headings, legacy data accumulates, and without a workflow the authority file decays until collocation breaks and users cannot find what they need. Good authority control is a system: it is built into the cataloging process, automated where possible, reviewed on a cycle, and connected to cooperative programs so the institution benefits from and contributes to shared authority data. Designing this system is a different skill from establishing a single authority record. It requires deciding where authority work happens in the workflow, how batch processing catches problems at scale, how local and cooperative authorities interact, and how the file is maintained as a living asset rather than a static archive.

Use this skill when designing or managing authority control workflows, planning batch or vendor authority processing, integrating authority work into cataloging operations, or coordinating with cooperative authority programs. The goal is to prevent the agent from treating authority control as a per-record afterthought, relying on manual cleanup that never happens, ignoring cooperative opportunities, or letting the authority file silently decay.

## Core Rules

### Build Authority Control Into The Cataloging Workflow

Authority work is most efficient and accurate when it happens at the point of cataloging, not as a separate cleanup pass. The workflow should make the right action the easy action.

Workflow integration:

- the cataloging interface should flag headings that lack matching authorities;
- new authorities should be created or searched as part of record creation;
- copy cataloging should include verification of authority links;
- batch-loaded records should be routed through authority validation;
- local practice should define when to create, search, or defer authority work.

When authority control is a separate optional step, it gets skipped under time pressure. Build it in so it cannot be bypassed without a deliberate decision.

### Separate New Authority Creation From Routine Cataloging

Not every cataloger should establish new authorities, and not every cataloging situation warrants it. Distinguish the decisions.

Typical separation:

- routine cataloging searches existing authorities and links to them;
- creating a new cooperative authority may require trained staff and the contribution process;
- local authorities for minor entities may be created by any cataloger with documentation;
- complex disambiguation or conflict resolution goes to experienced authority staff;
- a tiered routing system matches the task to the right expertise.

A workflow where every cataloger independently creates authorities produces inconsistency and duplicates. Route authority creation deliberately.

### Plan Batch Authority Processing For Scale

Individual authority work cannot keep up with large catalogs or vendor loads. Batch processing validates, links, and corrects headings at scale.

Batch processing covers:

- validating existing headings against the authority file;
- linking unmatched headings to authorities where a confident match exists;
- flagging ambiguous or unmatched headings for manual review;
- applying authorized forms to replace variants in bibliographic records;
- reporting statistics on match rates and problem areas.

Batch processing is powerful but blunt. Configure it to flag low-confidence matches for review rather than auto-applying them, since a wrong batch link is harder to detect than a manual error.

### Configure Validation And Matching Rules Carefully

Batch authority processing is only as good as its matching rules. Over-aggressive matching creates wrong links; over-cautious matching leaves headings unlinked.

Matching considerations:

- exact string match is safe but misses legitimate variants;
- fuzzy matching catches variants but risks false positives, especially for common names;
- date and title context improves match confidence;
- set a confidence threshold above which auto-linking occurs and below which review is required;
- exclude known problem areas, prolific authors or common names, from auto-linking.

Review the matching rules against a sample before running at scale, and audit the results afterward. A batch error propagated across thousands of records is a major cleanup.

### Coordinate With Cooperative Authority Programs

Cooperative authority files multiply the value of local work and reduce duplication. Coordination is a workflow decision, not just a per-record one.

Cooperative integration:

- use the LC Name Authority File or national equivalent as the primary source;
- contribute new authorities through SACO or the relevant program;
- report errors in cooperative authorities through the proper channels;
- synchronize local authority data with cooperative updates on a schedule;
- participate in program training and documentation.

A workflow that ignores cooperative files recreates work already done elsewhere and isolates local data. Build cooperation into the routine.

### Handle Vendor And Batch-Loaded Records Deliberately

Vendor records and batch loads often arrive with headings that do not match the local authority file. A policy for handling them prevents drift.

Policy elements:

- define whether vendor headings are accepted, replaced, or validated;
- route batch loads through authority validation before loading;
- decide how to handle vendor-supplied authorities that conflict with local ones;
- set a review sample rate for large loads;
- document the load process so it is repeatable.

Uncontrolled batch loads are a primary source of authority decay. A single large load of inconsistent headings can undo months of careful work.

### Maintain A Schedule For Authority Review And Cleanup

Authority files need ongoing maintenance. A review schedule prevents decay from accumulating.

Schedule elements:

- periodic batch validation of the entire catalog against the current authority file;
- targeted cleanup of known problem areas, prolific authors, legacy headings;
- application of standard revisions, RDA updates, to affected authorities;
- merge of duplicate authorities;
- review of authorities flagged for uncertainty or pending information.

Treat authority maintenance as a budgeted recurring activity, not an occasional project. Decay is continuous, so maintenance must be too.

### Document The Authority Control System

The authority workflow involves rules, routing, batch configurations, and cooperative relationships that must outlast individual staff.

Document:

- the workflow steps and who is responsible for each;
- the matching and validation rules for batch processing;
- the policy for vendor and batch-loaded records;
- the cooperative programs used and their contribution processes;
- the review and cleanup schedule;
- local authority conventions and exceptions.

Documentation turns authority control from individual expertise into institutional capability.

## Common Traps

### Treating Authority Control As A Per-Record Afterthought

When authority work is optional and separate, it gets skipped. Build it into the cataloging workflow.

### Letting Every Cataloger Create Authorities Independently

Unrouted authority creation produces duplicates and inconsistency. Separate creation from routine linking.

### Over-Aggressive Batch Auto-Linking

Matching that auto-links low-confidence matches creates wrong links at scale. Set thresholds and flag for review.

### Ignoring Cooperative Authority Files

Recreating authorities that already exist cooperatively wastes effort and fragments data. Search and contribute cooperatively.

### Loading Vendor Records Without Authority Validation

Unvalidated batch loads introduce inconsistent headings that decay the file. Route loads through validation.

### No Scheduled Authority Maintenance

Without a review cycle, decay accumulates until collocation breaks. Budget maintenance as recurring work.

### Undocumented Authority Workflows

Complex routing and batch rules that live only in one person's knowledge die with that person. Document the system.

### Treating Batch Errors As One-Time Events

A batch error affects many records. Audit results and build safeguards, not just fixes.

## Self-Check

- Is authority control built into the cataloging workflow so it cannot be silently bypassed?
- Is new authority creation routed to appropriate expertise rather than done ad hoc by every cataloger?
- Does batch authority processing validate, link, and flag headings with appropriately configured matching rules?
- Are low-confidence batch matches routed to manual review rather than auto-applied?
- Is the institution using and contributing to cooperative authority files as part of the routine workflow?
- Do vendor and batch-loaded records pass through authority validation before entering the catalog?
- Is there a scheduled, budgeted cycle for authority review, cleanup, and standard-revision application?
- Is the entire authority control system, workflow, rules, cooperative relationships, and maintenance schedule, documented for institutional continuity?
