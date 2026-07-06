---
name: server-rack-pdu-and-cabinet-power-distribution.md
description: Use when the agent is selecting and installing rack PDUs, sizing cabinet power, managing branch circuits to server racks, choosing metered versus switched versus outlet-metered PDUs, deciding single-phase versus 3-phase rack PDU, and planning rack power density in kW per rack without overloading branch circuits.
---

# Server Rack PDU and Cabinet Power Distribution

The rack PDU is where megawatts of data center power become useful work, and where most power-related outages actually happen. A rack that is fed correctly at the room level can still fail because a single branch circuit in a PDU is overloaded, because two power supplies were both plugged into the same phase, or because nobody knew a circuit was at 95 percent until a failover pushed it over. The judgment problem is that rack power density has grown faster than the legacy assumptions behind branch circuit sizing, and a modern high-density rack can draw more than a single branch can deliver, forcing careful phase balancing and sometimes 3-phase distribution inside the cabinet. Agents miss the issues because the PDU looks like a power strip and the servers appear to run fine, while the real questions — branch loading, phase balance, outlet-level visibility, failover behavior — only become visible at the moment a circuit trips.

## Core Rules

### Size the PDU and Branch Circuits to the Real Rack Load With Margin

A rack PDU is rated for a maximum current per phase and per branch circuit, and the connected load must fit within those ratings with margin for failover and growth. A common mistake is to size the PDU to the nameplate sum of the servers, which grossly overstates real draw, but the opposite error — sizing to an optimistic average — trips the branch when servers fail over or run a peak workload. The defense is to measure or estimate the real draw of the intended IT load, to select a PDU whose branch circuits carry that load at no more than 80 percent of rating under normal operation, and to confirm that under a failover scenario — where one feed's load shifts to the other — no branch exceeds its rating.

### Choose Single-Phase Versus 3-Phase Rack PDU Based on Density

A single-phase rack PDU on a 30-amp 208V circuit delivers a fixed amount of power per branch, which is adequate for low and moderate density racks. As density rises, a single-phase PDU runs out of capacity and a 3-phase PDU, which delivers more total power through the same physical PDU by distributing load across three phases, becomes necessary. The defense is to calculate the target kW per rack, compare it to what single-phase branches can deliver at the available voltage, and move to 3-phase distribution when the density exceeds what single-phase can support, while ensuring the load is balanced across the three phases so no one phase is overloaded.

### Select Metered, Switched, or Outlet-Metered PDU to Match the Operational Need

A basic PDU distributes power with no intelligence. A metered PDU reports total and per-branch current so operators can see loading. A switched PDU adds remote outlet switching so a locked-up server can be power-cycled remotely. An outlet-metered PDU reports current per outlet, enabling precise chargeback and the detection of a single server drawing abnormally. The defense is to match the PDU intelligence to the operational requirement: metered where loading must be monitored, switched where remote reboot is required for lights-out operation, and outlet-metered where granular billing or anomaly detection is needed, recognizing that more intelligence costs more and adds a management dependency.

### Match Outlet Types to the Equipment and Voltage

Rack PDUs offer C13 outlets (typically for servers and smaller gear up to about 10 amps) and C19 outlets (for larger gear such as blade chassis and large switches up to 16 amps), and the mix and voltage must match the equipment being installed. A PDU with the wrong outlet mix forces adapters, which are discouraged in data centers, or leaves outlets unusable. The defense is to inventory the equipment's power supplies and their connector types and voltages before selecting the PDU, to choose a PDU with the right C13/C19 mix, and to confirm the voltage matches what the power supplies are rated to accept, because a 120V server plugged into a 208V PDU will fail immediately.

### Plan Rack Power Density in kW per Rack and Coordinate With Cooling

Rack power density — expressed in kW per rack — drives both the PDU selection and the cooling design. A rack drawing 5 kW has very different PDU and cooling requirements than one drawing 20 kW or 40 kW, and the two must be planned together because power without cooling causes thermal shutdown. The defense is to set the target density early, to size the PDU and the branch feeds to that density with margin, and to coordinate with the cooling team so that the airflow or liquid capacity matches the heat the rack will reject, because a fully powered rack with inadequate cooling is a future outage.

