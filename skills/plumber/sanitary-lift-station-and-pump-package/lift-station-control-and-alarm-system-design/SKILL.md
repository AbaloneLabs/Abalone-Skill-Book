---
name: lift-station-control-and-alarm-system-design.md
description: Use when the agent is designing a sanitary lift station control and alarm system, configuring lead-lag-standby pump alternation, specifying high-water alarm (audible visual and auto-dialer), providing redundant floats, selecting control panel NEMA rating and location above flood level, integrating SCADA or BMS, providing manual override, or planning power backup (generator or inverter) to prevent undetected overflow and single-point failure.
---

# Lift Station Control and Alarm System Design

A lift station's pumps move the sewage, but the control and alarm system determines whether anyone knows when a pump fails — and in a below-grade or critical station, an undetected failure means a sewage backup into the occupied space within minutes. The judgment problem is that the control system must alternate the pumps to equalize wear, start the lag pump when the lead cannot keep up, alarm before the basin overflows, survive a flooded panel, and keep running through a power outage, and each of these is a separate failure point that a "wire it up and walk away" approach leaves unaddressed. An agent who hard-wires a single pump with no alternation, who mounts the panel at basin rim level, who omits the high-water dialer, or who provides no backup power will create a station that works until the first pump failure or outage, then floods the building. This skill covers the control, alarm, panel, integration, and backup-power decisions that make a lift station survivable — and the role limits that place large-station electrical work with a licensed electrician and engineer.

## Core Rules

### Configure Lead-Lag-Standby Alternation to Equalize Wear and Provide Capacity

In a multi-pump station, the control logic must alternate the lead pump each cycle so that all pumps wear equally (a pump that never runs seizes up, and a pump that runs every cycle wears out first), start the lag pump when the lead cannot keep up (the level continues to rise after the lead starts), and — for a triplex station — start the standby pump at a still-higher level. The alternation is handled by the control panel (a PLC or an alternating relay for simpler stations), and the lead/lag/standby assignments rotate on each cycle or on a time basis. The capacity benefit is that two or three pumps running together can meet a peak inflow that one pump cannot, buying time before the high-water alarm. The trap is hard-wiring one pump as permanent lead (it wears out while the others sit idle and seize) or omitting the lag-start logic (the station overflows if the lead cannot keep up, even with a healthy lag pump available). Configure alternation, lag-start, and standby-start logic, and verify each pump rotates into the lead role.

### Provide a High-Water Alarm That Is Audible, Visible, and Notifies Remotely

Every lift station must have a high-water alarm that triggers before the basin overflows into the fixtures or the floor, and in any unattended or critical station the alarm must notify someone off-site. The minimum is an audible and visible alarm (a horn and a strobe) at the station or in an occupied area; for unattended stations, a remote notification (an auto-dialer, a BMS or SCADA point, or a text/email alert) is essential, because a horn in an empty mechanical room warns no one. The high-water level should be set above the lag-pump-start level but below the invert (the basin overflow point), so the alarm gives time to respond before sewage enters the building. The trap is providing only a local horn in an unattended station, or setting the alarm level at the invert (no time to respond); the harm is an undetected overflow that floods the building. Provide audible/visual and remote notification for any unattended or critical station, set the alarm level with response time, and test the notification path.

### Use Redundant Level Sensing So a Single Float Failure Cannot Cause an Overflow

The level control — whether floats or a transducer — is a single point of failure if it is the only input, and a failed float (tangled cable, worn contact, fatigued tether) can either never start the pump (overflow) or never stop it (dry run, burnout). For any critical station, provide redundant level sensing: a primary system (floats or transducer) for normal control, plus an independent high-water alarm float set above the normal pump-off level. The high-water alarm float must be on a separate circuit from the control floats, so that a control-circuit failure does not disable the alarm. The trap is relying on a single float chain for both control and alarm, so that the same failure that stops the pump also silences the alarm; the harm is an undetected overflow. Provide independent control and alarm level inputs, test the high-water alarm float on schedule, and replace floats proactively (they are wearing parts).

### Mount the Control Panel in the Right NEMA Rating and Above the Flood Level

The control panel contains the motor starters, the alternation logic, and the alarm circuitry, and its location and enclosure rating determine whether it survives the conditions a lift station creates — moisture, corrosive sewer gas, and the possibility of a flood. The panel must be in a NEMA-rated enclosure appropriate to the environment (NEMA 4/4X for wet or washdown areas, NEMA 7 for hazardous locations where sewer gas may be present), and it must be mounted above the maximum flood level (the level the water would reach if the basin overflowed and the room flooded), so that a flood does not short the controls and disable the station. The panel should be accessible for service without entering the wet well or a confined space. The trap is mounting the panel at basin rim level or in an unsealed enclosure, so that a flood or sewer-gas corrosion takes out the controls exactly when they are needed; the harm is a station disabled by the very overflow it should have prevented. Mount the panel above the flood level in an appropriate NEMA enclosure, accessible for service.

