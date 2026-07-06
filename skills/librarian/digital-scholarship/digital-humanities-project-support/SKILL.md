---
name: digital_humanities_project_support.md
description: Use when the agent is supporting digital humanities projects, consulting on DH project design and scope, selecting tools and platforms for text analysis or digital editions, advising on data modeling for cultural heritage collections, planning digital humanities project sustainability, or helping researchers build DH projects with library collections, archival materials, or computational methods.
---

# Digital Humanities Project Support

Digital humanities (DH) work sits at the intersection of scholarship, technology, and cultural heritage collections, and supporting it is one of the most rewarding and most easily mishandled things a library can do. The danger is that DH support collapses into either pure technology provision, building whatever the researcher asks for, or pure collection provision, handing over materials with no engagement in the scholarly method. Both fail. Pure technology provision produces fragile projects that die when the graduate student leaves and that encode no scholarly argument. Pure collection provision misses the chance to shape how collections are transformed into evidence. Good DH support is a collaboration where the library contributes expertise in data modeling, metadata, preservation, sustainability, and project management, while respecting that the scholarly question and interpretation belong to the researcher. Agents tend to underestimate how much of DH project success is determined by decisions made at scoping, modeling, and sustainability stages rather than at the build stage.

Use this skill when consulting on DH project design, selecting DH tools and platforms, advising on data modeling for cultural heritage, or planning project sustainability. The goal is to prevent the agent from building technically impressive but unscholarly or unsustainable projects, ignoring the gap between research questions and tool capabilities, or treating DH support as a service transaction rather than a collaboration.

## Core Rules

### Start From The Scholarly Question, Not The Tool

The most common DH failure is tool-first design: a researcher arrives excited about a tool, network analysis, topic modeling, a mapping platform, a digital edition framework, and the project is built around what the tool can do rather than what the scholar wants to argue. The result is a project that demonstrates technical capability without advancing a scholarly claim. DH support must begin by clarifying the research question and the argument the project is meant to make, then evaluate whether and which tools serve that argument.

Clarify at the outset:

- the scholarly question or argument the project advances;
- the evidence the argument requires and whether the available collections can provide it;
- the contribution to the field, what would be different in the discipline if this project succeeds;
- the intended audience and how they will encounter and assess the argument;
- the relationship between the computational method and traditional humanistic interpretation.

A project with a clear scholarly question can survive a tool change; a project built around a tool dies when the tool is abandoned.

### Model Data And Metadata As Scholarship, Not As Bookkeeping

In DH, the data model and metadata are part of the argument, not a clerical layer. How a collection is encoded, what is treated as an entity, what relationships are modeled, and what is left out all encode interpretive decisions that shape what the project can claim. Treating data modeling as a technical detail to be rushed produces projects where the data cannot support the analysis or where the encoding silently biases the results.

Engage data modeling as scholarship:

- discuss what entities and relationships matter to the research question, and what modeling framework, TEI, CIDOC CRM, a custom ontology, best fits;
- surface the interpretive decisions in the encoding, such as how names, dates, places, and ambiguity are handled;
- document the modeling choices and their rationale, because future scholars will need to assess the evidence;
- connect the data model to interoperable standards so the project's data can be reused and compared;
- recognize that metadata labor is skilled labor, often performed by students or staff whose contribution must be credited.

The data model is where the library's expertise most directly shapes the scholarly quality of the project.

### Scope Projects To A Realistic Sustainability Horizon

DH projects have a notorious sustainability problem: they are funded and built, then orphaned when funding ends, the PI moves, or the graduate student graduates. A project that cannot be sustained is not a finished scholarly contribution; it is a liability. Sustainability must be planned at scoping, not discovered at handoff, and it must be honest about what the institution can and cannot maintain.

Plan sustainability by:

- distinguishing the project's components by maintenance need: the data, which may be preservable; the interface, which requires ongoing maintenance; and the software, which may need migration;
- defining a minimal sustainable state, such as a preserved dataset with documentation, even if the interactive interface cannot be maintained;
- identifying the long-term home for the data and documentation, such as the institutional repository, early in the project;
- estimating the ongoing cost of interface and software maintenance and confirming who bears it;
- building handoff and documentation into the project timeline, not as a final afterthought;
- using sustainable, open technologies where possible and avoiding bespoke dependencies that no one will maintain.

The sustainability conversation should happen before the build, because it often changes the scope.

### Credit Labor And Clarify Authorship

DH projects involve distributed labor: faculty PIs, graduate and undergraduate researchers, library staff, developers, and designers, and the contribution of each is often invisible in the final product. Unclear or unfair credit creates conflict, exploits student and staff labor, and undermines the collaborative ethos DH claims. Credit and authorship should be discussed explicitly and early.

Address credit by:

- discussing authorship and contributor roles at project inception, using frameworks like CRediT or NISO contributor roles;
- crediting metadata, encoding, and development labor by name, not as generic staff work;
- documenting contributions in a way that survives staff and student turnover;
- recognizing the different forms of credit appropriate to different roles, from co-authorship to acknowledgment to portfolio credit;
- being explicit about how student labor is credited, since students depend on credit for career advancement.

