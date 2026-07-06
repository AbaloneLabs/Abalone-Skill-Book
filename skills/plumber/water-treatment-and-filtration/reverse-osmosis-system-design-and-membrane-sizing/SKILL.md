---
name: reverse-osmosis-system-design-and-membrane-sizing.md
description: Use when the agent is designing or installing a point-of-use or whole-house reverse osmosis system, sizing RO membranes and storage tanks, calculating recovery rate and concentrate waste flow, selecting prefilter and postfilter stages, or troubleshooting low pressure, low output, or high TDS creep in an RO unit.
---

# Reverse Osmosis System Design and Membrane Sizing

Reverse osmosis (RO) forces water through a semi-permeable membrane that rejects the majority of dissolved solids, producing low-TDS permeate and a concentrated brine waste stream. The judgment problem is that RO performance depends on a balance of feed pressure, temperature, membrane area, recovery rate, and prefiltration — and an undersized, mis-prefiltered, or poorly plumbed system will produce too little water, waste too much concentrate, suffer premature membrane fouling, or deliver permeate whose TDS creeps up as the membrane is damaged. Worse, whole-house RO systems generate large volumes of concentrate waste and require atmospheric storage and repressurization, introducing a layer of complexity and contamination risk that point-of-use units do not have. This skill covers sizing, prefiltration, recovery, storage, and the role limits that place large commercial/industrial RO design with water treatment specialists.

## Core Rules

### Size the Membrane for Daily Demand and Recovery, Not Rated Output

RO membrane output is rated at standard conditions (commonly 77°F feed, 65 psi feed pressure, low TDS), and real-world output is almost always lower — roughly 30 to 50 percent lower for typical residential feed temperatures and pressures. The trap is selecting a "50 gpd membrane" expecting 50 gallons per day and getting 25, because the feed is 55°F (cold water produces far less permeate) and 45 psi (low pressure produces far less permeate). The disciplined approach is to derate the membrane for actual feed temperature and pressure using the manufacturer's correction factors, then size the membrane daily output to exceed the daily demand with margin. For whole-house systems, compute the daily household demand (drinking, cooking, and any plumbed RO fixtures), derate the membrane, and confirm the daily permeate production covers the demand within the storage tank's drawdown. Always verify feed pressure with a gauge before sizing — a system sized for 60 psi that receives 35 psi will underperform dramatically.

### Protect the Membrane with Correct Prefiltration

