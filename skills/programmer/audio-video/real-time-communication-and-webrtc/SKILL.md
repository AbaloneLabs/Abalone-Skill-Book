---
name: real_time_communication_and_webrtc.md
description: Use when the agent is building real-time communication features, WebRTC pipelines, voice or video calling, live streaming with sub-second latency, conferencing infrastructure, or handling ICE/STUN/TURN negotiation, jitter buffers, packet loss concealment, adaptive bitrate, and SFU versus peer-to-peer topology decisions.
---

# Real-Time Communication and WebRTC

Real-time communication (RTC) is a different engineering problem from streaming media. A video-on-demand pipeline can buffer seconds of content to smooth over network hiccups, but a live call cannot. Latency is the primary quality metric, and every millisecond of added delay compounds into awkward turn-taking, echo, and the feeling that the call is "broken." Agents who approach RTC with streaming assumptions tend to over-buffer, over-compress, and ship calls that look fine in a LAN test but collapse on real mobile networks.

The hidden difficulty is that RTC sits at the intersection of three independently hard problems: media capture and encoding, unreliable transport over the public internet, and real-time congestion control that must react faster than TCP ever could. A working call also requires NAT traversal (ICE), signaling, security (DTLS-SRTP), and often a server topology decision (mesh, SFU, or MCU) that is expensive to reverse after launch. The agent must reason about tail behavior, not average behavior, because a call is judged by its worst moment.

## Core Rules

### Treat latency as the dominant constraint, not a side effect

In RTC, latency and quality are in direct tension. The instinct to raise quality (higher resolution, higher bitrate) is often wrong because higher bitrate increases queuing delay under congestion. Define a latency budget first (e.g., mouth-to-ear under 150 ms is "good," 150-300 ms is "acceptable," above 400 ms feels broken for conversation), then choose quality targets that fit the budget. Prefer dropping resolution or frame rate over growing the jitter buffer. A choppy-but-live call beats a smooth-but-delayed one.

### Understand the topology decision and commit deliberately

The choice between full mesh (P2P), SFU (Selective Forwarding Unit), and MCU (multipoint control unit) shapes everything downstream and is hard to reverse:

- **Full mesh (P2P):** Each peer sends media directly to every other peer. Bandwidth and CPU scale as O(n^2). Fine for 2-4 participants on good networks. Do not use for larger groups.
- **SFU:** Each peer sends one stream up; the server forwards selected streams down. Bandwidth on the client is O(n) for receive. This is the dominant choice for group calls because it preserves end-to-end latency and lets each client receive a quality appropriate to its bandwidth (simulcast).
- **MCU:** Server decodes and composites everyone into one stream. Saves client bandwidth but adds decode/encode latency and a single point of quality loss. Rarely the right choice for interactive calls today.

The trap is starting with P2P because it is "free" (no server) and discovering at 5 participants that you must rebuild around an SFU. Decide topology based on your realistic maximum participant count, not your launch count.

### Get ICE and NAT traversal right or nothing works

ICE (Interactive Connectivity Establishment) is the mechanism by which two peers find a network path through NATs and firewalls. It gathers candidates of increasing cost: host (local), server-reflexive (STUN, your public IP), and relay (TURN, a server that proxies traffic). Most call failures on real networks are ICE failures, not media failures.

- Always deploy a TURN server for production. Corporate networks, carrier-grade NAT, and symmetric NATs will defeat host and STUN candidates. Without TURN, a meaningful fraction of users simply cannot connect.
- Budget for TURN bandwidth. TURN relays all media for users who need it, which can be expensive. Monitor the relay ratio; a high relay ratio is expected on restrictive networks but a sudden spike may indicate a STUN/infrastructure problem.
- Test ICE against hostile networks, not just your office Wi-Fi. Use network conditioners to simulate packet loss, jitter, bandwidth limits, and symmetric NAT.

### Build adaptive bitrate on top of honest congestion signals

RTC cannot wait for TCP-style congestion detection. Use the transport's built-in congestion controller (GCC in WebRTC) or implement one that reacts to packet loss and delay-gradient within tens of milliseconds. React by adjusting send bitrate via the encoder, not by buffering. Simulcast (sending multiple resolutions) lets an SFU forward a lower layer to weak clients without forcing everyone down. The agent must verify that bitrate adaptation actually triggers under load; a controller that never reduces bitrate under loss will produce a call that freezes instead of degrading gracefully.

