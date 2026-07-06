---
name: access_platforms.md
description: Use when the agent is choosing or configuring a digital collections access platform, designing how users discover and view digital objects, planning access controls and rights display, or evaluating platform sustainability and migration.
---

# Access Platforms

A digital collection only delivers value if users can actually find, view, and use it through an access platform. The platform is the interface between the carefully created digital objects and the public, researchers, students, and community members who need them. Platform choices shape discovery, usability, accessibility, rights enforcement, and long-term sustainability. A platform that is hard to search, that fails on mobile, that obscures rights, or that locks content into a proprietary system undermines the entire digitization investment. Good access platform decisions start from user needs and collection characteristics, evaluate sustainability and interoperability honestly, enforce rights and privacy appropriately, and plan for the inevitable migration to a future system.

Use this skill when selecting, configuring, or migrating a digital collections platform, designing discovery and viewing experiences, or planning access controls and sustainability. The goal is to prevent the agent from choosing a platform by feature checklist alone, ignoring accessibility and mobile use, locking content into non-portable systems, or treating the platform as a one-time decision rather than a lifecycle commitment.

## Core Rules

### Start From User Needs And Collection Characteristics

Platform choice begins with who uses the collection and what the collection contains. A platform ideal for image-heavy art collections may fail for oral history audio, and a platform built for scholarly text may frustrate community historians.

Define:

- the primary user groups, scholars, students, community, genealogists, internal staff;
- the material types, images, text, audio, video, datasets, complex compound objects;
- the discovery patterns users expect, search, browse, facet, map, timeline;
- the viewing needs, zoom, page-turning, audio playback, transcript alignment;
- the interaction needs, download, cite, share, annotate, request corrections.

Match platform capabilities to these needs. A feature-rich platform that lacks the specific capability your users need is the wrong choice.

### Evaluate Discovery And Search Honestly

Discovery is the core function of an access platform. A platform with weak search hides the collection no matter how good the objects are.

Evaluate:

- full-text search where OCR or transcripts exist;
- fielded search by title, creator, subject, date;
- faceted navigation by material type, date, subject, collection;
- browse by collection, subject, and hierarchy;
- support for complex and Boolean queries;
- relevance ranking quality;
- handling of non-Latin scripts and diacritics;
- search performance at scale.

Test discovery with real user queries, not just known-item searches. Weak discovery is often invisible until real users fail to find things.

### Prioritize Accessibility From The Start

Accessibility is not an add-on. A platform that excludes users with disabilities fails its public mission and may violate legal obligations.

Require:

- conformance with WCAG standards at the targeted level;
- keyboard navigation and screen reader compatibility;
- alt text and descriptive metadata support for images;
- captions and transcripts for audio and video;
- sufficient color contrast and resizable text;
- accessible document viewers and media players;
- compatibility with assistive technologies.

Build accessibility into the selection criteria and test with real users, including those who use assistive technology. Retrofitting accessibility is costly and incomplete.

### Ensure Mobile And Low-Bandwidth Usability

Many users, especially community and public users, access collections on phones and limited connections. A platform that works only on desktop broadband excludes them.

Check:

- responsive design that works on phones and tablets;
- performance on slow connections, image sizing, lazy loading;
- offline or low-data access options where feasible;
- touch-friendly navigation and viewing;
- download options appropriate for mobile use.

Test on real mobile devices and slow networks, not just on office broadband.

### Enforce Rights And Access Controls Appropriately

The platform must implement the rights and access decisions made during selection and metadata creation. This includes both enabling legitimate access and restricting where required.

Implement:

- clear, standardized rights statements visible on each object;
- access controls for restricted or sensitive materials;
- authentication for licensed or internal-only content;
- takedown and review workflows for rights complaints;
- watermarks or other protections where policy requires;
- clear citation and reuse guidance for users.

Rights enforcement should be transparent, not hidden. Users should understand what they can and cannot do with an object.

### Plan For Interoperability And Harvesting

Digital collections rarely live in isolation. The platform should support interoperability with aggregators, search engines, and other systems.

Require:

