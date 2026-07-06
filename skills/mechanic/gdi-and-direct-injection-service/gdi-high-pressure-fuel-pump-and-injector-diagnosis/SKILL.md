---
name: gdi-high-pressure-fuel-pump-and-injector-diagnosis.md
description: Use when the agent is diagnosing gasoline direct injection high-pressure fuel pump faults, fuel pressure codes, GDI injector leaks or coking, low or high fuel rail pressure, hard start or long crank on a GDI engine, or deciding whether a fuel pressure fault is the high-pressure pump, the pressure sensor, the injectors, or the low-pressure supply.
---

# GDI High-Pressure Fuel Pump and Injector Diagnosis

A gasoline direct injection (GDI) fuel system operates at two pressure levels: a low-pressure supply from the in-tank pump (typically 50-80 psi) feeds a mechanically driven high-pressure fuel pump (HPFP) that raises the pressure to the rail (typically 500-3000 psi, varying with load), and the system's complexity creates a range of faults that are easily misattributed. The judgment problem is that a fuel pressure code, a long crank, a rough idle, or a misfire can be the HPFP, the pressure sensor, a leaking injector, the low-pressure supply, or the fuel pressure control — and the components are expensive and labor-intensive to replace. A technician who condemns the HPFP for a leaking injector, or who replaces the injectors for a low-pressure supply fault, hands back a vehicle with the same symptom. This skill covers the disciplined isolation of GDI fuel system faults across the low-pressure supply, the HPFP, the pressure sensor, and the injectors.

## Core Rules

### Distinguish Low-Pressure Supply Faults From High-Pressure System Faults

The disciplined GDI fuel diagnosis starts by separating the low-pressure supply from the high-pressure system, because the two are diagnosed differently and a low-pressure fault mimics an HPFP fault. The disciplined technician reads the low-pressure fuel pressure (with a mechanical gauge at the low-pressure test port, or the scan tool's low-pressure PID where available) and the high-pressure rail pressure (the scan tool's fuel rail pressure PID), and compares both to the commanded and specified values. A low-pressure supply that is below spec starves the HPFP and produces a low rail pressure, and the fault is in the tank pump, the filter, or the supply line, not the HPFP. A low-pressure supply that is in spec with a low rail pressure points to the HPFP or the pressure control. The tradeoff is that the low-pressure check requires a mechanical gauge (most GDI systems have a low-pressure test port), but condemning the HPFP for a tank pump fault is a frequent error.

### Read the Commanded Versus Actual Rail Pressure to Isolate the HPFP and the Sensor

The disciplined diagnosis reads the commanded rail pressure and the actual rail pressure from the scan tool, and watches them under different conditions (idle, acceleration, deceleration). An actual rail pressure that lags the commanded pressure under load (the pump cannot keep up) points to a weak HPFP. An actual rail pressure that disagrees with the commanded pressure at idle but tracks under load may point to a biased fuel rail pressure sensor (FRPS). An actual rail pressure that is correct at idle but drops on acceleration points to a pump that cannot deliver volume under demand. The disciplined technician uses the rail pressure data to narrow the fault before condemning a component. The tradeoff is that the data reading requires a road test and familiarity with the PID, but it is the difference between the right and the wrong expensive part.

### Test the Fuel Rail Pressure Sensor Independently Before Condemning the Pump

The fuel rail pressure sensor (FRPS) reports the rail pressure to the PCM, and its failure (a biased reading, a drift with temperature, a complete failure) produces fuel pressure codes that mimic an HPFP fault. The disciplined diagnosis tests the FRPS by comparing its reading to a known mechanical pressure (where a high-pressure mechanical gauge can be connected, which is rare on GDI but possible on some systems), by checking its voltage signal against the OEM spec at key-on (the rail pressure should read atmospheric or a baseline), and by watching for a reading that does not change with the commanded pressure or that drifts with temperature. A biased FRPS that reads high causes the PCM to under-command the pump and produces a lean condition; a biased FRPS that reads low causes an over-pressure. The tradeoff is that the FRPS test is indirect on most GDI systems, but replacing the HPFP for a bad sensor is a frequent and expensive error.

### Evaluate GDI Injectors for Leaks, Coking, and Electrical Faults