The RO membrane is the most expensive and fragile component, and it is destroyed by three things: chlorine/chloramine (which embrittles and dissolves the polyamide thin-film membrane), particulate and sediment (which plugs the membrane surface), and scale (which forms when the concentrate exceeds the solubility of calcium, magnesium, silica, or iron at the membrane's recovery rate). Prefiltration must address all three: a sediment prefilter (commonly 5 micron, sometimes 1 micron) to protect the membrane from particulate; a carbon prefilter to remove chlorine and chloramine before the membrane (essential for chlorinated municipal supplies and critical for chloramine, which carbon removes more slowly than chlorine); and, where hardness or scale-forming minerals are high, a softener or antiscalant dosing ahead of the RO to prevent membrane scaling. The trap is installing an RO unit with only a single carbon prefilter on hard, chloraminated water — the carbon may not fully remove chloramine at the contact time, and the membrane scales within months. The disciplined approach is to analyze the feed water, specify the prefiltration to match, and replace prefilters on schedule so a spent carbon cartridge does not let chlorine through to the membrane.

### Manage Recovery Rate and Concentrate Waste

Recovery is the percentage of feed water that becomes permeate; the rest is concentrate (brine) sent to drain. Residential point-of-use RO systems typically run 15 to 25 percent recovery (4 to 7 gallons of waste per gallon of permeate), while commercial systems can be tuned higher with flow restrictors and energy recovery. The trap is assuming RO is "efficient" and ignoring the concentrate volume — a whole-house RO producing 100 gpd of permeate at 25 percent recovery sends 300 gpd of concentrate to drain, which over a year is over 100,000 gallons of wastewater, and in septic systems this volume and salinity can overload the leach field. The disciplined approach is to calculate the concentrate flow, confirm the drain and (if applicable) the septic system can accept it, and consider a permeate pump or higher-recovery design for whole-house systems. Never discharge RO concentrate to a graywater or rainwater system — the concentrate is high-TDS brine.

### Use Correct Storage: Pressurized for POU, Atmospheric for POE

Point-of-use RO systems store permeate in a small pressurized bladder tank (typically 2 to 4 gallons nominal, 1 to 1.5 gallons drawdown) that supplies the faucet. Whole-house (point-of-entry) RO systems require a large atmospheric storage tank (because the membrane produces slowly and the house draws in surges) plus a repressurization pump to deliver house pressure from the atmospheric tank. The trap is using a pressurized tank for a whole-house system and expecting it to supply multiple fixtures — the drawdown is far too small, and the fixtures will run dry the moment two are opened. The disciplined approach is to match the storage type to the application: pressurized bladder tanks for single-faucet POU, atmospheric storage with repressurization for POE. Atmospheric tanks require a hydro-pneumatic pressure tank, a pump with pressure switch, and — critically — a vent with insect screen and sanitized tank interior, because atmospheric storage is open to airborne contamination.

### Prevent TDS Creep and Membrane Degradation

TDS creep is the rise in permeate TDS that occurs when the system sits idle: concentrated brine diffuses through the membrane into the permeate side, so the first water drawn after a period of no use has elevated TDS. This is normal in small amounts, but persistent high permeate TDS indicates membrane damage (from chlorine, scaling, or chemical attack), a failed auto-shutoff or check valve, or exhaustion of the carbon prefilter allowing chlorine through. The trap is assuming "the RO makes water, so it's fine" while the permeate TDS climbs from 20 to 200 ppm over months as the membrane is slowly destroyed. The disciplined approach is to measure permeate TDS periodically with a handheld meter, compare to the feed TDS to compute rejection rate (a healthy membrane rejects 95 to 99 percent), and replace the membrane when rejection falls below the manufacturer's spec. Replace carbon prefilters on schedule to prevent chlorine reaching the membrane.

### Respect the Role Limits — Large RO Systems Need Specialists

Point-of-use under-sink RO units are within a licensed plumber's scope. Whole-house RO, commercial RO for restaurants and laboratories, and industrial high-purity RO are the province of water treatment specialists who can perform the detailed feed analysis, recovery calculations, antiscalant dosing design, and storage/repressurization engineering. Confirm scope before taking on a whole-house or commercial system, and escalate to a water treatment professional when the feed water is unusual (high silica, high TDS, hydrogen sulfide, industrial contaminants) or the demand is commercial.

## Common Traps

### Believing the Membrane's Rated Output

The installer selects a 75 gpd membrane expecting 75 gallons per day, but the feed is 50°F and 40 psi, and the real output is closer to 30 gpd. The trap is that the rated output is at standard conditions (77°F, 65 psi) that almost never exist in a real installation, and cold low-pressure feed produces dramatically less permeate. The mechanism is that membrane permeability is temperature- and pressure-dependent, and the derating is large. The false signal is "the membrane is rated 75 gpd." The harm is a system that cannot meet daily demand, runs the storage tank dry, and delivers no water at the faucet during peak use. The defense is to derate the membrane for actual feed conditions and size with margin.

### Single Carbon Prefilter on Chloraminated Hard Water

The installer puts a single carbon block ahead of the RO on municipal chloraminated, moderately hard water, with no softener. The trap is that chloramine is removed more slowly than chlorine (it needs longer carbon contact time), and a single undersized carbon cartridge may pass chloramine to the membrane, which it destroys over weeks to months; meanwhile the hardness scales the membrane. The mechanism is that polyamide membranes have zero chlorine/chloramine tolerance, and scale plugs the membrane surface. The false signal is "there's a carbon filter, so the membrane is protected." The harm is premature membrane failure (weeks to months instead of years) and steadily rising permeate TDS. The defense is to size carbon contact time for chloramine, use catalytic carbon where chloramine is present, and soften the feed when hardness is elevated.

### Discharging Concentrate to a Septic System Without Evaluation

The installer runs the RO concentrate line to the septic system on a rural whole-house install. The trap is that RO concentrate is high-TDS, high-salinity brine, and a whole-house RO can send hundreds of gallons per day of it into the septic tank, where the salinity can disrupt the biological treatment and the volume can hydraulically overload the leach field. The mechanism is that the brine's salinity and the daily volume exceed what the septic biology and soil can handle. The false signal is "it's just water going to the septic." The harm is septic system failure and costly leach field replacement. The defense is to calculate concentrate volume, evaluate the septic system's capacity, and consider a dry well, surface discharge (where permitted), or a higher-recovery design.

### Atmospheric Storage Without Sanitization or Protected Vent

The installer puts in a whole-house RO with an atmospheric storage tank but does not sanitize the tank, leaves the vent open without an insect screen, or omits post-treatment UV. The trap is that atmospheric storage is open to the environment, and an unsanitized tank with an unprotected vent grows bacteria that colonize the permeate and the downstream piping. The mechanism is that low-TDS RO permeate is actually more hospitable to some bacteria (it lacks the disinfectant residual of municipal water), and an open vent admits microbes and insects. The false signal is "the RO removes contaminants, so the water is pure." The harm is bacterial contamination of the "purified" water at the tap. The defense is to sanitize the tank on installation and at maintenance intervals, install a screened vent, and add post-treatment UV for whole-house RO.

## Self-Check

- Did I derate the membrane for actual feed temperature and pressure using the manufacturer's correction factors, and does the derated daily output exceed the daily demand with margin?
- Did I verify feed pressure with a gauge before sizing, and did I confirm the feed conditions the membrane rating assumes are not the real conditions?
- Did I specify prefiltration to match the feed analysis: sediment prefilter, carbon (or catalytic carbon for chloramine) sized for adequate contact time, and a softener or antiscalant where hardness or scale-forming minerals are elevated?
- Did I calculate the recovery rate and concentrate waste flow, confirm the drain can accept it, and evaluate septic impact (or route to a dry well/permitted discharge) for whole-house systems?
- For point-of-use, is a pressurized bladder tank sized for the faucet drawdown; for point-of-entry, is an atmospheric storage tank with repressurization pump and pressure tank used instead?
- If atmospheric storage is used, is the tank sanitized on installation, vented through an insect screen, and followed by post-treatment UV to prevent bacterial contamination of the permeate?
- Did I measure permeate TDS and compute rejection rate, and is there a plan to monitor TDS and replace the membrane when rejection falls below spec?
- Did I confirm my scope covers this system, and did I escalate whole-house, commercial, or unusual-feed systems to a water treatment specialist?
