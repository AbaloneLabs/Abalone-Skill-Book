---
name: solar-and-heat-pump-freeze-and-overheat-protection.md
description: Use when the agent is designing or troubleshooting freeze protection (glycol concentration, drainback, or recirculation), stagnation and boilover protection, solar-loop expansion tank and relief sizing, high-limit controller logic, or overheat dump zones for solar thermal and heat pump water heating systems where collector rupture, glycol degradation, and scald are the dominant failure modes.
---

# Solar and Heat Pump Freeze and Overheat Protection

Solar thermal and heat-pump water heating systems live at two extremes: the collector and exposed piping can freeze solid on a cold night, and the same collector can boil its fluid on a hot day when there is no draw. The judgment problem is that freeze and overheat protection are not features you add at the end — they are the architecture of the system, and a protection scheme that works in mild conditions can fail catastrophically in the exact edge case (a power loss, a controller failure, a prolonged vacation) it was meant to survive. Agents miss this because the system runs fine on an ordinary day, masking that the glycol is silently degrading, the drainback piping cannot drain, or the high limit will never trip until the tank is already scalding. This skill covers the freeze, stagnation, and overheat protection decisions that prevent collector rupture, fluid destruction, and scalding delivery.

## Core Rules

### Match the Freeze-Protection Strategy to the Climate and to Failure Mode

Three strategies dominate, and each fails differently. Closed-loop glycol protects by chemistry: a propylene-glycol/water mix (40 to 50 percent by volume, freeze-protected to roughly minus 60°F at 50 percent) stays fluid in the collector, but it degrades above roughly 325°F and must be tested and replaced on interval. Drainback protects by geometry: the collector and exposed piping drain to a reservoir whenever the pump stops, leaving no fluid to freeze, but it requires continuous pitch (minimum 1/4 inch per foot) and a pump sized to lift on every start. Recirculation protects by warm water: the controller starts the pump to circulate warm tank water through the collector when the collector sensor approaches freezing, but it wastes stored heat and fails if power is lost or the tank is cool. The disciplined rule is to choose the strategy whose failure mode is acceptable in the local climate: drainback where geometry allows (fails safe on power loss), glycol where it does not (test and replace the fluid), and recirculation only as a secondary or mild-climate strategy.

### Manage Stagnation and Boilover Before the Relief Valve Does

Every solar collector stagnates when the sun shines and there is no draw (a full hot tank, a power loss, a vacation). At stagnation the collector reaches 300 to 400°F, and the loop fluid either boils (water/glycol) or degrades (glycol). The system must be designed so stagnation is survivable: the expansion tank must accept the full cold-to-stagnation expansion, the relief valve must be solar-rated (high-temperature, commonly 75 to 87 psi) and discharge to a safe path, and the controller must shed heat (turn off the pump, open a dump zone) before the loop reaches the fluid's limit. The trap is assuming the relief valve is the overheat strategy — it is the last defense, not the primary control. The disciplined rule is to design a controller-driven overheat strategy (stop pumping, activate a heat-dump zone or night radiator) and to treat the relief valve as the final, non-routine protection.

### Size the Expansion Tank and Relief for High-Temperature Solar Service

A solar loop is not a hydronic loop. The expansion tank must be sized for the full expansion from cold fill to stagnation (not to 180°F operating), and the relief valve must be rated for solar temperatures and pressures (a standard 30 psi, 210°F boiler relief will be overwhelmed and may not reseat after a stagnation event). The relief discharge must be routed where boiling glycol or steam cannot scald a person, cannot spray onto a roof or into an attic, and cannot pool where it could be mistaken for a normal drip. The trap is using a standard hydronic expansion tank and a 30 psi boiler relief, both of which are undersized and mis-rated for solar service. The disciplined rule is to size the tank to stagnation volume, to install a solar-rated relief, and to verify the discharge path is safe under boilover.

### Set High-Limit Controller Logic to Protect the Tank and the Fluid

The differential controller that runs the solar pump should also enforce a high-limit (tank-full) function: when the tank reaches its high limit (commonly 160 to 180°F for a domestic tank, lower for scald protection), the controller must stop the pump so the collector can stagnate rather than continue heating an already-hot tank. The high-limit setpoint must be chosen against scald risk (water above 120°F is a scald hazard at the tap; a mixing valve is required above that setpoint) and against the fluid limit (glycol degrades above roughly 325°F, so the collector must be allowed to stagnate, not forced to run hotter). The trap is setting the high limit to "as hot as possible" to maximize solar yield, creating a scalding tank and driving the glycol toward degradation. The disciplined rule is to set the high limit to a safe storage temperature, install a mixing (tempering) valve on the domestic outlet, and let the controller stagnate the collector when the tank is full.

### Provide an Overheat Dump or Night-Cool Strategy for High-Yield Systems

