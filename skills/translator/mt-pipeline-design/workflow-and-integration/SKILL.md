---
name: workflow_and_integration.md
description: Use when the agent is designing or integrating a machine translation workflow into a content pipeline, connecting MT with translation memory and termbases, routing content by type and risk to raw MT light post-editing full post-editing or human translation, automating content handoff from CMS or source repositories, capturing post-editing feedback, or building the connectors and orchestration that move content through an MT-enabled localization process.
---

# Workflow And Integration

An MT workflow is the orchestration that moves content from its source system, through translation memory matching, terminology application, machine translation, post-editing, quality checks, and back to publication, and its quality determines whether MT creates value or chaos. The workflow is not the engine; a strong engine inside a broken workflow delivers late, inconsistent, or leaked content, while a modest engine inside a well-designed workflow can outperform a stronger engine that is poorly integrated. The integration decisions are where leverage is won or lost: whether translation memory matches take precedence over MT, whether terminology is applied before or after translation, whether content is routed automatically by type and risk or requires manual decision each time, whether post-editing corrections are captured to improve the engine and grow the memory, and whether the whole chain preserves the structure, markup, and confidentiality of the source. Agents miss the workflow because they focus on the engine and treat integration as plumbing, but integration is where the real cost, quality, and risk live.

The harm this skill prevents is MT deployments that publish inconsistent content, waste the organization's translation memory assets, expose confidential data through the wrong engine, route high-stakes content through MT by accident, or fail to capture the corrections that would let the system improve. The agent's freedom is to design the workflow, but every routing, precedence, and feedback decision must be deliberate, documented, and aligned with the content's risk profile.

## Core Rules

### Give Translation Memory Precedence Over MT

Translation memory holds the organization's own approved past translations, which are more reliable and more consistent than any MT output for the segments they cover. Configure the workflow so that high-quality TM matches, exact and high-fuzzy, are applied before MT and are not overwritten by it, because letting MT replace good TM wastes the most trustworthy asset and introduces inconsistency. Use MT primarily for segments without good TM matches, where it adds the most leverage, and feed post-edited content back into TM so the asset grows. Measure the leverage, the share of content handled by TM versus MT versus human translation, because that ratio tells you whether the workflow is extracting value from existing assets or duplicating effort. A workflow that runs MT without consulting TM is paying to retranslate what it already owns.

### Apply Terminology Before And After Translation

Terminology consistency is where MT workflows most often fail, because MT renders the same term differently across segments and a post-editor who fixes segments in isolation leaves the document inconsistent. Integrate the termbase so that approved terms are applied as early as possible, through constrained decoding or pre-translation glossary injection where the engine supports it, and again during post-editing as a QA check that flags deviations. Make the termbase the single source of truth for terminology across TM, MT, and post-editing, so that all three align. Without termbase integration, the workflow produces documents whose terminology reflects MT's drift rather than the organization's standard, and that drift is the defect readers notice first.

### Route Content Automatically By Type And Risk

Manual routing, deciding per content item whether to use raw MT, light post-editing, full post-editing, or human translation, does not scale and produces inconsistent decisions. Automate routing using metadata: content type, domain, audience, risk classification, and shelf life. Route high-volume, lower-stakes content such as support articles and catalogs to MT with appropriate post-editing; route high-stakes content such as legal, medical, safety, and regulatory to human translation, not MT; route creative and marketing to human transcreation. Define the routing rules explicitly, encode them in the orchestration, and review them periodically as content and risk evolve. Automated routing ensures that high-stakes content never reaches MT by accident and that low-stakes content is not over-handled.

### Preserve Structure, Markup, And Context Through The Pipeline

Content arrives in the workflow embedded in structure: markup, tags, placeholders, variables, formatting, and document structure. The workflow must preserve this structure through TM matching, MT, and post-editing, so that what comes out can be published without manual reassembly. Use parsers and filters that protect markup, configure the engine to handle tags correctly, and verify in QA that tags, placeholders, and variables are intact and correctly placed in the output. For concatenated and templated content, verify the assembled result, not just the fragments. Structural corruption is a frequent and expensive failure, because it is invisible until publication breaks, and it forces manual rework that erases the efficiency MT was meant to provide.

### Capture Post-Editing Feedback To Improve The System

