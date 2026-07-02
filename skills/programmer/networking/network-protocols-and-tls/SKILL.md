---
name: network_protocols_and_tls.md
description: Use when the agent is choosing a transport or application protocol for a service-to-service, client-to-server, or real-time communication path; deciding between TCP, UDP, QUIC, WebSockets, gRPC, HTTP/1.1, HTTP/2, or HTTP/3; configuring TLS/SSL handshakes, certificate authorities, certificate pinning, or mutual TLS (mTLS) for service-to-service authentication; setting up DNS, service discovery, or load balancing (L4 vs L7); designing connection management (keep-alive, connection pools, multiplexing); reasoning about latency, bandwidth, head-of-line blocking, or congestion control; or diagnosing TLS handshake failures, certificate expiry, protocol downgrade, connection exhaustion, or cross-network compatibility issues. Also covers the security/performance tradeoff, the architectural impact of protocol choice (streaming RPC vs request/response, ordered vs unreliable), TLS termination placement, and the failure mode of treating the network as a reliable homogeneous medium.
---

# Network Protocols And TLS

Every networked system is built on a stack of protocol decisions: which transport carries the bytes, which application framing sits on top, how the two endpoints prove who they are to each other, and how connections are found, balanced, and kept alive. Each of these is a judgment problem, not a default. TCP gives ordered reliable delivery but pays for it in head-of-line blocking and connection-setup latency; UDP is fast and cheap but offloads reliability entirely to the application; HTTP/2 multiplexes but can stall behind a single lost packet on one stream; mTLS gives strong service identity but adds certificate lifecycle and operational cost. The wrong choice does not fail immediately — it fails under load, under network partitions, under certificate expiry, or under the specific latency and ordering requirements the business actually has.

Agents tend to under-invest here because the happy path is trivial: a request goes out, a response comes back, the certificate is valid. The harm appears only at the margins and is severe: a TLS certificate that expires silently takes the whole service down; a protocol that multiplexes over one TCP connection stalls all requests behind a slow one; a load balancer that terminates TLS at the edge but forwards plaintext internally leaves service-to-service traffic unauthenticated; a "real-time" feature built on request polling collapses under scale; a UDP-based system that assumed ordered delivery silently corrupts state. The judgment problem is deciding, for each communication path, what reliability, ordering, latency, and authentication guarantees the business requires, then choosing the protocol and security posture that delivers them — and verifying the choice holds under failure, not only on the happy path.

This skill covers protocol and transport selection, TLS and mTLS, DNS and service discovery, load balancing, and connection management. It complements the HTTP-caching skill (which covers HTTP headers, caching, and browser mechanics), the cryptography skill (which covers key management and algorithm selection in depth), and the reliability skill (which covers retries, timeouts, and circuit breakers at the call site).

## Core Rules

### Choose The Transport By The Reliability, Ordering, And Latency The Business Requires

TCP and UDP solve fundamentally different problems, and the choice is dictated by what the application cannot tolerate, not by which is "faster." Decide per communication path:

- **TCP** provides reliable, ordered, connection-oriented delivery. It retransmits lost packets, delivers bytes in order, and manages congestion. Use it when the application needs all data to arrive intact and in order — most request/response APIs, file transfer, databases, financial transactions. The cost: head-of-line blocking (one lost packet stalls everything behind it until retransmitted), connection-setup latency (the handshake), and state per connection.
- **UDP** provides unreliable, unordered, connectionless delivery. No retransmission, no ordering, no congestion control unless the application adds it. Use it when occasional loss is acceptable or preferable to delay — voice/video (a late packet is worse than a dropped one), DNS queries, metrics/telemetry, discovery broadcasts. The cost: the application owns reliability, ordering, congestion control, and security.
- **QUIC** (the transport under HTTP/3) is UDP-based but implements its own reliable stream multiplexing and TLS 1.3 natively. It eliminates the head-of-line blocking of HTTP/2 over TCP (each stream retransmits independently) and speeds connection setup (1-RTT or 0-RTT). Use it where connection-setup latency and stream independence matter (mobile, flaky networks, many parallel requests) and where the infrastructure supports it.

