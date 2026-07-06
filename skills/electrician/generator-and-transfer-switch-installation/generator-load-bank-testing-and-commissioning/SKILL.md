---
name: generator-load-bank-testing-and-commissioning.md
description: Use when the agent is commissioning standby generators with load bank testing, verifying performance per NFPA 110, selecting resistive versus reactive load banks, planning step loading and building load tests, or documenting commissioning results for emergency and legally required standby systems.
---

# Generator Load Bank Testing and Commissioning

A standby generator that has never carried real load is an unverified promise, and the load it must carry during an outage is the worst possible time to discover that it cannot. The judgment problem is that generators fail in ways that only appear under load: the engine may produce rated speed unloaded but droop under the heat and torque of full kilowatts, the cooling system may hold temperature for ten minutes and overheat at thirty, the alternator may sustain voltage at half load but collapse at full load, and the governor may hold frequency at steady state but hunt badly under step changes. Load bank testing is the only way to prove the generator can do its job, and NFPA 110 makes specific testing mandatory for Level 1 and Level 2 systems. An electrician who runs the generator unloaded, declares it commissioned, and walks away delivers a system whose first real test is the first real outage. This skill covers the test types, the step-loading protocol, the building-versus-portable load bank decision, and the documentation that makes the test legally and operationally meaningful.

## Core Rules

### Match the Load Bank Type to the Test Objective and the Load Characteristic

Load banks come in resistive and reactive types. A resistive load bank draws real power (kilowatts) only, which tests the engine's ability to produce torque and the cooling system's ability to reject heat, and it is the standard tool for NFPA 110 testing. A reactive load bank draws real and reactive power (kilowatts and kilovars), which additionally loads the alternator's excitation system and tests voltage regulation under lagging power factor, closer to a real building load with motors and transformers. The trap is using only a resistive bank on a generator that will serve a heavily inductive load, so the test passes but the alternator struggles with the real load's power factor. The defense is to use a resistive bank for the standard NFPA 110 test and to add reactive loading or a combined load bank where the served load is heavily inductive or where the alternator's performance under power factor is in question.

### Apply Step Loading to Verify the Engine and Governor Response

A generator must accept load in steps without excessive voltage and frequency droop, and NFPA 110 and ISO 8528 define acceptable transient response (commonly voltage recovery within a few seconds and frequency droop within a few percent). Step loading applies the load in increments (for example 25, 50, 75, 100 percent) and records the transient and the recovery at each step, which is the only way to verify the governor and excitation can handle the real load steps the building will impose. The trap is ramping the load bank up smoothly to 100 percent and recording a pass, which proves steady-state capability but not the step response that matters when the ATS transfers. The defense is to apply the load in steps matching the building's expected load sequence, record the voltage and frequency transient and recovery at each step, and verify the response is within the standard's limits.

### Conduct the NFPA 110 Required Duration Test at the Specified Load

NFPA 110 requires Level 1 generators to be tested at a specific load and duration, typically 30 percent for 30 minutes monthly (operational test) and a full-load test for the Class duration (such as 2 or 3 hours, or up to 4 hours for some classes) annually or triennially, with the exact requirements depending on the edition and the authority having jurisdiction. The test must reach the required load level, and if the building load is insufficient a supplemental load bank must be used. The trap is running the monthly test at whatever building load happens to be present (often far below 30 percent), which causes wet stacking in diesel engines and does not satisfy the test intent. The defense is to know the specific NFPA 110 test requirements adopted by the authority, ensure the test reaches the required load (using a load bank if building load is insufficient), and run the required duration.

### Decide Between Building Load and Portable Load Bank Based on Test Adequacy

The test load can be the actual building load (preferred where it is sufficient and representative) or a portable load bank brought to the site. Building load is realistic but may be too low (especially at night or on weekends), too variable, or unable to reach the required percentage without disrupting operations. A portable load bank is controllable and can reach any load level but adds cost and logistics. The trap is relying on building load that is consistently below the NFPA 110 minimum, so the generator is never properly loaded and wet-stacks or fails to prove full-load capability. The defense is to measure the typical building load during the test window, use a portable load bank to supplement where the building load is insufficient, and document the load source and magnitude for each test.

### Prevent Wet Stacking by Loading Diesel Generators Adequately

Diesel generators that run at low load (below about 30 percent) do not reach full combustion temperature, and unburned fuel and carbon accumulate in the exhaust (wet stacking), which fouls the turbocharger, the exhaust, and the cylinders and reduces reliability over time. NFPA 110's 30 percent minimum load requirement exists largely to prevent this. The trap is exercising the generator unloaded or at minimal building load for years, so the engine wet-stacks and then fails when finally called to full load. The defense is to ensure every operational test reaches at least 30 percent load (using a load bank if needed), to perform periodic full-load tests to clear deposits, and to recognize wet stacking (oily exhaust, smoke, slobbering) as a sign of chronic underloading.

### Document the Test With the Data That Proves Compliance and Supports Trending

A load bank test that is not documented did not happen, for NFPA 110 compliance purposes, and a test without data cannot be trended to detect degradation. The documentation must record the date, the load steps and durations, the voltage, frequency, exhaust temperature, cooling temperature, oil pressure, and any alarms, signed by the tester. The trap is recording only "generator tested, OK," which satisfies no inspector and provides no basis to detect that the engine is slowly losing output. The defense is to record the full test data on a standardized form, retain it for the authority having jurisdiction, and trend the data over time to catch degradation before it becomes a failure.

