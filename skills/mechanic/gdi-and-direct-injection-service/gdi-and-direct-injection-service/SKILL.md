---
name: gdi-and-direct-injection-service.md
description: Use when the agent is diagnosing gasoline direct injection misfire, carbon buildup, injector leak or clog, high-pressure fuel pump faults, GDI fuel trims, or performing intake valve decarbonizing, injector balance, and high-pressure fuel system diagnosis on direct-injection engines.
---

# GDI and Direct Injection Service

Gasoline direct injection (GDI) engines deliver fuel directly into the cylinder at high pressure, and they introduce a set of failure modes that port-injection engines do not have: carbon buildup on the intake valves (because fuel no longer washes the valves), high-pressure fuel pump (HPFP) failures, direct injector coking and leaks, and LSPI (low-speed pre-ignition) tied to oil formulation. The judgment problem is that GDI symptoms (misfire, rough idle, cold-start stumble, lean codes, startability, carbon-knock) overlap across these causes and across conventional causes (ignition, compression, MAF), and because the high-pressure fuel system is dangerous to open and requires specific procedures to diagnose. A technician who decarbons the valves for what is an HPFP, or who condemns injectors for what is carbon, hands back a vehicle with the same rough idle. This skill covers the disciplined diagnosis of GDI-specific faults: carbon, high-pressure fuel, and injectors.

## Core Rules

### Recognize Intake Valve Carbon as the Hallmark GDI Fault and Diagnose by Symptom and Scope

