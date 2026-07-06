---
name: offline_and_sync.md
description: Use when the agent is designing an offline-first or occasionally-connected mobile application, choosing a local storage strategy and sync model, handling conflicts when the same data is edited offline and online, designing background sync, or dealing with partial connectivity and intermittent networks. Also covers the failure mode of assuming connectivity, losing offline work on sync, last-write-wins silently discarding concurrent edits, sync loops or duplicate writes, and the gap between a local-first UI and a correct reconciliation with the server.
---

# Offline And Sync

Offline-first means the app works against local data and reconciles with the server when connectivity allows, rather than assuming the network is always present. The judgment problem is that connectivity on mobile is not binary (online vs. offline) but a spectrum — intermittent, slow, partial, captive — and the hard problems live in reconciliation, not in caching. When the same record is edited on two devices, or edited offline and changed on the server, the sync must reconcile, and the reconciliation strategy (last-write-wins, merge, manual resolution) determines whether user work is preserved or silently lost. The discipline is to make the local store the source of truth for the UI, to sync as a background reconciliation rather than a blocking operation, to detect and resolve conflicts deliberately (with a strategy matched to the data), to make sync idempotent and resumable so that interruptions do not duplicate or lose writes, and to handle partial connectivity (the connection that opens then drops mid-sync) without corrupting state.

Agents tend to treat offline as "add a cache" and sync as "push when online," which works for read-only data and collapses for writes. The harm appears as offline edits that vanish on sync (overwritten by older server state), as last-write-wins silently discarding concurrent edits, as sync loops or duplicate writes when a sync is interrupted and retried, and as UI that blocks on a network the user does not have. The judgment is to design the local store as the source of truth, to version records for conflict detection, to choose a conflict strategy per data type (some data tolerates last-write-wins; some requires merge or manual resolution), to make every sync operation idempotent and resumable, and to treat partial connectivity as the normal case rather than an error. Offline-first is a reconciliation problem disguised as a caching problem.

## Core Rules

### Make The Local Store The Source Of Truth For The UI

In an offline-first app, the UI reads and writes against local data, and the local store is the source of truth the user sees. The server is a peer that the local store reconciles with, not the master the UI waits on. This is what makes the app usable without connectivity.

- **The UI never blocks on the network.** Reads come from local storage; writes go to local storage and are queued for sync. A user on a flaky network should not stare at a spinner.
- **The local store is authoritative for the user's view.** Even if the server has newer data, the user sees their local state until a sync reconciles; this is what makes offline edits feel real.
- **Sync is a background reconciliation, not a user-blocking operation.** The user creates, edits, and reads; sync happens behind them and updates the local store when it succeeds.

### Detect Conflicts Deliberately, Resolve With A Strategy Matched To The Data

When the same record is edited in two places (two devices, or offline and on the server), the sync must detect the conflict and resolve it. The resolution strategy must match the data: some data tolerates last-write-wins; some requires field-level merge; some requires manual resolution.

- **Version records to detect conflicts.** A version (revision number, timestamp, vector clock) on each record lets sync detect that two edits diverged from a common base; without versioning, conflicts are invisible and silently lost.
- **Choose the strategy per data type.** Last-write-wins is acceptable for ephemeral or single-user data; field-level merge for multi-user collaborative data; manual resolution for high-stakes data where the user must decide.
- **Never silently discard user edits.** Last-write-wins that overwrites an offline edit with older server state loses user work silently; if a conflict cannot be auto-resolved safely, surface it rather than discarding.
- **Preserve discarded edits for recovery.** When a conflict resolution discards an edit, keep the discarded version accessible (a conflict history) so the user can recover it if the resolution was wrong.

### Make Sync Idempotent And Resumable

Sync operates over an unreliable network, so it will be interrupted — mid-batch, mid-request, mid-transaction. A sync that is not idempotent duplicates writes on retry; one that is not resumable restarts from the beginning on interruption and never completes a large backlog.

- **Make every sync operation idempotent.** Replaying the same sync request must produce the same result as sending it once; use client-generated IDs and operation IDs so a retried write does not duplicate.
- **Make sync resumable from the point of interruption.** Track sync progress (a cursor, a per-record ack) so an interrupted sync resumes where it left off rather than restarting.
- **Handle partial failure within a batch.** A batch where some records succeed and some fail must ack the successes and retry only the failures, not redo the whole batch or lose the successes.

