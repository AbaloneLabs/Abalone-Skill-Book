---
name: bibliometric_mapping_and_network_analysis.md
description: Use when the agent is mapping a research field with citation or co-citation networks, interpreting bibliometric indicators, choosing between citation databases, running co-authorship or keyword co-occurrence analyses, or using network metrics to describe the structure of a literature.
---

# Bibliometric Mapping And Network Analysis

Bibliometrics turns citations into maps of a field, but those maps are only as honest as the data and choices behind them. Citation counts, co-citation clusters, and centrality metrics are shaped by which database was queried, how the field boundary was drawn, how documents were deduplicated, and which algorithm produced the clusters. Agents often generate attractive network visualizations and read meaning into them without questioning whether the underlying data represents the field or an artifact of coverage. A cluster in a map is not a research community until someone shows that it corresponds to one.

Use this skill when mapping a field with citation or co-citation networks, interpreting bibliometric indicators, choosing data sources, or drawing conclusions from network structure. The goal is to prevent the agent from presenting a visualization as insight while ignoring the coverage limits and parameter choices that produced it.

## Core Rules

### Choose The Data Source With Coverage Limits In Mind

Different citation databases cover different fields, regions, languages, and document types. The chosen source silently defines what the field looks like.

Evaluate sources by:

- disciplinary coverage (one database may dominate a field, another may miss it);
- coverage of books, conference proceedings, and non-English venues;
- citation indexing completeness and timeliness;
- author and affiliation disambiguation quality;
- access and licensing constraints.

State which source was used and what it is known to miss. A map built from one database is a map of that database's coverage, not necessarily of the field.

### Define The Field Boundary Explicitly

The set of documents analyzed determines everything downstream. A loose boundary floods the map with tangential work; a tight one erases relevant streams.

Define the boundary by:

- the query and filters that produced the corpus;
- the time window and its justification;
- document type and language restrictions;
- how the boundary was validated (expert check, seed papers, recall test);
- the resulting corpus size and known omissions.

Document the query so the corpus is reproducible. A field map whose corpus cannot be regenerated is an opinion.

### Deduplicate And Disambiguate Before Counting

Citation data is noisy: duplicate records, variant author names, merged or split institutions, and retracted or corrected papers all distort counts and networks.

Clean by:

- deduplicating records that represent the same work;
- disambiguating authors and affiliations where possible;
- handling errata, corrections, and retractions consistently;
- noting where disambiguation was infeasible and the likely impact.

Author name ambiguity in particular can inflate or merge distinct researchers, corrupting productivity and collaboration metrics.

### Match The Network Type To The Question

Different networks answer different questions, and using the wrong one produces a misleading map.

Choose by question:

- direct citation: traces intellectual lineage and foundational works;
- co-citation: groups works cited together, revealing shared intellectual bases;
- bibliographic coupling: groups works sharing references, revealing current frontiers;
- co-authorship: reveals collaboration structure and communities;
- keyword or term co-occurrence: reveals conceptual themes.

Reading a co-citation cluster as a "current research frontier" misstates what co-citation shows, which is a shared cited base.

### Treat Clusters As Hypotheses, Not Facts

Clustering algorithms will produce clusters from any input, including noise. A cluster is a candidate grouping that must be interpreted and validated.

Validate clusters by:

- examining the top-cited or central documents in each;
- checking whether the cluster corresponds to a recognizable theme or community;
- testing stability across algorithms and parameters;
- labeling clusters from content, not from the algorithm's output alone.

Do not name a cluster from a single outlier keyword or an artifact of the resolution parameter.

### Interpret Centrality And Impact Metrics Cautiously

Metrics like citation count, h-index, betweenness, and PageRank encode specific assumptions and are sensitive to field, age, and database norms.

Interpret by:

- normalizing for field, year, and document type where possible;
- recognizing that raw citation counts favor older and review papers;
- noting that centrality reflects the analyzed corpus, not global importance;
- avoiding league tables that rank on a single unnormalized metric.

A highly central node in a narrowly drawn corpus may be peripheral in the broader field.

### Acknowledge What Bibliometrics Cannot Measure

Citation-based methods measure attention and linkage, not quality, correctness, or influence on practice. Conflating these is a common overreach.

State limits:

- citations can be critical, not endorsing;
- highly cited work may be flawed, contested, or retracted;
- influential work in practice or policy may be poorly cited academically;
- negative citations and self-citations inflate counts.

Do not equate citation prominence with validity or importance.

### Make The Analysis Reproducible

Bibliometric results depend on query date, database version, and software parameters. Without these, the map cannot be regenerated.

Record:

- database, access date, and version;
- the exact query and filters;
- deduplication and cleaning steps;
- clustering algorithm, parameters, and software version;
- the full corpus or a stable identifier for it.

A map that cannot be reproduced is a snapshot of an undocumented process.

## Common Traps

### Treating One Database As The Field

A single source's coverage biases the corpus, especially against books, conferences, and non-English work.

### Undocumented Field Boundaries

A corpus assembled by ad hoc searching cannot be regenerated or audited, making the map unverifiable.

### Reading Clusters As Established Communities

Algorithms always cluster; the question is whether the clusters mean anything, which requires interpretation.

### Equating Citations With Quality Or Endorsement

A citation can be a critique; prominence is not validity.

### Unnormalized Metric League Tables

Ranking authors or journals on raw citation counts favors old, review-heavy, or large-field work.

### Ignoring Author And Affiliation Ambiguity

Merged or split identities corrupt collaboration and productivity analyses.

### Over-Reading A Visualization

An attractive network plot is not an argument; the data and parameters behind it carry the meaning.

## Self-Check

- Is the citation database named, with its known coverage gaps disclosed?
- Is the field boundary defined by a reproducible query, filters, and time window?
- Were records deduplicated and authors and affiliations disambiguated where possible?
- Does the network type (direct citation, co-citation, coupling, co-authorship) match the question?
- Are clusters interpreted and labeled from content and validated for stability, not accepted as given?
- Are centrality and impact metrics normalized for field, year, and document type where possible?
- Are the limits of citation-based measurement (attention, not quality) acknowledged?
- Are database version, query, cleaning, algorithm, and parameters recorded for reproducibility?
- Is the visualization supported by documented data rather than presented as self-evident insight?