Post-editing corrections are the workflow's most valuable improvement signal, and a workflow that does not capture them cannot learn. Log every post-edit, the change from MT output to final, and feed clean corrections back as training data for engine fine-tuning and as new TM entries. Track edit distance and time per segment to identify where MT is weak and where post-editing effort concentrates. Guard the feedback quality: do not feed back corrections that reflect post-editor error or inconsistency, and deduplicate so that repeated corrections do not over-weight a single pattern. A workflow without a feedback loop is static; it delivers the same quality forever and decays as content and language drift.

### Enforce Confidentiality And Data Handling At The Integration Layer

Confidentiality is a workflow property, not just an engine property, because the integration determines where content is sent and what is exposed. Classify content by confidentiality at ingestion and route confidential content only to engines that contractually protect it, never to consumer-grade or public MT. For custom and self-hosted engines, verify the deployment boundary and that no telemetry or logging leaks content. Ensure that one client's content does not train or influence an engine that serves another client. Build confidentiality controls into the orchestration so that protection is automatic, not dependent on an editor remembering. Confidentiality breaches in MT workflows are often invisible until they cause contractual or legal damage.

### Make The Workflow Observable And Auditable

A workflow that cannot be observed cannot be improved or trusted. Instrument the pipeline so that every content item carries its routing decisions, the TM and MT steps applied, the post-editing level and effort, the QA results, and the final disposition. Log enough to reconstruct what happened to any segment, which is essential for diagnosing defects, defending quality, and feeding improvement. Make the workflow auditable for confidentiality, showing that confidential content never reached an unapproved engine. Observability is what turns MT from a black box into a managed system, and it is the precondition for the optimization and monitoring that should follow.

### Design For Fallback And Graceful Degradation

MT engines and integrations fail: the engine times out, a connector breaks, a model is updated and degrades, or a language pair drops in quality. Design the workflow with fallback so that failure does not block publication silently: fall back to TM-only, to human translation, or to a queued retry, and alert the team rather than publishing raw unverified output or dropping content. Graceful degradation means the workflow fails safely, routing to a slower or more expensive but reliable path, rather than failing unsafely by publishing wrong content. Test the fallback paths before you need them.

## Common Traps

### Running MT Without Leveraging Translation Memory

Ignoring TM retranslates what the organization already owns and reduces quality where TM would have helped. Give TM precedence.

### No Termbase Integration Across TM, MT, And Post-Editing

Without termbase integration, terminology drifts and documents ship inconsistent. Make the termbase the single source of truth.

### Manual Routing That Does Not Scale

Per-item manual routing produces inconsistent decisions and does not scale. Automate by metadata and review periodically.

### Structural Corruption Through The Pipeline

Markup, tags, and placeholders damaged in transit break publication and force manual rework. Protect structure and verify in QA.

### No Feedback Loop From Post-Editing

Without captured corrections, the engine never improves and TM does not grow. Log, clean, and feed back post-edits.

### Confidentiality Treated As An Engine Concern Only

The integration decides where content goes. Build confidentiality classification and routing into the orchestration.

### An Unobservable, Unauditable Workflow

A black-box workflow cannot be diagnosed, improved, or defended. Instrument routing, steps, and disposition for every item.

### No Fallback When The Engine Or Integration Fails

Failure without fallback blocks publication or publishes unverified output. Design and test graceful degradation.

## Self-Check

- Does translation memory take precedence over MT for exact and high-fuzzy matches, with post-edited content fed back to grow TM, and is the TM-versus-MT leverage measured?
- Is the termbase integrated as the single source of truth, applied before translation where possible and checked during post-editing, so terminology is consistent across TM, MT, and editing?
- Is content routed automatically by type, domain, audience, risk, and shelf life, with high-stakes content sent to human translation and rules reviewed periodically?
- Does the workflow preserve markup, tags, placeholders, variables, and document structure through all steps, verified in QA on the assembled output?
- Are post-editing corrections captured, cleaned, deduplicated, and fed back as training data and TM entries, with edit distance and time tracked to locate MT weakness?
- Is content classified by confidentiality at ingestion and routed only to contractually protected engines, with no cross-client leakage and no unapproved public MT for confidential content?
- Is the workflow instrumented so every item's routing, steps, post-editing level, QA results, and disposition are logged and auditable?
- Are fallback paths designed and tested so engine or integration failure routes to a reliable path with alerting rather than publishing unverified output?
- No high-stakes content reaches MT by accident, and no confidential content reaches an unapproved engine.
- The workflow is observable, auditable, and capable of improvement through its feedback loop.
