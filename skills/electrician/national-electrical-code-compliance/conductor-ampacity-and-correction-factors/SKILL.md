---
name: conductor-ampacity-and-correction-factors.md
description: Use when the agent is selecting conductor sizes from Table 310.16, applying temperature correction factors, applying bundling derating for current-carrying conductors, sizing for continuous loads, or determining the usable ampacity of a conductor after all adjustments.
---

# Conductor Ampacity and Correction Factors

The ampacity printed in NEC Table 310.16 is not the ampacity a conductor can actually carry in a real installation — it is a baseline that must be corrected for ambient temperature and derated for the number of current-carrying conductors bundled together, and then limited by the temperature rating of the terminals it connects to. An electrician who reads the table, picks a conductor, and moves on has skipped three adjustments that can reduce the usable ampacity by 30 percent or more. The judgment problem is that each adjustment is invisible until applied, the reductions compound, and the consequence of skipping them is a conductor that overheats under load, degrades its insulation, and eventually fails or ignites. Worse, the breaker does not protect the conductor because the breaker is sized to the table ampacity, not the derated ampacity. This skill covers how to determine the true usable ampacity of a conductor by applying every applicable correction and derating factor in the correct sequence.

## Core Rules

### Start With the Correct Table and Temperature Column

NEC Table 310.16 provides allowable ampacities for insulated conductors rated up to 2000 volts, in three temperature columns: 60 degrees Celsius, 75 degrees Celsius, and 90 degrees Celsius. The column used depends on the conductor's insulation rating and the terminal temperature rating. The 90-degree column (for THHN, XHHW-2, and similar) gives the highest ampacity, but that ampacity is only usable if every termination in the circuit is rated for 90 degrees — and most equipment terminals are rated for 75 degrees or less. The trap is selecting from the 90-degree column because the wire is THHN, ignoring that the terminals limit the usable ampacity to the lower column. The defense is to identify the terminal temperature rating of every device the conductor connects to, and to use the column corresponding to the lowest-rated termination in the circuit, recognizing that the 90-degree column is typically used only as the starting point for derating calculations, not for final ampacity.

### Apply Temperature Correction Based on the Actual Ambient Temperature

Table 310.16 is based on an ambient temperature of 30 degrees Celsius (86 degrees Fahrenheit). When the actual ambient temperature exceeds 30 degrees Celsius, the ampacity must be reduced using the correction factors at the bottom of the table. When the ambient is below 30 degrees Celsius, the ampacity may be increased, subject to limitations. The correction factor is multiplied by the table ampacity to give the temperature-corrected ampacity. The trap is ignoring ambient temperature correction entirely, particularly for conductors in hot locations like attics, boiler rooms, or direct sun exposure, where the ambient can reach 40 to 50 degrees Celsius and the correction factor reduces ampacity significantly. The defense is to determine the actual ambient temperature for the conductor's location, apply the correction factor from the table for the conductor's temperature rating column, and recognize that hot locations may require upsizing the conductor.

### Apply Bundling Derating for More Than Three Current-Carrying Conductors

When more than three current-carrying conductors are installed in the same raceway, cable, or bundle for a distance exceeding 24 inches, the ampacity must be derated using Table 310.15(C)(1). The derating ranges from 80 percent for four to six conductors down to 35 percent for 41 or more conductors. The conductors counted are the current-carrying conductors — neutrals count as current-carrying in certain conditions (such as on a three-phase wye circuit with harmonic loads), and equipment grounding conductors do not count. The trap is not counting conductors in a bundle, or assuming that "separate cables" do not trigger derating when they are bundled together with ties or run through a common bored hole. The defense is to count every current-carrying conductor in each bundle or raceway, apply the derating factor when the count exceeds three, and remember that neutrals on certain circuits count as current-carrying.

### Combine Temperature Correction and Bundling Derating, and Compare to the Termination Limit

When both temperature correction and bundling derating apply, they compound: the temperature-corrected ampacity is multiplied by the bundling factor. However, the final derated ampacity is subject to a floor — it cannot be used at a value higher than the termination ampacity limit allows. The correct procedure is: start with the 90-degree column ampacity (the highest, used as the calculation base), apply temperature correction, apply bundling derating, and then compare the result to the ampacity from the termination-limit column (75 or 60 degrees). The usable ampacity is the lower of the derated value and the termination-limit value. The trap is applying only one adjustment, or applying both but then ignoring the termination limit, resulting in a conductor that is over its ampacity at the terminals even though the derating was performed. The defense is to perform the full sequence and take the lower of the derated ampacity and the termination-limited ampacity.

### Size Continuous Loads at 125 Percent of the Continuous Portion

A continuous load — defined as a load where the maximum current is expected to continue for three hours or more — must be sized at 125 percent of its continuous portion when selecting the overcurrent device and the branch-circuit conductors. The non-continuous portion is added at 100 percent. For example, a circuit serving 12 amps of continuous lighting load and 4 amps of non-continuous receptacle load requires conductors and a breaker sized for at least 19 amps (12 times 1.25 plus 4). The trap is treating all loads as non-continuous and sizing to the total connected load, which leaves no margin for the sustained heating that continuous loads impose on conductors and terminals. The defense is to identify which loads are continuous (lighting in many commercial spaces, certain process heaters, some receptacle loads in specific applications), apply the 125 percent factor to the continuous portion, and verify that the breaker and conductor ampacity meet or exceed the combined value.

### Verify the Final Ampacity Supports the Load and the Breaker

