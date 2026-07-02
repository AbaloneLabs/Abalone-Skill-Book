---
name: cloud_service_selection.md
description: Use when the agent is choosing which managed cloud service to use for a workload, comparing managed versus self-hosted options, assessing vendor lock-in and portability, selecting a region or availability zone for data sovereignty and latency, reading availability SLAs and computing composite SLA, checking service quotas and limits that constrain a design, weighing preview versus GA service risk, or planning migration cost between services. Also covers capability fit, architecture fit, data export friction, abstraction layers for portability, paired regions, and service availability per region. Use before provisioning a new managed database, queue, cache, object store, streaming, search, or compute service, or when an architecture review asks whether the chosen service is the right one for the need.
---

# Cloud Service Selection

Choosing a cloud service is the decision that fixes the most downstream constraints for the least immediate effort. A few hours of selection work locks in the data model, the API surface, the scaling ceiling, the operational responsibility boundary, the failure modes, and the exit cost for years. Because the choice feels easy — pick the service whose marketing matches the workload — agents reach for the first plausible option, copy a default from a tutorial, or inherit whatever the team used last, and only later discover the constraints that mattered: a quota that caps the design, an SLA that excludes the failure mode that actually occurs, a region that lacks the service, a managed offering that cannot be tuned to the workload, or a proprietary API whose data cannot be exported without a rewrite.

The judgment problem is that "which service" is never one question. It is a bundle of capability fit, operational burden, control, portability, geography, limits, reliability guarantees, and exit cost — and the right answer depends on which of those will actually bite this specific workload. A startup prototype and a regulated production system answer differently even for the same workload type. The harm of wrong selection is large and delayed: a service that works in dev fails under the region's real constraints in production, a managed service that cannot be tuned forces a re-platforming under load, a proprietary choice turns a migration into a multi-quarter project, or a preview service is deprecated just as the system depends on it. This skill is about choosing the right service for the capability and architecture need. It deliberately does not cover spending less — that is a separate skill. Cost here is a constraint among several, not the objective.

## Core Rules

### Decide Managed Versus Self-Hosted As An Explicit Trade Of Control For Operational Burden

The first and most consequential choice is whether to run a managed service the provider operates, or to operate the software yourself on infrastructure you control. The two are not interchangeable; they trade different things and agents often make the trade implicitly by defaulting to managed.

