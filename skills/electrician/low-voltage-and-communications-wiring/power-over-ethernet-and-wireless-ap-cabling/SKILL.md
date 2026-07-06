---
name: power-over-ethernet-and-wireless-ap-cabling.md
description: Use when the agent is deploying wireless access points, calculating PoE power budgets and switch port class limits, planning AP placement and cable home-runs, or diagnosing AP brownouts and port shutdowns from exceeded power budgets.
---

# Power over Ethernet and Wireless AP Cabling

Power over Ethernet (PoE) has turned the structured cabling plant into both the data path and the power path for wireless access points, IP cameras, and powered devices, and the convenience of one cable for both hides a power-budget problem that is easy to miss. The judgment problem is that a PoE switch port is rated to a class with a maximum wattage, the cable itself dissipates power over distance, and a switch has a total power budget across all ports that is often far lower than the sum of the port ratings. An installer who plugs in access points until the switch refuses to power more, or who runs long cables that brown out high-power APs, builds a wireless network that underperforms or drops devices at random. This skill covers the decisions that determine whether a PoE-powered wireless deployment is reliable or chronically unstable.

## Core Rules

### Match the PoE Class and Wattage to Each Device, Not the Cable

PoE comes in several standards: IEEE 802.3af (up to about 15.4 W at the source, 12.95 W delivered), 802.3at (PoE+, about 30 W at the source, 25.5 W delivered), 802.3bt Type 3 and Type 4 (up to 60 W and 100 W at the source). Each powered device (PD), access point, camera, or phone, draws a specific class and wattage, and the switch port and the midspan must be rated to deliver at least that class. The defense is to read the device's PoE class requirement, confirm the switch port supports that class, and never assume that because the connector fits the power will be adequate. A high-power Wi-Fi 6 or 6E access point that needs 802.3bt will not fully function on an 802.3at port and may boot into a degraded radio mode or refuse to enable all radios.

### Calculate the Switch Power Budget Across All Ports

The most common and most overlooked failure is that a 24-port PoE switch is rated to deliver full power on every port simultaneously only if its total power budget equals the sum, and most switches have a total budget well below that sum. A 24-port PoE+ switch with a 370-watt budget can deliver full 30 W to only about 12 ports, not 24. The defense is to sum the actual draw of every powered device, compare it to the switch's total power budget (not the per-port rating), and add ports, switches, or an external power supply where the budget is exceeded. The trap is filling every port and discovering that the switch refuses to power the last several devices, or worse, brownouts when all devices peak together.

### Account for Cable Resistance and Distance Loss

The cable itself dissipates power: the resistance of the copper conductors causes a voltage drop and a wattage loss over distance, so a device at the end of a 90-meter run receives less power than the same device at 10 meters. For low-power devices this is within the standard's delivered-power allowance, but for high-power Type 3 and Type 4 devices on long runs, the loss can push the delivered voltage below the device's operating threshold, causing intermittent reboots or degraded performance. The defense is to keep AP and high-power device runs as short as practical, to use good-quality cable with solid conductors (not CCA), and to verify that long runs to high-power devices have adequate delivered power. The trap is assuming the 90-meter data limit is also the power limit with no loss.

### Place Access Points for Coverage and Capacity, Not for Cable Convenience

Wireless AP placement determines coverage, capacity, and roaming, and the AP should go where the radio coverage demands, with the cable routed to it, not the reverse. The defense is to do a predictive or on-site survey, place APs to overlap coverage with a target received signal and a manageable channel plan, and to home-run a cable to each AP location from the TR. Staggering APs so that no two adjacent APs share a channel, and sizing the AP count for client density (not just area), prevents the all-too-common result of one strong AP and dead zones elsewhere. The trap is mounting APs where the existing conduit runs and accepting the resulting coverage holes.

### Home-Run Every AP and Provide a Service Loop

Each AP should have a dedicated home-run cable back to the switch so that it can be powered, managed, and troubleshot independently, and so that a single cable fault affects only one AP. The defense is to pull a single cable per AP (or two for redundancy on critical APs), label both ends, and leave a service loop in the ceiling so the AP can be repositioned or lowered for service without re-pulling. The trap is daisy-chaining or splitting, which is not supported by PoE for independent devices and which makes a single fault take down multiple APs.

### Use the Correct Cable Category and Avoid CCA for PoE