The decisive questions: can the application tolerate a lost message? If no, you need reliability (TCP, or QUIC, or reliability layered on UDP). Does ordering matter? If yes, the transport or application must preserve it. Is latency more important than completeness? If yes (real-time media), prefer UDP with application-level loss handling. Do not reach for TCP reflexively for everything, and do not reach for UDP "for speed" and then rebuild TCP badly inside the application.

### Match The Application Protocol To The Interaction Pattern

HTTP/1.1, HTTP/2, HTTP/3, gRPC, and WebSockets differ in how they frame, multiplex, and model the conversation, and the choice should follow the interaction pattern, not familiarity:

- **HTTP/1.1** — request/response, one response per request, text-based. Simple, universally supported, but one request per TCP connection at a time unless pipelining or connection pooling is used. Fine for simple APIs and where universal compatibility outweighs efficiency.
- **HTTP/2** — multiplexes many concurrent requests over a single TCP connection, binary framing, header compression. Eliminates the connection-per-request overhead. But all streams share one TCP connection, so a lost packet blocks *all* streams until retransmitted (TCP-level head-of-line blocking).
- **HTTP/3** — HTTP/2 semantics over QUIC, so each stream retransmits independently and connection setup is faster. Best where networks are lossy or mobile and the infrastructure supports it; the tradeoff is newer, less-universal support and UDP-blocking middleboxes.
- **gRPC** — HTTP/2-based, strongly-typed contracts (Protocol Buffers), bidirectional streaming, code generation across languages. Strong for internal service-to-service RPC with complex schemas and streaming needs; weaker for browser-direct or public APIs where the contract and HTTP/2 requirement add friction.
- **WebSockets** — a single upgraded TCP connection for bidirectional, low-latency, server-initiated messages. Use for real-time push (chat, live updates) where you need the server to initiate and low per-message overhead. The cost: you own framing, reconnection, authentication, and backpressure on a long-lived connection.

The judgment: request/response workloads fit HTTP; strongly-typed internal RPC with streaming fits gRPC; server-initiated or high-frequency bidirectional traffic fits WebSockets or server-sent events. Forcing one pattern into another (simulating push with polling, or simulating request/response over a streaming connection) produces fragile, inefficient systems.

### Treat TLS As The Identity And Integrity Layer, Not Just "Encryption"

TLS provides three guarantees: confidentiality (eavesdroppers cannot read the traffic), integrity (tampering is detected), and authentication (the endpoint is who it claims to be). The third is the most underused and the most important for service-to-service security. "We use TLS" is meaningless if the client does not verify the server certificate, or if the verification is disabled to make a connection work.

- **Always verify certificates.** A TLS connection with certificate verification disabled is encrypted but unauthenticated — an attacker who can intercept traffic can present any certificate and man-in-the-middle the connection. Never disable verification to fix a connection error; fix the certificate or trust store.
- **Understand the handshake.** TLS 1.3 (the modern default) does a 1-RTT handshake (one round trip to establish the secure connection), with an optional 0-RTT resumption for returning clients. TLS 1.2 does 2-RTT. The handshake cost is real on every new connection, which is why connection reuse and session resumption matter for latency-sensitive paths.
- **Negotiate modern versions and cipher suites.** Disable TLS 1.0/1.1 and legacy ciphers; require TLS 1.2+ (preferably 1.3). Weak configurations expose the connection to downgrade and known-attacks even when the certificate is valid.
- **SNI and the certificate chain.** The Server Name Indication tells the server which certificate to present; a mismatch (wrong SAN, missing intermediate) causes failures that appear only against the real server, not in local testing.

Treat TLS configuration as a security decision reviewed like access control, not as a deployment checkbox. See the cryptography skill for the underlying key and algorithm choices.

### Use Mutual TLS Where Services Must Prove Identity To Each Other

