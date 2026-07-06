---
name: turbocharger-and-supercharger-service.md
description: Use when the agent is diagnosing turbocharger or supercharger boost loss, overboost, wastegate or boost control faults, turbo oil leakage or shaft play, supercharger noise or bypass valve faults, or evaluating charge-air cooling, boost leak, and forced-induction lubrication on boosted engines.
---

# Turbocharger and Supercharger Service

Forced induction multiplies engine output and engine stress, and the turbocharger or supercharger sits at the center of both, driven by exhaust heat (turbo) or engine torque (supercharger) and spinning or compressing at speeds and pressures that leave no margin for lubrication, airflow, or control faults. The judgment problem is that boost symptoms (low power, overboost, surge, whine, smoke, code) overlap across the turbo or supercharger itself, the boost control system (wastegate, bypass, solenoid), the charge-air system (intercooler, leaks), and the engine breathing and lubrication that feed it. A technician who replaces a turbo for what is a boost leak, or a wastegate for what is a stuck boost-control solenoid, hands back a vehicle with the same low power and a large invoice. This skill covers the disciplined diagnosis of forced-induction faults: separating the compressor from the control from the charge-air from the lubrication.

## Core Rules

### Distinguish Low Boost (Underboost) Causes: Leak, Control, or Compressor

An underboost code or low-power complaint has three main causes, and the disciplined diagnosis separates them by where the boost is lost. A boost leak (a cracked charge pipe, a blown intercooler, a loose clamp, a torn bypass valve diaphragm) loses compressed air between the compressor and the throttle; the turbo spins and builds pressure, but the pressure escapes, and the ECM sees less boost than commanded. A boost control fault (a wastegate stuck open, a failed boost-control solenoid, a stuck-open bypass) prevents the turbo from reaching target boost by venting exhaust past the turbine (wastegate) or venting charge air (bypass). A compressor fault (a worn or damaged turbo, a clogged air filter starving the compressor, a restricted exhaust choking the turbine) means the turbo cannot make boost even with correct control.

The tradeoff is that the turbo is the most expensive part and the leak is the cheapest, but jumping to the turbo is the common error. The disciplined technician pressure-tests the charge-air system for leaks (pressurize from the turbo outlet and listen and soap-test for leaks), commands the wastegate and bypass with a scan tool, and reads the commanded versus actual boost before condemning the turbo.

### Diagnose Overboost Through the Wastegate and Boost Control System

Overboost — actual boost exceeding the commanded target, often triggering a code and a fuel-cut limp mode — is almost always a boost control fault, not a turbo that is "too strong." The disciplined diagnosis focuses on the wastegate and the boost-control solenoid: a wastegate stuck closed (mechanical binding, a broken actuator linkage, a failed pneumatic actuator) cannot vent exhaust and the turbo over-speeds; a boost-control solenoid stuck closed or with a blocked reference line cannot modulate the wastegate; a VGT (variable-geometry turbo) mechanism stuck in the high-boost position overboosts on diesels. The diagnosis commands the wastegate or VGT with a scan tool, checks the actuator for free movement and correct reference pressure, and inspects the control lines for cracks and blockage. The tradeoff is that the wastegate and solenoid are serviceable, but overboost is often misdiagnosed as a turbo fault.

### Evaluate Turbo Shaft Play, Wheel Damage, and Oil Leakage Correctly

A turbocharger's health is assessed by shaft play, wheel condition, and oil leakage, and the disciplined evaluation uses all three. Shaft play: a small amount of radial play is normal (the journal bearings float on an oil film and have clearance), but play that lets the wheels contact the housings (felt as the wheel scraping when the shaft is pushed and turned) indicates worn bearings; axial play (in and out) beyond spec indicates thrust bearing wear. Wheel condition: the compressor and turbine wheels are inspected (with the intake and exhaust off) for damage — bent or broken fins from foreign object ingestion (a piece of the air filter, a broken injector tip), and contact scoring on the housings from shaft play. Oil leakage: oil in the compressor outlet or the exhaust indicates failed turbo seals, but oil leakage must be distinguished from a clogged crankcase ventilation system that pressurizes the crankcase and forces oil into the turbo drain.

The tradeoff is that shaft play alone is not a condemnation — some play is normal — but play with wheel contact or oil leakage is. The disciplined technician evaluates all three and checks the crankcase ventilation before condemning the turbo for oil leakage.

### Verify the Oil Supply and Drain That Determine Turbo Life

A turbocharger spins at over 100,000 rpm on a thin film of oil, and the oil supply and drain are the single biggest determinants of turbo life. The disciplined turbo service verifies: the oil supply line is clear and not restricted (a clogged or kinked supply line starves the turbo and cooks the bearings in seconds), the oil drain line is clear and gravity-fed without restriction (a clogged or low-mounted drain backs oil into the turbo and floods the seals), the oil is the correct specification and clean (sludged or wrong-viscosity oil does not lubricate the journal bearings), and the oil pressure and level are adequate. A turbo that fails repeatedly is almost always an oil supply or drain problem, and replacing the turbo without fixing the oil issue guarantees a repeat failure.