GDI injectors operate at high pressure and high temperature, and their failure modes include external leaks (a leaking injector dumps fuel into the cylinder, causing a long crank, a rough idle, a fuel smell, and a dilution of the oil), internal leaks or poor atomization (a coked or worn injector sprays poorly, causing a misfire and a lean code), and electrical faults (an open or shorted coil, causing a misfire). The disciplined diagnosis performs a relative compression and an injector balance test (where the scan tool commands each injector and watches the RPM or the rail pressure drop), checks the fuel trims (a rich trim at idle that leans out under load suggests a leaking injector), and inspects the spark plugs (a fuel-fouled plug in one cylinder suggests a leaking injector in that cylinder). The tradeoff is that the injector diagnosis takes multiple tests, but replacing the HPFP for a leaking injector misses the fault.

### Verify the Repair With Fuel Pressure Data and a Road Test

After the fuel system repair (the HPFP, the FRPS, the injectors, or the low-pressure supply), the disciplined technician verifies the repair by reading the commanded and actual rail pressure under the conditions that triggered the fault, confirming the pressures track and are within spec, clearing the codes, and road-testing to confirm the symptom (the long crank, the misfire, the lean code) is resolved. A fuel pressure fault that returns indicates an incomplete repair or a second fault. The tradeoff is that the verification takes a road test and data reading, but a fuel system repair returned without verification risks a comeback.

## Common Traps

### Condemning the HPFP for a Low-Pressure Supply Fault — The rail pressure is low, the HPFP is replaced, and the fault continues because the in-tank pump or the filter is restricted and starving the HPFP. The trap mechanism is that a low-pressure supply produces a low rail pressure, and the supply is not checked. The false signal is the low rail pressure; the harm is a needless HPFP. The disciplined technician checks the low-pressure supply first.

### Replacing the HPFP for a Biased Fuel Rail Pressure Sensor — A fuel pressure code sets, the HPFP is replaced, and the code returns because the FRPS is biased and reporting the wrong pressure. The trap mechanism is that the sensor's error sets the code, and the pump is fine. The false signal is the fuel pressure code; the harm is a needless HPFP. The disciplined technician tests the FRPS before the pump.

### Missing a Leaking Injector as the Long Crank or Rough Idle Cause — The vehicle cranks long and runs rough on startup, the HPFP is diagnosed, and the cause is a leaking injector dumping fuel into a cylinder while parked. The trap mechanism is that a leaking injector floods a cylinder, and the fuel system pressure is fine. The false signal is the long crank resembling a fuel pressure fault; the harm is a needless HPFP and a continued leak that dilutes the oil. The disciplined technician checks the fuel trims, the plugs, and performs an injector balance test.

### Ignoring Oil Dilution From a Leaking Injector — A leaking injector is replaced, but the oil diluted by the leak is not changed, and the engine is at risk of a runaway or bearing damage. The trap mechanism is that the leaked fuel dilutes the oil, and the diluted oil is not changed. The false signal is the injector being fixed; the harm is engine damage from fuel-diluted oil. The disciplined technician checks and changes the oil after an injector leak.

### Condemning Multiple Injectors for a Single-Cylinder Misfire — One cylinder misfires, all the injectors are replaced, and the cause was one coked or failed injector. The trap mechanism is that the misfire is cylinder-specific, and the failed injector is not isolated. The false signal is the misfire on a GDI engine; the harm is needless injectors. The disciplined technician isolates the misfiring cylinder and tests its injector.

## Self-Check

- Did I check the low-pressure supply (tank pump, filter, supply line) before condemning the HPFP?
- Did I read the commanded versus actual rail pressure under idle, acceleration, and deceleration?
- Did I test the fuel rail pressure sensor for bias, drift, and the correct baseline reading before condemning the pump?
- Did I perform an injector balance test, check the fuel trims, and inspect the spark plugs for a leaking or coked injector?
- After an injector leak, did I check and change the oil diluted by the leaked fuel?
- Did I isolate a misfire to a specific cylinder and test its injector before condemning multiple injectors?
- After the repair, did I verify the commanded and actual rail pressure track and are within spec under the original conditions?
- Did I road-test and confirm the original symptom (long crank, misfire, lean code, fuel pressure code) is resolved?
