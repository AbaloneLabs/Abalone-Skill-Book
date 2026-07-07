---
name: network_and_protocol_performance.md
description: Use when the agent is optimizing the performance of networked communication — HTTP/1.1 vs HTTP/2 vs HTTP/3, connection reuse and keep-alive, TCP tuning (Nagle, window size, buffers), TLS handshake overhead and session resumption, connection pool sizing for outbound HTTP/gRPC/database clients, batching and multiplexing requests, payload size and compression (gzip, brotli, zstd), protocol selection (REST, gRPC, GraphQL, messaging), or diagnosing latency caused by network round-trips, head-of-line blocking, or connection setup. Covers round-trip minimization, connection management, protocol tradeoffs, payload optimization, and the interaction of network behavior with application performance.
---

# Network And Protocol Performance

In a distributed system, the network is on the critical path of almost every operation, and its characteristics — round-trip latency, bandwidth, connection setup cost, head-of-line blocking, handshake overhead — dominate the performance of operations that are individually fast on the server. A database query that takes 2ms to execute but requires a fresh TLS connection adds 100ms of handshake; a client that fetches ten resources sequentially over HTTP/1.1 pays ten round-trips where HTTP/2 multiplexing would pay one; a payload compressed with the wrong algorithm ships megabytes where a better codec would ship kilobytes. The application logic is often fast; the network behavior around it is often slow, and the two are easy to confuse. An engineer who profiles the handler and finds it cheap may conclude the endpoint is fast, while the user waits on connection setup and sequential round-trips the profile never showed.

Agents tend to treat the network as a transparent medium — "make the request, get the response" — and to optimize the handler while ignoring the connection management, protocol choice, and round-trip structure that determine the user-perceived latency. The judgment problem is recognizing that network performance is shaped by decisions about how connections are established and reused, how requests are batched and multiplexed, which protocol carries them, and how payloads are encoded — and that these decisions, made once and then forgotten, silently determine the latency of every operation thereafter. This skill covers the discipline of making network communication fast: minimizing round-trips, managing connections, choosing protocols and encodings deliberately, and diagnosing latency whose cause is the network, not the handler.

## Core Rules

### Minimize Round-Trips, Because Latency Dominates At Distance

A round-trip is the time for a request to reach the server and the response to return, and it is dominated by the speed of light and the path between client and server — not by the work done. At intercontinental distances a round-trip is 100-300ms, and an operation that requires many sequential round-trips is slow regardless of how fast each server is.

- **Identify the number of sequential round-trips on the critical path.** A page that fetches data, then fetches related data based on the result, then fetches more, pays each round-trip in series. The total latency is the sum, and it grows with distance. Reducing sequential round-trips (batching, embedding related data, parallelizing independent fetches) is often the highest-leverage optimization.
- **Batch independent requests into one round-trip.** If a client needs ten independent resources, one batched request (or a parallel fan-out) beats ten sequential requests by the latency of nine round-trips. APIs that support compound requests or field expansion reduce round-trips.
- **Embed related data to avoid follow-up fetches.** Returning a resource with its related resources included (GraphQL field selection, compound REST responses) avoids the round-trips of fetching each relation separately. Balance against payload size (below).
- **Parallelize independent operations.** Independent calls that can run concurrently should; the latency is the max, not the sum. A waterfall of sequential calls where parallel would suffice is pure wasted latency.

### Reuse Connections; Connection Setup Is Expensive

Establishing a connection is costly: DNS resolution, TCP handshake (one round-trip), TLS handshake (one to two round-trips for TLS 1.2, one for TLS 1.3), and protocol negotiation. A fresh connection per request pays this cost every time; a reused connection pays it once.

- **Enable keep-alive and connection reuse for clients and servers.** HTTP keep-alive allows multiple requests over one TCP connection, amortizing the handshake. Ensure it is enabled on both ends and not defeated by proxy or load-balancer behavior that closes connections.
- **Use connection pools for outbound clients (HTTP, gRPC, database).** A pool maintains a set of warm connections reused across requests, avoiding per-request handshake. Size the pool to concurrent demand (see resource-and-capacity-planning); a pool too small serializes requests.
- **Enable TLS session resumption.** Resuming a TLS session (session id or session ticket) skips the full handshake, reducing setup to one round-trip or zero (in TLS 1.3 with 0-RTT). Ensure the server supports resumption and the client uses it.
- **Prefer HTTP/2 or HTTP/3 over HTTP/1.1 for multiplexing.** HTTP/2 multiplexes many requests over one connection, eliminating the head-of-line blocking of HTTP/1.1 (where one slow request blocks others behind it on the same connection) and reducing connection setup. HTTP/3 (over QUIC) further reduces connection setup and survives packet loss better.

### Choose The Protocol For Its Performance Characteristics

Protocols differ in ways that affect performance: multiplexing, streaming, payload encoding, connection behavior, and semantics. Choose deliberately, not by default.

- **HTTP/2 multiplexes, reducing connection count and head-of-line blocking.** Prefer it over HTTP/1.1 for multi-request interactions. Its server push feature is largely deprecated and unreliable; do not rely on it.
- **gRPC over HTTP/2 offers multiplexing, streaming, and compact binary encoding.** Well-suited to high-throughput, low-latency inter-service communication, with bidirectional streaming for ongoing exchanges. Its protobuf encoding is smaller and faster to parse than JSON, at the cost of human readability and schema management.
- **HTTP/3 over QUIC reduces connection setup (0-RTT resumption) and handles packet loss better** (no head-of-line blocking across streams because QUIC streams are independent). Valuable for mobile and lossy networks; ensure infrastructure supports it.
- **Match the protocol to the interaction pattern.** Request-response suits HTTP; streaming updates suit WebSockets, SSE, or gRPC streaming; high-throughput inter-service suits gRPC or a messaging system. Do not force a streaming pattern onto request-response protocols or vice versa.

