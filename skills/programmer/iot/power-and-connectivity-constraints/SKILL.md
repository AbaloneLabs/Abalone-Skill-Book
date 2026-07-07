---
name: power_and_connectivity_constraints.md
description: Use when the agent is designing a battery-powered or constrained-network IoT system and reasoning about power budget, battery life, energy harvesting, sleep and wake duty cycling, or current draw budgets; dealing with duty cycle limits and legal airtime caps on regulated networks (LoRaWAN, NB-IoT, Sigfox, sub-GHz ISM); handling intermittent connectivity, store-and-forward, and connectivity survival; designing mesh routing, self-healing, and gateway redundancy; trading off reporting frequency against battery and network capacity; modeling metered or cellular network cost; or diagnosing a fleet design that drains batteries in weeks, exceeds regulatory airtime, drowns a metered uplink in cost, or collapses when a gateway fails. Covers the constraint-side judgment (power budget, duty cycle, intermittent connectivity survivability) distinct from protocol selection and edge-vs-cloud tiering.
---

# Power And Connectivity Constraints

A battery-powered IoT device is an energy budget with a sensor attached, and a constrained-network deployment is an airtime budget with a fleet attached. Both budgets are finite, often legally capped, and consumed by everything the device does — and agents trained on always-connected, mains-powered systems routinely design as if power and connectivity were free. They pick a reporting frequency that "feels responsive," add a feature that wakes the radio, or funnel a region through one gateway — and the result is a fleet whose batteries last weeks instead of years, that exceeds the legal duty-cycle limit on its radio band, or whose metered cellular bill is ten times what was projected. The judgment problem is treating power, airtime, and connectivity survival as hard constraints that shape the design — not as optimizations applied after the fact.

The harm of ignoring these constraints is structural and late-appearing: a design that works in a lab demo (plugged in, on Wi-Fi, reporting every second) ships to the field and dies, throttles, or costs a fortune. These constraints must be modeled before the design is committed, because they determine reporting frequency, topology, sleep behavior, and even what features are possible.

## Core Rules

### Build A Power Budget Before Choosing Anything Else

Battery life is not a hope; it is an arithmetic consequence of how much current the device draws in each state and how long it spends there. Build the budget explicitly: average sleep current (which dominates for a device that sleeps 99.9% of the time), active current when sensing, the radio's transmit and receive current (almost always the largest spike), and the frequency and duration of each wake cycle. A device that draws 5 µA in sleep and 100 mA for 2 seconds per hour to transmit has a calculable lifetime from a given battery; a device designed without this arithmetic has an unknown lifetime that is usually far shorter than expected. Account for the non-obvious consumers: the quiescent current of regulators, leakage, sensors that never fully power down, and the energy cost of a TLS handshake that runs every wake. Name the target lifetime in years, work backward to the energy available per day, and let that budget drive reporting frequency, sleep depth, and hardware selection. A design without a power budget is a design whose battery life is a surprise.

### Treat Duty Cycle And Airtime As Legal Or Hard Limits, Not Targets

On regulated low-power networks — LoRaWAN (sub-GHz ISM bands, typically 1% duty cycle), Sigfox, and some NB-IoT deployments — the airtime a device may consume is legally capped by spectrum regulations, not merely a performance concern. Exceeding the duty cycle is non-compliant and can result in throttling, device lockout, or regulatory trouble, and the cap is shared: a device that sends long payloads frequently will hit the limit and be forced silent when it most needs to report. Model airtime explicitly: payload size, spreading factor / data rate (which multiplies airtime dramatically at long range), and message frequency against the regulatory cap. The tradeoff is sharp — doubling range often multiplies airtime, and a "responsive" reporting interval can blow the daily airtime budget in the first hour. Treat the duty cycle as a hard ceiling that constrains payload size, frequency, and data rate together, and design reporting around it. A fleet that exceeds duty cycle is a fleet that goes silent at the worst moment, or that is simply illegal.

### Trade Reporting Frequency Against Battery And Capacity Deliberately

Reporting frequency is the single largest lever on both battery life and network load, and it is almost always chosen too high. Every transmission costs energy (radio on, possibly a handshake) and airtime/bandwidth, and the marginal value of the Nth sample per hour is usually far less than its cost. Decide reporting frequency by the information rate the consumer actually needs, not by a desire for "real-time" data: a temperature that changes slowly does not need per-second reporting; an alarm needs to be prompt but rare. Batch samples on-device and transmit aggregates or change-events rather than raw streams, and use adaptive reporting (more when something is happening, less when idle). On the network side, aggregate fleet traffic against gateway and backhaul capacity — a gateway that handles 100 devices reporting hourly may drown under 100 devices reporting every minute. Make frequency a conscious tradeoff against battery lifetime and network capacity, with the consumer's actual information need as the floor.

