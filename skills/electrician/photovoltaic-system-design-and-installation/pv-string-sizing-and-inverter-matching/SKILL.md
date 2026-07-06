---
name: pv-string-sizing-and-inverter-matching.md
description: Use when the agent is sizing PV strings, matching module count to inverter MPPT range, and calculating voltage and current limits for solar arrays, covering temperature-corrected Voc and Isc, inverter maximum DC voltage, string count optimization, NEC 690 calculations, and overcurrent protection.
---

# PV String Sizing and Inverter Matching

A photovoltaic array is a set of modules wired in strings to produce a DC voltage and current that an inverter can accept and convert. The judgment problem is that string sizing looks like simple arithmetic of adding module voltages until the inverter range is met, which ignores the temperature behavior that governs real-world voltage, the inverter's hard maximum voltage that cannot be exceeded, and the code calculations that protect the conductors and the equipment. When these are missed, strings either fail to start in cold weather because they exceed the inverter's maximum voltage, or they underproduce in heat because they fall below the inverter's MPPT window, and conductors and overcurrent devices are undersized relative to the real short-circuit current. This skill covers the temperature-corrected voltage and current calculations, the inverter matching logic, the string count optimization, and the NEC 690 calculations that make a PV array safe and productive.

## Core Rules

### Correct Open-Circuit Voltage for the Record Low Temperature

Module open-circuit voltage (Voc) rises as temperature falls, and the cold condition, not the standard test condition, sets the maximum voltage the inverter and conductors see. The sizing rule is to take the module's rated Voc at standard test conditions (STC), apply the manufacturer's temperature coefficient (a percentage per degree or millivolts per degree), and calculate the Voc at the site's record low ambient, often taken as the ASHRAE 99.6 percent heating design value or the local extreme. The resulting voltage, multiplied by the number of modules in series, must stay below the inverter's maximum DC input voltage with margin. A string sized only at STC will exceed the inverter's maximum on the first cold, bright morning and either shut the inverter down or damage it. Cold correction is the first and most consequential calculation in string sizing.

### Confirm the String Voltage Stays Inside the MPPT Window Across Conditions

The inverter's maximum power point tracking (MPPT) operates only within a stated voltage window, and the string voltage must stay inside that window across the full operating temperature range. Module voltage falls as temperature rises, so the hot-condition voltage, calculated at the record high cell temperature (often 70 degrees C cell or the ASHRAE 2 percent high ambient adjusted), sets the low end. If the hot-condition string voltage falls below the MPPT minimum, the inverter cannot track the maximum power point and the array underproduces exactly when irradiance is highest. The string count must satisfy both ends: enough modules to stay above the MPPT floor in heat, few enough to stay below the maximum voltage in cold. This dual constraint is the core of inverter matching.

### Size Conductors and Overcurrent Devices to the Corrected Short-Circuit Current

PV module short-circuit current (Isc) is rated at STC, but the array can produce more under enhanced irradiance from cloud edge effects, snow reflection, or high-altitude conditions, so NEC 690 requires conductors and overcurrent devices to be sized to 125 percent of the corrected Isc, with additional derating for continuous current and conduit fill. The Isc of a single string, or the summed Isc of parallel strings through a combiner, multiplied by 1.25 for irradiance and 1.25 for continuous load, sets the minimum conductor ampacity. Undersizing conductors to the STC Isc creates a fire and performance risk, because the conductors can carry more than their rating under real conditions. Every conductor and overcurrent device in the DC side is sized to the corrected and derated current, not the nameplate.

### Match the Inverter's DC Input Capacity to the Array's Power and the Climate

The inverter has a maximum DC input power and a maximum and minimum input current per MPPT, and the array's total power must be matched to the inverter's capacity with a deliberate DC-to-AC ratio. A ratio above 1.0 (array larger than inverter) is common and often economical, because the array rarely reaches nameplate in real conditions, and mild clipping at peak is acceptable for higher shoulder-season yield. A ratio that is too high, however, wastes energy in sustained clipping and stresses the inverter. The ratio is chosen by climate, orientation, and economics, and it is documented so that the design intent is clear. The inverter's current limits per MPPT must also be respected when paralleling strings.

### Optimize String Count for Shading, Orientation, and MPPT Allocation

Where multiple orientations, shading patterns, or module types exist, strings must be grouped so that each MPPT sees a homogeneous set of modules. Mixing orientations or shaded and unshaded modules on one string drags the whole string down to the weakest module, because series current is limited by the lowest performer. Each roof plane or shaded zone should be its own string or set of strings on its own MPPT, and string length may differ between MPPTs to accommodate different orientations. Optimizers or microinverters are an alternative where shading or orientation complexity makes string-level homogeneity impossible. String topology is a design decision driven by the roof, not an afterthought.

### Apply NEC 690 Calculations Consistently and Document Them

