---
name: api_gateway_and_edge_design.md
description: Use when the agent is designing or configuring an API gateway, edge proxy, or ingress layer (Kong, Envoy, NGINX, cloud API gateways, CDN edge), deciding which cross-cutting concerns belong at the edge (authentication, rate limiting, routing, TLS termination, request/response transformation, request validation), shaping traffic (rate limits, quotas, throttling), handling gateway failure modes, or choosing between gateway-managed and service-managed concerns. Also covers the failure modes of a gateway as a single point of failure and bottleneck, inconsistent authn/authz between gateway and services, rate limits that are too coarse or too aggressive, transformation logic that couples the gateway to service internals, and the recurring mistake of pushing business logic into the gateway until it becomes an unmaintainable, stateful, service-coupled component rather than a thin cross-cutting edge.
---

# API Gateway And Edge Design

An API gateway sits between external clients and internal services, and it is the natural place to consolidate cross-cutting concerns that every request needs — TLS termination, authentication, routing, rate limiting, request validation, observability. The judgment problem is that the gateway is powerful and convenient, and that convenience invites the wrong decisions. Push too little to the edge, and every service reimplements auth and rate limiting inconsistently. Push too much — especially business logic, stateful behavior, or service-specific transformation — and the gateway becomes a coupled, stateful, unmaintainable component that is also a single point of failure and a bottleneck for the entire system. The gateway is also the system's most exposed component: it faces untrusted clients, it must fail safely (fail-closed on auth, not fail-open), it must scale with total traffic (not per-service traffic), and its failure mode is total outage. The discipline is to keep the gateway thin (cross-cutting, stateless, standardized concerns), fail it safely, make it highly available and horizontally scalable, and resist the gravitational pull to put service-specific logic at the edge.

Agents tend to under-invest here because a gateway's defaults route traffic and terminate TLS, and the first API works. The harm appears as traffic and services grow. Authentication is implemented in the gateway for some services and in the service for others, producing inconsistent security boundaries. A rate limit is set globally and throttles legitimate bursty traffic while failing to protect a specific service. Transformation logic in the gateway reaches into service internals, so a service change requires a gateway change, coupling deployment. The gateway holds state (sessions, caches) that is not replicated, so it cannot scale horizontally or its failure loses state. The gateway fails open on an auth backend outage, letting unauthenticated traffic through. The judgment problem is to decide what belongs at the edge (and what does not), implement cross-cutting concerns consistently and safely, and operate the gateway as a highly available, horizontally scalable, thin layer.

This skill covers what belongs at the edge, traffic shaping, gateway failure modes and high availability, the gateway-service boundary, and operational concerns. It complements the load-balancing-strategies skill (distributing traffic behind the gateway), the network-protocols-and-tls skill (TLS and protocol concerns), and the connection-management skill (connections at the edge). Here the focus is the gateway as the cross-cutting edge layer.

## Core Rules

### Put Cross-Cutting, Stateless Concerns At The Edge; Keep Business Logic In Services

The gateway's value is consolidating concerns every request needs, implemented once and consistently. The boundary between gateway and service is the key decision:

- **Consolidate truly cross-cutting concerns at the edge.** TLS termination, authentication (verifying a token), coarse authorization (is this caller allowed at all), routing, rate limiting, request size validation, and observability (logging, metrics, tracing at the edge) belong at the gateway because every request needs them and centralizing them ensures consistency.
- **Keep business logic, domain rules, and fine-grained authorization in services.** A service's domain logic (what a user can do with a specific resource, business rules, data transformation specific to the service) does not belong in the gateway. Pushing it there couples the gateway to service internals and makes the gateway a second service implementation.
- **Be suspicious of request/response transformation at the edge.** Protocol translation (e.g., external REST to internal gRPC) is a legitimate edge concern; transforming service-specific payloads (renaming fields, computing derived values) couples the gateway to service schemas and should be avoided or kept minimal and versioned.
- **Avoid stateful behavior in the gateway.** The gateway should be stateless (state in dedicated stores — session stores, caches — not in the gateway process), so it can scale horizontally and fail without losing stateful data. A stateful gateway cannot be freely scaled or replaced.