### Manage Power Connections So Failover Does Not Overload a Branch

Dual-corded servers are designed so that either power supply can carry the load, and in normal operation each supply draws roughly half. When one feed fails, the surviving supply draws the full load, doubling its current. If too many servers on one branch are paired with servers on another branch, a feed failure can shift enough load to trip the surviving branch. The defense is to distribute dual-corded server connections so that A and B feeds are balanced, to monitor branch loading so that no branch exceeds roughly 80 percent normally — leaving room for the failover surge — and to verify under a simulated failover that no surviving branch exceeds its rating.

## Common Traps

### Sizing the PDU to Server Nameplate and Wasting Capacity

The installer totals server power supply nameplate ratings and sizes the PDU to that sum. The false signal is that the total is a safe upper bound. The mechanism of failure is that nameplate vastly overstates real draw, so the PDU is oversized and the branch circuits are underutilized, wasting PDU and branch capacity and forcing more PDU installations than necessary. The harm is wasted money and branch circuits, and ironically a false sense that there is plenty of headroom that masks real loading measured differently.

### Sizing the PDU to an Optimistic Average and Tripping on Failover

The installer sizes the PDU to the measured average draw and assumes the margin is adequate. The false signal is that the average fits comfortably. The mechanism of failure is that when one feed fails, the surviving branch absorbs the failover surge and exceeds its rating, tripping the branch and dropping servers that the redundancy was supposed to protect. The harm is an outage caused not by the failure but by the recovery, the exact scenario dual feeds were meant to prevent.

### Loading One Phase of a 3-Phase PDU and Tripping It

The installer plugs servers into a 3-phase PDU without balancing the phases and loads one phase near its limit. The false signal is that the total PDU load is within rating. The mechanism of failure is that the PDU's per-phase branch breaker trips on the overloaded phase even though the total is fine, dropping part of the rack. The harm is a partial rack outage and a mystery trip that is hard to diagnose without per-phase metering.

### Plugging Both Power Supplies of a Dual-Corded Server Into One Feed

The installer, reaching for the nearest outlets, plugs both power supplies of a dual-corded server into the same PDU or the same phase. The false signal is that the server has redundant power supplies. The mechanism of failure is that both supplies are on one feed, so a failure of that feed drops the server entirely, defeating the redundancy. The harm is a server that was believed to be protected but drops on the first feed failure.

### Using Adapters to Fit the Wrong Outlet or Voltage

The installer uses a C13-to-C19 adapter or a voltage adapter to connect equipment to a PDU with the wrong outlets or voltage. The false signal is that the connection works and the equipment powers on. The mechanism of failure is that the adapter may exceed the outlet's current rating, or the voltage may be wrong for the power supply, causing overheating or immediate failure. The harm is an overloaded outlet, a damaged power supply, and an unmanaged connection that nobody documented.

### Ignoring Outlet-Level Monitoring Until a Branch Trips

The facility installs metered PDUs but never reviews the per-branch or per-outlet data. The false signal is that the PDUs are present and "monitoring." The mechanism of failure is that branch loading creeps up as servers are added, and the first indication of overload is the trip itself, with no warning. The harm is surprise outages that monitoring was supposed to prevent, because the data was collected but never acted upon.

## Self-Check

- Did I size the PDU and branch circuits to the real rack load, with branches running at no more than 80 percent of rating under normal operation?
- Did I choose single-phase versus 3-phase PDU based on the target kW per rack, and is the load balanced across the three phases?
- Did I select metered, switched, or outlet-metered PDU to match the operational requirement for monitoring, remote reboot, or chargeback?
- Did I inventory equipment power supplies and match the PDU's C13/C19 outlet mix and voltage to the equipment, avoiding adapters?
- Did I set the rack power density target early and coordinate PDU selection with the cooling design so power and cooling are matched?
- Did I distribute dual-corded server connections to balance A and B feeds, and did I verify under simulated failover that no surviving branch exceeds its rating?
- Did I configure monitoring and alerting on per-branch and per-outlet current so that overload is detected before a trip?
- Is the rack power design documented so that adds, moves, and changes can be checked against branch and phase capacity?
