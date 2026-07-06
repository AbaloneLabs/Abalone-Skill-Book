---
name: surge-protection-device-selection-and-installation.md
description: Use when the agent is selecting and installing surge protective devices at service entrances and branch panels, coordinating Type 1, Type 2, and Type 3 devices, evaluating SCCR, MCOV, VPR, and kA ratings per NEC Article 285, and building a cascaded protection strategy for sensitive loads.
---

# Surge Protective Device Selection and Installation

A surge protective device (SPD) absorbs or diverts transient overvoltages — from lightning, utility switching, and large load switching inside the facility — before they reach and destroy sensitive equipment. The judgment problem is that "surge protection" is sold as a single commodity, when in fact a real installation is a coordinated cascade of devices of different types, each placed at a specific point in the system, each rated by specific electrical parameters that determine whether it survives and whether it protects. An SPD chosen by price or by the presence of indicator lights, installed at the wrong point, or wired with long leads can provide essentially no protection while giving complete false confidence. Agents miss the issues because the device bolts on and shows a green light, while the parameters that determine real protection — SCCR, MCOV, VPR, kA rating, lead length — are never examined.

## Core Rules

### Apply NEC Article 285 to Select and Locate Type 1, Type 2, and Type 3 Devices

NEC Article 285 defines three SPD types by where they may be installed. Type 1 is installed on the line side of the service disconnect, ahead of the main overcurrent device, suitable for the harshest external surges. Type 2 is installed on the load side of the service disconnect, at panelboards and feeders, the workhorse of facility protection. Type 3 is point-of-utilization, at or near the sensitive equipment, the final stage of a cascade. The defense is to match the type to the installation location per 285, to use a Type 1 or Type 2 at the service entrance to clamp external surges, and to add Type 3 at the most sensitive loads, building a cascade that reduces the surge at each stage.

### Verify the SCCR Equals or Exceeds the Available Fault Current

The short-circuit current rating (SCCR) of an SPD is the maximum fault current the device can safely withstand at its point of installation. If the available fault current at the panel exceeds the SPD's SCCR, the SPD can rupture explosively during a fault, creating a hazard rather than preventing one. The defense is to determine the available fault current at every point where an SPD will be installed — from a fault current study or the utility — and to select an SPD whose SCCR equals or exceeds that value, because an underrated SCCR turns the protector into a projectile during a fault.

### Confirm the MCOV Exceeds the System's Maximum Normal Voltage

The maximum continuous operating voltage (MCOV) is the highest steady-state voltage at which the SPD can operate without conducting or degrading. An SPD whose MCOV is below the system's normal voltage (including tolerances and overvoltage conditions) will conduct continuously, overheat, and fail. The defense is to select an SPD whose MCOV exceeds the maximum normal voltage of the system it protects — including the upper end of utility voltage tolerance — so that the device sits idle during normal operation and only conducts during a genuine transient.

### Use VPR and kA Rating Together to Judge Real Protection Quality

The voltage protection rating (VPR) is the let-through voltage the SPD permits during a standard surge — lower is better, because less voltage reaches the load. The kA rating (nominal discharge current, In, and maximum discharge current, Imax) is the surge current the device can handle — higher is better, because the device survives larger events. Neither number alone tells the whole story: a low VPR with a tiny kA rating clamps well but dies in the first real surge; a huge kA rating with a high VPR survives but lets damaging voltage through. The defense is to evaluate both together, preferring devices with a low VPR backed by a substantial kA rating, and to be suspicious of any device marketed on a single impressive number.

### Keep SPD Lead Lengths Short to Minimize Let-Through Voltage

The voltage the load actually sees during a surge is the SPD's clamp voltage plus the voltage developed across the leads connecting the SPD to the bus, and at surge-current magnitudes even short leads add significant voltage (the inductive drop equals L times di/dt). Long SPD leads can double or triple the let-through voltage, defeating the device's low VPR. The defense is to mount the SPD as close as possible to the protected bus, to keep the connecting leads as short and straight as possible — ideally a foot or less — and never to coil excess lead, because the lead inductance, not the SPD rating, often determines what the load actually experiences.

