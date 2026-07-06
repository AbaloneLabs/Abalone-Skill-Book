---
name: metadata_quality_assessment.md
description: Use when the agent is assessing the quality of a metadata collection, defining quality criteria and metrics, conducting metadata audits or reviews, measuring completeness, consistency, and accuracy, or establishing baseline quality for a digital collection or catalog.
---

# Metadata Quality Assessment

Metadata quality is not a single property but a set of measurable characteristics: completeness, accuracy, consistency, conformance, uniqueness, and richness. A collection can be complete but inconsistent, accurate but sparse, or rich but non-conforming. Assessing quality means defining which characteristics matter for the collection's purpose, measuring them against real data, and producing a baseline that reveals where to invest improvement effort. Without assessment, quality problems stay invisible until users fail to find things, and improvement efforts are guesswork. With assessment, an institution can target the specific defects that most harm discovery, track whether interventions work, and make a defensible case for quality investment. The work is partly technical, running validation and analysis, but mostly judgmental: deciding what quality means for this collection, what tradeoffs to accept, and what level of defect is tolerable given the collection's use and the institution's capacity.

Use this skill when assessing metadata quality, defining quality criteria, conducting audits, measuring quality metrics, or establishing baselines for improvement. The goal is to prevent the agent from treating quality as a vague aspiration, measuring the wrong things, accepting metrics without context, or assessing once and never re-measuring.

## Core Rules

### Define Quality Criteria For The Collection's Purpose

Quality is not absolute. What matters depends on the collection's users, systems, and goals. Define quality criteria before measuring.

Criteria to define:

- completeness, are required and important elements populated;
- accuracy, do the values correctly describe the objects;
- consistency, are similar objects described similarly across the collection;
- conformance, does the metadata follow the schema, profile, and vocabularies;
- uniqueness, are identifiers and access points non-duplicated;
- richness, is description deep enough for the collection's use.

Weight the criteria by the collection's purpose. A discovery-first collection prioritizes completeness and consistency; a research collection prioritizes accuracy and richness. Document the weighting so assessment targets what matters.

### Measure Completeness As A First Indicator

Completeness is the easiest quality dimension to measure and often the most impactful, because empty fields cannot support discovery. Measure it systematically.

Completeness measurement:

- calculate population rates for each element, especially mandatory and recommended ones;
- identify elements with systematically low population;
- distinguish truly empty from default or placeholder values;
- segment completeness by collection, format, or creator to find problem areas;
- track completeness over time to detect drift.

Low completeness in a key element, like title, creator, or subject, directly harms discovery. Completeness measurement often reveals where metadata creation practice is breaking down.

### Assess Consistency Across The Collection

Consistency is whether similar objects are described similarly. Inconsistency breaks faceting, browsing, and collocation, and it is harder to detect than incompleteness.

Consistency checks:

- are controlled vocabularies used consistently, or mixed with free-text;
- are names and subjects drawn from the same authority sources;
- are dates in consistent formats;
- are similar object types described at similar depth;
- are conventions, capitalization, abbreviations, applied uniformly.

Use automated analysis to find variant forms, mixed vocabularies, and format inconsistencies. Segment by creator or batch to locate the source of inconsistency.

### Verify Accuracy Against Source Objects

Accuracy is whether the metadata correctly describes the objects. It cannot be fully automated; it requires comparison to the objects or authoritative sources.

Accuracy assessment:

- sample records and compare metadata to the actual objects;
- check transcribed elements against the source of information;
- verify dates, names, and subjects against authoritative references;
- identify systematic errors, wrong dates, misassigned subjects, conflated names;
- record error types and rates by element and collection.

Accuracy assessment is labor-intensive, so use sampling targeted at high-use or high-value collections. Document the sample size and confidence.

### Check Conformance To Schema And Profile

Conformance is whether the metadata follows the structural rules of the schema and application profile. Non-conforming data fails validation and may break systems.

Conformance checks:

- validate all records against the schema and profile rules;
- check mandatory elements are populated;
- check controlled vocabularies are used where required;
- check data types, formats, and cardinality;
- check value lists and enumerated fields for valid values.

Automated validation catches most conformance errors. Configure validation rules to match the profile and report errors by type and collection.

### Detect Duplicates And Identifier Problems

Duplicate records and non-unique identifiers fragment discovery and confuse users. Detect them as part of quality assessment.

Duplicate detection:

- find records describing the same object, by identifier, title, or content similarity;
- check for non-unique or reused identifiers;
- identify split entities that should be collocated;
- identify merged entities that should be separated.

Duplicates are common in collections built from multiple sources or migrations. Document the deduplication needed and its priority.

### Segment Quality Findings To Target Improvement

Aggregate quality scores hide where the problems are. Segment findings by collection, format, creator, batch, or time period to target improvement.

Segmentation:

- identify collections or batches with the lowest quality;
- identify creators or workflows producing inconsistent metadata;
- identify formats or object types with specific problems;
- prioritize improvement by impact on discovery and feasibility.

A collection-wide completeness score of 80 percent may hide one batch at 40 percent. Segmentation turns a number into an actionable diagnosis.

### Establish A Baseline And Re-Measure Over Time

Quality assessment is most valuable as a recurring measurement. A baseline shows where you started; re-measurement shows whether improvement works.

Baseline and tracking:

- record baseline metrics for each quality dimension;
- set targets for improvement;
- re-measure on a schedule after interventions;
- track whether targeted improvements raise the specific metrics;
- report trends to sustain investment in quality.

One-time assessment is a snapshot. Recurring assessment is quality management.

## Common Traps

### Treating Quality As A Vague Aspiration

Quality must be defined and measured to be improved. Define criteria before assessing.

### Measuring Completeness Only

Completeness is easy but insufficient. Assess consistency, accuracy, conformance, and uniqueness too.

### Accepting Metrics Without Context

An 80 percent score hides problem segments. Segment findings to target improvement.

### Skipping Accuracy Assessment

Accuracy needs object comparison and cannot be automated away. Sample and verify.

### Ignoring Duplicates And Identifier Problems

Duplicates fragment discovery. Detect and prioritize deduplication.

### One-Time Assessment Without Re-Measurement

Snapshots cannot show improvement. Establish baselines and re-measure.

### Aggregate Scores That Hide Problems

Collection-wide averages mask low-quality segments. Segment to diagnose.

### No Documentation Of Assessment Method

Without documented method, results cannot be reproduced or compared. Document criteria, sampling, and tools.

## Self-Check

- Are quality criteria defined and weighted for the collection's purpose before measurement?
- Is completeness measured systematically, with population rates by element and segment?
- Is consistency assessed for vocabularies, names, dates, and description depth across the collection?
- Is accuracy verified against source objects and authoritative references through sampling?
- Does conformance validation check mandatory elements, vocabularies, data types, and cardinality?
- Are duplicates and identifier problems detected and prioritized for deduplication?
- Are quality findings segmented by collection, creator, batch, or format to target improvement?
- Is a baseline established and re-measurement scheduled to track improvement over time?
