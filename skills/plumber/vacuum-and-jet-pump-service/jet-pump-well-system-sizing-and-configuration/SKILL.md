---
name: jet-pump-well-system-sizing-and-configuration.md
description: Use when the agent is sizing or configuring a jet pump well system, choosing between shallow-well and deep-well (two-pipe or packer) ejector designs, evaluating suction lift limits and venturi performance, selecting the pressure tank and precharge, assessing well yield and recovery, or diagnosing cavitation, no-water, and pump burnout conditions in a rural water supply.
---

# Jet Pump Well System Sizing and Configuration

A jet pump moves water by using a venturi (ejector) down-well to push water to the surface rather than lifting it, which lets a surface pump serve wells beyond the practical suction-lift limit of a straight centrifugal pump, but only if the system is sized to the well's depth, yield, and recovery. The judgment problem is that a jet pump's performance is governed by the interaction of suction lift, ejector geometry, well yield, and pressure-tank precharge, and an undersized ejector, a well that yields less than the pump needs, or a tank with the wrong precharge produces cavitation, intermittent water, or a pump that runs dry and burns out. The harm is no water, a destroyed pump motor, or a system that never delivers rated pressure. This skill covers the shallow-versus-deep, ejector, suction-lift, tank, and yield decisions that make a jet pump system reliable.

## Core Rules

### Choose Shallow-Well or Deep-Well Configuration Based on Actual Pumping Level

A shallow-well jet pump (single-pipe, ejector built into the pump body) is practical where the pumping level (the water level while pumping, not the static resting level) is within roughly 25 feet of the pump, because atmospheric pressure can only lift water that far in practice. A deep-well jet pump (two-pipe or packer ejector, with the venturi down-well) can push water from far deeper because the ejector assists lift, with practical depths to several hundred feet depending on ejector sizing. The disciplined rule is to measure the pumping level under draw, not the static level, and to choose shallow-well only if the pumping level stays within the practical suction-lift limit at peak demand. Choosing shallow-well for a deep well guarantees cavitation and no water; choosing deep-well for a shallow well wastes cost but still works.

### Match the Ejector (Venturi) to the Well Depth and Pump Capacity

The ejector is the heart of a jet system: it converts pressure from the drive water into suction and lift at the nozzle. Ejectors are depth-specific (a shallow-well ejector will not work in a deep two-pipe setup, and a deep-well ejector is sized to a depth range and a pump GPM). The trap is installing a generic or wrong-depth ejector, or pairing an ejector with a pump GPM it was not designed for, which kills venturi performance. The disciplined rule is to select the ejector from the pump manufacturer's chart for the actual well depth and required delivery GPM, to use the correct nozzle and throat, and to verify the drive-water ratio is per design. A mismatched ejector moves little water and overheats the pump.

### Respect the Practical Suction Lift Limit of About 25 Feet

Even with a perfect shallow-well jet pump, atmospheric pressure (about 14.7 psi, equivalent to 33.9 feet of head at sea level) limits suction lift to roughly 25 feet practical after losses, and less at altitude or with a low-yield well where the pumping level drops. The trap is designing to the theoretical 33 feet and discovering the pump cavitates at 27 because of friction and vapor pressure. The disciplined rule is to treat 25 feet of pumping-level lift as the shallow-well ceiling (lower at altitude), to measure the pumping level under peak draw, and to move to a deep-well ejector or a submersible when the pumping level approaches the limit. Cavitation is the symptom of exceeding suction lift, and it destroys impellers.

### Size the Pressure Tank and Set the Precharge to the Cut-In Pressure

The pressure tank prevents rapid pump cycling (which burns out the motor and switch) by storing pressurized water, and its performance depends on the air precharge. The precharge (the air charge in the tank with no water pressure) must be set 2 to 4 psi below the pump cut-in pressure, so the tank empties most of its water before the pump starts and the bladder is not strained. The trap is a factory precharge left at 38 psi with a 30/50 switch, so the tank delivers almost no water and the pump short-cycles. The disciplined rule is to size the tank to the pump GPM and the desired cycle volume (drawdown), to set the precharge to cut-in minus 2 to 4 psi with the system depressurized, and to check the precharge annually. An undersized or mischarged tank shortens pump and switch life dramatically.