In systems with a large collector array relative to load (vacation homes, summer-dominated loads, undersized tanks), a sunny day can deliver more heat than the tank can store, forcing continuous stagnation. A heat-dump zone — a finned-tube radiator, a buried loop, or a pool/spa heat exchanger — gives the controller somewhere to reject excess heat so the collector does not stagnate and the glycol does not degrade. A night-radiation or night-flush strategy runs the pump after sundown to reject heat to the cool night sky. The trap is designing a high-yield system with no dump strategy and relying on the relief valve to manage every sunny day. The disciplined rule is to add a dump zone or night-cool strategy whenever the array can exceed the tank's storage capacity, and to verify the controller logic activates it before the high limit.

## Common Traps

### Treating the Relief Valve as the Overheat Strategy

A plumber installs a solar loop with no controller-driven overheat logic, assuming the relief valve will handle stagnation. The trap is that the relief valve is a last-resort device, not a routine control, and each stagnation discharge dumps hot glycol and degrades the fluid. The mechanism is that the relief opens at its set pressure and discharges until pressure falls; the false signal is "the relief is there, so we're protected." The harm is repeated fluid loss, glycol degradation, and scald risk at the discharge. The defense is to design controller-driven overheat logic (stop pump, dump zone) and treat the relief as non-routine.

### Using Standard Hydronic Expansion Tank and Boiler Relief

A plumber installs a 30 psi boiler relief and a small hydronic expansion tank on a solar loop. The trap is that these are rated for hydronic temperatures (210°F) and pressures, not solar stagnation (300 to 400°F). The mechanism is that stagnation expansion exceeds the tank's acceptance and the relief's rating; the false signal is "an expansion tank and relief are installed." The harm is a relief that weeps or fails to reseat and a tank that cannot absorb stagnation expansion. The defense is to size the tank to stagnation and install a solar-rated high-temperature relief.

### Setting the Tank High Limit Too High for Yield

A plumber sets the solar high limit to 180 to 200°F to "get more free heat," creating a scalding tank and pushing the glycol toward degradation. The trap is that hotter storage maximizes solar fraction but creates scald and stagnation risk. The mechanism is that tank temperature drives both tap scald potential and collector-loop operating temperature; the false signal is "hotter is more efficient." The harm is scald injuries and accelerated glycol breakdown. The defense is to set the high limit to a safe storage temperature and install a mixing valve on the domestic outlet.

### Letting Glycol Sit Until It Is Already Degraded

A plumber fills a glycol loop and never tests it, assuming the fluid lasts indefinitely. The trap is that glycol degrades with heat and time, turning acidic and corroding the loop. The mechanism is that stagnation events and oxidation break the glycol into organic acids; the false signal is "the system still heats water." The harm is loop corrosion, failed fluid, and costly replacement. The defense is to test the glycol (pH, freeze point, reserve alkalinity) on a set interval (commonly every 3 to 5 years) and to replace it when it falls out of spec.

### Relying on Recirculation Freeze Protection Where Power Loss Is Likely

A plumber uses recirculation freeze protection (warm tank water pumped through the collector at freezing) in an area prone to outages. The trap is that recirculation needs power and a warm tank to work, and a cold-night outage freezes the collector. The mechanism is that recirculation is an active strategy that fails on power loss; the false signal is "the controller handles freeze." The harm is freeze rupture on the first cold outage. The defense is to prefer drainback or glycol (which fail safe) and to use recirculation only as a secondary or mild-climate strategy.

## Self-Check

- Did I choose a freeze-protection strategy (glycol, drainback, or recirculation) whose failure mode is acceptable in the local climate, preferring drainback where geometry allows?
- For a glycol loop, is the concentration (typically 40 to 50 percent by volume) matched to the design low temperature, and is there a test-and-replace interval (commonly every 3 to 5 years)?
- For a drainback system, is all collector and exposed piping pitched at least 1/4 inch per foot to the reservoir with no traps, and is the pump sized to lift on every start?
- Is there a controller-driven overheat strategy (stop pump, dump zone, or night-cool) so stagnation is managed before the relief valve, rather than relying on the relief as the primary control?
- Is the expansion tank sized for the full cold-to-stagnation expansion (not just operating temperature), and is a solar-rated high-temperature relief valve installed?
- Is the relief-valve discharge routed so a boilover cannot scald a person, spray hot glycol onto a roof or into an attic, or pool where it looks like a normal drip?
- Is the tank high-limit setpoint chosen for safe storage (with a mixing/tempering valve on the domestic outlet), rather than maximized for yield?
- For a high-yield system (large array, small load, vacation use), is a heat-dump zone or night-radiation strategy provided so the collector does not stagnate every sunny day?
- Did I verify the differential controller's high-limit logic actually stops the pump when the tank is full, rather than continuing to heat an already-hot tank?
- Did I document the freeze strategy, overheat strategy, expansion and relief sizing, high-limit setpoint, and glycol test interval so the protection design is verifiable?
