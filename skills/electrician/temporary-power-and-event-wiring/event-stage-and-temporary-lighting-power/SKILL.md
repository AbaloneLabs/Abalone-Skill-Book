---
name: event-stage-and-temporary-lighting-power.md
description: Use when the agent is powering event stages, temporary lighting trusses, and audio systems with portable distribution, balancing 3-phase distro loads across phases, managing dimmer racks, and protecting cable runs across public walkways using entertainment industry power practices.
---

# Event Stage and Temporary Lighting Power

Entertainment power is a discipline of its own. A stage distro carries large 3-phase feeders into dimmer racks and audio racks, splits them into dozens of branch circuits, and sends those branches up trusses and across floors to moving lights, LED fixtures, line-array amplifiers, and backline gear — all of it assembled in hours, run for a show, and torn down the same night. The judgment problem is that the loads are highly nonlinear, the phases must be balanced to avoid neutral overload and dimmer flicker, and the cable runs cross public spaces where a single damaged run can injure an audience member. Agents miss the issues because the gear looks rugged and the show "worked at sound check," while the real risks — neutral overload from harmonics, phase imbalance, unsecured truss cable, dimmer rack heat — only appear under full show load or when a cable is kicked.

## Core Rules

### Balance Single-Phase and Nonlinear Loads Across All Three Phases

A 3-phase distro feeds phase-to-neutral branch circuits, and the loads on each phase must be balanced so that the neutral carries only the unbalanced current. With resistive loads, a balanced system returns little neutral current. With nonlinear loads — dimmer racks, switch-mode audio amplifier power supplies, LED drivers — the neutral carries harmonic currents that do not cancel, and on a 3-phase 4-wire system the neutral can carry more current than the phase conductors. The defense is to distribute phase-to-neutral loads as evenly as possible, to measure phase currents under show load, and to specify a neutral sized at least as large as the phases, because harmonics can make the neutral the most heavily loaded conductor.

### Size the Distro and Feeder for the Real Show Load, Not the Nameplate Sum

Stage loads are deceptive. Moving lights draw a surge when their lamps strike and their motors home; audio amplifiers draw enormous short-term current on bass transients that the feeder must supply without sagging. Sizing the distro to the nameplate sum of all fixtures over-provisions and wastes feeder capacity, but sizing it to a naive diversity factor can sag the bus on the first big bass note. The defense is to use the entertainment industry's established load factors for lighting and audio, to allow for the inrush of lamp strikes, and to verify under full show load that no phase is at its limit and the bus voltage holds during transients.

### Manage Dimmer Racks for Heat, Neutral Loading, and Phase Control

Theatrical dimmer racks switch phase-controlled SCRs that generate harmonics and heat. A rack must be ventilated, its neutral must be sized for harmonic loading, and its phase feeds must be balanced so no dimmer is starved. Dimmer racks also generate conducted emissions that can interfere with audio, so power and audio distribution should be planned together. The defense is to position racks with adequate clearance and airflow, to use a full-capacity neutral, to balance dimmer channels across phases, and to keep audio on separately derived or technically grounded feeds to avoid ground loops and dimmer noise.

### Protect Cable Runs Across Walkways, Doorways, and Audience Areas

Event cable crosses spaces the public uses, and every run is both a trip hazard and a damage target. Overhead rigging is preferred for truss feeders and data lines; where cable must cross a floor, it must be in a listed cable ramp secured to the surface, routed along walls where possible, and never run through a doorway where it will be crushed. The defense is to plan cable paths before running them, to keep power and signal separated where required, to use ramps rated for the expected traffic including forklifts in load-in, and to tape and secure every floor run so it cannot move and cannot lift to create a trip edge.

### Use Listed Entertainment Cable and Connectors Within Their Ratings

Entertainment cable — typically extra-hard usage portable cord such as SOOW or entertainment-grade cable — and the connectors used on it, including stage pin, NEMA, and cam-lock types, are listed for specific voltage and current ratings. Overrating a connector, using a splitter that exceeds the branch rating, or daisy-chaining fixtures beyond the connector's capacity overheats the connection. The defense is to verify every connector and cable is within its rating, to avoid improvised splitters, and to derate for the high ambient temperatures found inside trusses and near hot lights, because a connector at its nameplate in a hot truss is already near its limit.