### Handle packet loss explicitly, never silently

Audio and video tolerate loss differently. Audio packet loss is immediately audible; use packet loss concealment (PLC) and forward error correction (FEC, like RED/opus in-band FEC) for low-latency recovery without retransmission. Video loss causes artifacts and keyframe requests; a lost packet should not stall the decoder waiting for a retransmit beyond a few RTTs. Prefer generating a new keyframe on demand (PLI/FIR) over waiting for the next interval. Never design an RTC pipeline that assumes reliable, ordered delivery; if you find yourself wanting TCP semantics, you are building the wrong thing.

### Separate signaling from media transport

Signaling (offer/answer, candidate exchange, room state) is not standardized in WebRTC and is your responsibility. Use any reliable transport (WebSocket, your existing app protocol) but keep it strictly out of the media path. Signaling must be resilient to flaky connections with reconnection and state reconciliation. A common bug is treating signaling reliability as media reliability; a signaling reconnect must not tear down an established media session.

### Plan for security from the first commit

Every RTC media stream must be encrypted (SRTP with DTLS key exchange). Never ship unencrypted media, even for "internal" tools. Verify certificate fingerprints in signaling to prevent man-in-the-middle. For server topologies, decide explicitly whether you need end-to-end encryption (which an SFU cannot decrypt) or hop-by-hop (which an SFU can inspect/record); this decision affects recording, transcription, and moderation features.

## Common Traps

### Optimizing average latency instead of tail latency

A call with 80 ms average latency but 900 ms spikes feels worse than a call with a steady 200 ms. Agents often report the mean from a LAN test and declare victory. Measure and optimize the 95th and 99th percentile latency under realistic network conditions. The tail is the user experience.

### Assuming a working LAN demo means a working product

LAN and localhost tests have near-zero loss, jitter, and NAT complexity. They validate almost nothing about production. A call that works perfectly in the office and fails for 15% of mobile users is the default outcome without hostile-network testing.

### Over-buffering to eliminate jitter

A jitter buffer smooths packet arrival timing, but every added millisecond is added latency. Agents under time pressure often grow the buffer until jitter artifacts disappear, trading liveliness for smoothness. Tune the buffer adaptively and keep it small; accept minor jitter artifacts over perceptible delay for conversational use.

### Ignoring echo and audio processing

Echo cancellation, noise suppression, and automatic gain control are not optional polish; they are required for a usable call. Without acoustic echo cancellation (AEC), audio from speakers re-enters the microphone and the other party hears themselves delayed. Enable the platform's audio processing module (the WebRTC audio module includes AEC/NS/AGC) and test with real speakers, not just headphones.

### Treating TURN as optional because STUN works in testing

STUN fails on symmetric NAT and many corporate networks. If TURN is not deployed, those users cannot connect at all. The failure is silent from the developer's perspective because it never happens on the test network.

### Forgetting that simulcast requires SFU cooperation

Enabling simulcast on the sender does nothing useful unless the SFU actively selects and switches layers per receiver. Agents enable the flag, see three encodings, and assume adaptation works. Verify end-to-end that a bandwidth-constrained receiver actually receives a lower layer.

## Self-Check

- Have you defined a mouth-to-ear (or glass-to-glass) latency budget, and does your design keep the jitter buffer small enough to meet it under realistic network conditions?
- Is a TURN server deployed and tested, and do you know what fraction of production sessions require relay candidates?
- Did you test ICE connectivity against simulated packet loss, jitter, bandwidth limits, and symmetric NAT, not just a clean LAN?
- Does your congestion control actually reduce send bitrate under induced loss, and does a weak receiver in a simulcast setup receive a lower layer?
- Is all media encrypted (SRTP/DTLS), and are certificate fingerprints verified in signaling?
- For audio, are AEC, noise suppression, and AGC enabled and tested with open speakers (not just headphones)?
- Have you measured 95th/99th percentile latency, not just the mean, and is the tail within an acceptable range for conversation?
- For group calls, is the topology (mesh/SFU/MCU) chosen for your realistic maximum participant count rather than your launch count?
- Does signaling reconnect independently of an established media session without tearing it down?
- For packet loss, do you have PLC/FEC for audio and on-demand keyframe requests (PLI/FIR) for video, rather than relying on retransmission that stalls the decoder?
