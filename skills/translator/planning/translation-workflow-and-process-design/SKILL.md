---
name: translation_workflow_and_process_design.md
description: Use when the agent is designing a translation workflow or production process, deciding on stages and handoffs, choosing between human translation machine translation and post-editing, defining review and QA gates, or structuring how content moves from source to localized deliverable across teams and tools.
---

# Translation Workflow And Process Design

A translation workflow is the sequence of stages, decisions, and handoffs that move content from source to localized deliverable. It is easy to treat workflow as a detail to be worked out during production, but the workflow determines almost everything about cost, speed, consistency, and quality. A workflow that routes high-risk content through raw machine translation ships dangerous errors. A workflow with no review gate delivers unverified content to users. A workflow that hands off files without context produces translators who guess. A workflow that captures no post-editing data prevents the MT engine from ever improving. Workflow design is where the strategy of a translation program becomes operational, and a poorly designed workflow quietly undermines good linguists and good tools. The work is to make the stages explicit, assign the right process to each content type, define the gates that protect quality, and ensure the handoffs between stages carry the information each stage needs.

Use this skill when designing a translation workflow, defining stages and handoffs, choosing among human translation, machine translation, and post-editing, or structuring how content moves from source to deliverable. The goal is to design a process that produces the required quality efficiently, with explicit gates and clean handoffs.

## Core Rules

### Map The End-To-End Process Before Assigning Stages

Before deciding who does what, map the full process a piece of content must pass through from source to delivery. Designing stages in isolation produces gaps.

A complete workflow typically includes source preparation and finalization, content extraction and handoff to the translation system, pre-translation leveraging from translation memory and termbase, the production stage, whether human translation, MT, or post-editing, review and revision, in-country or client review where applicable, QA and quality measurement, and final formatting, assembly, and delivery. Map these stages for each content type, because different content may follow different paths. Identifying the full path first prevents the common failure where a stage is assumed but never assigned an owner.

A workflow with an unmapped stage is a workflow with a hole that content falls through.

### Match The Production Method To Content Risk And Volume

The production method, human translation, machine translation, raw or post-edited, or a mix, must match the content's risk profile and volume. This is the central workflow decision.

High-risk content, legal, medical, safety, financial, and regulatory, warrants human translation by qualified linguists, because the consequence of MT error is too high. High-volume, low-risk content, such as support articles or internal communications, may suit MT with light post-editing or even raw MT, because speed and cost matter more than perfection. Medium-risk content may suit MT with full post-editing, combining speed with publication quality. Match the method to both risk and volume: a low-risk but brand-visible content may still warrant human translation for quality, while a high-risk but low-volume content cannot be economized through MT.

Defaulting every content type to the same method either over-spends on low-risk volume or under-protects high-risk content.

### Define Explicit Quality Gates

A workflow without quality gates delivers whatever the production stage produced, verified or not. Define gates where quality is checked before content advances.

Place a gate after production, where the translator or post-editor confirms completeness and self-checks. Place a gate after review, where the reviewer confirms the content meets the quality target. Place a final gate before delivery, where format, terminology, and completeness are verified against acceptance criteria. Define what each gate checks, who owns it, and what happens when content fails, whether it returns for rework or escalates. Gates are what prevent unverified or defective content from reaching delivery.

A gate that is assumed but not defined is not a gate; it is a hope.

### Design Handoffs That Carry Required Context

Most workflow failures occur at handoffs, where content moves between stages or people. Design handoffs to carry the information the next stage needs.

A handoff should include the source content in its final version, the termbase and translation memory, the style guide and brief, reference materials and context such as screenshots or builds, the query log and decisions made so far, the quality target and acceptance criteria, and the deadline. A handoff that delivers only files and a date leaves the next stage to guess at requirements, producing inconsistent quality. Standardize handoff packages so every stage receives the same complete set of inputs.

Clean handoffs are the difference between a workflow that produces consistent quality and one that produces surprises.

### Leverage Translation Memory And Termbase In The Workflow