The tradeoff is that the oil lines are hard to access and often skipped, but a new turbo on clogged lines fails immediately. The disciplined technician inspects and cleans or replaces the oil supply and drain lines on every turbo replacement and primes the turbo before the first start.

### Address Charge-Air Cooling and Heat Soak on Intercooled Engines

The compression of intake air heats it, and the intercooler (air-to-air or air-to-water) removes that heat to increase density and prevent detonation. A restricted, leaking, or heat-soaked intercooler reduces boost density and causes power loss and detonation. The disciplined evaluation checks the intercooler for external damage and mud packing (restricting airflow), pressure-tests it for leaks, and on air-to-water systems checks the coolant pump, the heat exchanger, and the coolant level. The tradeoff is that the intercooler is often overlooked in favor of the turbo, but a restricted intercooler causes the same low-power symptom.

### Pre-Oil and Follow Break-In on Every Turbo or Supercharger Replacement

A new or rebuilt turbo must be pre-oiled before the first start, because dry journal bearings score in the first seconds. The disciplined pre-oiling fills the oil inlet with clean oil and cranks the engine (fuel or ignition disabled) until oil pressure builds, or uses a pressurized oiler through the supply line. The break-in avoids sustained high load and high boost for the first few hundred miles, allowing the bearings to seat. The tradeoff is that pre-oiling and break-in take discipline, but a dry-started turbo fails within hours.

## Common Traps

### Replacing the Turbo for a Boost Leak — The engine has low power and an underboost code, the technician replaces the turbo, and the power is still low because the charge pipe was cracked. The trap mechanism is that a boost leak produces the same underboost symptom as a failed turbo, and the turbo is the expensive, easy-to-name part. The false signal is the underboost code; the harm is an expensive turbo replaced for a cheap hose. The disciplined technician pressure-tests the charge-air system before the turbo.

### Condemning the Wastegate Actuator for a Boost-Control Solenoid Fault — The engine overboosts, the technician replaces the wastegate actuator, and the overboost persists because the boost-control solenoid or its reference line was the fault. The trap mechanism is that the wastegate is driven by the solenoid, and a solenoid fault mimics an actuator fault. The false signal is the overboost pointing at the wastegate; the harm is an actuator replaced for a solenoid. The disciplined technician commands the wastegate and tests the solenoid and reference lines first.

### Replacing a Smoking Turbo Without Checking Crankcase Ventilation — The turbo smokes from the exhaust, the technician replaces the turbo, and it smokes again because a clogged PCV pressurized the crankcase and forced oil into the turbo drain. The trap mechanism is that crankcase pressure pushes oil past the turbo seals, and the new turbo smokes the same way. The false signal is the oil smoke pointing at the turbo seals; the harm is a repeat turbo failure. The disciplined technician checks the PCV and crankcase pressure before the turbo.

### Installing a New Turbo on Clogged Oil Lines — The turbo is replaced, the oil supply and drain lines are reused, and the new turbo fails within hours because the clogged supply starved it or the clogged drain flooded it. The trap mechanism is that the oil lines caused the original failure, and reusing them dooms the new turbo. The false signal is the lines "looking fine"; the harm is an immediate repeat turbo failure. The disciplined technician inspects, cleans, or replaces the oil lines and pre-oils the new turbo.

### Ignoring Foreign Object Damage and Replacing the Turbo Without the Source — The compressor wheel is damaged by an ingested object, the technician replaces the turbo, and the new turbo is damaged again because the source (a disintegrating air filter, a broken intake pipe) was not fixed. The trap mechanism is that foreign object damage has a source upstream, and replacing the turbo without removing the source repeats the damage. The false signal is the damaged wheel pointing at the turbo; the harm is a repeat turbo failure. The disciplined technician finds and removes the source of the ingested object.

## Self-Check

- For an underboost complaint, did I pressure-test the charge-air system for leaks, command the wastegate and bypass, and read commanded versus actual boost before condemning the turbo?
- For an overboost complaint, did I test the wastegate actuator, the boost-control solenoid, and the reference lines before the turbo?
- Did I evaluate turbo shaft play (radial and axial), wheel condition, and oil leakage together, and distinguish normal play from bearing failure?
- For a smoking turbo, did I check the crankcase ventilation and crankcase pressure before condemning the turbo seals?
- Did I verify the oil supply line is clear, the drain line is unrestricted, the oil is the correct specification and clean, and the oil pressure is adequate?
- On every turbo replacement, did I inspect, clean, or replace the oil supply and drain lines, and pre-oil the turbo before the first start?
- Did I inspect the compressor and turbine wheels for foreign object damage and find and remove the source of any ingested object?
- Did I evaluate the intercooler for restriction, leaks, and (on air-to-water systems) coolant pump and heat exchanger function?
- Did I follow the break-in procedure (no sustained high boost for the first few hundred miles) and road-test to verify correct boost under load?
