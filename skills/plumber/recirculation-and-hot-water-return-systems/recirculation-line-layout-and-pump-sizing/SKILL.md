---
name: recirculation-line-layout-and-pump-sizing.md
description: Use when the agent is laying out a hot water recirculation system (dedicated return versus under-skin crossover valve, home-run versus trunk-and-branch), sizing the recirculator pump by loop length and head, setting the temperature-maintenance GPM, or choosing an aquastat, timer, or demand (sensor, button, occupancy) control where long wait times, energy waste, and pump burnout are the risks.
---

# Recirculation Line Layout and Pump Sizing

Hot water recirculation exists to eliminate the wait for hot water at distant fixtures, but a poorly laid out or oversized recirculation loop trades one problem (the wait) for several others (continuous heat loss, energy waste, short-cycled pumps, and uneven delivery). The judgment problem is that the loop architecture, the pipe sizing, the pump curve, and the control strategy must be designed together: a dedicated return line that is too small starves the far fixture, an oversized pump erodes pipe and burns out, and a continuous-return loop left on 24 hours dumps the water heater's heat into the building envelope. Agents miss this because "hot water arrives fast" at the test, masking that the pump is running dry against a closed check valve, the near loop is hogging all the flow, or the loop is bleeding energy all night. This skill covers the layout, pump-sizing, and control decisions that produce a recirculation system that is fast, efficient, and durable.

## Core Rules

### Choose the Loop Architecture for the Building and the Retrofit Constraint

Two architectures dominate. A dedicated return loop runs a separate pipe from the farthest fixture back to the water heater, with a pump at or near the heater; it gives precise control and is the standard for new construction, but it requires a return pipe that retrofits rarely have. An under-skin (crossover) system uses the existing cold line as the return path through a thermostatic crossover valve installed under the farthest fixture; it avoids running a new return pipe but warms the cold line (a comfort and energy penalty) and depends on the valve closing at the right temperature. The disciplined rule is to use a dedicated return for new construction and major renovations, to use a crossover valve only where a return pipe cannot be installed, and to place the crossover valve at the farthest fixture so the whole branch is kept warm.

### Lay Out the Loop to Keep Every Branch Warm Without Hogging

Within a dedicated-return system, the loop topology matters. A home-run (manifold) layout with a return from each remote fixture is the most controllable but the most pipe-intensive; a trunk-and-branch loop with a single return from the end of the trunk is simpler but can leave short branches cold if not balanced. The disciplined rule is to design the return from the farthest point of the hot distribution (so the entire hot system is swept), to keep the loop as a single continuous path (sub-loops create balancing problems), and to install balancing valves (circuit setters) on every branch that returns to the main so that a short branch cannot hog the pump's flow while a long branch runs cold. Avoid dead-end runs longer than roughly 30 feet off the loop, which will not be kept warm.

### Size the Pump by Loop Length, Head, and Temperature-Maintenance Flow

A recirculation pump does not need to move much water — it needs to move just enough to keep the loop warm. Temperature-maintenance flow is small (commonly 0.5 to 1.5 GPM for residential, 1 to 3 GPM for light commercial), and the pump must overcome the head of the full loop (the longest supply plus the return, with fitting equivalents). The disciplined rule is to compute the developed length of the loop (supply plus return plus fittings), estimate the head at the target GPM from a pipe-friction table, and select a pump whose curve delivers the target GPM at that head, operating near its best efficiency point. For variable load, prefer a small ECM (electronically commutated) circulator with an integral check valve over a fixed-speed pump, which avoids oversizing and the erosion and noise that come with it.

### Select the Control Strategy for Wait-Time, Energy, and Durability

Four control strategies trade off wait-time, energy, and pump life. Continuous (always on) gives instant hot water but wastes the most energy and wears the pump fastest. An aquastat (a temperature switch on the return) runs the pump only when the return cools below setpoint, saving energy but still cycling the pump frequently. A timer restricts operation to occupancy hours (mornings, evenings), saving energy when no one is home. A demand system (a button, motion sensor, or occupancy sensor at the fixture) runs the pump only when hot water is actually wanted, giving the lowest energy use and longest pump life but a short (10 to 30 second) wait. The disciplined rule is to match the strategy to the occupancy pattern, to avoid continuous operation except where instant delivery is a hard requirement, and to prefer demand or timer-plus-aquastat for energy and pump life.