- **Managed services shift operational burden to the provider** — patching, backups, upgrades, high availability, scaling, and uptime are the provider's responsibility within the service's defined boundary. You gain velocity and a smaller on-call surface, and you lose control over the internals: tuning parameters, upgrade timing, version pinning, plugin or extension support, and the ability to fix a bug yourself.
- **Self-hosted (including self-managed on VMs/containers, or a vendor's "bring your own" edition) keeps control** — you tune, patch, version, extend, and repair it, and you own the failure modes. The cost is the standing operational team and toil required to run it at the reliability you promise. A self-hosted database run by a team without a dedicated operator is almost always less reliable than the managed equivalent, because the managed provider has done the reliability work at scale.
- **Match the choice to the team's actual capacity, not to the workload's theoretical needs.** A workload that needs exotic tuning, a specific version, or an extension the managed service forbids is a real reason to self-host — but only if the team can sustain the operations. A workload that fits comfortably inside a managed service's supported configuration should use the managed service; running it yourself to "save" control you will never exercise is borrowing incidents.

The strong choice is to default to managed when the workload fits the managed service's supported shape, and to self-host only when a concrete requirement (tuning, version, extension, data residency the managed service cannot meet, or cost at a scale where self-hosting crosses over) justifies the operational burden. The weak choice is to self-host because it feels more powerful, or to go managed without checking whether the service supports the configuration the workload needs.

### Assess Vendor Lock-In By Data Export Friction And Migration Cost, Not By Label

Every managed service creates some lock-in; the question is how much, of what kind, and whether it is justified. Lock-in is not a binary and it is not avoided by choosing "open" software — it is a function of how expensive it is to leave.

- **Measure lock-in by exit cost.** Ask concretely: if we had to move off this service in a year, what would it cost in engineering time, data conversion, downtime, and re-training? A managed Postgres has low lock-in because the data and SQL are portable and competing managed Postgres offerings exist. A proprietary document/graph/vector database with a bespoke query language and no standard export has high lock-in because leaving means rewriting queries and re-shaping data.
- **Data export friction is the strongest signal.** Can you export your data in a standard, complete, lossless format on demand? If the service makes export slow, partial, priced per-GB, or requires a proprietary tool, the lock-in is high regardless of the marketing. A service you can fully and cheaply export is one you can leave.
- **Proprietary APIs and SDKs raise lock-in in proportion to how much of your code depends on them.** A few calls to a queue or object API are tolerable; a deep dependency on a proprietary query language, proprietary indexing, or proprietary orchestration primitives is structural lock-in that survives any later desire to move.
- **Justify lock-in with capability you cannot get otherwise.** Lock-in is acceptable when the service provides a capability, scale, or reliability that the portable alternatives cannot match, and that capability is worth the exit cost. Lock-in for mere convenience, with a portable alternative available, is usually a bad trade.

Portability strategies reduce but do not eliminate lock-in: abstraction layers (a repository interface over the data API, a standard protocol like S3 or Kafka or SQL, an ORM, a Terraform provider that targets multiple clouds) confine the proprietary surface to a bounded module so a future migration touches one layer, not the whole system. The trap is over-abstracting — building a thick portability layer "in case we move" for a move that never happens, paying complexity forever for an option you never exercise. Abstract where a move is genuinely plausible or contractually required; do not abstract speculatively.

### Select Region And Availability Zone For Data Sovereignty, Latency, And Service Availability Together

Region selection is not "pick the nearest one." It is a joint optimization over legal constraints, user latency, service catalog, and resilience pairing, and getting any one of them wrong invalidates the others.

- **Data sovereignty and compliance set hard constraints first.** Regulations (data residency, sector rules, contractual obligations) may require data to stay in a country or region. A region that violates these is not an option regardless of latency or price. Establish the legal floor before optimizing anything else.
- **Latency to users drives the experience.** For user-facing workloads, the region's distance to the majority of users sets a floor on round-trip latency that no amount of tuning removes. Measure from the real user geography, not from your office. For background workloads latency matters less and other factors dominate.
- **Service availability varies per region.** Not every managed service, instance family, or feature is available in every region, and new features land in a few regions first. A region chosen for latency that lacks the service you depend on forces a compromise — a different service, a wait, or a multi-region split. Check the service catalog for the candidate region before committing.
- **Paired regions and availability zones structure resilience.** Providers pair regions for disaster recovery (with preferred pairing for replication and distance tuned for correlated failure). Availability zones within a region give independent power, cooling, and network for high availability. Decide whether your resilience need is zonal (a zone fails, common) or regional (a whole region fails, rare but catastrophic), and choose a region that offers the zones and the paired region your design needs.
- **Beware region-scoped managed services and single-zone defaults.** Some managed services are regional and self-healing across zones; others default to a single zone and require explicit configuration for multi-AZ. Read the deployment topology, not the marketing, to know what actually fails over.

The strong choice is to let sovereignty set the floor, then optimize latency to users among compliant regions, then confirm the service catalog and the zone/paired-region topology support the reliability design. The weak choice is to pick a region for price or familiarity and discover the constraint in production.

### Read The Real SLA, Then Compute The Composite SLA

The published SLA is a contract term, not a reliability guarantee, and agents frequently misread it in two directions — treating it as a promise of uptime it does not make, or ignoring the composite effect that makes a system's real SLA far worse than any component's.

- **Read what the SLA excludes.** SLAs define the conditions under which they apply and exclude many real failure modes: planned maintenance windows, dependent-service outages, failures caused by your configuration, throttling from exceeded quotas, preview/beta services, and often entire categories like "internet health" or "DNS." A service with a 99.99% SLA that excludes the failure mode you actually fear gives you nothing for that fear. The exclusions matter more than the headline number.
- **Understand what an SLA is and is not.** It is typically a service credit mechanism — if the provider misses the number, you get a partial bill credit, not restored availability or compensation for your business loss. It is not a guarantee your system will be up, and the credit is usually capped well below the cost of your outage. Treat the SLA as a signal of the provider's confidence and investment, not as risk transfer.
- **Compute the composite SLA for multi-service architectures.** When availability depends on several independent services in series (request must traverse load balancer, compute, database, cache), the composite availability is roughly the product of the components, and it is always worse than the weakest single component when dependencies stack. A system of four 99.9% components in series is about 99.6% — roughly 3.5 hours of downtime a month, not the 43 minutes a single 99.9% implies. Redundancy and fallbacks change the math (parallel paths multiply availability upward), so model the real dependency graph, including the dependencies the SLA excludes.
- **Derive the SLA you need from the business requirement, then verify the architecture can meet it.** Start from the tolerated downtime and data loss for this system, translate to an availability percentage, and check whether the composite of the chosen services — counting their exclusions — can plausibly meet it. If not, add redundancy, change the architecture, or reset the business expectation. Do not pick services first and hope the SLA works out.

### Check Service Quotas And Limits Before They Constrain The Design

Every managed service ships with default quotas and hard limits — on resources, throughput, concurrency, request rate, number of entities, payload size, and configuration values — and these are discovered at the worst time if not checked up front. Quotas are not minor footnotes; they cap the design.

- **Default quotas are conservative and requestable, but raises take time and approval.** The default limit on accounts, partitions, connections, or requests-per-second is often far below what a production workload needs, and a quota increase is a support ticket with lead time. Discovering the cap during a launch forces either a delay or a degraded design.
- **Hard limits are not raisable and force architectural choices.** Some limits (maximum record size, maximum query timeout, maximum number of indexes, maximum retention, maximum fan-out) are fixed by the service design and cannot be lifted. A workload that needs to exceed a hard limit must be reshaped around it — sharding, partitioning, or a different service.
- **Map the workload's projected scale against the limits early.** Estimate peak entities, peak throughput, peak concurrency, and largest payloads, and compare to the service's documented limits before committing. A service that fits the workload at launch but cannot hold the projected peak is the wrong service.
- **Account for throttling as a design input.** When a service throttles on exceeded limits, your system must handle backpressure, retries, and degradation — and the retry behavior itself can amplify load. Quota-driven throttling is a real failure mode that the SLA often excludes.

### Treat Data Transfer And Movement Cost As A Selection Constraint

This skill is not about spending less, but data movement cost is a selection factor because it changes which service is architecturally correct, independent of how cheaply you run it. (Reducing an existing transfer bill is the cost-optimization skill's concern.)

- **Co-locate data with its compute and consumers.** A service chosen in one region whose data is consumed in another generates recurring cross-region transfer that may dwarf the service's own price. Selecting the service and the region together — so data, compute, and consumers sit together — is a selection decision, not a later optimization.
- **Some services charge for the movement itself** (ingest APIs, cross-region replication, backup restore egress, data scan on large datasets). A service whose pricing model charges per-GB-moved can be the wrong selection for a workload that moves data constantly, even if its storage price is attractive.
- **Migration between services has a one-time transfer cost** that should be estimated before selection, not after. Moving terabytes out of one managed store into another can cost more in egress than either service saves, and that exit cost is part of the lock-in assessment above.

### Weigh Migration Cost And Preview Risk As Part Of The Selection

Two selection factors are routinely underestimated: the cost of moving *between* services later, and the risk of choosing a service that is not yet stable.

- **Migration cost between services is often larger than the original build.** Selecting a service you may need to leave means accepting that future migration will require schema conversion, data backfill, dual-running, cutover, and rollback planning. Prefer services with standard interfaces and documented migration paths so that a future change is bounded; treat a service with no migration story as a one-way door.
- **Preview, beta, and "latest generation" services carry real risk.** They may be deprecated, have their API changed, lack SLAs, have incomplete features, or have undocumented limits. Using a preview service in production means accepting that the provider can change or withdraw it. Use preview services when the capability is essential and unavailable in GA, with an isolation layer and a fallback plan; do not use them for core stateful systems where deprecation would be catastrophic, and never assume a preview service has the same reliability or support as GA.

## Common Traps

### Picking The Service Whose Marketing Matches The Workload

Selecting a managed database, queue, or search service because its landing page describes the use case, without checking quotas, tuning limits, region availability, or the real SLA. Marketing describes capability; the constraints that bite live in the limits and exclusions. Read the service's quotas, deployment topology, and SLA exclusions before committing.

### Defaulting To Managed Without Checking Configuration Support

Choosing the managed edition for velocity, then discovering it forbids the extension, the version, the tuning parameter, or the authentication mode the workload needs — forcing either a self-hosting pivot under pressure or a degraded design. Confirm the managed service supports the required configuration before selecting it; managed is not always a superset of self-hosted capability.

### Treating The Headline SLA Number As The System's Reliability

Reading "99.99%" and designing as if the system will be up that much, ignoring the exclusions and the composite effect of stacked dependencies. A multi-service architecture's real availability is the composite, counting exclusions, and it is almost always lower than the headline. Compute the composite from the dependency graph and the business downtime tolerance.

### Ignoring Region Service Availability And Pairing

Choosing a region for latency or price, then discovering the managed service or instance family is unavailable there, or that the region lacks a paired region for the DR design. Check the service catalog and the region's pairing before selecting the region; a region without the service or the resilience topology is not viable.

### Measuring Lock-In By "Open" Label Instead Of Exit Cost

Believing a service is low-lock-in because it is built on open-source software, while the data model, query language, or operational tooling is proprietary and export is slow or lossy. Measure lock-in by concrete exit cost — can you export your data, in a standard format, completely, cheaply, on demand — not by the openness of the underlying engine.

### Over-Abstracting For Portability That Is Never Used

Building a thick abstraction layer over a managed service "in case we move clouds," paying permanent complexity and performance overhead for a move that never happens. Abstract where a move is genuinely plausible or required; otherwise confine the proprietary surface to a thin, well-bounded module rather than a speculative portability framework.

### Hitting A Hard Limit At Peak And Reshaping Under Load

Selecting a service without mapping projected peak entities, throughput, or payload size against its hard limits, then hitting a non-raisable cap during a peak and being forced to shard or replatform under pressure. Estimate peak scale against documented limits before selection; a service that cannot hold the projected peak is the wrong service.

### Trusting A Preview Service As If It Were GA

Building a core system on a preview or beta managed service because it has an attractive feature, then suffering an API change, a deprecation, or an unmet reliability expectation. Use preview services only when the capability is essential and unavailable in GA, behind an isolation layer with a fallback; treat them as changeable and withdrawable.

### Selecting Region And Compute Separately

Picking a managed service in one region and compute in another, then paying recurring cross-region transfer and latency that makes the architecture uneconomic regardless of unit price. Select service, region, and compute placement together so data and consumers are co-located; transfer cost is a selection constraint, not a later optimization.

### Underestimating The Cost Of Leaving

Choosing a service with no migration path because it is convenient now, then facing a multi-quarter migration involving schema conversion, backfill, and dual-running when requirements change. Treat migration cost as part of selection — prefer services with standard interfaces and documented migration paths, and treat no-migration-story services as one-way doors.

## Self-Check

- [ ] The managed-versus-self-hosted decision was made explicitly: managed is the default where the workload fits the service's supported configuration, and self-hosting is justified by a concrete requirement (tuning, version, extension, residency) that the team can sustainably operate — not chosen for perceived power or by default.
- [ ] Vendor lock-in was assessed by concrete exit cost, not label: the data can be exported in a standard, complete, lossless format on demand, the proprietary API surface is bounded, and any abstraction layer exists only where a move is genuinely plausible rather than speculatively for portability never used.
- [ ] Region and availability-zone selection satisfied data sovereignty first, then optimized latency to the real user geography, then confirmed the service catalog and the zone/paired-region topology support the reliability design — no region was chosen for price or familiarity alone.
- [ ] The real SLA was read including its exclusions (maintenance, dependent outages, configuration-caused failures, throttling, preview services), and the composite SLA was computed across the actual dependency graph and compared against the business downtime tolerance; the architecture can plausibly meet the required availability.
- [ ] Service quotas and hard limits were checked against projected peak scale (entities, throughput, concurrency, payload size, retention) before selection; raisable defaults were identified with their lead time, and non-raisable hard limits were confirmed not to constrain the design.
- [ ] Data transfer and movement were treated as a selection constraint: data, compute, and consumers are co-located in region, services whose pricing charges per-GB-moved were evaluated against the workload's movement pattern, and one-time migration transfer cost was estimated before committing.
- [ ] Migration cost between the chosen service and alternatives was estimated and found acceptable; services with no migration path were treated as one-way doors and used only where justified.
- [ ] Preview, beta, or latest-generation services are used only where the capability is essential and unavailable in GA, behind an isolation layer with a documented fallback — not for core stateful systems where deprecation would be catastrophic, and never assumed to carry GA reliability or support.
