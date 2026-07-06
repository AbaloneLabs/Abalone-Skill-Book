---
name: correlator-use-and-pipe-material-signal-considerations.md
description: Use when the agent is using a leak noise correlator to locate a pipe leak, applying the time-delay and sound-velocity math, accounting for how pipe material and diameter affect acoustic velocity in PVC steel copper and ductile iron, placing sensors within distance limits, choosing radio versus cable sensors, judging accuracy versus pipe length and access, or verifying a correlation result before excavation to avoid a mislocated dig.
---

# Correlator Use and Pipe Material Signal Considerations

A leak noise correlator is the most precise tool for locating a pressurized pipe leak, because it uses two sensors on the pipe and the time delay of the leak sound arriving at each to compute the leak position mathematically, independent of the operator's ear and largely independent of ambient noise. The judgment problem is that the math depends on the acoustic velocity of sound in the pipe, and that velocity varies dramatically with pipe material and diameter: sound travels fast in metal (steel and copper around 4000 to 5000 m/s) and slow in plastic (PVC often 400 to 600 m/s), and entering the wrong velocity shifts the computed location by many feet, sending the excavation to the wrong spot. Agents misuse correlators by accepting the default velocity, by spacing sensors beyond the signal's reach, or by excavating on a correlation without verifying it, and the result is a mislocated dig, repeated excavations, and a leak that keeps running. This skill covers the correlator math, the material-and-diameter velocity table, sensor placement and distance limits, radio-versus-cable selection, accuracy limits, and the verification step that must precede any excavation, to prevent the wrong-velocity and mislocated-dig failures that define poor correlation practice.

## Core Rules

### Understand the Correlation Math and How Velocity Errors Propagate Into Location Errors

A correlator places two sensors on the pipe at known positions, measures the time delay between the leak sound arriving at each sensor, and computes the leak location using the acoustic velocity of the pipe: the leak is at a distance from the midpoint equal to half the velocity multiplied by the time delay. The math is exact given correct inputs, but the velocity is the dominant source of error, because a wrong velocity multiplies directly into the location. For example, on a 300-foot span with a true leak 100 feet from one sensor, a 10 percent velocity error shifts the computed location by roughly 15 feet, enough to miss the leak in a single excavation; on plastic pipe, where the velocity is uncertain and varies with wall thickness and temperature, the error can be far larger. The discipline is to treat the velocity as the critical input, to use the correct value for the specific pipe material and diameter (and to measure it where possible by tapping the pipe at a known point), and to understand that a correlation is only as good as its velocity assumption. A correlation run with a default velocity on an unknown pipe is a guess dressed in math.

### Use the Correct Acoustic Velocity for the Pipe Material and Diameter

Acoustic velocity in a water-filled pipe depends on the pipe material's elastic modulus and wall thickness, and the variation across common materials is large. Typical velocities are: steel and ductile iron around 4000 to 5000 m/s; copper around 4000 m/s; cast iron roughly 1200 to 1500 m/s (varies with age and condition); PVC commonly 400 to 600 m/s, strongly dependent on wall thickness (SDR) and diameter; and HDPE even lower and more variable. Within a material, larger diameter and thicker wall change the velocity, and the correlator's library or a published table (such as those in the device manual or from research literature) gives the value for the specific pipe. The discipline is to identify the pipe material and diameter before correlating (from records, a test pit, or an exposed fitting), to enter the matching velocity, and where the material is unknown or the velocity uncertain (especially plastic), to measure it on site by creating a known tap and letting the correlator back-calculate. Entering a metal velocity on a PVC line can shift the location by hundreds of feet on a long run.

### Place Sensors Within the Signal's Reach and at Accessible Fittings

A correlator needs two contact points on the pipe, and the signal must be strong enough at both sensors to correlate. The usable distance between sensors depends on the pipe material and condition: metal pipe carries the leak sound over long distances (commonly several hundred feet, sometimes over 1000 feet on a quiet, intact main), while plastic pipe attenuates the signal severely (often usable only over 100 to 300 feet, sometimes less). Sensors are placed at accessible fittings (valves, hydrants, meter setters, service curbs, air-release valves) where the pipe is exposed or can be contacted; where fittings are far apart on plastic, the signal may not reach both sensors and the correlation fails or produces a spurious result. The discipline is to identify the accessible contact points along the suspect span, confirm the span length is within the signal reach for the material, and if it is not, add an intermediate contact point (a hydrant, a tapped service, or a temporary exposed section) to shorten the span. A correlation over too long a plastic span produces a confident but wrong location.

### Choose Radio or Cable Sensors for the Site and Confirm Data Quality

Correlator sensors transmit their data to the processor either by radio (wireless, convenient for spread-out sites and long runs) or by cable (wired, immune to interference and useful where radio is blocked or where the run is short). Radio sensors are standard for most outdoor water-main work but can be disrupted by RF interference, by distance, or by structures between the sensor and the processor; cable sensors are used where radio is unreliable (inside large buildings, near heavy electrical equipment) or for short, precise spans. The discipline is to select the transmission mode for the site, to confirm during the run that both sensors are transmitting clean data (the correlator displays signal quality and coherence), and to reject or re-run a correlation where the signal quality is low or the coherence is poor. A correlation built on weak or noisy sensor data produces a confident-looking but unreliable location, and the operator must read the signal quality, not just the computed position.

