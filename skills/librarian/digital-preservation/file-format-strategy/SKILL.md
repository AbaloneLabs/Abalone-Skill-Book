---
name: file_format_strategy.md
description: Use when the agent is choosing file formats for long-term preservation, evaluating format obsolescence risk, planning format migration or normalization, or establishing acceptable formats for a digital repository.
---

# File Format Strategy

File formats are the substrate of digital preservation. A format that is well-documented, widely supported, and open gives the institution a fighting chance of accessing content decades later. A format that is proprietary, opaque, or narrowly supported becomes a time bomb, readable today and unreadable when the creating software disappears. File format strategy is the set of decisions about which formats to accept at ingest, which to normalize, when to migrate, and how to monitor the format landscape over time. These decisions are reversible only at high cost, and getting them wrong silently locks content into decay. Good format strategy favors openness and documentation, plans for migration rather than pretending formats are permanent, and monitors obsolescence as an ongoing responsibility.

Use this skill when setting acceptable formats for a repository, evaluating a format's preservation risk, planning normalization or migration, or establishing format policy. The goal is to prevent the agent from accepting any format handed over, treating proprietary formats as safe, delaying migration until access fails, or ignoring the monitoring that preservation requires.

## Core Rules

### Favor Open, Well-Documented Formats For Preservation

The strongest preservation formats are open, with publicly documented specifications, widely implemented, and free of licensing restrictions. These properties maximize the chance that tools to read them will exist in the future.

Preferred preservation format properties:

- open specification, not proprietary or trade-secret;
- widely adopted and implemented by multiple tools;
- no licensing or patent barriers to implementation;
- self-describing where possible, containing its own metadata;
- lossless for content where fidelity matters;
- stable, with a mature specification not in flux.

Examples include TIFF for images, WAV or FLAC for audio, PDF/A for documents, and plain text or XML for structured data. Proprietary or undocumented formats should be normalized to open equivalents at ingest where feasible.

### Distinguish Preservation Masters From Access Copies

Preservation and access have different format needs, and conflating them compromises both. Masters prioritize fidelity and longevity; access copies prioritize delivery and size.

Strategy:

- preserve masters in open, lossless, high-fidelity formats;
- generate access derivatives in compressed, web-friendly formats;
- keep masters separate from access systems to protect their integrity;
- regenerate access copies from masters as formats and needs change.

Never let the access format become the only copy. A JPEG access copy is not a preservation master.

### Evaluate Format Risk Systematically

Not all formats carry the same risk. A format risk assessment informs which to accept, normalize, or reject. Assess formats on several dimensions.

Risk dimensions:

- openness of the specification;
- number and independence of implementing tools;
- dependency on specific software or operating systems;
- presence of encryption or DRM that blocks access;
- embedded proprietary codecs or compression;
- adoption in the preservation community;
- stability and age of the format;
- complexity that complicates migration.

Use format registries like PRONOM and the Library of Congress Sustainability of Digital Formats to inform assessment. Document the risk rating for each format in your collection.

### Define Acceptable And Watch Formats In Policy

A repository should have a clear format policy stating which formats are accepted for preservation, which are accepted but flagged for normalization, and which are rejected or require special handling.

Policy elements:

- preferred preservation formats by content type;
- acceptable formats that will be normalized at ingest;
- watch-list formats with elevated obsolescence risk;
- rejected formats and the rationale;
- handling for formats that cannot be normalized, emulation candidates;
- review cycle for the format policy.

The policy makes ingest decisions consistent and defensible. Without it, each deposit is handled ad hoc.

### Normalize At Ingest Where Feasible

Normalization converts deposited files to preferred preservation formats at ingest. It front-loads the migration burden and creates a homogeneous collection that is easier to preserve.

Normalization decisions:

- normalize to open, lossless formats where a clean conversion exists;
- preserve the original alongside the normalized version when conversion is lossy or uncertain;
- document the normalization event and any quality loss;
- avoid normalizing when the original format carries significant properties the target cannot capture.

Normalization is not always appropriate. Some formats carry information, like embedded macros or dynamic features, that normalization loses. Assess per format.

### Plan Migration Before Access Fails

