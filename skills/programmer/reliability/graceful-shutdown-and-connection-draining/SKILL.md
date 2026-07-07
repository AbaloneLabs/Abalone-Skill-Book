---
name: graceful_shutdown_and_connection_draining.md
description: Use when the agent is handling process shutdown, SIGTERM/SIGINT, Kubernetes pod termination, deploy rollouts, draining in-flight requests, closing database connections and background workers cleanly, avoiding abrupt kills that drop messages or corrupt writes, or reviewing whether a service shuts down without losing work.
---

# Graceful Shutdown And Connection Draining

A server does not simply stop. When a deploy, a scale-down, an autoscaler, a rolling update, or an orchestrator tells a process to terminate, the process is usually in the middle of work: holding open HTTP requests, mid-way through a database transaction, holding a leased message from a queue, writing to a file, or running a background job that will be left half-finished. The judgment problem is that "stop the process" is two very different operations depending on how it is done. A hard kill (`SIGKILL`, a container being shot, a force-stop) severs everything instantly and leaves in-flight work lost, messages un-acked and redelivered, transactions half-committed, and clients seeing broken connections. A graceful shutdown drains the work that is already in progress, refuses new work, waits for it to finish within a budget, and only then exits — so that a routine deploy does not look like a small outage to every client and worker caught mid-request.

Agents tend to treat shutdown as an afterthought because the happy path — start up and serve — is where the features are. The shutdown path is exercised on every deploy and every scale event, yet it is rarely tested, and its failure modes are silent until production: requests that error during every rollout, queue messages that are processed twice because they were claimed but never acked, database connections that exhaust the pool because the process died without returning them, and background jobs that vanish mid-run and leave the system in an inconsistent state. The disciplined engineer designs shutdown as a first-class lifecycle phase: stop accepting new work, drain what is in flight within a bounded deadline, clean up resources, and exit cleanly — and verifies it under the conditions that actually occur (an orchestrator that sends SIGTERM, waits a fixed grace period, then SIGKILLs).

This skill is about the shutdown and drain path specifically. It complements the deployment skills (which decide rollout strategy) and the reliability skills (which handle runtime failures). Here the question is: when the process is told to stop, how do you ensure no work is lost and no resource is leaked.

## Core Rules

### Treat Shutdown As A Lifecycle Phase, Not An Emergency Stop

Design the shutdown sequence as deliberately as the startup sequence. A service that boots carefully but dies abruptly has only solved half its lifecycle. The shutdown phase has a defined shape:

- **Stop accepting new work.** Close the listening socket, stop pulling from the queue, stop the scheduler, and stop the health-check from reporting ready, so load balancers and orchestrators stop sending new requests.
- **Let in-flight work finish.** Allow requests, transactions, and jobs that have already started to complete, up to a deadline.
- **Refuse or hand off new work.** Return an appropriate response (503, connection close) to anything that still arrives, or let an upstream load balancer shed it.
- **Clean up resources.** Close database connections, release locks and leases, flush buffers, close files, deregister from service discovery.
- **Exit.** Return a success code when the drain completed, or let the deadline force an exit when it did not.

Write this sequence down. A shutdown path that exists only in the developer's head will not survive the next person who edits the server.

### Know What Will Send The Kill Signal, And How Long You Have

The shutdown budget is not yours to choose freely; it is set by whatever manages the process. Before designing the drain, find out:

- **What signal will you get?** Orchestrators and process managers typically send `SIGTERM` first, then `SIGKILL` after a grace period. Some send `SIGINT`. Some send `SIGTERM` twice. Know which your platform uses and trap the right signals.
- **How long is the grace period?** Kubernetes' `terminationGracePeriodSeconds` defaults to 30 seconds; systemd, ECS, Nomad, and cloud autoscalers each have their own. Your drain must finish comfortably inside this window, because after it the process is killed hard regardless of in-flight work.
- **Will you get a pre-stop hook?** Some platforms run a `preStop` hook before sending SIGTERM, which you can use to start deregistration before the signal arrives.

If your drain can legitimately take longer than the grace period (a long-running batch job, a large file upload), you cannot rely on graceful shutdown alone — you need checkpointing, work-stealing, or a separate worker that survives the process. Design for the budget you actually have, not the budget you wish you had.

### Deregister Before Draining, And Account For Propagation Lag

The single most common cause of request errors during deploys is a race between shutdown and load-balancer propagation. The process stops accepting connections, but upstream routers still think it is healthy and send traffic for a few more seconds, which fails. Close this gap:

- **Tell the world you are leaving before you stop serving.** Deregister from service discovery, or have the health endpoint start failing readiness, before closing the listener. Use a `preStop` hook or an explicit shutdown step.
- **Wait for propagation.** Give routers time to observe the deregistration and stop sending new connections. A sleep or a readiness-probe interval of several seconds in the `preStop` hook is often the cheapest, most effective fix for deploy-time errors.
- **Then drain.** Only after propagation should you stop accepting new connections and let in-flight work finish.

