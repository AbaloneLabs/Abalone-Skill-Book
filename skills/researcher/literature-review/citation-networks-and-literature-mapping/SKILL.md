---
name: citation_networks_and_literature_mapping.md
description: Use when the agent is tracing citations forward and backward, building a literature map, using bibliometric or network analysis, identifying foundational and influential works, verifying citation accuracy, or judging whether a literature search has converged on the relevant body of work.
---

# Citation Networks And Literature Mapping

A literature is not a list of papers; it is a network of influence, disagreement, and lineage. Researchers who read forward from one seed paper, or who trust a search engine's ranking, often miss foundational works, parallel literatures, and the disputes that define a field. Citation network methods exist to make that structure visible, but they are often misused to produce impressive-looking maps that hide shallow search, citation errors, and uncritical reliance on counts.

Use this skill when mapping a literature, when tracing the intellectual lineage of a concept, when using bibliometric or network tools, when identifying which works matter, and when verifying that a search has actually converged. The goal is to keep the agent from mistaking a citation count for quality, from importing citation errors, and from presenting a network diagram as a substitute for reading. The agent has latitude in tools and visualization, but must ground claims in verified links and interpreted structure.

## Core Rules

### Treat Citation Networks As Maps, Not Verdicts

A citation network shows structure: which works cite which, which clusters form, which nodes are central. It does not show quality, correctness, or importance on its own.

Use the map to:

- locate foundational and high-connectivity works;
- find parallel literatures disconnected from the seed;
- identify clusters that signal subfields or schools;
- trace the spread of a concept or method;
- spot bridges between otherwise separate communities;
- find works cited only critically, which may be influential through disagreement.

Then read the works the map highlights. A map without reading is a decoration.

### Search Bidirectionally From Seed Papers

A common failure is to read only forward in time or only the references of one paper. Both directions matter, and so do the lateral moves across clusters.

Use:

- backward citation tracing, reading the references a key paper builds on;
- forward citation tracing, finding newer works that cite the key paper;
- lateral tracing, examining works that cite the same set of sources;
- author tracing, following the most prolific or central authors;
- journal and venue tracing, mapping where the field publishes;
- cluster tracing, exploring distinct subnetworks.

A literature that converges from multiple starting points is more trustworthy than one built from a single seed.

### Verify Citations Before Trusting Them

Citation databases contain errors: misattributed works, duplicated records, variant name spellings, and citations that misrepresent the cited work. A network built on unverified links inherits these errors.

Check:

- that the cited work actually says what the citing paper claims;
- that author and year disambiguation is correct;
- that duplicated or merged records are reconciled;
- that self-citations are flagged where they distort centrality;
- that retracted or corrected works are treated appropriately;
- that the database coverage matches the field's publishing norms.

Citation misrepresentation is common. A few minutes verifying key links prevents propagating it.

### Choose Centrality And Clustering Measures Deliberately

Different network measures answer different questions, and the choice changes what the map appears to show.

Match measure to question:

- in-degree and citation counts for raw influence, with all their flaws;
- betweenness for works that bridge clusters;
- eigenvector or PageRank-style measures for influence weighted by the importance of citers;
- co-citation and bibliographic coupling for similarity among works;
- community detection for identifying subfields or schools;
- burst detection for works or terms rising suddenly in attention.

Report which measure was used and why. A different measure can produce a different "most important" work.

### Do Not Equate Citation Count With Quality

Citation counts are inflated by field size, publication age, controversy, self-citation, and gaming. A highly cited work may be influential because it is wrong, contested, or convenient.

Interpret counts alongside:

- the proportion of citations that are critical or negative;
- whether citations are concentrated in a single community;
- whether counts are driven by a method or dataset rather than the claim;
- the field's baseline citation rate;
- the age and trajectory of attention;
- presence of retractions, corrections, or replications.

A central node in a network is a hypothesis about importance, not a proof of correctness.

### Map For Gaps And Silences, Not Only For Density

The most useful parts of a literature map are often the empty regions: works that should be connected but are not, clusters that ignore each other, and populations or methods absent from the network.

Look for:

- disconnected clusters that address the same phenomenon under different names;
- foundational works no longer cited, indicating a forgotten lineage;
- sharp drops in citation between subfields, suggesting a schism;
- absence of replication or critique, suggesting an unquestioned claim;
- over-reliance on a single method or dataset;
- geographic or linguistic concentration that limits generalization.

Silence in a network is data, but it requires interpretation, not just visualization.

### Document The Search And Convergence

A literature map is only as trustworthy as the search that produced it. Document the process so the map can be reproduced and updated.

Record:

- databases and platforms used;
- search strings and filters;
- seed papers and selection rationale;
- time window and its justification;
- inclusion rules for nodes;
- deduplication and disambiguation steps;
- tools and parameters for clustering and centrality;
- date of the search, since networks shift over time.

A map drawn from an undocumented search cannot be checked, updated, or trusted.

### Use Mapping To Plan Reading, Not Replace It

The temptation with network tools is to substitute diagrams for engagement. A map that is not followed by careful reading of the works it highlights produces shallow synthesis.

Use mapping to:

- prioritize which works to read deeply;
- identify the small set that anchors a subfield;
- choose which parallel literatures to engage;
- decide when a search has converged;
- structure a review's organization around real clusters.

Then read, summarize, and critique. The map points; the reading justifies.

## Common Traps

### Reading Only Forward Or Only Backward

Single-direction tracing produces a lineage that misses parallel and converging literatures. Both directions and lateral moves are needed.

### Trusting Citation Counts As Quality

High counts can reflect controversy, convenience, or gaming as easily as merit. Counts are a starting hypothesis, not a verdict.

### Building Maps On Unverified Citations

Databases carry errors, and citing papers often misrepresent cited works. Unverified links propagate the errors into the map.

### Substituting Diagrams For Reading

A beautiful network diagram is worthless if the works it highlights have not been read and critiqued.

### Ignoring Silences And Disconnected Clusters

The empty regions of a map often reveal the most important gaps and schisms. Density alone is not insight.

### Over-Reading A Single Centrality Measure

Different measures highlight different works. Reporting one without justification can manufacture a false "most important" node.

### Forgetting That Networks Shift Over Time

A snapshot map becomes stale. Fields move, and a map without a date cannot be trusted or updated.

## Self-Check

- [ ] Is the citation network treated as a map to guide reading, not as a verdict on quality?
- [ ] Does the search trace backward, forward, laterally, and across clusters from multiple seeds?
- [ ] Are key citations verified for accuracy, disambiguation, and representation of the cited work?
- [ ] Are centrality and clustering measures chosen deliberately and reported with rationale?
- [ ] Are citation counts interpreted alongside critical citations, field baselines, and trajectory?
- [ ] Does the mapping identify gaps, silences, disconnected clusters, and forgotten lineages?
- [ ] Is the full search process documented, including databases, strings, seeds, time window, and tools?
- [ ] Are the works the map highlights actually read, summarized, and critiqued?
- [ ] Are retracted, corrected, or contested works treated appropriately in the network?
- [ ] Is the map dated and reproducible so it can be checked and updated?
