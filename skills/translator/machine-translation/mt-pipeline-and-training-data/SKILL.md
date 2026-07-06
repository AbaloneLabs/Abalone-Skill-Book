---
name: mt_pipeline_and_training_data.md
description: Use when the agent is building or advising on a machine translation pipeline, curating training data and parallel corpora for custom MT engines, managing data quality confidentiality and domain adaptation, or integrating MT with translation memory termbase and post-editing workflows.
---

# MT Pipeline And Training Data

A machine translation pipeline is the system that produces, delivers, and improves MT output across an organization's content. It is not a single tool but a chain of decisions: which engine to use, what data to train or fine-tune it on, how to integrate it with translation memory and termbases, how to route content to raw MT or post-editing, how to capture post-editing data to improve the engine, and how to govern quality, confidentiality, and cost over time. Each decision affects the others, and weak links undermine the whole. Poor training data produces an engine that confidently errs. Missing confidentiality controls expose sensitive content. Disconnection from translation memory wastes leverage. Failure to capture corrections means the engine never improves. Designing and maintaining an MT pipeline is an engineering and governance task, not a one-time setup, and it determines whether MT creates value or risk.

Use this skill when building, advising on, or maintaining an MT pipeline, curating training data, managing domain adaptation, or integrating MT with translation workflows. The goal is to build a pipeline that produces useful MT output while protecting confidentiality, leveraging existing assets, and improving over time.

## Core Rules

### Choose The Engine Type For The Use Case

Engine choice determines quality, cost, control, and confidentiality. Choose based on the use case, not on fashion.

Generic cloud MT offers fast integration, broad language coverage, and no infrastructure, but offers less control, may store content, and may underperform on specialized domains. Custom-trained neural MT, fine-tuned on domain data, offers better domain quality and control but requires data, expertise, and infrastructure. Open-source or self-hosted MT offers confidentiality and control but demands engineering effort. The right choice depends on volume, domain specialization requirements, confidentiality constraints, language pairs, and available expertise.

Document the rationale for the engine choice, including what was considered and rejected.

### Curate Training Data For Quality

Custom MT quality depends on training data quality. Curate data deliberately, because bad data produces a confidently wrong engine.

Collect parallel corpora, source-target pairs, from authoritative sources such as approved past translations, aligned translation memory, published parallel texts, and domain corpora. Clean the data by removing misaligned pairs, corrupted segments, source-target mismatches, duplicates that skew training, and content with quality issues. Balance the data across domains and registers the engine must handle. Tag data with metadata such as domain, client, and date to enable selective training. More data is not always better; clean, relevant data outperforms large noisy data.

Document the data sources, cleaning steps, and composition so the training set is reproducible and defensible.

### Manage Confidentiality And Data Rights

Training data and MT queries can expose confidential content. Manage confidentiality and data rights explicitly.

For cloud MT, confirm the provider's data handling: whether queries are stored, used for training, or shared. Use enterprise agreements that prohibit reuse where required, and never send confidential content to consumer-grade MT. For custom training, ensure you have rights to use the training data, especially client-owned translations, and that training an engine on one client's data does not leak it to another client's output. Anonymize or exclude personal data and trade secrets from training sets.

Confidentiality breaches in MT pipelines can violate contracts, privacy law, and client trust, and they are often invisible until damage is done.

### Integrate MT With Translation Memory And Termbases

MT delivers the most value when integrated with existing translation assets. Integrate it deliberately.

Configure the pipeline so translation memory matches take precedence over MT, because approved past translations are more reliable. Apply termbase terminology to MT output, either before output through constrained decoding where supported or during post-editing. Feed post-edited content back into translation memory to grow the asset. Use MT primarily for segments without good TM matches, where it adds the most leverage. Measure the leverage, how much content MT handles versus TM, to evaluate the pipeline's value.

A pipeline that runs MT without leveraging TM wastes the organization's most reliable asset.

### Route Content By Suitability

Not all content suits MT. Route content by suitability to avoid wasting effort and creating risk.

