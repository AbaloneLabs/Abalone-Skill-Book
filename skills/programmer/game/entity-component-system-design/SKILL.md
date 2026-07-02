---
name: entity_component_system_design.md
description: Use when the agent is architecting a game engine, simulation, or any system with many heterogeneous entities; choosing between inheritance and composition/ECS; designing component storage (archetypes, sparse sets, bitsets); ordering systems and their dependencies; reasoning about cache locality and data-oriented design; managing entity lifecycles (creation, deferred deletion, component add/remove); or diagnosing performance problems rooted in pointer chasing, cache misses, or system ordering bugs. Covers the tradeoffs of archetype vs sparse-set storage, system parallelism, query design, and when ECS is the wrong abstraction.
---

# Entity Component System Design

An Entity Component System is not "the modern way to structure a game." It is a specific data-organization choice that trades the familiarity of object-oriented hierarchies for cache-friendly batch processing of homogeneous data. The decision matters because it reshapes how the whole codebase reasons about entities: what an entity *is* (an ID, not an object), where its data lives (contiguous component arrays, not fields on a struct), and how behavior runs (systems that iterate queries, not methods on instances). Agents often reach for ECS because it is fashionable, then build it as "classes but called components" — retaining pointer-heavy, allocation-heavy, inheritance-flavored patterns that defeat the entire purpose.

The judgment problem is deciding whether ECS fits the workload, how to lay out component storage for the access patterns that matter, how to order and parallelize systems without hidden dependencies, and how to manage entity lifecycles without invalidating iterators or leaking references. The harm of getting this wrong is silent: the game runs, but frame time is dominated by cache misses, systems interact through subtle ordering bugs, and adding a feature requires touching the core because the data model fights the feature.

## Core Rules

### Decide Whether ECS Fits Before Adopting It

ECS shines when the workload is "do the same thing to many entities with the same components" — particle systems, unit AI, physics bodies, rendering batches. It pays its cost when entities are few and behavior is highly individual, or when the data model is naturally hierarchical (a UI tree, a scene graph where parent-child transforms dominate). Adopting ECS for a UI toolkit or a deeply nested document editor adds indirection and iteration overhead without the batch-processing benefit. Name the dominant access pattern first: if it is "iterate all entities with {Position, Velocity} and update Position," ECS is a strong fit; if it is "walk this tree and propagate parent state to children," a tree with cached transforms may be simpler and faster.

### Treat Entities As IDs, Not Objects

An entity is a lightweight, opaque identifier — typically an integer, often paired with a generation counter to detect stale references. It has no behavior and no data of its own; its data lives in component stores keyed by entity ID. This is the foundation of data-oriented design: behavior is decoupled from identity, and systems operate on the data, not on "the entity." Resist the urge to add methods or state to the entity itself; an entity that grows behavior is just an object with extra indirection. The ID-plus-generation pattern exists specifically to make dangling references detectable — a deleted entity's ID, when reused, must not be mistaken for the old entity.

### Choose Component Storage Based On Query And Mutation Patterns

The storage layout is the performance-critical decision, and the two dominant strategies have opposite strengths.

- **Archetype storage** groups entities that share the same set of components into contiguous memory blocks. Iterating "all entities with {Position, Velocity}" is a tight linear scan of packed arrays — excellent cache locality and the reason archetype ECS is fast for homogeneous batches. The cost: adding or removing a component moves the entity to a different archetype block (a copy), and queries that need "entities with Position but *without* Health" must skip archetypes or maintain index structures.
- **Sparse-set storage** keeps one set per component type, each mapping entity IDs to component data. Adding/removing a component is cheap (set membership change), and arbitrary component combinations are natural. The cost: iterating a multi-component query intersects sets, and component data for one entity is scattered across multiple sets, hurting locality.

Strong choice: a particle simulation with millions of identical entities uses archetype storage for maximum locality. Weak choice: using archetype storage for a game where entities constantly gain and lose components (status effects, equipment), thrashing the archetype graph with copies. Match the storage to whether the workload is "stable component sets, heavy iteration" (archetype) or "fluid component sets, moderate iteration" (sparse set).

### Make System Order Explicit And Dependency-Driven

Systems are the unit of behavior, and their order is a correctness concern, not a convenience. If `MovementSystem` writes `Position` and `CollisionSystem` reads it, the order matters; if both run and the order is unspecified, the result is nondeterministic. Specify system dependencies explicitly (declare reads and writes, let a scheduler topologically sort) rather than relying on registration order or a manually maintained list that drifts. The common defect is two systems that both mutate the same component with an implicit ordering that breaks the day a third system is inserted between them.

Parallelism multiplies this: systems with non-overlapping component access can run concurrently, but the scheduler must know the access sets to schedule safely. Declaring access (read/write per component) is what enables both correct ordering and safe parallelism; a system that reaches into a global registry or a singleton outside its declared query breaks the scheduler's assumptions silently.

