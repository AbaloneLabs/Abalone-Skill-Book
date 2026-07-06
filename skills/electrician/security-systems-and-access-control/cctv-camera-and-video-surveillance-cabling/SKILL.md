---
name: cctv-camera-and-video-surveillance-cabling.md
description: Use when the agent is pulling CCTV cabling, planning camera placement, selecting coax versus UTP versus IP camera cabling, sizing VMS headend and NVR storage, calculating PoE and PoE+ voltage drop for PTZ cameras, or matching camera environmental ratings to outdoor and harsh locations.
---

# CCTV Camera and Video Surveillance Cabling

A video surveillance system is judged not by how the picture looks on day one but by whether the cameras still produce usable evidence three years later in rain, darkness, and network congestion, and that durability is determined almost entirely by the cabling, the power, and the placement decisions made during installation. The judgment problem is that an installer who pulls whatever cable is on the truck, powers cameras from the nearest convenient source, and mounts cameras for the best angle will produce a system that looks good in commissioning and degrades quickly, with dropped PTZs, water-damaged outdoor cameras, dark images at night, and storage that fills before the retention period ends. The cable type, the PoE budget, the environmental rating, and the storage sizing are the decisions that determine whether the system is a tool or a liability. This skill covers the cabling, power, placement, and storage decisions that determine whether a CCTV installation delivers reliable evidence.

## Core Rules

### Select the Correct Cable Type for the Camera Technology

The cable type must match the camera technology, and the three regimes, analog coax, HD-over-coax, and IP over UTP or fiber, have different distance, bandwidth, and termination requirements. Analog and HD-over-coax cameras run over 75-ohm RG59 or RG6 coaxial cable, with RG6 preferred for longer runs and HD signals. IP cameras run over twisted-pair copper (Cat6 or better) for runs within the 100-meter channel limit, and over fiber for longer runs or for immune backbone links between buildings. Mixing cable types or using the wrong coax impedance causes ghosting, rolling, or no picture. For IP cameras, the cable category must support the camera's data rate, and a Cat5e run to a high-bitrate camera will limit resolution and frame rate. Fiber links require proper termination and are immune to ground loops and lightning, which matters for runs between buildings.

The trap is using whatever cable is available. The defense is to match the cable type and category to the camera technology and data rate, to use fiber for long or inter-building runs, and to test each link for bandwidth and continuity.

### Power IP Cameras Within the PoE Budget and Voltage Drop Limit

Power over Ethernet (PoE) simplifies camera wiring by carrying power on the data cable, but each switch or injector has a total PoE budget that cannot be exceeded, and each camera has a class that defines its draw. Standard PoE (IEEE 802.3af) delivers up to 15.4 watts at the source, PoE+ (802.3at) delivers up to 30 watts, and PoE++ (802.3bt) delivers higher levels for heaters and heavy PTZs. PTZ cameras with heaters and wipers often require PoE+ or a separate power supply, because the heater inrush can exceed standard PoE. The total draw of all cameras on a switch must stay within the switch budget, and voltage drop on long runs reduces the power delivered to the camera, causing resets or failure at the far end. A camera that works on the bench may fail on a 90-meter run if the PoE budget and drop are not checked.

The trap is counting cameras instead of watts. The defense is to sum the class draw of all cameras against the switch budget, to verify PoE+ or PoE++ for PTZs with heaters, and to check voltage drop on long runs.

### Match the Camera Environmental Rating to the Location

Outdoor and harsh-environment cameras require an ingress protection (IP) rating appropriate to the exposure, and a camera rated IP66 or IP67 is the minimum for exposed outdoor locations, with IP67 providing immersion resistance for flood-prone mounts. Indoor cameras in dust or humidity, such as loading docks or kitchens, also need an elevated rating. The rating must apply to the entire installed assembly, including the connection, because a water-tight camera with an unsealed connector or an un-weatherproofed cable entry will take on water and fail. Conduit hubs, drip loops, and sealed junction boxes protect the connection, and the manufacturer's accessory enclosure must be used where specified. A camera installed without drip protection will fail within a season regardless of its rating.

The trap is trusting the camera rating and ignoring the connection. The defense is to specify a rating for the location, to weatherproof every connection and cable entry with drip loops and sealed hubs, and to use the listed accessory enclosure where required.

### Plan Camera Placement for Coverage, Lighting, and Field of View

Camera placement determines what the system actually captures, and a camera with the wrong angle, the wrong lens, or a view into the sun produces a feed that captures nothing useful. Each camera must be placed to cover the target area with the required resolution at the target distance, considering the lens field of view and the pixels-per-meter or pixels-per-foot needed to identify a person or a license plate. Lighting must be considered: a camera aimed into a bright entrance or the setting sun will silhouette everything in shadow, and a camera without infrared or supplemental light will see nothing at night. Wide dynamic range helps with mixed lighting but does not rescue a fundamentally bad aim. The placement should also consider tampering, with cameras mounted out of reach and supplemented by a camera covering the camera.

The trap is mounting cameras for a clean angle without checking coverage and lighting. The defense is to calculate the resolution at the target distance, to aim away from bright light sources, and to provide infrared or supplemental lighting for night coverage.

### Size NVR and VMS Storage for Retention, Resolution, and Motion

Storage must hold the recorded video for the required retention period at the cameras' resolution and frame rate, and undersized storage overwrites evidence before it is needed. Storage size depends on the number of cameras, the resolution, the frame rate, the compression, and the percentage of motion versus continuous recording. Motion recording reduces storage dramatically in low-traffic areas but requires correct motion detection setup to avoid missing events. The retention period is often set by regulation or policy, such as 30 or 90 days, and the storage must be sized with margin for that period. Redundant storage, RAID, and offsite backup matter for evidentiary systems, because a single failed drive can destroy weeks of footage. The headend and network bandwidth must also support the aggregate camera bitrate without dropping frames.