### Commission the Full System, Not Just the Engine

Commissioning a standby system means proving the engine, the alternator, the ATS, the controls, the alarms, and the integration all work together under simulated outage conditions. This includes a full utility-failure simulation (opening the normal source, verifying the generator starts and the ATS transfers within the required time, the load is carried, and retransfer occurs correctly when utility restores). The trap is load-bank testing the generator in isolation and assuming the ATS and controls will work, then discovering at the first outage that the ATS logic or the load sequencing fails. The defense is to include a full system functional test in the commissioning, simulate the outage end to end, and verify every alarm and control function.

## Common Traps

### Running the Monthly Test Unloaded and Wet-Stacking the Engine

The monthly operational test is run with no load or minimal building load because that is easiest, and over months the diesel wet-stacks. The mechanism of the failure is that low-load operation keeps combustion temperature too low to burn fuel completely, unburned fuel and carbon accumulate in the exhaust and cylinders, and the engine's reliability degrades until it cannot carry full load. The false signal is that the generator "starts and runs" each month, which proves the starter and engine work but not that the engine can carry load. The harm is a generator that fails exactly when the outage demands full output. The defense is to load every operational test to at least 30 percent using a load bank if needed.

### Ramping the Load Smoothly Instead of Stepping It

The load bank is ramped smoothly to 100 percent and the test is recorded as a pass, without any step loading. The mechanism of the failure is that smooth ramping proves steady-state capability but never exercises the governor and excitation's transient response, so the first real ATS transfer imposes a step load the generator has never seen and the voltage or frequency droops beyond the load's tolerance. The false signal is that the generator carries 100 percent, which is true in steady state but unproven under step. The harm is a failed transfer or load drop during the real outage. The defense is to apply load in steps matching the building sequence and record the transient response.

### Using Only a Resistive Bank on a Heavily Inductive Load

The generator is tested with a resistive load bank only, passes comfortably, but the served building load is full of motors and transformers with a poor power factor. The mechanism of the failure is that the resistive test does not load the alternator's excitation system the way a lagging power factor load does, so the alternator's voltage regulation under real load is untested and may collapse or hunt. The false signal is that the resistive test passes at full kilowatts, which proves the engine but not the alternator under the real load characteristic. The harm is voltage instability when the real inductive load transfers. The defense is to add reactive loading where the served load is heavily inductive.

### Recording Only "Tested OK" Without Trendable Data

The test report records only that the generator was tested and is OK, with no load steps, voltages, frequencies, or temperatures. The mechanism of the failure is that without numeric data there is no basis to trend degradation, so a slow loss of output or a rising exhaust temperature goes unnoticed until it becomes a hard failure. The false signal is that a report exists, which satisfies the appearance of compliance but not the intent of detecting degradation. The harm is a generator that fails between tests with no warning. The defense is to record full numeric data on a standardized form and trend it over time.

### Load-Bank Testing the Generator in Isolation and Skipping System Integration

The generator is load-bank tested at the terminals and declared commissioned, but the ATS, the controls, and the load sequencing are never tested as a system. The mechanism of the failure is that the integration, not the engine, is where most standby system failures occur, and an isolated engine test does not exercise the ATS transfer logic, the load shed, or the alarms. The false signal is that the generator carries load, which proves the engine but not the system. The harm is a system that fails to transfer or sequence correctly at the first outage. The defense is to include a full end-to-end utility-failure simulation in commissioning.

### Failing to Reach the NFPA 110 Required Load and Duration

The annual test is run at whatever building load is present for whatever time is convenient, falling short of the NFPA 110 required load percentage and duration. The mechanism of the failure is that the test does not meet the code requirement, so it does not satisfy the authority having jurisdiction and does not prove the generator can sustain the rated load for the required time. The false signal is that a test was performed, which proves activity but not compliance. The harm is a failed inspection and an unproven generator. The defense is to know the specific NFPA 110 load and duration requirements and to use a supplemental load bank to meet them.

## Self-Check

- Did I select a resistive, reactive, or combined load bank appropriate to the test objective and the power factor of the served load?
- Did I apply the load in steps matching the building's expected load sequence and record the voltage and frequency transient and recovery at each step against the standard's limits?
- Did I conduct the NFPA 110 operational and duration tests at the required load percentage and duration for the system's Level and Class, using a supplemental load bank where building load is insufficient?
- Did I decide between building load and portable load bank based on measured building load adequacy, and document the load source and magnitude for each test?
- Did I ensure every operational test reaches at least 30 percent load to prevent wet stacking, and did I perform periodic full-load tests to clear deposits?
- Did I record full numeric test data (load steps, voltage, frequency, temperatures, pressures, alarms) on a standardized form, retain it for the authority, and set up trending to detect degradation?
- Did I include a full end-to-end utility-failure simulation in commissioning that exercises the generator, ATS, controls, load sequencing, and alarms together?
- Is the commissioning documentation complete enough that the authority having jurisdiction accepts it and another practitioner can reproduce and trend the results?
