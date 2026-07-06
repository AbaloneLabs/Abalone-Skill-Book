---
name: descriptive_metadata.md
description: Use when the agent is creating descriptive metadata for digital objects, choosing descriptive elements and schemas, writing titles and descriptions for digital items, mapping across metadata standards, or deciding description depth for digital collections.
---

# Descriptive Metadata

Descriptive metadata for digital objects carries the same burden as cataloging, finding, identifying, selecting, and obtaining, but in a less standardized and more fragmented environment. Unlike book cataloging, digital description often draws from multiple schemas, serves multiple platforms, and must work for objects that have no clean title page. A scanned photograph, an oral history recording, a dataset, and a born-digital document each present different description challenges, and the metadata must serve both human users and machine harvesting. Poor descriptive metadata hides digital objects from search, misrepresents their content, or fails to interoperate with aggregators. Good descriptive metadata is schema-aware, user-task anchored, consistent across a collection, and honest about what is and is not known.

Use this skill when creating or mapping descriptive metadata for digital objects, choosing or combining schemas, writing titles and descriptions, or establishing description practice for a digital collection. The goal is to prevent the agent from inventing metadata, under-describing complex objects, ignoring interoperability, or producing inconsistent metadata across a project.

## Core Rules

### Anchor Description To User Tasks And Use

Descriptive metadata exists so users can find, identify, select, and obtain digital objects. Every element should serve at least one task.

Apply the tasks:

- find, title, creator, subject, and date enable discovery;
- identify, title, type, format, and rights distinguish the object;
- select, description, subject, coverage, and language help users judge relevance;
- obtain, identifier, source, relation, and rights tell users how to access.

Before adding an element, ask which task it serves. This prevents both clutter and the gaps that hide objects from search.

### Choose The Schema By Object Type And Audience

Different schemas suit different object types and audiences. Choosing the wrong schema limits interoperability and forces awkward mapping.

Common schemas and their strengths:

- Dublin Core, broad interoperability, good for cross-domain aggregation;
- MODS, richer than Dublin Core, good for library and cultural heritage;
- VRA Core, for visual resources and images;
- Darwin Core, for biological occurrence and specimen data;
- DDI, for social science datasets;
- PREMIS, for preservation metadata, paired with descriptive;
- schema.org, for web discovery and search engine indexing.

Match the schema to the object type and to the platforms and aggregators the collection must reach. Using Dublin Core for everything loses richness; using a highly specialized schema loses interoperability.

### Write Titles That Describe, Not Just Label

The title is the most important descriptive element for discovery. A title like "Image 0042" or "Document 3" tells users nothing. A descriptive title enables search and selection.

Good titles:

- name the subject, event, person, or place depicted;
- include a date when known;
- distinguish the object from similar ones;
- avoid internal codes or filenames as the sole title;
- remain concise but informative.

For a photograph, "Women voting at the community hall, 1920" beats "Photo 17." For an oral history, "Interview with Maria Chen on garment worker organizing, 1985" beats "Recording 003."

### Use Controlled Vocabularies For Subjects And Names

Free-text subjects and names fragment discovery. Controlled vocabularies collocate objects across a collection and improve precision.

Use:

- Library of Congress Subject Headings or LC Name Authority File for broad interoperability;
- Getty AAT or TGN for art and architecture, places;
- MeSH for medical and health topics;
- local thesauri for specialized collections, with crosswalks to standard vocabularies;
- FAST as a simpler alternative to full LCSH.

Record the vocabulary source so the terms are interpretable. Avoid mixing controlled and free-text terms in the same element without distinction.

### Record What Is Known And Mark What Is Not

Digital objects often have gaps in provenance, date, creator, or location. Honesty about uncertainty prevents misleading users and future researchers.

Practice:

- record known information accurately;
- mark inferred or estimated values clearly, using conventions like [approximately] or [between X and Y];
- use a source note to explain the basis for inferred data;
- leave elements empty rather than guessing when nothing is known;
- distinguish transcribed information from cataloger-supplied description.

