---
name: boiler-controls-combustion-and-efficiency.md
description: Use when the agent is setting or diagnosing boiler operating controls (aquastat outdoor reset limit), performing combustion analysis with a flue gas analyzer, adjusting air-to-fuel ratio or burner rate, or evaluating boiler efficiency stack temperature and carbon monoxide production.
---

# Boiler Controls, Combustion, and Efficiency

A boiler that fires and heats the building is not necessarily a boiler that operates efficiently or cleanly — the difference between a well-tuned boiler and a poorly-tuned one is 10 to 30 percent in fuel cost and the difference between clean combustion and carbon monoxide production. The judgment problem is that boiler efficiency and safety are determined by the combustion setup (the air-to-fuel ratio, verified by flue gas analysis) and the control strategy (the water temperature setpoints, including outdoor reset), and a plumber who starts the boiler, confirms it fires, and leaves without combustion analysis or control setup is leaving significant efficiency and safety on the table. This skill covers the combustion, control, and efficiency decisions that determine whether a boiler operates cleanly, safely, and economically.

## Core Rules

### Perform Combustion Analysis With a Calibrated Flue Gas Analyzer at Every Setup

Combustion analysis measures the flue gas composition — oxygen (O2) or carbon dioxide (CO2), carbon monoxide (CO), stack temperature, and excess air — and the readings determine whether the burner is set up correctly. The target ranges depend on the fuel and burner type, but generally: CO should be below 100 ppm (ideally below 50) at setup, excess air should be in the 20 to 40 percent range (corresponding to roughly 6 to 9 percent O2 or 8 to 11 percent CO2 for natural gas), and the stack temperature should be high enough to avoid condensation (for non-condensing boilers) but not so high as to waste heat. The trap is starting the boiler, confirming the flame looks blue and steady, and declaring it set up, without combustion analysis — a flame that "looks right" can still produce excess CO or operate at low efficiency. The disciplined rule is to perform combustion analysis with a calibrated analyzer at every boiler setup, burner adjustment, or annual service, and to adjust the air-to-fuel ratio to the target readings.

### Set the Operating Controls for Efficiency and Comfort, Including Outdoor Reset

The boiler's operating controls — the high limit, the low limit (for tankless coil systems), the aquastat's differential, and the outdoor reset control — determine the water temperature at which the boiler operates, and these setpoints have a major effect on efficiency and comfort. Outdoor reset is particularly important: it lowers the boiler water temperature as the outdoor temperature rises (the building needs less heat on milder days), which reduces standby loss, improves condensing-boiler efficiency, and improves comfort (lower-temperature emitters cycle less and deliver more even heat). The trap is leaving the controls at factory defaults or at the previous boiler's settings, which may be a fixed 180°F high limit regardless of outdoor temperature, wasting energy on milder days. The disciplined rule is to set the controls per the building's heat emitters and the boiler's capabilities, to enable and configure outdoor reset where the system supports it, and to verify the actual operating temperatures against the setpoints.

### Understand the Difference Between Condensing and Non-Condensing Boiler Operation

Condensing boilers (modulating, with a secondary heat exchanger that extracts latent heat from the flue gas by condensing the water vapor) achieve efficiencies of 90 to 98 percent by operating with low return water temperatures (below 130°F), which allows condensation. Non-condensing boilers (cast-iron or copper-tube, conventional) operate at higher temperatures (160 to 200°F) and must not condense (the condensation would corrode the cast iron or copper). The control strategy is entirely different: a condensing boiler should be operated at the lowest water temperature that satisfies the heat load (to maximize condensation and efficiency), while a non-condensing boiler must be operated above the condensation threshold (to protect the heat exchanger). The trap is operating a condensing boiler at 180°F (no condensation, no efficiency benefit, wasted money on the condensing technology) or operating a non-condensing boiler at 130°F (condensation, corrosion, premature failure). The disciplined rule is to identify the boiler type and operate it at the appropriate temperature range.

### Adjust the Burner Rate and Air Shutter to the Analyzer Readings, Not by Flame Appearance

The burner rate (the firing rate, adjusted by the gas valve or oil burner nozzle) and the air shutter (the combustion air adjustment) are set to achieve the target combustion readings — not by the appearance of the flame. A flame that looks "nice and blue" can still be over-aired (wasting heat up the stack) or under-aired (producing CO). The adjustment procedure is to set the firing rate, measure the combustion, adjust the air shutter to bring the O2/CO2 and CO into the target range, and re-measure after each adjustment, iterating until the readings are stable and within range at both high and low fire (for modulating boilers). The trap is adjusting the air shutter "by the flame" without measuring, which produces a setup that looks right and reads wrong. The disciplined rule is to adjust only with the analyzer readings as feedback, at high and low fire.