### Provide a Separately Derived System and Technical Grounding for Sensitive Loads

A stage distro fed from a generator or an isolation transformer creates a separately derived system with its own neutral-to-ground bond, which is the preferred topology for entertainment audio to control ground loops and noise. Lighting and audio may be fed from separate sources or separate transformers to keep dimmer noise out of the audio. The defense is to establish a single neutral-ground bond at the source, to run isolated equipment grounding conductors where the system design calls for technical grounding, and to coordinate power and audio grounding so that the show is quiet as well as safe.

## Common Traps

### Balancing Only Resistive Loads and Ignoring Harmonic Neutral Current

The installer balances the phase-to-neutral loads by wattage and assumes the neutral is lightly loaded. The false signal is that the phase currents are equal. The mechanism of failure is that dimmer and switch-mode loads produce triplen harmonic currents that add in the neutral rather than cancel, so the neutral can carry more than the phase current even when the phases are balanced. The harm is an overloaded neutral that overheats, with no breaker to trip because the neutral has no overcurrent device, leading to a melted cable inside the distro.

### Sizing the Audio Feeder to Nameplate and Sagging on Bass Transients

The installer totals amplifier nameplate draw and sizes the feeder to that figure. The false signal is that the total fits the feeder. The mechanism of failure is that bass transients draw many times the average current for milliseconds, and a feeder sized to average draw sags in voltage on every transient, limiting amplifier output and stressing the power supplies. The harm is a system that sounds weak at full power and amplifiers that run hot and fail early.

### Coiling Excess Cable on the Reel Under Load

The installer leaves excess feeder or truss cable coiled on its reel or in a tight loop to tidy the run. The false signal is that the coil is neat and out of the way. The mechanism of failure is that coiled cable under load forms an inductor and concentrates heat, melting the insulation from the inside out. The harm is a destroyed cable, an unexpected feeder failure mid-show, and a fire source hidden inside the coil.

### Running Floor Cable Without Ramps in Audience Areas

The installer tapes a feeder across an audience floor without a cable ramp to save setup time. The false signal is that gaffer tape holds the cable flat. The mechanism of failure is that foot traffic and cleaning lift the tape edges, the cable shifts, and a trip edge forms; a heel catches it and a person falls or the cable is yacked out of the distro. The harm is audience injury, a disconnected feeder, and liability for an unprotected run.

### Overloading a Stage Pin or Edison Connector by Daisy-Chaining Fixtures

The installer daisy-chains several fixtures through a connector rated for less than the combined load. The false signal is that the connectors mate and the fixtures light. The mechanism of failure is that the connector and its jumper cable carry more than their rated current and overheat, especially in a hot truss. The harm is a melted connector, a fixture run that fails mid-show, and an arc source inside the truss.

### Sharing a Source Between Dimmers and Audio Without Isolation

The installer feeds dimmer racks and audio amplifiers from the same distro source with no isolation. The false signal is that the power is "all the same." The mechanism of failure is that dimmer harmonics and neutral noise couple into the audio system, producing hum and interference that appears only under show load. The harm is a noisy show, ground loops that are hard to diagnose live, and audio that never reaches its designed quality.

## Self-Check

- Did I balance phase-to-neutral loads across all three phases and specify a neutral sized for harmonic loading, not just unbalanced current?
- Did I size the distro and feeder for real show load, including lamp-strike inrush and audio bass transients, and verify phase currents and bus voltage under full load?
- Did I position dimmer racks with adequate airflow, balance dimmer channels across phases, and keep audio on a separately derived or isolated feed to control noise?
- Did I protect every cable run across walkways and audience areas with listed ramps or overhead rigging, with no unprotected floor runs?
- Did I verify every connector, cable, and splitter is within its rating, with derating applied for hot truss and stage ambient temperatures?
- Did I establish a single neutral-ground bond at the separately derived source and coordinate power and audio grounding to avoid ground loops?
- Did I avoid coiling excess cable under load, instead laying it in a figure-eight or non-inductive pattern?
- Is the show power documented so that any imbalance, sag, or noise found during the show can be traced to its source?
