---
name: text_and_data_mining_support.md
description: Use when the agent is supporting text and data mining (TDM) research, advising on corpus building and licensing for TDM, negotiating researcher access to licensed databases for mining, selecting or configuring TDM tools and workflows, handling copyright and contractual constraints on mining, or helping researchers mine library-licensed or open corpora, cultural heritage collections, and large-scale text collections.
---

# Text And Data Mining Support

Text and data mining (TDM) is a research method that extracts patterns, entities, topics, and relationships from large corpora, and library support for it is one of the highest-leverage and most constraint-laden services the library offers. The tension at the heart of TDM support is that researchers want to mine everything, while the content they want is governed by a thicket of copyright, license terms, technical access controls, and contractual prohibitions that vary by vendor and jurisdiction. Agents tend to treat TDM support as a technical service, finding tools and building corpora, while underestimating the rights and access constraints that determine whether a mining project is even lawful, let alone feasible. A TDM project built on a corpus the researcher had no right to mine, or mined in violation of a license, taints the resulting research and exposes the institution. Good TDM support integrates technical, legal, and collections expertise and treats the rights analysis as a precondition, not an obstacle.

Use this skill when supporting TDM research, building corpora, negotiating database access for mining, selecting TDM tools, or addressing copyright and licensing constraints. The goal is to prevent the agent from enabling mining that violates license or copyright, building corpora without provenance and rights documentation, or treating TDM as pure computation divorced from the legal and collections context.

## Core Rules

### Resolve Rights And Access Before Building The Corpus

The single most important step in TDM support is determining whether the researcher has the right to mine the desired corpus and the technical means to access it. This must happen before any corpus building or tool selection, because a corpus built without rights is unusable and potentially infringing. The rights landscape is complex: copyright limits reproduction, database licenses often prohibit or restrict mining, technical protection measures may block access, and jurisdictional exceptions such as the EU DSM Directive text and data mining provisions or US fair use vary in scope.

Conduct a rights and access analysis that addresses:

- the source of each corpus component: open access, library-licensed, purchased, or scraped;
- the copyright status of the underlying works and any reproduction the mining requires;
- the license terms for licensed databases, many of which explicitly prohibit or condition TDM, discoverable in the license or via the institution's electronic resources librarian;
- whether the license offers a TDM option, such as a dedicated API, a corpus download, or a requirement to notify the vendor;
- applicable statutory exceptions for TDM and their conditions, including any non-commercial or research purpose limitations;
- technical access controls, including DRM or rate limits, that must not be circumvented.

When licensed content cannot be mined under its terms, the answer is negotiation or an alternative corpus, not silent non-compliance. Document the rights basis for the corpus so the research is defensible.

### Build Corpora With Provenance And Documentation

A TDM corpus is a research artifact whose construction is part of the method, and it must be documented as such. A corpus assembled without provenance, selection criteria, or known limitations produces results that cannot be assessed or replicated. The library's expertise in corpus documentation is one of its most valuable contributions to TDM projects.

Document every corpus with:

- the source of each document and the access method used;
- the selection criteria, what was included and excluded and why;
- the volume, date range, languages, and known biases of the corpus;
- any cleaning, normalisation, or deduplication applied, with the tools and parameters;
- the rights basis for retention and any conditions on sharing the corpus;
- known limitations, such as OCR quality, coverage gaps, or representativeness.

A corpus whose construction is transparent is a scholarly contribution; one whose construction is opaque undermines every result derived from it.

### Match Tools And Methods To The Question And The Data

TDM encompasses a wide range of methods, from simple frequency and concordance analysis, to named entity recognition and topic modeling, to word embeddings, classification, and large language model-based analysis. Selecting a method requires matching it to the research question and to what the data can actually support. Topic modeling a corpus too small or too heterogeneous to produce stable topics, or applying a method whose assumptions the data violates, generates confident-looking nonsense.

Match method to question and data by:

- clarifying what the researcher wants to discover and which methods can reveal it;
- assessing whether the corpus is large enough, clean enough, and consistent enough for the intended method;
- recognizing the assumptions of each method, such as the bag-of-words assumption in topic modeling or the need for labeled data in supervised classification;
- choosing tools appropriate to the team's technical capacity, from GUI tools like Voyant to programmatic workflows in Python or R;
- planning for validation, because TDM results require interpretation and checking against the source texts.

Methodological fit is a scholarly judgment, and the library should support it, not just supply tools.

### Negotiate Licensed Database Access Proactively

Many high-value TDM corpora live in licensed databases whose default terms prohibit mining. Rather than treating this as a dead end, the library can negotiate access, and increasingly must, because researchers expect it. Negotiation is most effective when it is proactive, informed, and coordinated with the institution's licensing function.

