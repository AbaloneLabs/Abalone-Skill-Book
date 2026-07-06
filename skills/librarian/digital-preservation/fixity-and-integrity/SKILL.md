---
name: fixity_and_integrity.md
description: Use when the agent is establishing fixity checking for digital preservation, choosing checksum algorithms, scheduling integrity audits, responding to fixity failures, or designing monitoring that detects silent corruption.
---

# Fixity And Integrity

Fixity is the proof that a digital file has not changed. Without it, an institution cannot know whether its preserved content is intact, corrupted, or silently altered. Digital files change for many reasons: storage media decay, bit rot, software errors, migration mistakes, malicious tampering, or accidental overwrites. Most of these changes are invisible until someone tries to open the file and finds it broken, by which point recovery may be impossible. Fixity checking is the ongoing process that detects change early, while restoration from intact copies is still possible. It is not a one-time computation but a sustained monitoring commitment, and designing it well, choosing algorithms, scheduling checks, and planning responses, determines whether preservation is real or aspirational.

Use this skill when designing fixity checking, choosing checksum algorithms, scheduling integrity audits, defining response procedures, or reviewing monitoring effectiveness. The goal is to prevent the agent from computing checksums once and never checking them, choosing weak algorithms, failing to log and respond to failures, or treating fixity as a checkbox rather than a detection system.

## Core Rules

### Understand Why Fixity Matters

Fixity checking exists to detect unwanted change in digital content over time. Without it, corruption accumulates silently and is discovered too late.

Changes fixity detects:

- bit rot from degrading storage media;
- silent corruption from controller or filesystem errors;
- unintended modification during migration or processing;
- accidental overwrites or deletions;
- malicious tampering or ransomware encryption;
- incomplete or failed transfers between systems.

The purpose is early detection. A corruption found while intact copies exist is recoverable; one found after all copies are affected is a loss.

### Choose A Cryptographically Strong Algorithm

The checksum algorithm determines how reliably changes are detected. Weak algorithms can miss certain corruptions; strong ones make missed detection astronomically unlikely.

Algorithm guidance:

- use SHA-256 or stronger for preservation fixity;
- MD5 and SHA-1 are acceptable for some non-security uses but vulnerable to collisions;
- avoid CRC32 or simple checksums for preservation, they detect only some changes;
- use the same algorithm consistently across the collection;
- document the algorithm in technical metadata.

Collision resistance matters because fixity must detect not just random corruption but potential intentional alteration. SHA-256 is the current practical standard.

### Generate Checksums At Ingest And Store Them Separately

Checksums must be generated when content enters preservation, capturing the file's initial state, and stored where they cannot be altered along with the file.

Practice:

- compute the checksum immediately upon ingest, before any processing;
- store checksums in administrative or preservation metadata;
- keep a copy of checksums separate from the content they protect;
- consider an additional offline or immutable copy of checksums;
- record the algorithm and the date of computation.

Checksums stored alongside content can be corrupted or tampered with together. Separation preserves their value as an independent reference.

### Schedule Regular Fixity Checks

A checksum computed once is useless unless it is checked against the file repeatedly over time. The schedule determines how quickly corruption is detected.

Scheduling considerations:

- check all content on a regular cycle, annually is common, more often for high-value;
- check recently ingested content soon after ingest to catch transfer errors;
- check content after any migration, move, or processing event;
- prioritize high-value or high-risk content for more frequent checks;
- balance check frequency against storage and compute cost.

Document the schedule and track completion. A schedule that is not followed provides false assurance.

### Log Every Fixity Check As An Event

Each fixity check is a preservation event that should be recorded. The log is the evidence that monitoring is actually happening and the record needed to investigate failures.

Log for each check:

- the object checked;
- the date and time;
- the outcome, match or mismatch;
- the algorithm and stored checksum used;
- the agent, person or software, performing the check;
- any action taken in response.

PREMIS provides a structure for event logging. The log itself should be preserved and protected from alteration.

### Respond To Fixity Failures Immediately And Systematically

A fixity failure means the file has changed and is no longer trustworthy. The response must be immediate, systematic, and documented, not ad hoc.