### Design Queries To Express The Real Access Set

A query like "all entities with Position" is rarely correct; the real intent is usually "all entities with Position *and* Velocity, excluding those that are Static or Dead." Under-specifying queries (querying too broadly) forces the system to skip entities inside the loop and wastes work; over-specifying (querying a narrow archetype when the intent is broader) silently drops entities that should be processed. Express exclusion and optional components explicitly in the query rather than branching inside the loop. The query is the contract between a system and the data — get it precise, and the system runs fast and correct; leave it loose, and the system does extra work or misses entities.

### Manage Entity Lifecycle With Deferred Operations

Deleting an entity or adding/removing components *during* a system's iteration invalidates the very structures being iterated — archetype blocks shift, sparse sets change size, iterators dangle. Robust ECS implementations defer structural changes: queue deletions and component changes during the loop, and apply them after the system (or the whole frame) completes. The rule: never mutate the entity-component graph while iterating it. This also makes lifecycle deterministic — deferred deletions apply in a known order, not interleaved with logic that may read a half-deleted entity. Reserve immediate structural change for the rare cases where it is provably outside any iteration.

### Respect Cache Locality In Component Design

The performance win of ECS comes from contiguous, homogeneous data. A component that holds a pointer to a heap-allocated object, a string, or a nested structure defeats the win — iterating it is pointer-chasing again. Design components as flat, packed data (vectors, numeric fields, small fixed arrays). Split hot data (iterated every frame: Position, Velocity) from cold data (rarely accessed: Description, Lore) into separate components, so the hot loop does not drag cold bytes through the cache. This "hot/cold splitting" is the single most impactful data-oriented refactor available.

## Common Traps

### Using ECS As Renamed OOP

Defining a `Creature` component that holds health, inventory, AI state, and a virtual `update()` method, then "iterating creatures." This is a god-object with extra indirection — no locality, no batch benefit, all the OOP coupling. Components should be small data slices; behavior belongs in systems.

### Adding A Component For Every Concept

Creating `Health`, `MaxHealth`, `HealthRegen`, `HealthBar`, `HealthHistory` as separate components when they are always used together and always iterated together. Over-granular components fragment memory and complicate queries; merge components that change together and are queried together.

### Ignoring System Order Until It Breaks

Registering systems in an ad-hoc order, getting correct behavior by luck, then watching it break when a system is added or reordered. Order is a dependency; declare it, and let the scheduler enforce it. The bug "physics works until I add the audio system" is almost always a hidden ordering dependency.

### Mutating The Graph During Iteration

Deleting an entity mid-loop because its health hit zero, then dereferencing it later in the same loop. The deletion either corrupts the iterator or silently skips the next entity. Defer structural changes to a safe point.

### Pointer-Heavy Components That Destroy Locality

A `Mesh` component holding a `Box<dyn RenderData>` or a `String` handle that indirects to the heap. The "ECS" iteration becomes a tour of scattered allocations, no faster than the OOP it replaced. Keep hot components flat; push indirection into cold, rarely-iterated components.

### Assuming Sparse Sets And Archetypes Are Interchangeable

Picking archetype storage because "it's the fast one," then building a game where entities constantly change component sets, paying a copy on every status-effect toggle. Or picking sparse sets for a million-particle simulation and wondering why iteration is slow. The storage strategy is a workload-dependent decision, not a default.

### Stale Entity References From ID Reuse

Reusing a freed entity ID immediately, so a system holding the old ID now points at a brand-new entity and corrupts it. Generation counters (or never reusing IDs) exist precisely to prevent this; omitting them turns entity deletion into a latent corruption bug.

## Self-Check

- [ ] ECS was adopted because the dominant workload is batch iteration over many homogeneous entities — not because it is fashionable; hierarchical or few-entity workloads were considered and rejected with reason.
- [ ] Entities are opaque ID-plus-generation handles with no behavior or data of their own; components are flat data slices, not god-objects.
- [ ] Component storage (archetype vs sparse-set) was chosen to match the workload: archetype for stable component sets with heavy iteration, sparse-set for fluid component sets with moderate iteration.
- [ ] System order is declared via explicit read/write dependencies and resolved by a scheduler, not by registration order; parallel systems have non-overlapping declared access.
- [ ] Queries express the precise access set including exclusions and optional components, rather than over-broad queries with in-loop skipping.
- [ ] Structural changes (entity deletion, component add/remove) are deferred to a safe point outside iteration, never applied mid-loop.
- [ ] Hot components are flat and packed; cold data is split into separate components so hot loops do not drag cold bytes through cache; pointer-heavy indirection is confined to rarely-iterated components.
- [ ] Entity ID reuse is guarded by generation counters (or IDs are never reused), so stale references are detected rather than silently corrupting new entities.
- [ ] A profiling pass confirmed the hot loop is bound by computation, not by cache misses or pointer-chasing from poor component layout.
