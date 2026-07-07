---
name: container_storage_and_stateful_workloads.md
description: Use when the agent is running a stateful workload in containers, choosing between an emptyDir, hostPath, configMap/secret volume, or a persistent volume, deciding between a Deployment and a StatefulSet, configuring a PersistentVolumeClaim and a StorageClass with the right access mode and reclaim policy, placing replicas with pod anti-affinity or node/topology spread for data locality, designing backup and restore for persistent data, migrating a stateful workload between storage backends or clusters, or debugging data that disappeared after a pod restart. Use when the agent is storing a database, message queue, cache with disk persistence, or application state in containers, reasoning about where state actually lives, or reviewing a manifest that writes to the container filesystem. Risk surfaces include data loss on pod recreation, split-brain from multi-attachable volumes claimed by two writers, silent corruption from forced detach, backups that were never tested, and the assumption that a volume is a substitute for designing the workload for its state.
---

# Container Storage And Stateful Workloads

Containers are designed to be ephemeral: a container is created, runs, and is destroyed, and a correct container can be killed and recreated at any time without losing anything that matters. That property is what makes containers safe to scale, update, and reschedule — but it is also what makes stateful workloads hard. The moment a workload needs data to survive the container's death, the storage must live *outside* the container's writable layer, and the orchestration must place, attach, and reattach that storage correctly. Most data-loss incidents in containerized systems are not exotic failures; they are the predictable consequence of writing state somewhere that was never meant to be durable, or of treating a persistent volume as if it were just a fast disk attached to a generic pod.

Agents tend to reproduce the "just mount a volume" reflex they learned from VMs, and stop there. That reflex hides a series of decisions that each have a wrong default: storing state in the container's writable layer (lost the instant the pod is recreated); choosing a Deployment for a workload that needs stable identity and ordered, non-concurrent storage access; selecting a volume access mode that the backend does not actually support or that permits two writers to corrupt data; assuming a volume's data is backed up because a snapshot schedule exists, without ever testing a restore; and migrating stateful data by changing the manifest and hoping the new backend has the bytes. The judgment problem is to treat state as a first-class concern — decide where it lives, how it is named and attached, how it survives failure and migration, and how it is recovered — rather than as a side effect of mounting a volume.

This skill covers persistent storage and stateful workload design. It is distinct from `orchestration-and-scaling`, which covers replica counts, rolling updates, autoscaling, and graceful shutdown for workloads generally; here the focus is the *state* — volumes, PVCs, StorageClasses, StatefulSet semantics, data locality, backup, and migration. The image and runtime concerns belong to the image and runtime skills.

## Core Rules

### Never Store Durable State In The Container's Writable Layer

Every container has a writable layer on top of its read-only image layers, and anything written there — logs, database files, uploaded assets, cache dumps — exists only for the lifetime of that container. When the pod is rescheduled, evicted, OOM-killed, or rolled, the new container starts from the image with an empty writable layer, and the data is gone. This is not a rare edge case; it is the normal lifecycle of a container. The rule is absolute: if the data must survive the container, it must live in a *volume*, not in the container filesystem.

The practical discipline is to identify every path the application writes to and classify each: is it truly ephemeral (a `/tmp` scratch that can be lost), or must it persist (a database directory, a queue's durable store, user uploads)? Ephemeral writes can use an `emptyDir` (which at least survives container restarts within a pod's life, though not pod deletion); durable writes must use a persistent volume. A read-only root filesystem (covered in the runtime skill) makes this classification enforceable rather than aspirational — when the rootfs is read-only, any write that is not to a mounted volume fails loudly, so a misclassified path is caught immediately instead of silently lost later.

### Match The Volume Type To The Data's Lifetime And Sharing Needs

Kubernetes and Docker offer several volume types, and they differ in lifetime, sharing, and backing. Choosing the wrong one is a common source of data loss and corruption:

- **`emptyDir`** — lives with the pod, deleted when the pod is removed. Good for scratch space, shared buffers between a container and its sidecar, or a disk-backed cache you accept losing. Never use for data that must survive rescheduling.
- **`configMap` / `secret`** — read-only (or projected) configuration injected by the control plane; not for application-generated state. Use for config files, TLS certs, and credentials.
- **`hostPath`** — mounts a path from the node's filesystem into the pod. Coupling to a specific node breaks rescheduling, is a security risk (it can expose host files), and should be avoided in production except for system daemons that genuinely need node-local access.
- **Persistent volumes (PV/PVC)** — the durable, cluster-managed option, backed by networked storage (EBS, GCE PD, Azure Disk, NFS, Ceph, or a CSI driver) or a local persistent volume. This is the correct choice for any data that must survive pod recreation.

