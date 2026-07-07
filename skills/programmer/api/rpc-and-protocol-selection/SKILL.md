---
name: rpc_and_protocol_selection.md
description: Use when the agent is choosing an RPC or communication protocol for service-to-service or client-server communication — REST, gRPC, GraphQL, GraphQL, Thrift, message queues, WebSocket, server-sent events; deciding between synchronous and asynchronous communication; choosing a serialization format (JSON, Protobuf, Avro); or evaluating protocol tradeoffs (text vs binary, schema-driven vs schemaless, bidirectional vs request-response). Covers the protocol decision by workload characteristics (request-response vs streaming, internal vs external, schema needs, performance, browser support), serialization format tradeoffs, sync vs async communication, and the principle that protocol choice constrains the API's capabilities and ecosystem.
---

# RPC And Protocol Selection

The choice of communication protocol — REST, gRPC, GraphQL, a message queue, WebSocket — is a foundational decision that constrains an API's capabilities, performance, ecosystem, and fit for the workload. Each protocol embodies a set of tradeoffs: REST (HTTP/JSON) is ubiquitous, human-readable, and browser-native, but lacks a strong schema and efficient binary encoding; gRPC (HTTP/2 + Protobuf) is strongly typed, efficient, and supports bidirectional streaming, but is not browser-native without a gateway; GraphQL offers flexible client-driven queries, but adds server complexity; message queues enable asynchronous decoupling, but are not request-response. The protocol is not a matter of "use REST" or "use the newest thing"; it is a match between the workload's characteristics (request-response vs streaming, internal vs external, schema requirements, performance needs, client capabilities) and the protocol's strengths. A mismatch — REST for high-throughput internal binary streaming, or gRPC for a browser-facing API without a gateway — creates friction that the team lives with for the system's lifetime.

Agents tend to default to REST/JSON for everything (familiar, ubiquitous) without analyzing whether the workload benefits from a different protocol's strengths, or to chase a new protocol (gRPC, GraphQL) without accounting for its costs (ecosystem, browser support, complexity). The judgment problem is recognizing that protocol choice is a tradeoff among dimensions (schema, efficiency, streaming, browser support, ecosystem), that the workload's characteristics determine which dimensions matter, and that the choice constrains the API's capabilities and should be made deliberately. This skill covers the discipline of RPC and protocol selection: the protocol decision by workload, serialization formats, sync vs async, and the tradeoffs of each major option.

## Core Rules

### Match The Protocol To The Workload's Communication Pattern

The first axis is the communication pattern: request-response, streaming, asynchronous, or bidirectional. Different protocols support different patterns natively.

- **Request-response (client asks, server answers): REST, gRPC unary, GraphQL.** The dominant pattern for APIs. REST (HTTP request/response) and gRPC unary are straightforward; GraphQL is request-response with client-specified shape. Choose among them by other axes (schema, efficiency, flexibility).
- **Server streaming (server sends a sequence to the client): gRPC server streaming, WebSocket, Server-Sent Events (SSE), chunked HTTP.** When the server sends a stream of data (live updates, large results), a streaming protocol is native. gRPC server streaming is efficient and typed; WebSocket is bidirectional and browser-native; SSE is simple and browser-native but server-to-client only.
- **Bidirectional streaming (both sides stream): gRPC bidirectional streaming, WebSocket.** When both client and server send streams (chat, real-time collaboration, control loops), a bidirectional protocol is required. gRPC bidi is typed and efficient; WebSocket is flexible and browser-native.
- **Asynchronous / decoupled (producer sends, consumer receives later): message queues (Kafka, RabbitMQ, SQS).** When the producer and consumer should be decoupled (the producer does not wait for the consumer), a message queue provides asynchronous communication. Not request-response; see webhook-and-event-api-design and queue-and-background-job-reliability.

### Consider Schema, Typing, And Contract Enforcement

Protocols differ in whether they enforce a schema (a contract on message structure) and how strongly.

- **Schema-driven protocols (gRPC/Protobuf, GraphQL, Thrift, Avro): a schema defines the messages; code is generated from it.** The schema is the contract; clients and servers are type-checked against it; changes are versioned. Strong fit for internal APIs where type safety and contract enforcement matter.
- **Schemaless or weakly-schemed protocols (REST/JSON): messages are self-describing (JSON), schema is optional (JSON Schema, OpenAPI).** Flexible and human-readable, but type safety relies on optional schema validation, and drift is easier. Strong fit for external or browser-facing APIs where flexibility and ubiquity matter.
- **Match the schema strictness to the need.** Internal APIs with many consumers benefit from schema-driven (type safety, contract enforcement); external APIs benefit from schemaless flexibility (JSON is universally consumable). Do not impose schema-driven protocols on external consumers who cannot easily consume them, or schemaless on internal teams who need type safety.
- **A schema registry can bring contract enforcement to schemaless protocols.** REST/JSON with an OpenAPI contract and validation approximates schema-driven contracts; the contract is enforced via middleware rather than code generation.

