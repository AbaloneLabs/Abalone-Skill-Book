---
name: storage_and_redundancy.md
description: Use when the agent is designing digital preservation storage, planning redundancy and geographic distribution, choosing storage media or services, managing backup versus preservation copies, or evaluating storage risk and recovery.
---

# Storage And Redundancy

Storage is where preservation either holds or fails. Digital content is fragile in a way physical books are not: a single disk failure, a corrupted array, a ransomware attack, or a single-site disaster can destroy years of work irrecoverably. Preservation storage is not the same as backup, and redundancy is not the same as having two copies in the same building. Good storage strategy follows the principle of multiple independent copies, distributes risk geographically and across media or services, monitors for failure, and tests recovery before it is needed. Storage decisions made casually at the start of a project are extremely hard to fix after data is lost.

Use this skill when designing preservation storage, planning redundancy and geographic distribution, choosing storage media or cloud services, or establishing backup and recovery practice. The goal is to prevent the agent from relying on a single copy or site, confusing backup with preservation, choosing storage by price alone, or failing to test recovery until a loss occurs.

## Core Rules

### Follow The LOCKSS Principle: Lots Of Copies Keep Stuff Safe

The foundational principle of preservation storage is redundancy through multiple independent copies. A single copy, no matter how good the medium, is a single point of failure.

Apply redundancy:

- maintain at least three copies of preservation content;
- store copies on different media or services where feasible;
- distribute copies geographically to survive site disasters;
- keep at least one copy under separate administrative control;
- ensure copies are independent, so a corruption in one does not propagate.

Three copies in three locations is a common baseline; high-value content may warrant more. Two copies in one building is not adequate redundancy.

### Distinguish Preservation Storage From Backup And Access Storage

Preservation storage, backup, and access storage serve different purposes and have different requirements. Conflating them compromises preservation.

The distinctions:

- preservation storage holds the authoritative masters, immutable, fixity-checked, long-term;
- backup protects against operational loss, is overwritten on a cycle, and may not retain history;
- access storage serves delivery, is optimized for speed, and may be compressed or cached.

Preservation copies must be separate from access systems so that a compromise or corruption in access does not affect the preservation master. Backup alone is not preservation because it cycles and may not retain deleted or changed content.

### Distribute Risk Geographically

A single geographic location is vulnerable to fire, flood, power loss, and regional disasters. Geographic distribution of copies is essential.

Distribution principles:

- store at least one copy in a different building, ideally a different region;
- consider seismic, climatic, and political stability of locations;
- use different network paths and providers to avoid correlated failures;
- for cloud, understand the regions and availability zones used;
- ensure a regional disaster does not take out all copies.

Two copies in the same data center, even on different arrays, do not survive a data center loss. Geographic separation is non-negotiable for irreplaceable content.

### Use Diverse Media And Services

Different storage media and services have different failure modes. Diversity reduces the chance that a single class of failure affects all copies.

Diversity options:

- mix on-premises disk or tape with cloud storage;
- use different cloud providers for different copies;
- combine spinning disk with archival tape or object storage;
- avoid relying solely on one vendor's ecosystem.

Diversity adds complexity, so balance it against manageability. The goal is that no single failure mode, media decay, vendor outage, or format issue, affects all copies.

### Choose Storage By Longevity, Integrity, And Cost, Not Price Alone

Storage cost is real, but choosing the cheapest option often means choosing higher risk. Evaluate storage on multiple dimensions.

Evaluate:

- bit error rate and data integrity features;
- media longevity and degradation characteristics;
- vendor stability and exit options;
- data portability and lock-in risk;
- support for fixity checking and audit logs;
- total cost including egress, API calls, and management;
- scalability as the collection grows.

Cheap storage that loses data or traps content is expensive in the long run. Factor in the cost of loss, which for unique content is total.

### Implement The 3-2-1 Rule Or Stronger

The 3-2-1 rule is a widely used storage baseline: three copies, on two different media, with at least one offsite. For preservation, consider exceeding it.

