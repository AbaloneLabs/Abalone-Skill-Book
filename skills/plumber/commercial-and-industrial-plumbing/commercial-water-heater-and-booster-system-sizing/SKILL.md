---
name: commercial-water-heater-and-booster-system-sizing.md
description: Use when the agent is sizing a commercial water heating system (storage tank, tankless, semi-instantaneous) for a building, calculating peak hot water demand by occupancy type, designing recovery rate and storage volume tradeoffs, sizing a booster or circulating system, or selecting a water heater for a commercial kitchen or laundry.
---

# Commercial Water Heater and Booster System Sizing

Commercial water heating — for hotels, hospitals, apartments, restaurants, schools, laundries — must deliver a large volume of hot water during peak demand periods (morning showers in a hotel, meal times in a restaurant, laundry cycles), and the sizing depends on the occupancy type, the fixture count, the peak demand profile, and the tradeoff between recovery rate (how fast the heater can heat water) and storage volume (how much hot water is held in reserve). The judgment problem is that a commercial water heater sized by "gallons per fixture" or by a residential rule of thumb will either run out of hot water during peak demand (undersized storage or recovery) or waste energy in standby losses (oversized storage). This skill covers the sizing and system-design decisions for commercial water heating.

## Core Rules

### Calculate Peak Hot Water Demand by Occupancy Type