### Evaluate Serialization Format Tradeoffs

The serialization format (how messages are encoded on the wire) affects size, speed, human-readability, and schema evolution.

- **JSON: human-readable, ubiquitous, schemaless (or with optional schema).** Pros: readable, debuggable, universally supported, browser-native. Cons: larger (text encoding), slower to parse, no inherent schema. The default for external and browser-facing APIs.
- **Protobuf: binary, compact, schema-driven.** Pros: small (binary encoding), fast (de)serialization, strong schema with code generation, efficient for high-throughput. Cons: not human-readable (requires tooling to inspect), requires schema management. The default for gRPC and high-throughput internal APIs.
- **Avro: binary, compact, schema-driven with schema in the message.** Pros: compact, schema evolution support, common in data pipelines (Kafka). Cons: not human-readable, schema management overhead. Strong for event streaming and data pipelines.
- **Match the format to the need: JSON for readability and ubiquity, Protobuf for efficiency and type safety, Avro for streaming pipelines.** The format is often determined by the protocol (REST uses JSON, gRPC uses Protobuf), but where it is a choice (message queue payloads), select by the tradeoffs.

### Decide Synchronous Vs Asynchronous Communication

Whether communication is synchronous (the caller waits for a response) or asynchronous (the caller sends and proceeds) has major implications for coupling, latency, and failure handling.

