---
name: preservation_actions_in_repositories.md
description: Use when the agent is planning or performing preservation actions within a digital repository, including fixity checking, format characterization and migration, redundancy and replication, PREMIS event documentation, integrity monitoring, defining preservation policies and action plans, or responding to fixity failures and format obsolescence risk in an institutional repository or digital archive.
---

# Preservation Actions In Repositories

Preservation is the set of actions that keep digital content authentic, understandable, and usable over time, and it is the core obligation that distinguishes a repository from storage. The danger is that preservation is easy to defer: content does not visibly decay, fixity checks run silently, and format obsolescence arrives slowly and unevenly. Institutions routinely build repositories that store content securely today while having no operational plan for what happens when a fixity check fails, when a format becomes unreadable, or when a storage layer degrades. Agents tend to treat preservation as a feature to enable rather than a set of recurring, staffed, decision-bearing actions, and they confuse having a preservation tool with performing preservation. A repository that cannot demonstrate, with evidence, what preservation actions it has taken and why is not a trusted repository regardless of how much content it holds.

Use this skill when planning preservation actions, writing preservation policy, configuring fixity and characterization, responding to integrity failures, or addressing format obsolescence. The goal is to prevent the agent from treating preservation as configuration rather than operations, from enabling checks without planning responses, or from performing actions without the documentation that makes them auditable and trustworthy.

## Core Rules

### Distinguish Bit-Level From Functional Preservation And Commit To Both Explicitly

Preservation is not one activity but a layered set of commitments, and confusing the layers produces false confidence. Bit-level preservation ensures the exact bytes deposited are the exact bytes retrieved, through fixity, redundancy, and geographic replication. Functional preservation ensures the content remains understandable and usable, through format characterization, migration, and documentation. A repository can be excellent at bit-level preservation and still lose access to content when formats become obsolete, because it never planned the functional layer.

Define and commit to a preservation level for each content class:

- bit-level only, secure storage with fixity monitoring, appropriate for content where format migration is not feasible or not required;
- functional preservation, with format characterization and a migration plan, required for content the institution commits to keeping usable;
- full preservation with emulation readiness, for high-value complex objects where migration may not suffice.

Make the commitment explicit in policy. Promising functional preservation while only performing bit-level work is a failure that surfaces only when it is too late to recover.

### Make Fixity Checking Operational, Not Cosmetic

Fixity checking, comparing checksums to detect bit-level change, is the heartbeat of bit-level preservation. The trap is enabling fixity checks and assuming preservation is done. Fixity checking only has value if failures are detected promptly, surfaced to a human, investigated, and resolved, and if the results are recorded as evidence.

Operational fixity requires:

- a defined checking frequency appropriate to the content's value and risk, with high-value content checked more often;
- automated scheduling that does not depend on a person remembering;
- alerting that routes failures to a named owner with a response procedure, not to a log no one reads;
- a defined response to failure: identify the affected object, restore from a verified replica, investigate the cause, and record the event;
- redundancy sufficient that a fixity failure in one copy can be repaired from another, which is the entire point of replication;
- recording every check and every response as a PREMIS event so the preservation history is auditable.

A fixity check that runs and is never reviewed is theater. The operational loop, check, alert, respond, record, is what preserves content.

### Characterize Formats And Act On Obsolescence Risk

Format characterization identifies the file formats in the repository and assesses their preservation risk. It is the foundation of functional preservation because you cannot plan migration for formats you have not identified or assessed. Characterization must be paired with a process that turns the intelligence into action.

Format preservation requires:

- running characterization tools such as DROID, JHOVE, or Siegfried across the repository to identify formats, versions, and validity;
- mapping identified formats to a risk registry such as PRONOM or the NARA format registry to assess obsolescence risk;
- defining normalisation and migration triggers, such as when a format's risk crosses a threshold or when a viewer becomes unavailable;
- planning migration paths for at-risk formats, including target formats chosen for stability and openness;
- documenting every migration as a PREMIS event with the source and target format, the tool used, and any quality verification;
- retaining original files alongside normalised copies when feasible, because migration is lossy and reversible preservation is safer.

Characterization without action is reporting. The value is in the migration decisions it informs.

### Build Redundancy And Replication As A Recovery Strategy

Redundancy is not about having more copies; it is about being able to recover when a copy fails. The design of redundancy should be driven by recovery scenarios, not by a copy count target. Three copies on the same failing storage array provide no recovery; geographically separated copies on independent infrastructure do.

Design redundancy around recovery:

- maintain multiple copies, with the number justified by content value and threat model;
- ensure geographic separation so that a local disaster does not destroy all copies;
- use independent storage infrastructure, not copies on the same hardware or provider, to avoid correlated failures;
- verify each copy independently with fixity checks, because an unverified copy is not a trustworthy recovery source;
- define and test recovery procedures, because untested recovery is an assumption;
- consider the "dark archive" pattern for a geographically remote, offline copy protecting against catastrophic loss.

The test of redundancy is not how many copies exist but whether content can be restored after a realistic failure.

### Document Preservation Actions As Evidence

Preservation is only trustworthy if it is demonstrable. Every preservation action, fixity check, replication, characterization, migration, and response to failure, should be recorded as structured evidence using PREMIS or an equivalent model. This evidence is what a certification audit, a researcher, or a future custodian will examine to confirm that the repository has honored its preservation commitment.

PREMIS event documentation should record:

- the event type, such as fixity check, replication, migration, or validation;
- the object or objects affected;
- the timestamp and the agent, human or software, that performed the action;
- the outcome, success or failure, and any detail;
- the tool or method used, with version, so actions are reproducible;
- any linkage to related events, such as a migration triggered by a characterization result.

Preservation without documentation is indistinguishable from neglect. The documentation is the deliverable.

### Write A Preservation Policy That Guides Decisions

A preservation policy is not a marketing statement; it is the decision framework that tells staff what to do when preservation choices arise. A policy that says "we preserve content" guides no one. A useful policy specifies the preservation levels, the formats accepted and the migration commitments, the fixity and replication regime, the response to failures, and the review cycle.

A preservation action plan operationalizes the policy: it lists the content classes, their preservation level, the scheduled actions, the responsible roles, and the tools. The action plan is what staff execute; the policy is what they justify decisions against. Both should be reviewed regularly because formats, tools, and risks evolve.

### Plan For The Preservation Workforce, Not Just The Tools

Preservation actions require people who understand the tools, the formats, and the decisions, and who have the authority and time to act. The most common preservation failure is not a missing tool but a missing owner: fixity alerts go to a queue no one monitors, characterization reports sit unread, and migrations are deferred until formats are already obsolete. Preservation must be staffed as a recurring operational responsibility with named owners, scheduled time, and escalation paths, not as a project that completes.

## Common Traps

### Confusing Storage With Preservation

Storing content on reliable infrastructure feels like preservation but provides no format obsolescence protection or documented evidence. This is a trap because the content is safe today but the institution has no plan or record for tomorrow.

### Enabling Fixity Checks Without A Response Plan

Running fixity checks that no one reviews or acts on provides false assurance. This is a trap because the check produces evidence of integrity that is never verified or used, and failures go unnoticed until content is needed.

### Characterizing Formats But Never Migrating

Producing format reports that identify at-risk content but never triggering migration lets obsolescence arrive unaddressed. This is a trap because the intelligence exists but the action does not, and the window for orderly migration closes.

### Counting Copies Instead Of Designing Recovery

Pursuing a copy count target without considering geographic and infrastructural independence provides redundant storage that fails together. This is a trap because the redundancy looks sufficient but cannot recover from correlated failures.

### Performing Actions Without PREMIS Documentation

Doing preservation work without recording it as events leaves no auditable evidence. This is a trap because the work happened but cannot be demonstrated, undermining trust and certification.

### Promising Functional Preservation While Doing Only Bit-Level

Committing to keep content usable while only securing the bytes sets up a failure when formats become obsolete. This is a trap because the gap between promise and practice is invisible until access is lost.

### Letting Preservation Depend On One Person

Concentrating preservation knowledge and responsibility in a single staff member means turnover halts preservation. This is a trap because the risk is latent and the recovery cost is high.

### Deferring Migration Until Formats Are Already Obsolete

Waiting until a format is unreadable before planning migration forces emergency, lossy, or impossible conversions. This is a trap because orderly, tested migration ahead of obsolescence is far cheaper and safer.

## Self-Check

- Have you defined an explicit preservation level (bit-level, functional, emulation-ready) for each content class, matched to the institution's commitment?
- Is fixity checking scheduled, automated, alerting to a named owner, and paired with a documented response and recovery procedure?
- Are formats characterized with tools like DROID, JHOVE, or Siegfried, mapped to a risk registry, and paired with defined migration triggers?
- Is redundancy designed around recovery scenarios, with geographic and infrastructural independence and tested restore procedures?
- Is every preservation action recorded as a PREMIS event with type, object, timestamp, agent, outcome, and tool, forming an auditable history?
- Does a written preservation policy specify levels, formats, fixity, replication, failure response, and review, distinct from an operational action plan?
- Are preservation responsibilities assigned to named owners with scheduled time and escalation paths, not treated as a completed project?
- Are original files retained alongside normalised copies so migrations are reversible?
- Is there a scheduled review of the preservation policy and action plan against evolving format risk and tool availability?
