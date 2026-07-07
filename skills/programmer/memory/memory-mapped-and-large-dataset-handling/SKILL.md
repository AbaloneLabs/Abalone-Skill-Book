---
name: memory_mapped_and_large_dataset_handling.md
description: Use when the agent is working with datasets larger than memory or that should not be fully loaded — memory-mapped files (mmap), streaming processing of large files or datasets, processing multi-gigabyte logs/exports/dumps, database engines or storage engines, working with huge arrays or matrices, virtual memory management, page cache behavior, lazy loading of large data, or designing a system that processes data larger than RAM without OOM. Covers mmap tradeoffs, streaming versus loading, page cache and OS-level caching, working set management, fragmentation and address space, and the failure modes (OOM, page faults, SIGBUS) of large-dataset handling.
---

# Memory Mapped And Large Dataset Handling

When a dataset is larger than memory, or when loading it fully would be wasteful or slow, the program must work with a subset at a time — streaming, memory-mapping, or lazily loading the portion it needs. This is a different programming model from "load the data into an array and index it," and it has different failure modes. A program that memory-maps a 50GB file and then accesses it randomly may trigger page faults that thrash the disk; a streaming processor that buffers too much defeats the purpose of streaming and OOMs; a working set that exceeds RAM causes the OS to swap, collapsing performance by orders of magnitude. The intuition that "memory is memory" breaks down precisely at the boundary where the data does not fit, and the operating system's virtual memory machinery — page cache, paging, mmap — becomes the dominant factor in performance and reliability.

Agents tend to treat large-dataset handling as ordinary in-memory processing with a bigger buffer, reaching for `mmap` or "read the whole file" without understanding the paging behavior, the working-set implications, or the failure modes (SIGBUS on a truncated file, OOM from an oversized buffer, thrashing from random access over a mapping larger than RAM). The judgment problem is recognizing that processing data larger than memory is a streaming and locality problem, that the operating system's virtual memory is a partner whose behavior must be designed for, and that the choice among loading, streaming, and memory-mapping depends on the access pattern, the working set, and the failure tolerance. This skill covers the discipline of handling datasets that do not fit in memory: choosing the right access model, managing the working set, leveraging the page cache, and avoiding the OOM, thrashing, and crash failures that naive large-data code produces.

## Core Rules

### Choose The Access Model By The Access Pattern

How you access the data determines whether to load it, stream it, or memory-map it. The wrong model for the pattern produces slowness or failure.

- **Load fully into memory when the dataset fits comfortably and is accessed randomly and repeatedly.** If the data fits in RAM with headroom and the access pattern benefits from random indexing (a lookup table, an in-memory index), loading it is simple and fast. Do not load data that does not fit, or that is accessed once and discarded.
- **Stream when the data is processed sequentially and once.** A log processor, an ETL transform, a file converter that reads, processes, and writes element-by-element uses bounded memory regardless of input size. Streaming is the right model for sequential, single-pass processing; it never OOMs on input size.
- **Memory-map (mmap) when the data is large, accessed with locality, and benefits from OS-managed paging.** mmap maps a file into the process's address space; the OS pages portions in and out as accessed. It is excellent for read-mostly, locality-friendly access (a database reading pages, a binary format with an index) because the OS page cache manages the working set. It is poor for random access over a mapping much larger than RAM (thrashing) and for patterns needing explicit control over what is resident.
- **Use a database or storage engine when the data is large, indexed, and updated.** Do not hand-roll what a database provides; an indexed, transactional store handles large data with queries, durability, and concurrency that custom code rarely replicates correctly.

### Manage The Working Set, Not Just The Dataset

The working set — the portion of the data actively accessed — is what determines performance, not the dataset size. A 100GB dataset with a 1GB working set performs like a 1GB dataset if the working set fits in RAM; the same dataset with a 100GB working set thrashes regardless of how cleverly it is mapped.

- **Identify and optimize for the working set.** What portion of the data is accessed repeatedly, and does it fit in RAM? Design access patterns and data layouts to keep the hot working set small and resident.
- **Layout data for locality.** Data accessed together should be stored together (contiguous, in the same pages) so that paging one piece brings in the related pieces. A format that scatters related data across the file defeats the page cache; a format that clusters it benefits. This is why columnar formats, B-trees, and log-structured stores exist.
- **Avoid random access over a mapping much larger than RAM.** Random access over a 50GB mmap with a 50GB working set faults on every access, thrashing the disk. Either reduce the working set (index into a smaller hot region), pre-load the needed pages (madvise, explicit read), or use a storage engine designed for this.
- **Prefetch and advise the OS of access patterns.** `madvise(WILLNEED)` prefetches pages; `madvise(SEQUENTIAL)` hints at sequential access; `madvise(DONTNEED)` releases pages no longer needed. Use these to cooperate with the page cache when the access pattern is known.

### Leverage The Page Cache; Do Not Fight It

The OS page cache is a shared, system-managed cache of file contents, and it is usually better to cooperate with it than to bypass it. Code that fights the page cache (direct I/O everywhere, manual caching of file contents in application memory) often performs worse and uses more memory.

- **Prefer buffered I/O and mmap for read-mostly data, letting the page cache manage residency.** The page cache is shared across processes, survives process restarts, and is tuned by the OS. Re-reading a hot file from the page cache is free; re-reading it from disk is not.
- **Use direct I/O only when you have a specific reason.** Direct I/O bypasses the page cache, useful when the application manages its own cache (a database with a buffer pool) and double-caching wastes memory, or for write workloads where the page cache adds overhead. Do not use it by default; it often slows read-mostly workloads.
- **Do not duplicate the page cache in application memory unnecessarily.** Reading a file into an application buffer that the page cache already holds doubles the memory for the same data. For read-mostly access, mmap or buffered reads share the page cache; copying into application memory is redundant.
- **Be aware that the page cache is volatile and shared.** Memory pressure evicts pages; another process's activity can evict your hot pages. For data that must be resident, pin it (mlock, with care) or accept that residency is best-effort.

