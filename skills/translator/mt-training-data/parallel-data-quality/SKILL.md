---
name: parallel_data_quality.md
description: Use when the agent is assessing the quality of parallel sentence pairs for machine translation training, detecting misalignment and translationese, scoring parallel data with quality estimation or cross-lingual filters, identifying synthetic or MT-contaminated pairs, handling low-resource language pairs with scarce parallel data, or setting acceptance thresholds for parallel data before training.
---

# Parallel Data Quality

Parallel data quality is the property that each source-target pair actually means the same thing, and it is the single largest determinant of custom MT quality that is entirely under the curator's control. A parallel corpus can be large, well-aligned at the sentence level, in the right languages, and still be low quality, because quality lives at the pair level: does the target faithfully translate the source, in natural target-language idiom, without omission, addition, or distortion. Most parallel data in the wild fails this test in some fraction of pairs, and that fraction is what ruins training. Published corpora contain translationese, target text that is grammatical but calqued from the source and unidiomatic. Crawled corpora contain pairs that are not translations of each other at all, merely co-occurring text. Client translation memories contain pairs where the source was edited after translation, where segments were concatenated or split, or where the translation was itself machine output. Quality estimation and cross-lingual filtering exist to find and remove these pairs, but the tools are imperfect and the judgment of what to keep remains with the curator. Agents miss pair-level quality because sentence-level statistics look clean and because inspecting pairs is slow, so the bad pairs slip through into training and emerge as the engine's systematic errors.

The harm this skill prevents is engines trained on pairs that are not real translations, which learn correspondences that do not hold and produce fluent output that is semantically wrong in characteristic ways. The agent's freedom is to set and apply quality criteria, but the criteria must address pair-level fidelity, not just corpus-level statistics, and the cost of excluding good data must be weighed against the cost of keeping bad data.

## Core Rules

### Judge Quality At The Pair Level, Not Just The Corpus Level

Corpus-level statistics, total pairs, average length, language distribution, domain coverage, describe the corpus but say nothing about whether any given pair is a correct translation. Quality lives at the pair level, and pair-level quality is what the engine learns from. Build quality assessment around sampling and scoring individual pairs, not around aggregate statistics that can look healthy while a large fraction of pairs are wrong. Inspect samples stratified by source, domain, and length, and quantify the fraction of pairs that are misaligned, incomplete, distorted, or unidiomatic. A corpus with ten percent bad pairs is a corpus in which ten percent of training signal teaches the engine errors.

### Detect And Remove Non-Translation Pairs

Crawled and assembled corpora contain pairs where source and target are not translations of each other: co-occurring text from multilingual pages, segments aligned by position rather than meaning, and boilerplate paired with unrelated boilerplate. These pairs are poison because they teach the engine arbitrary correspondences. Use cross-lingual sentence embeddings to score semantic similarity between source and target and remove pairs below a threshold; use language models to flag pairs where the target is not a plausible translation of the source; and inspect the filtered-out set to calibrate the threshold so you remove true non-translations without discarding legitimate free translations. Document the filter, its threshold, and the false-positive rate you accepted.

### Identify And Handle Translationese

Translationese is target text that is grammatically correct but structurally and lexically calqued from the source, the fingerprint of literal or hurried translation. Training heavily on translationese produces an engine that generates translationese, which reads as stilted and non-native to target-language readers. Detect translationese with features such as unnatural word order, over-literal renderings of idioms, and source-language syntactic patterns in the target. You need not remove all translationese, which would shrink many corpora drastically, but you should know how much is present, weight against it where possible, and supplement with naturally produced target text. In domains where translationese dominates, set expectations that engine output will inherit that register.

### Detect Synthetic And MT-Contaminated Pairs

Some parallel data is itself the output of machine translation, either deliberately back-translated synthetic data or accidentally MT that was presented as human translation. Synthetic data has known uses for augmentation but must be labeled and used deliberately, never mixed into a corpus as if it were human. MT-contaminated data presented as human is dangerous because it propagates the contaminating system's errors and biases into the new engine. Detect MT contamination using classifiers, repetition and fluency signatures, and provenance checks, and exclude contaminated pairs from the human-quality pool. Never train an engine on a competitor's MT output while claiming the engine learned from human translation.

### Score Pairs With Quality Estimation And Cross-Lingual Filters