Commercial hot water demand is calculated by occupancy type, using established demand profiles (from ASHRAE, ASPE, or the manufacturer's sizing guides): a hotel's demand is driven by guest showers (peak in the morning), a restaurant's by dishwashing and hand-washing (peak at meal times), a hospital's by patient care and laundry (sustained high demand), a school's by lavatories and cafeteria (peak between periods and at meals). Each profile gives a peak hot water demand in gallons per hour (GPH) at a target temperature (typically 120°F for fixtures, 140°F or higher for dishwashers and laundry, with a mixing valve to deliver 120°F to fixtures). The trap is sizing by a generic "gallons per fixture" without the occupancy-specific demand profile, which undersizes for the actual peak. The disciplined rule is to calculate the peak hot water demand using the occupancy-specific demand profile, and to document the calculation.

### Balance Recovery Rate and Storage Volume

Commercial water heating is a balance between the recovery rate (the heater's input, in BTUH or kW, which determines how fast water is heated, expressed in GPH recovery at a temperature rise) and the storage volume (the tank capacity, in gallons, held in reserve). A high-recovery, low-storage system (a semi-instantaneous heater with a small tank) heats water fast but holds little reserve, suitable for steady, predictable demand. A low-recovery, high-storage system (a large tank with a smaller input) heats slowly but holds a large reserve, suitable for peaky demand (a large peak that exceeds the recovery, drawn from storage). The tradeoff is that storage has standby losses (energy wasted keeping the stored water hot), while high recovery requires a larger fuel or electrical input. The trap is sizing storage and recovery independently or defaulting to a large tank without considering the recovery, which either runs out (low recovery, peak exceeds it) or wastes energy (large storage, high standby losses). The disciplined rule is to balance the recovery rate and the storage volume against the demand profile, to minimize both running-out and standby losses, and to document the tradeoff.

### Provide 140°F (or Higher) Sanitizing Hot Water With a Mixing Valve

Commercial applications, especially food service and healthcare, often require 140°F or higher hot water for sanitizing (the final rinse of a commercial dishwasher, laundry sanitizing), but fixture water must be delivered at a safe temperature (120°F maximum, to prevent scalding). The solution is to generate and store hot water at 140°F or higher (which also prevents Legionella growth in the storage tank), and to use a master mixing valve (thermostatic) to deliver 120°F to the fixtures, with a separate 140°F or higher line to the sanitizing fixtures (the dishwasher, the laundry). The trap is generating at 120°F (no sanitizing water, Legionella risk in storage) or generating at 140°F and delivering 140°F to all fixtures (scalding risk). The disciplined rule is to generate and store at 140°F or higher, to use a master mixing valve to deliver 120°F to fixtures, and to provide a dedicated high-temperature line to sanitizing fixtures, with the mixing valve sized and verified.

### Design the Circulating System for Temperature Maintenance

In a large commercial building, hot water must circulate from the heater (or the storage tank) through the distribution piping and back, to maintain temperature at the fixtures (eliminating the "wait for hot water" and the waste of running water until it is hot) and to prevent Legionella growth in stagnant piping. The circulating system consists of a circulating pump, a return line from the far end of the distribution back to the heater, and balancing valves to ensure even flow through all branches. The pump is sized for the heat loss of the piping (the flow needed to keep the return water within a target temperature of the supply), and the system is balanced so that every branch has adequate return flow. The trap is omitting the circulating system (long waits, water waste, Legionella risk) or sizing the pump without balancing (some branches too cool, others over-circulated). The disciplined rule is to design a circulating system for temperature maintenance, to size the pump for the piping heat loss, and to balance the return flow across all branches.

### Provide Redundancy and Consider High-Efficiency Options

Commercial water heating is a critical function (a hotel or hospital without hot water is a crisis), and the system should provide redundancy (multiple heaters, so that one can fail without total loss of service) and should consider high-efficiency options (condensing gas heaters, heat pump water heaters, solar preheat) for energy and operating cost. The redundancy can be N+1 (one spare heater beyond the required capacity) or a split system (two heaters, each sized for a fraction of the peak, such as 70 percent each). The high-efficiency options reduce operating cost but may have higher first cost and specific application requirements (condensing heaters need low inlet water temperature to condense; heat pumps need a warm space and recover slowly; solar needs a backup). The trap is providing a single heater with no redundancy (a failure stops all hot water) or specifying high-efficiency without considering the application. The disciplined rule is to provide redundancy (multiple heaters, N+1 or split), to evaluate high-efficiency options against the application and the operating cost, and to document the rationale.

## Common Traps

### Sizing by "Gallons per Fixture" Without the Occupancy Demand Profile

A plumber sizes a commercial water heater by "gallons per fixture" or a residential rule of thumb, without the occupancy-specific demand profile, and the heater runs out of hot water during the actual peak. The trap is that the rule of thumb undersizes for the real peak. The mechanism is that demand is occupancy-specific. The false signal is that "the heater is sized." The harm is running out of hot water during peak demand. The defense is to calculate the peak demand using the occupancy-specific profile.

### Defaulting to a Large Tank Without Considering Recovery

A plumber defaults to a large storage tank without considering the recovery rate, and the tank runs out during a sustained peak that exceeds the recovery, or wastes energy in standby losses if oversized. The trap is that the large tank seems safe but the recovery is the bottleneck or the storage is wasteful. The mechanism is that storage and recovery must be balanced. The false signal is that "the tank is large." The harm is running out or energy waste. The defense is to balance the recovery rate and the storage volume against the demand profile.

### Generating at 120°F (No Sanitizing Water, Legionella Risk) or Delivering 140°F to All Fixtures

A plumber generates and stores hot water at 120°F, providing no sanitizing water and allowing Legionella risk in the storage tank, or generates at 140°F and delivers 140°F to all fixtures, creating a scalding risk. The trap is that the temperature strategy is wrong at one end or the other. The mechanism is that generation, storage, and delivery temperatures must be coordinated. The false signal is that "the water is hot." The harm is no sanitizing, Legionella, or scalding. The defense is to generate and store at 140°F or higher, use a master mixing valve to deliver 120°F to fixtures, and provide a dedicated high-temperature line to sanitizing fixtures.

### Omitting the Circulating System or Sizing the Pump Without Balancing

A plumber omits the hot water circulating system in a large building (long waits, water waste, Legionella risk) or sizes the pump without balancing the return flow (some branches too cool). The trap is that the system "has hot water" but the distribution is inadequate. The mechanism is that circulation maintains temperature and prevents stagnation. The false signal is that "the heater works." The harm is fixture delays, water waste, and Legionella risk. The defense is to design a circulating system, size the pump for the piping heat loss, and balance the return flow across all branches.

### Providing a Single Heater With No Redundancy

A plumber provides a single commercial water heater with no redundancy, and a heater failure stops all hot water in the building, a crisis for a hotel or hospital. The trap is that the single heater "meets the demand" but has no backup. The mechanism is that commercial hot water is critical. The false signal is that "the heater is sized." The harm is total loss of hot water on a failure. The defense is to provide redundancy (multiple heaters, N+1 or split) and to evaluate high-efficiency options against the application.

## Self-Check

- Did I calculate the peak hot water demand (GPH at the target temperature) using the occupancy-specific demand profile (hotel, restaurant, hospital, school)?
- Did I balance the recovery rate (BTUH or kW input, GPH recovery) and the storage volume (gallons) against the demand profile, to minimize both running-out and standby losses?
- Did I generate and store hot water at 140°F or higher (for sanitizing and Legionella prevention), and use a master mixing valve to deliver 120°F to fixtures?
- Did I provide a dedicated high-temperature line (140°F or higher) to sanitizing fixtures (dishwasher, laundry), separate from the mixed 120°F fixture supply?
- Did I design a hot water circulating system for temperature maintenance, with the pump sized for the piping heat loss and the return flow balanced across all branches?
- Did I provide redundancy (multiple heaters, N+1 or split system) so that a heater failure does not stop all hot water?
- Did I evaluate high-efficiency options (condensing gas, heat pump, solar preheat) against the application, the operating cost, and the first cost?
- Did I verify that the mixing valve is sized for the peak fixture flow and that it delivers a stable 120°F (or the target) across the flow range?
- Did I confirm the heater fuel or electrical input is available and adequate (gas pressure, electrical capacity, venting) for the selected heater?
- Did I document the demand calculation, the recovery-storage balance, the temperature strategy, the circulating system design, and the redundancy, so the system is verifiable?