Format migration is inevitable for long-lived content. Planning migration as a deliberate, scheduled activity is far cheaper and safer than emergency migration when access suddenly fails.

Migration planning:

- monitor the format landscape and tool support regularly;
- set triggers for migration, such as declining tool support or a new preferred format;
- test migrations on samples before applying at scale;
- preserve originals through migration to allow re-migration;
- document every migration as a preservation event;
- budget for migration as a recurring cost.

Waiting until a format is dead means migrating under crisis with fewer tools available.

### Preserve Significant Properties Through Format Changes

Different content types have different significant properties that must survive any format conversion. Losing them degrades the preserved object even if the file remains readable.

Identify significant properties by type:

- images, resolution, color depth, color accuracy;
- text, structure, formatting, embedded fonts, accessibility features;
- audio, sample rate, bit depth, channels, dynamic range;
- video, resolution, frame rate, interlacing, color space, audio sync;
- structured data, schema, relationships, validation rules.

Define what matters for each content type and verify that migrations preserve it. Test conversions against the significant properties, not just against whether the file opens.

### Handle Proprietary And Complex Formats Deliberately

Some content arrives in proprietary or complex formats that cannot be cleanly normalized: databases, software-dependent documents, email archives, 3D models, or dynamic web content. These need deliberate strategies.

Options:

- preserve the original and document its environment;
- normalize what can be converted and retain the original;
- plan for emulation of the creating environment;
- capture as much descriptive and technical metadata as possible;
- flag as high-risk and schedule regular review.

Do not pretend a complex proprietary format is safe because it opens today. Plan for the environment dependency explicitly.

### Monitor The Format Landscape Continuously

Format obsolescence is not a one-time problem. New formats emerge, tools drop support, and preferred formats change. Monitoring is an ongoing preservation responsibility.

Monitor:

- format registry updates and risk assessments;
- vendor announcements about format support and discontinuation;
- preservation community guidance and best practices;
- new migration tools and their quality;
- changes in your own collection's format distribution.

Schedule format review at least annually. A format policy that is never reviewed becomes stale and misleading.

### Document Format Decisions For Institutional Memory

Format strategy involves many decisions that future staff and systems must understand. Documentation preserves the reasoning and supports future migrations.

Document:

- the format policy and its rationale;
- risk assessments for each format;
- normalization rules and exceptions;
- migration events and their outcomes;
- significant properties defined per content type;
- monitoring findings and policy updates.

Documentation turns format strategy from individual expertise into institutional capability that survives staff turnover.

## Common Traps

### Accepting Any Format Handed Over

Without a format policy, the repository accumulates high-risk and proprietary formats that become unmaintainable. Define and enforce acceptable formats.

### Treating Proprietary Formats As Safe

A format that opens today may not open when its software disappears. Favor open, documented formats for preservation.

### Conflating Masters And Access Copies

An access copy is not a master. Preserve lossless masters and derive access copies from them.

### Waiting Until Access Fails To Migrate

Crisis migration has fewer tools and more risk. Plan and schedule migration before formats die.

### Normalizing Blindly

Some normalizations lose significant properties. Define what matters per content type and verify migrations.

### Ignoring Complex And Proprietary Formats

Databases, email, and dynamic content need deliberate strategies, not wishful thinking. Plan emulation or environment preservation.

### No Ongoing Monitoring

Format risk changes over time. Annual review of the format landscape is a preservation duty, not optional.

### Undocumented Format Decisions

Without documentation, future staff cannot understand or repeat format choices. Record policy, risk, and migration decisions.

## Self-Check

- Are preservation masters in open, well-documented, widely supported formats?
- Are preservation masters and access copies clearly distinguished and separated?
- Is each format in the collection assessed for obsolescence risk using registry resources?
- Does the repository have a documented format policy defining acceptable, watch, and rejected formats?
- Is normalization applied at ingest where clean conversion exists, with originals retained where it does not?
- Is migration planned proactively with triggers, testing, original retention, and event documentation?
- Are significant properties defined per content type and verified through migrations?
- Are proprietary and complex formats handled with deliberate preservation or emulation strategies?
- Is the format landscape monitored at least annually with policy updates as needed?
- Are all format decisions documented for institutional memory and future migrations?
