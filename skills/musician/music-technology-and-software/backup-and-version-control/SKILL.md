---
name: backup_and_version_control.md
description: Use when the agent is establishing a backup strategy for music projects, setting up version control for sessions and assets, recovering from data loss, deciding on storage and redundancy, planning for disaster recovery of creative work, or ensuring that sessions, audio, and project files survive drive failure, theft, or catastrophe.
---

# Backup and Version Control

Creative work that exists only on a single drive is work that can disappear in an instant — to drive failure, theft, corruption, or accidental deletion. The judgment problem is that backups feel like administrative overhead when everything is working, so they are deferred until the catastrophe that makes them essential, at which point they are too late. Music projects are especially vulnerable because they are large, complex (many interdependent files), and irreplaceable (a lost session cannot be recreated from memory). The agent must treat backup and version control as part of the creative workflow, must follow the redundancy rule that prevents single-point-of-failure loss, and must test restores, because an untested backup is a hope, not a safeguard.

## Core Rules

### Follow the 3-2-1 Rule for All Creative Work

The 3-2-1 rule is the minimum standard for data safety: keep at least 3 copies of the data (the working copy plus two backups), on at least 2 different media types (e.g., internal drive, external drive, cloud), with at least 1 copy offsite (cloud or physically remote). This structure survives any single failure: a drive crash is covered by the local backup, a local catastrophe (theft, fire) is covered by the offsite copy. A single external drive backup is not enough — it fails the same day the working drive does, or it is stolen with the computer. Build 3-2-1 into the workflow from the start.

**Decision criteria:** Are there at least 3 copies of all creative work, on 2 media types, with 1 offsite? Could any single failure (drive crash, theft, corruption) cause data loss?

### Automate Backups So They Do Not Depend on Memory

Manual backups ("I'll copy to the external drive when I remember") fail because humans forget, especially under deadline pressure. Backups should be automated: scheduled software backups to local drives, continuous cloud sync for working files, and regular full-image backups. Automation ensures backups happen consistently without relying on discipline. Verify that the automation is running (check logs, check that the backup drive is current) periodically, because silent backup failure is as bad as no backup.

**Decision criteria:** Are backups automated (scheduled software, continuous sync), not dependent on manual memory? Is the automation verified to be running and current?

### Version Creative Work Iteratively, Not Just Backup the Latest

Backup protects against total loss; version control protects against bad changes. Saving iterative versions (Version 1, Version 2, dated saves) means you can return to any prior state — before a destructive edit, before a mix decision that did not work, before a collaborator's changes. This is separate from backup: a backup of only the latest version preserves only the current state, losing the ability to revert. For DAW sessions, use incremental "save as" with clear naming; for code and text, use git or similar. Versioning makes creative work non-destructive and reversible.

**Decision criteria:** Are versions saved incrementally with clear naming, allowing return to any prior state? Is creative work non-destructive, with the ability to revert bad changes?

### Keep Project Files and Assets Together and Self-Contained

A DAW session is not a single file; it is a project file plus its referenced audio, MIDI, and settings. Backing up only the session file without its assets is useless, because the session cannot open without them. Projects must be stored self-contained (all assets within the project folder), and backups must capture the entire folder, not just the session file. Use the DAW's "collect files" or "save as package" feature before backup or transfer to ensure self-containment. A backup of a session without its audio is a false safeguard.

**Decision criteria:** Are projects self-contained, with all assets within the project folder? Do backups capture the entire project folder, not just the session file? Do sessions open from backup without missing files?

### Test Restores Regularly

An untested backup is an assumption, not a safeguard. Backup software can fail silently, formats can become incompatible, and restore processes can be more complex than expected. Periodically test the restore process: open a backed-up session, verify the audio is present and the session is intact, and confirm the restore works end to end. The time to discover a backup is corrupt is not during a crisis. Schedule restore tests at regular intervals and document any issues.

**Decision criteria:** Has a restore from backup been tested recently, confirming the backed-up sessions open intact? Is the restore process documented and verified?

### Plan for Catastrophe: Theft, Fire, and Total Local Loss

Local backups protect against drive failure but not against catastrophe that takes the whole location — theft of the computer and drives, fire, flood. The offsite copy (cloud or physically remote) is what survives these events. For irreplaceable creative work, the offsite copy must be current (synced regularly, not months old) and must be complete. Consider the scenario: if the studio burned tonight, what would be recoverable? If the answer is "nothing recent," the offsite strategy is inadequate. Plan for total local loss, not just drive failure.

**Decision criteria:** If the local location were destroyed (theft, fire), would recent creative work be recoverable from the offsite copy? Is the offsite copy current and complete?

## Common Traps

### Single Drive With No Backup

Creative work on a single drive, with no backup, is one drive failure from total loss. The trap mechanism is that the drive is working, so the risk feels theoretical. The harm is catastrophic, irreversible loss of irreplaceable work. The corrective is the 3-2-1 rule, applied from the start.

### Manual Backups That Depend on Memory

Backups that rely on remembering to copy files fail under pressure and are inconsistent. The trap mechanism is that the intention to back up feels sufficient. The harm is backups that are weeks or months out of date when needed. The corrective is automated backups.

### Backing Up Only the Latest Version

A backup of only the current state cannot revert bad changes, destructive edits, or unwanted collaborator modifications. The trap mechanism is that the latest version is the one that matters. The harm is inability to recover a prior, better state. The corrective is incremental versioning alongside backup.

### Backing Up Session Files Without Assets

A session file without its audio and assets is useless, yet many backups capture only the project file. The trap mechanism is that the file was "backed up." The harm is sessions that cannot open from backup. The corrective is self-contained projects and full-folder backup.

### Untested Backups That Silently Fail

Backup software can fail without obvious symptoms, producing backups that are incomplete or corrupt. The trap mechanism is that the software reports success. The harm is a false sense of security until a restore is attempted in crisis. The corrective is regular restore testing.

### No Offsite Copy, So Local Catastrophe Means Total Loss

Backups on a drive next to the computer survive drive failure but not theft or fire. The trap mechanism is that local backup feels sufficient. The harm is total loss in a catastrophe that takes the location. The corrective is an offsite copy that is current and complete.

## Self-Check

- Are there at least 3 copies of all creative work, on 2 media types, with 1 offsite (the 3-2-1 rule)?
- Are backups automated, not dependent on manual memory, and verified to be running?
- Are versions saved incrementally, allowing return to any prior creative state?
- Are projects self-contained, with backups capturing the entire project folder including all assets?
- Has a restore from backup been tested recently, confirming sessions open intact?
- If the local location were destroyed, would recent creative work be recoverable from the offsite copy?
- Could you, right now, recover the current project from backup and continue working within an hour?
