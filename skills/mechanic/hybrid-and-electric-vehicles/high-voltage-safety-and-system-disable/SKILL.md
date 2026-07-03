---
name: high-voltage-safety-and-system-disable.md
description: Use when the agent is servicing hybrid or electric vehicles, performing high-voltage battery disconnect, evaluating personal protective equipment, verifying zero-voltage state, or managing arc flash and stored energy risks during traction battery and inverter service.
---

# High-Voltage Safety and System Disable

High-voltage (HV) systems on hybrid and electric vehicles present lethal hazards that do not exist on conventional vehicles, and the consequences of a procedural error are not a burned finger or a blown fuse—they are cardiac arrest from electrocution, arc-flash burns, or a lithium battery thermal runaway fire. The judgment problem is that the HV system appears inert when the vehicle is off, the contactors are open, and the key is removed, yet the battery pack retains hundreds of volts and the capacitors in the inverter retain potentially lethal charge for minutes after shutdown. The technician must approach every HV vehicle with the assumption that lethal voltage is present until proven otherwise, must follow the manufacturer's disable procedure exactly, and must verify a zero-energy state with calibrated equipment before touching any orange-cabled component. There is no room for improvisation, shortcut, or "it's probably fine."

## Core Rules

### Understanding the Hazard and Voltage Levels

Hybrid and electric vehicle traction systems operate at voltages far above the 12-volt system: full hybrids typically 200-300 volts DC, plug-in hybrids 300-400 volts, and battery electric vehicles 350-800 volts DC, with three-phase AC between the inverter and motor at similar peak voltages. These voltages can drive lethal current through the human body across the skin-to-skin path (hand to hand, hand to foot), causing muscle contraction, respiratory arrest, and ventricular fibrillation. The threshold for danger is roughly 50 volts DC and 30 volts AC; above these levels, the system must be treated as lethal. The HV system is isolated from the vehicle chassis by design (it is a floating system, not grounded to the body), which is why a single contact with one HV pole does not produce a shock—but contact with both poles, or with one pole and a compromised isolation, does. Understanding this architecture is essential because it explains why the disable procedure involves breaking the circuit at the battery, not just disconnecting the 12-volt system.

### Personal Protective Equipment Requirements

HV work requires specific personal protective equipment (PPE) rated for the voltages present, and using the wrong PPE or none at all is a life-safety violation. Class 0 rubber insulating gloves, rated for 1000 volts, are the minimum for most HV work; they must be visually inspected for cuts and pinholes before each use, air-tested for leaks, and electrically tested by a certified lab at the interval specified by the manufacturer (typically every 6 months). Leather protectors are worn over the rubber gloves to prevent mechanical damage. Insulated hand tools, rated for 1000 volts and marked with the voltage rating, are required for any work near live HV components, because a dropped wrench across two bus bars produces an arc flash that can blind and burn. A face shield and arc-rated clothing are required for tasks with arc-flash risk, such as opening a battery pack or working near energized bus bars. Always verify the gloves are within their test date and undamaged before each use; an out-of-date or pinholed glove provides no protection.

### The System Disable Procedure

The HV disable procedure is the single most important sequence in HV service, and it must follow the manufacturer's documented steps without deviation. The general sequence is: turn the ignition off and remove the key (or ensure the vehicle is powered down), disconnect the 12-volt battery (which powers the contactors), remove the HV service disconnect (a physical plug or switch on the battery pack that breaks the HV circuit), and wait the manufacturer-specified time (typically 5-15 minutes) for the inverter capacitors to discharge. The service disconnect must be removed completely and kept in the technician's possession or locked out to prevent another person from reinstalling it. After the wait, verify zero voltage at the inverter terminals or the service disconnect terminals with a calibrated CAT III or CAT IV multimeter rated for at least 600 volts (1000 volts preferred). Never assume the system is discharged because the time has elapsed—always measure. Some vehicles have a residual voltage that requires an additional active discharge via scan tool; follow the procedure exactly. Apply lockout/tagout to the service disconnect and the 12-volt battery so the vehicle cannot be re-energized while service is in progress.

### Verifying Zero Voltage and Isolation

