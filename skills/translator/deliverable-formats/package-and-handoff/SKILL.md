---
name: package_and_handoff.md
description: Use when the agent is assembling the final translation package for delivery, organizing files naming and folder structure, producing handoff documentation and readme files, verifying completeness against the source set, managing secure transfer of deliverables, or conducting the final pre-delivery check that confirms the package is complete correct and ready for the client.
---

# Package And Handoff

Packaging and handoff is the last operation in a translation project and the one the client judges first. The linguistic work may be excellent, the formatting may be correct, and the files may be individually complete, but if the package is disorganized, files are missing or misnamed, the folder structure does not match what the client's downstream process expects, or the handoff lacks the documentation needed to use the deliverable, the client's first experience is confusion and rework. Agents miss issues at handoff because the work feels finished and the packaging feels mechanical, because the translator's working view shows complete content while the package assembled from it may be incomplete, and because the cost of a bad handoff is paid by the client in a different environment the translator never sees. The harm this skill prevents is deliverables that are technically translated but practically undeliverable: incomplete packages that force the client to request missing files, misnamed files that break automated import, missing documentation that leaves the client guessing at scope and basis, and insecure transfer that exposes confidential content. A clean handoff is a deliberate, verified operation, not the act of attaching files to a message.

Use this skill when assembling the final deliverable package, organizing files and folders for client handoff, producing handoff documentation, verifying package completeness against source, managing secure transfer, or conducting the final pre-delivery check. The goal is a complete, correctly structured, documented, and securely transferred package that the client can use immediately without follow-up queries.

## Core Rules

### Assemble The Package Against A Completeness Checklist

Handoff must be verified against an explicit completeness checklist, not assembled from memory. The checklist should cover every element the deliverable requires: all translated files in every agreed format, the source files or references, any bilingual or review files, the translation memory and termbase exports if agreed, the handoff documentation, and any client-specific artifacts such as a delivery note or compliance statement. Build the package item by item against the checklist and confirm each item is present before declaring the package complete. A package assembled without a checklist will be missing something, and the missing item is usually discovered by the client after the translator has considered the work done, which forces an embarrassing follow-up and erodes confidence. The checklist is the objective measure that the package is complete; without it, completeness is an assumption.

### Organize Files And Folders To Match The Client's Downstream Process

The package structure must match what the client's downstream process expects, not what is convenient for the translator. If the client's content management system imports from a specific folder structure, replicate it. If the client's build expects resource files in specific locations with specific names, match them. If the client reviews by language, organize the top level by language and the content within consistently. Use a naming convention that is explicit, consistent, and unambiguous: include the language variant, the version, and a clear file identifier, avoiding names that differ only by invisible characters or that collide across languages. Confirm the naming and structure with the client at the specification stage, because a package structured for translator convenience that ignores the client's import or build expectations forces the client to reorganize, which is rework the translator caused. The package is delivered to be used, and its structure must serve that use.

### Produce Handoff Documentation That Makes The Package Self-Explaining

A package without documentation forces the client to guess at its scope, basis, and contents. Produce a handoff document, often a readme or delivery note, that states: what the package contains, file by file or folder by folder; the source version and basis the translation corresponds to, with checksums where applicable; the language variant and locale delivered; the scope, including any exclusions or partial-scope items; any known issues, queries, or flags the client must address; and the contact for follow-up. The documentation should let a person who was not involved in the project understand and use the deliverable without further communication. For complex packages, include a manifest that lists every file with its checksum so the client can verify transfer integrity. Handoff documentation is not overhead; it is the interface between the translator's work and the client's use of it, and a self-explaining package prevents the queries and confusion that follow an undocumented delivery.

### Verify Integrity And Correspondence Before Transfer

Before transferring the package, perform a final integrity and correspondence verification. Confirm that every file in the package opens and parses in its native format, catching any corruption introduced during assembly or export. Confirm that the translated files correspond to the correct final source version, with no superseded content remaining. Confirm that file integrity is intact by computing checksums on the deliverable files, which the client can use to verify the transfer. Confirm that no working artifacts, temporary files, internal notes, or source content that should not be in the deliverable have been included. This final verification is the last gate before the package leaves the translator's control, and it catches the assembly errors that segment-level review cannot see. A package transferred without final verification may contain a broken file, a stale version, or a leaked working artifact that the client discovers first.