### Protect the Pump, the Water Heater, and the Potable System

A recirculation pump must have an integral or external check valve so hot water cannot thermosiphon backward into the cold supply, and it must be located where it cannot run dry (below the water heater's waterline, on the return near the heater). The water heater must have a means to handle the expanded volume of the now-larger hot system (a properly sized expansion tank on the cold inlet, since the recirculation loop increases the heated volume), and the loop must not bypass any tempering (mixing) valve on the domestic outlet. The disciplined rule is to install a check valve at the pump, to locate the pump on the return at the heater (never on the hot supply where it can run dry), to verify the expansion tank is sized for the loop volume, and to keep the tempering valve between the heater and all fixtures.

## Common Traps

### Oversizing the Pump "to Be Sure"

A plumber installs a large fixed-speed pump to guarantee fast hot water. The trap is that the oversized pump erodes pipe (especially at copper fittings), makes noise, and shortens its own life. The mechanism is that excessive flow velocity wears the pipe and the pump runs off its best efficiency point; the false signal is "hot water is instant." The harm is pinhole leaks, noise, and early pump failure. The defense is to compute the loop head at the temperature-maintenance GPM and select a pump that operates near its best efficiency point.

### Leaving a Continuous Loop On 24 Hours

A plumber commissions a dedicated return loop with the pump running continuously. The trap is that the loop dumps the water heater's heat into the building envelope around the clock. The mechanism is that the hot pipe loses heat continuously to the surrounding air; the false signal is "instant hot water." The harm is high energy bills and short pump life. The defense is to use an aquastat, timer, or demand control and to avoid continuous operation except where instant delivery is a hard requirement.

### Forgetting the Check Valve and Creating Thermosiphon

A plumber installs a recirculation pump with no check valve. The trap is that hot water thermosiphons backward into the cold supply when the pump stops. The mechanism is that hot water rises and cold water falls, creating a gravity flow loop; the false signal is "the pump is off." The harm is warm water at the cold tap and energy waste. The defense is to install an integral or external check valve at the pump.

### Hogging Flow in a Short Branch While a Long Branch Runs Cold

A plumber ties several branches into a return with no balancing valves. The trap is that the shortest, lowest-head branch takes most of the pump's flow and the long branch runs cold. The mechanism is that flow distributes by hydraulic resistance; the false signal is "the pump runs and the near fixture is hot." The harm is cold delivery at the far fixture. The defense is to install balancing (circuit setter) valves on every returning branch and to balance by measured flow.

### Letting the Pump Run Dry on the Hot Supply

A plumber mounts the recirculation pump on the hot supply outlet of the water heater. The trap is that if the heater is drained or the line is above the waterline, the pump runs dry and seizes. The mechanism is that a wet-rotor circulator needs water to lubricate and cool the bearings; the false signal is "the pump is at the heater." The harm is rapid pump failure. The defense is to locate the pump on the return line at the heater, below the waterline, so it cannot run dry.

## Self-Check

- Did I choose a dedicated return loop for new construction or major renovation, and reserve the under-skin crossover valve for retrofits where a return pipe cannot be installed?
- Is the return taken from the farthest point of the hot distribution so the entire hot system is swept, with no dead-end runs longer than roughly 30 feet off the loop?
- Did I compute the developed loop length (supply plus return plus fitting equivalents), estimate the head at the target GPM, and select a pump that operates near its best efficiency point (commonly 0.5 to 1.5 GPM residential, 1 to 3 GPM light commercial)?
- Did I avoid oversizing the pump, and did I prefer a small ECM circulator with an integral check valve over a large fixed-speed pump?
- Are balancing (circuit setter) valves installed on every returning branch so a short branch cannot hog flow while a long branch runs cold?
- Is the control strategy (continuous, aquastat, timer, or demand) matched to the occupancy pattern, and did I avoid continuous operation except where instant delivery is a hard requirement?
- Is an integral or external check valve installed at the pump to prevent thermosiphon of hot water into the cold supply?
- Is the pump located on the return line at the heater, below the waterline, so it cannot run dry?
- Is the thermal expansion tank on the cold inlet sized for the increased heated volume of the loop, and is the tempering (mixing) valve between the heater and all fixtures?
- Did I document the loop layout, pump selection (GPM and head), control strategy, balancing valve settings, and check-valve and expansion-tank provisions so the design is verifiable?