### Design For Intermittent Connectivity And Survivable Topology

Real deployments are intermittently connected by design: battery devices sleep and are "offline" between transmissions; cellular and mesh networks drop in and out; gateways restart; backhaul fails. A system that assumes connectivity and fails or loses data when it drops is fragile by construction. The robust pattern is store-and-forward: the device buffers readings locally (in non-volatile memory, sized for the expected outage), transmits when connectivity returns, and the consumer reconciles backfilled data with a clear "this is delayed" flag so it is not mistaken for live data. Topology determines what happens when a node fails: a star topology where every device funnels through one gateway has a single point of failure (lose the gateway and the region goes dark), while a mesh is self-healing but adds latency and forces relay nodes to stay awake (breaking the low-power sleep of leaf devices). Decide per data type how long to buffer and what to drop if the buffer fills, and match topology to the survivability requirement — where a region going dark is unacceptable, use mesh self-healing, gateway redundancy, or local autonomous operation. Design for the failure, not just the happy path.

### Model Metered And Constrained-Network Cost At Fleet Scale

On metered networks (cellular, satellite), bandwidth is a recurring per-byte cost that scales with fleet size and reporting frequency, and it is easy to design a system whose connectivity bill exceeds the hardware cost. Model the cost explicitly: payload size × frequency × fleet size × price-per-unit, plus the often-overlooked connection overhead (keepalives, handshakes) that can dominate a small payload. A fleet of 10,000 devices sending a 1 KB payload every minute is over 14 GB/month before overhead — and on some plans, the keepalive traffic alone costs more than the data. Satellite multiplies this by orders of magnitude. Optimize at the source (smaller payloads, less frequent reporting, batching, compression) because every byte saved is multiplied by fleet size and time. The recurring error is designing for functionality and discovering the cost only at the first monthly bill, by which point the deployed design is expensive to change. Model metered cost at fleet scale before committing the design.

## Common Traps

### Choosing Reporting Frequency By Feel

Picking a per-second or per-minute interval because it "feels responsive," then draining batteries and saturating the network for data whose information content is far lower than the reporting rate. Choose frequency by the consumer's information need against the energy and capacity budget.

### Exceeding Duty Cycle On Regulated Networks

Sending frequent or large payloads on LoRaWAN or other duty-cycle-regulated bands, hitting the legal airtime cap and being forced silent or made non-compliant. Model airtime as a hard limit.

### No Power Budget, Battery Life A Surprise

Designing the device and its behavior without computing the current budget, so battery life is unknown until deployment and invariably shorter than expected. Build the budget first and work backward from the target lifetime.

### Assuming Always-On Connectivity Or A Single Point Of Failure

Posting readings in real time with no local buffering, so network dropouts produce silent data gaps; or funneling a region through one gateway with no redundancy or mesh self-healing, so one failure darkens the whole region. Use store-and-forward and match topology to the survivability requirement.

### Discovering Metered Cost On The First Bill

Designing for functionality and learning the cellular or satellite cost only after deployment, when the design is expensive to change — and ignoring connection overhead (handshakes, keepalives) that often dominates a small transmission. Model metered cost at fleet scale before committing.

## Self-Check

- [ ] A power budget was built before design: sleep current, active/sense current, radio TX/RX current, wake frequency and duration, and non-obvious consumers (regulator quiescent, handshake energy) are summed against the battery and a target lifetime in years.
- [ ] On regulated networks (LoRaWAN, NB-IoT, Sigfox), duty cycle and airtime are modeled as legal/hard limits: payload size, data rate/spreading factor, and message frequency are confirmed within the regulatory cap, not treated as targets.
- [ ] Reporting frequency is a deliberate tradeoff against battery lifetime and network/backhaul capacity, driven by the consumer's actual information need; batching, aggregation, and adaptive reporting are used to cut transmissions.
- [ ] Intermittent connectivity is treated as normal: the device buffers readings in non-volatile memory sized for expected outages, uses store-and-forward on reconnection, and backfilled data is flagged so it is not mistaken for live data.
- [ ] Topology survivability is designed for failure: single-gateway single-points-of-failure are identified, and where a region going dark is unacceptable, mesh self-healing, gateway redundancy, or local autonomous operation is used.
- [ ] Metered/constrained-network cost is modeled at fleet scale (payload × frequency × fleet × price, plus connection overhead) before the design is committed, and optimization happens at the source.
- [ ] The full cost of a transmission — including handshakes, keepalives, and headers, not just the payload — is accounted for in both energy and bandwidth models.
- [ ] A worst-case projection was run: the fleet at full size, reporting at the chosen frequency, on the chosen network, for the target lifetime — confirming battery life, duty-cycle compliance, and metered cost are all within acceptable bounds.
