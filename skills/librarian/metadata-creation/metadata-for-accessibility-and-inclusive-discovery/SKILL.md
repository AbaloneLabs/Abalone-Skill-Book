---
name: metadata_for_accessibility_and_inclusive_discovery.md
description: Use when the agent is creating metadata that supports accessible discovery and use, describing resources for users with disabilities, adding accessibility metadata such as screen reader compatibility or alternative format availability, ensuring discovery systems surface accessible formats, or diagnosing why patrons with disabilities cannot find or use resources the library holds.
---

# Metadata For Accessibility And Inclusive Discovery

Metadata determines not only what can be found but who can find and use it. A resource may exist in an accessible format, but if the metadata does not record that accessibility, the patron who needs it cannot discover it through search, and the library effectively withholds the resource from the users who depend on it. Accessibility metadata, describing whether a resource is screen-reader compatible, has captions or transcripts, offers alternative formats, or meets specific accessibility standards, is the bridge between an accessible collection and the patrons who need accessible access. When it is absent, incomplete, or buried where discovery systems cannot surface it, the collection's accessibility is invisible and therefore unusable for the people it matters to most. The judgment problem is creating accessibility metadata that is accurate, complete, standardized, and exposed through discovery in a way that lets patrons filter and find what they can actually use, rather than recording accessibility as an afterthought or assuming a resource is accessible because it is digital.

Use this skill when creating or auditing metadata for accessibility, when configuring discovery systems to surface accessible formats, when describing resources for users with disabilities, or when diagnosing why patrons cannot find accessible versions of materials the library holds. The goal is to prevent the agent from treating accessibility metadata as optional, from recording it inconsistently, and from allowing discovery systems to hide accessible formats behind generic records.

## Core Rules

### Treat Accessibility Metadata As Essential, Not Optional

Accessibility metadata is often treated as a nice-to-have, added when convenient and omitted when not. For patrons with disabilities, it is essential infrastructure: without it, they cannot determine whether a resource is usable, and they cannot search for materials that meet their needs. Treating it as optional makes the collection inaccessible in practice regardless of what accessible formats are held.

Treat it as essential by:

- including accessibility elements in the default metadata record, not as a special addition;
- training metadata creators to record accessibility as a standard part of description;
- building accessibility fields into templates and workflows so they are not forgotten;
- recognizing that omitting accessibility metadata withholds information from patrons who depend on it.

A record that does not state a resource's accessibility forces every patron who needs that information to ask individually, which most will not do.

### Record Accessibility Accurately And Specifically

Vague accessibility claims are nearly as useless as none. "Accessible" or "alternative formats available" tells the patron nothing they can act on. Accurate accessibility metadata records specifically what accessibility features exist, against what standards, so a patron can determine whether the resource meets their particular needs.

Record specifically:

- the format and its accessibility features, such as screen-reader compatible, captioned, transcribed, described video;
- the standard conformance level, such as WCAG 2.1 AA, where applicable;
- the presence of alternative text, structural navigation, or reflowable text;
- the availability of alternative formats through the library or on request;
- known limitations, such as partial captioning or images without descriptions.

A patron who uses a screen reader needs to know whether the resource is genuinely compatible, not that it is "digital." Specificity lets them filter and choose.

### Use Standardized Accessibility Vocabularies And Schemas

Accessibility described in free text cannot be reliably searched, filtered, or exchanged. Standardized vocabularies and schema elements, such as Schema.org accessibility properties, ONIX accessibility codes, or WCAG conformance levels, make accessibility metadata machine-actionable and interoperable across systems. Without standards, accessibility information is trapped in individual records.

Use standards by:

- applying Schema.org accessibility properties, such as accessibilityFeature, accessibilityHazard, and accessibilitySummary;
- using ONIX accessibility codes for commercial e-resources where applicable;
- recording WCAG conformance levels where the resource has been evaluated;
- mapping local accessibility fields to standard vocabularies for exchange and discovery;
- avoiding idiosyncratic terms that only local staff understand.

Standardized metadata lets discovery systems offer accessibility filters and lets records exchange cleanly with consortial and aggregator systems.

### Expose Accessibility Through Discovery Systems

Accessibility metadata that exists in a record but is not surfaced in the discovery interface is invisible to patrons. The discovery system must allow patrons to search for, filter by, and identify accessible resources, or the metadata serves no practical purpose. Exposing accessibility is as important as recording it.

