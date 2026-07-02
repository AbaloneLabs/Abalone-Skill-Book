---
name: ets_and_mnesia.md
description: Use when the agent is designing ETS tables in Elixir or Erlang, choosing access modes and table types (set ordered_set bag duplicate_bag), writing concurrent reads and writes, using Mnesia for distributed transactions, designing table ownership and heir options, or diagnosing race conditions, memory growth, lock contention, and consistency issues in ETS and Mnesia systems.
---

# ETS and Mnesia

ETS (Erlang Term Storage) and Mnesia are BEAM's in-memory and distributed storage, and they look like simple key-value stores until you depend on them for correctness. ETS is a concurrent, in-memory table whose access semantics (`set`, `ordered_set`, `bag`, `duplicate_bag`), ownership, and read/write concurrency modes determine whether your code is correct under contention. Mnesia adds distribution and transactions, and with them a set of consistency, availability, and partition-tolerance tradeoffs that are easy to get wrong. The judgment problem is not "how do I create an ETS table" but understanding that ETS trades the immutability and message-passing safety of normal BEAM code for shared mutable state with specific concurrency guarantees, and that the guarantees you assume (atomicity of read-modify-write, isolation between processes) are only provided by the specific operations you choose.

The recurring failure mode is a developer who uses ETS as a global mutable map, performs a read followed by a write expecting them to be atomic, and discovers that two processes racing on the same key corrupt the state because `:ets.lookup` and `:ets.insert` are individually atomic but not together. Or a team that adopts Mnesia for "distributed transactions" without understanding that Mnesia's consistency model under network partitions can block or produce inconsistent results, and that table fragmentation and schema management add operational complexity. Real ETS/Mnesia design requires choosing the right table type and concurrency mode for the access pattern, using `:ets.update_counter` or `:ets.select_replace` for atomic read-modify-write, and treating Mnesia as a distributed database with real CAP tradeoffs rather than a drop-in ETS.

## Core Rules

### Choose the table type by the access pattern

ETS table type determines what the table can hold per key:

- **`set`**: one object per key; `insert` replaces. The default; use for key-value lookups.
- **`ordered_set`**: one object per key, ordered by key; supports range and nearest-key queries via `select` and `first`/`next`/`last`/`prev`. Use when you need ordering; it is slower than `set` for plain lookups.
- **`bag`**: multiple objects per key, no duplicates. Use when a key maps to a set of values.
- **`duplicate_bag`**: multiple objects per key, duplicates allowed. Use when duplicates are meaningful (e.g., event logs).

Choosing the wrong type forces workarounds (manual lists in a `set`) or loses guarantees (accidental dedup in a `bag`).

### Choose access mode and concurrency options by the workload

- **`public`**: any process can read/write. Required for shared caches but shifts correctness to the callers.
- **`protected`** (default): any process can read, only the owner can write. Good balance for read-heavy shared state.
- **`private`**: only the owner can read/write. Use for owner-local state that happens to need ETS's speed.

For `public` tables, enable `{:read_concurrency, true}` for read-heavy workloads (optimizes for concurrent reads, disables some write optimizations) or `{:write_concurrency, true}` for write-heavy workloads (increases write throughput at the cost of memory). These are tradeoffs; benchmark your pattern.

### Use atomic operations for read-modify-write

The most common ETS bug is `lookup` then `insert` from two racing processes. Use atomic operations instead:

- **`:ets.update_counter/3`**: atomically increment a counter field; the standard pattern for counters and sequences.
- **`:ets.select_replace/3`**: atomically match and replace an object; use for conditional updates.
- **`:ets.insert_new/2`**: atomically insert only if the key does not exist; use for "create if absent."
- **`:ets.update_element/3`**: atomically update a specific field by key.

For complex atomic transformations, `:ets.select_replace` with a match spec, or a serialized owner process (GenServer holding the table), is the correct approach.

### Design table ownership and heir options deliberately

An ETS table is owned by a process and is destroyed when the owner terminates. Rules:

- For long-lived tables, own them in a dedicated long-lived process (a supervisor-started GenServer), not in a transient request handler.
- Use the `heir` option to transfer the table to a heir process on owner termination, enabling graceful handoff without data loss.
- For tables that must survive the owning process, use `:ets.give_away/3` to transfer ownership explicitly.

A table owned by a request-handling process disappears when the request ends; this is a common source of "where did my cache go" bugs.