Approach database access negotiation by:

- reviewing current licenses for TDM clauses before researchers need access, not after;
- identifying vendors that offer TDM APIs, corpus downloads, or dedicated mining provisions;
- coordinating with the electronic resources librarian or consortial licensing body to add TDM-friendly terms at renewal;
- documenting the access method agreed with each vendor so researchers know the legitimate path;
- tracking which databases are mineable under what conditions, as a service to researchers.

Proactive TDM licensing turns a recurring obstacle into a routine service.

### Address Computational Infrastructure Realistically

TDM at scale requires computational infrastructure: storage for large corpora, compute for processing, and often GPU access for machine learning methods. Researchers frequently underestimate this, and the library's TDM support should connect them to realistic infrastructure rather than letting them discover the gap mid-project. This may involve institutional high-performance computing, cloud platforms, or shared research computing services.

Assess infrastructure needs by:

- estimating corpus size, processing requirements, and storage over the project life;
- identifying available institutional or cloud computing resources and their access conditions;
- considering data residency and security, especially for licensed or sensitive corpora;
- planning for reproducibility, including environment and dependency documentation.

Infrastructure is not glamorous, but its absence halts TDM projects.

### Treat OCR And Data Quality As First-Class Concerns

Much historical and cultural heritage TDM relies on OCR or transcribed text whose quality varies enormously, and the quality directly affects the validity of results. Treating OCR text as if it were clean ground truth produces unreliable findings. The library should help researchers assess and where possible improve data quality and to report it as a limitation.

Address data quality by:

- assessing OCR or transcription quality for historical corpora before analysis;
- using quality-aware methods where possible, such as confidence-weighted or corrected text;
- documenting known quality issues as corpus limitations;
- considering correction workflows for high-value subsets;
- reporting data quality in the methods so readers can assess the results.

Acknowledged data quality is honest scholarship; ignored data quality is a hidden source of error.

### Plan For Reproducibility And Method Transparency

TDM research is computational, and computational research must be reproducible to be credible. A TDM study whose corpus, code, parameters, and environment cannot be reconstructed cannot be verified or built upon. The library's data management expertise directly supports reproducibility.

Support reproducibility by:

- preserving the corpus or a documented path to reconstruct it, respecting rights;
- preserving the code, with versioning and dependencies, in a repository such as GitHub or Zenodo;
- documenting parameters, random seeds, and tool versions;
- linking the publication to the data and code via persistent identifiers;
- applying good research data management practice throughout.

Reproducibility is not an add-on; it is the credibility condition for computational scholarship.

## Common Traps

### Mining Licensed Content In Violation Of Terms

Proceeding to mine a licensed database without checking or honoring TDM clauses taints the research and risks institutional access. This is a trap because the technical capability to mine exists while the right to do so does not.

### Building Corpora Without Provenance

Assembling a corpus without documenting sources, selection, and cleaning makes results unassessable. This is a trap because the corpus construction is invisible and so are its biases.

### Applying Methods The Data Cannot Support

Running topic modeling or machine learning on data too small, dirty, or mismatched produces confident but invalid results. This is a trap because the output looks rigorous while violating the method's assumptions.

### Treating OCR Text As Ground Truth

Analyzing historical OCR as if it were clean text hides substantial error in the results. This is a trap because the error is distributed and invisible, undermining downstream findings.

### Ignoring Infrastructure Until The Project Stalls

Discovering compute or storage limits mid-project halts progress and forces emergency workarounds. This is a trap because the gap is foreseeable and the recovery is disruptive.

### Deferring Reproducibility To Publication

Leaving corpus, code, and parameter documentation until the end means it is incomplete or absent. This is a trap because unreproducible computational research cannot be verified.

### Circumventing Technical Access Controls

Bypassing DRM or rate limits to obtain content, even for research, can violate anti-circumvention law regardless of copyright exceptions. This is a trap because the legal risk persists even when the underlying use might be permitted.

## Self-Check

- Did you resolve the copyright, license, and statutory exception basis for every corpus component before building it?
- For licensed databases, did you check the TDM clauses and pursue legitimate access via API, corpus download, or license negotiation?
- Is the corpus documented with source, selection criteria, volume, biases, cleaning, rights basis, and known limitations?
- Is the chosen TDM method matched to the research question and to the corpus's size, cleanliness, and assumptions?
- Have you assessed and documented OCR or data quality, and reported it as a limitation where relevant?
- Is computational infrastructure, storage and compute, scoped and available before processing begins?
- Is the project planned for reproducibility, with corpus, code, parameters, and environment preserved and linked via persistent identifiers?
- Did you avoid circumventing technical protection measures, pursuing legitimate access paths instead?
- Is the rights analysis documented so the research and the institution can defend the mining if challenged?
