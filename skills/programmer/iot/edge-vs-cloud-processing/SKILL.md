---
name: edge_vs_cloud_processing.md
description: Use when the agent is designing an IoT, embedded, or distributed system and deciding where computation runs — on the device, on a local edge gateway, on a regional edge cluster, or in the cloud; reasoning about latency, bandwidth, privacy and data locality, offline operation, model deployment and update, cost allocation, or fault isolation; architecting a tiered processing pipeline; or diagnosing a system that is too latency-sensitive for the cloud, too bandwidth-hungry, or too fragile when connectivity drops. Covers the tradeoffs of edge vs cloud, hybrid tiering, model lifecycle at the edge, and the failure-mode differences between connected and autonomous operation.
---

# Edge Vs Cloud Processing

Where computation runs is one of the most consequential architectural decisions in a distributed system, and it is not a binary "edge or cloud." It is a tiering decision: each piece of data and each piece of logic has a natural home based on latency sensitivity, bandwidth cost, privacy constraints, offline requirements, and the economics of compute placement. Agents routinely default to "send everything to the cloud and process it there" because that is the familiar model and it works in a demo — then ship systems that cannot meet a latency target because of round-trip time, that drown a cellular uplink in raw sensor data, that fail completely when connectivity drops, or that violate data-locality requirements by shipping sensitive data across borders. The judgment problem is deciding, for each data flow and each computation, which tier it belongs on, and designing the system so the tiers cooperate rather than assuming permanent cloud connectivity.

The harm of a wrong tiering decision is structural and late-appearing: a latency budget blown by a round trip that "should have been fast," a bandwidth bill dominated by raw data that could have been compressed to events at the source, a system that is useless during the network outages every real deployment experiences. Getting tiering right requires reasoning about the full pipeline from sensor to decision, not treating the cloud as the default and the edge as an optimization.

## Core Rules

### Place Computation By Latency Requirement First

Latency is the tiering criterion that cannot be worked around, because the speed of light and network hops are physics, not configuration. A control loop that must react in 10 ms cannot round-trip to a cloud region, no matter how fast the cloud is; the answer must be computed where the sensor is or one hop away. A user-facing dashboard that refreshes in seconds can tolerate a cloud round trip. For each computation, name its latency budget and where it must be satisfied:

- **Sub-millisecond to a few ms.** Must run on the device or a directly-attached controller (motor control, collision avoidance).
- **Tens to hundreds of ms.** Local edge gateway or on-premises cluster (factory-floor coordination, local alerting).
- **Seconds to minutes.** Cloud is acceptable (analytics, reporting, model training).

The recurring error is placing a latency-sensitive computation in the cloud because the cloud has more compute, then missing the budget by the round-trip time alone. Place by latency first; let other concerns (compute capacity, cost) adjust within the latency constraint, not override it.

### Reduce Bandwidth By Processing At The Source

Raw sensor streams are almost always far larger than the information they carry. A camera that streams full video to the cloud to detect whether a person is present wastes enormous bandwidth transmitting frames when the answer is a single event per second. A vibration sensor that streams raw waveforms to detect an anomaly could send only the anomaly. The principle: process at the source to extract events, features, or aggregates, and transmit only what carries information. This is not just cost optimization — on constrained uplinks (cellular, satellite, low-power mesh), raw streaming may be physically impossible or prohibitively expensive. For each data flow, ask: what does the consumer actually need, and how much can be computed and discarded at the edge before transmission? The transform from raw stream to meaningful event is the highest-leverage edge computation in most IoT systems.

### Design For Offline And Degraded Operation

Real deployments lose connectivity — cellular dropouts, gateway failures, cloud outages, intentional air-gaps. A system that requires cloud connectivity to function is fragile by construction. Decide, per function, what must work offline and what may degrade:

- **Safety-critical functions.** Must operate fully autonomously, with no cloud dependency. A machine guard, a medical alarm, a vehicle brake — these cannot wait for a cloud round trip or fail when connectivity drops.
- **Core operational functions.** Should operate offline with local logic, queuing data to sync when connectivity returns. The system keeps working; only cloud-side analytics lag.
- **Non-critical functions.** May degrade or pause (a cloud dashboard showing "data stale" during an outage).

The design that follows: local state and logic for the must-work functions, store-and-forward for data that needs to reach the cloud eventually, and explicit degraded-mode behavior rather than silent failure. A system whose safety function depends on a cloud call is a system that fails dangerously during the outage that will eventually occur.

### Handle Privacy And Data Locality As A First-Class Constraint

Some data cannot or should not leave its origin: regulated personal data, proprietary process data, data subject to residency or sovereignty laws, or simply data whose transmission creates risk without benefit. The edge is where privacy-preserving computation lives — infer locally and transmit only the result, or aggregate to anonymity before sending. For each data category, decide: must this stay on-device, may it reach a local edge but go no further, or may it go to the cloud? Treat data locality as a hard constraint that shapes tiering, not a compliance afterthought. A design that ships raw video of people to the cloud for inference, when local inference would suffice, creates privacy and regulatory exposure that local processing avoids entirely.

