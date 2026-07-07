---
name: content_modeling.md
description: Use when the agent is defining a content model, structuring content types and fields, deciding relationships between content entities, designing a schema for headless CMS, planning reusable content components, or mapping how content is authored, stored, and assembled across channels.
---

# Content Modeling

A content model is the blueprint for how content is structured, related, and reused. It looks like a database schema, but it is really a set of decisions about what content exists as a distinct thing, what each thing contains, how things connect, and how the same content can serve multiple surfaces without being duplicated. Agents tend to treat content modeling as a technical afterthought, design fields around one page layout, or copy a structure that worked for a different product. The harm is invisible until content needs to change channel, scale, or ownership, at which point a weak model becomes an expensive restructuring.

Use this skill before finalizing content types, fields, relationships, or a CMS schema. The goal is to prevent the agent from building a model tied to one presentation, from fragmenting content that should be reusable, or from creating relationships that cannot be maintained as content grows.

## Core Rules

### Model Content As Things, Not As Pages

The foundational shift in content modeling is thinking in terms of content types (an article, a product, an event, a person) rather than pages. A page is a presentation of content; a content type is the structured thing itself. Modeling by page couples content to one layout and prevents reuse.

Build from content types:

- identify the distinct kinds of content the product needs, each with its own attributes and relationships;
- define each type by the fields that are intrinsic to what it is, not by what one page happens to display;
- allow the same content type to be rendered in multiple layouts and channels from one source;
- separate content from presentation so a redesign does not require re-authoring.

If a "homepage" is modeled as a content type, the model is wrong. A homepage is an assembly of other content types.

### Define Fields By Purpose, With Clear Authoring Intent

Every field in a content type should have a defined purpose, an expected data type, and clear guidance for authors. Fields added ad hoc, without documentation, become inconsistent across entries and impossible to query reliably.

For each field, define:

- the field name and a human-readable label authors understand;
- the data type (text, rich text, number, date, reference, media, boolean);
- whether it is required or optional, and the consequence of leaving it empty;
- authoring guidance: what belongs here, what does not, and examples;
- validation rules (character limits, formats, allowed values) that prevent garbage data.

A field with no guidance becomes a dumping ground. A field with no validation becomes inconsistent. Treat the authoring experience as part of the model.

### Model Relationships Explicitly

Content rarely exists in isolation. A product belongs to a category, references related products, and is sold by a vendor. An article has authors, tags, and a series. These relationships are the connective tissue that enables dynamic assembly, related content, and consistent updates.

Define relationships deliberately:

- use references (pointing to another content entity) rather than duplicating data, so updates propagate;
- distinguish one-to-one, one-to-many, and many-to-many relationships and model each correctly;
- decide whether relationships are owned by one side or both, and what happens when one side is deleted;
- consider ordering and ranking within relationships, since "related products" often implies a curated order.

Duplicating related data across entries guarantees drift: one copy is updated, the others are not. References keep content consistent.

### Design For Reuse Across Channels

A content model built for one website will not survive a mobile app, an email campaign, a voice assistant, or an API consumer. Reuse requires that content be structured into meaningful, self-contained pieces that can be assembled differently per channel.

Build for reuse:

- break content into components (a headline, a summary, a body, a call to action) that can be selected per channel;
- avoid baking layout or channel-specific markup into fields;
- define which fields are required for each channel so a content entry cannot be published half-complete;
- consider length variants (short title for mobile, long title for web) where channel constraints differ.

If content must be re-authored for each new channel, the model has coupled content to presentation.

### Separate Evergreen From Time-Bound Content

Content has different lifecycles. A product description is evergreen and changes rarely. A promotion is time-bound and expires. Mixing these in one content type creates stale content, broken displays, and authoring confusion.

Distinguish lifecycles:

- model time-bound content (events, promotions, announcements) as separate types with start and end dates;
- define what happens when time-bound content expires: hide it, archive it, redirect it;
- keep evergreen content stable so links and references do not break;
- use scheduling so content can be authored in advance and published automatically.

A model that treats a temporary banner and a permanent policy page the same way will accumulate rot.

### Plan For Localization From The Start

If content may ever be translated, the model must support it from the beginning. Retrofitting localization into a model built for one language is painful and error-prone.

For localization readiness:

- avoid hard-coding text in templates; all translatable text lives in fields;
- model translation as a variant of the same content entity, not a separate entity;
- account for text expansion (translated text is often 20 to 40 percent longer) in field limits and layouts;
- decide which fields are translatable and which are language-neutral (images, numbers, codes);
- plan fallback behavior when a translation is missing.

A model that cannot be localized locks the product to one market.

### Govern The Model With Versioning And Ownership

A content model is not static. Fields are added, renamed, deprecated, and restructured over time. Without governance, the model drifts, fields accumulate, and no one knows which are still used.

Govern the model:

- version the schema so changes are deliberate and traceable;
- document ownership: who can add fields, who can deprecate them, who maintains the model;
- deprecate fields with a plan rather than deleting them, since existing content depends on them;
- audit field usage periodically to find dead fields that can be retired.

An ungoverned model becomes a graveyard of unused fields that no one dares to remove.

## Common Traps

### Modeling Pages Instead Of Content Types

Structuring content around page layouts couples content to one presentation and prevents reuse across channels and redesigns.

### Duplicating Related Data Across Entries

Copying related information into each content entry guarantees drift; use references so updates propagate from one source.

### Adding Fields Without Authoring Guidance

Fields with no documentation or validation become inconsistent dumping grounds that cannot be queried reliably.

### Baking Layout Into Content Fields

Embedding markup, styling, or channel-specific structure in fields prevents reuse and forces re-authoring for each surface.

### Mixing Evergreen And Time-Bound Content

Combining permanent and temporary content in one type creates stale displays and broken references when the temporary content expires.

### Ignoring Localization Until It Is Needed

Retrofitting translation support is expensive; model translatable fields and fallback behavior from the start.

### Letting The Model Drift Without Governance

Without versioning, ownership, and deprecation processes, the schema accumulates unused fields that no one understands.

## Self-Check

- [ ] Content is modeled as types (article, product, event) with intrinsic fields, not as pages defined by one layout.
- [ ] Each field has a defined purpose, data type, required status, authoring guidance, and validation rules.
- [ ] Relationships between content entities use references, with clear cardinality and deletion behavior defined.
- [ ] Content is broken into reusable components that can be assembled differently per channel without re-authoring.
- [ ] Evergreen and time-bound content are separate types with appropriate lifecycle and expiration handling.
- [ ] The model supports localization through translatable fields, translation variants, text-expansion tolerance, and fallback behavior.
- [ ] The schema is versioned, with documented ownership and a deprecation process for unused fields.
- [ ] No layout, markup, or channel-specific structure is baked into content fields.
