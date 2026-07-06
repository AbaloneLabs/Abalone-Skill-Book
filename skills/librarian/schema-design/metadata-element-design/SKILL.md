---
name: metadata_element_design.md
description: Use when the agent is designing metadata elements and properties, defining element semantics and granularity, deciding whether to split or combine elements, establishing data types and cardinality, or creating local extensions to a standard schema.
---

# Metadata Element Design

Designing a metadata element is a decision with lasting consequences. Once an element exists, creators populate it, systems index and facet on it, users search and filter by it, and downstream consumers depend on its meaning. Changing an element after data has accumulated is costly and lossy, so the design must be right at the start. Element design is not naming fields; it is defining semantics, granularity, obligation, data type, controlled vocabulary, and the relationships between elements. The central tensions are between specificity and interoperability, between richness and simplicity, and between what creators can reliably populate and what users need to find. An element defined too broadly captures inconsistent data; one defined too narrowly fragments related information. An element with no controlled vocabulary becomes free-text noise; one with an overly rigid vocabulary rejects legitimate values. Good element design resolves these tensions deliberately, documents the semantics so every creator and consumer shares one understanding, and tests the design against real objects before adoption.

Use this skill when designing metadata elements or properties, defining element semantics and granularity, setting data types and cardinality, or creating local schema extensions. The goal is to prevent the agent from defining elements by name alone, splitting or combining information without analysis, omitting controlled vocabularies, or adopting elements without testing against real data.

## Core Rules

### Define Element Semantics Precisely

An element's name is not its definition. Two creators reading "coverage" may populate it with dates, places, or temporal ranges, producing unusable inconsistency. The semantics must be defined precisely enough that any two creators apply the element the same way.

Define for each element:

- what real-world information it captures;
- what it explicitly does not capture, to prevent scope creep;
- the unit or granularity of the value;
- how it relates to other elements, is it distinct, overlapping, or a refinement;
- examples that disambiguate edge cases.

Write the definition as if the reader has never seen the schema. A precise definition is the foundation of consistent population.

### Decide Granularity Deliberately

Granularity is how finely an element splits information. "Date" as one element is coarse; "date created," "date issued," "date copyrighted" are fine. The right granularity depends on how users search and how creators can reliably distinguish.

Granularity decisions:

- split when users need to filter or facet on the distinction;
- split when creators can reliably populate the distinct elements;
- combine when the distinction is rarely meaningful or rarely known;
- avoid splitting so finely that creators guess or leave fields empty;
- document the basis for each split or combination.

Over-splitting produces empty and guessed fields; under-splitting loses useful facets. Test granularity against real objects and realistic search behavior.

### Choose Data Types And Formats That Enforce Consistency

The data type and format of an element determine whether it can be validated, sorted, filtered, and faceted. Free-text where structured data is needed breaks all of these.

Type and format choices:

- use controlled vocabularies or authority files for names, subjects, and genres;
- use standardized date formats, ISO 8601, for temporal data;
- use structured identifier schemes, URIs, for identifiers;
- use enumerated lists for finite value sets, rights statements, object types;
- use numeric types for measurable quantities;
- constrain free-text to elements where variability is inherent, like descriptions.

The data type should make incorrect values detectable. A date field that accepts any string cannot be sorted or filtered reliably.

### Set Cardinality Based On Realistic Data

Cardinality, whether an element holds one or many values, affects how data is entered, stored, and faceted. Wrong cardinality forces awkward workarounds or loses information.

Cardinality rules:

- allow repetition where an object genuinely has multiple values, multiple subjects, multiple creators;
- constrain to single value where repetition would be meaningless or confusing, like a primary identifier;
- define how multiple values are ordered or prioritized;
- define a maximum or expected typical count to guide creators;
- ensure the system can store, display, and facet repeated values correctly.

An element that should be repeatable but is constrained to one value loses information; one that is repeatable but always single-valued complicates display and faceting.

### Design Elements To Serve Specific User Tasks

Every element should serve at least one user task, find, identify, select, or obtain. Elements that serve no task clutter the schema and burden creators without benefit.

Task mapping:

- find, title, creator, subject, date enable discovery;
- identify, type, format, identifier distinguish the object;
- select, description, subject, coverage, language help users judge relevance;
- obtain, identifier, source, relation, rights tell users how to access.

Before adding an element, name the task it serves. If none, reconsider whether it belongs. This prevents schema bloat and focuses effort on useful description.

### Avoid Overlapping And Redundant Elements

Elements that capture overlapping information force creators to choose inconsistently and confuse users about which to trust. Design elements to be distinct.

Overlap prevention:

- define clear boundaries between similar elements;
- merge elements that always hold the same information;
- use refinements or qualifiers rather than separate elements for closely related data;
- document which element to use when content could fit more than one;
- review the schema for redundancy during design.

"Description" and "Abstract" and "Summary" as three separate elements invite inconsistency. Define distinct roles or merge them.

### Create Local Extensions Carefully

When a standard schema lacks a needed element, a local extension may be necessary. Extensions must be designed to interoperate, not to replace standard elements.

Extension practice:

- exhaust standard elements and refinements before creating a local one;
- design the local element to crosswalk to standard elements where possible;
- namespace and document the extension clearly;
- record the extension in the application profile with full semantics;
- plan for how the extension migrates if the schema changes.

Local elements that ignore interoperability isolate the collection. Design them to map outward even if they are richer than the standard.

### Test Element Design Against Real Objects

Element design validated only in theory fails on real data. Testing against representative objects catches granularity, vocabulary, and cardinality problems before adoption.

Testing:

- select objects spanning the collection's variety;
- attempt to populate each element for each object;
- note where elements are ambiguous, empty, or forced;
- check whether the populated data supports intended search and faceting;
- revise the design based on findings before full adoption.

An element untested against real objects will be populated inconsistently from the start. Testing is part of design, not an optional review.

## Common Traps

### Defining Elements By Name Alone

Names mislead. Define precise semantics so creators share one understanding.

### Over-Splitting Into Empty Fields

Too-fine granularity produces guessed or empty fields. Split only where distinctions are meaningful and reliably knowable.

### Free-Text Where Structured Data Is Needed

Unstructured values cannot be sorted, filtered, or faceted. Use controlled vocabularies and standard formats.

### Wrong Cardinality

Constrained repetition loses information; unnecessary repetition complicates display. Set cardinality based on realistic data.

### Elements That Serve No User Task

Taskless elements clutter the schema and burden creators. Map every element to a task.

### Overlapping Elements

Similar elements force inconsistent choices. Define distinct boundaries or merge.

### Local Extensions That Ignore Interoperability

Isolated local elements fragment the collection. Design extensions to crosswalk outward.

### Adopting Elements Without Testing

Untested designs fail on real data. Test against representative objects before adoption.

## Self-Check

- Does each element have a precise semantic definition beyond its name, including what it does and does not capture?
- Is granularity chosen deliberately, splitting only where distinctions are meaningful and reliably knowable?
- Do data types and formats enforce consistency through controlled vocabularies, standard dates, and structured identifiers?
- Is cardinality set based on realistic data, with ordering and typical counts defined?
- Does each element serve at least one user task, find, identify, select, or obtain?
- Are overlapping and redundant elements eliminated through clear boundaries or merging?
- Are local extensions designed to interoperate and crosswalk to standard elements?
- Has the element design been tested against representative real objects before adoption?