### Plan The Model And Logic Lifecycle Across Tiers

When logic or ML models run at the edge, they become a deployment and update problem: how does a new model or new logic reach thousands of edge nodes, how do you know it is running, and how do you roll it back? Edge deployment is harder than cloud deployment because the nodes are remote, intermittently connected, resource-constrained, and may be running older versions. The disciplines: version all logic and models, deploy in stages (canary a few nodes before the fleet), monitor which versions are running where, and have a rollback path (see the OTA skill for the device-side mechanics). A model trained in the cloud and deployed to the edge must also handle drift — the edge environment differs from the training distribution, and a model that worked in validation may degrade in the field. Plan for monitoring and retraining, not a one-time deployment.

### Allocate Cost Deliberately Across Tiers

Compute and bandwidth cost differently at different tiers, and the total cost depends on where work is done. Cloud compute is cheap per unit but you pay for bandwidth in and out and for continuous running; edge compute is a capital cost (the device) but local communication is free. For a system processing high-volume data, doing the work at the edge can be dramatically cheaper than paying to transmit and store raw data in the cloud, even if the edge hardware costs more up front. For low-volume, bursty workloads, cloud elasticity wins. Reason about the total system cost — hardware, bandwidth, storage, compute, and operations — across the tiers, not the cost of any single tier in isolation. A design that is "cheap" because it uses free cloud tiers may be paying in bandwidth what it saved in compute.

### Isolate Faults Across Tiers

Each tier can fail independently, and the system's behavior under partial failure is a design property, not luck. A cloud outage should not brick edge devices; an edge gateway failure should not lose safety functions (they run on the device); a device failure should not flood the cloud with alerts or stall the pipeline. Design fault isolation: loose coupling between tiers, idempotent and resumable sync, bounded retry and backoff so a failed tier is not hammered, and clear ownership of each function so a failure in one tier has a defined fallback rather than cascading. The question to ask of each tier boundary: what happens to the rest of the system when this tier is down, slow, or returning wrong answers? A well-tiered system degrades gracefully; a poorly-tiered one cascades.

## Common Traps

### Defaulting To Cloud For Everything

Sending all data and logic to the cloud because it is familiar, then missing latency targets, drowning bandwidth, and failing during outages. Place by latency and bandwidth first; let cloud handle what genuinely belongs there.

### Streaming Raw Data Instead Of Events

Transmitting raw sensor streams when the consumer needs only derived events, paying bandwidth and storage for information that was discarded at the consumer anyway. Process at the source.

### Requiring Cloud For Safety-Critical Functions

Designing a guard or alarm that depends on a cloud round trip, so it fails dangerously during the outage that will occur. Safety functions run locally and autonomously.

### Treating Privacy As A Compliance Afterthought

Shipping raw personal or regulated data to the cloud for processing, then bolting on controls, when local inference would have avoided the exposure. Make data locality a tiering input.

### One-Shot Edge Model Deployment

Deploying a model to the edge once, with no versioning, monitoring, or rollback, then watching it silently drift as the field environment diverges from training. Version, stage, monitor, and plan for retraining.

### Ignoring Cost Across Tiers

Optimizing one tier's cost (cheap cloud compute) while ignoring another (expensive bandwidth), so the total is worse. Reason about total system cost across hardware, bandwidth, storage, and operations.

### Tight Coupling That Cascades Failures

Designing tiers that depend on each other synchronously, so a slow or failed cloud takes down edge functions, or a failed edge floods the cloud. Decouple tiers and define fallback behavior at each boundary.

### Assuming Connectivity For Core Functions

Building operational functions that stall when connectivity drops, rather than queuing and resuming. Core functions work offline; only cloud-side analytics lag.

## Self-Check

- [ ] Each computation is placed by its latency requirement first (device for sub-ms, edge for tens-hundreds of ms, cloud for seconds+); latency budgets are not blown by round trips.
- [ ] High-volume data flows are processed at the source into events, features, or aggregates before transmission; raw streaming is reserved for cases where the consumer truly needs the raw signal.
- [ ] Safety-critical functions run fully autonomously with no cloud dependency; core operational functions work offline with store-and-forward; only non-critical functions may degrade during outages.
- [ ] Privacy and data locality are tiering inputs: data that must stay local is processed at the edge, and only results or anonymized aggregates leave the source.
- [ ] Edge logic and models are versioned, deployed in stages with canaries, monitored for running version and drift, and have a rollback path; retraining is planned, not one-shot.
- [ ] Total system cost across tiers (hardware, bandwidth, storage, compute, operations) was analyzed; no tier is optimized in isolation in a way that raises total cost.
- [ ] Fault isolation is designed at each tier boundary: loose coupling, idempotent resumable sync, bounded retry/backoff, and defined fallbacks so a failed tier degrades rather than cascades.
- [ ] A test under partial failure (cloud outage, edge gateway failure, device failure, network partition) confirmed the system degrades gracefully and safety functions continue.
- [ ] The tiering decision was documented per data flow and per computation, so future changes reason about placement rather than defaulting back to cloud.
