---
name: backflow-test-procedure-and-gauge-accuracy-decisions.md
description: Use when the agent is field-testing a backflow preventer (RPZ, DCDA, DCVA, PVBA, SVBA), following ASSE 5110 or USC FCCC manual procedures, using a 5-valve differential test kit, interpreting check valve tightness and relief valve opening pressures, judging pass/fail criteria, or documenting test results on required jurisdictional forms for the water purveyor.
---

# Backflow Test Procedure and Gauge Accuracy Decisions

A backflow preventer test is the only field evidence that an assembly is actually protecting the potable supply, and the test is only as valid as the procedure and the gauge behind it. The judgment problem is that each assembly type has a distinct procedure (an RPZ requires a four-value test including relief-valve opening at or above 2 psi differential; a DCDA tests two independently-spring-loaded checks; a PVBA or SVBA tests the air-inlet opening under vacuum), and a tester who runs the wrong sequence, uses a gauge that has drifted out of calibration, or reads a marginal 1.7 psi as "close enough to 2.0" will pass assemblies that have failed. A bad test is worse than no test, because it generates paperwork asserting protection that does not exist. This skill covers the procedural, gauge-accuracy, and documentation decisions that keep tests legally defensible and the potable supply actually protected.

## Core Rules

### Match the Procedure to the Assembly Type and the Accepted Manual

Each assembly type demands its own procedure. A reduced pressure principle assembly (RPZ/RPDA) is tested per the USC FCCC Manual or ASSE 5110 in a specific sequence: record static line pressure at test cock 2, verify check 1 tightness by bleeding downstream and confirming it holds at or above 2 psi differential, verify check 2 tightness, then verify the relief valve opens at or above 2 psi differential. A double check (DCVA/DCDA) tests only the two checks for tightness and leak-down. A pressure vacuum breaker (PVBA) and spill-resistant vacuum breaker (SVBA) test the air-inlet valve opening under a induced vacuum, not a positive differential. The disciplined rule is to identify the assembly type and manufacturer before connecting the kit, to follow the procedure written for that type, and never to substitute a DCVA sequence for an RPZ or vice versa. Record the procedure reference used on the test form.

### Verify Gauge Accuracy Before Every Test, Not Just at Calibration

A differential gauge is the instrument that makes the test a test; if it has drifted, every reading is wrong in the same direction. The annual calibration by an accredited lab is the baseline, but field verification before each job catches drift, shipping damage, and a gauge that was dropped. With a 5-valve test kit, perform a field zero check: vent all hoses, confirm the needle sits at zero, then apply a known reference pressure and confirm the gauge tracks. A digital gauge should be zeroed per the manufacturer on a level surface. A gauge that will not zero, that sticks, or that reads off by more than its stated tolerance (commonly 0.2 psi or 2% full scale) is out of service until recalibrated. The disciplined rule is: no test proceeds on a gauge that fails the field check, regardless of what the calibration sticker says.

### Apply the Pass/Fail Thresholds Without Rounding in the Assembly's Favor

Pass criteria are not matters of judgment. For an RPZ the relief valve must open at 2.0 psi differential or greater, and both checks must hold their differential without leaking down faster than the standard allows; a DCVA's checks must hold tight against backpressure and backsiphonage; a PVBA air inlet must open at or above 1.0 psi vacuum. The trap is reading 1.8 psi and recording 2.0, or watching a check leak down slowly and calling it "tight enough." A reading below threshold is a failure regardless of how close, because the threshold is the fail-safe line. The disciplined rule is to record the actual numeric reading on the form, to compare it to the published threshold, and to mark FAIL when any value is below threshold. Marginal passes are not passes; they are assemblies needing repair and re-test.

### Document Every Reading, Gauge ID, and Calibration Date on the Required Form