Translation memory and termbase are not passive assets; they must be integrated into the workflow to create leverage. Design the workflow to use them.

Configure pre-translation to apply translation memory matches and termbase terms before human or machine production, so linguists start from leverage rather than from blank. Configure the workflow to capture every translation decision back into memory and the termbase, so leverage compounds over time. Ensure terminology is enforced at the production and review stages, not just checked at the end. A workflow that does not leverage memory and termbase pays full cost for every segment forever and accumulates inconsistency.

Leverage is a workflow property, not a tool property; the tools provide capability, the workflow realizes it.

### Integrate Machine Translation Deliberately

When machine translation is part of the workflow, integrate it deliberately, because MT affects every other stage.

Decide where MT sits in the process: as pre-translation that a post-editor corrects, as raw output for low-risk content, or as a suggestion tool within a CAT environment. Decide how MT output is evaluated before it enters post-editing, because routing poor MT to post-editing wastes effort. Decide how post-editing data is captured to improve the engine, because without feedback the MT never improves. Decide confidentiality controls, because MT can expose sensitive content if it sends data to external engines. MT is not a plug inserted into a workflow; it reshapes the workflow around it.

### Plan For Content Updates And Continuous Localization

Content is rarely translated once. Source content evolves, and the workflow must handle updates, not just initial translation. Design for continuity.

Define how source updates are detected, how they trigger re-translation of affected segments, how translation memory identifies unchanged content to avoid rework, and how updated content flows through the same gates. For products with frequent updates, establish a continuous localization process integrated with the content pipeline, rather than treating each update as a separate project. A workflow designed only for one-time translation breaks down under ongoing updates, producing drift and inconsistency.

### Assign Clear Ownership For Each Stage

Every stage needs an owner who is accountable for its output. A stage with no owner is a stage no one is responsible for.

Assign ownership for source finalization, production, review, QA, terminology, and delivery. Clarify whether ownership sits with an individual, a vendor, or a function. Ensure owners have the authority and information to fulfill their stage. A workflow with ambiguous ownership produces finger-pointing when defects surface, because no one owns the stage where the defect originated.

## Common Traps

### Designing Stages In Isolation

Stages designed without mapping the full process leave gaps where content and accountability fall through.

### Using One Production Method For All Content

Defaulting everything to human translation or to MT either over-spends on low-risk volume or under-protects high-risk content.

### Assuming Quality Gates Without Defining Them

A gate that is not explicitly defined, owned, and checked is not a gate; defective content passes through it.

### Handoffs That Deliver Files Without Context

Handoffs without termbase, style guide, references, and decisions leave the next stage guessing and produce inconsistent quality.

### Failing To Leverage Memory And Termbase

A workflow that does not apply and capture memory and termbase pays full cost forever and accumulates inconsistency.

### Inserting MT Without Reshaping The Workflow

MT is not a plug; it changes evaluation, post-editing, data capture, and confidentiality across the workflow.

### Designing Only For One-Time Translation

Workflows that ignore updates break under continuous content evolution, producing drift and inconsistency.

### Stages Without Clear Ownership

Ambiguous ownership leads to finger-pointing when defects surface and to stages no one is accountable for.

## Self-Check

Before approving a translation workflow or process design, verify:

- The end-to-end process is mapped for each content type, with every stage identified and owned.
- The production method, human translation, MT, or post-editing, is matched to each content type's risk and volume.
- Quality gates are explicitly defined after production, review, and before delivery, with owners and failure handling.
- Handoffs are standardized to carry source, termbase, memory, style guide, references, query log, target, and deadline.
- Translation memory and termbase are integrated for pre-translation leverage and for capturing decisions back into assets.
- Machine translation, if used, is integrated deliberately with evaluation, post-editing, data capture, and confidentiality controls.
- The workflow handles content updates and continuous localization, not just one-time translation.
- Every stage has a clear owner with the authority and information to fulfill it.
- No high-risk content is routed through a method that cannot protect its consequence, and no gate is assumed without being defined.
- The workflow produces the required quality efficiently, with explicit gates and clean, complete handoffs.