After all corrections and deratings, the final usable ampacity of the conductor must be at least equal to the load it serves and at least equal to the rating of the overcurrent device protecting it (with specific exceptions for motor and HVAC circuits). If the derated ampacity falls below the breaker rating, the conductor is under-protected — the breaker will not trip before the conductor is overloaded. The trap is sizing the conductor to the table ampacity and the breaker to match, then discovering after derating that the conductor's usable ampacity is below the breaker rating. The defense is to perform all derating first, confirm the final ampacity, then select the breaker to match the derated ampacity, and upsize the conductor if the derated ampacity is inadequate for the load or the required breaker.

## Common Traps

### Selecting From the 90-Degree Column When Terminals Are Rated 75 Degrees

An electrician sizes a 6 AWG THHN conductor using the 90-degree column of Table 310.16, which lists 75 amps, and installs a 70-amp breaker. But the conductor terminates at a panel with terminals rated for 75 degrees, where the 6 AWG ampacity is 65 amps. The trap is that the wire is rated for 90 degrees, so the 90-degree column felt correct, but the terminal cannot dissipate the heat that a 75-amp load would generate. The mechanism of harm is that the terminal overheats, the insulation at the connection degrades, and the connection can arc or ignite at the panel. The false signal is that the wire rating appeared to support the ampacity. The defense is to identify the terminal temperature rating and use the corresponding column for the termination ampacity, using the 90-degree column only as the base for derating calculations.

### Ignoring Temperature Correction in a Hot Location

An electrician runs feeders through a rooftop conduit in direct sun, where the ambient temperature inside the conduit reaches 45 degrees Celsius in summer. The conductors are sized from Table 310.16 at the 30-degree baseline, with no temperature correction applied. The trap is that the table assumes 30 degrees, and the actual ambient is 15 degrees higher, requiring a correction factor that reduces the ampacity. The mechanism of harm is that the conductors carry more current than their temperature-corrected ampacity allows, the insulation operates above its rated temperature, and the insulation degrades prematurely or fails. The false signal is that the table ampacity matched the load at the baseline temperature. The defense is to estimate or measure the actual ambient temperature in the conductor's location, apply the correction factor, and upsize the conductor if the corrected ampacity is inadequate.

### Forgetting to Count Conductors When Bundling Separate Cables

An electrician runs eight separate two-wire NM cables through a common bored hole in a floor joist, bundling them together for a run of several feet. The cables are not in a raceway, and the installer assumes bundling derating applies only to conductors in a single raceway or cable. But the rule applies to conductors bundled together for more than 24 inches regardless of whether they are in a raceway, and sixteen current-carrying conductors in the bundle require a derating factor of 50 percent. The trap is that the cables are physically separate, so they feel exempt, but the thermal effect of bundling is the same. The mechanism of harm is that the bundled cables cannot dissipate heat, the conductors overheat, and the ampacity is far below the breaker rating. The false signal is that each cable is individually within its ampacity. The defense is to count current-carrying conductors in every bundle exceeding 24 inches, whether in a raceway or in bundled cables, and apply the derating factor.

### Applying Only One Derating Factor When Both Apply

An electrician runs six current-carrying conductors in a raceway through a 40-degree-Celsius ambient. The bundling derating (80 percent for four to six conductors) is applied, but the temperature correction is forgotten. The conductor is sized to 80 percent of the table ampacity, but the actual required derating is 80 percent times the temperature correction factor, which is significantly lower. The trap is that one adjustment was performed, so the calculation felt complete, but the second adjustment was omitted. The mechanism of harm is that the conductor carries more current than its true derated ampacity, overheats, and degrades. The false signal is that a derating calculation was performed and the conductor appeared adequate. The defense is to apply temperature correction and bundling derating in sequence, multiplying the factors, and to verify the final ampacity against the load and breaker.

### Sizing a Continuous Load at 100 Percent Instead of 125 Percent

An electrician sizes a circuit for commercial lighting that runs continuously for the entire business day, treating it as a non-continuous load and sizing the conductor and breaker to the connected load. The trap is that the lighting is a continuous load by the code definition (three hours or more at maximum), and the conductor and breaker must be sized at 125 percent of the continuous portion. The mechanism of harm is that the conductor and breaker operate at their full rating for hours, the terminals heat to their limit and beyond, and the insulation and connection degrade over time. The false signal is that the load matched the breaker rating at 100 percent. The defense is to identify continuous loads by the three-hour definition, apply the 125 percent factor to the continuous portion, and verify that the breaker and conductor ampacity meet the combined continuous and non-continuous total.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- Did I select the conductor size from the temperature column that matches the lowest terminal rating in the circuit, using the 90-degree column only as the base for derating calculations?
- Did I determine the actual ambient temperature in the conductor's location and apply the temperature correction factor from Table 310.16?
- Did I count every current-carrying conductor in the same raceway or bundle (including neutrals where applicable), and apply the bundling derating from Table 310.15(C)(1) when the count exceeds three?
- Did I apply temperature correction and bundling derating in sequence, multiplying the factors, and then compare the result to the termination-limit ampacity to find the usable value?
- Did I identify any continuous loads (three hours or more at maximum current) and size the conductors and overcurrent device at 125 percent of the continuous portion plus 100 percent of the non-continuous portion?
- After all corrections and deratings, does the final usable ampacity meet or exceed both the load and the rating of the overcurrent device protecting the circuit?
- If the derated ampacity fell below the breaker rating, did I upsize the conductor or reduce the breaker rather than leaving the conductor under-protected?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