Response procedure:

- quarantine the affected file to prevent use of the corrupted version;
- investigate the cause, media failure, transfer error, tampering;
- restore from an intact copy, verifying its fixity first;
- log the failure, investigation, and restoration as preservation events;
- check whether the failure indicates a broader problem, failing media or array;
- review and strengthen storage or monitoring if a systemic cause is found.

A failure that is noticed but not acted on is worse than no monitoring, because it signals a broken process. Build the response into the workflow.

### Verify Fixity After Any Transformation

Any transformation, migration, normalization, move, or copy, introduces risk of change. Fixity must be verified before and after such events.

Verification around transformations:

- verify the source file's fixity before transformation;
- compute and record fixity for the transformed output;
- verify that the transformation produced the intended result, significant properties preserved;
- log the transformation as a preservation event with both checksums;
- retain the original until the transformed version is verified.

Treating a transformation as complete without fixity verification risks propagating errors into the preservation copy.

### Monitor At Scale With Automation

Manual fixity checking does not scale to real collections. Automation is required to check thousands or millions of files on schedule.

Automation requirements:

- tools that compute and compare checksums in batch;
- scheduling that runs checks without manual intervention;
- alerting that notifies staff of failures promptly;
- reporting that tracks check completion and outstanding failures;
- integration with the preservation metadata and event log.

Choose or build tools that fit the repository scale. A monitoring system that depends on someone remembering to run it will eventually be forgotten.

### Distinguish Fixity From Authenticity And Significance

Fixity proves a file has not changed, but it does not prove the file is the right one, authentic, or meaningful. These are separate concerns.

Clarify:

- fixity, the bitstream is unchanged since the checksum was computed;
- authenticity, the object is what it claims to be, provenance and identity;
- significance, the object preserves the important properties of the original.

Fixity is necessary but not sufficient for preservation. Pair it with provenance metadata, significant property definitions, and authenticity documentation.

### Review And Improve Monitoring Over Time

Fixity monitoring is not set-and-forget. As collections grow, storage changes, and threats evolve, the monitoring must be reviewed and improved.

Review periodically:

- is the check schedule being met;
- are failures being detected and responded to;
- is the algorithm still considered adequate;
- are new content types being checked;
- are new threats, ransomware, addressed by the monitoring;
- are tools keeping up with collection scale.

A monitoring system designed five years ago may not meet current needs. Schedule review as a preservation responsibility.

## Common Traps

### Checksums Computed Once And Never Checked

A checksum is only useful if compared repeatedly. Schedule and perform regular checks.

### Weak Algorithms For Preservation

MD5 and CRC cannot be trusted against intentional alteration or all corruption. Use SHA-256 or stronger.

### Checksums Stored Only With Content

Checksums corrupted alongside content cannot detect the corruption. Store them separately and immutably.

### No Response Procedure For Failures

Detecting corruption without a restoration process is useless. Build and document the response.

### No Logging Of Checks

Without event logs, there is no evidence monitoring happened. Log every check and outcome.

### Treating Transformation As Complete Without Verification

Migrations and moves can introduce errors. Verify fixity before and after any transformation.

### Manual Checking At Scale

Manual processes are forgotten and do not scale. Automate scheduling, checking, and alerting.

### Confusing Fixity With Authenticity

An unchanged file may still be the wrong file. Pair fixity with provenance and significance metadata.

## Self-Check

- Is the purpose of fixity, early detection of unwanted change, understood and communicated?
- Is a cryptographically strong algorithm, SHA-256 or better, used consistently?
- Are checksums generated at ingest and stored separately from the content they protect?
- Is there a documented schedule for regular fixity checks, with completion tracked?
- Is every fixity check logged as a preservation event with outcome and agent?
- Is there an immediate, systematic response procedure for fixity failures, including restoration from intact copies?
- Is fixity verified before and after every migration, normalization, or move?
- Is monitoring automated with scheduling, alerting, and reporting at collection scale?
- Is fixity distinguished from authenticity and significance, with all three addressed?
- Is the monitoring system reviewed and improved periodically as collections and threats evolve?
