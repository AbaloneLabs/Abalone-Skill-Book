---
name: corpus_preparation_and_curation.md
description: Use when the agent is assembling, cleaning, aligning, deduplicating, or curating a parallel corpus for machine translation training, sourcing parallel data from translation memories and published corpora, segmenting and language-filtering bilingual data, deciding what to include or exclude from a training set, or documenting corpus provenance and composition for reproducible training.
---

# Corpus Preparation And Curation

A custom MT engine is, more than anything else, a function of its training corpus, and corpus preparation is where engine quality is won or lost before a single training step runs. The work is unglamorous and easy to underestimate: collecting parallel data from scattered sources, aligning it at the sentence level, filtering out the noise, deduplicating, language-checking, balancing across domains, and documenting every decision so the corpus can be reproduced and defended. The dominant temptation is to maximize volume, on the assumption that more data means a better engine. That assumption is false. A large corpus polluted with misaligned pairs, wrong-language segments, machine-translated content presented as human, and domain mismatches produces an engine that is confidently wrong in the directions the corpus was wrong. Curation is the discipline of choosing what to exclude as much as what to include, because bad data teaches the engine bad habits that surface as fluent errors in production. Agents miss this because cleaning is invisible work and a corpus looks impressive by size, while the defects that ruin training are visible only to someone who inspects the data.

The harm this skill prevents is engines trained on polluted, misaligned, or unrepresentative data that produce systematic errors no amount of tuning removes, and training runs that cannot be reproduced or defended because no one documented what was in the corpus. The agent's freedom is to compose the corpus, but every inclusion, exclusion, and transformation must be deliberate and recorded.

## Core Rules

### Prioritize Quality And Relevance Over Volume

The single most important rule is that clean, relevant data outperforms large noisy data. A modest corpus of well-aligned, human, in-domain parallel text will train a better engine for that domain than a massive corpus diluted with out-of-domain and noisy data. Resist the volume reflex. Evaluate every data source for whether it is human translation, whether it is in-domain or adjacent, whether it is well-aligned, and whether its register and style match the target output. Prefer fewer high-quality sentence pairs over many questionable ones. When in doubt about a source, exclude it and measure the effect of adding it back in a controlled experiment rather than assuming more is better.

### Align At The Sentence Level And Verify Alignment Quality

Parallel data rarely arrives perfectly aligned, and misaligned pairs are among the most damaging contaminants because they teach the engine false correspondences. Run sentence-level alignment with a robust aligner, then verify alignment quality on samples, checking that source and target segments actually correspond. Filter out pairs with extreme length ratios, which usually indicate misalignment, and pairs where one side is empty or a fragment. For document-level sources, verify that the alignment did not drift across the document. Misalignment is invisible in aggregate statistics and visible only in inspection, so inspect samples from the beginning, middle, and end of each source.

### Filter Aggressively For Language, Noise, And Provenance

A parallel corpus must contain the languages it claims, in the registers it claims, without noise. Language-identify every segment and remove pairs where either side is not the expected language, because corpora routinely contain wrong-language segments from boilerplate, navigation, or mixed-language documents. Remove noise: HTML remnants, encoding artifacts, OCR errors, repeated boilerplate, navigation strings, and segments that are not natural language. Remove pairs that are themselves machine translation presented as human, because training on MT output, especially from a different system, propagates that system's errors and collapses quality. Tag every segment with provenance, source, domain, and date, so you can later filter or weight by origin and so the corpus composition is auditable.

### Deduplicate At The Right Granularity

Duplicates skew training by over-weighting repeated content, and near-duplicates, the same sentence with minor variation, compound the effect. Deduplicate at segment level to remove exact repeats, and consider fuzzy deduplication to reduce near-duplicate clusters that would otherwise dominate. Beware over-deduplication: legitimate recurring sentences, such as formulaic legal or UI phrases, carry real signal and should not all be removed. Also deduplicate against the test and validation sets, because any overlap between training and evaluation data invalidates the evaluation and inflates scores. Treat train-evaluation leakage as a critical defect to check before trusting any metric.

### Balance Across Domains, Registers, And Lengths

