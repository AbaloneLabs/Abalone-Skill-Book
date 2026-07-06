---
name: hydronic-radiant-floor-heating-design.md
description: Use when the agent is designing or installing a hydronic radiant floor heating system, calculating heat loss and required water temperature, selecting tube spacing and loop length, choosing the installation method (slab-on-grade, over-pour, dry below-deck), or sizing the circulator and manifold for even heat distribution across all loops.
---

# Hydronic Radiant Floor Heating Design

Radiant floor heating circulates warm water through tubing embedded in or below the floor, turning the floor into a low-temperature radiant panel that heats the space. The judgment problem is that radiant performance depends on a careful match between the building's heat loss, the floor's output capacity (which is limited by tube spacing, water temperature, and floor covering resistance), and the hydronic distribution (loop lengths, manifold balance, and circulator sizing). An installer who sizes by rule of thumb, who spaces tubes for convenience, or who ignores the floor covering's insulating effect will deliver a system that runs constantly and never satisfies the thermostat, or one with cold stripes and hot stripes where the loops are unbalanced. Worse, a poorly designed radiant system is expensive to fix because the tubing is embedded in concrete or under the finished floor. This skill covers heat loss, tube layout, loop length, water temperature, and the role limits that place complex radiant design with hydronic specialists.

## Core Rules

### Perform a Room-by-Room Heat Loss Before Sizing Anything

Radiant design begins with a room-by-room heat loss calculation (Manual J or an equivalent), because the floor must produce enough heat to offset each room's loss at the design outdoor temperature. The heat loss determines the required floor output (BTU per square foot of heated floor), which in turn determines the tube spacing, the water temperature, and the required flow. The trap is skipping the heat loss and installing a "standard" tube spacing (commonly 6 or 12 inches on center) throughout, so rooms with high heat loss (large windows, north exposure, over a crawl space) are under-heated while rooms with low loss are over-heated. The disciplined approach is to calculate the heat loss for every room, determine the required BTU per square foot, and adjust the tube spacing and water temperature to meet each room's demand. Where the required floor output exceeds what the floor can deliver (limited by maximum surface temperature, typically around 85°F for comfort and floor-material limits), supplemental heat is required — a radiant floor cannot heat a poorly insulated room by itself.

### Match Tube Spacing and Water Temperature to the Required Output

The floor's heat output is a function of the average water temperature, the tube spacing (closer spacing means more tube per square foot and higher output), the floor covering resistance (hardwood, carpet, and pad insulate and reduce output; tile and concrete conduct well), and the installation method (slab-embedded tubes conduct better than dry below-deck tubes with air gaps). The trap is using a fixed water temperature (say 120°F) and a fixed spacing regardless of the room's requirement and the floor covering, so a carpeted room with 12-inch spacing and 120°F water produces half the required output and the room is cold. The disciplined approach is to use the manufacturer's output tables (or design software) to find the combination of water temperature, spacing, and floor covering that delivers the required BTU per square foot, and to design each room's loop accordingly. Reduce spacing (e.g., from 12 to 6 inches on center) in high-loss rooms and at exterior walls (perimeter heat strips), and account for the floor covering's resistance — a thick carpet and pad may require closer spacing or higher water temperature, or may make radiant impractical.

### Limit Loop Length and Balance Flow at the Manifold

Each radiant loop has a maximum length (commonly 250 to 300 feet for 1/2-inch PEX, longer for larger tubing) beyond which the pressure drop becomes excessive and the flow inadequate, leading to a large temperature drop across the loop and a cold far end. Loops should be roughly equal in length so that balancing at the manifold is achievable; a loop that is much longer than the others will have low flow and under-perform even after balancing. The trap is running loops of widely varying length (a 150-foot loop and a 300-foot loop on the same manifold), so the short loop gets too much flow and the long loop too little, and the manifold balancing cannot correct the difference. The disciplined approach is to design loops of similar length (within about 10 percent), keep each loop under the maximum length for the tubing and the design temperature drop, and use the manifold's balancing valves (flow meters) to set each loop to its design flow. For rooms requiring more than one loop, design the loops to cover the area evenly.

### Size the Circulator for the Total Loop Pressure Drop and Flow

The circulator (pump) must move the design flow through every loop simultaneously, overcoming the pressure drop of the tubing, the manifold, the mixing valve (if present), the heat source, and the piping. The trap is sizing the circulator by "a typical radiant pump" without calculating the total flow (sum of all loop flows) and the pressure drop of the longest loop (which sets the pump head requirement), so the circulator is undersized and the remote loops are starved. The disciplined approach is to calculate the total system flow (sum of loop design flows), calculate the pressure drop of the longest/highest-resistance loop at its design flow (this sets the required pump head), and select a circulator whose curve delivers the required flow at the required head. Variable-speed circulators (with delta-T or delta-P control) are strongly preferred for radiant systems because they adapt to the changing resistance as zone valves open and close.

