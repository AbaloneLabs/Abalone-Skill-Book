---
name: temporary-power-gfi-and-wet-location-protection.md
description: Use when the agent is protecting temporary and outdoor installations with GFCI, applying NEC 590.6 GFCI rules for temporary wiring, ensuring wet-location safety for events and construction, selecting ELCB devices for events, and avoiding daisy-chain GFCI limitations that defeat ground-fault protection.
---

# Temporary Power GFCI and Wet-Location Protection

Temporary power almost always lives outdoors, in wet conditions, on wet ground, and around people who are themselves wet. NEC 590.6 recognizes this by requiring GFCI protection on temporary installations more broadly than permanent work requires it, because the combination of exposure, movement, and condensed schedule makes ground faults both more likely and more dangerous. The judgment problem is that GFCI is treated as a checkbox instead of as a system to be designed: where the device sits in the distribution, how multiple devices interact, what happens when one trips, and how wet connections are protected all determine whether the protection actually works. Agents miss the issues because a GFCI that passes its test button appears functional, while the real questions of coverage, coordination, and wet-location sealing are never examined until a fault injures someone.

## Core Rules

### Apply NEC 590.6 GFCI Requirements to All Temporary Receptacles

NEC 590.6 requires GFCI protection for personnel on all 125-volt, 15-, 20-, and 30-ampere receptacles used for temporary wiring in construction, remodeling, maintenance, repair, demolitions, and similar activities, and for certain decorative lighting. The rule also extends GFCI to other receptacles where supplied by temporary wiring. The defense is to treat 590.6 as a broad mandate: every temporary receptacle in the covered scope gets GFCI, the device is installed at the source of the branch so the entire run is protected, and the requirement is not limited to "wet" receptacles because temporary environments are presumed wet and rough.

### Place the GFCI at the Source of the Branch, Not at the End

A GFCI protects only what is downstream of it. A device placed at the end of a long run, or on a single tool at the end of a daisy chain, leaves the entire upstream run unprotected. The defense is to install the GFCI at the origin of the branch — in the spider box, at the generator receptacle, or at the first outlet of the run — so that every foot of cable and every connector downstream is covered. This single placement decision determines whether a cut cable energizes the soil or trips the device instantly.

### Avoid Daisy-Chaining GFCI Devices That Defeat Coordination

When multiple GFCI devices are in series on one branch, a downstream device's trip may be masked or may cause confusing upstream behavior, and testing becomes unreliable. More importantly, some configurations of series GFCIs cause one device to trip on the test of another, leaving the installer unsure which device failed. The defense is to use a single GFCI at the source of each branch rather than stacking devices, to use GFCI breakers where the entire branch including the spider box should be protected, and to label each device so that when one trips, the source is identifiable and the cause can be found rather than guessed.

### Use ELCB and Higher-Capacity Devices Where the Load Exceeds Standard GFCI Ratings

Standard receptacle GFCIs are rated for 15 to 30 amps. Larger temporary loads — stage feeders, big tool banks, event distribution — exceed these ratings and require equipment leakage circuit breakers (ELCBs) or GFCI breakers rated for the higher current, sometimes with a higher trip threshold appropriate to the load. The defense is to match the ground-fault device's current rating to the branch it protects, to use ELCB or listed GFCI breakers for high-current branches, and to never substitute a standard breaker for a required GFCI because the load is "too big" for the available device.

### Protect All Connections and Devices From Wet Weather

GFCI protection clears faults, but wet connections corrode, track current across wet surfaces, and degrade the insulation that prevents the fault in the first place. Temporary connections must be kept out of standing water, elevated where possible, and weather-rated or enclosed where they will see rain. The defense is to use in-use weatherproof covers on receptacles, to elevate spider boxes and cord connections off the ground, to orient connectors so water does not run into them, and to shield distribution points from rain, because a wet connection is both a shock source and a source of nuisance GFCI tripping that tempts bypass.

### Test Every GFCI at Install and Verify It Trips, Never Bypass a Tripping Device

