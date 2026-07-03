---
name: taxonomy_and_labeling_decisions.md
description: Use when the agent is defining taxonomies, choosing labels and terminology, naming product elements, creating categorization schemes, or deciding how to name and classify content and features so users understand them without explanation.
---

# Taxonomy And Labeling Decisions

The words a product uses are its interface. Labels, category names, menu items, button text, and feature names determine whether users understand what they can do, where things belong, and what will happen when they act. Despite this, labeling is consistently treated as a minor decision, filled in at the end by whoever is writing the copy, based on what sounds reasonable to an insider. The result is products full of terms that make sense to the team and confuse the users, categories that overlap or leave gaps, and names that mislead about function.

Taxonomy and labeling are not copywriting details; they are structural decisions that shape how users comprehend the product. A good label makes a feature self-explanatory; a bad one generates support tickets, mis-clicks, and abandoned tasks. A good taxonomy lets users classify and find content intuitively; a bad one makes every search and browse a gamble. The product manager who treats naming and classification as serious decisions builds products users understand; one who treats them as finishing touches builds products users stumble through.

Use this skill before naming features or categories, before defining a classification scheme, before choosing the labels that appear in navigation and interface, or when users misunderstand what things mean. Ask: do these labels use the users' language or the team's? Can a user predict what a category contains from its name? Do the categories overlap confusingly or leave obvious gaps? Have the labels been tested with people who did not build the product?

## Core Rules

### Use The Users' Language, Not The Team's

The most common labeling failure is using internal terminology that users do not share. Teams develop shorthand for concepts, drawn from engineering, from the domain, from internal product names, or from competitive positioning, and this shorthand becomes invisible to the people who use it. Users encounter these labels cold, without the context that made them sensible to the team, and they guess at meaning, often wrongly.

Ground labels in the language users actually use, discovered through research, support transcripts, search queries, and interviews. If users call a feature "statements" and the team calls it "transactional reports", the user-facing label should move toward "statements". This is not about dumbing down; it is about matching the mental model and vocabulary of the audience. Internal precision can be preserved in documentation and code; the interface should speak the user's language. When a precise internal term must be used, define it for the user rather than assuming comprehension.

### Make Labels Predictive Of Content And Action

A label should let users predict what they will find or what will happen. "Account settings" predicts a collection of configuration options. "Delete" predicts removal. When labels are vague, clever, or metaphorical, users cannot predict, and they either avoid the option, missing value, or trigger it accidentally, causing harm. Predictability is the core requirement of a label.

Test each label by asking what a user would expect to find or happen. Does the expectation match reality? If a label called "Resources" could contain documentation, templates, training, or third-party tools, it is not predictive, and users will not know which. Prefer specific labels over clever or generic ones. "Download invoice" is predictive; "Get document" is not. Resist the temptation to be catchy at the expense of clarity, especially for consequential actions where a wrong prediction causes damage.

### Design Taxonomies For Mutual Exclusivity And Coverage

A taxonomy, a system of categories, works when items fit clearly into one category and when the categories together cover the full range of content. Two failures undermine taxonomies: overlap, where an item could plausibly go in multiple categories, leaving users unsure where to look; and gaps, where an item fits no category, exposing the scheme's incompleteness. Both erode trust in the classification and make finding harder.

Design categories to be mutually exclusive where possible, with clear definitions of what each contains and what it excludes. Where overlap is unavoidable, because some items genuinely span categories, decide on a primary assignment rule and apply it consistently, or allow multiple assignment with clear indication. Test coverage by attempting to classify a representative sample of real content; items that fit nowhere reveal gaps that need a new category or a redefinition of existing ones. A taxonomy that cannot classify the actual content is not yet finished.

### Choose The Right Granularity

Categories can be too broad, lumping dissimilar items together so users cannot find specifics, or too narrow, fragmenting related items so users must check many places. The right granularity depends on the content volume and how users search. With few items, broad categories suffice and are easier to navigate. With many items, finer categories help users narrow, but only if the distinctions are meaningful to them.