Standard TLS authenticates the server to the client. In a service mesh or internal system, each service must also prove its identity to the others — otherwise any service on the internal network can impersonate another. Mutual TLS (mTLS) requires both sides to present and verify certificates, giving cryptographically strong service identity.

- **When mTLS is warranted.** Internal service-to-service traffic crossing a network you do not fully trust; zero-trust architectures; regulated environments requiring strong authentication of every hop; any system where a compromised peer on the network is a realistic threat.
- **The real cost is certificate lifecycle, not the handshake.** Each service needs a certificate issued, distributed, rotated before expiry, and revoked if compromised. This is operationally significant: short-lived certificates (hours to days) improve security but demand automated issuance (a certificate authority, SPIFFE/SPIRE, or a service mesh's CA) and rotation. Manual certificate management does not scale past a handful of services.
- **mTLS complements, not replaces, authorization.** mTLS proves *who* the caller is; it does not say what they may do. Layer authorization (the caller may call this endpoint, may read this tenant's data) on top of the authenticated identity.
- **Termination placement matters.** If TLS terminates at the load balancer and traffic continues as plaintext internally, the internal hops are unauthenticated. For end-to-end authentication, either run mTLS hop-by-hop or accept that trust ends at the termination point and secure the internal network separately.

Decide deliberately where trust is established and where it ends. "TLS-terminated at the edge" is a valid architecture; "TLS-terminated at the edge and then assuming the internal network is trusted" is a vulnerability in a zero-trust world.

### Decide L4 Vs L7 Load Balancing By What You Need To Route On

Load balancers operate at different OSI layers, and the layer determines what they can see and do:

- **L4 (transport layer).** Balances on IP and port (TCP/UDP). Fast, protocol-agnostic, opaque to payload. Good for raw TCP/UDP throughput, TLS passthrough (the balancer does not decrypt), and where you do not need to inspect content. Cannot route by URL, header, or cookie; cannot do content-based affinity or L7 health checks.
- **L7 (application layer).** Balances on HTTP/URL/headers/cookies (or other application protocol). Can route by path, host, header; can do content-based affinity (sticky sessions by cookie); can modify requests/responses. Required when routing decisions depend on content. The cost: the balancer must terminate TLS (to read headers), adding decryption overhead and making it the trust boundary.

The tradeoff: L4 is faster and preserves end-to-end TLS (the balancer never sees plaintext), but is content-blind; L7 is content-aware but requires TLS termination at the balancer and more processing. Choose L4 for simple, high-throughput, TLS-passthrough distribution; choose L7 when you need path-based routing, sticky sessions, header-based decisions, or content-aware health checks. Many systems use both: an L4 balancer distributing to L7 balancers or ingress controllers.

### Treat DNS And Service Discovery As A Reliability Surface

Service-to-service communication depends on knowing where the peer is, and that "where" changes constantly as services scale, redeploy, and fail over. DNS and service discovery are not plumbing to set once; they are a reliability surface with specific failure modes:

- **DNS caching hides changes and outages.** Clients and intermediate resolvers cache DNS records for the TTL. A record cached for a long TTL will keep sending traffic to an endpoint that has moved or failed until the TTL expires. Short TTLs enable faster failover but increase DNS query load; long TTLs reduce load but slow propagation. Match the TTL to how quickly you need changes to take effect.
- **Service discovery vs static DNS.** For dynamic, autoscaling, or frequently-redeploying services, a service registry (Consul, etcd, Kubernetes DNS, cloud service discovery) that tracks live, health-checked instances is more reliable than static DNS. The registry should reflect health, not just existence.
- **DNS failure is total.** If clients cannot resolve a name, the service is unreachable regardless of its actual health. DNS outages and misconfiguration are a top cause of full outages; treat DNS as critical infrastructure with redundancy and monitoring, not an afterthought.
- **Stale or split-brain discovery.** A registry that lists dead instances sends traffic to them; a registry that lags behind deploys routes to old or draining instances. Health checking and timely deregistration are part of the discovery contract.

Decide how instances are discovered, how quickly changes propagate, and what happens when discovery itself fails. "We use DNS" is not a discovery strategy until you have answered the TTL, health, and failure questions.

### Manage Connections Deliberately — They Are Finite, Stateful Resources

Connections are not free. Each open TCP connection consumes kernel memory, file descriptors, and ports; connection setup (especially with TLS) adds latency; and unmanaged connections exhaust resources under load. Connection management is a first-class design decision:

- **Keep-alive and connection reuse.** Reusing a connection across requests avoids repeated handshakes (TCP and TLS), dramatically reducing latency for chatty clients. HTTP keep-alive and connection pools exist for this. Disabling keep-alive means a full handshake per request, which is catastrophic for latency-sensitive paths.
- **Connection pools sized to the workload.** A pool too small serializes requests behind a few connections; a pool too large exhausts server resources and file descriptors. Size pools against expected concurrency and server capacity, and bound them so a misbehaving client cannot open unlimited connections.
- **Multiplexing tradeoff.** HTTP/2 and QUIC multiplex many requests over one connection (efficient, fewer handshakes), but a single connection means all requests share that connection's fate — a stuck or slow stream, or a connection-level issue, affects everything on it. Understand whether your workload benefits from multiplexing or from more independent connections.
- **Idle and half-open connections.** Connections left idle can be silently killed by middleboxes, NATs, or peers, leaving the client thinking the connection is alive (half-open). Use keepalive probes or heartbeat messages to detect dead connections, and close connections explicitly when done.

The default question: what happens to connections under peak load, under peer failure, and over long-lived sessions? If the answer is "they accumulate until something breaks," the connection management is incomplete.

### Ground Latency Reasoning In The Real Network, Not The Local One

Network latency and bandwidth behave very differently across environments, and assumptions valid in local development fail in production. The real constraints:

- **Speed of light and distance are irreducible.** Cross-region or cross-continent round trips cost tens to hundreds of milliseconds of physics; a synchronous call across regions pays this on every round trip. A design that works locally (sub-millisecond hops) becomes unusable when each hop is 50–150ms.
- **Bandwidth is not uniform.** A payload fine on a datacenter link may saturate a mobile or cross-region link. Large messages, chatty protocols, and uncompressed payloads that are harmless locally become bottlenecks in production.
- **Head-of-line blocking compounds.** A protocol that stalls behind a lost packet stalls worse on lossy or congested networks. Test protocol choices on realistic network conditions, not the ideal local network.
- **Connection setup is not free at scale.** A TLS handshake that is negligible for one client is significant when thousands of clients connect per second. Connection reuse and session resumption exist precisely to amortize this.

When designing or diagnosing a networked path, ask: what is the real round-trip time, bandwidth, and loss rate in production? Decisions made assuming a local network (synchronous cross-region calls, no connection reuse, large chatty payloads) are the most common source of "it worked in dev, it's slow in prod."

## Common Traps

### Disabling Certificate Verification To Fix A Connection Error

Turning off TLS verification (or setting `verify=false`) to make a failing connection succeed. The connection is now encrypted but unauthenticated — any man-in-the-middle can present a self-signed certificate. Fix the certificate, the trust store, or the hostname mismatch; never disable verification.

### A Multiplexed Protocol That Stalls Behind One Slow Stream

Using HTTP/2 multiplexing over TCP and assuming concurrent streams are independent. A lost packet on one stream blocks all streams sharing the TCP connection until retransmitted. Where this matters (lossy networks, many parallel requests), HTTP/3 over QUIC or multiple connections may serve better.

### TLS Terminated At The Edge, Internal Traffic Assumed Trusted

Terminating TLS at the load balancer and forwarding plaintext internally, then treating the internal network as trusted so services do not authenticate each other. In a zero-trust or compromised-peer threat model, internal traffic is unauthenticated and forgeable. Either run mTLS hop-by-hop or consciously accept the trust boundary and document it.

### Choosing UDP "For Speed" Then Rebuilding TCP Badly

Selecting UDP to avoid TCP's overhead, then reimplementing reliability, ordering, and congestion control ad hoc inside the application — usually incorrectly. If you need reliability and ordering, use TCP or QUIC. Only choose UDP when you genuinely can tolerate loss or have a proven application-level protocol.

### A Certificate That Expires And Takes The Service Down

A long-lived TLS certificate that no one rotates until it expires, at which point every connection fails and the service is down. Certificates are operational artifacts with expiry dates; automate issuance, rotation, and expiry monitoring. An unmonitored expiry is a scheduled outage.

### DNS TTL Mismatched To Failover Needs

A long DNS TTL that feels efficient until a failover is needed, and clients keep hitting the dead endpoint for the TTL duration. Or a short TTL that enables failover but is ignored by non-compliant resolvers. Match TTL to your actual failover requirement and test that propagation works.

### Assuming The Local Network Reflects Production

Designing and testing only on a low-latency local network, then discovering in production that synchronous cross-region calls, large payloads, or chatty protocols are unusable. Validate latency-sensitive designs against realistic round-trip times, bandwidth, and loss.

### L7 Load Balancing Without TLS Termination Awareness

Choosing L7 for content-based routing but not accounting for the requirement to terminate TLS at the balancer (to read headers), which makes the balancer the trust boundary and adds decryption load. Or choosing L4 passthrough and then being surprised routing cannot depend on content. Align the layer with the routing and trust requirements.

### Unbounded Or Idle Connections Exhausting Resources

Opening a new connection per request (handshake overhead at scale) or never closing idle connections (file descriptor and memory exhaustion, half-open connections after a silent kill). Reuse connections with bounded pools and detect dead connections with keepalive or heartbeats.

### Treating "We Use HTTPS" As The Complete Security Answer

Equating TLS with security. TLS protects data in transit between two authenticated endpoints; it does not protect data at rest, does not authorize what the authenticated caller may do, and provides nothing if the endpoint itself is compromised. TLS is one layer, not the whole posture.

## Self-Check

- [ ] For each communication path, the transport (TCP, UDP, QUIC) was chosen against whether the application can tolerate loss, needs ordering, and prioritizes latency over completeness — not reflexively; the reliability, ordering, and latency requirements are stated and the transport meets them.
- [ ] The application protocol (HTTP/1.1, HTTP/2, HTTP/3, gRPC, WebSockets) matches the interaction pattern (request/response, typed RPC, server-initiated push, bidirectional streaming), and no pattern is forced to emulate another.
- [ ] TLS is configured to verify certificates on both sides, only modern versions and cipher suites (TLS 1.2+, preferably 1.3) are negotiated, and no connection disables verification to work around an error.
- [ ] mTLS is used where services must prove identity to each other, with an automated certificate lifecycle (issuance, rotation before expiry, revocation); manual certificate management is not the plan, and mTLS identity is layered with authorization.
- [ ] The trust boundary is explicit: where TLS is terminated, where it is re-established, and whether internal traffic is authenticated — and "TLS-terminated at the edge then trusted internally" is a conscious, documented decision rather than an assumption.
- [ ] Load balancing layer (L4 vs L7) matches the routing requirement (content-based routing and sticky sessions need L7 and TLS termination; raw passthrough and speed favor L4), and the TLS-termination consequence of L7 is accounted for.
- [ ] DNS and service discovery have a defined TTL matching failover needs, health-checked live instances (not static records), and a plan for discovery failure — DNS is treated as critical infrastructure, not plumbing.
- [ ] Connections are managed deliberately: keep-alive/reuse is enabled, pools are bounded and sized to concurrency, multiplexing tradeoffs are understood, and idle/half-open connections are detected via keepalive or heartbeats.
- [ ] Latency and bandwidth decisions were validated against realistic production network conditions (cross-region round trips, real bandwidth, loss), not the local network; synchronous cross-region calls, large payloads, and chatty protocols were specifically scrutinized.
- [ ] The highest-risk cases were verified — certificate expiry and rotation, head-of-line blocking under loss, internal traffic authentication, DNS-driven failover, and connection exhaustion under load — not only the happy-path request/response.