### Control the Water Temperature and Mixing for Each Zone

Radiant floors require much lower water temperature (typically 85 to 120°F) than a standard boiler output (140 to 180°F), so a mixing assembly (a mixing valve, injection loop, or dedicated radiant circulator with a mixing block) is required to temper the supply down to the radiant temperature. Different zones may require different temperatures (a tile bathroom vs. a carpeted living room, or a high-loss room vs. a low-loss room), so the mixing and the supply temperature must be designed per zone. The trap is supplying a single high temperature to all loops, overheating the low-temperature rooms and risking floor damage (hardwood can warp or crack at excessive floor temperature), or supplying a single low temperature that under-heats the high-loss rooms. The disciplined approach is to design the mixing to deliver the correct supply temperature to each zone, use outdoor reset (which lowers the supply temperature as outdoor temperature rises) for efficiency and comfort, and protect each floor covering from excessive temperature.

### Respect the Role Limits — Complex Radiant Design Needs Hydronic Specialists

Simple residential slab-on-grade radiant systems are within a licensed plumber or radiant installer's scope. Complex systems (multiple zones with different temperatures, high heat-loss buildings, combination systems with radiators and radiant, snow-melt integration) are designed by hydronic specialists or mechanical engineers who can perform the detailed heat loss, output, and hydraulic calculations. Confirm scope before taking on a complex radiant system, and escalate multi-zone and high-performance designs to a hydronic specialist.

## Common Traps

### Skipping the Heat Loss and Using Fixed Spacing

The installer skips the heat loss calculation and runs 12-inch-on-center tubing throughout, so high-loss rooms are under-heated and low-loss rooms are over-heated. The trap is that heat loss varies by room (windows, exposure, floor below), and a fixed spacing cannot meet the varying demand. The mechanism is that the floor output is fixed by the spacing and temperature, and the room's demand varies. The false signal is "the tubing is evenly spaced." The harm is cold rooms, a system that runs constantly, and an expensive retrofit. The defense is to perform a room-by-room heat loss and vary the spacing and temperature to meet each room's demand.

### Ignoring the Floor Covering Resistance

The installer designs a radiant floor for tile output and the owner later installs hardwood and a thick rug, halving the floor's heat output. The trap is that floor coverings insulate, and the design must account for the actual (or worst-case) covering. The mechanism is that the covering adds thermal resistance between the tube and the room, reducing output. The false signal is "the tubing is in the floor." The harm is an under-heated room after the covering is installed. The defense is to design for the actual floor covering resistance, reduce spacing or raise temperature for high-resistance coverings, and warn the owner against changing to a higher-resistance covering later.

### Loops of Widely Varying Length on One Manifold

The installer runs a 150-foot loop and a 300-foot loop on the same manifold, and the short loop gets most of the flow while the long loop is starved. The trap is that flow distributes by resistance, and a short loop has far less resistance, so balancing cannot fully correct the difference. The mechanism is that the long loop's higher pressure drop reduces its flow. The false signal is "the manifold has balancing valves." The harm is a cold far end on the long loop and an overheated short loop. The defense is to design loops of similar length (within about 10 percent) and under the maximum length.

### Undersized Circulator Starves the Remote Loops

The installer uses a "standard radiant circulator" without calculating the total flow and the longest loop's pressure drop, and the remote loops are starved. The trap is that the circulator must overcome the system's pressure drop at the total flow, and a default pump may be inadequate. The mechanism is that the pump curve does not deliver the required flow at the required head. The false signal is "the pump runs." The harm is low flow, large temperature drops, and cold far ends. The defense is to calculate the total flow and the longest loop's pressure drop and select the circulator from its pump curve.

## Self-Check

- Did I perform a room-by-room heat loss calculation (Manual J or equivalent) before sizing the tubing, and does each room's floor output meet its heat loss at the design outdoor temperature?
- Did I select the tube spacing and water temperature for each room based on the required output and the floor covering resistance, using manufacturer output tables or design software?
- Are all loops within the maximum length for the tubing and the design temperature drop, and are the loops on each manifold of similar length (within about 10 percent) so balancing is achievable?
- Did I calculate the total system flow (sum of loop flows) and the pressure drop of the longest loop, and does the selected circulator's curve deliver the required flow at the required head?
- Is a mixing assembly installed to temper the supply down to the radiant temperature, and is the supply temperature designed per zone where zones have different requirements?
- Is outdoor reset control used to lower the supply temperature as outdoor temperature rises, improving efficiency and comfort?
- Does the design protect each floor covering from excessive temperature (especially hardwood, which can warp or crack), and is the owner warned against changing to a higher-resistance covering?
- Did I confirm my scope covers this system, and did I escalate multi-zone, combination, or high-performance systems to a hydronic specialist or mechanical engineer?
- Are the heat loss, output calculations, loop layout, circulator sizing, mixing design, and control strategy documented for the owner and future service?
