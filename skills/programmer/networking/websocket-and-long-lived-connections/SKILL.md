---
name: websocket_and_long_lived_connections.md
description: Use when the agent is designing or operating a system that uses WebSocket, Server-Sent Events, HTTP long-polling, or other long-lived connections (chat, live dashboards, notifications, multiplayer, streaming delivery), handling connection lifecycle (open, ping/pong, reconnect, close), scaling connection fan-out across many servers, managing backpressure over a live socket, deciding between WebSocket and SSE and polling, or handling the operational realities of long-lived connections (connection limits, sticky sessions, memory per connection, idle and zombie connections). Also covers the failure modes of unbounded connections exhausting server resources, missing heartbeats leaving zombie connections, reconnect storms after an outage, no backpressure flooding slow clients, and the recurring mistake of treating a long-lived connection like a normal HTTP request when its stateful, long-lived, fan-out nature changes nearly every operational property.
---

# WebSocket And Long-Lived Connections

A normal HTTP request is short-lived and stateless: a request arrives, the server responds, the connection closes, and the server holds no per-connection state. A WebSocket, Server-Sent Events stream, or long-poll is the opposite: the connection stays open for minutes or hours, the server holds per-connection state, messages flow in both directions over time, and the same server must track and fan out to thousands or millions of simultaneous connections. The judgment problem is that the long-lived, stateful, fan-out nature of these connections changes nearly every operational property. Resource accounting shifts (each connection holds memory and a file descriptor for its lifetime, not for a request's). Reliability shifts (a dropped connection must be detected and reconnected, and the reconnect must not lose or duplicate messages). Scaling shifts (fan-out to a million connections cannot happen from one server, and a client's connection may need to be sticky to the server holding its state). Backpressure shifts (a slow client on a live socket can flood its buffers and block the server). Treating a long-lived connection like a normal request produces a system that exhausts resources, accumulates zombie connections, storms on reconnect, and floods slow clients.

Agents tend to under-invest here because opening a WebSocket and sending a message works in the demo, and the short-lived mental model carries over. The harm appears at scale and over time. Connections accumulate with no heartbeat, and the server holds millions of zombie connections from clients that disconnected uncleanly, exhausting memory and file descriptors. An outage causes every client to reconnect simultaneously, and the reconnect storm overwhelms the servers that survived. A slow client cannot keep up with the message rate, and with no backpressure the server buffers unboundedly or blocks, degrading all connections on that server. A client reconnects to a different server that does not hold its state or its missed messages, so the user sees gaps or duplicates. The judgment problem is to choose the right protocol, manage connection lifecycle (heartbeats, reconnect, cleanup), scale fan-out, apply backpressure, and handle the operational realities of long-lived stateful connections.

This skill covers protocol choice, connection lifecycle and health, scaling and fan-out, backpressure, and operational concerns. It complements the connection-management skill (general connection handling), the load-balancing-strategies skill (distributing connections), and the stream-processing skill (backpressure concepts). Here the focus is long-lived, stateful, bidirectional connections.

## Core Rules

### Choose The Right Protocol For The Requirement

WebSocket, Server-Sent Events (SSE), long-polling, and even plain polling each fit different requirements, and the choice determines complexity, directionality, and infrastructure:

- **WebSocket for bidirectional, low-latency, message-stream communication.** Two-way real-time messaging (chat, multiplayer, collaborative editing) fits WebSocket. The tradeoff is the most complexity (custom framing, lifecycle, reconnection logic) and infrastructure (sticky sessions, connection-aware load balancing).
- **Server-Sent Events for server-to-client streaming over HTTP.** When the server pushes and the client rarely needs to send (live updates, notifications, dashboards), SSE is simpler than WebSocket, works over plain HTTP, and reconnects automatically. It is unidirectional; choose it when one-way server push suffices.
- **Long-polling or polling as a fallback, not a default.** Polling is simpler and works everywhere (including restrictive proxies), but it is inefficient (latency, overhead) for frequent updates. Use it as a fallback where WebSocket/SSE are not viable, or for low-frequency updates where its simplicity wins.
- **Match the protocol to the real-time need and the infrastructure.** Bidirectional and low-latency needs WebSocket; one-way server push fits SSE; restrictive environments or low-frequency updates may justify polling. Do not default to WebSocket for a one-way push that SSE handles more simply.