### Integrate With SCADA or BMS, Provide Manual Override, and Plan Power Backup

For commercial and critical stations, the control system should integrate with the building management system (BMS) or a SCADA system to report station status (pump run, alarm, level) to a monitored location, and it must include a manual override (a hand-off-auto switch on each pump) so that a control failure can be bypassed and the pump run by hand to clear the basin. Power backup is essential for any station that serves occupied space: a generator, an inverter, or a dual-power-source arrangement so that a utility outage does not stop the pumps and overflow the basin. The backup must be sized to the pump's starting load (the inrush), not just the running load. The trap is omitting the manual override (a control failure strands the station) or the backup power (an outage overflows the basin); the harm is a station that fails on a control fault or a power outage. Integrate with BMS/SCADA where appropriate, provide a manual override on each pump, and provide backup power sized to the pump's starting load for any critical station.

## Common Traps

### Hard-Wiring One Pump as Permanent Lead

The agent wires one pump as the permanent lead and the second as permanent lag, with no alternation. The trap mechanism is that the lead pump runs every cycle and wears out first, while the lag pump sits idle and seizes (the bearings freeze, the impeller corrodes in place), so that when the lead finally fails, the lag — called upon for the first time in years — does not start, and the station overflows. The false signal is that "the station has two pumps." The harm is a total station failure when the lead pump dies. The defense is to configure alternation so each pump rotates into the lead role, and to test the lag pump's start on schedule.

### Providing Only a Local Horn in an Unattended Station

The agent installs a high-water horn and strobe in the mechanical room of an unattended building and no remote notification. The trap mechanism is that a horn in an empty room warns no one, and the overflow proceeds undetected until sewage appears in the occupied space or the next inspection. The false signal is that "there is an alarm." The harm is an undetected overflow that floods the building. The defense is to provide remote notification (auto-dialer, BMS/SCADA point, or text/email alert) for any unattended or critical station, and to test the notification path.

### Relying on a Single Float Chain for Control and Alarm

The agent uses a single set of floats for pump-on, pump-off, and high-water alarm, all on one circuit. The trap mechanism is that the same float-chain failure that stops the pump (a tangled cable, a worn contact) also disables the alarm, so the overflow is both caused and concealed by the failure. The false signal is that "the floats control the station." The harm is an undetected overflow from a single failure. The defense is to provide an independent high-water alarm float on a separate circuit, test it on schedule, and replace floats proactively.

### Mounting the Panel at Basin Rim Level or in an Unsealed Enclosure

The agent mounts the control panel at the basin rim or on the wall at a low elevation, in a standard enclosure, to minimize conduit runs. The trap mechanism is that a basin overflow floods the room and the panel together, shorting the controls and disabling the station exactly when the alarm should be sounding, and sewer-gas corrosion attacks an unsealed enclosure over time. The false signal is that "the panel is installed." The harm is a station disabled by the overflow it should have caught. The defense is to mount the panel above the maximum flood level in an appropriate NEMA enclosure (4/4X for wet areas, 7 for hazardous locations), accessible for service.

### Omitting Manual Override or Backup Power in a Critical Station

The agent provides no hand-off-auto switch and no backup power for a station serving occupied space. The trap mechanism is that a control-circuit failure strands the station (no way to run the pump by hand), and a utility outage stops the pumps and overflows the basin, because the station has no manual override or backup. The false signal is that "the station runs automatically." The harm is a station failure on a control fault or a power outage, overflowing into occupied space. The defense is to provide a manual override (HOA switch) on each pump and backup power (generator or inverter) sized to the pump's starting load for any critical station.

## Self-Check

- Did I configure lead-lag-standby alternation so each pump rotates into the lead role, with lag-start and standby-start logic, and did I verify each pump takes its turn as lead?
- Does the high-water alarm trigger above the lag-start level but below the invert, with audible/visual indication and remote notification (auto-dialer, BMS/SCADA, or text/email) for any unattended or critical station?
- Is the high-water alarm on an independent float and a separate circuit from the control floats, so a control failure cannot silence the alarm, and is the alarm float tested on schedule?
- Is the control panel mounted above the maximum flood level in a NEMA-rated enclosure appropriate to the environment (4/4X for wet areas, 7 for hazardous locations), and is it accessible for service without confined-space entry?
- Does the station integrate with BMS or SCADA to report pump run, alarm, and level to a monitored location, for commercial or critical stations?
- Is there a manual override (hand-off-auto switch) on each pump so a control failure can be bypassed and the pump run by hand to clear the basin?
- Is backup power (generator, inverter, or dual source) provided and sized to the pump's starting load (inrush), not just the running load, for any station serving occupied space?
- Did I confirm my licensing scope covers the control and electrical work (plumber for small station controls, licensed electrician and engineer for large or engineered stations), and did I escalate beyond-scope work?
- Are the alternation logic, alarm setpoints and notification path, panel location and NEMA rating, BMS/SCADA integration, manual override, and backup power documented for inspection and maintenance?