### Handle Partial Connectivity As The Normal Case

Mobile connectivity is not binary; it is intermittent, slow, captive (a portal that blocks until accepted), and partial (drops mid-sync). Design for this spectrum rather than for a clean online/offline toggle.

- **Assume the connection can drop mid-sync.** A sync in progress must tolerate the connection dropping at any point and resume cleanly; a half-completed sync must not leave the local store or the server in an inconsistent state.
- **Detect captive portals and degraded networks.** A connection that "works" but cannot reach the server (captive portal, DNS failure, extreme latency) should be treated as offline for sync purposes.
- **Back off and retry sensibly.** Aggressive retry on a failing network drains battery and hammers the server; use exponential backoff with jitter, and respect the OS's network-availability signals.

### Communicate Sync State To The User

Offline-first changes the user's mental model: their edits may not yet be on the server, and conflicts may need their attention. Communicate sync state (pending, syncing, synced, conflict) so the user understands where their work lives.

- **Show pending and syncing state.** The user should know which edits are local-only and which have synced, especially before closing the app or switching devices.
- **Surface conflicts that need resolution.** When a conflict requires a user decision, tell them clearly what conflicted and offer the resolution options; do not silently pick.
- **Communicate failures honestly.** A sync that keeps failing should tell the user, not silently queue forever; the user may need to act (open the app, fix connectivity, resolve a conflict).

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Assuming Connectivity

Designing the app to require the network, so it is unusable on a flaky connection. Make the local store the source of truth; the UI never blocks on the network.

### Last-Write-Wins Silently Discarding Concurrent Edits

Using last-write-wins for collaborative data, silently discarding an offline edit when the server has a newer version. Version records to detect conflicts; choose the strategy per data type; never silently discard user edits; preserve discarded edits for recovery.

### Non-Idempotent Sync Duplicating On Retry

A sync that duplicates writes when retried because it lacks client-generated IDs or operation IDs. Make every sync operation idempotent.

### Non-Resumable Sync Restarting On Interruption

A sync that restarts from the beginning when interrupted, never completing a large backlog. Track sync progress and resume from the point of interruption; handle partial failure within a batch.

### Sync Blocking The UI

Performing sync on the UI thread or gating UI updates on sync completion, so the app freezes on a slow network. Sync is a background reconciliation; the UI reads and writes local data.

### Partial Connectivity Corrupting State

A connection that drops mid-sync leaving the local store or server inconsistent, because the sync was not designed for mid-operation interruption. Assume the connection can drop at any point; design sync to resume cleanly.

### Silent Sync Failures

A sync that keeps failing without telling the user, so edits sit queued forever and the user assumes they are synced. Communicate pending, syncing, synced, and conflict state; surface persistent failures honestly.

## Self-Check

- [ ] The local store is the source of truth for the UI: reads and writes are local, the UI never blocks on the network, and sync is a background reconciliation that updates the local store when it succeeds.
- [ ] Conflicts are detected deliberately (records carry versions — revision numbers, timestamps, or vector clocks — so divergence from a common base is detectable) and resolved with a strategy matched to the data type (last-write-wins for ephemeral/single-user data, field-level merge for collaborative data, manual resolution for high-stakes data).
- [ ] User edits are never silently discarded: conflicts that cannot be auto-resolved safely are surfaced, and discarded edits are preserved in a conflict history for recovery.
- [ ] Sync is idempotent (client-generated IDs and operation IDs so retried writes do not duplicate) and resumable (a cursor or per-record ack so an interrupted sync resumes from the point of interruption, with partial-batch failure acking successes and retrying only failures).
- [ ] Partial connectivity is treated as the normal case: sync tolerates the connection dropping mid-sync and resumes cleanly, captive portals and degraded networks are detected and treated as offline, and retry uses exponential backoff with jitter respecting OS network signals.
- [ ] Sync state is communicated to the user (pending, syncing, synced, conflict), conflicts needing a decision are surfaced with clear options, and persistent failures are reported honestly rather than queued silently.
- [ ] The highest-risk cases were verified — an offline edit reconciled without loss, a conflict detected by versioning and resolved by strategy, a sync interrupted mid-batch resumed without duplication, and a captive/degraded network treated as offline — not only the clean always-online path.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