### Implement Traffic Shaping That Protects Without Strangling

Rate limiting, throttling, and quotas at the edge protect services from overload and abuse, but poorly shaped limits harm legitimate traffic:

- **Rate-limit per the right dimension.** Limits per client/API key, per user, per IP, and per service each protect different things; choose the dimension that matches the threat (per-user for fair use, per-service to protect a specific backend). A single global limit is usually too coarse.
- **Distinguish rate limiting (protect the system) from quotas (enforce policy).** Rate limiting prevents overload (throttle when traffic exceeds capacity); quotas enforce business policy (a free-tier user's daily limit). Conflating them produces limits that neither protect well nor enforce policy cleanly.
- **Choose the limiting algorithm for the behavior needed.** Token bucket allows bursts (good for human-driven traffic); fixed/leaky bucket smooths traffic (good for downstream protection); concurrent-request limits protect against slow-consumer overload. Match the algorithm to the traffic and the protection goal.
- **Make limits tunable and per-service, not one-size-fits-all.** Different services and endpoints have different capacities and traffic patterns; global limits throttle some services and fail to protect others. Per-service, per-endpoint limits that are tunable without redeploying let you respond to real traffic.
- **Return useful, spec-compliant throttle responses.** A throttled request should return the right status (429) with headers indicating the limit and when to retry (Retry-After), so well-behaved clients back off correctly. Silently dropping or returning a generic error breaks client retry logic.

### Fail Safely: The Gateway Must Fail Closed On Security, Not Fail Open

Because the gateway handles security, its failure mode is safety-critical. A gateway that fails open on an auth-backend outage lets unauthenticated traffic through; one that fails closed may reject traffic but stays secure:

- **Fail closed for authentication and authorization.** If the auth backend is unavailable, the gateway must reject requests (fail closed), not admit them unauthenticated (fail open). Fail-open on auth is a security breach waiting for the next backend hiccup.
- **Degrade gracefully for non-security concerns.** If a non-critical edge feature (a logging pipeline, a transformation) fails, the gateway should pass traffic through with that feature degraded, not fail the request. Distinguish security-critical (fail closed) from non-critical (degrade) concerns.
- **Handle downstream service failure with appropriate behavior.** When a backend is unavailable, the gateway should return a clear error, apply a circuit breaker (see the load-balancing skill), or serve a cached/fallback response where appropriate, not hang or cascade the failure.
- **Test the failure modes.** What happens when the auth backend goes down? When a rate-limit store is unreachable? When a backend times out? Each failure mode must have a defined, tested behavior; untested failure modes fail in the worst way at the worst time.

### Make The Gateway Highly Available And Horizontally Scalable

The gateway is a single point of failure for all traffic and a bottleneck at scale. Its availability and scalability are system-critical:

- **Run multiple gateway instances with no single point of failure.** Deploy across availability zones with a load balancer in front; no single gateway instance should be critical. A single gateway instance is a single point of failure for the entire system.
- **Scale the gateway with total traffic.** The gateway handles all external traffic, so its capacity must scale with aggregate demand, not per-service demand. Under-provisioning the gateway throttles or fails all services; monitor and scale it as a first-class capacity concern.
- **Keep the gateway stateless for horizontal scaling.** Stateless gateway instances can be added and removed freely and load-balanced; stateful instances require sticky sessions or replicated state, complicating scaling and failure. Move state out of the gateway.
- **Health-check and auto-replace unhealthy instances.** The gateway's own health must be monitored, and unhealthy instances automatically replaced, so a failing gateway does not degrade all traffic.

### Govern The Gateway-Service Boundary And Configuration

A gateway's configuration (routes, limits, auth policies) is itself a critical, versioned artifact that must be governed:

- **Treat gateway configuration as code.** Routes, rate limits, and policies should be version-controlled, reviewed, and deployed through the same pipeline as application code, not edited in an admin UI. UI-edited gateway config drifts, is unreviewed, and cannot be reproduced.
- **Define clear ownership of gateway configuration.** Is the gateway owned by a platform team (consistent, but a bottleneck for service teams) or by service teams (flexible, but risks inconsistency)? Define the model and the guardrails (standards, review) that keep it consistent.
- **Version and test configuration changes.** A gateway misconfiguration affects all traffic; test configuration changes (in staging, or via canary) and have a fast rollback. A bad route or limit pushed to the gateway is a system-wide incident.
- **Decouple gateway deployment from service deployment where possible.** A service change should not require a gateway change (and vice versa) unless the contract genuinely changed; coupling their deployment creates coordination overhead and risk.

## Common Traps

### Business Logic In The Gateway

Pushing service-specific logic, domain rules, or fine-grained authorization into the gateway, coupling it to service internals until it becomes an unmaintainable, stateful, service-coupled component. Keep the gateway thin; business logic stays in services.

### Inconsistent Authn/Authz Between Gateway And Services

Authentication handled at the gateway for some services and in the service for others, producing inconsistent security boundaries and gaps. Consolidate cross-cutting auth at the edge consistently, with fine-grained authorization in services.

### Failing Open On An Auth Backend Outage

A gateway that admits unauthenticated traffic when the auth backend is unavailable, turning a backend hiccup into a security breach. Fail closed for security-critical concerns; test the failure mode.

### A Single Gateway Instance As A Single Point Of Failure

One gateway instance with no redundancy, so its failure is a total outage, or a stateful gateway that cannot scale horizontally. Run multiple instances across zones, keep the gateway stateless, and health-check and auto-replace.

### Rate Limits Too Coarse Or Too Aggressive

A single global limit that throttles legitimate bursty traffic while failing to protect a specific service, or limits that are not tunable per service/endpoint. Limit per the right dimension, choose the algorithm for the behavior, and make limits tunable.

### UI-Edited Gateway Configuration That Drifts

Gateway routes and policies edited in an admin UI with no version control or review, drifting from intended state and irreproducible. Treat gateway configuration as code, versioned and deployed through a pipeline.

### Transformation Logic Coupling The Gateway To Service Schemas

Request/response transformation that reaches into service-specific payloads, so a service change requires a gateway change. Keep transformation minimal and versioned; push service-specific logic into the service.

## Self-Check

- [ ] The gateway handles cross-cutting, stateless concerns (TLS, auth, coarse authz, routing, rate limiting, validation, observability) consolidated once and consistently, while business logic, domain rules, and fine-grained authorization remain in services, and transformation is minimal and versioned.
- [ ] The gateway is stateless (state in dedicated stores, not in the gateway process), so it scales horizontally and fails without losing stateful data.
- [ ] Traffic shaping protects without strangling: limits are per the right dimension (client/user/service), distinguish rate limiting from quotas, use the algorithm matched to the behavior (token bucket for bursts, leaky for smoothing, concurrent limits for slow consumers), are per-service and tunable, and return spec-compliant throttle responses (429 with Retry-After).
- [ ] The gateway fails safely: fail-closed on authentication/authorization (rejects, not admits, when the auth backend is down), degrades gracefully for non-security concerns, handles downstream failure with defined behavior (error, circuit breaker, fallback), and failure modes are tested.
- [ ] The gateway is highly available and horizontally scalable: multiple instances across availability zones, scaled with total traffic, stateless for free scaling, and health-checked with auto-replacement.
- [ ] Gateway configuration is governed: treated as code (version-controlled, reviewed, pipeline-deployed, not UI-edited), with clear ownership and guardrails, versioned/tested changes with fast rollback, and deployment decoupled from services where the contract has not changed.
- [ ] The highest-risk cases were verified — an auth backend outage that failed closed, a rate limit that protected a specific service without throttling legitimate traffic, a gateway instance failure with no total outage, and a configuration change rolled back cleanly — not only the clean routing demo.