Automated quality scoring lets you process millions of pairs that no human could inspect. Apply quality estimation models that score pairs without a reference, cross-lingual similarity scores that flag non-translations, and language-model fluency scores that flag unidiomatic or broken target text. Combine scores into a quality filter and tune its threshold against a human-labeled development set, measuring precision and recall: how many bad pairs it removes and how many good pairs it wrongly discards. Treat the filter as a tool with a measured error rate, not as ground truth, and re-tune when the data distribution changes. Report the filter's measured performance alongside the cleaned corpus.

### Handle Low-Resource Pairs With Extra Care

Low-resource language pairs, those with scarce parallel data, face a cruel tradeoff: every pair is precious, so the temptation is to keep marginal-quality data, but marginal data hurts more when there is little of it because each pair has higher weight in training. For low-resource pairs, invest more in human inspection of available data, use provenance to prefer authoritative sources over crawled data, and consider augmentation such as back-translation labeled as synthetic rather than diluting the human pool with noisy crawled pairs. Be honest that low-resource engines will be weaker and set deployment expectations accordingly; do not compensate for data scarcity by lowering quality bars silently.

### Set And Document Acceptance Thresholds

Quality filtering requires thresholds, and thresholds require justification. Set acceptance thresholds based on measured filter performance and the cost of errors in the target domain: a medical or legal engine needs stricter thresholds than a gisting engine. Document the threshold, the measured precision and recall at that threshold, the resulting corpus size and composition, and the rationale. When the threshold excludes a meaningful fraction of data, run a controlled training experiment to confirm the excluded data was hurting rather than helping. Thresholds pulled from intuition produce either over-cautious corpora that are too small or permissive corpora that are too noisy; thresholds tuned against measurement produce corpora that can be defended.

### Preserve A Defensible Provenance And Quality Record

Every pair that enters training should carry its source, its quality score, and the filter decisions applied to it, so the corpus can be audited and reproduced. When questions arise later, why does the engine mistranslate this construction, does the corpus contain unauthorized data, can we rebuild this training set, the provenance and quality record is the only way to answer. Store the filtering pipeline as versioned code and record the thresholds and model versions used. A parallel dataset without a quality and provenance record is an opaque artifact whose effects cannot be diagnosed.

## Common Traps

### Judging Quality By Corpus Statistics Alone

Aggregate stats hide pair-level defects. Inspect and score individual pairs; the engine learns from pairs, not from averages.

### Keeping Co-Occurring Non-Translation Pairs

Pairs that are not real translations teach arbitrary correspondences. Filter with cross-lingual similarity and calibrate the threshold.

### Ignoring Translationese

Training on calqued target text produces an engine that generates stilted, non-native output. Detect, weight, and supplement with natural text.

### Mixing Synthetic Or MT-Contaminated Data As Human

Unlabeled synthetic or competitor-MT data propagates errors and misrepresents the corpus. Label synthetic data and exclude contamination.

### Untuned Quality Filters

Filters with intuition-based thresholds either over-shrink or under-clean the corpus. Tune against human-labeled data and report precision and recall.

### Keeping Marginal Data In Low-Resource Pairs

Scarce data tempts permissive thresholds, but marginal data hurts more when there is little of it. Inspect carefully and label augmentation honestly.

### Thresholds Without Justification Or Experiment

Unjustified thresholds cannot be defended. Document the rationale and confirm excluded data was hurting via controlled experiment.

### No Provenance Or Quality Record

Without per-pair provenance and quality scores, the corpus cannot be audited or reproduced. Record source, score, and filter decisions.

## Self-Check

- Is quality assessed at the pair level through stratified sampling, with the fraction of misaligned, incomplete, distorted, or unidiomatic pairs quantified?
- Are non-translation co-occurring pairs detected with cross-lingual similarity and removed, with the threshold calibrated against inspection and the false-positive rate documented?
- Is translationese detected and its prevalence known, with weighting or supplementation by natural target text where it dominates?
- Is synthetic data labeled as such and MT-contaminated data detected and excluded from the human-quality pool?
- Are quality estimation, cross-lingual similarity, and fluency scores combined into a filter whose precision and recall are measured on a human-labeled set?
- Are low-resource pairs handled with extra human inspection, preference for authoritative sources, and honest expectations about engine strength?
- Are acceptance thresholds set from measured filter performance and domain error cost, documented with rationale, and validated by controlled training experiment?
- Does every pair carry source, quality score, and filter decisions, with the pipeline stored as versioned code and thresholds and model versions recorded?
- No competitor MT output is trained on as human translation, and no marginal data is kept silently to pad a low-resource corpus.
- The cleaned corpus and its quality record can be reproduced and audited.