Baseline and enhancements:

- three copies minimum;
- two different media types, disk and tape, or disk and cloud;
- one copy offsite, geographically separated;
- for high-value content, add a fourth copy or an immutable, write-once copy;
- consider a dark archive copy that is offline or air-gapped against ransomware.

Match the redundancy level to the value and irreplaceability of the content. Born-digital unique content warrants the strongest protection.

### Make Preservation Copies Immutable Or Append-Only

Preservation masters should not be modifiable accidentally or maliciously. Immutability protects against corruption, ransomware, and operator error.

Implement immutability:

- use write-once-read-many, WORM, storage or object lock features;
- restrict write access to controlled preservation workflows;
- log all access and any legitimate modifications as events;
- maintain version history so changes are recoverable;
- use checksums to detect any unauthorized change.

Ransomware that encrypts preservation storage is a real and growing threat. Immutable or air-gapped copies are the defense.

### Monitor Storage Health Continuously

Storage media fail, silently and over time. Without monitoring, failures accumulate until recovery is impossible.

Monitor:

- disk and array health metrics, SMART data, error rates;
- tape readability and migration schedules;
- cloud storage integrity and access logs;
- fixity checks on all copies on a regular schedule;
- capacity and performance trends;
- vendor service notifications and end-of-life announcements.

Monitoring is an ongoing operational duty. A storage system that is set up and never checked is decaying invisibly.

### Test Recovery Before You Need It

An untested backup or copy is an assumption, not a safeguard. Recovery must be practiced so that when loss occurs, restoration works.

Test recovery:

- periodically restore files from each copy and verify integrity;
- test full recovery workflows, not just single-file restores;
- measure recovery time and identify bottlenecks;
- document recovery procedures and train staff;
- run disaster recovery exercises including loss of a site.

Recovery testing reveals problems, corrupted copies, missing metadata, broken procedures, while there is still time to fix them.

### Plan For Storage Migration And Growth

Storage media and services have finite lifetimes. Tapes degrade, disks reach end of life, and cloud services change. Plan for migration as a normal activity.

Plan:

- media refresh schedules based on manufacturer and observed lifetimes;
- cloud service migration paths if a vendor changes terms or shuts down;
- capacity planning as the collection grows;
- budget for storage migration as a recurring cost;
- documentation so migration is repeatable.

Storage is not permanent. Treat it as a lifecycle component that requires scheduled renewal.

## Common Traps

### Single Copy Or Single Site

One copy or one location is a single point of failure. Maintain multiple geographically distributed copies.

### Confusing Backup With Preservation

Backup cycles and may not retain history. Preservation storage is immutable and authoritative.

### Same Media And Same Provider For All Copies

A single failure mode can affect all copies. Diversify media and services.

### Choosing Storage By Price Alone

Cheap storage that loses data or locks content is costly overall. Evaluate integrity, longevity, and portability.

### No Immutability Or Air-Gap

Ransomware and operator error can modify or delete copies. Use immutable or offline copies for critical content.

### Set And Forget Storage

Media fail silently. Continuous monitoring and fixity checking are required.

### Untested Recovery

An untested backup is an assumption. Practice recovery before loss occurs.

### No Migration Plan

Storage media and services have finite lives. Plan and budget for refresh and migration.

## Self-Check

- Are there at least three independent copies of preservation content?
- Are preservation, backup, and access storage clearly separated in purpose and system?
- Is at least one copy geographically distributed to survive a site disaster?
- Are copies stored on diverse media or services to avoid correlated failures?
- Is storage chosen on integrity, longevity, portability, and total cost, not price alone?
- Does the strategy meet or exceed the 3-2-1 rule, with immutability for high-value content?
- Are preservation copies immutable or append-only, with access logged and checksummed?
- Is storage health monitored continuously, including fixity checks on all copies?
- Is recovery tested periodically with full workflows, not just single-file restores?
- Is there a documented plan and budget for storage migration and capacity growth?
