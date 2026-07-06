---
name: solar-thermal-collector-and-loop-design-decisions.md
description: Use when the agent is selecting a solar water heating collector (flat-plate or evacuated tube), choosing a loop type (closed-loop glycol, drainback, or direct), setting collector orientation and tilt, sizing a solar heat exchanger, or sizing the expansion tank and relief for a high-temperature solar loop where freeze damage, boilover, and scald are the dominant risks.
---

# Solar Thermal Collector and Loop Design Decisions

Solar thermal domestic water heating is a high-temperature, intermittently driven system in which the collector can reach stagnation temperatures far above boiling and the loop can freeze solid on a cold, clear night. The judgment problem is that the collector type, loop architecture, and protection strategy must be matched to climate and load as a single system; an agent who picks a collector by price or copies a mild-climate closed-loop glycol layout into a hard-freeze region, or a drainback layout into a tall building where the collector cannot drain, produces freeze rupture, glycol degradation, boilover, or scalding delivery. This skill covers the collector, loop, orientation, heat exchanger, and high-temperature expansion decisions that a plumber tends to under-think because the system appears to "just heat water."

## Core Rules

### Match Collector Type to Climate, Load, and Operating Temperature

Flat-plate collectors are robust, cheaper per square foot, and efficient at low delivery temperatures (pool heating, domestic water to 120 to 140°F), but they lose heat rapidly to cold ambient air and stagnate at roughly 180 to 220°F. Evacuated-tube collectors cost more, are fragile (glass tubes, hail), but retain efficiency at high delivery temperatures (140 to 180°F) and in cold ambient air because the vacuum suppresses convective loss; they are the better choice for cold-climate domestic hot water and for high-temperature loads. The trap is choosing by first cost or by a single efficiency number, ignoring that flat-plate efficiency collapses when the delta between collector and ambient is large. The disciplined rule is to select the collector from a rated efficiency curve (optical efficiency and heat-loss coefficient, from SRCC or equivalent certification) at the intended delivery temperature and the local design ambient, and to size the array to the daily hot-water load in BTU, not to roof area.

### Choose the Loop Architecture for the Freeze and Overheat Climate

Three loop types dominate. Direct (open) loops circulate potable water through the collector and are acceptable only in essentially frost-free climates; they are cheap but offer no freeze margin and scale the collector in hard water. Closed-loop glycol (indirect) systems circulate a propylene-glycol/water mix through the collector and transfer heat through a double-wall heat exchanger to the potable side; they protect to the glycol freeze point (typically rated to minus 60°F at 50 percent by volume) but require stagnation management because glycol degrades above roughly 325°F. Drainback systems drain the collector and exposed piping into a reservoir whenever the pump stops, providing absolute freeze and overheat protection with plain water, but they require the collector to be above the tank, pitched piping (minimum 1/4 inch per foot) to drain, and a pump sized to lift to the collector on every start. The disciplined rule is to use drainback where geometry allows, closed-loop glycol where it does not, and direct only in frost-free climates.

### Set Orientation, Tilt, and Shading to the Load Profile, Not Due South at Latitude

Collector orientation and tilt should be optimized to when the load occurs and when the sun is available, not to a generic "south at latitude tilt." For year-round domestic hot water, face within 30 degrees of true south and tilt at latitude plus 0 to 15 degrees. For winter-dominated loads, tilt steeper (latitude plus 15 to 20 degrees) to capture low winter sun and to shed snow; for summer pool or vacation loads, tilt shallower (latitude minus 15 degrees). Shading must be checked across the full solar window, not at noon: a chimney, tree, or adjacent building that shades only in winter can cut winter yield by half. The trap is orienting to the roof's existing plane or to a presumed "south at 45 degrees" without modeling the actual solar path and shading. The disciplined rule is to perform a shade analysis (a Solar Pathfinder or equivalent) and to orient and tilt to the modeled yield for the load profile.

### Size the Heat Exchanger and Storage to the Collector Array and Draw Pattern

The heat exchanger in an indirect system must transfer the collector's peak output to the potable water without an excessive approach temperature; undersizing forces the collector to run hotter (lower efficiency, higher stagnation risk). Use a double-wall heat exchanger for potable separation, and size it so the approach temperature (collector-loop in versus potable out) is under 10 to 15°F at design flow. Storage tank volume should be roughly 1.5 to 2.5 gallons per square foot of collector for domestic hot water, sized so a sunny day's yield can be stored without boiling and a cloudy day still meets partial draw. The trap is pairing a large collector array with a small tank (the system boils and dumps heat through the relief valve) or a small array with a large tank (the water never reaches useful temperature). The disciplined rule is to size the exchanger to peak BTU and the tank to daily yield and draw, and to verify the tank has a usable stratification path so solar input feeds the bottom and draw comes from the top.

### Size the Expansion Tank and Relief for Stagnation, Not Just Operating Temperature