### Match specs and `select` are powerful but opaque

`:ets.select` with a match specification is the way to query by pattern, but match specs are a mini-language that is hard to read and easy to get wrong. Rules:

- Use `Ex2ms` (the `ex2ms` library) to write match specs as Elixir syntax that compiles to match specs; this makes them readable and checkable.
- Prefer `select` over `tab2list` + `Enum.filter` for large tables; `tab2list` copies the entire table into a list, which is O(n) memory.
- Use `select` with a `limit` to page through large result sets rather than materializing all matches.

### Mnesia is a distributed database with CAP tradeoffs

Mnesia extends ETS with distribution, transactions, and persistence. Rules:

- Understand that Mnesia's transaction semantics under network partitions depend on the configuration; a partitioned node may block or serve stale data.
- Choose table storage type (`ram_copies`, `disc_copies`, `disc_only_copies`) by durability and latency needs.
- Use transactions (`:mnesia.transaction`) for multi-table atomicity; use "dirty" operations (`:mnesia.dirty_read`) for single-key speed when atomicity across tables is not needed.
- Mnesia schema management (adding/removing nodes, changing table types) is operational work that must be planned, not done at runtime casually.

### Size ETS and Mnesia tables and monitor memory

ETS and Mnesia live in BEAM memory; unbounded growth crashes the VM. Rules:

- Bound tables by design (TTL eviction, LRU, max size) or accept that growth must be monitored.
- Use `:ets.info(table, :memory)` and `:mnesia.table_info` to monitor table size.
- For caches, prefer a bounded cache library (`Cachex`, `Nebraska`) over a hand-rolled unbounded ETS table.

### Treat `public` ETS as shared mutable state

A `public` ETS table is shared mutable state across processes, which is the exception to BEAM's share-nothing model. Rules:

- Ensure all writers use atomic operations or a serialized owner; concurrent non-atomic writes corrupt state.
- Document that the table is shared mutable state, and prefer `protected` (single writer) where possible.
- Do not use a `public` table as a substitute for message passing when the data is per-process; per-process state belongs in the process.

## Common Traps

### `lookup` then `insert` racing across processes

Two processes read the same key, both compute an update, both insert; the second overwrites the first. Use `update_counter`, `select_replace`, or `insert_new` for atomicity.

### `tab2list` on a large table

`:ets.tab2list/1` copies the entire table into a list, O(n) memory and time. Use `select` with a limit and page through results.

### Table owned by a transient process

An ETS table owned by a request handler or short-lived GenServer is destroyed when the owner ends. Own long-lived tables in a supervised process, or use `heir`/`give_away`.

### `public` table with non-atomic writers

A `public` table where writers do `lookup`/`insert` without atomic operations corrupts under concurrency. Serialize writes through the owner or use atomic ops.

### Match specs written by hand

Hand-written match specs are opaque and error-prone. Use `ex2ms` to write them as Elixir and let the compiler check them.

### Mnesia treated as drop-in distributed ETS

Mnesia's distribution brings CAP tradeoffs, schema management, and partition handling. Adopt it understanding the consistency model, not as a simple shared map.

### Unbounded table growth

An ETS or Mnesia table that grows without bound crashes the VM. Bound caches by size or TTL, and monitor table memory.

## Self-Check

- Is the table type (`set`/`ordered_set`/`bag`/`duplicate_bag`) chosen by the access pattern, with `ordered_set` used only where ordering is needed?
- Is the access mode (`public`/`protected`/`private`) chosen by who must read/write, with read/write concurrency options benchmarked for the workload?
- Are read-modify-write operations atomic (`update_counter`/`select_replace`/`insert_new`/`update_element`), with no `lookup`-then-`insert` races across processes?
- Is the table owned by a supervised long-lived process (or using `heir`/`give_away`), so it survives the processes that use it?
- Are queries using `select` with `ex2ms` and a `limit` for paging, rather than `tab2list` + `Enum.filter` on large tables?
- If using Mnesia, are the CAP tradeoffs, storage types, and transaction-vs-dirty-operation choices understood, with schema management planned rather than ad hoc?
- Are tables bounded by size or TTL (or via a cache library), with memory monitored via `:ets.info`/`:mnesia.table_info`?
- Are `public` tables documented as shared mutable state with all writers using atomic operations, and is per-process data kept in processes rather than shared tables?
