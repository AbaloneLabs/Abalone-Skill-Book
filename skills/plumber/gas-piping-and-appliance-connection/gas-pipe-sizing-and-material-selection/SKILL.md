---
name: gas-pipe-sizing-and-material-selection.md
description: Use when the agent is sizing natural gas or propane piping, selecting pipe material for fuel gas service, calculating pressure drop and equivalent length, or determining whether CSST, black steel, PE, or copper is permitted for a gas installation.
---

# Gas Pipe Sizing and Material Selection

Fuel gas piping is one of the few plumbing systems where a sizing or material error does not cause a slow leak or a nuisance — it causes an explosion, asphyxiation, or carbon monoxide poisoning. The judgment problem is that gas pipe sizing is governed by long-run pressure drop and appliance demand, not by the peak flow at the meter, and the correct material depends on gas type, pressure tier, location (concealed, exposed, underground), and local code amendments that frequently forbid what the national fuel gas code allows. An installer who sizes by "what worked last time" or picks material by what is in the truck will eventually undersupply an appliance, causing flame rollout, sooting, and CO production, or will install a material that is prohibited in the jurisdiction and fails inspection. This skill covers the decisions that determine whether a gas piping system will deliver the required flow at every appliance under every demand condition, using materials that are legal and durable.

## Core Rules

### Size by Longest Run and Total Connected Load, Not by Appliance Count

Gas pipe sizing is a network problem, not a per-branch problem. The correct method is to identify the longest run from the meter or regulator to the most remote appliance, then size every segment in the system to carry its connected load at the pressure drop assumed for that longest run. The trap is sizing each branch only for its own appliance and ignoring that upstream segments carry the sum of all downstream loads. A 3/4-inch line that is fine for a single 60,000 BTU furnace becomes inadequate when the same trunk also feeds a dryer, a range, and a tankless water heater, because the pressure drop accumulates along the shared run and the most remote appliance starves. Always sum connected loads (with the appropriate diversity factor where the code allows) and use the longest-run method or the branch-length method consistently — do not mix them on one system, because they assume different pressure drops and produce inconsistent results.

### Account for Equivalent Length, Not Just Measured Pipe

Fittings and valves impose resistance far greater than their physical length. A 90-degree elbow can add 2 to 5 feet of equivalent pipe depending on size; a globe valve can add dozens of feet. The installer who measures the straight run, ignores the fittings, and picks the next size up "for safety" is sometimes right and sometimes wrong, because the equivalent length of a heavily fitting-laden run can exceed the straight length by a factor of two or more. The disciplined approach is to calculate equivalent length for every segment, add it to the measured length, and size from the tables using that total. When the calculated size falls between two standard sizes, round up — gas piping has no margin for undersizing, and the cost difference between 3/4 and 1 inch is trivial compared to the cost of a starved appliance or a redone installation.

### Match Material to Gas Type, Pressure Tier, and Location

Material selection is not a matter of preference; it is governed by code and by the physical demands of the service. Black steel pipe is the default for interior natural gas and propane at low pressure, but it must be protected from corrosion in damp locations and is generally prohibited underground without a factory-applied coating and cathodic protection or a wrapped assembly. Corrugated stainless steel tubing (CSST) is permitted and convenient but requires bonding per the manufacturer and local amendments, and several jurisdictions restrict it in concealed spaces or require arc-resistant jackets. Polyethylene (PE) is the standard for underground outdoor gas service but is never permitted inside a building because it cannot be joined by mechanical fittings through a building wall without a transition. Copper tubing is allowed for natural gas in some codes if flare-nit joints are used, but many jurisdictions prohibit it entirely due to sulfide attack on the copper from natural gas odorants. The decision must be made against the adopted code (IFGC, NFPA 54, or local), the gas type, the pressure, and the specific location — never against general habit.

### Verify Pressure Tier Before Sizing

Gas systems operate at low pressure (typically under 2 psi for residential), medium pressure (2 to 5 psi common for tankless and commercial), and high pressure (above 5 psi, requiring regulators at each appliance). The sizing tables are entirely different for each tier, and a pipe sized for low pressure will be grossly oversized or undersized if the system is actually medium pressure. Before sizing anything, confirm the supply pressure at the meter or regulator, confirm whether appliance regulators are present, and confirm that every appliance is rated for the delivered pressure. A tankless water heater fed medium-pressure gas without a properly set appliance regulator will receive excessive pressure and lock out or fail; the same unit fed low-pressure gas through an undersized line will underfire and produce insufficient hot water. The pressure tier is a design input, not an afterthought.

### Pressure-Test Before and After Concealment

Every gas piping system must be pressure-tested before being put in service, and the test must be witnessed and documented. The standard test is air or nitrogen at a pressure and duration specified by code (commonly 1.5 times working pressure for at least 10 minutes, or a fixed minimum such as 10 psi for residential, with longer holds for larger systems). The test must be performed with a calibrated gauge, all outlets capped, and the system isolated from the meter and appliances. The trap is testing only at the end of the job, after the piping is concealed in walls and floors — a leak then requires destructive access to find and repair. The disciplined practice is to test in stages: rough test before concealment, and a final test before appliance connection, with both witnessed and recorded. Never use the gas meter or appliance as part of the test boundary, and never test with gas itself.