### Manage Connection Lifecycle: Heartbeats, Reconnection, Cleanup

Long-lived connections drop (network changes, proxies timeout, clients disappear), and the system must detect drops, reconnect cleanly, and clean up server-side state:

- **Implement heartbeats (ping/pong) to detect dead connections.** A TCP connection can appear open when the client is gone (half-open); periodic heartbeats detect liveness and close dead connections. Without heartbeats, the server accumulates zombies that consume resources indefinitely.
- **Define reconnection with backoff and jitter.** Clients must reconnect on drop, with exponential backoff and jitter to avoid a reconnect storm when many clients drop at once (e.g., after a network blip). A reconnect storm after an outage can overwhelm the servers that survived it.
- **Handle missed messages on reconnect.** When a client reconnects, what did it miss? Define the strategy (resume from a sequence number or cursor, accept a gap, fetch a snapshot) so reconnection does not lose or duplicate messages. A reconnect that silently drops recent messages produces an inconsistent client state.
- **Clean up server-side state on close.** When a connection closes (cleanly or detected-dead), the server must release its state (subscriptions, buffers, session data). Leaked per-connection state accumulates and exhausts memory over time.

### Scale Fan-Out: Connection Distribution, Sticky Sessions, And Coordination

Fan-out to many connections cannot happen from one server, and a client's connection may need to reach the server that holds its state or its subscriptions:

- **Distribute connections across many servers.** A single server cannot hold a million long-lived connections; distribute them, with connection-aware load balancing (a load balancer that tracks connection counts, not just round-robin requests).
- **Use sticky sessions or shared state where connection-specific state matters.** If a connection's server holds its session or subscriptions, the client must reconnect to the same server (sticky), or the state must be shared (in a shared store) so any server can serve any connection. Choose the model and its tradeoff (sticky limits failover; shared adds a dependency and latency).
- **Coordinate fan-out across servers with a pub/sub backbone.** To deliver a message to all subscribers across servers, use a pub/sub layer (Redis, a message broker) so the publishing server does not need to know which servers hold which connections. Each server subscribes and fans out to its local connections.
- **Account for the cost of fan-out.** Delivering to a million connections is a million writes; the fan-out path must be designed for that scale (batching, parallelism, shared infrastructure), not assumed free.

### Apply Backpressure For Slow Clients

On a live socket, the server pushes messages; a slow client (or a bad network) cannot keep up, and without backpressure the server's buffers grow unboundedly or the server blocks:

- **Detect slow consumers and apply backpressure.** Monitor the send buffer per connection; when it grows (the client is not reading), apply backpressure — slow the producer, drop non-critical updates, or close the slow connection — rather than buffering forever.
- **Decide the backpressure policy per use case.** For a live dashboard, dropping stale updates is fine (the latest value matters, not every intermediate). For a chat or transactional stream, dropping is not acceptable; the policy must slow the producer or buffer boundedly and eventually disconnect a persistently slow client.
- **Do not let one slow client degrade others.** A server that blocks on a slow client's socket degrades all connections on that server; backpressure must be per-connection (async, non-blocking sends) so one slow consumer does not stall the rest.
- **Make backpressure observable.** Track buffer depths, slow-consumer events, and drops, so a buildup of slow clients is visible before it degrades the system.

### Handle The Operational Realities Of Long-Lived Connections

Long-lived connections impose operational constraints that short-lived HTTP does not, and these must be designed for:

- **Account for per-connection resource cost.** Each connection holds memory (buffers, state), a file descriptor, and often a goroutine/thread for its lifetime. Provision and monitor for the steady-state connection count, not the request rate; a server sized for request throughput may exhaust memory on connection state.
- **Handle connection and proxy limits.** Proxies, load balancers, and browsers impose limits on connections per client or per server; design around them (connection multiplexing, appropriate timeouts, proxy configuration) or hit limits that manifest as mysterious failures.
- **Secure long-lived connections.** A long-lived connection is an attack surface (resource exhaustion via connection flooding, unauthorized message injection); authenticate the connection (not just the initial request), authorize each message, and rate-limit per connection.
- **Handle idle connections deliberately.** Some connections are legitimately idle (a user away from chat); decide whether to keep them alive (heartbeat, resource cost) or close and reconnect on activity (resource savings, reconnection cost). Do not let idle connections accumulate without a policy.

## Common Traps

### Zombie Connections From No Heartbeats

Connections from clients that disconnected uncleanly, appearing open but dead, accumulating and exhausting memory and file descriptors. Implement heartbeats (ping/pong) to detect and close dead connections.

### Reconnect Storms After An Outage

Every client reconnecting simultaneously after a network blip or outage, overwhelming the surviving servers. Reconnect with exponential backoff and jitter.

### No Backpressure Flooding Slow Clients

A slow client or bad network causing unbounded buffer growth or a blocked server, degrading all connections. Detect slow consumers, apply a per-connection backpressure policy (slow, drop stale, or disconnect), and keep sends non-blocking.

### Missed Or Duplicated Messages On Reconnect

A reconnect that silently drops recent messages or replays them, leaving the client inconsistent. Define reconnect semantics (resume from sequence/cursor, accept gap with snapshot fetch) and handle it explicitly.

### One Server Holding Too Many Connections

A single server attempting to hold a connection count it cannot support, exhausting memory or file descriptors. Distribute connections across servers with connection-aware load balancing and a pub/sub backbone for fan-out.

### Treating A Long-Lived Connection Like A Normal HTTP Request

Carrying over the short-lived, stateless mental model, ignoring per-connection resource cost, lifecycle, backpressure, and scaling — and being surprised by resource exhaustion, zombies, and storms. Treat the connection as long-lived and stateful from the start.

### Sticky Sessions Without Failover, Or Shared State Without Considering The Dependency

Sticky sessions that prevent failover when a server dies (the client's state is lost), or shared state that adds a dependency and latency not accounted for. Choose the model deliberately and design for its tradeoff.

## Self-Check

- [ ] The protocol matches the requirement (WebSocket for bidirectional low-latency, SSE for one-way server push, polling only as fallback or for low frequency), chosen for the real-time need and infrastructure, not defaulted to the most complex option.
- [ ] Connection lifecycle is managed: heartbeats (ping/pong) detect dead connections, reconnection uses backoff with jitter to avoid storms, missed messages are handled on reconnect (resume from sequence/cursor or snapshot), and server-side state is cleaned up on close.
- [ ] Fan-out scales: connections are distributed across servers with connection-aware load balancing, sticky sessions or shared state is chosen deliberately for connection-specific state, a pub/sub backbone coordinates cross-server fan-out, and the fan-out cost is accounted for.
- [ ] Backpressure is applied for slow clients: send buffers are monitored per connection, a per-use-case policy (slow the producer, drop stale updates, disconnect persistent slow consumers) is defined, sends are non-blocking so one slow client does not degrade others, and slow-consumer events are observable.
- [ ] Operational realities are handled: per-connection resource cost (memory, file descriptors, threads) is provisioned and monitored for steady-state connection count, proxy/browser connection limits are designed around, connections are authenticated and per-message authorized and rate-limited, and idle connections have a deliberate policy.
- [ ] The highest-risk cases were verified — zombie connections cleaned by heartbeats, a reconnect storm absorbed by backoff/jitter, a slow client handled by backpressure without degrading others, a server failure with connection state handled by shared state or acceptable reconnection, and resource exhaustion under connection flood — not only the clean single-connection demo.
