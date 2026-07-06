---
name: file_version_and_integrity.md
description: Use when the agent is managing source file versions during a translation project, preventing work against outdated or superseded source, tracking which source version a translation is based on, handling mid-project source updates, verifying file integrity with checksums, avoiding version collisions and duplicate work, or ensuring the delivered translation corresponds to the correct final source version.
---

# File Version And Integrity

Translation projects unfold over time, and source files change during that time. A client sends version 3 of a document, the translator works for two days, and the client sends version 4 with revisions that invalidate part of the completed work. Two team members translate different sections against different source versions and the sections cannot be reconciled. A file is transferred, something corrupts a few bytes, and the translator works against a subtly damaged source that differs from the client's actual file. Version and integrity management is the discipline of knowing exactly which source a translation is based on, ensuring that source is the correct and current one, and guaranteeing the delivered translation corresponds to the intended final source. Agents miss issues here because files all look the same, because version labels are informal or absent, and because the cost of working against the wrong version only appears at delivery when the client compares the translation to a source that has moved on. The harm this skill prevents is wasted work against superseded versions, unreconcilable contributions from different team members, deliverables that do not match the client's final source, and the disputes that arise when no one can establish which version was actually translated.

Use this skill when managing source versions during a project, preventing work against outdated source, handling mid-project updates, verifying file integrity, avoiding version collisions in team work, or confirming deliverable-to-source correspondence at handoff. The goal is a translation whose source basis is known, current, and verifiable from intake to delivery.

## Core Rules

### Establish And Record The Source Baseline At Intake

At intake, establish a recorded baseline of the source: the exact files, their versions, their checksums or hashes, and the date and source of delivery. This baseline is the reference against which all later changes are measured. Without a recorded baseline, no one can later determine whether a source file changed, whether a delivered translation corresponds to the current source, or whether two team members worked against the same version. Record the baseline explicitly, store the source files in a way that preserves their identity, such as a version-controlled repository or a timestamped archive, and reference the baseline throughout the project. The baseline is not a formality; it is the foundation of traceability, and a project without one cannot defend its deliverable against a version dispute.

### Track Which Source Version Each Translation Unit Is Based On

As work proceeds, maintain the link between translated content and the specific source version it was based on. This is straightforward in a CAT tool with version-aware translation memory, but it must be explicit in manual or semi-manual workflows. When the source updates mid-project, the translator must be able to identify which completed segments are affected, which are still valid, and which must be retranslated against the new source. A translation memory or bilingual file that does not record its source version becomes ambiguous after an update: a segment may reflect the old source, the new source, or a mix, and no one can tell which. Maintain version metadata at the segment or file level so that retranslation after an update targets only the changed content, not the entire project.

### Handle Mid-Project Source Updates Through A Controlled Process

Source updates during a project are common and must be handled through a controlled process, not absorbed ad hoc. When a new source version arrives, compare it against the baseline to identify exactly what changed: new content, deleted content, revised content, and restructured content. Classify each change by its impact on completed work: unchanged segments need no action, revised segments need retranslation or review, deleted segments are removed from scope, and new segments are added to scope. Communicate the impact to the client, including any schedule or cost consequences, before proceeding. Apply the update by re-aligning the bilingual file to the new source, preserving valid completed work and targeting only the changed content. An uncontrolled update, where the translator simply switches to the new source and reworks everything or, worse, works against a mix of versions, wastes effort and produces a deliverable whose basis is unclear.

### Prevent Version Collisions In Team Work

When multiple translators or reviewers work on the same project, version collisions are a primary risk. Two translators working against different source versions produce sections that cannot be merged cleanly. Two reviewers editing the same bilingual file without coordination overwrite each other's work. A translator working from a shared drive picks up a file another translator has partially edited. Prevent collisions by assigning clear ownership of files or sections, by distributing source from a single controlled baseline rather than letting team members source their own copies, and by using a version-controlled or check-in-check-out workflow that prevents concurrent uncoordinated edits. Where concurrent work is unavoidable, define a merge process and a single integrator responsible for reconciling contributions. Uncoordinated team work against uncontrolled versions produces a deliverable assembled from incompatible pieces.

