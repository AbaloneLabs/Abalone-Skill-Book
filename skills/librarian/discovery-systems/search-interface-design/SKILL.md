---
name: search_interface_design.md
description: Use when the agent is designing or evaluating a library discovery search interface, choosing search box placement and behavior, deciding on default scopes and filters, handling zero-results states, or improving the usability of search for diverse patron populations.
---

# Search Interface Design

The search box is the front door of the library for most users. If patrons cannot find what they need through discovery, the collection, no matter how rich, is effectively invisible to them. Search interface design is therefore not a cosmetic concern; it is an access and equity concern. A well-designed search interface anticipates how users actually search, which is messier and less structured than librarians assume, surfaces the most relevant results, guides users when they fail, and works across devices and abilities. Poor design hides materials, frustrates users into giving up, and pushes them to commercial search engines that do not serve their scholarly or information needs.

Use this skill when designing or evaluating a discovery search interface, choosing search behavior and defaults, handling zero-results states, or improving search usability. The goal is to prevent the agent from designing for expert search behavior, hiding advanced features behind jargon, treating zero results as a dead end, or ignoring accessibility and mobile realities.

## Core Rules

### Design For How Users Actually Search

Users do not construct Boolean queries with field limits. They type natural phrases, paste citations, enter typos, and expect the system to understand intent. Design must meet this reality.

Implications:

- support natural language and phrase queries;
- handle common misspellings and typos through stemming or correction;
- interpret ambiguous queries generously;
- make advanced search available but not required;
- tolerate the messy reality of real queries.

Designing for the ideal expert search leaves most users behind. The default experience must work for the untrained user.

### Make The Search Box Prominent And Persistent

The search box should be the most obvious element of the interface, visible on every page without scrolling.

Placement principles:

- prominent on the homepage and persistent in navigation;
- clear placeholder text indicating scope;
- minimal required input, one box for the common case;
- obvious but unobtrusive scope selectors, all, articles, catalog;
- visible on mobile without zooming.

A search box buried below banners or behind menus adds friction and reduces use.

### Choose Default Scopes Deliberately

The default search scope determines what users see first. The choice shapes their experience and what they find.

Scope considerations:

- a broad scope, catalog plus articles plus repositories, maximizes discovery but may overwhelm;
- a catalog-only default serves known-item searches but hides articles;
- separate tabs or scopes for different content types add clarity but complexity;
- the default should match the most common user need, often a mix.

There is no universally correct default. Choose based on user behavior data and revisit it as needs change.

### Handle Zero-Results States As Guidance, Not Dead Ends

A zero-results page is a critical moment. The default message, no results found, abandons the user. Treat it as an opportunity to guide.

Good zero-results handling:

- suggest spelling corrections or alternative terms;
- offer to broaden the scope or remove filters;
- show related or partial matches;
- provide links to search help, databases, or a librarian;
- explain why results may be missing and what to try.

A helpful zero-results state turns failure into guidance. Many users give up at this screen; design it to recover them.

### Surface Relevance And Context In Results

Result lists must help users judge relevance quickly. Dense bibliographic dumps do not serve scanning.

Surface in results:

- clear titles and authors;
- availability status, available online, on shelf, checked out;
- format icons or labels;
- snippets or abstracts where available;
- sorting options that match user goals, relevance, date, author;
- facets for narrowing by format, date, subject, location.

Users scan results in seconds. Give them the information to decide quickly and accurately.

### Provide Faceted Navigation For Refinement

Facets let users narrow large result sets to what they need. They are essential for discovery usability.

Facet design:

- choose facets that match user mental models, format, date, subject, author, location;
- show counts so users understand the effect of applying a facet;
- allow multiple selections within and across facets;
- make applied facets visible and easy to remove;
- order facets by usefulness, not alphabetically.

Facets turn an overwhelming result set into a navigable space. See the faceted-navigation skill for depth.

### Support Known-Item And Exploratory Search

Users search in two modes: known-item, looking for a specific work, and exploratory, browsing a topic. The interface should support both.

For known-item:

- prioritize exact title and author matching;
- offer citation linker tools;
- handle ISBN, ISSN, and DOI lookups.

For exploratory:

- support subject browsing and related-result suggestions;
- offer relevance ranking that clusters by topic;
- provide facets and sorting for exploration.

Designing for only one mode fails the other. Accommodate both.

### Ensure Accessibility And Mobile Usability

Discovery must work for users with disabilities and on mobile devices. Accessibility is a legal and ethical obligation, not an enhancement.

Requirements:

- keyboard navigability for all functions;
- screen reader compatibility with proper ARIA roles;
- sufficient color contrast and readable typography;
- touch-friendly targets on mobile;
- responsive layout that adapts to screen size;
- no functionality dependent solely on mouse or color.

Test with assistive technology and real mobile devices. Compliance checklists are necessary but not sufficient; real testing reveals real barriers.

### Use Analytics To Understand And Improve

Search analytics reveal how users actually behave, what they search for, where they succeed and fail, and where the interface needs work.

Use analytics to:

- identify common zero-result queries;
- find queries with low click-through;
- understand which facets and sorts are used;
- measure session length and abandonment;
- compare behavior across user groups.

Analytics turn design from opinion into evidence. Review them regularly and test changes against them.

### Provide Escape Hatches And Human Help

No interface serves every need. Provide clear paths to alternatives and human assistance.

Escape hatches:

- links to specialized databases for advanced research;
- links to the catalog for precise known-item searches;
- contact options for a librarian via chat, email, or appointment;
- search tips and tutorials;
- feedback mechanisms.

The discovery system is one tool among many. Help users find the right tool when discovery is not it.

## Common Traps

### Designing For Expert Search Behavior

Boolean and fielded search serve power users. The default must work for untrained natural-language search.

### Burying The Search Box

A search box below the fold or in a menu adds friction. Make it prominent and persistent.

### Zero-Results As A Dead End

No results found abandons users. Use the screen to guide with corrections, broadening, and help.

### Dense Bibliographic Result Lists

Users scan in seconds. Surface availability, format, and relevance cues for quick judgment.

### Ignoring Mobile And Accessibility

Non-accessible or non-responsive design excludes users. Test with assistive tech and real devices.

### Opinion-Based Design Without Analytics

Designing without usage data is guessing. Use analytics to find what users actually do.

### No Escape Hatches

Discovery cannot serve every need. Provide links to databases, catalog, and librarian help.

### Overcomplicating The Interface

Too many options overwhelm. Keep the default simple and surface complexity progressively.

## Self-Check

- Does the default search experience work for natural-language, messy queries, not just expert Boolean?
- Is the search box prominent, persistent, and visible without scrolling on all devices?
- Is the default search scope chosen deliberately based on user needs and behavior data?
- Do zero-results states guide users with corrections, broadening, related matches, and help links?
- Do result lists surface availability, format, and relevance cues for quick scanning?
- Is faceted navigation provided with useful facets, counts, and multi-select?
- Are both known-item and exploratory search modes supported?
- Is the interface accessible to users with disabilities and usable on mobile devices?
- Are search analytics used to understand behavior and guide improvement?
- Are escape hatches provided to databases, catalog, and librarian help when discovery is insufficient?
