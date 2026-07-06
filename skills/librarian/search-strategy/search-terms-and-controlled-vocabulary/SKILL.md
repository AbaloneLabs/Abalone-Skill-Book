---
name: search_terms_and_controlled_vocabulary.md
description: Use when the agent is building search terms for a catalog or database, combining concepts, finding synonyms and broader or narrower terms, using subject headings and thesauri, applying Boolean operators, phrase searching, truncation, and filters, and iterating based on result quality.
---

# Search Terms And Controlled Vocabulary

A search strategy is not a single keyword dropped into a box. It is a structured combination of concepts, synonyms, controlled vocabulary, operators, and filters, adjusted iteratively as the results teach you what works. Patrons, and inexperienced librarians, tend to type one natural-language phrase, accept whatever comes back, and conclude the search is done. That approach misses the bulk of relevant material: items indexed under different terminology, under controlled subject headings, or in adjacent concepts. Building a deliberate search strategy is what turns a database from a slot machine into a research tool.

Use this skill whenever you are constructing a search in a catalog, database, or discovery layer. The goal is to prevent the agent from running a single unstructured query, from ignoring controlled vocabulary, and from accepting the first page of results as the answer.

## Core Rules

### Decompose The Topic Into Core Concepts

Most research topics combine two or more concepts. A search for "the effect of sleep on adolescent academic performance" contains at least three concepts: sleep, adolescents, and academic performance. Searching the whole phrase as one string usually fails because databases match exact words, not meaning.

Decompose the topic:

- identify each distinct concept;
- list each concept separately;
- plan to combine concepts with AND so results contain all of them;
- consider whether any concept is optional or a limit rather than a core concept.

Each concept becomes a building block you will expand with synonyms and controlled terms.

### Expand Each Concept With Synonyms, Broader, Narrower, And Related Terms

Databases index the same idea under many words. If you search only one term per concept, you miss everything indexed under the alternatives.

For each concept, brainstorm:

- synonyms (e.g., sleep / rest / slumber);
- variant spellings and transliterations (behavior / behaviour; Quran / Koran / Qur'an);
- broader terms (sleep / circadian rhythms);
- narrower terms (sleep / REM sleep / insomnia);
- related terms (sleep / fatigue / drowsiness);
- discipline-specific terminology (a medical database may use clinical terms a general one does not);
- historical terminology (terms change over time; older literature uses different vocabulary);
- regional or cultural terminology.

Combine the variants of a concept with OR so the search accepts any of them.

### Use Controlled Vocabulary And Thesauri When Available

Controlled vocabulary, subject headings, descriptors, or a thesaurus, is the database's official terminology for a concept. Searching with controlled terms retrieves items the database has tagged with that concept, regardless of which words the author used. This often produces more precise and complete results than keyword searching.

Use controlled vocabulary by:

- opening the database's thesaurus or subject heading list;
- searching your concept to find the preferred term;
- noting broader, narrower, and related terms the thesaurus offers;
- combining controlled terms with keyword terms to catch both indexed and full-text matches;
- exploding a heading to include its narrower terms when breadth is wanted.

Different systems use different vocabularies: Library of Congress Subject Headings in catalogs, MeSH in PubMed, ERIC descriptors in education databases. Learn the relevant one.

### Combine Concepts With Boolean Operators Deliberately

Boolean operators control how concepts combine. Used well, they sharpen a search; used carelessly, they break it.

Operators:

- AND narrows: results must contain all concepts (sleep AND adolescents AND performance);
- OR broadens: results may contain any variant of a concept (sleep OR rest OR insomnia);
- NOT excludes: removes a concept, but use cautiously because it can drop relevant items;
- nesting with parentheses groups OR sets before combining with AND: (sleep OR rest) AND (adolescent OR teen).

Many discovery layers and web tools default to an implicit AND between words. Do not assume; check how the specific tool treats multiple terms.

### Use Phrase Searching, Proximity, Truncation, And Wildcards

These features increase precision or recall depending on the tool.

- phrase searching with quotes locks words together: "academic performance" finds the phrase, not the words scattered;
- proximity operators find terms within n words of each other: sleep w/3 performance;
- truncation with a symbol expands a root: educat* finds education, educational, educator;
- wildcards replace a character: wom?n finds woman and women.

Syntax varies by platform. Check the help documentation before assuming a symbol works the way you expect.

### Apply Filters To Narrow Without Over-Narrowing

Filters (sometimes called facets or limits) narrow results by date, format, language, age group, methodology, and more. They are powerful and easy to overuse.

Apply filters when:

- the result set is too large to review;
- the patron needs a specific format, date range, or population;
- a methodology or evidence level is required.

Avoid over-filtering when:

- the topic is narrow and results are already few;
- the filter would exclude relevant items indexed differently;
- you have not yet seen what unfiltered results look like.

Review the effect of each filter. If results drop to zero, remove filters one at a time to find which one was too restrictive.

### Treat Results As Feedback And Iterate

Search results are evidence about the quality of your strategy. Read them before accepting or abandoning the search.

Diagnose from results:

- too many results: add a concept, apply a filter, use phrase searching, or use more specific terms;
- too few results: drop a concept, add synonyms, use broader terms, remove filters, try controlled vocabulary;
- irrelevant results: the terms may be ambiguous; add a distinguishing concept or use controlled vocabulary;
- missing expected sources: check terminology, coverage, and whether the database indexes that publication type;
- repeated useful terms in good results: harvest them as additional search terms;
- subject headings on relevant results: click through to find more items with the same heading.

Iteration is the norm, not a sign of failure. A search rarely succeeds on the first attempt.

### Adapt Strategy To The Specific Tool

Every tool interprets searches differently. A strategy that works in PubMed may fail in a catalog or a discovery layer. Before searching a new tool, learn its syntax, default operators, available filters, and controlled vocabulary. Transfer the concepts, not the exact query string.

### Record Effective Searches For Reproducibility

For research that matters, record the final effective search: database, terms, operators, filters, and date range. This supports reproducibility, lets you or a colleague update the search later, and provides evidence for a systematic review or a literature update.

## Common Traps

### Typing One Natural-Language Phrase

Natural-language queries match exact words and miss meaning. Decompose into concepts and combine deliberately.

### Ignoring Controlled Vocabulary

Keyword-only searches miss items indexed under official subject headings. Use the thesaurus or subject headings.

### Using Only One Term Per Concept

Synonyms and variant spellings matter. Expand each concept with OR.

### Overusing NOT

NOT can silently drop relevant items. Prefer positive combinations or filters.

### Assuming Boolean Syntax Is Universal

Operators, nesting, truncation, and wildcards differ by platform. Check the help.

### Stacking Filters Until Nothing Remains

Each filter narrows. Remove filters one at a time when results vanish.

### Accepting The First Page Of Results

Results diagnose the strategy. Read them and iterate.

### Concluding Nothing Exists After One Failed Query

A failed query usually means the strategy needs revision, not that the literature is absent. Try different terms, controlled vocabulary, broader scope, or a different database.

## Self-Check

- Did you decompose the topic into distinct core concepts to combine with AND?
- Did you expand each concept with synonyms, variant spellings, broader, narrower, related, and historical terms combined with OR?
- Did you consult and use the database's controlled vocabulary or thesaurus where available?
- Did you combine concepts with deliberate Boolean operators and correct nesting?
- Did you use phrase searching, proximity, truncation, and wildcards where the tool supports them?
- Did you apply filters to narrow without excluding relevant items, removing them when results vanish?
- Did you read the results as feedback and iterate the strategy rather than accepting the first page?
- Did you adapt the strategy to the specific tool's syntax and vocabulary rather than copying a query?
- Did you record the effective search for reproducibility where the work matters?
