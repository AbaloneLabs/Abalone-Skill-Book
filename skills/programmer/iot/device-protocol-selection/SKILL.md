---
name: device_protocol_selection.md
description: Use when the agent is choosing a communication protocol for IoT devices or embedded systems; deciding between MQTT, CoAP, HTTP, AMQP, or custom protocols; designing MQTT topic hierarchies and QoS levels; reasoning about payload size, connection reliability, battery constraints, and bandwidth; building protocol bridges or gateways; or diagnosing dropped messages, excessive battery drain, or unreliable device connectivity. Covers QoS semantics, topic design, keepalive and session management, payload efficiency, and the tradeoffs of request-response versus publish-subscribe models on constrained networks.
---

# Device Protocol Selection

The protocol an IoT device speaks is not a networking detail — it is a decision about battery life, reliability, latency, infrastructure cost, and how the system behaves on the lossy, intermittent networks that real devices actually use. A device that wakes every minute, opens a TLS connection, posts a JSON payload over HTTP, and tears the connection down will burn most of its energy on connection overhead rather than sensing. A device that uses an unreliable protocol for a critical alarm will lose the alarm during a network blip. Agents trained on always-connected web services routinely reach for HTTP and JSON because they are familiar, then ship devices whose batteries last weeks instead of years, or whose telemetry silently disappears under packet loss. The protocol choice couples deeply to the device's hardware, power budget, and reliability requirements, and it is expensive to change after deployment.

The judgment problem is deciding which protocol's semantics match the device's constraints (battery, bandwidth, intermittent connectivity), the data's requirements (criticality, frequency, size), and the infrastructure's realities (broker availability, NAT traversal, security). The harm of a casual choice is a fleet that drains batteries, drops critical messages, or cannot be maintained at scale.

## Core Rules

### Match The Protocol To The Device's Real Constraints

The dominant protocols have sharply different profiles, and the choice should follow the device, not the developer's familiarity.

- **MQTT.** Lightweight publish-subscribe over TCP, designed for constrained devices and unreliable networks. Small header overhead, persistent sessions, retained messages, three QoS levels, and a broker model that handles fan-out. Strong default for telemetry and command-and-control when a broker is acceptable infrastructure.
- **CoAP.** UDP-based, REST-like, designed for very constrained devices on lossy low-power networks (6LoWPAN, mesh). Request-response with confirmable messages; extremely low overhead. Strong for battery devices on mesh networks where TCP's connection cost is prohibitive.
- **HTTP.** Ubiquitous and familiar, but heavy per-request (TCP + TLS handshake, verbose headers) and request-response only. Acceptable for infrequent, high-value interactions or cloud-to-cloud integration; wasteful for frequent telemetry from battery devices.
- **Custom / binary.** Minimum overhead, full control, but you own framing, reliability, security, and interoperability. Justified only when standard protocols genuinely cannot meet the constraint; otherwise you are rebuilding MQTT poorly.

Strong choice: a battery-powered sensor on a cellular network uses MQTT or CoAP to minimize connection and header overhead. Weak choice: using HTTP for minute-by-minute telemetry from a battery device because the team knows HTTP, then replacing batteries monthly. Name the binding constraint — battery, bandwidth, reliability, infrastructure — and let it drive the protocol.

### Choose QoS Deliberately Per Message Type

MQTT's QoS levels are not a setting to crank to the max; they are a per-message tradeoff between reliability, latency, and resource cost, and choosing the wrong level is a common defect.

- **QoS 0 (at most once).** Fire-and-forget. No acknowledgment, no retry. Lowest overhead; message loss is accepted. Right for high-frequency, low-value telemetry where one missed sample is harmless (temperature every second).
- **QoS 1 (at least once).** Acknowledged and retried until received, but duplicates can occur. The receiver must be idempotent. Right for telemetry where loss is unacceptable but duplicates are tolerable and deduplicated.
- **QoS 2 (exactly once).** Four-part handshake guarantees no duplicates and no loss, at higher overhead and latency. Right for commands or billing-grade events where both loss and duplication cause harm.

The recurring error is using QoS 2 everywhere "to be safe," paying high overhead on telemetry that tolerates QoS 0, or using QoS 0 for a critical alarm that then vanishes on packet loss. Assign QoS per message criticality, and ensure consumers handle the duplicate semantics of QoS 1.

### Design Topic Hierarchies And Payloads For Scale And Evolution

In a pub/sub model, the topic structure is the routing and access-control schema, and it is hard to change once devices are deployed. Design topics to be hierarchical and filterable (e.g., `tenant/site/device/sensor/measurement`), so subscribers can wildcard-subscribe at the granularity they need and access control can be applied per level. Avoid encoding volatile data (timestamps, sequence numbers) in the topic — topics are for routing, not data. Payloads should be compact and versioned: prefer a compact binary or schema-based format (Protobuf, CBOR, MessagePack) over verbose JSON for battery and bandwidth savings, and include a schema or version field so the payload can evolve without breaking consumers. A flat topic namespace or an unversioned JSON payload works in a lab of ten devices and collapses at ten thousand.

### Account For Connection Reliability And Intermittent Connectivity