### Bound Memory Explicitly For Streaming And Batch Processing

Streaming and batch processing must bound their memory use explicitly, or a pathological input (a single enormous record, a fan-out that buffers everything) causes OOM despite the streaming design.

- **Process records one at a time or in bounded batches, not by accumulating the whole input.** A stream processor that accumulates results in memory before writing OOMs on large input; one that writes incrementally uses bounded memory. Bound batch sizes explicitly.
- **Bound the size of individual records and fields.** A single record with a multi-gigabyte field, or a nested structure that expands unboundedly, defeats streaming. Validate and bound record sizes; reject or truncate pathological inputs.
- **Bound fan-out and buffering in parallel pipelines.** A parallel stream that reads ahead faster than it processes buffers unbounded data in flight. Bound the concurrency and the in-flight buffer (backpressure) so memory is bounded by the pipeline depth, not the input size. See backpressure-and-load-shedding.
- **Spill to disk when in-memory state exceeds a bound.** A sort or aggregation that exceeds memory should spill to disk (external sort) rather than OOM. Design operators that can spill gracefully.

### Handle The Failure Modes Of Large-Data Operations

Large-data operations have specific failure modes that ordinary in-memory code does not, and they must be handled or the program crashes or hangs.

- **SIGBUS on a memory-mapped file that is truncated or on a filesystem that cannot serve a page.** An mmap access to a page that cannot be satisfied (the file shrank, a network filesystem dropped, a disk failed) raises SIGBUS, crashing the process. Handle this for robustness (signal handler) or avoid mmap on unreliable storage.
- **OOM from oversized buffers or unbounded accumulation.** Loading a file larger than memory, or accumulating results without bound, triggers OOM. Bound memory explicitly; stream or spill.
- **Thrashing when the working set exceeds RAM.** Random access over a too-large mapping, or a working set that grows beyond RAM, causes paging that collapses performance by 100x+. Monitor working-set size; detect and shed load when it exceeds RAM.
- **Address space exhaustion on 32-bit or constrained processes.** A very large mmap may exhaust the process's virtual address space (notably on 32-bit). Use 64-bit processes; be aware of address space limits in constrained environments.
- **Fragmentation in long-lived mappings or arenas.** Repeatedly mapping and unmapping, or an arena with a fragmentation-prone pattern, can fragment address space or memory. See allocation-patterns-and-arena.

## Common Traps

### Loading A File Larger Than Memory

Reading an entire large file into memory, OOM-ing or triggering swap that collapses performance. Stream, memory-map, or use a storage engine for data larger than RAM.

### Random Access Over A Mapping Larger Than RAM

mmap-ing a huge file and accessing it randomly, thrashing the disk with page faults. Reduce the working set, use an index into a hot region, or use a storage engine designed for random access.

### Streaming That Accumulates Unboundedly

A stream processor that buffers all records or accumulates results in memory, OOM-ing on large input despite the streaming design. Process incrementally; bound batches and in-flight buffers.

### Fighting The Page Cache

Using direct I/O or manual application caching where the page cache would suffice, doubling memory and slowing reads. Prefer buffered I/O and mmap for read-mostly data.

### Ignoring Working Set Size

Treating the dataset size as the performance determinant, when the working set is what matters, and letting the working set grow beyond RAM into thrashing. Identify and bound the working set; lay out data for locality.

### No Bound On Record Or Field Size

A streaming pipeline that OOMs on a single pathological record with an enormous field. Validate and bound record and field sizes.

### Unhandled SIGBUS On mmap

A memory-mapped file access crashing with SIGBUS when the file is truncated or storage fails, with no handling. Avoid mmap on unreliable storage or handle the signal.

### Spill-Free Operators That OOM

A sort or aggregation that holds all data in memory and OOMs rather than spilling to disk. Design operators to spill when memory exceeds a bound.

## Self-Check

- [ ] The access model (load fully, stream, memory-map, or use a storage engine) is chosen by the access pattern — load for random repeated access that fits in RAM, stream for sequential single-pass, mmap for large locality-friendly read-mostly, storage engine for large indexed/updated data.
- [ ] The working set (not the dataset size) is identified as the performance determinant, kept within RAM, and optimized via data layout for locality (related data stored together) so the page cache is effective.
- [ ] Random access over a mapping much larger than RAM is avoided (it thrashes); instead the working set is reduced, an index into a hot region is used, or a storage engine designed for random access is employed.
- [ ] The OS page cache is leveraged rather than fought — buffered I/O and mmap for read-mostly data, direct I/O only with specific reason, no redundant application-memory duplication of cached file contents — and access patterns advise the OS (madvise) where known.
- [ ] Streaming and batch processing bound memory explicitly: records processed one-at-a-time or in bounded batches, individual record/field sizes validated and bounded, parallel fan-out bounded with backpressure, and operators that can spill to disk when memory exceeds a bound.
- [ ] Large-data failure modes are handled: SIGBUS on mmap (avoided on unreliable storage or handled), OOM from oversized buffers (prevented by bounding), thrashing from working-set exceeding RAM (monitored and shed), address space limits (64-bit processes), and fragmentation in long-lived mappings/arenas.
- [ ] The data layout is designed for the access pattern (columnar, B-tree, log-structured, clustered) so that paging brings in related data, rather than scattering related data across the file.
- [ ] Where the data is large, indexed, and updated, a database or storage engine is used rather than a hand-rolled solution that omits transactions, concurrency, and query optimization.