## Common Traps

### Sizing the Trunk Like a Branch

The installer runs a 3/4-inch trunk from the meter and tees off to each appliance, sizing each tee for its appliance alone. The trunk, however, carries the sum of all downstream appliances, and at full demand the pressure drop along the trunk starves the most remote fixture. The trap is that each branch calculation looks correct in isolation, so the error is invisible until multiple appliances fire simultaneously — typically the furnace, water heater, and range on a cold morning. The mechanism is the additive nature of pressure drop along shared pipe, and the harm is appliance underperformance, nuisance lockouts, and in severe cases CO production from incomplete combustion. The defense is to always calculate trunk loads as the sum of downstream connected loads and size from the longest-run tables.

### Ignoring Equivalent Length of Fittings

The installer measures 40 feet of straight pipe, ignores the eight elbows and two valves, and selects a size from the table using 40 feet. The actual equivalent length is closer to 70 feet, and the selected size is now undersized. The trap is that the error is silent — the pipe is installed, the appliances fire, and nothing obviously fails during a brief test, because not all appliances run at once. The false signal is "it works," which is actually "it works under partial load." The harm manifests later as intermittent appliance problems that are difficult to diagnose because the piping is concealed and the symptom is intermittent. The defense is to always add equivalent length for every fitting and valve, and to round up when between sizes.

### Using Prohibited Material Because It Is Convenient

The installer has CSST on the truck and runs it through a concealed wall cavity in a jurisdiction that requires arc-resistant jacket or prohibits CSST in concealed spaces without bonding. Or the installer uses copper for natural gas in a jurisdiction that prohibits it, or uses black steel underground without protection. The trap is that the material is physically capable of carrying the gas, so the installation appears functional, and the prohibition is a code rule rather than a physical impossibility. The false signal is "it holds pressure," which says nothing about code compliance or long-term safety. The harm is a failed inspection, a forced redo, or — worse — a code violation that contributes to a later incident and becomes a liability issue. The defense is to verify permitted materials against the adopted code and local amendments before selecting, not after installing.

### Assuming Low Pressure When the System Is Medium Pressure

The installer sizes a gas line using the low-pressure tables because that is the default for residential work, but the supply is actually medium pressure with appliance regulators. The resulting pipe is grossly oversized — wasteful but not dangerous — or, more dangerously, the installer assumes medium pressure and sizes small, then the supply is actually low pressure and the appliances starve. The trap is that pressure tier is invisible until measured, and the assumption is rarely verified at the meter. The false signal is that the appliances appear to fire during a brief test. The harm is intermittent underperformance or, in the worst case, CO production. The defense is to always verify supply pressure with a manometer before sizing, and to confirm the pressure tier in writing with the gas utility or on the regulator nameplate.

### Testing Only at the End of the Job

The installer completes the entire gas system, conceals it in walls, and then pressure-tests. A leak is found, but the piping is now inaccessible without opening finished surfaces. The trap is that end-of-job testing is technically compliant with a literal reading of the test requirement but defeats the purpose of testing, which is to find leaks while they are cheap to fix. The false signal is "I tested it," which is true but late. The harm is expensive rework, damaged finishes, and schedule disruption. The defense is to test in stages — rough test before concealment, final test before connection — and to record both.

## Self-Check

- Did I size every segment using the sum of downstream connected loads, not just the local appliance load, and did I use the longest-run or branch-length method consistently throughout the system?
- Did I add equivalent length for every fitting, valve, and meter assembly before selecting pipe size from the tables, and did I round up when the calculated size fell between standard sizes?
- Did I verify the supply pressure at the meter or regulator with a manometer before sizing, and did I confirm whether the system is low, medium, or high pressure and select the correct tables?
- Did I confirm that every material selected (black steel, CSST, PE, copper) is permitted by the adopted fuel gas code and local amendments for the gas type, pressure, and specific location (concealed, exposed, underground, inside building)?
- If CSST is used, is bonding specified per manufacturer and local requirements, and is the correct jacket type used where required?
- Is underground piping protected against corrosion with factory coating, wrap, or cathodic protection as required, and is PE prohibited inside the building except for the approved transition?
- Did I pressure-test with air or nitrogen at the code-specified pressure and duration using a calibrated gauge, with all outlets capped and the system isolated from the meter and appliances?
- Did I test in stages — rough test before concealment and final test before appliance connection — and are both tests witnessed and documented?
- Did I confirm that every appliance is rated for the delivered pressure and that appliance regulators are present and correctly set where medium or high pressure is supplied?
- Are pressure drop assumptions, material selections, and test results recorded so that a future inspector or technician can verify the design basis?