IoT devices lose connectivity constantly — cellular dropouts, sleep cycles, gateway restarts, network partitions. The protocol and session design must handle this. MQTT persistent sessions let a device reconnect and resume without losing subscriptions or queued messages (the broker holds QoS 1+ messages during disconnect). Keepalive intervals must be tuned to the network: too short and the broker declares the device dead during a transient slowdown; too long and a truly dead device is detected late. For devices that sleep, design around the connection lifecycle: connect, publish, disconnect quickly, and rely on the broker's session and retained messages rather than holding connections open. A design that assumes an always-on connection works in testing and produces a fleet of devices that cannot reconnect after a network blip.

### Reason About Battery And Bandwidth As Primary Costs

For battery devices, the radio is almost always the largest power draw, and protocol overhead translates directly to battery life. The costs that matter: connection establishment (TLS handshakes are expensive on a constrained device), header overhead per message (HTTP's verbose headers dominate a small payload), and frequency of transmission. Optimizations: batch samples and send less frequently; use session resumption to avoid full handshakes; use compact payload formats; and let the device sleep with the radio off between transmissions. A device that sends a 50-byte payload with 500 bytes of HTTP header is paying ten times the necessary radio-on time. On constrained networks (LoRaWAN, NB-IoT), bandwidth and duty-cycle limits may legally cap airtime, making payload efficiency a compliance requirement, not just an optimization.

### Plan Security, Identity, And Bridging From The Start

Every device needs an identity and a secure channel, and these are protocol-adjacent decisions that are painful to retrofit. Use per-device credentials or certificates (not a shared fleet key), TLS or DTLS for transport security, and least-privilege topics so a compromised device cannot read or publish outside its scope. When devices speak one protocol (MQTT, CoAP) and cloud services speak another (HTTP, AMQP, Kafka), a bridge or gateway translates — and the bridge is a reliability, security, and semantic boundary. Design the bridge to preserve message semantics (QoS mapping is not always 1:1 across protocols), handle backpressure, and not become a single point of failure. A bridge that silently downgrades QoS 2 to QoS 0 breaks exactly-once guarantees the device thought it had.

## Common Traps

### Using HTTP For Frequent Battery-Device Telemetry

Posting JSON over HTTP every minute from a battery sensor, burning energy on connection and header overhead. Use MQTT or CoAP; reserve HTTP for infrequent or cloud-side interactions.

### Cranking QoS To Maximum Everywhere

Setting QoS 2 for all messages "to be safe," paying high overhead on telemetry that tolerates QoS 0, and adding latency where it harms UX. Assign QoS per message criticality.

### QoS 0 For Critical Alarms

Using fire-and-forget for a smoke or intrusion alarm, then losing the alarm on a packet-loss event. Critical messages need acknowledged delivery.

### Flat Or Unversioned Payloads

A JSON payload with no version field, or a flat topic namespace, that works at ten devices and cannot evolve or route at ten thousand. Design topics for filtering and access control; version payloads.

### Assuming Always-On Connectivity

Designing as if the device's connection never drops, then producing a fleet that cannot reconnect or resume sessions after the routine network blips every real device experiences. Use persistent sessions and tune keepalives.

### Shared Fleet Credentials

One key or certificate for the whole fleet, so one compromised device exposes every device. Use per-device identity and least-privilege topics.

### A Bridge That Silently Drops Semantics

A gateway that translates MQTT QoS 2 to HTTP (which has no exactly-once), breaking the guarantee the device relied on, or that becomes a single point of failure for the whole fleet. Preserve semantics and design for bridge redundancy.

### Ignoring Duty-Cycle And Bandwidth Limits

Sending large or frequent payloads on a constrained network (LoRaWAN, NB-IoT) with legal airtime limits, hitting compliance caps or getting throttled. Payload efficiency is a constraint, not an optimization, on these networks.

## Self-Check

- [ ] The protocol (MQTT, CoAP, HTTP, custom) was chosen to match the device's binding constraint — battery, bandwidth, reliability, or infrastructure — not the developer's familiarity.
- [ ] QoS level is assigned per message criticality: QoS 0 for low-value high-frequency telemetry, QoS 1 for loss-intolerant telemetry with idempotent consumers, QoS 2 for commands/events where duplication also causes harm.
- [ ] Topic hierarchies are designed for filtering and access control (tenant/site/device/...), volatile data is in the payload not the topic, and payloads are compact and versioned for evolution.
- [ ] Connection reliability is designed for: persistent sessions, tuned keepalives, quick connect-publish-disconnect for sleeping devices, and graceful reconnection after drops.
- [ ] Battery and bandwidth costs were analyzed: connection/handshake overhead, header-to-payload ratio, transmission frequency, and sleep behavior are optimized for device lifetime; on constrained networks, duty-cycle/airtime limits are respected.
- [ ] Security is designed in from the start: per-device identity and credentials, TLS/DTLS transport, least-privilege topic scoping so a compromised device is contained.
- [ ] Protocol bridges or gateways preserve message semantics across the translation (QoS mapping is explicit), handle backpressure, and are not a single point of failure.
- [ ] A test under realistic conditions (packet loss, intermittent connectivity, sleep cycles, broker restart) confirmed no silent message loss on critical paths and acceptable battery behavior.
- [ ] The design was projected to fleet scale (many devices, shared broker, wildcard subscriptions) and the topic structure, payload versioning, and access control hold up.