- **Synchronous (REST, gRPC, GraphQL): the caller waits for the response.** Tighter coupling (the caller depends on the callee's availability and latency), simpler reasoning (the result is immediate), but the caller blocks and fails if the callee is down or slow. Suitable for operations needing an immediate result.
- **Asynchronous (message queue, events): the caller sends and proceeds, the response (if any) comes later.** Looser coupling (the caller does not depend on the callee's immediate availability), better failure isolation (a down callee does not fail the caller), but more complex reasoning (eventual consistency, idempotency, out-of-band results). Suitable for decoupled workflows, long-running operations, and fan-out.
- **Match the coupling to the requirement.** Operations that need an immediate result (a read, a synchronous validation) use synchronous; operations that can proceed without an immediate result (a notification, a background process, a fan-out to multiple services) use asynchronous. Over-using synchronous creates tight coupling and cascading failures; over-using asynchronous adds complexity.
- **Consider the failure modes: synchronous fails loudly (the caller gets an error), asynchronous fails silently (the message is queued, the failure is discovered later).** Synchronous failures are easier to handle immediately; asynchronous failures require monitoring of the queue and consumer. Match the failure visibility to the operation's needs.

### Account For Ecosystem, Browser Support, And Tooling

A protocol's ecosystem (libraries, tooling, browser support, developer familiarity) determines the integration cost and the system's maintainability.

- **REST/JSON: the most ubiquitous, browser-native, universally supported.** Every language, every platform, every developer. The lowest-friction choice for external or polyglot APIs. Lacks the efficiency and schema enforcement of newer protocols.
- **gRPC: strong in backend ecosystems, limited browser support without a gateway (gRPC-Web).** Excellent for internal service-to-service communication (efficient, typed, streaming), but browser clients require gRPC-Web or a REST gateway, adding complexity. Strong fit for internal microservices.
- **GraphQL: strong for flexible client queries, requires server-side schema and resolver implementation.** Excellent when clients need flexible, client-specified data shapes (mobile clients, aggregating APIs), but the server complexity (schema, resolvers, query parsing, N+1 prevention) is significant. Strong fit for client-diverse APIs.
- **Message queues: ecosystem depends on the broker (Kafka, RabbitMQ, NATS, cloud queues).** Strong for asynchronous decoupling, but introduce a broker (operational component) and a communication model (queues, topics, subscriptions) that the team must operate.
- **Weigh the ecosystem cost.** A protocol with poor ecosystem support in the team's languages or limited developer familiarity has a high integration and maintenance cost. Choose a protocol the team and the consumers can work with effectively.

### Consider Performance And Efficiency Where It Matters

For high-throughput or latency-sensitive communication, the protocol's performance characteristics matter.

- **Binary protocols (gRPC/Protobuf, Thrift) are more efficient than text (REST/JSON).** Smaller messages (binary encoding), faster (de)serialization, lower latency and bandwidth. Significant for high-throughput internal communication; less relevant for low-volume external APIs.
- **HTTP/2 (used by gRPC) supports multiplexing (multiple requests over one connection), reducing connection overhead.** REST over HTTP/1.1 has per-request connection overhead (mitigated by HTTP/2 or keep-alive); gRPC's HTTP/2 multiplexing is more efficient for many requests.
- **Streaming protocols avoid per-message request overhead.** A streaming protocol (gRPC streaming, WebSocket) sends many messages over one connection, avoiding per-message HTTP overhead. Significant for high-frequency or real-time communication.
- **Do not over-optimize protocol performance where it does not matter.** For a low-volume external API, REST/JSON's performance is fine; the ubiquity and simplicity outweigh the efficiency of binary protocols. Optimize protocol performance where the workload is high-throughput or latency-sensitive.

## Common Traps

### Defaulting To REST/JSON Without Analysis

Using REST/JSON for a workload that would benefit from a different protocol's strengths (gRPC for internal streaming, a queue for decoupling), missing the benefit. Analyze the workload's pattern and needs.

### Chasing A New Protocol Without Accounting For Costs

Adopting gRPC or GraphQL for the novelty, without accounting for ecosystem cost (browser support, tooling, team familiarity, complexity). Weigh the ecosystem and complexity.

### Synchronous Where Asynchronous Is Needed

Using synchronous request-response for a workflow that should be decoupled (a long-running operation, a fan-out), creating tight coupling and blocking callers. Use asynchronous for decoupled workflows.

### gRPC For Browser Clients Without A Gateway

Using gRPC for a browser-facing API without gRPC-Web or a gateway, which browsers cannot consume. Use REST/JSON or a gateway for browser clients.

### Schemaless Protocol For An Internal API Needing Type Safety

Using REST/JSON for an internal API with many consumers, missing the type safety and contract enforcement of schema-driven protocols. Use gRPC or schema-enforced REST for internal type safety.

### Over-Optimizing Protocol Performance For Low-Volume APIs

Choosing a binary protocol for efficiency on a low-volume API where REST/JSON's ubiquity and simplicity matter more. Match the optimization to the workload.

### Ignoring Failure Mode Differences

Using synchronous communication without handling the callee's failures (timeouts, circuit breaking), or asynchronous without monitoring the queue and consumer. Match failure handling to the communication model.

### Protocol Choice Not Aligned With Consumer Capabilities

Choosing a protocol the consumers cannot easily consume (gRPC for a polyglot external audience, a proprietary protocol for third-party integration). Match the protocol to the consumers.

## Self-Check

- [ ] The protocol is matched to the workload's communication pattern — request-response (REST, gRPC unary, GraphQL), server streaming (gRPC streaming, WebSocket, SSE), bidirectional streaming (gRPC bidi, WebSocket), or asynchronous decoupling (message queues) — rather than defaulted to REST/JSON or chosen by novelty.
- [ ] Schema and typing strictness is matched to the need — schema-driven (gRPC/Protobuf, GraphQL, Thrift, Avro) for internal APIs needing type safety and contract enforcement, schemaless (REST/JSON) for external or browser-facing APIs needing flexibility and ubiquity, with a schema registry or OpenAPI where contract enforcement on schemaless protocols is needed.
- [ ] The serialization format (JSON for readability/ubiquity, Protobuf for efficiency/type safety, Avro for streaming pipelines) is matched to the need, often determined by the protocol but selected deliberately where it is a choice.
- [ ] Synchronous vs asynchronous communication is chosen by the requirement — synchronous for operations needing an immediate result, asynchronous for decoupled workflows, long-running operations, and fan-out — with failure handling matched to the model (timeouts and circuit breaking for sync, queue and consumer monitoring for async).
- [ ] Ecosystem, browser support, and tooling are weighed: REST/JSON for ubiquity and browser-native support, gRPC for internal efficiency (with gRPC-Web or gateway for browsers), GraphQL for flexible client queries (with server complexity accepted), message queues for decoupling (with broker operation accepted).
- [ ] Performance and efficiency are considered where the workload is high-throughput or latency-sensitive (binary protocols, HTTP/2 multiplexing, streaming), and not over-optimized where the workload is low-volume and ubiquity/simplicity matter more.
- [ ] The protocol choice is aligned with consumer capabilities — browsers consume REST/JSON or gRPC-Web, internal polyglot services consume gRPC or schema-enforced REST, external third parties consume widely-supported protocols — rather than imposing a protocol consumers cannot easily use.
- [ ] The decision and its tradeoffs are documented (why this protocol, what alternatives were considered, what constraints it imposes), so future changes are informed and the team understands the protocol's capabilities and limitations.
