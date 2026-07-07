---
name: menu_and_information_architecture.md
description: Use when the agent is organizing product content into menus, labels, categories, and taxonomies, naming navigation entries, deciding menu structure and ordering, or structuring an information architecture so users can predict where things live and find them without trial and error.
---

# Menu And Information Architecture

Menu and information architecture work is deciding what the things in a product are called, how they group, and where they live. It looks like writing labels, but it is really the decision of whether a user can predict where a feature lives before they have found it. Agents tend to mirror the database or the org chart, to choose labels that are internally consistent but externally opaque, and to optimize the menu for completeness rather than for the user's mental model. The harm is users who open every section looking for one feature, who never find capabilities because the label does not match their vocabulary, and who conclude the product cannot do what it can.

Use this skill before structuring menus, naming navigation entries, grouping features into categories, or reorganizing where things live. The goal is to prevent the agent from building an information architecture that makes sense to the team but not to users, from using labels that require inside knowledge, or from burying high-value items under categories that no one would guess.

## Core Rules

### Structure The Architecture Around User Mental Models

The information architecture should reflect how users think about the product's capabilities, not how the team builds or stores them. When the menu mirrors internal structure (engineering teams, database tables, backend services), users must learn the architecture before they can navigate it. When it mirrors their own model, they find things by prediction.

Build the architecture from user research:

- learn the words users use for tasks and objects, not the internal names;
- map the groups users naturally form when sorting features;
- identify the few destinations that account for most visits and prioritize them;
- treat the team's structure as a hypothesis to test against user language.

A menu organized by backend module is invisible to users; a menu organized by user intent is self-explanatory.

### Choose Labels That Match User Vocabulary

Labels are the single most consequential IA decision. A label that matches the user's word lets them find the feature instantly; a label that does not makes the feature invisible even when it is right there. Labels should be the user's language, plain and unambiguous.

Write labels so that:

- they use the user's term, not internal jargon or brand names;
- they are specific enough to predict content but short enough to scan;
- they avoid synonyms that split related items across labels;
- they are mutually distinguishable so users do not stare at two near-identical entries.

"Settings" is vague but conventional; "Notification preferences" is clearer. "Entities" means nothing to users. Choose words the user would say aloud.

### Order By Priority And Frequency, Not By Importance To The Team

Menu order signals priority, and users assume the top entries matter most. Ordering by the team's sense of importance, or alphabetically, wastes that signal. Order by how often users actually visit each destination and by the task's weight.

Order intentionally:

- place the most-used destinations first, within the limits of scanability;
- group related destinations so users find neighbors they expect;
- avoid alphabetical order for primary menus, which scatters related items;
- reserve prominent positions for high-value, high-frequency items.

Alphabetical order feels objective but ignores relationship and frequency, forcing users to scan the whole list each time.

### Keep Categories Distinct And Mutually Exclusive

Categories fail when users cannot decide which one holds their target. If a feature could plausibly live in two categories, some users will look in the wrong one and conclude it does not exist. Categories should be distinct enough that a user's first guess is usually right.

Make categories work by:

- ensuring each has a clear, single organizing principle;
- testing where users would look for representative items;
- avoiding overlapping categories that split a user's guess;
- merging categories that are too thin to be worth a separate entry.

When "Account" and "Profile" and "Settings" all exist and overlap, users guess wrong half the time. Collapse or clearly differentiate.

### Limit Breadth And Depth To What Users Can Hold

Both too many top-level entries and too many nested levels harm findability. Breadth overwhelms with choice; depth buries with clicks. The architecture must balance the two against the user's memory and patience.

Balance breadth and depth:

- keep top-level entries to a scannable number (commonly three to seven);
- avoid more than two or three levels of nesting for primary paths;
- flatten by promoting frequent deep items and merging thin categories;
- use cross-links and search to reduce reliance on pure hierarchy.

A menu with twenty top-level entries is unscannable; one with five levels of nesting is unnavigable. Find the balance for the content.

### Make The Architecture Survive Growth

Information architecture must accommodate features that do not exist yet. An architecture that fits today's content perfectly often breaks when the product grows, forcing painful reorganizations that move things users had memorized. Design for plausible expansion.

Design for growth by:

- choosing categories general enough to absorb related future items;
- avoiding structures so tuned to current content that one new feature breaks them;
- planning where the next likely additions would live;
- preferring stable top-level structure with flexible sub-structure.

### Provide Search And Cross-Links As Escape Hatches

No hierarchy is perfect, and users will guess wrong. Search and cross-links let users recover when the menu fails them, reaching content without knowing its category. These are not optional extras in a product of any size.

Provide recovery paths:

- search that reaches deep content by user vocabulary;
- cross-links between related items so lateral movement is possible;
- "see also" or related-entry pointers at dead ends;
- recent and frequently visited shortcuts for repeat access.

### Validate The Architecture Before Locking It

Information architecture feels right in a spreadsheet and fails in user testing. Because IA is about prediction, it must be tested with real users doing real tasks, not reviewed internally. Internal review confirms team logic; user testing confirms user prediction.

Validate by:

- card sorting to learn how users group items;
- tree testing to learn if users can find tasks in the proposed structure;
- first-click testing to check that the first guess is right;
- reviewing labels with users who do not know the product.

## Common Traps

### Mirroring The Database Or Org Chart

Menus structured around internal modules or teams force users to learn the organization's structure before they can navigate.

### Labels In Internal Jargon

Labels using brand or engineering terms that users do not say aloud make features invisible even when present.

### Alphabetical Primary Menus

Alphabetical ordering ignores relationship and frequency, scattering related items and forcing full-list scanning.

### Overlapping Categories

Categories that are not mutually exclusive split the user's guess, so they look in the wrong place and conclude the feature is absent.

### Too Many Top-Level Entries

Broad menus with many entries overwhelm scanning and bury high-value items in a long list.

### Deep Nesting For Tidiness

Levels added for organizational neatness bury common tasks under clicks users should not have to make.

### Architecture Tuned Only To Current Content

Structures so perfectly fitted to today's features break with the next addition, forcing reorganizations that displace memorized locations.

### Skipping User Validation

Information architecture reviewed only internally confirms team logic but not user prediction, and fails exactly where users need it most.

## Self-Check

- [ ] The architecture is structured around user mental models and vocabulary, not internal database or org-chart structure.
- [ ] Labels use the user's terms, are specific but scannable, avoid jargon, and are mutually distinguishable.
- [ ] Menu ordering reflects user frequency and priority, not team importance or alphabetical convenience.
- [ ] Categories are distinct and mutually exclusive, validated so a user's first guess is usually right.
- [ ] Breadth and depth are balanced, with top-level entries scannable and primary paths no deeper than necessary.
- [ ] The architecture can absorb plausible future growth without forcing reorganization of memorized locations.
- [ ] Search, cross-links, related pointers, and recent-visited shortcuts exist as recovery paths when the menu fails.
- [ ] The architecture was validated with users through card sorting, tree testing, or first-click testing, not only internal review.