The trap is sizing storage by rule of thumb. The defense is to calculate storage from resolution, frame rate, compression, and retention, to use motion recording where appropriate, and to provide redundancy for evidentiary systems.

### Terminate and Test Every Link, and Manage the Headend Power

Every coax, UTP, and fiber link must be properly terminated and tested, because a cable that conducts but is poorly terminated causes intermittent video, rolling, or data errors that appear only under load. Coax requires correct BNC compression connectors on the right cable, UTP requires correct termination and testing to the category spec, and fiber requires proper polishing and testing for loss. Cable runs must be home-run or properly patched, not daisy-chained, and the headend must have conditioned power and cooling sized for the NVR, switches, and monitors. A headend that overheats or loses power takes the whole system down, so UPS backup and ventilation are part of the installation, not optional accessories.

The trap is plugging in and walking away if the picture appears. The defense is to terminate and test every link to spec, to home-run or properly patch runs, and to provide conditioned power, UPS, and cooling at the headend.

## Common Traps

### Wrong Cable Type or Category for the Camera Technology

The installer runs Cat5e to a high-resolution IP camera or uses the wrong impedance coax on an HD-over-coax link. The mechanism of the trap is that cable type and category are matched to the signal, and the wrong cable limits bandwidth, causes ghosting or rolling, or delivers no picture at all, with the problem appearing as intermittent video under load. The false signal is that some picture appears on the bench, which proves continuity but not adequate bandwidth. The harm is degraded or missing video that defeats the purpose of the camera. The defense is to match the cable type and category to the camera technology and data rate and to test each link.

### PoE Budget Exceeded or PTZ Heater Undersized

The installer connects many cameras to a PoE switch without summing the wattage, or powers a PTZ with a heater on standard PoE. The mechanism of the trap is that each PoE switch has a fixed total budget and each PTZ heater draws a surge, so exceeding the budget causes the switch to drop ports or the PTZ to reset, especially when the heater cycles on in cold weather. The false signal is that the cameras power up individually during commissioning, which proves each works but not the aggregate load. The harm is cameras that drop offline in winter or under load. The defense is to sum the class draw against the switch budget, to use PoE+ or PoE++ for PTZs with heaters, and to check voltage drop.

### Outdoor Camera With an Unsealed Connection

The installer mounts an IP66-rated outdoor camera but leaves the pigtail connection exposed or runs the cable into an unsealed box. The mechanism of the trap is that the camera rating applies to the housing, not to the connection, and water follows the cable into the connector and the camera body, corroding contacts and shorting the electronics within a season. The false signal is that the camera passes a dry-weather commissioning, which proves the rating but not the seal. The harm is premature camera failure and water damage. The defense is to weatherproof every connection with sealed hubs and drip loops and to use the listed accessory enclosure.

### Camera Aimed Into the Sun or Without Night Coverage

The installer aims a camera for the cleanest daytime angle, pointing it toward a bright entrance or the western sky, or installs a camera with no infrared in a dark area. The mechanism of the trap is that a camera exposed to bright backlight silhouettes everything in shadow and loses all detail, and a camera without infrared or supplemental light sees nothing after dark, so the camera captures nothing useful exactly when incidents occur. The false signal is that the daytime image looks crisp, which proves the camera works but not that it covers the target under real conditions. The harm is useless footage at the moments that matter. The defense is to aim away from bright light, to use wide dynamic range where needed, and to provide infrared or supplemental lighting.

### Storage Sized by Rule of Thumb and Overwriting Early

The installer sizes the NVR storage by a rough per-camera estimate without calculating for resolution, frame rate, and retention. The mechanism of the trap is that storage consumption scales with bitrate and time, and an undersized NVR overwrites footage before the retention period ends, so an incident from three weeks ago is gone when it is requested. The false signal is that the system records and plays back during commissioning, which proves recording works but not the retention. The harm is missing evidence that the system was supposed to preserve. The defense is to calculate storage from the actual parameters and retention requirement and to provide redundancy.

### Headend Without UPS or Cooling Fails on Power Loss or Heat

The installer racks the NVR and switches on a basic power strip with no UPS and no consideration of cooling. The mechanism of the trap is that the headend concentrates heat and depends on continuous power, so a power bump reboots the NVR and loses recording, and accumulated heat degrades the drives and shortens their life, causing the whole system to fail silently. The false signal is that the headend runs fine in an air-conditioned commissioning, which proves operation but not resilience. The harm is gaps in recording and premature drive failure. The defense is to provide UPS backup and adequate cooling at the headend.

## Self-Check

- Did I match the cable type and category to the camera technology and data rate, use fiber for long or inter-building runs, and test every link for bandwidth and continuity?
- Did I sum the PoE class draw of all cameras against the switch budget, verify PoE+ or PoE++ for PTZs with heaters, and check voltage drop on long runs?
- Did I specify an IP rating for each camera's location, weatherproof every connection and cable entry with drip loops and sealed hubs, and use the listed accessory enclosure?
- Did I calculate the resolution at the target distance, aim cameras away from bright light sources, and provide infrared or supplemental lighting for night coverage?
- Did I size NVR and VMS storage from resolution, frame rate, compression, and retention, use motion recording where appropriate, and provide redundancy for evidentiary systems?
- Did I terminate and test every coax, UTP, and fiber link to spec, home-run or properly patch runs, and avoid daisy-chaining?
- Did I provide conditioned power, UPS backup, and adequate cooling at the headend to prevent recording gaps and premature drive failure?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