Fabricated metadata destroys trust. A clearly marked unknown is more useful than a confident guess.

### Provide Sufficient Description For Selection

Users often cannot open every object to judge relevance. The description element should give enough for selection without opening the file.

Include in description:

- what the object depicts, contains, or records;
- the context of creation if known;
- significant content not obvious from the title;
- physical or technical characteristics that affect use;
- relationships to other objects or a parent collection.

A one-sentence generic description fails the selection task. Write descriptions specific to the object.

### Capture Identifiers And Relationships

Digital objects exist in webs of relationships, to source items, to parent collections, to derived formats, and to other objects. Identifiers and relationship metadata keep these connections navigable.

Record:

- a persistent identifier for the digital object itself;
- the source item identifier if derived from a physical original;
- collection or series identifiers for context;
- relationships to other versions, translations, or parts;
- the source repository or holding institution.

Without identifiers and relationships, objects become orphans that users cannot trace or contextualize.

### Document Rights Clearly

Rights metadata determines whether users can view, download, reuse, or redistribute an object. Vague or missing rights suppress use and create legal risk.

Record:

- the copyright status if known;
- the rights holder or source of permission;
- a standardized rights statement, such as RightsStatements.org or Creative Commons;
- any access restrictions and their basis;
- reuse permissions and conditions.

Do not leave rights to assumption. A standardized statement protects the institution and enables confident reuse.

### Ensure Interoperability And Harvestability

Digital collections rarely live in isolation. Metadata must harvest cleanly into aggregators and map across schemas.

Practice:

- use standard element names and structures;
- provide required elements for harvesting protocols like OAI-PMH;
- include persistent identifiers and stable links;
- avoid local-only fields that do not map;
- validate metadata against the schema before publication;
- test harvest into target aggregators.

Metadata that works only in the local system limits the collection's reach.

### Maintain Consistency Across A Collection

A digital collection is only as discoverable as its most inconsistent record. Mixed practices in titles, subjects, dates, and rights fragment discovery.

Enforce consistency through:

- a project metadata guide or application profile;
- templates and controlled input forms;
- regular review and cleanup passes;
- training for all metadata creators;
- authority control for names and subjects.

Consistency is a collective property. One carelessly described object can break browsing and faceting.

## Common Traps

### Generic Or Code-Only Titles

"Image 0042" hides the object from search. Write descriptive titles that name the content.

### Free-Text Subjects And Names

Uncontrolled terms fragment discovery across the collection. Use controlled vocabularies and record their source.

### Inventing Metadata To Fill Fields

Guessing dates, creators, or locations misleads users. Mark uncertainty and leave fields empty when unknown.

### One-Sentence Generic Descriptions

A description that could apply to any object fails the selection task. Write object-specific descriptions.

### Missing Or Vague Rights

Unclear rights suppress reuse and create legal risk. Use standardized rights statements.

### Schema Mismatch To Object Type

Using the wrong schema loses richness or interoperability. Match schema to object and audience.

### Local-Only Fields That Do Not Harvest

Fields that map to nothing limit aggregation. Use standard elements or document crosswalks.

### Inconsistent Practice Across Records

Mixed titles, subjects, and dates break faceting and browsing. Enforce a project profile and review.

## Self-Check

- Does each metadata element serve at least one of the find, identify, select, obtain tasks?
- Is the schema chosen to match the object type and the required platforms and aggregators?
- Are titles descriptive and specific, naming the content rather than using codes or filenames?
- Are subjects and names drawn from controlled vocabularies with the source recorded?
- Is uncertainty clearly marked, with unknowns left empty rather than guessed?
- Do descriptions provide enough detail for users to select without opening the file?
- Are persistent identifiers and relationships to source items, collections, and related objects recorded?
- Are rights captured with standardized statements covering status, holder, and reuse conditions?
- Does the metadata validate against its schema and harvest cleanly into target aggregators?
- Is practice consistent across the collection, enforced by a project profile and review?
