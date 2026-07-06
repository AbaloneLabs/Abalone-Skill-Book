---
name: project-and-asset-management.md
description: Use when the agent is managing a music production project, organizing sessions and assets, versioning project files, managing samples and plugins across systems, collaborating with remote contributors, or evaluating whether a project's file organization, naming, and backup practices protect the work and enable smooth handoff to mixing and mastering.
---

# Project and Asset Management

A music production project generates dozens of files — session files, audio stems, samples, MIDI, plugin presets, alternate versions — and the organization of those files determines whether the project survives to release or collapses into an unrecoverable mess. The judgment problem is that file management feels like overhead during the creative phase, so producers defer it, then discover too late that the session will not open because a sample is missing, that the "final" version cannot be distinguished from seven near-identical alternates, or that a drive failure destroyed weeks of work because there was no backup. Unlike performance or composition, asset management is pure risk: every shortcut is invisible until the moment it causes loss, and by then the work is gone. This skill covers the decisions that govern project and asset management: file organization, version control, sample and plugin management, collaboration handoff, and backup discipline.

## Core Rules

### Organize Files With a Consistent Project Structure From the Start

Every project should follow a consistent folder structure that separates session files, audio, samples, MIDI, bounces, and deliverables, so that any file can be found by convention rather than memory. The decision is to establish a template structure and to use it from the project's first save, not to retrofit organization later. The trap is saving files wherever is convenient in the moment, which produces a scattered project where critical assets are unfindable. The criterion is that another engineer could open the project folder and navigate it without guidance. Define the structure, use it consistently, and enforce naming that identifies content (e.g., "SongName_VocalTake3.wav" not "Audio_012.wav").

### Version Session Files With Clear, Incremental Naming

Session files should be versioned incrementally (SongName_v01, v02, v03) so that each saved state is recoverable and the progression is traceable. The decision is to save a new version at every meaningful milestone — after each major edit, before any risky change, at the end of each session — rather than overwriting a single file. The trap is working in one session file and overwriting it, which means a bad save or a corrupted file destroys all work with no rollback. The criterion is that you can return to any previous state of the project. Version deliberately, and never overwrite the last known-good version without a backup.

### Manage Samples and Plugins for Portability

A session that relies on specific samples or plugin presets will not open correctly on another system if those assets are missing or different. The decision is to manage dependencies: store project-specific samples within the project folder, freeze or bounce tracks that depend on specific plugins, and document the plugins and versions used. The trap is assuming the session will always be opened on the same system, which fails when the project moves to a mix engineer, a collaborator, or a new computer. The criterion is that the session is portable — it opens and plays correctly on a system with the documented assets. Test portability by opening the project on a clean system before handoff.

### Establish Clear Handoff Packages for Collaborators

When a project moves to a mix engineer, a vocalist, or a label, the handoff package must contain everything needed and nothing extraneous. The decision is to prepare a defined handoff: consolidated audio stems from zero, clearly labeled and named; the session file if requested; documentation of tempo, key, and any special processing; and a readme explaining the contents. The trap is sending a raw session folder full of unused takes, missing assets, and ambiguous naming, which costs the recipient hours of reconstruction. Define a handoff template and prepare it deliberately for each recipient.

### Back Up With the 3-2-1 Rule and Verify Restores

A single copy of a project is a project waiting to be lost. The decision is to follow the 3-2-1 rule: three copies of the data, on two different media, with one copy offsite. In practice, this means the working copy, a local backup (external drive or NAS), and an offsite backup (cloud or remote drive). The trap is assuming a local backup is sufficient, which fails when the studio is damaged or the local drives fail together. Equally important, backups must be verified by test restores — an unverified backup may be silently corrupted. Schedule regular backups and periodic restore tests.

### Document Decisions and Context for Future Recall

A project will be reopened weeks, months, or years later by people who do not remember the decisions made, including the original producer. The decision is to document context in a project notes file: tempo, key, tuning, sample rate, the intent of each section, known issues, and the status of each version. The trap is relying on memory, which degrades rapidly and is unavailable to collaborators. The criterion is that a future reader can understand the project's state and decisions from the documentation alone. Maintain a running notes file from project start, and update it at each milestone.

## Common Traps

### Saving Files Wherever Is Convenient

The producer saves files to the desktop or a scratch folder during the session, intending to organize later. The false signal is that the work is progressing. The harm is a scattered project where critical assets are unfindable, sessions will not open due to missing files, and handoff is impossible. The mechanism is deferring organization. The remedy is a consistent structure from the start.

### Overwriting a Single Session File

The producer works in one file and saves over it repeatedly. The false signal is efficiency. The harm is total loss when a save corrupts the file or an unwanted change is saved, because there is no previous version to recover. The mechanism is treating versioning as overhead. The fix is incremental versioning at every milestone.

### Assuming the Session Will Always Open on the Same System

The producer does not manage sample and plugin dependencies, assuming the current setup is permanent. The false signal is that the session works now. The harm is a session that breaks when moved to a collaborator's system, a new computer, or after a plugin update, with missing samples and broken presets. The mechanism is ignoring portability. The remedy is dependency management and portability testing.

### Sending a Raw Session Folder as Handoff

The producer zips the entire session folder and sends it, full of unused takes and missing assets. The false signal is that the recipient has "everything." The harm is hours of reconstruction for the recipient, missing files, and a damaged professional reputation. The mechanism is treating handoff as file transfer rather than package preparation. The fix is a defined handoff template.

### Trusting a Single Local Backup

The producer backs up to one external drive and considers the project safe. The false signal is that a backup exists. The harm is total loss when the local drives fail together, are stolen, or are damaged, because there is no offsite copy. The mechanism is underestimating failure correlation. The remedy is the 3-2-1 rule with offsite backup.

### Relying on Memory for Project Context

The producer does not document decisions, assuming they will remember. The false signal is that the project is fresh in mind. The harm is lost context when the project is reopened weeks later, with no record of intent, known issues, or version status, leading to confusion and rework. The mechanism is treating documentation as overhead. The fix is a running notes file.

## Self-Check

- Does the project follow a consistent folder structure from the first save, with naming that identifies content without needing to open files?
- Are session files versioned incrementally at every meaningful milestone, so any previous state is recoverable?
- Are sample and plugin dependencies managed — project samples stored locally, plugin-dependent tracks frozen or bounced, dependencies documented — so the session is portable?
- Is there a defined handoff package template, prepared deliberately for each collaborator, containing consolidated stems and documentation?
- Does the project follow the 3-2-1 backup rule with an offsite copy, and have backups been verified by test restores?
- Is there a running project notes file documenting tempo, key, decisions, known issues, and version status, readable by someone who was not present?
- If the studio drives failed tonight, could the project be fully recovered from backup, and if a new engineer took over tomorrow, could they understand the project from the files alone?