In a GDI engine, the intake valves no longer see the fuel spray that washed them clean in port injection, and over miles the PCV vapors and oil mist bake onto the back of the intake valves as hard carbon, causing rough idle, cold-start stumble, misfire codes (often random or specific to one bank), and a ticking or rattling at idle. The carbon restricts airflow at low valve lift and disrupts the air-fuel mixture, and the symptoms worsen gradually. The disciplined diagnosis confirms carbon with a borescope inspection through the intake (the valves are visibly caked), distinguishes it from ignition and compression causes (the misfire is not tied to a single cylinder's plug or compression), and evaluates whether the carbon is severe enough to cause the symptom (light carbon is normal on all GDI engines; heavy asymmetric carbon causes the stumble).

The tradeoff is that borescope inspection takes minutes and confirms the cause, but assuming "all GDI needs decarbonizing" leads to unnecessary service. The disciplined technician scopes the valves, evaluates the carbon severity against the symptom, and recommends decarbonizing (walnut blasting or chemical) only when the carbon is severe and the symptom matches.

### Diagnose the High-Pressure Fuel Pump Through Pressure Data, Not Just Codes

The HPFP raises fuel pressure from the low-pressure side (50 to 60 psi from the in-tank pump) to the direct-injection rail pressure (often 500 to 3000 psi), and its failure modes (internal wear, cam follower wear, pressure regulator fault) cause low rail pressure, lean codes, long crank, no-start, and a characteristic chirp or tick. The disciplined diagnosis reads the actual versus the commanded rail pressure on the scan tool (the ECM commands a target; a healthy HPFP tracks it across load; a failing HPFP falls behind under load), checks the low-pressure supply to the HPFP (a weak in-tank pump starves the HPFP and mimics its failure), and evaluates the HPFP cam follower or lifter on engines where it is accessible (a worn follower damages the pump cam and is a known failure point). The tradeoff is that the HPFP is expensive, but condemning it without verifying the low-pressure supply and the commanded-versus-actual pressure wastes the replacement.

### Test Direct Injectors With Balance, Leak-Down, and Scope, Not Just Codes

GDI injectors fail by coking (restricted flow, causing a lean misfire on that cylinder), leaking (fuel dripping into the cylinder, causing a rich condition, soot on the plug, and a long crank or hydro-lock risk), and electrical failure (open or short, a hard misfire). The disciplined diagnosis uses the scan tool's cylinder power balance and injector balance tests (which kill each injector in turn and measure the RPM drop — a weak or coked injector shows a smaller drop), a fuel rail leak-down test (the rail pressure should hold after shutdown; a rapid drop indicates a leaking injector), and a borescope inspection of each cylinder (a clean, washed piston top indicates a leaking injector; a sooty plug indicates a rich or leaking injector). The tradeoff is that injectors are expensive and labor-intensive to replace, but condemning one without balance and leak-down verification wastes the replacement.

### Evaluate Fuel Trims Across Load to Localize Lean and Rich Faults

Fuel trims (short-term and long-term) reveal the ECM's correction for deviations from the commanded mixture, and on GDI engines they localize lean and rich faults by load. A positive trim (adding fuel) at idle that improves at higher load points to a vacuum leak or an unmetered air source; a positive trim that worsens with load points to a fuel delivery problem (weak HPFP, coked injectors, low in-tank pump pressure); a negative trim (subtracting fuel) points to a leaking injector or a rich condition. The disciplined technician reads the trims at idle, at cruise, and under load, and uses the pattern to localize the fault before condemning parts. The tradeoff is that trim analysis takes a road test with a scan tool, but it localizes the fault and prevents random parts replacement.

### Use the Correct Oil to Mitigate LSPI and Carbon on GDI Engines

GDI engines, especially turbocharged ones, are prone to LSPI (low-speed pre-ignition), an abnormal combustion event that can crack pistons, and to accelerated intake-valve carbon from oil consumed through the PCV. The disciplined service uses the OEM-specified oil (often a low-SAPS, LSPI-mitigating formulation with the correct API SP or OEM approval), maintains the PCV system (a stuck-open PCV dumps excess oil vapor into the intake and accelerates carbon), and recommends oil intervals appropriate to the operating conditions. The tradeoff is that the correct oil is more expensive, but the wrong oil causes LSPI and piston damage and accelerates carbon.

## Common Traps

### Decarbonizing Valves for an HPFP Fault — The GDI engine has a rough idle and lean code, the technician assumes carbon, decarbons the valves, and the symptom persists because the HPFP cannot maintain rail pressure. The trap mechanism is that carbon and HPFP failure produce overlapping symptoms (rough idle, lean), and the carbon is visible on every GDI engine, so it is an easy (wrong) target. The false signal is the visible carbon; the harm is a decarbon service that did not address the HPFP. The disciplined technician reads the rail pressure commanded versus actual before decarbonizing.

### Condemning the HPFP Without Checking the Low-Pressure Supply — The rail pressure is low, the technician replaces the HPFP, and the pressure is still low because the in-tank low-pressure pump is weak and starving the HPFP. The trap mechanism is that the HPFP depends on the low-pressure supply, and a weak in-tank pump mimics HPFP failure. The false signal is the low rail pressure pointing at the HPFP; the harm is an expensive HPFP replaced for a cheap in-tank pump. The disciplined technician verifies the low-pressure supply to the HPFP first.

### Replacing Injectors for Carbon-Induced Misfire — The GDI engine misfires, the technician condemns the injectors, and the misfire persists because the cause is intake-valve carbon disrupting airflow. The trap mechanism is that carbon causes misfire that looks like an injector fault, and the injectors are the expensive, easy-to-name part. The false signal is the misfire code; the harm is injector replacement that did not address the carbon. The disciplined technician borescopes the valves and runs a balance test before the injectors.

### Ignoring a Leaking Injector and Hydro-Locking the Engine — A GDI injector leaks fuel into the cylinder after shutdown, the engine cranks long and starts rough, and the technician dismisses it, until the cylinder fills enough to hydro-lock and bend a rod on the next start. The trap mechanism is that a leaking direct injector fills the cylinder with liquid fuel, which is incompressible; cranking a filled cylinder bends the connecting rod. The false signal is the long crank feeling like a fuel-pressure issue; the harm is a bent rod and engine damage. The disciplined technician performs a rail leak-down test and borescopes for a washed piston on any long-crank GDI engine.

### Using Conventional Oil on a Turbo GDI and Causing LSPI — The technician uses a conventional or wrong-specification oil, and the turbo GDI engine suffers LSPI events that crack a piston. The trap mechanism is that LSPI is tied to oil formulation (calcium and other additives promote it), and the wrong oil increases LSPI frequency on turbo GDI engines. The false signal is "oil is oil"; the harm is piston damage from pre-ignition. The disciplined technician uses the OEM-specified LSPI-mitigating oil.

## Self-Check

- For a GDI rough idle or misfire, did I borescope the intake valves to evaluate carbon severity and confirm it matches the symptom before decarbonizing?
- Did I read the commanded versus actual high-pressure rail pressure across load to evaluate the HPFP before condemning it?
- Did I verify the low-pressure fuel supply (in-tank pump) to the HPFP before replacing the HPFP?
- Did I run a cylinder power balance and injector balance test and a rail leak-down test before condemning injectors?
- Did I borescope each cylinder for a washed piston (leaking injector) or sooty plug (rich) before replacing parts?
- Did I read fuel trims at idle, cruise, and load, and use the pattern to localize lean (vacuum leak, fuel delivery) and rich (leaking injector) faults?
- For a long-crank GDI engine, did I perform a leak-down test to rule out a hydro-lock risk from a leaking injector?
- Did I use the OEM-specified LSPI-mitigating oil and check the PCV system for excess oil consumption that accelerates carbon?
- Did I road-test after the repair and verify no misfire, stable fuel trims, and correct rail pressure under load?