Verification of zero energy is a two-step process: confirm the DC bus voltage is zero, and confirm the HV system is isolated from the chassis. Measure the DC bus voltage at the inverter input or the service disconnect contacts with a calibrated high-voltage meter; the reading should be below 30 volts DC (ideally below 10 volts) before any HV component is touched. Then check isolation (if the procedure requires) by measuring between each HV pole and the chassis ground; a high reading (typically above 500 ohms per volt of system voltage, or per the manufacturer's spec) confirms the system is floating and safe. A low isolation reading indicates a fault (a component is leaking HV to the chassis) and the vehicle must not be driven or serviced further until the fault is found. Always use meter leads with the correct rating (CAT III or IV, 600V or 1000V) and with finger guards; standard low-voltage leads can arc over and fail at HV levels. Document the zero-voltage verification in the repair order, including the meter reading and the time of the disable, because this is the legal record that the procedure was followed.

### Lithium Battery Thermal Runaway and Fire

Lithium-ion traction batteries can enter thermal runaway—a self-sustaining exothermic reaction—from physical damage, overcharging, internal short, or manufacturing defect, and once runaway begins it is extremely difficult to stop. A cell in runaway releases flammable electrolyte vapor, ignites, and heats adjacent cells, propagating the runaway through the pack. The signs are smoke with a sweet chemical odor, heat from the pack, hissing or popping from venting cells, and visible flame. If thermal runaway is suspected, evacuate the area, call the fire department, and inform them it is a lithium battery fire (they have specific protocols and may use large volumes of water or let the pack burn controlled). A conventional fire extinguisher will not stop lithium thermal runaway. Do not attempt to move a burning vehicle. After a thermal event, the pack is a total loss and the vehicle may be a total loss; the scene must be treated as a hazardous materials incident because the vapor and residue are toxic. Prevent runaway by never piercing or dropping a battery pack, never charging a damaged pack, and following the manufacturer's handling procedures exactly.

### Specific Service Precautions

Several service situations require heightened HV awareness beyond the standard disable. When removing a battery pack from the vehicle, support it properly (packs weigh 300-1000+ pounds), never allow the pack to tip or impact, and use a lift table or transmission jack rated for the weight. When servicing the inverter or motor, verify zero voltage at the component, not just at the service disconnect, because capacitors or backfeed from the motor can retain charge. When working on the cooling system for the HV battery or inverter, be aware that the coolant is in contact with HV components and may be conductive if contaminated; never open a coolant line while the system is energized. When jump-starting, never use the HV battery to jump another vehicle and never apply external voltage to the HV system; only the 12-volt system can be jump-started, and only per the manufacturer's procedure. When towing, follow the manufacturer's towing procedure (typically flatbed only) because towing with drive wheels on the ground can energize the system through regenerative braking.

## Common Traps

### Trusting the System Is Off Without Verifying Zero Voltage

The most lethal trap is assuming the HV system is de-energized because the key is off and the service disconnect is removed, without measuring the voltage. The mechanism is that the inverter capacitors store charge for minutes after the contactors open, and on some vehicles a fault or a slow discharge leaves lethal voltage on the bus well beyond the specified wait time. The false signal is the vehicle appearing completely off, with no warning lights and no fan operation. The harm is electrocution when the technician contacts a bus bar or an orange cable, with risk of cardiac arrest and death. Always measure the DC bus voltage with a calibrated HV-rated meter after the specified wait, and never touch an HV component until the reading is confirmed below 30 volts.

### Using Damaged or Out-of-Date Insulating Gloves

A second trap is using Class 0 insulating gloves that are pinholed, cut, or past their electrical test date. The mechanism is that rubber degrades over time and from ozone, oil, and mechanical stress, and a pinhole that is invisible to the eye allows HV to pass through to the hand during a contact event. The false signal is that the gloves look intact and feel fine. The harm is that during an accidental HV contact, current flows through the pinhole to the technician's hand, defeating the glove's purpose and causing electrocution or severe burn. Always air-test the gloves before each use (roll them up to inflate and check for leaks), visually inspect for damage, verify the test date is current, and replace immediately if any defect is found.

### Removing the Service Disconnect Without Disconnecting the 12-Volt Battery

A third trap is removing the HV service disconnect without first disconnecting the 12-volt battery. The mechanism is that the 12-volt system powers the contactors that close the HV circuit, and on some vehicles removing the service disconnect under load (with contactors closed) can draw an arc at the disconnect contacts, causing burns or contact welding. The false signal is that the disconnect is designed to be removed, so it seems safe to pull. The harm is an arc flash at the disconnect that burns the technician's hand or face, or welds the contacts so the disconnect cannot be reinstalled. Always disconnect the 12-volt battery first, wait for the contactors to open audibly or per the procedure, then remove the service disconnect.

### Using the Wrong Multimeter or Leads

A fourth trap is using a standard low-voltage multimeter or standard leads to measure HV circuits. The mechanism is that meters and leads have category ratings (CAT II, III, IV) and voltage ratings that indicate their ability to withstand transients, and a CAT II meter or unrated leads can arc over internally or at the probe when connected to a 400-volt DC bus, causing a short through the meter and an arc flash. The false signal is that the meter reads voltage and appears to function. The harm is meter failure, arc flash, and injury to the technician from the arc and from the meter's fragmentation. Always use a CAT III or CAT IV meter rated for at least 600 volts (1000 preferred) with matching rated leads and finger-guarded probes for any HV measurement.

### Improper Towing or Jump-Starting

A fifth trap is towing or jump-starting an HV vehicle using conventional methods. The mechanism is that towing with the drive wheels on the ground spins the motor, which acts as a generator and energizes the HV system through the inverter, potentially charging the DC bus to lethal levels even with the vehicle off; and jump-starting by applying external voltage to the wrong terminals can damage the HV control system or energize the bus unexpectedly. The false signal is that the vehicle "should be towable" like a conventional vehicle. The harm is energizing the HV system during tow, creating a shock hazard for the tow operator and anyone handling the vehicle, and damaging the inverter or battery. Always use a flatbed tow truck, follow the manufacturer's towing procedure, and jump-start only the 12-volt system at the designated terminals.

## Self-Check

- Did I confirm the vehicle is an HV system and identify the system voltage (200-800V) before beginning service?
- Did I wear Class 0 insulating gloves within their test date, air-tested and visually inspected, with leather protectors?
- Did I use insulated, HV-rated hand tools and a CAT III or IV meter with rated leads for all HV work?
- Did I turn the vehicle off, disconnect the 12-volt battery, remove the service disconnect, and wait the full specified time?
- Did I verify zero voltage (below 30V) at the DC bus with a calibrated HV meter before touching any HV component?
- Did I check isolation between each HV pole and chassis if the procedure requires it?
- Did I apply lockout/tagout to the service disconnect and 12-volt battery to prevent re-energization?
- Did I follow the manufacturer's towing procedure (typically flatbed) and avoid towing with drive wheels down?
- Did I jump-start only the 12-volt system at the designated terminals and never the HV system?
- Did I know the thermal runaway signs (smoke, heat, hissing) and the emergency response (evacuate, call fire department, identify as lithium fire)?