### Transfer Through A Secure And Appropriate Channel

The transfer channel must match the sensitivity of the content. Confidential, personal, legal, medical, or commercially sensitive content requires a secure transfer method, encrypted transfer, a secure portal, or a client-approved system, not an unencrypted email attachment or an unmanaged file-sharing link. Confirm the client's preferred transfer method at the specification stage and use it. For large packages, use a method that handles size reliably and provides transfer confirmation. For content subject to data protection or regulatory requirements, ensure the transfer method complies, and retain only what is appropriate, deleting intermediate copies that should not persist. An insecure transfer of sensitive content is a confidentiality breach regardless of the quality of the translation, and it can create legal and reputational liability that dwarfs the project value. Treat transfer security as a substantive part of handoff, not a logistical afterthought.

### Confirm Receipt And Close The Handoff

Handoff is not complete when the package is sent; it is complete when the client confirms receipt and the package is accepted. Confirm receipt with the client, particularly for large or sensitive packages where transfer failure is a risk. Address any immediate issues the client raises on opening the package, distinguishing transfer problems, which are the translator's to resolve, from acceptance questions, which follow the agreed review process. Record the handoff as complete, with the delivery date, the package contents, the source basis, and the transfer confirmation, so that the project has a defensible closeout record. A handoff that is sent but never confirmed may have failed silently, and the translator only learns when the client follows up weeks later wondering where the deliverable is.

## Common Traps

### Assembling The Package From Memory Without A Checklist

Relying on memory to build the package guarantees that something is missing, discovered by the client after the work is declared done. This is a trap because the working view shows complete content. Assemble against an explicit completeness checklist.

### Structuring The Package For Translator Convenience

A folder structure that ignores the client's import or build process forces the client to reorganize. This is a trap because the structure looks logical to the translator. Match the client's expected structure and naming, confirmed at specification.

### Delivering Without Handoff Documentation

An undocumented package forces the client to guess at scope, basis, and contents, generating queries and eroding confidence. This is a trap because the files are all present. Produce a self-explaining handoff document and manifest.

### Skipping The Final Integrity And Correspondence Check

Transferring without verifying that files parse, correspond to the final source, and contain no working artifacts lets assembly errors reach the client. This is a trap because segment review passed. Perform a final integrity and correspondence verification before transfer.

### Using An Insecure Transfer For Sensitive Content

Sending confidential or regulated content by unencrypted email or unmanaged link is a confidentiality breach regardless of translation quality. This is a trap because the transfer feels routine. Match the transfer channel to content sensitivity and client preference.

### Considering Handoff Complete When The Package Is Sent

A package sent but not confirmed may have failed to arrive, and the gap is discovered late. This is a trap because sending feels like finishing. Confirm receipt and record a defensible closeout.

## Self-Check

Before delivering a translation package to the client, verify:

- The package was assembled against an explicit completeness checklist, and every required element, all formats, source references, review files, memory and termbase exports, documentation, and client-specific artifacts, is confirmed present.
- The file and folder structure, including naming convention with language variant, version, and identifiers, matches the client's downstream import or build process, confirmed at specification rather than chosen for translator convenience.
- Handoff documentation is included, making the package self-explaining: contents, source basis with checksums, language variant, scope and exclusions, known issues, and contact, plus a manifest for complex packages.
- A final integrity verification confirmed every file opens and parses in its native format, the translated files correspond to the correct final source version, and no working artifacts, temporary files, or leaked source content remain.
- The transfer channel matches the content's sensitivity and the client's preference, using secure or encrypted methods for confidential, personal, legal, medical, or commercially sensitive content, with compliance to any data protection requirements.
- Checksums were computed on deliverable files so the client can verify transfer integrity.
- Receipt was confirmed with the client, and any immediate transfer issues were distinguished from acceptance questions.
- A closeout record exists capturing delivery date, package contents, source basis, and transfer confirmation, making the handoff defensible and complete.