Expose by:

- configuring the discovery layer to display accessibility fields prominently in records;
- enabling faceted filtering by accessibility feature, such as captioned or screen-reader compatible;
- ensuring accessible format records are linked to or grouped with the main record so patrons find them;
- avoiding burying accessibility information in notes fields that are not searchable or filterable;
- testing the discovery experience from the perspective of a patron who needs accessibility filtering.

A patron should be able to search for a topic and filter to resources they can use, without having to open each record and read the notes.

### Link Accessible Formats To The Underlying Work

A library may hold a title in multiple formats, some accessible and some not. If each format is a separate, unlinked record, the patron seeking an accessible version may not find it, or may find the inaccessible version and conclude the title is unavailable to them. Accessible formats must be linked to the underlying work so all manifestations are discoverable together.

Link formats by:

- using expression and manifestation relationships to connect formats of the same work;
- ensuring the accessible format record references the main work and vice versa;
- presenting format options, including accessible ones, within a unified record display;
- avoiding isolating accessible formats in separate collections or catalogs that patrons do not know to search.

A patron who finds a title should immediately see all available formats, including accessible ones, without a separate search.

### Avoid Assuming Digital Means Accessible

A common and damaging assumption is that a digital resource is inherently accessible. Many e-resources are not: PDFs may be image-based and unreadable by screen readers, videos may lack captions, interactive content may be keyboard-inaccessible, and DRM may block assistive technology. Assuming accessibility based on format leads to metadata that claims access that does not exist.

Avoid the assumption by:

- testing digital resources for actual accessibility before claiming it;
- recording known accessibility features based on verification, not format inference;
- noting hazards, such as flashing content or lack of captions, honestly;
- distinguishing born-accessible resources from those that require accommodation.

A patron told a resource is accessible when it is not wastes their time and erodes trust in the library's metadata.

### Maintain Accessibility Metadata As Resources And Standards Evolve

Accessibility metadata is not static. Resources are remediated, captions are added, standards evolve, and conformance levels change. Records that record accessibility once and are never updated become inaccurate, claiming features that were added or omitting ones that now exist. Maintenance is part of accessibility metadata practice.

Maintain by:

- updating records when resources are remediated or reformatted;
- reviewing accessibility claims against current standards periodically;
- incorporating accessibility into ongoing quality assurance workflows;
- responding to patron reports of inaccurate accessibility metadata promptly.

Stale accessibility metadata that overstates or understates access harms patrons either way.

## Common Traps

### Treating Accessibility Metadata As Optional

Omitting accessibility fields by default withholds essential information from patrons who depend on it. Include it as standard.

### Vague Accessibility Claims

"Accessible" or "alternative formats available" tells patrons nothing actionable. Record specific features and standards.

### Free-Text Accessibility That Cannot Be Searched

Accessibility in uncontrolled notes cannot be filtered or exchanged. Use standardized vocabularies and schema.

### Metadata Recorded But Not Exposed In Discovery

Accessibility that exists in records but is not surfaced in the interface is invisible. Configure discovery to display and filter by it.

### Isolating Accessible Formats From The Main Record

Separate, unlinked records for accessible formats hide them from patrons. Link all manifestations to the underlying work.

### Assuming Digital Means Accessible

Many digital resources are not accessible. Verify and record actual features rather than inferring from format.

### Stale Accessibility Metadata

Records that never update become inaccurate as resources are remediated or standards evolve. Maintain accessibility metadata over time.

## Self-Check

- [ ] Accessibility metadata is included as a standard part of the record, not treated as an optional addition.
- [ ] Accessibility features are recorded specifically and accurately, including standards conformance and known limitations.
- [ ] Standardized vocabularies and schema, such as Schema.org accessibility properties or ONIX codes, are used for searchability and interoperability.
- [ ] The discovery system surfaces accessibility fields and enables filtering by accessibility feature.
- [ ] Accessible format records are linked to the underlying work so all manifestations are discoverable together.
- [ ] No resource is claimed as accessible based on format alone without verification of actual features.
- [ ] Accessibility metadata is maintained and updated as resources are remediated or standards evolve.
- [ ] Patrons can search for a topic and filter to resources they can use without opening each record.
- [ ] Known accessibility hazards are recorded honestly rather than omitted.
- [ ] The discovery experience has been tested from the perspective of a patron who depends on accessibility filtering.