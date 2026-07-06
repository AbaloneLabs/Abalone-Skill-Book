---
name: daw_workflow_and_project_organization.md
description: Use when the agent is setting up a DAW project, organizing tracks and folders, building a session template, naming and color-coding tracks, managing session sample rate and bit depth, deciding on routing and bus structure, or establishing a workflow that scales from demo to finished production.
---

# DAW Workflow and Project Organization

A DAW session is a workspace, and like any workspace, its organization determines whether work flows or stalls. The judgment problem is that DAWs make it trivially easy to create tracks, add plugins, and accumulate takes, so sessions grow chaotically until they are unmanageable — hundreds of unlabeled tracks, inconsistent routing, missing files, and templates rebuilt from scratch every project. A disorganized session slows every task, makes collaboration painful, and introduces errors (wrong routing, lost edits, broken plugins) that surface at the worst moments. The agent must treat session organization as engineering, not tidiness, and must build templates, naming conventions, and routing structures that scale from the first demo to the final mix.

## Core Rules

### Build and Use a Project Template as the Starting Point

Every recurring project type (band recording, beat production, film cue, podcast) should have a template: a saved session with pre-built track layout, routing, grouping, color-coding, and commonly used plugins already configured. Starting from a template saves setup time, enforces consistency across projects, and ensures that routing and organization are correct from the first track. Rebuilding session structure from scratch each project is wasted time and invites inconsistency. The template should be refined over time as workflow improves, and different templates should exist for different project types.

**Decision criteria:** Does a project template exist for each recurring project type? Does every new session start from the appropriate template, with consistent track layout, routing, and organization?

### Name and Color-Code Tracks Consistently and Meaningfully

Track names should be descriptive and consistent (Kick, Snare, OH-L, Bass DI, Vocal Lead, Gtr Rhythm) so that anyone opening the session can navigate instantly. Color-coding by group (drums one color, bass another, vocals another) makes the session visually scannable. Generic names (Audio 1, Audio 2, Track 17) make sessions opaque and slow. Establish a naming convention and apply it from the first track; renaming hundreds of tracks later is far harder than naming them correctly on creation.

**Decision criteria:** Are all tracks named descriptively and consistently? Is color-coding applied by group, making the session visually navigable? Are there any generic default names remaining?

### Establish a Routing and Bus Structure That Scales

A session needs a logical routing structure: individual tracks route to group buses (Drum Bus, Vocal Bus, Guitar Bus), which route to a master bus, with sends to effect buses (reverb, delay) as needed. This structure allows group processing, easy level management, and clean export of stems. A flat structure (every track direct to master, no groups) becomes unmanageable as track count grows and prevents group processing. Build the bus structure into the template and maintain it as the session grows.

**Decision criteria:** Does the session use a group/bus structure (tracks to group buses to master, with effect sends)? Can groups be processed and managed as units? Can stems be exported cleanly?

### Set and Maintain Correct Sample Rate, Bit Depth, and Project Settings

Session sample rate and bit depth must match the project's delivery requirements and must be consistent across all related sessions. A project delivered at 48 kHz/24-bit should be recorded and mixed at those settings; mismatches require sample-rate conversion that can degrade quality. The project tempo and time signature must be set correctly (especially for production that will use MIDI, loops, or tempo-synced effects). Verify these settings at session creation, because changing sample rate mid-project is disruptive. Document the project's technical specs so collaborators align.

**Decision criteria:** Are sample rate, bit depth, tempo, and time signature set correctly at session creation and consistent across the project? Do they match delivery requirements and collaborator settings?

### Manage Files and Assets So Nothing Is Lost or Orphaned

A DAW session references audio files, and if those files are moved, renamed, or stored outside the project folder, the session breaks ("missing files"). All audio, MIDI, and related assets should be stored within the project folder, and the session should be saved with all referenced files. When receiving files from collaborators, save them into the project before importing. Use the DAW's "collect files" or "save as package" feature to consolidate assets for transfer. A session with external or orphaned files is a session that will break when moved.

**Decision criteria:** Are all audio, MIDI, and assets stored within the project folder? Does the session open without missing-file errors on any machine? Is "collect files" used before transfer?

### Save Versions Iteratively and Never Destroy Original Takes

Every significant change should be saved as a new version (Version 1, Version 2, or dated saves), preserving the ability to return to any prior state. Comp destructive editing (overwriting takes, deleting regions) — always keep original takes available, even after comping or editing. The DAW's undo is not a substitute for saved versions, because undo is lost when the session closes. A versioning discipline means no change is irreversible and no good take is ever lost to a bad edit. Name versions clearly so the progression is legible.

**Decision criteria:** Are versions saved incrementally with clear naming? Are original takes preserved and recoverable, never destroyed by edits? Could you return to any prior state of the project?

## Common Traps

### Starting Every Project From Scratch Without a Template

Rebuilding session structure, routing, and organization for every project wastes time and produces inconsistency. The trap mechanism is that each project feels unique, so a template seems inapplicable. The harm is wasted setup time and sessions organized differently each time. The corrective is building and refining templates for recurring project types.

### Generic Track Names and No Color-Coding

Sessions full of "Audio 1, Audio 2" with no visual organization are opaque and slow to navigate, especially for collaborators. The trap mechanism is that the creator knows what each track is, so naming feels unnecessary. The harm is confusion when returning to the session later or sharing it. The corrective is descriptive naming and group color-coding from the first track.

### Flat Routing With No Group Buses

A session where every track goes directly to the master, with no group buses, cannot be managed or processed at the group level and makes stem export impossible. The trap mechanism is that direct routing is the default and works for small sessions. The harm is unmanageable large sessions. The corrective is a group/bus routing structure built into the template.

### Sample Rate or Bit Depth Mismatch

Recording at 44.1 kHz and mixing at 48 kHz, or delivering at a different spec than tracked, requires conversion that can degrade quality and cause sync issues. The trap mechanism is that the settings are easy to overlook at creation. The harm is quality loss and rework. The corrective is verifying and matching settings at session creation.

### Orphaned or External Files That Break the Session

Audio files stored outside the project folder, or moved after import, cause "missing files" errors when the session is opened on another machine or after reorganization. The trap mechanism is that the session works on the original machine, so the problem is invisible. The harm is broken sessions during collaboration or transfer. The corrective is storing all assets within the project and using "collect files" for transfer.

### Destructive Editing With No Versioning

Overwriting takes, deleting regions, or editing without saving versions makes changes irreversible, so a bad edit or a change of direction cannot be undone. The trap mechanism is that the edit seems right at the moment. The harm is lost takes and inability to revert. The corrective is incremental versioning and preservation of originals.

## Self-Check

- Does every new session start from an appropriate project template with pre-built routing, grouping, and organization?
- Are all tracks named descriptively and consistently, with color-coding by group?
- Does the session use a group/bus routing structure that allows group processing and clean stem export?
- Are sample rate, bit depth, tempo, and time signature correct and consistent across the project?
- Are all audio, MIDI, and assets stored within the project folder, with no orphaned or external files?
- Are versions saved incrementally with clear naming, and are original takes preserved and recoverable?
- Could a collaborator open the session and navigate it efficiently from the organization alone?