Decide based on lifetime (per-pod vs durable), sharing (single-writer vs read-many), and performance (networked vs local). For stateful services, the answer is almost always a persistent volume provisioned via a StorageClass.

### Use A StatefulSet When The Workload Needs Stable Identity Or Ordered Storage

A Deployment creates replicas that are interchangeable: they have random names, can be created and destroyed in any order, and share nothing by default. That is correct for stateless services and wrong for many stateful ones. A database, a distributed store with a leader/follower topology, or any workload that needs a stable network identity, a stable persistent volume per replica, and ordered, non-concurrent startup and teardown requires a *StatefulSet*. The StatefulSet gives each pod a stable name (`db-0`, `db-1`), a stable DNS name, and — critically — a persistent volume claim that is bound to that specific ordinal and follows it across rescheduling, so `db-0` always gets the same disk back.

The decision criteria: if the workload needs each replica to own a distinct, durable piece of state (a shard, a log segment, a unique data directory), or needs stable identity for peer discovery (cluster nodes that address each other by stable name), use a StatefulSet. If replicas are truly interchangeable and state is shared externally (a cache backed by Redis, a stateless API in front of a managed database), a Deployment is fine. Do not use a StatefulSet merely because the workload is "important"; use it because the workload's state semantics require stable, per-replica storage and ordering. And do not run multiple writers against a single `ReadWriteOnce` volume — that access mode permits only one node to mount for writes, and forcing multi-attach corrupts data or fails to attach.

### Choose The Access Mode And StorageClass For What The Backend Actually Supports

A persistent volume's access mode (`ReadWriteOnce`, `ReadOnlyMany`, `ReadWriteMany`) is a contract between what you request and what the backing storage can do, and mismatches cause silent failures or stuck pods. `ReadWriteOnce` (RWO) restricts the volume to a single node for writes — correct for block storage like EBS or GCE PD, and the safe default for single-writer databases. `ReadWriteMany` (RWX) allows many nodes to mount for writes simultaneously and requires a shared filesystem (NFS, CephFS, AWS EFS, Azure Files); it is necessary for workloads that truly share writable state, but it introduces the complexity and performance characteristics of networked shared storage. Requesting RWX against a backend that only provides RWO means the PVC never binds.

The StorageClass defines *how* volumes are provisioned (the driver, the disk type, IOPS tier, encryption, and the reclaim policy). Match the class to the workload's performance and retention needs: a database wants a fast, low-latency block device with a `Retain` reclaim policy so the data survives PVC deletion; a scratch cache may accept a slower tier with `Delete`. The reclaim policy is a particularly high-stakes setting — `Delete` destroys the volume and its data when the PVC is removed, which is correct for ephemeral data and catastrophic for a database if a namespace is torn down carelessly. Default to `Retain` for anything durable, and protect stateful namespaces with the same care you would give the underlying database.

### Place Replicas With Data Locality And Anti-Affinity In Mind

Where a stateful pod runs affects performance, availability, and cost. A pod that mounts a networked volume can, in principle, run on any node, but a pod that benefits from locality — a local persistent volume, or a cache that warms from a co-located data node — should be placed near its data using node affinity, pod anti-affinity, or topology spread constraints. Anti-affinity is also how you prevent all replicas of a stateful set from landing on one node and failing together: spread them across zones and hosts so a single failure does not take down the whole quorum.

The trap is over-constraining placement so that the scheduler cannot find a valid node, leaving pods Pending indefinitely, or under-constraining so that correlated failures are possible. Define anti-affinity against real failure domains (hosts, and preferably zones for anything that needs high availability), and verify the cluster has enough of those domains for the desired replica count. For workloads with local storage, accept that rescheduling is constrained to nodes that hold a copy of the data, and design replication so that the loss of one node does not mean the loss of the only copy.

### Back Up Persistent Data And Rehearse The Restore

A persistent volume is not a backup. It is the primary copy of the data, and it can be lost to storage backend failure, region outage, accidental deletion, corruption, or ransomware. A real backup strategy captures point-in-time, restorable copies *separate* from the primary volume — via volume snapshots (CSI `VolumeSnapshot`), storage-backend-level snapshots (EBS snapshots, disk images), or application-consistent exports (a database dump, a WAL archive) — and stores them with a retention policy and, for disaster recovery, in a separate location or region.