A GFCI that does not trip is worse than no GFCI because it creates a false sense of protection. Every temporary GFCI must be tested at installation with the test button and ideally with a calibrated tester, and any device that trips repeatedly must be diagnosed, not bypassed. The defense is to test every device at install, to re-test after any reconfiguration, and to treat repeated tripping as a real ground fault or moisture problem that must be located and fixed, because defeating a tripping GFCI removes the only fast-acting shock protection on a wet, rough site.

## Common Traps

### Placing the GFCI at the Last Outlet Instead of the Source

The installer puts a GFCI receptacle at the last outlet of a long temporary run and calls the branch protected. The false signal is that a GFCI is present and tests good. The mechanism of failure is that the entire upstream run — the cable on the ground, the connectors in the mud — is upstream of the device and completely unprotected. The harm is a cut or wet cable upstream that energizes the soil with no device to clear it, exactly the hazard the GFCI was meant to prevent.

### Stacking Multiple GFCIs in Series and Losing Coordination

The installer adds a GFCI receptacle downstream of a GFCI breaker, "for extra protection." The false signal is that more protection seems safer. The mechanism of failure is that the two devices interact, one masks the other's trip, and testing becomes ambiguous so the installer cannot tell which device failed or whether either works. The harm is unreliable protection and a branch that may be unprotected after a trip that was never correctly diagnosed.

### Omitting GFCI on a Hardwired or High-Current Temporary Load

The installer wires a large temporary load directly because no receptacle GFCI fits the rating, reasoning that GFCI is a receptacle rule. The false signal is that the load is hardwired and therefore exempt. The mechanism of failure is that the branch has no ground-fault protection at all, so a fault energizes the frame or the soil until an upstream overcurrent device trips — far too slow to prevent shock. The harm is an unprotected high-current branch in a wet environment, the worst combination.

### Bypassing a Tripping GFCI to Keep the Job Going

The temporary GFCI trips repeatedly and the installer replaces it with a standard receptacle to stop the downtime. The false signal is that the tripping was a nuisance and the work can now proceed. The mechanism of failure is that the tripping indicated a real leakage path — a wet connector, a damaged cable, water in a tool — and removing the protection leaves the hazard live. The harm is loss of the only fast shock protection and a wet fault that can injure the next person to touch the equipment.

### Leaving Connections in Standing Water Because the GFCI "Protects" Them

The installer leaves spider boxes and cord connections lying in puddles because the GFCI will trip if a fault occurs. The false signal is that the GFCI makes the wet placement acceptable. The mechanism of failure is that wet connections corrode and track current continuously, producing nuisance trips and degrading insulation until a fault the GFCI may or may not clear occurs. The harm is chronic tripping, corroded connectors, and a wet fault that eventually defeats the protection the installer relied on.

### Using Indoor-Rated Covers on Outdoor Temporary Receptacles

The installer uses standard flip covers on outdoor temporary receptacles because they were on hand. The false signal is that a cover is a cover. The mechanism of failure is that a standard cover does not seal around an inserted plug, so rain enters the receptacle and tracks current across the wet face. The harm is nuisance GFCI tripping, corroded receptacles, and a shock path across the wet device face.

## Self-Check

- Did I apply NEC 590.6 GFCI protection to every temporary receptacle in the covered scope, treating the temporary environment as wet by default?
- Did I place each GFCI at the source of its branch — spider box, generator, or first outlet — so the entire downstream run is protected?
- Did I avoid stacking multiple GFCIs in series, and is each device labeled so a trip can be traced to its source?
- Did I use ELCB or listed GFCI breakers rated for the current on branches that exceed standard receptacle GFCI ratings?
- Did I protect all connections and devices from wet weather with in-use covers, elevation off the ground, and shielding from rain?
- Did I test every GFCI at install with the test button and a calibrated tester, and did I avoid bypassing any tripping device?
- Did I diagnose the cause of any nuisance tripping rather than removing the protection, and is the wet connection or damaged cable fixed?
- Is the GFCI coverage documented so that every temporary branch can be confirmed protected before the site goes live?