An engine trained on an unbalanced corpus performs well on the over-represented domains and poorly on the rest. Examine the corpus composition by domain, register, formality, sentence length, and content type, and rebalance by up-weighting under-represented but important domains or down-weighting over-represented but less relevant ones. Balance is not equality; it is matching the corpus distribution to the production distribution the engine will face, with extra weight on high-value or high-risk content. Document the target distribution and the actual composition, and revisit the balance when production content shifts.

### Separate Training, Validation, And Test Sets Cleanly

The integrity of evaluation depends on clean separation of training, validation, and test data, and leakage between them is the most common cause of inflated, misleading results. Hold out validation and test sets before any augmentation or deduplication that could leak, ensure no document appears across splits when document-level information could leak, and verify zero overlap after preprocessing. Make the split deterministic and documented so it can be reproduced. A test set that overlaps training tells you nothing about real performance and leads to deploying engines that underperform expectations.

### Document Provenance, Composition, And Transformations

A corpus that cannot be reproduced cannot be defended, and an engine trained on undocumented data cannot be audited for bias, confidentiality, or rights. Document every source with its origin, license, and date of acquisition; every transformation, alignment, filtering, deduplication, and balancing step with its parameters; and the final composition by source, domain, language variety, and size. Store the cleaning pipeline as versioned code so the corpus can be rebuilt from sources. Provenance documentation is what lets you answer the questions that arise later: why does the engine mishandle this domain, does the corpus contain content we did not have rights to use, can we reproduce this training run.

### Respect Licensing, Confidentiality, And Rights

Corpus preparation is where legal and ethical risk enters the engine. Verify the license of every source and that it permits the intended use, including training and redistribution where relevant. For client-owned translation memory, confirm the right to use it for training and ensure one client's data does not leak into an engine that serves another. Remove or anonymize personal data, confidential content, and trade secrets unless their use is explicitly authorized. A corpus built on data without rights creates liability that surfaces long after the engine is deployed, and confidentiality breaches in training data are often invisible until damage is done.

## Common Traps

### Maximizing Volume At The Expense Of Quality

Large noisy data produces a confidently wrong engine. Clean relevant data outperforms volume; exclude dubious sources.

### Trusting Alignment Without Verifying

Misaligned pairs teach false correspondences and are invisible in aggregate stats. Inspect alignment samples.

### Training On Machine-Translated Data Presented As Human

MT output in training propagates that system's errors and collapses quality. Detect and exclude MT-contaminated data.

### Ignoring Language Identification

Wrong-language segments from boilerplate or mixed documents pollute the corpus. Language-filter every segment.

### Train-Evaluation Leakage From Overlap

Overlap between training and test or validation inflates scores and invalidates evaluation. Deduplicate across splits and verify zero overlap.

### Unbalanced Corpus Hiding Domain Weakness

Over-represented domains dominate; under-represented ones perform poorly. Balance to the production distribution.

### Undocumented Corpus That Cannot Be Reproduced

Without provenance and transformation records, the corpus cannot be audited or rebuilt. Document and version everything.

### Using Data Without Rights Or With Confidentiality Leakage

Unlicensed or client-confidential data creates legal and trust risk. Verify licenses and isolate client data.

## Self-Check

- Was quality and relevance prioritized over volume, with each source evaluated for being human, in-domain, well-aligned, and register-appropriate?
- Is sentence-level alignment verified on samples from the beginning, middle, and end of each source, with extreme length-ratio and fragment pairs removed?
- Has every segment been language-identified and filtered, with noise such as HTML, OCR errors, and boilerplate removed?
- Has machine-translated content presented as human been detected and excluded to avoid propagating another system's errors?
- Is deduplication applied at the right granularity, with train, validation, and test overlap checked and verified as zero?
- Is the corpus balanced across domains, registers, lengths, and content types to match the production distribution, with composition documented?
- Are training, validation, and test sets cleanly separated, deterministic, documented, and verified for leakage including document-level leakage?
- Is full provenance documented: sources, licenses, acquisition dates, transformations, parameters, and final composition, with the pipeline stored as versioned code?
- Are licensing, confidentiality, and rights verified for every source, with personal and confidential data removed or authorized and client data isolated?
- The corpus can be rebuilt from sources and its composition defended against questions of bias, rights, and reproducibility.