NEC Article 690 governs PV systems and requires specific calculations: voltage and current correction as above, conductor ampacity with the 1.25 irradiance and 1.25 continuous factors, overcurrent device sizing, disconnect means, and rapid shutdown. These calculations must be applied consistently across the DC and AC sides, documented in the design package, and verifiable by the inspector. A PV design that cannot show its temperature correction, its continuous current derating, and its overcurrent coordination will fail inspection and, more importantly, may be unsafe. The code calculations are not bureaucratic; they are the engineering that keeps the system within safe limits under real conditions.

### Allow Margin for Degradation, Soiling, and Future Expansion

Module power degrades over its life, soiling reduces yield, and sites often expand. The design should allow conductor and overcurrent margin for these factors, sizing the DC home run and combiner for a possible future string and choosing an inverter with headroom. Designing to the exact minimum leaves no room for correction and forces costly rework when a string is added or a module type is substituted. Margin is a small cost at installation and a large saving later.

## Common Traps

### String Sized at STC Voltage Exceeds Inverter Maximum in Cold

The designer adds modules until the STC string voltage reaches the inverter's maximum, ignoring the temperature coefficient. The mechanism is that Voc rises as temperature falls, so on the first cold bright morning the string voltage climbs past the inverter's maximum DC rating, and the inverter shuts down to protect itself or is damaged. The false signal is that the STC calculation looks safely under the limit. The harm is a system that fails exactly on the high-irradiance cold days that should produce the most energy, and possible inverter damage.

### Hot-Condition Voltage Falls Below the MPPT Floor

The designer uses too few modules per string, so the voltage at high cell temperature drops below the inverter's MPPT minimum. The mechanism is that module voltage falls with temperature, and a short string in heat cannot hold the MPPT window, so the inverter cannot track the peak power point. The false signal is that the string voltage is within range at STC. The harm is chronic underproduction during the hottest, highest-irradiance hours, exactly when yield should be greatest.

### Conductors Sized to Nameplate Isc Without Correction

The installer sizes the DC conductors to the module's nameplate Isc, ignoring the 1.25 irradiance and 1.25 continuous factors. The mechanism is that real irradiance and continuous operation push the current above the nameplate, so the conductors and overcurrent devices operate above their ampacity. The false signal is that the conductor ampacity exceeds the nameplate Isc. The harm is overheated conductors, nuisance trips, and a fire risk that an inspector will catch and that real conditions will expose.

### DC-to-AC Ratio So High That Clipping Wastes Sustained Energy

The designer oversizes the array far beyond the inverter to chase nameplate, ignoring the climate. The mechanism is that a high ratio causes the inverter to clip for sustained hours on clear days, exporting nothing above its AC rating while the array could produce more. The false signal is that the array nameplate is large. The harm is wasted energy, lost revenue, and inverter stress, with the loss hidden in production data that no one benchmarks.

### Mixed Orientations on One String Drag the Whole String Down

The installer wires a shaded east-facing module and an unshaded south-facing module into the same string. The mechanism is that series current is limited by the weakest module, so the shaded module caps the output of the entire string far below the sum of the modules' potential. The false signal is that the string produces power. The harm is a string that chronically underperforms because one mismatched module throttles the rest, a loss that is invisible without per-string monitoring.

### Overcurrent Device Sized to Match the Conductor, Not the Corrected Current

The installer chooses an overcurrent device that matches the conductor ampacity without checking the corrected Isc of the parallel strings. The mechanism is that the summed and corrected Isc of parallel strings may exceed the device rating, so the device operates near its rating or fails to protect. The false signal is that the device and conductor ratings match. The harm is nuisance tripping or, worse, a device that cannot clear a fault because it was selected by convenience rather than by calculation.

## Self-Check

- Did I calculate the temperature-corrected Voc at the site record low temperature using the manufacturer's coefficient, and confirm the string voltage stays below the inverter's maximum DC voltage with margin?
- Did I calculate the hot-condition string voltage at high cell temperature and confirm it stays inside the inverter's MPPT minimum, so the array does not fall out of tracking in heat?
- Did I size all DC conductors and overcurrent devices to 125 percent of the corrected Isc with the additional 1.25 continuous factor and conduit fill derating as required?
- Did I choose a DC-to-AC ratio appropriate to the climate and orientation, documenting the intent and confirming the inverter's per-MPPT current limits are respected?
- Did I group strings so each MPPT sees a homogeneous set of modules by orientation, shading, and type, using optimizers or microinverters where homogeneity is impossible?
- Did I apply and document the NEC 690 calculations consistently across the DC and AC sides, including voltage and current correction, conductor ampacity, overcurrent sizing, disconnects, and rapid shutdown?
- Did I allow conductor and overcurrent margin for degradation, soiling, and possible future expansion rather than designing to the exact minimum?
- Does the output stay within the agent's scope, deferring licensed judgment, stamped engineering, AHJ approval, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