Calibrate granularity to the user's need, not to the team's sense of completeness. A taxonomy that distinguishes categories the team cares about but users do not adds navigation cost without findability benefit. Conversely, a taxonomy too coarse for a large content set forces users to scan within oversized categories. Test by having users find specific items and observing whether the category structure helps them narrow efficiently or forces them to scan too much.

### Avoid Jargon, Acronyms, And Ambiguous Terms

Jargon, acronyms, and ambiguous terms exclude users who lack the context to decode them. An acronym that is obvious to the team may be meaningless or misleading to users. A term with a specialized meaning may be interpreted in its common sense, leading users astray. Ambiguous terms, those with multiple meanings, force users to guess which sense applies.

Prefer plain, unambiguous language. Spell out acronyms or replace them with descriptive terms. Where a specialized term is necessary because no plain alternative exists, introduce it with a brief explanation the first time the user encounters it. Audit labels for ambiguity by considering alternative interpretations a user might bring. "Submit" is ambiguous about what is being submitted and to whom; "Send application to review" is clearer. The cost of a longer, clearer label is almost always less than the cost of user confusion.

### Keep Labels Consistent Across The Product

Consistency in labeling reduces cognitive load and builds a vocabulary users can rely on. If the same action is called "Save" in one place and "Update" in another, users wonder whether the actions differ. If the same concept is labeled differently across sections, users must relearn terms and may miss that they are looking at the same thing. Inconsistent labeling fragments understanding.

Establish a labeling lexicon for the product, defining the canonical terms for key concepts and actions, and apply it consistently. When the same function must be described differently for context, make the relationship clear. Review new features against the lexicon to prevent drift. Consistency is unglamorous, but it is what allows users to build a stable mental model of the product's vocabulary, and that model is what lets them navigate confidently.

### Validate Labels And Taxonomies With Users

Labels and taxonomies that feel obvious to the team routinely confuse users, because the team has context users lack. The only reliable way to know whether labels work is to test them with people who did not build the product. Methods include first-click testing, where users click where they would go to accomplish a task based on labels alone; card sorting, for taxonomy validation; and comprehension testing, where users explain what they expect a label or category to mean.

Validate before committing labels and categories to the interface, because changing them after launch is costly and disorienting for existing users. Look for labels users misinterpret, categories they cannot distinguish, and terms they do not recognize. Iterate until the structure and labels are comprehensible to representative users without explanation. A label the team must explain is a label that needs work.

## Common Traps

### Insider Language

Using terminology the team understands and users do not. The trap is labels that feel precise to insiders and are opaque to the audience.

### Non-Predictive Labels

Labels too vague or clever to let users predict content or action. The trap is users who avoid or mis-trigger options because they cannot anticipate the result.

### Overlapping Or Gappy Taxonomy

Categories that overlap or fail to cover real content. The trap is a classification users cannot trust to organize or retrieve.

### Wrong Granularity

Categories too broad to narrow or too fine to navigate. The trap is a taxonomy calibrated to the team's distinctions rather than the user's needs.

### Jargon And Ambiguity

Acronyms and ambiguous terms that exclude or mislead. The trap is labels users interpret in the wrong sense.

### Inconsistent Vocabulary

The same concept labeled differently across the product. The trap is a fragmented mental model that prevents confident navigation.

### Unvalidated Labels

Assuming labels are clear because the team understands them. The trap is shipping labels that confuse the users who encounter them cold.

## Self-Check

- [ ] Labels use the users' language, discovered through research, not the team's internal terminology.
- [ ] Each label lets users predict what they will find or what will happen, avoiding vague or clever wording.
- [ ] Taxonomy categories are mutually exclusive where possible, with clear definitions, and together cover the real content.
- [ ] Granularity matches user needs and content volume, not the team's sense of completeness.
- [ ] Jargon, acronyms, and ambiguous terms have been replaced or explained for users.
- [ ] A labeling lexicon exists and is applied consistently, so the same concept uses the same term throughout.
- [ ] Labels and taxonomy were validated with users who did not build the product, through first-click, card sort, or comprehension testing.
- [ ] Labels users misinterpreted, categories they could not distinguish, and terms they did not recognize were identified and revised.
- [ ] Consequential action labels are unambiguous about what will happen, to prevent accidental harm.
- [ ] No label requires team explanation to be understood by a representative user.