### Verify Stack Temperature, Draft, and CO Before Leaving the Boiler in Service

The stack temperature, the draft (for atmospheric and Category I boilers), and the CO reading are the final verification of safe and efficient operation. Stack temperature should be high enough to avoid condensation (for non-condensing) but not excessively high (which indicates soot, poor heat transfer, or over-firing); draft should be steady and negative (for Category I) to prevent spillage; CO should be below the action level (typically below 100 ppm air-free). The trap is confirming the boiler fires and the building heats, without checking these readings, and leaving a boiler that spills flue gas, produces CO, or operates at low efficiency. The disciplined rule is to measure stack temperature, draft, and CO at every setup and service, to address any reading out of range, and to document the readings.

## Common Traps

### Declaring the Boiler Set Up Without Combustion Analysis

A plumber starts the boiler, confirms the flame looks blue and steady, and declares the setup complete, without performing combustion analysis. The trap is that the flame appearance does not reveal excess CO, over-airing, or under-firing, all of which the analyzer would catch. The mechanism is that combustion quality is measurable, not visible. The false signal is that "the flame looks good." The harm is CO production, low efficiency, or both. The defense is to perform combustion analysis with a calibrated analyzer at every setup and to adjust to the target readings.

### Leaving Controls at Factory Defaults or Previous Settings

A plumber installs a new boiler and leaves the aquastat and outdoor reset at factory defaults or at the old boiler's settings, and the boiler operates at a fixed 180°F regardless of outdoor temperature, wasting energy on milder days. The trap is that the boiler "works" at the default settings but operates inefficiently. The mechanism is that the controls determine the operating temperature, which determines efficiency. The false signal is that "the boiler heats." The harm is wasted fuel. The defense is to set the controls per the heat emitters and boiler type, to enable and configure outdoor reset where supported, and to verify the operating temperatures.

### Operating a Condensing Boiler at Non-Condensing Temperatures

A plumber installs a high-efficiency condensing boiler but operates it at 180°F supply, preventing condensation and eliminating the efficiency benefit the condensing technology was meant to provide. The trap is that the boiler "works" but operates at non-condensing efficiency, wasting the premium paid for condensing technology. The mechanism is that condensing efficiency requires low return temperature. The false signal is that "the boiler is high-efficiency." The harm is wasted fuel and an unrecouped equipment premium. The defense is to operate condensing boilers at the lowest water temperature that satisfies the load, with outdoor reset, to maximize condensation.

### Adjusting the Burner by Flame Appearance Without Measuring

A plumber adjusts the air shutter "to get a nice blue flame" without using the analyzer, and the combustion is over-aired (wasting heat) or under-aired (producing CO), despite the flame looking correct. The trap is that the flame appearance is a poor proxy for combustion quality. The mechanism is that the analyzer readings (O2, CO2, CO) are the true measure. The false signal is that "the flame looks good." The harm is inefficiency or CO production. The defense is to adjust only with the analyzer readings as feedback, iterating to the target ranges at high and low fire.

### Failing to Verify Stack Temperature, Draft, and CO Before Leaving

A plumber confirms the boiler fires and the building heats, without checking stack temperature, draft, or CO, and leaves a boiler that spills flue gas, produces CO, or operates at low efficiency. The trap is that the boiler "works" but operates unsafely or inefficiently. The mechanism is that these readings are the final safety and efficiency verification. The false signal is that "the boiler heats the building." The harm is CO spillage, inefficiency, or vent failure. The defense is to measure stack temperature, draft, and CO at every setup and service and to address any out-of-range reading.

## Self-Check

- Did I perform combustion analysis with a calibrated flue gas analyzer at every boiler setup, burner adjustment, or annual service?
- Are the O2 (or CO2), CO, and excess air readings within the target ranges for the fuel and burner type, at both high and low fire for modulating boilers?
- Is CO below the action level (typically below 100 ppm air-free), and did I address any elevated CO reading before leaving?
- Did I set the operating controls (high limit, low limit, differential, outdoor reset) per the heat emitters and boiler type, rather than leaving factory defaults or previous settings?
- For condensing boilers, am I operating at the lowest water temperature that satisfies the load (with outdoor reset) to maximize condensation and efficiency?
- For non-condensing boilers, am I operating above the condensation threshold to protect the heat exchanger?
- Did I adjust the burner rate and air shutter using the analyzer readings as feedback, iterating to the target ranges, rather than by flame appearance?
- Did I verify the stack temperature (high enough to avoid condensation for non-condensing, not excessively high), draft (steady and negative for Category I), and CO reading before leaving?
- Did I document the combustion readings, control setpoints, and stack conditions so the setup is verifiable and repeatable at the next service?
- For atmospheric or Category I boilers, did I verify draft under operating conditions (smoke or flame test at the draft hood) and confirm no spillage?