Route high-volume, lower-stakes content such as support articles, product catalog, and user-generated content to MT with appropriate post-editing. Route high-stakes content such as legal, medical, safety, and regulatory to human translation, not MT. Route creative and marketing content to human transcreation. Use metadata, content type, and risk classification to automate routing so content flows to the right process without manual decision each time.

Routing everything through MT regardless of suitability produces both quality failures on hard content and wasted human effort on easy content.

### Capture Post-Editing Data For Improvement

Post-editing data is the pipeline's most valuable improvement signal. Capture and use it.

Log post-edits, the changes made to MT output, and feed them back as training data to fine-tune the engine. Track which segments required heavy editing to identify MT weaknesses. Use edit distance and time metrics to measure MT quality over time and detect degradation. Periodically retrain or fine-tune the engine with accumulated clean post-editing data, while guarding against training on post-editor errors or inconsistent corrections.

A pipeline that does not capture post-editing data never improves and may degrade as content and language evolve.

### Govern Quality Over Time

MT quality is not static. Govern it over time with monitoring and maintenance.

Monitor quality through sampling, post-editor feedback, and automated metrics. Watch for quality drops that can result from domain drift, source content changes, or engine issues. Maintain the termbase and feed terminology updates to the engine or post-editing workflow. Schedule periodic retraining with fresh, clean data. Assign ownership of MT quality so someone is responsible for monitoring and improvement, because ungoverned pipelines decay silently.

### Measure Cost Quality And Turnaround Holistically

Evaluate the pipeline on cost, quality, and turnaround together, not in isolation. Optimize the balance, not one dimension.

MT typically reduces cost and turnaround but may reduce quality on hard content. Post-editing raises quality but adds cost and time. Translation memory reduces all three when it matches. Measure the actual outcomes, cost per word, quality scores, turnaround time, and reviewer rework, for each content type and route. Use the measurements to adjust routing, engine choice, and post-editing levels. A pipeline optimized only for cost will publish poor content; one optimized only for quality will be too slow and expensive.

## Common Traps

### Choosing An Engine Without Considering Confidentiality

Sending confidential content to an unsuitable engine, or training on data without rights, creates legal and trust risk.

### Training On Noisy Or Unbalanced Data

Large dirty data produces a confidently wrong engine; clean relevant data outperforms volume.

### Running MT Without Leveraging Translation Memory

Ignoring TM wastes the most reliable asset and reduces quality where TM would have helped.

### Routing All Content Through MT

High-stakes or creative content routed through MT produces quality failures and risk.

### Not Capturing Post-Editing Data

Without feedback, the engine never improves and may degrade as content evolves.

### Assuming MT Quality Is Static

Ungoverned pipelines decay; without monitoring and retraining, quality drops silently.

### Optimizing Only For Cost

Cost-only optimization publishes poor content; balance cost, quality, and turnaround.

### Leaking One Client's Data Into Another's Engine

Training an engine on one client's translations that then influences another's output breaches confidentiality.

## Self-Check

Before approving or maintaining an MT pipeline, verify:

- The engine type was chosen for the use case, with documented rationale considering quality, control, cost, and confidentiality.
- Training data is curated for quality, cleaned, balanced, tagged, and documented for reproducibility.
- Confidentiality and data rights are managed, with provider handling confirmed, content rights verified, personal data anonymized, and cross-client leakage prevented.
- MT is integrated with translation memory and termbases, with TM precedence, terminology application, and post-edit feedback to TM.
- Content is routed by suitability, with high-stakes and creative content sent to human processes and routing automated by metadata.
- Post-editing data is captured and fed back for engine improvement, with edit metrics tracked.
- Quality is governed over time through monitoring, termbase maintenance, periodic retraining, and assigned ownership.
- Cost, quality, and turnaround are measured holistically per content type and route, and routing is adjusted based on outcomes.
- No confidential content reaches an unsuitable engine, and no client's data leaks into another's output.
- The pipeline produces useful MT output while protecting confidentiality, leveraging assets, and improving over time.