The solar loop reaches stagnation temperatures (no draw, full sun, pump off or power lost) that can exceed 300 to 400°F, far above any normal hydronic loop. The expansion tank must accept the full expansion of the loop fluid from cold fill to stagnation, and the loop must have a pressure relief valve rated for the fluid and temperature (commonly 75 to 87 psi, high-temperature rated, piped to a safe, visible discharge that can tolerate boiling glycol). For glycol loops, the relief discharge must be routed so a stagnation boilover cannot scald a person or spray hot glycol onto a roof or into an occupied space. The trap is using a standard hydronic expansion tank and a 30 psi boiler relief valve, both of which are overwhelmed on the first stagnation event. The disciplined rule is to size the expansion tank to stagnation volume, install a high-temperature solar-rated relief valve, and verify the discharge path is safe under boilover conditions.

## Common Traps

### Using a Direct Loop in a Climate With Any Freeze Risk

A plumber installs a direct (open) solar loop because it is cheap and simple, in a climate that occasionally drops below freezing. The trap is that a single hard freeze, or a brief power loss on a cold clear night, freezes the collector and ruptures it. The mechanism is that potable water in the collector has no freeze protection; the false signal is that "it rarely freezes here." The harm is a burst collector and water damage. The defense is to use an indirect (glycol or drainback) loop wherever freezing is possible, and to treat any freeze as a design condition.

### Undersizing the Heat Exchanger and Letting the Collector Stagnate

A plumber selects a small, cheap heat exchanger, and on a sunny day the collector cannot transfer its heat to the tank, so the collector-loop temperature climbs to stagnation, boiling the glycol and tripping the relief. The trap is that the system "works" on cloudy days but fails on the best solar days. The mechanism is that heat transfer is limited by exchanger area; the false signal is "the pump runs." The harm is glycol degradation, relief-valve discharge, and shortened fluid life. The defense is to size the exchanger to the collector's peak BTU with a small approach temperature.

### Sizing the Expansion Tank for Operating, Not Stagnation, Temperature

A plumber installs a standard small hydronic expansion tank sized for a 180°F loop, and on the first stagnation the loop fluid expands beyond the tank's acceptance volume and lifts the relief valve, discharging hot glycol. The trap is that the tank "looked right" for a heating loop. The mechanism is that solar stagnation temperatures are far higher than hydronic operating temperatures; the false signal is "an expansion tank is installed." The harm is repeated relief discharge, fluid loss, and scald risk. The defense is to size the tank to the full cold-to-stagnation expansion and to use a solar-rated relief valve.

### Pitching Drainback Piping Flat So It Cannot Drain

A plumber runs drainback loop piping level or with insufficient pitch, and when the pump stops the water does not drain back, leaving fluid in the collector to freeze or boil. The trap is that the piping "looks fine" but does not meet the minimum 1/4 inch per foot pitch to drain. The mechanism is that drainback protection depends entirely on gravity drainage; the false signal is "the pump runs and the system heats." The harm is freeze rupture or boilover on the first pump stop in adverse conditions. The defense is to pitch all collector and exposed piping continuously down to the reservoir with no traps or low spots.

### Ignoring Shading Outside the Noon Hour

A plumber orients the collector south and checks shading at noon, declaring the site "unshaded," but a tree or building shades the collector in the low winter morning and afternoon sun, cutting winter yield by half. The trap is that a noon check misses the seasonal solar path. The mechanism is that winter sun is low and casts long shadows; the false signal is "no shade at noon." The harm is a system that underperforms in the heating season when the load is highest. The defense is to perform a full solar-window shade analysis across all seasons before committing to the array location.

## Self-Check

- Did I select the collector type (flat-plate versus evacuated tube) from rated efficiency curves at the intended delivery temperature and design ambient, rather than by first cost or a single efficiency number?
- Did I choose a loop architecture (direct, closed-loop glycol, or drainback) matched to the freeze climate, and is a direct loop used only where freezing is essentially impossible?
- For a drainback system, is all collector and exposed piping pitched at least 1/4 inch per foot down to the reservoir with no traps or low spots, and is the pump sized to lift to the collector on every start?
- For a glycol loop, is the glycol concentration (typically 40 to 50 percent by volume) matched to the design low temperature, and is the heat-transfer fluid rated for stagnation?
- Did I orient within 30 degrees of true south and set tilt to the load profile (year-round, winter-dominated, or summer), with a full solar-window shade analysis confirming no significant seasonal shading?
- Is the heat exchanger double-wall for potable separation and sized so the approach temperature is under 10 to 15°F at design flow, avoiding forced collector stagnation?
- Is the storage tank sized to roughly 1.5 to 2.5 gallons per square foot of collector, with stratified solar input at the bottom and draw from the top?
- Is the solar-loop expansion tank sized for the full cold-to-stagnation expansion (not just operating temperature), and is a high-temperature solar-rated relief valve installed?
- Is the relief-valve discharge routed so a stagnation boilover cannot scald a person or spray hot fluid into an occupied space or onto a roof?
- Did I document the collector selection, loop architecture, orientation and tilt with shade analysis, exchanger sizing, tank sizing, and expansion/relief sizing so the design is verifiable?