### Verify File Integrity At Every Transfer

Every file transfer, from client to translator, between team members, and from translator to client, is a point where integrity can be lost. A transfer may truncate a file, corrupt bytes, or deliver an incomplete set. Verify integrity at every transfer using checksums or hashes: compute a checksum on the source, confirm it matches after transfer, and flag any mismatch immediately. For large projects or high-stakes content, integrity verification is mandatory because a corrupted file worked on silently produces a corrupted deliverable. Do not assume that a file that opened correctly is intact; openability does not prove integrity, because corruption may affect content the opening tool did not read. Use checksums as the objective measure and investigate any mismatch before proceeding.

### Confirm Deliverable-To-Source Correspondence At Handoff

At delivery, confirm and document that the delivered translation corresponds to the correct final source version. This means verifying that the source the translation is based on is the version the client intends as final, that no superseded content remains in the deliverable, and that any mid-project updates were fully incorporated. Provide the client with the source version reference, the checksum, and a statement of correspondence so that both parties agree on what was translated. A deliverable without a source-version attestation is ambiguous: the client may compare it to a newer source and find discrepancies that are actually version differences, not translation errors. Establishing correspondence at handoff prevents these disputes and provides a defensible record of what was done.

## Common Traps

### Working Against An Unversioned Or Unlabeled Source

Translating a source file with no version label makes it impossible to later determine what was translated or to detect that the source has moved on. This is a trap because the file looks current and the problem only appears at delivery. Record a versioned baseline at intake.

### Absorbing A Mid-Project Update Without Impact Analysis

Switching to a new source version without comparing it to the baseline wastes completed work and produces a deliverable of unclear basis. This is a trap because the new source looks like the right thing to use. Compare, classify impact, and communicate before applying an update.

### Losing The Segment-Level Link To Source Version

A bilingual file or translation memory that does not record its source version becomes ambiguous after an update, so retranslation cannot target only changed content. This is a trap because the content looks complete. Maintain version metadata at the segment or file level.

### Team Members Sourcing Their Own Copies Of Files

When team members obtain source independently, they may work against different versions and produce unreconcilable sections. This is a trap because each copy looks identical. Distribute source from a single controlled baseline with clear ownership.

### Assuming An Openable File Is An Intact File

Treating successful opening as proof of integrity misses corruption in content the tool did not read. This is a trap because the file appears fine. Verify integrity with checksums at every transfer, not by openability.

### Delivering Without A Source-Version Attestation

A deliverable without a stated source basis invites disputes when the client compares it to a different version and finds discrepancies. This is a trap because the translation may be correct for its source. Confirm and document deliverable-to-source correspondence at handoff.

## Self-Check

Before delivering a translation, verify:

- A recorded source baseline exists at intake, capturing exact files, versions, checksums, delivery date, and source, and is referenced throughout the project.
- Each translation unit or file carries version metadata linking it to the specific source version it was based on, so retranslation after an update can target only changed content.
- Any mid-project source update was handled through a controlled process: compared against the baseline, changes classified by impact, consequences communicated to the client, and only changed content retranslated.
- Team work was organized with clear file or section ownership, source distributed from a single controlled baseline, and a defined merge or integration process preventing version collisions.
- File integrity was verified with checksums or hashes at every transfer, client to translator, between team members, and translator to client, and any mismatch was investigated before proceeding.
- At handoff, the delivered translation is confirmed to correspond to the correct final source version, with no superseded content remaining and all updates incorporated.
- A source-version attestation accompanies the deliverable, stating the source version, checksum, and correspondence, so both parties agree on what was translated.
- No work was performed against an unversioned, superseded, or integrity-compromised source without detection and correction.