The discipline that separates a real backup from a false one is *testing the restore*. A snapshot schedule that has never been restored from is an unverified assumption: the snapshots may be corrupt, the restore procedure may be wrong, or the recovery time may be hours longer than the business can tolerate. Rehearse restores into a non-production environment on a cadence, measure the recovery point and time objectives actually achieved, and document the runbook. For stateful workloads, also plan the migration path: moving between storage backends, CSI drivers, or clusters usually requires a controlled copy (snapshot-and-restore, or an application-level replication) with a cutover, not a manifest change and a hope.

## Common Traps

### Storing State In The Container's Writable Layer

Writing database files, uploads, or logs to a path inside the container and losing everything when the pod is recreated or rescheduled. The container's writable layer is ephemeral by design; durable data must live in a mounted volume, and a read-only root filesystem makes misclassified writes fail loudly instead of silently disappearing.

### Using A Deployment For A Workload That Needs Per-Replica Storage

Running a database or distributed store as a Deployment with a shared PVC, so replicas race on the same volume or lose their identity on rescheduling. Workloads needing stable per-replica storage, stable network identity, or ordered startup use a StatefulSet; interchangeable replicas in front of external state use a Deployment.

### Requesting An Access Mode The Backend Does Not Support

Asking for `ReadWriteMany` against block storage that only offers `ReadWriteOnce`, leaving the PVC Pending forever, or forcing two writers onto an RWO volume and corrupting data. Match the access mode to what the backing storage actually provides; RWO is the safe default for single-writer databases.

### A Reclaim Policy Of Delete On Durable Data

Provisioning a database volume with `Delete` so that removing the PVC (or tearing down the namespace) destroys the data. Use `Retain` for anything durable, and treat stateful namespaces with the same care as the underlying database; protect them from casual teardown.

### Assuming A Volume Is A Backup

Treating the persistent volume as the backup because it is "persistent," with no separate point-in-time copy, so a backend failure, region outage, or accidental deletion loses the only copy. A backup is a restorable copy separate from the primary; capture snapshots or exports and rehearse restoring them.

### Untested Backups That Cannot Actually Restore

Running a snapshot schedule for years and never restoring from it, so the team discovers under incident pressure that the snapshots are corrupt, the procedure is wrong, or recovery takes far longer than the objective. Rehearse restores into a non-production environment on a cadence and measure the real RPO/RTO.

### Migrating Stateful Data By Changing The Manifest

Pointing a workload at a new storage backend or cluster by editing the manifest and expecting the data to follow, when in fact the new PVC is empty. Stateful migration requires a controlled copy (snapshot-and-restore, application-level replication) with a planned cutover, not a configuration change and a hope.

### Over-Constrained Or Under-Constrained Placement

Spreading stateful replicas so loosely that a single node or zone failure takes down the quorum, or so tightly (or with local storage) that the scheduler cannot place pods and they stay Pending. Define anti-affinity against real failure domains and verify the cluster has enough domains for the replica count.

## Self-Check

- [ ] No durable state is written to the container's writable layer; every persistent write path is classified and backed by a mounted volume, and (ideally) a read-only root filesystem makes misclassified writes fail loudly.
- [ ] The volume type matches the data's lifetime and sharing needs — `emptyDir` only for pod-scoped scratch, `configMap`/`secret` for injected config, `hostPath` avoided in production, and persistent volumes for anything that must survive pod recreation.
- [ ] A StatefulSet is used when the workload needs stable per-replica storage, stable network identity, or ordered startup/teardown; a Deployment is used only when replicas are truly interchangeable with externalized state.
- [ ] The access mode (`RWO`/`ROX`/`RWX`) matches what the backing storage actually supports, and no workload forces multiple writers onto a single `ReadWriteOnce` volume.
- [ ] The StorageClass and reclaim policy are chosen for the workload: fast block storage with `Retain` for durable data like databases, and `Delete` reserved for genuinely ephemeral data; stateful namespaces are protected from casual teardown.
- [ ] Replicas are placed with anti-affinity or topology spread against real failure domains (hosts, zones), the cluster has enough domains for the replica count, and pods are not stuck Pending from over-constraint or co-located for correlated failure from under-constraint.
- [ ] Persistent data has a real backup — point-in-time snapshots or application-consistent exports, stored separately from the primary, with a retention policy and (for DR) a separate location or region.
- [ ] Backups have been restored in rehearsal into a non-production environment, and the real recovery point objective and recovery time objective have been measured, not just assumed.
- [ ] Any planned migration between storage backends, CSI drivers, or clusters has a defined copy-and-cutover procedure (snapshot restore or application replication), rather than a manifest edit that would leave the new PVC empty.
