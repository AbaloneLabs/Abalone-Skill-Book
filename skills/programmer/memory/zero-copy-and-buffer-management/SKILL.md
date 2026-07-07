---
name: zero_copy_and_buffer_management.md
description: Use when the agent is optimizing data movement on a hot path — reducing copies in parsing, serialization, network I/O, or inter-component data passing; using zero-copy techniques (sendfile, splice, io_uring, scatter-gather, reference-counted buffers, bytes/Buf/ByteBuffer slicing); managing buffer pools and lifetimes; choosing between copying and sharing; or diagnosing performance lost to unnecessary memory copies, allocations, or buffer churn. Covers when zero-copy helps and when it does not, buffer ownership and lifetime across async/thread boundaries, pooled vs allocated buffers, and the correctness risks (aliasing, use-after-free) of shared buffers.
---

# Zero Copy And Buffer Management

Every byte of data that moves through a system is, by default, copied at each boundary: read from the network into a buffer, copied into a parser's structure, copied into a domain object, copied into a serializer's output, copied into the write buffer. On a hot path moving megabytes or gigabytes per second, these copies dominate CPU and memory bandwidth, and each allocation churns the allocator and the garbage collector. Zero-copy techniques — passing references to a buffer rather than copying its contents, using OS-level scatter-gather and sendfile to move data without touching user space, slicing a buffer into views that share the underlying memory — eliminate the copies and the allocations. But they introduce a different problem: when multiple consumers share one buffer, the buffer's lifetime, ownership, and mutation discipline become a correctness concern, and a mistake becomes a data race or a use-after-free.

Agents tend to either ignore copies (treating data movement as free) or apply zero-copy reflexively (adding reference-counting and slicing everywhere, including paths where a copy is cheaper and simpler). Neither is right. The judgment problem is recognizing where copies are expensive enough to matter (hot paths, large buffers, high throughput) and where they are not (cold paths, small data, simplicity wins), and managing the ownership and lifetime complexity that zero-copy introduces only where its benefit justifies it. This skill covers the discipline of reducing unnecessary data movement: identifying the copies that matter, applying zero-copy techniques appropriately, and managing buffer ownership and lifetime correctly so the optimization does not introduce correctness bugs.

## Core Rules

### Identify Where Copies Actually Cost, Before Eliminating Them

Not every copy matters. Eliminating copies that do not contribute to the bottleneck is complexity for no benefit. Measure where the copies are and what they cost before optimizing.

- **Profile to find the copies on the hot path.** A copy of a 100-byte header once per request is negligible; a copy of a 1MB payload per request at 10,000 requests/sec is 10GB/sec of memory bandwidth. The copies that matter are those of large buffers or high-frequency small ones on the path that dominates throughput or latency.
- **Distinguish allocation cost from copy cost.** Sometimes the expense is the allocation (churning the allocator or GC) rather than the bytes moved; a buffer pool that reuses allocations may help more than zero-copy. Sometimes it is the copy itself (memory bandwidth); zero-copy helps. Know which.
- **Optimize the dominant copy first.** As with all performance work (see optimization-prioritization), address the copy that contributes the most to the bottleneck, not the one that is technically interesting. A zero-copy parser that leaves the response-serialization copy untouched optimizes the wrong half.

### Use Zero-Copy Techniques Appropriate To The Boundary

Different boundaries offer different zero-copy mechanisms. Use the one that fits where the data moves.

- **OS-level zero-copy for file-to-network (sendfile, splice, io_uring).** Serving a file over the network can bypass user space entirely: the kernel copies file pages directly to the socket. This eliminates the read-into-buffer, copy-to-user, copy-to-socket path. Use it for static file serving and proxying of large files.
- **Scatter-gather and vectored I/O (readv/writev, iovec) to assemble/disassemble without copying.** Writing headers and body from separate buffers in one syscall, without concatenating them into one buffer, avoids a copy. Use vectored I/O when composing output from multiple parts.
- **Buffer slicing and views to share without copying.** A parser that returns slices (views) into the input buffer, rather than copying fields into new strings, avoids copies — provided the input buffer's lifetime covers the slices' use. Languages provide this as `bytes`/`ByteString` slicing, `Buf`/`ByteBuffer` views, or `span`.
- **Reference-counted shared buffers for multi-consumer data.** When multiple consumers need the same buffer (a fan-out to multiple sinks), a reference-counted buffer (Arc<Vec>, shared_ptr, reference-counted ByteBuffer) avoids copying while keeping the buffer alive until all consumers finish. The cost is atomic reference-count operations; the benefit is avoiding a copy per consumer.

### Manage Buffer Ownership And Lifetime Explicitly

Zero-copy means multiple references to one buffer, and the buffer must remain valid for as long as any reference is used. This is the core correctness concern, and the source of the bugs zero-copy can introduce.

- **Establish clear ownership for every shared buffer.** Know whether the buffer is owned by a single consumer (who frees it), reference-counted (freed when the last reference drops), or owned by an external system (a memory-mapped region, a pool) with a defined return. Ambiguous ownership leads to use-after-free or double-free.
- **Ensure buffer lifetime covers all references' use.** A slice into a buffer is valid only as long as the buffer is not freed or mutated. If a slice escapes the scope of the buffer's ownership (returned from a function, stored long-term, sent across a thread), the buffer's lifetime must be extended (reference-counting) or the slice must be copied.
- **Beware mutation through shared references.** If one consumer mutates a shared buffer while another reads it, the reader sees inconsistent data (a data race in concurrent code, a logic bug in sequential code). Shared buffers for read-only consumers are safe; shared buffers with any writer require synchronization or copy-on-write.
- **Be careful with buffer pools and reuse.** A pooled buffer returned to the pool and then reused must not be referenced by a consumer that still holds it. Pool return must happen only after all references are released, or the pool lends buffers with a clear contract that the borrower no longer references them on return.