The test record is a legal document relied on by the water purveyor, the AHJ, and any party in a contamination lawsuit. It must capture the assembly make, model, size, serial number, and location; the test date; the tester name and certification number; the gauge manufacturer, serial number, and calibration date; each individual reading (not just pass/fail); and the pass/fail determination. Many jurisdictions mandate a specific form (the purveyor's form, the state form, or a USC FCCC form). The disciplined rule is to fill the required form completely in the field while connected, never to reconstruct readings from memory later, and to include the gauge calibration date so the record is auditable. An incomplete or back-dated form is unverifiable and, if falsified, fraudulent.

### Re-Test After Any Repair and Treat a Failed Test as a Work Order

When an assembly fails, the failed component (a check cartridge, relief-valve diaphragm, spring, or seat) must be repaired or replaced with the manufacturer's kit, and the assembly must be re-tested to confirm the repair restored function. The trap is "adjusting" a worn check by tightening the spring to push the reading above 2 psi, which masks the worn seat and guarantees a repeat failure between annual tests. The disciplined rule is to identify the failed component, repair or replace it, and re-test using the full procedure; document the failure, the parts replaced, and the re-test readings on the same record. An assembly that cannot be repaired in place must be tagged out of service and reported to the purveyor so the supply is not left unprotected.

## Common Traps

### Running the Wrong Sequence for the Assembly Type

A tester trained on RPZs runs an RPZ four-value sequence on a DCDA, or applies a positive-differential check test to a PVBA that requires a vacuum air-inlet test. The trap is that the kit connects and produces numbers that look plausible. The mechanism is that the procedures differ by assembly family, and a wrong sequence yields readings that have no pass/fail meaning. The false signal is a completed form with numbers on it. The harm is an assembly certified on an invalid test, leaving a hazard unverified. The defense is to identify assembly type and manufacturer first, follow the matched procedure, and record the procedure reference on the form.

### Trusting the Calibration Sticker Over the Field Zero Check

A gauge was calibrated 11 months ago and the sticker is still valid, but it was dropped last week and now reads 0.4 psi high. The trap is that the tester trusts the sticker and skips the field check. The mechanism is that mechanical and digital gauges drift between calibrations from shock, temperature, and zero shift. The false signal is a valid calibration sticker. The harm is every test that day recorded with a 0.4 psi error, passing assemblies whose relief valves actually open at 1.6 psi. The defense is a field zero/reference check before every job and pulling the gauge from service when it will not track.

### Recording a Marginal Reading as a Pass

A relief valve opens at 1.8 psi on an RPZ requiring 2.0, and the tester writes 2.0 "because it's close." The trap is that the threshold is a hard line set to guarantee fail-safe discharge. The mechanism is that below 2.0 psi differential the relief valve may not open during a real backpressure event. The false signal is "the relief valve did open." The harm is an assembly left in service below its fail-safe point. The defense is to record the actual reading and mark FAIL below threshold, then repair and re-test.

### Filling the Form From Memory After Leaving the Site

A tester performs the test but completes the paperwork back at the shop, reconstructing readings. The trap is that reconstructed readings are not evidence; they are fiction. The mechanism is that the form is the legal record and must reflect observed values. The false signal is a complete, submitted form. The harm is that in a contamination lawsuit the record cannot defend the tester or the purveyor. The defense is to complete the required form in the field while connected, recording each reading as observed.

### Masking a Worn Check With a Spring Adjustment

A check leaks down too fast, so the tester tightens the adjusting screw to push the reading above threshold, avoiding a repair. The trap is that the adjustment raises today's reading but does not restore the worn seat. The mechanism is that the underlying wear remains. The false signal is a passing re-test. The harm is the assembly fails again within weeks, between annual tests, while the form says it passed. The defense is to replace the failed component, not adjust around it, and to re-test after the repair.

## Self-Check

- Did I identify the assembly type (RPZ, DCDA, DCVA, PVBA, SVBA) and manufacturer before connecting the kit, and follow the matched procedure from ASSE 5110 or the USC FCCC manual?
- Did I perform a field zero/reference check on the differential gauge before the test, and is the gauge within its calibration interval?
- For an RPZ, did I record all four values (static pressure, check 1 tightness, check 2 tightness, relief valve opening pressure) and confirm the relief valve opens at or above 2.0 psi differential?
- Did I record the actual numeric readings, not rounded values, and mark FAIL for any reading below the published threshold?
- Did I avoid treating a marginal reading (e.g., 1.8 psi) as a pass, and instead flag the assembly for repair?
- For a failed assembly, did I identify and replace the failed component rather than adjusting the spring to mask the failure, and re-test after the repair?
- Did I complete the jurisdiction-required form in the field with assembly make/model/size/serial/location, test date, tester certification number, gauge serial and calibration date, each reading, and pass/fail?
- Did I submit the completed test record to the water purveyor within the required timeframe?
- Did I confirm there is no untested bypass around the assembly and that it is installed in the correct orientation with required clearances and (for RPZ) proper drain provision?
- If the assembly could not be repaired in place, did I tag it out of service and notify the purveyor so the supply is not left unprotected?