### Optimize Payload Size And Encoding

The size of what crosses the network affects both latency (time to transmit) and cost (bandwidth). Smaller payloads are faster and cheaper, up to the point where compression cost exceeds transmission savings.

- **Compress responses with an appropriate codec.** gzip is ubiquitous; brotli compresses better for text (HTML, JS) and is supported by modern browsers; zstd offers a strong speed/ratio tradeoff and is increasingly available. Apply compression to text-based responses; do not compress already-compressed formats (images, video, pre-compressed assets) — it wastes CPU for no gain.
- **Choose efficient encodings for inter-service data.** Protobuf, FlatBuffers, and MessagePack are smaller and faster to serialize/deserialize than JSON. For high-volume inter-service traffic, the encoding choice materially affects both bandwidth and CPU.
- **Select only the fields the client needs.** Over-fetching (returning fields the client ignores) wastes bandwidth and serialization. APIs that allow field selection (GraphQL, sparse fieldsets, projections) let clients request only what they use.
- **Paginate large collections.** Returning an entire large collection in one response is slow and memory-heavy; pagination bounds the payload. See the pagination skill for cursor vs offset tradeoffs.
- **Compress request bodies for uploads.** Large request bodies (file uploads, batch payloads) benefit from compression, subject to the same codec choice and the same caveat about already-compressed data.

### Tune TCP And Connection Behavior For The Workload

TCP has parameters that affect throughput and latency, and the defaults are tuned for general use, not for every workload. Understand the relevant ones for high-throughput or high-latency links.

- **Be aware of Nagle's algorithm and TCP_NODELAY.** Nagle coalesces small packets to reduce overhead, adding latency for small, interactive writes. For interactive protocols (SSH, gaming, small request/response), disabling Nagle (TCP_NODELAY) reduces latency; for bulk transfer, leave it on. HTTP/2 and HTTP/3 generally handle this appropriately.
- **Be aware of TCP window size and buffer tuning for high-bandwidth-delay-product links.** On long-fat pipes (high bandwidth × high latency), the default window may limit throughput. Large windows and buffers improve throughput on such links; this is usually a kernel/stack setting, not application code.
- **Enable BBR or modern congestion control where available.** BBR congestion control can improve throughput and reduce bufferbloat compared to loss-based algorithms like CUBIC. It is a kernel/stack setting with meaningful effect on long-distance throughput.

## Common Traps

### Sequential Round-Trips Where Parallel Or Batched Would Suffice

A client fetching data then related data in series, or making N independent requests sequentially, paying N round-trips where one batched or parallel request would pay one. Batch, embed, or parallelize independent operations.

### Fresh Connection Per Request

Opening a new TCP/TLS connection for each request, paying handshake latency every time. Enable keep-alive and use connection pools.

### HTTP/1.1 Head-Of-Line Blocking

Multiple requests queued behind a slow one on the same HTTP/1.1 connection, or many parallel connections each paying setup. Prefer HTTP/2 multiplexing.

### Over-Fetching Fields The Client Ignores

Returning large payloads with fields the client does not use, wasting bandwidth and serialization. Support field selection; return only what is needed.

### Compressing Already-Compressed Data

Applying gzip to images, video, or pre-compressed assets, wasting CPU for no size reduction. Compress text; skip already-compressed formats.

### Wrong Encoding For Inter-Service Volume

Using verbose JSON for high-volume inter-service traffic where protobuf or MessagePack would be smaller and faster. Match the encoding to the volume and performance need.

### Ignoring TLS Setup Cost

A client making many short-lived TLS connections, each paying the handshake. Enable session resumption and connection reuse.

### Treating The Handler As The Latency Source

Profiling the server handler, finding it fast, and concluding the endpoint is fast — while the user waits on connection setup, sequential round-trips, or payload transmission the handler profile never showed. Measure end-to-end, including the network.

## Self-Check

- [ ] The number of sequential round-trips on the critical path is identified and minimized — independent requests are batched, parallelized, or related data is embedded — rather than fetched in a waterfall that sums round-trip latencies.
- [ ] Connections are reused (keep-alive, connection pools for outbound HTTP/gRPC/database clients) sized to concurrent demand, and TLS session resumption is enabled, so per-request connection setup cost is amortized rather than paid every time.
- [ ] The protocol (HTTP/1.1, HTTP/2, HTTP/3, gRPC, WebSocket, messaging) is chosen for its performance characteristics (multiplexing, streaming, encoding, connection behavior) and matched to the interaction pattern, with HTTP/2+ preferred over HTTP/1.1 for multi-request interactions.
- [ ] Payload size is optimized: responses are compressed with an appropriate codec (gzip/brotli/zstd for text, skipped for already-compressed formats), inter-service data uses efficient encodings (protobuf/MessagePack) for high-volume traffic, clients select only needed fields, and large collections are paginated.
- [ ] TCP and connection behavior is tuned for the workload where relevant (TCP_NODELAY for interactive small writes, window/buffer sizing for high-bandwidth-delay-product links, modern congestion control like BBR), with awareness of why defaults may not fit.
- [ ] End-to-end latency (including network, connection setup, round-trips, and payload transmission) is measured as the user experiences it, not just the server handler — so network-induced latency is visible and addressable, not hidden by a fast handler profile.
- [ ] The interaction between client and server is reviewed for round-trip structure (waterfall vs parallel, sequential vs batched), connection management (reuse vs fresh, pool sizing), protocol fit, and payload efficiency — each optimized at the layer where it occurs.
- [ ] Where latency is dominated by geographic distance (cross-region, global), the design accounts for it via locality (serve from nearby), caching, or round-trip minimization, rather than assuming the network is fast.