### Verify the Correlation Result Before Excavation, Especially on Plastic or Long Runs

A correlation computes a location, but before excavating (particularly through pavement or finished surfaces), the result should be verified, because velocity uncertainty, signal attenuation, and reflections can all shift the computed spot. The verification is a combination of listening at the computed location with a ground microphone (to confirm a leak sound peaks there), checking the correlation's confidence and coherence indicators, and where the run is long or the pipe plastic, re-correlating with a different sensor pair or a measured velocity to see if the location is stable. Agreement between the correlation and a listening peak is strong evidence; a correlation with low coherence, or one that moves significantly when the velocity is adjusted within its plausible range, is not reliable enough to excavate. The discipline is to treat the correlation as a high-quality candidate that still needs confirmation, to listen at the computed spot, and to re-correlate with varied inputs on uncertain runs, before committing to a dig.

## Common Traps

### Accepting the Default Velocity on an Unknown or Plastic Pipe

The operator runs the correlator with the default velocity on a line whose material is unknown, and the computed location is dozens of feet off because the default is a metal value and the line is PVC. The trap is that the math looks authoritative. The mechanism is that velocity multiplies directly into the location, and the wrong velocity shifts the spot proportionally. The false signal is that "the correlator computed a location." The harm is a dry excavation. The defense is to identify the pipe material and diameter before correlating, enter the matching velocity, and measure it on site for plastic or uncertain lines.

### Spacing Sensors Beyond the Signal Reach on Plastic Pipe

On a long PVC run, the operator places sensors at two hydrants 800 feet apart, the plastic attenuates the signal so it barely reaches the far sensor, and the correlator produces a spurious location from noise. The trap is that a correlation is run over a span the signal cannot traverse. The mechanism is that plastic attenuates leak sound far more than metal, and beyond a few hundred feet the signal is lost. The false signal is that "the correlator returned a location." The harm is a confident but wrong dig. The defense is to keep sensor spans within the material's signal reach (commonly 100 to 300 feet for PVC), adding intermediate contact points where needed.

### Excavating on a Correlation Without Listening Verification

The correlator computes a location and the crew excavates immediately, without a ground-microphone check, and the dig is dry because a reflection or a velocity error shifted the spot. The trap is that the computed location is treated as certain. The mechanism is that correlation has confounders (velocity, attenuation, reflections) that a listening check would reveal. The false signal is that "the math located the leak." The harm is a dry excavation. The defense is to listen at the computed location with a ground microphone, confirm a leak-sound peak, and re-correlate with varied inputs on uncertain runs before digging.

### Trusting a Correlation With Low Coherence or Poor Signal Quality

The correlator displays a location but the coherence indicator is low and the sensor signals are noisy, and the operator excavates anyway on the computed spot. The trap is that the location number is read without the quality indicators. The mechanism is that a correlation from weak or noisy data is unreliable. The false signal is that "a location was computed." The harm is a dry or mislocated dig. The defense is to read the signal quality and coherence, reject or re-run low-quality correlations, and confirm clean data from both sensors before trusting the result.

### Ignoring the Effect of Diameter and Wall Thickness on Velocity

The operator knows the pipe is PVC but enters a generic PVC velocity without accounting for the diameter and SDR, and the actual velocity differs enough to shift the location on a long run. The trap is that "PVC" is treated as one value. The mechanism is that velocity within a material varies with wall thickness and diameter. The false signal is that "the material was entered correctly." The harm is a shifted location and a dry dig. The defense is to use the correlator's library or a published table for the specific material, diameter, and wall thickness, and to measure the velocity on site where the value is uncertain.

## Self-Check

- Did I identify the pipe material and diameter before correlating, and enter the acoustic velocity that matches the specific material, diameter, and wall thickness (for example, 400 to 600 m/s for PVC, 4000 to 5000 m/s for steel)?
- Where the material is unknown or the velocity uncertain (especially plastic), did I measure the velocity on site by tapping the pipe at a known point and letting the correlator back-calculate?
- Did I confirm the sensor span is within the signal reach for the material (commonly several hundred feet for metal, 100 to 300 feet for plastic), adding an intermediate contact point where the span is too long?
- Did I place sensors at accessible fittings (valves, hydrants, meters) with good pipe contact, and confirm both sensors are transmitting clean data?
- Did I select radio or cable transmission for the site, and confirm during the run that signal quality and coherence are high, rejecting or re-running low-quality correlations?
- Did I read the correlator's confidence and coherence indicators, not just the computed location, and treat a low-coherence result as unreliable?
- Did I verify the computed location with a ground-microphone listening check, confirming a leak-sound peak at the spot, before excavating?
- On long runs or plastic pipe, did I re-correlate with a different sensor pair or a varied (but plausible) velocity to confirm the location is stable?
- Did I account for reflections and changes of pipe material along the run, which can produce false correlation peaks, by confirming the pipe is uniform between the sensors?
- Did I document the pipe material and diameter, the velocity used, the sensor positions and span, the signal quality, and the listening confirmation, so the excavation location is defensible before digging?