PoE, especially high-power PoE, depends on the DC resistance of the cable, and copper-clad aluminum (CCA) cable has significantly higher resistance than pure copper, causing excessive voltage drop, heat at the connectors, and in extreme cases, fire risk at the punchdown. The defense is to use only pure-copper, standards-compliant cable (verified by the UL or ETL listing and the cable print legend), to avoid CCA entirely for any PoE run, and to be especially vigilant on high-power Type 3 and Type 4 runs where the current is highest. The trap is buying cheap CCA cable that passes a basic continuity test but fails under PoE load.

## Common Traps

### Filling Every Port and Exceeding the Switch Power Budget

The installer plugs access points into all 24 ports of a PoE switch, each drawing roughly 25 W, totaling 600 W against a 370 W budget. The mechanism of the failure is that the switch cannot deliver the sum and either refuses to power the last ports, or powers them all and then drops devices when the load peaks, causing random AP reboots and client disconnections. The false signal is that the first ports power up and the APs associate, which proves the per-port capability but not the total budget. The harm is an unstable wireless network where APs drop offline unpredictably and the root cause is hidden in the switch power management log. The defense is to sum the device draw against the switch's total budget and to add capacity where needed.

### High-Power AP on a Lower-Class Port

The installer connects a Wi-Fi 6E access point that requires 802.3bt (Type 4, up to 100 W) to an 802.3at (PoE+) port rated for 30 W. The mechanism of the failure is that the port cannot deliver the power the AP needs, so the AP either refuses to boot, boots into a degraded mode with fewer active radios and lower transmit power, or reboots repeatedly when it tries to enable full capability. The false signal is that the AP powers on and broadcasts a basic SSID, which proves the connection but not full function. The harm is reduced coverage and capacity that no one attributes to the power class mismatch. The defense is to read each device's PoE class and match the port or midspan to it.

### Long CCA Cable Browning Out a High-Power Device

The installer uses leftover copper-clad aluminum cable to reach an AP 85 meters away. The mechanism of the failure is that CCA's higher resistance causes excessive voltage drop over the long run, so the delivered voltage at the AP falls below its operating threshold under load, causing the AP to reboot or throttle. The false signal is that the AP powers on briefly during boot when the draw is low, which proves the connection but not the sustained delivery. The harm is an AP that reboots under client load and resists diagnosis because the cable passed a continuity test. The defense is to use pure-copper cable for all PoE runs and to keep high-power runs short.

### APs Mounted at the Nearest Conduit Rather Than the Coverage Point

The installer mounts each AP at the nearest existing junction box to avoid pulling new cable. The mechanism of the failure is that the AP locations do not match the coverage plan, leaving dead zones where clients cannot roam and creating interference where APs cluster. The false signal is that each AP is powered and broadcasting, which proves the deployment but not the coverage. The harm is a wireless network with complaints about dead spots and poor roaming that require a full re-survey and re-pull. The defense is to place APs by survey and to home-run cable to each location.

### Daisy-Chained or Split PoE Drops

The installer runs one cable to a ceiling location and uses a PoE splitter to feed two APs from the single run. The mechanism of the failure is that the single port's power and the cable's current capacity are shared, so both devices are underpowered, and a fault on the shared run takes down both APs at once, defeating the redundancy that home-runs provide. The false signal is that both APs light up, which proves the split works electrically but not reliably. The harm is correlated failures and underpowered radios. The defense is to home-run one cable per AP.

## Self-Check

- Did I read each powered device's PoE class and wattage and confirm that every switch port or midspan is rated to deliver at least that class?
- Did I sum the actual draw of all powered devices and compare it to the switch's total power budget, adding capacity where the budget is exceeded?
- Did I use pure-copper standards-compliant cable for all PoE runs, and did I avoid copper-clad aluminum entirely?
- Did I account for cable resistance on long runs to high-power devices and verify adequate delivered power at the far end?
- Did I place access points based on a coverage and capacity survey rather than on existing conduit, and did I stagger channels so adjacent APs do not interfere?
- Did I home-run a dedicated, labeled cable to each AP with a service loop, avoiding daisy-chains and splits?
- Did I verify after deployment that every AP is receiving full power and operating all radios, not just booting into a degraded mode?
- Is the PoE power budget, AP placement plan, and cable schedule documented so that another technician could add devices without exceeding the switch budget?