Labor credit is not a courtesy; it is a scholarly integrity issue in collaborative work.

### Match Tools To Questions And To Capacity

The DH tool landscape is large and seductive, from Omeka and Scalar for exhibits, to TEI-based edition platforms, to Gephi and Palladio for network analysis, to Voyant and MALLET for text analysis, to GIS platforms for spatial work. Selecting tools requires matching them to the research question, the data, the team's capacity, and the sustainability horizon. A tool that requires a developer the team does not have, or that produces a dependency the institution cannot maintain, is the wrong tool regardless of its capability.

Evaluate tools against:

- alignment with the research question and the type of evidence the data provides;
- the team's technical capacity to install, configure, and maintain the tool;
- the data the tool requires and whether the project's data fits its expectations;
- the output format and whether results are portable or locked into the tool;
- the tool's own sustainability, community, and licensing;
- accessibility of the tool's output for the intended audience, including readers with disabilities.

Prefer tools that produce portable, standards-based output over tools that create captive projects.

### Build Accessibility And Audience Into The Design

DH projects are public scholarship, and their publicness carries accessibility obligations that are often treated as optional. A digital edition, exhibit, or analysis tool that cannot be used by readers with disabilities, or that assumes a specific device or bandwidth, excludes part of its audience and may violate accessibility obligations. Accessibility must be designed in, not retrofitted.

Design for accessibility by:

- following WCAG guidelines for the project's web presence from the start;
- ensuring text-based content is screen-reader compatible, including any encoded text and transcripts;
- providing alternatives for visual and interactive elements, including data visualizations and maps;
- considering bandwidth and device diversity, especially for projects reaching global or underserved audiences;
- documenting accessibility decisions so they are maintained through updates.

Accessibility is both an ethical and often a legal obligation; treat it as a requirement, not a feature.

### Treat Documentation As A Project Deliverable

DH projects fail when their methods, data, and decisions are not documented. A digital edition whose encoding decisions are unexplained, a corpus whose selection criteria are unclear, or an analysis whose parameters are unrecorded cannot be assessed, replicated, or reused by other scholars. Documentation is a scholarly deliverable, not a clerical task.

Documentation should cover:

- the research question and method, written for both humanistic and technical readers;
- the data, its provenance, selection criteria, and known limitations;
- the encoding or modeling decisions and their rationale;
- the tools and parameters used in analysis, with versions;
- the contributor roles and credit;
- the sustainability and access conditions.

A project whose documentation allows another scholar to assess and potentially reuse its data has succeeded as scholarship.

## Common Traps

### Tool-First Project Design

Building a project around an exciting tool rather than a research question produces technically capable but scholarly empty work. This is a trap because the project demos well but advances no argument and is hard to defend in peer review.

### Ignoring Sustainability Until Handoff

Treating sustainability as a problem for the end of the project guarantees orphaned, decaying projects. This is a trap because the build is funded but the maintenance is not, and the work is lost within a few years.

### Treating Data Modeling As Clerical

Rushing the data model and metadata to get to the build produces projects whose evidence cannot support their claims. This is a trap because the interpretive decisions hidden in the encoding silently shape or bias the results.

### Invisible Labor And Uncredited Work

Failing to credit graduate students, library staff, and developers exploits collaborative labor and creates conflict. This is a trap because it undermines the collaborative ethos and harms contributors whose careers depend on credit.

### Bespoke Dependencies No One Will Maintain

Building on custom software or fragile dependencies creates projects that break when the builder leaves. This is a trap because the project's lifespan is tied to one person's presence.

### Accessibility As Afterthought

Adding accessibility at the end, or omitting it, excludes audiences and may breach obligations. This is a trap because retrofitting accessibility is far harder and more expensive than designing it in.

### Documentation Deferred

Leaving documentation to the end means it is rushed, incomplete, or never done. This is a trap because undocumented projects cannot be assessed, replicated, or sustained.

### Confusing Prototype With Product

Treating a proof-of-concept built for a specific analysis as a finished public product overstates its robustness. This is a trap because prototypes lack the reliability, accessibility, and documentation a public scholarly product requires.

## Self-Check

- Does the project begin from a clearly stated scholarly question and argument, with tools selected to serve it?
- Is the data model treated as interpretive scholarship, with encoding decisions documented and justified?
- Is there a realistic sustainability plan distinguishing data, interface, and software, with a minimal sustainable state defined?
- Is the long-term home for data and documentation identified before the build begins?
- Are contributor roles and credit discussed explicitly and documented, including student and staff labor?
- Were tools evaluated against the research question, team capacity, data fit, output portability, and the tool's own sustainability?
- Is accessibility designed in from the start, following WCAG, with alternatives for visual and interactive elements?
- Is documentation treated as a deliverable covering question, method, data provenance, encoding, tools, and credit?
- Is the project honestly scoped as prototype versus public product, with robustness matched to its intended audience?