Skipping the propagation wait is why "we do graceful shutdown but we still see 5xx on every rollout." The drain handles in-flight work; propagation handling stops new work from arriving during the drain.

### Drain In-Flight Work Within A Deadline, Then Force

A drain without a deadline is a drain that can hang forever — a slow client, a stalled upstream, a stuck transaction keeps the process alive until the orchestrator force-kills it, at which point you lose both the stuck work and any work that finished but had not yet been cleaned up. Give the drain a hard internal deadline that is shorter than the platform's grace period:

- **Set a drain timeout** (for example, 25 seconds inside a 30-second grace window) after which you stop waiting and begin forced cleanup.
- **Track in-flight work** with a counter, a wait-group, or the runtime's request-tracking mechanism, so you know when the last request finishes and can exit early instead of always waiting the full timeout.
- **Log what did not finish.** If the deadline expires with work still in flight, record which requests, jobs, or messages were abandoned, so the loss is visible and debuggable rather than silent.

Exiting early when work is done (rather than always sleeping the full grace period) speeds up rollouts; exiting on deadline (rather than hanging) guarantees the process terminates within the window. Both matter.

### Handle Each Kind Of In-Flight Work According To Its Semantics

Different work needs different drain treatment, because "finishing" means different things and the cost of interruption differs:

- **Synchronous requests (HTTP, RPC).** Let them finish their response within the deadline. New requests get refused. A request that cannot finish in time returns an error or a partial result; clients retry against a healthy instance.
- **Database transactions.** Never let a transaction be killed mid-commit. A transaction should either commit fully or roll back fully; a process dying between the writes of a multi-statement transaction can leave data inconsistent. Ensure the drain waits for transactions to reach a commit point, or rolls them back cleanly, before exit.
- **Queue consumers.** A message that was claimed but not acked will be redelivered after the consumer dies — which is safe only if processing is idempotent. During shutdown, stop fetching new messages, finish or abandon the in-flight one, and let the broker redeliver it. Never ack a message you did not finish processing just to exit cleanly.
- **Background jobs and schedulers.** A job interrupted mid-run may leave partial state. Either make the job resumable (checkpoint progress) or ensure it is idempotent so a retry completes it safely. Long jobs that cannot finish in the grace window must run in a process that is not tied to the request-serving lifecycle.
- **File and buffer writes.** Flush buffered writes before exit. A process that buffers writes in memory and is killed hard can lose the last batch of writes silently.

Map every category of work your process does and decide, for each, what "safe to interrupt" means. A shutdown sequence that handles only HTTP requests but ignores the background worker is incomplete.

### Make Resources Reclaimable: Connections, Locks, Leases, And File Handles

Abrupt termination leaks resources that other processes depend on. A database connection not returned to the pool, a distributed lock not released, a lease not renewed, a temp file not deleted — these linger and cause failures long after the process is gone (pool exhaustion, stuck locks that block other workers, disk filling with orphans). During shutdown:

- **Close connections in an orderly way** so the pool or the remote side knows they are gone, rather than discovering it via a timeout.
- **Release locks and leases explicitly**, or ensure they have short TTLs so an abandoned one expires quickly rather than blocking others until manual intervention.
- **Clean up side effects**: temp files, partial uploads, scratch directories, registered callbacks, and spawned child processes. A process that forks children must propagate the shutdown signal to them, or it leaves orphans running.

Resources with a TTL (a lock that auto-expires, a lease that auto-renews) are more forgiving of a hard kill than resources that require explicit release. Prefer TTL-backed resources for anything that might be interrupted.

### Distinguish Planned Shutdown From Crash, And Design For Both

Graceful shutdown handles the planned case: a signal arrives, there is time to drain. The unplanned case — a crash, an OOM kill, a hardware failure, a `SIGKILL` — gives you no drain window at all. A robust system is correct under both, which means graceful shutdown cannot be the only safety mechanism:

- **Idempotency and retryability** ensure that work interrupted by a hard kill can be safely retried by another instance.
- **Durable state and transactions** ensure that a crash mid-write does not corrupt data.
- **Locks and leases with TTLs** ensure that a crashed process does not hold a resource forever.

Design graceful shutdown to make the common case (routine deploys, scale-downs) clean and lossless. Design the system underneath to survive the uncommon case (crashes, hard kills) by being idempotent, transactional, and TTL-backed. If you rely on graceful shutdown to be correct — if a hard kill loses data — then every unexpected crash is a data-loss event, and crashes are inevitable.

### Test The Shutdown Path, Especially Under Load

The shutdown path is the least-tested and most-exercised code in many services. It runs on every deploy and every scale event, yet it is almost never tested, so it fails only in production. Test it the way it actually happens:

- **Send SIGTERM under load** and verify that in-flight requests complete, new requests are refused, and the process exits within the deadline.
- **Test the deadline expiry**: send SIGTERM with a request that cannot finish in time, and verify the process exits anyway and logs what was abandoned.
- **Test the grace-period boundary**: verify the drain finishes inside the orchestrator's grace window, not just inside your own timeout.
- **Test propagation**: verify that a load balancer stops sending traffic before the process stops serving, so no requests error during the handoff.
- **Test queue and job semantics**: verify a redelivered message is handled idempotently, and an interrupted job is safely retried.

A shutdown path that has never been tested under load will drop work on the first real deploy. Treat the drain as a feature with its own tests.

## Common Traps

### Closing The Listener But Forgetting Load-Balancer Propagation

The process stops accepting connections the instant it gets SIGTERM, but the upstream load balancer still routes to it for several seconds, so those requests fail with connection errors during every rollout. The fix is to deregister or fail readiness first, wait for propagation, then close the listener. A graceful drain that does not account for propagation still causes deploy-time errors.

### Waiting Forever For In-Flight Work

A drain with no deadline hangs on a slow or stuck request until the orchestrator force-kills the process, losing both the stuck work and any cleanup. The fix is a hard internal drain deadline shorter than the grace period, after which the process begins forced cleanup and exits. "Graceful" does not mean "unbounded."

### Assuming The Grace Period Is Long Enough

Designing a drain that takes 40 seconds inside a platform whose grace period is 30 seconds. The orchestrator sends SIGKILL at 30 seconds regardless, cutting off the drain mid-way. The fix is to measure the platform's actual grace period, keep the drain comfortably inside it, and move long-running work out of the request-serving process. You do not get to choose the kill deadline.

### Acking Messages Just To Exit Cleanly

A queue consumer, on shutdown, acks the in-flight message it did not finish processing so the count looks clean and the process exits. The message is then lost. The fix is to never ack unfinished work; stop fetching, abandon or finish the in-flight message, and let the broker redeliver it to a healthy consumer — which is only safe because processing is idempotent. A clean exit count is not worth a lost message.

### Killing A Process Mid-Transaction

A multi-statement transaction is interrupted by shutdown between writes, leaving the data partially updated. The fix is to ensure the drain waits for transactions to reach a commit or rollback point, and to design transactions so they are atomic — all or nothing. A process should never die between the writes of a single logical operation.

### Ignoring Background Workers And Schedulers

The shutdown sequence carefully drains HTTP requests but does not stop or checkpoint the background job runner, so a job vanishes mid-run and leaves partial state. The fix is to enumerate every category of work — requests, jobs, schedulers, consumers — and give each an explicit drain policy. A shutdown that handles only the request path is incomplete.

### Leaking Connections, Locks, And Child Processes

The process exits without releasing database connections, distributed locks, or temp files, and without propagating the signal to spawned children. The pool exhausts, the lock blocks other workers, the disk fills, or orphans keep running. The fix is explicit cleanup of every resource and signal propagation to child processes. Resources that require explicit release should be TTL-backed as a safety net.

### Relying On Graceful Shutdown As The Only Correctness Guarantee

The team assumes deploys are safe because shutdown is graceful, but a crash, OOM kill, or `SIGKILL` bypasses the drain entirely and loses work. The fix is to treat graceful shutdown as an optimization for the common case, while making the system correct under hard kills through idempotency, durable transactions, and TTL-backed resources. If a hard kill loses data, the system is not actually safe — it is only safe when nothing goes wrong.

## Self-Check

- [ ] Shutdown is designed as a lifecycle phase with a defined sequence: stop new work, wait for propagation, drain in-flight work within a deadline, clean up resources, then exit.
- [ ] The kill signal the platform sends (SIGTERM/SIGINT) is trapped, and the drain finishes comfortably inside the platform's actual grace period — not an assumed one.
- [ ] The service deregisters or fails readiness before closing the listener, and waits for load-balancer propagation, so no requests error during the handoff.
- [ ] The drain has a hard internal deadline shorter than the grace period, tracks in-flight work to exit early when done, and logs what was abandoned if the deadline expires.
- [ ] Each category of work (requests, transactions, queue messages, background jobs, buffered writes) has an explicit drain policy matching its semantics — transactions reach commit/rollback, messages are not acked unfinished, jobs are idempotent or checkpointed.
- [ ] Connections, locks, leases, file handles, temp files, and child processes are cleaned up or TTL-backed, so an exit does not leak resources other instances depend on.
- [ ] The system remains correct under a hard kill or crash (idempotency, durable transactions, TTL-backed resources), not only under a graceful shutdown.
- [ ] The shutdown path is tested under load, including deadline expiry, the grace-period boundary, load-balancer propagation, and queue/job redelivery semantics.