### Build a Cascaded Strategy Rather Than Relying on One Device

A single SPD at the service entrance clamps the largest external surges but leaves the load exposed to internally generated surges and to the residual let-through. A cascade — a robust Type 1 or Type 2 at the entrance, a Type 2 at the branch panel, and a Type 3 at the sensitive load — reduces the surge progressively, with each stage handling a smaller residual. The defense is to design protection as a cascade, with higher-kA devices upstream and lower-VPR devices downstream near the protected equipment, and to recognize that one big device at the entrance protects the panelboard but not necessarily the server on the far side of the building.

## Common Traps

### Choosing an SPD by Price or Indicator Light Alone

The installer buys the cheapest SPD with a green indicator light and bolts it on. The false signal is that the light is on, so the device "works." The mechanism of failure is that the device's SCCR, MCOV, VPR, and kA rating are unknown and may be wholly inadequate for the installation, so the device either fails during a fault, conducts continuously, or lets damaging voltage through. The harm is false confidence and equipment destroyed by a surge the SPD was believed to stop.

### Installing a Type 2 Where a Type 1 Is Required (or Vice Versa)

The installer puts a Type 2 device on the line side of the main disconnect, or a Type 1 deep in the branch system, ignoring the type's intended location. The false signal is that the device fits the enclosure. The mechanism of failure is that a Type 2 on the line side of the main is not rated for that exposure and can fail during a large external surge, while a Type 1 deep in the system is overkill upstream and leaves downstream loads unprotected. The harm is a misapplied device that either fails or provides no cascade benefit.

### Ignoring SCCR Against the Available Fault Current

The installer mounts an SPD with a modest SCCR on a service with very high available fault current. The false signal is that the device is listed and installed correctly. The mechanism of failure is that during a downstream fault the available current exceeds the SPD's SCCR and the device ruptures. The harm is an exploding SPD during a fault, creating a fire and shrapnel hazard exactly where protection was intended.

### Selecting an MCOV Below the System's Normal Voltage

The installer picks an SPD with an MCOV just above nominal voltage, not accounting for utility tolerance. The false signal is that the MCOV "exceeds" the nominal voltage. The mechanism of failure is that normal utility overvoltage pushes the system above the MCOV, the SPD conducts continuously, overheats, and fails. The harm is a device that burns itself out during normal operation and is dead before the first real surge arrives.

### Mounting the SPD With Long, Coiled Leads

The installer mounts the SPD at the bottom of the panel and coils the excess lead to tidy the installation. The false signal is that the connection is neat and secure. The mechanism of failure is that the long, coiled leads add inductive voltage drop during a surge, so the let-through voltage at the bus is far higher than the SPD's rated VPR. The harm is a device whose real-world protection is a fraction of its rating, leaving the load exposed despite a "good" SPD.

### Relying on One Entrance SPD to Protect Sensitive Distant Loads

The installer puts a large SPD at the service entrance and assumes the whole facility is protected. The false signal is that the entrance is where surges enter. The mechanism of failure is that surges generated inside the facility — from large motor and HVAC switching — never pass through the entrance SPD, and the entrance device's residual let-through travels unattenuated to distant sensitive loads. The harm is destroyed equipment far from the entrance, from surges the single device never saw.

## Self-Check

- Did I apply NEC Article 285 to select the correct SPD type (Type 1, 2, or 3) for each installation location?
- Did I verify the SPD's SCCR equals or exceeds the available fault current at every point of installation?
- Did I confirm the MCOV exceeds the system's maximum normal voltage, including utility tolerance, so the device does not conduct continuously?
- Did I evaluate VPR and kA rating together, preferring a low VPR backed by a substantial kA rating rather than a single impressive number?
- Did I mount each SPD as close to the protected bus as possible with short, straight leads — ideally a foot or less — and no coiled excess?
- Did I design protection as a cascade, with robust upstream devices and low-VPR downstream devices near the most sensitive loads?
- Did I account for internally generated surges, not just external entrance surges, when placing devices?
- Is the SPD installation documented with type, ratings, and location so that maintenance and replacement match the original protection design?