### Verify Well Yield and Recovery Exceed the Pump Demand

A jet pump can only deliver what the well can yield. If the well yields 5 GPM and the pump is sized for 12 GPM, the well draws down, the pump runs out of water, cavitates, and may run dry and burn out. The disciplined rule is to obtain a well yield test (a documented pumping test, often in GPM over time) before sizing the pump, to size the pump delivery to a fraction of the sustained yield (commonly no more than the well can sustain over the duty cycle), and to include low-well protection (a pressure or flow switch that drops the pump if the well draws down). A pump oversized to the well yield is a burnout waiting for the next dry season.

## Common Traps

### Using a Shallow-Well Pump for a Deep Pumping Level

A shallow-well single-pipe jet pump is installed where the pumping level is 35 feet, and the pump cavitates and delivers little water. The trap is designing to static level, not pumping level. The mechanism is that atmospheric pressure limits suction lift to roughly 25 feet practical. The false signal is that "the pump runs." The harm is no usable water and impeller damage from cavitation. The defense is to measure pumping level under draw and use a deep-well ejector or submersible beyond 25 feet.

### A Mismatched or Wrong-Depth Ejector

An ejector from a different pump model or depth range is installed, and the venturi produces poor lift. The trap is that the ejector looks generic but is depth- and GPM-specific. The mechanism is that nozzle and throat geometry must match the application. The false signal is that "water moves." The harm is low delivery, overheating, and wasted energy. The defense is to select the ejector from the manufacturer chart for the actual depth and required GPM.

### Exceeding the Practical Suction Lift and Cavitation

The system is designed near the 33-foot theoretical limit and cavitates at 27 feet due to friction and altitude. The trap is designing to theory, not practice. The mechanism is that vapor pressure and friction reduce usable lift below the theoretical. The false signal is that "it worked at first." The harm is cavitation noise, impeller erosion, and eventual failure. The defense is to treat 25 feet as the practical ceiling and measure pumping level under peak demand.

### Pressure Tank With Wrong or No Precharge

The tank's precharge is left at factory or never checked, and the pump short-cycles every few seconds. The trap is that the tank is present but delivers no drawdown. The mechanism is that precharge must be cut-in minus 2 to 4 psi. The false signal is that "there is a tank." The harm is rapid cycling that burns out the motor and pressure switch. The defense is to set precharge to cut-in minus 2 to 4 psi with the system empty and to check it annually.

### Pump Oversized to a Low-Yield Well

The pump is sized for 12 GPM on a well that yields 5 GPM, and the well draws down until the pump runs dry. The trap is that the pump can out-pump the well. The mechanism is that well yield limits sustainable delivery. The false signal is that "the pump is powerful." The harm is cavitation, dry running, and motor burnout. The defense is a documented yield test, sizing the pump to a fraction of sustained yield, and low-well protection.

## Self-Check

- Did I measure the pumping level (under draw), not just the static resting level, before choosing shallow-well versus deep-well configuration?
- Is the pumping level within the practical suction-lift limit (roughly 25 feet, less at altitude) for a shallow-well jet pump, or did I specify a deep-well ejector/submersible for deeper levels?
- Is the ejector (venturi) selected from the pump manufacturer's chart for the actual well depth and required delivery GPM, with the correct nozzle and throat?
- Is the pump delivery sized to a fraction of the well's sustained yield, based on a documented yield test, with low-well protection installed?
- Is the pressure tank sized to the pump GPM and desired drawdown, and is the precharge set to cut-in minus 2 to 4 psi with the system depressurized?
- Did I verify there is no cavitation (noise, vibration, low delivery) at peak demand, indicating suction lift or ejector mismatch?
- Is the pump protected from dry running (low-well or flow switch) so it cannot burn out if the well draws down?
- Did I confirm the drive-water ratio and two-pipe (or packer) arrangement match the deep-well ejector design?
- Is the pressure switch cut-in/cut-out set correctly and the tank precharge rechecked annually?
- Did I document the well yield, pumping level, ejector selection, tank size, and precharge so the system is verifiable and maintainable?
