---
name: discovery_layer_and_catalog_interface.md
description: Use when the agent is configuring, evaluating, redesigning, or troubleshooting a library discovery layer or online catalog interface, choosing scopes and facets, tuning relevance ranking, designing record displays and availability indicators, integrating linking resolvers and database results, handling e-resource access, or improving the search experience for patrons using Summon, Primo, EBSCO Discovery Service, WorldCat Discovery, Aspen Discovery, or a traditional OPAC.
---

# Discovery Layer And Catalog Interface

The discovery layer is where the collection meets the patron. It is the single most consequential interface in the library because it determines whether millions of dollars of acquired material is findable. A well-configured discovery system connects a patron's messy, natural-language query to the right book, article, or database; a poorly configured one buries relevant items, returns overwhelming result lists, breaks links to full text, and pushes users to commercial search engines that do not serve their needs. Discovery is not a product you install and leave at default settings; it is a service that must be shaped, tuned, integrated, and continuously evaluated against how real patrons actually search.

The judgment problem is that discovery layers are complex systems with many interacting settings: search scopes, relevance tuning, facet configuration, record display, availability logic, e-resource linking, and database integration. Vendors ship defaults optimized for generic installations, not for any particular library's collection, community, or priorities. Librarians often lack time to evaluate the system from the patron's perspective, so the defaults persist even when they produce poor results. Worse, discovery systems sit at the intersection of multiple data sources, the catalog, institutional repositories, e-resource vendors, and linking resolvers, and the seams between them are where access breaks: a record shows as available but the link fails, an article appears but is embargoed, a holdings display is wrong. The agent's job is to treat discovery as an active, evidence-driven service, not a passive vendor product.

Use this skill when configuring or evaluating any discovery or catalog interface: setting scopes and facets, tuning relevance, designing record displays, integrating full-text linking, handling e-resource access problems, comparing discovery products, or improving the patron search experience. The goal is to prevent the agent from leaving vendor defaults unexamined, optimizing for expert search, hiding availability and access status, breaking the link between discovery and full text, or treating discovery as a one-time implementation rather than an ongoing service.

## Core Rules

### Understand The Discovery Model And Its Limits

A discovery layer is not a catalog and not a web search engine; it is a unified index that attempts to search across local holdings and centralized article metadata in one interface. Understanding what it does and does not cover is the foundation of every other decision.

Key distinctions:

- a traditional OPAC searches only local catalog records, usually MARC, with precise fielded matching;
- a discovery layer searches a broad unified index that blends local catalog records, institutional repository items, and a massive central index of article metadata licensed from publishers and aggregators;
- coverage of the central index is not complete; some publishers, some databases, and some full text are not indexed or not full-text searchable;
- discovery favors recall and serendipity over precision, which is the right tradeoff for most patrons but frustrates expert researchers.

Implication: never assume discovery searches everything. Document coverage gaps, maintain a curated database list for deep research, and help patrons move between discovery and specialized databases.

### Choose Search Scopes Deliberately

The default scope determines what patrons see first and shapes their entire experience. This decision is too important to leave at the vendor default.

Scope options and tradeoffs:

- a broad scope blending catalog, articles, and repositories maximizes discovery and matches how undergraduates search, but can overwhelm with article results that obscure local books;
- a catalog-only default serves known-item searches for physical items but hides the article literature entirely;
- separate tabs or scope selectors add clarity but add a step and many users never switch;
- a default that prioritizes local holdings, with articles available via a toggle, often serves community needs well.

Choose the default based on evidence: what do your patrons actually search for, and where do they succeed or fail? Revisit the decision as the collection and community change.

### Configure Facets To Match Patron Mental Models

Facets are how patrons narrow large result sets. They must reflect the categories patrons actually think in.

Effective facet configuration:

- offer facets for format, availability, date, subject, author, language, location, and collection;
- show result counts so patrons understand the effect of applying a facet;
- allow multiple selections within a facet and across facets;
- order facets by usefulness, not alphabetically;
- make applied facets visible and easy to remove;
- avoid facet overload; too many facets paralyze choice.

Facets that mirror internal data structures, such as a raw call number, are less useful than facets patrons understand, such as format or topic.

### Tune Relevance With Evidence, Not Intuition

Relevance ranking is the core of discovery quality. Vendor defaults are generic; tuning requires evidence about your collection and patrons.

Relevance tuning practices:

- start with vendor best-practice settings, then adjust based on local testing;
- build a set of representative test queries drawn from real search logs;
- evaluate whether known-relevant items surface near the top;
- check that local catalog records are not drowned by article metadata when appropriate;
- verify that exact title and author matches rank highly for known-item searches;
- watch for over-boosting that promotes popularity over relevance;
- retest after every configuration change.

Relevance tuning is iterative and ongoing. Set a recurring schedule to review search logs and adjust.

### Make Availability And Access Status Obvious

Patrons search to get something. The result list must clearly show whether an item is available, where, and in what format.

Availability display requirements:

- show clear status for physical items: available, checked out, on hold, on order, with location and call number;
- show online access clearly with a direct link to full text;
- distinguish peer-reviewed, open access, and subscription content;
- indicate when full text is embargoed or not available so patrons are not sent to dead ends;
- show holdings for journals and multi-volume works clearly;
- update status in near real time where possible.

A result that hides availability forces patrons to click through to find out, increasing frustration and abandonment.

### Integrate Linking Resolvers Carefully

The link between a discovery record and the actual full text is the most failure-prone part of discovery. The linking resolver must be configured and monitored.

Linking best practices:

- ensure the resolver is properly configured with your e-resource holdings and knowledge base;
- test links across major publishers and aggregators regularly;
- provide a clear path when full text is not available, such as interlibrary loan request links;
- handle embargoed and restricted content honestly;
- monitor broken link reports and fix knowledge base errors promptly;
- consider a "best available access" approach that offers multiple legitimate paths.

A discovery system that surfaces an article but cannot deliver the full text breaks the promise of discovery. Linking integrity is a core service metric.

### Design Record Displays For Quick Comprehension

The full record display must help patrons decide quickly whether the item is what they need and how to get it.

Record display elements:

- clear title, author, and publication information;
- format and availability prominently shown;
- abstract or summary where available;
- subject headings or tags for context and further exploration;
- direct links to full text or physical location;
- citation and export tools, including persistent links;
- virtual browse for physical items to support shelf browsing;
- clear labeling of peer-reviewed and open-access status.

Dense bibliographic dumps serve catalogers, not patrons. Display what patrons need to act.

### Support Both Known-Item And Exploratory Search

Patrons search in two modes, and discovery must serve both.

For known-item search:

- prioritize exact title, author, ISBN, ISSN, and DOI matching;
- offer a citation linker for partial citations;
- handle common variations and typos.

For exploratory search:

- support subject browsing and related-result suggestions;
- use relevance ranking that clusters by topic;
- provide facets and sorting for exploration;
- offer "more like this" suggestions.

Designing for only one mode fails the other. Most undergraduate searches are exploratory; most faculty searches are known-item.

### Provide Clear Escape Hatches To Specialized Resources

Discovery cannot serve every research need. Provide clear paths to specialized databases and tools.

Escape hatches:

- maintain a curated, well-organized database list with descriptions;
- offer research guides that direct patrons to the best resources by discipline;
- provide links to specialized search tools for things discovery handles poorly;
- offer chat, email, and appointment options for librarian help;
- include search tips and tutorials.

The discovery system is one tool among many. Help patrons find the right tool when discovery is not it.

### Treat Discovery As An Ongoing Service

Discovery is not a project with a launch date. It is a service that requires continuous attention.

Ongoing service practices:

- review search analytics regularly for zero-result queries and low click-through;
- monitor broken links and access failures;
- retest relevance after collection or vendor changes;
- gather patron feedback through surveys and help-desk data;
- stay current with vendor updates and new features;
- document configuration decisions and rationale;
- schedule regular usability testing.

The most common failure is set-and-forget, where a discovery system launched with effort slowly degrades as the collection, vendors, and patron needs change.

## Common Traps

### Leaving Vendor Defaults Unexamined

Vendor defaults are generic. They rarely match a specific library's collection, community, or priorities. Evaluate and adjust every default setting.

### Assuming Discovery Searches Everything

The central index has coverage gaps. Some publishers, databases, and full text are not included. Document gaps and maintain a curated database list for deep research.

### Optimizing For Expert Boolean Search

Most patrons search with natural language and messy phrases. Tuning for power users leaves the majority behind. The default must work for untrained search.

### Hiding Availability And Access Status

A result that does not show availability forces extra clicks and causes abandonment. Surface availability, format, and access clearly in the result list.

### Broken Or Misleading Full-Text Links

The link between discovery and full text is the most failure-prone seam. Configure the resolver carefully, test links regularly, and provide interlibrary loan fallbacks.

### Drowning Local Holdings In Article Metadata

A broad scope can bury relevant books under thousands of article results. Balance local and central index weighting based on patron needs.

### Treating Discovery As A One-Time Implementation

Discovery decays without ongoing attention. Relevance drifts, links break, and coverage changes. Treat discovery as a continuously maintained service.

### Ignoring Analytics And Patron Feedback

Search logs reveal where patrons fail. Ignoring this evidence means problems persist undetected. Review analytics and act on findings regularly.

## Self-Check

- Is the default search scope chosen deliberately based on patron behavior data rather than vendor default?
- Do facets reflect patron mental models, show counts, support multi-select, and order by usefulness?
- Has relevance been tuned using a set of real test queries, with known-relevant items verified to surface well?
- Does the result list clearly show availability, format, location, and online access status for each item?
- Is the linking resolver properly configured, tested across major publishers, and monitored for broken links?
- Are embargoed and unavailable items labeled honestly with clear fallback paths such as interlibrary loan?
- Does the record display surface the information patrons need to decide and act, not just full bibliographic detail?
- Are both known-item and exploratory search modes supported?
- Are escape hatches provided to specialized databases, research guides, and librarian help?
- Is discovery treated as an ongoing service with regular analytics review, link monitoring, relevance retesting, and usability testing?