### Choose Copying Versus Sharing Deliberately

Zero-copy is not always the right choice. Sometimes a copy is simpler, safer, and not measurably slower. Decide based on the tradeoff, not on the principle that "fewer copies is better."

- **Copy when the buffer is small, the path is cold, or simplicity matters more than the saved copy.** A copy of a small struct or a cold-path header is cheap and eliminates ownership complexity. Premature zero-copy on cold paths adds complexity for no benefit.
- **Copy when the data must be owned long-term or mutated independently.** Data stored in a long-lived structure, or that a consumer will mutate, should be copied so the consumer owns its independent copy. Sharing a buffer into long-term storage couples the storage's lifetime to the buffer's origin.
- **Share (zero-copy) when the buffer is large, the path is hot, and the consumers are read-only or short-lived.** A large payload fanned out to multiple read-only sinks, or parsed into views used within the request, benefits from zero-copy and has manageable lifetime.
- **Copy at trust or ownership boundaries.** When data crosses a boundary where the source should not retain influence over the destination (untrusted input parsed into trusted structures, data handed to a background task), copying decouples the lifetimes and prevents the source from mutating the destination.

### Reduce Allocation Churn, Not Just Copies

Often the cost on a hot path is not the copy but the allocation and deallocation churn — many small allocations per operation that stress the allocator and, in managed runtimes, the garbage collector.

- **Pool buffers to reuse allocations on hot paths.** A buffer pool lends buffers and reclaims them, avoiding per-operation allocation. Effective for fixed-size, high-churn buffers (network read buffers, serialization buffers). Ensure the pool's contract (borrow/return, no retained references) is respected.
- **Pre-size buffers to avoid reallocation.** A buffer that grows incrementally (vector doubling) reallocates and copies as it grows; pre-sizing to the known final size avoids the intermediate copies. Estimate the size from the input where possible.
- **Use arena or bump allocation for bulk-scoped allocations.** Allocations within a request or operation scoped to an arena are freed together (arena reset), avoiding per-object deallocation and reducing fragmentation. See allocation-patterns-and-arena.
- **Reduce allocation in managed runtimes on hot paths.** In GC'd languages, allocation rate drives GC frequency and pause time (see gc-tuning). Reusing buffers, pooling, and avoiding per-request allocation reduces GC pressure.

## Common Traps

### Zero-Copy Everywhere, Including Cold Paths

Applying reference-counting and slicing to cold paths or small buffers where a copy is cheaper and simpler, adding ownership complexity for no measurable benefit. Copy where simplicity wins; zero-copy where measurement shows it matters.

### Shared Buffer With Lifetime Shorter Than References

A slice or reference into a buffer that is freed or reused before the reference is done, causing use-after-free or stale data. Ensure buffer lifetime covers all references; copy references that escape the buffer's scope.

### Mutation Through A Shared Reference

One consumer mutating a shared buffer while another reads, producing inconsistent data or a data race. Shared buffers are safe for read-only consumers; any writer requires synchronization or copy-on-write.

### Misused Buffer Pool With Retained References

Returning a pooled buffer to the pool while still referencing it, then having the reused buffer's contents change under the stale reference. Respect the pool's borrow/return contract; ensure no retained references on return.

### OS-Level Zero-Copy Where It Does Not Apply

Using sendfile or splice for data that is not file-to-network or that requires transformation in user space, where it cannot help. Match the zero-copy technique to the boundary.

### Ignoring Allocation Churn As The Real Cost

Optimizing copies while the actual cost is allocation churn (many small allocations stressing the allocator or GC). Profile to distinguish copy cost from allocation cost; address the dominant one.

### Growing Buffers Incrementally On A Hot Path

A buffer that reallocates and copies as it grows, on a path where pre-sizing would avoid the intermediate copies. Pre-size buffers to the known or estimated final size.

### Copying At A Boundary That Decouples Lifetimes Unnecessarily

Copying data that could be shared read-only, adding allocation and bandwidth cost where a reference would suffice. Share where consumers are read-only and short-lived; copy only at true ownership or trust boundaries.

## Self-Check

- [ ] The copies that matter (large buffers or high-frequency small ones on the hot path) are identified by profiling, and the dominant copy is addressed first — not every copy is eliminated reflexively.
- [ ] The zero-copy technique matches the boundary: OS-level (sendfile/splice/io_uring) for file-to-network, vectored I/O for multi-part composition, slicing/views for parsing, reference-counted buffers for multi-consumer fan-out.
- [ ] Buffer ownership and lifetime are explicit for every shared buffer (single owner, reference-counted, or externally owned with defined return), and buffer lifetime provably covers all references' use — references that escape the buffer's scope are copied or the lifetime is extended.
- [ ] Shared buffers with any writer use synchronization or copy-on-write; read-only shared buffers are confirmed read-only; mutation through shared references is prevented.
- [ ] Buffer pools, where used, have a clear borrow/return contract that no borrower retains references after return, and pre-sized buffers avoid incremental reallocation copies on hot paths.
- [ ] The choice between copying and sharing is deliberate: copy for small/cold/simple cases or long-term independent ownership and trust boundaries; share for large/hot/read-only/short-lived cases — driven by measurement, not by a reflexive "fewer copies is better."
- [ ] Allocation churn is distinguished from copy cost and addressed where it is the dominant expense (buffer pools, pre-sizing, arena allocation, reduced allocation in managed runtimes).
- [ ] The zero-copy design has been reviewed for correctness risks — aliasing, use-after-free, mutation through shared references, pool contract violations — each addressed, so the optimization does not introduce data-race or lifetime bugs.