- OAI-PMH or modern harvesting APIs for aggregation;
- persistent identifiers, ARKs, DOIs, handles, for stable linking;
- standard metadata export in Dublin Core, MODS, or other schemas;
- schema.org or structured data for search engine indexing;
- IIIF for image and audiovisual interoperability where applicable;
- APIs for programmatic access by researchers and partners.

Interoperability multiplies the collection's reach. A platform that traps content reduces return on investment.

### Evaluate Sustainability And Total Cost Of Ownership

A platform is a multi-year commitment with ongoing costs beyond the initial license or setup. Evaluate the total cost of ownership honestly.

Cost factors:

- licensing or subscription fees and their escalation;
- hosting, whether local, cloud, or vendor-managed;
- storage costs as the collection grows;
- staff time for administration and maintenance;
- upgrade and migration costs over the system's life;
- support and training costs;
- exit costs if migrating away.

A platform that is cheap to acquire but expensive to sustain, or that locks content behind high exit barriers, is a poor long-term choice.

### Avoid Vendor Lock-In And Ensure Portability

Content and metadata must remain portable so the institution is not trapped in a platform that declines or becomes unaffordable. Lock-in is a serious strategic risk.

Protect portability by:

- owning or easily exporting all content and metadata in open formats;
- avoiding proprietary metadata structures that do not map to standards;
- retaining master files outside the platform in a preservation repository;
- documenting the platform's data model and mappings;
- negotiating data export rights in contracts up front.

A platform migration is painful; a platform migration without portability is catastrophic. Build portability in from the start.

### Design For Collection Growth And Evolution

The platform must handle the collection as it grows and changes over years. A platform that fits today's collection may not fit tomorrow's.

Plan for:

- storage and performance scaling as objects accumulate;
- new material types and complex object support;
- evolving metadata standards and schema updates;
- integration with new systems and campus tools;
- user feature expectations that change over time.

Choose a platform with a development roadmap and a community or vendor committed to evolution, not a static product.

### Plan The Migration From The Beginning

Every platform will eventually be replaced. Planning for migration from the beginning reduces the cost and risk when that day comes.

Migration planning:

- maintain complete metadata and content exports;
- document the platform's data model and custom configurations;
- track usage analytics to inform future platform needs;
- review the platform against alternatives every few years;
- budget for migration as a known future cost.

Institutions that plan for migration migrate smoothly. Those that do not face crises when a platform is abandoned or becomes unaffordable.

## Common Traps

### Choosing By Feature Checklist Alone

A long feature list does not mean the platform fits your users or collection. Evaluate against real needs and real queries.

### Ignoring Accessibility

Accessibility retrofitted late is costly and incomplete. Require WCAG conformance and test with assistive technology users.

### Desktop-Only Design

Mobile and low-bandwidth users are excluded by platforms built only for desktop broadband. Test on real devices and networks.

### Weak Discovery

A platform that cannot find objects hides the collection. Test discovery with real user queries, not just known items.

### Hidden Or Missing Rights Display

Users cannot reuse what they do not understand. Display standardized rights statements clearly on every object.

### Vendor Lock-In

Content trapped in a proprietary system creates strategic risk. Ensure portability and export from the start.

### Underestimating Sustainability Cost

Licensing, hosting, storage, and staff time accumulate. Evaluate total cost of ownership, not just acquisition.

### No Migration Plan

Every platform is eventually replaced. Plan and budget for migration from the beginning.

## Self-Check

- Is the platform chosen based on defined user groups, material types, and discovery patterns?
- Does discovery support full-text, fielded, faceted, and browse access tested with real user queries?
- Does the platform conform to WCAG accessibility standards, tested with assistive technology?
- Does the platform work on mobile devices and low-bandwidth connections?
- Are rights statements, access controls, and reuse guidance clearly implemented and visible?
- Does the platform support interoperability through harvesting APIs, persistent identifiers, and IIIF?
- Is the total cost of ownership, including sustainability and escalation, evaluated honestly?
- Is content and metadata portable in open formats to avoid vendor lock-in?
- Can the platform scale with collection growth and evolving metadata and feature needs?
- Is there a migration plan, with exports, documentation, and budget, from the beginning?
