---
name: cctv-and-video-surveillance-installation.md
description: Use when the agent is laying out camera coverage and field of view, selecting resolution and codec, running PoE cabling for cameras, sizing NVR or VMS storage for retention, or addressing lighting, IR illumination, and environmental housings for video surveillance.
---

# CCTV and Video Surveillance Installation

A surveillance camera that powers up and shows an image is not a surveillance system; it is a camera that shows an image. The difference is whether the image is useful after the fact, whether a face or license plate is recognizable at the distance that matters, whether the recording survives long enough to be reviewed, and whether the camera keeps working through the night, the rain, or the power event that precipitated the incident. The judgment problem is that camera installation is easy to do badly and hard to do well, because the failures, insufficient resolution at the target distance, a blind spot at the critical angle, a recording that overwrites before it is reviewed, a camera blinded by its own IR reflection, are invisible until the one moment the system is actually needed. This skill covers the layout, cabling, storage, and environmental decisions that determine whether a video system provides evidence or just motion.

## Core Rules

### Design Coverage From the Investigative Objective, Not From the Camera Count

Camera placement begins with what must be recognizable after the fact, not with how many cameras the budget allows. The key metric is pixels-per-foot (PPF) at the target plane: roughly 40 PPF for general scene observation, 80 to 100 PPF for facial identification, and 120 PPF or more for license plate recognition. Field of view and PPF trade against each other for a given resolution, so a wide-angle camera covering a large area yields low PPF and cannot identify faces at distance, while a narrow camera yields high PPF over a small area and misses the wider context. The design must work backward from the objective, choose the lens and resolution that deliver the required PPF at the target distance, and then place cameras to cover the critical zones with the required detail.

The trap is spacing cameras evenly to cover square footage. The defense is to define the investigative objective for each zone, calculate PPF at the target plane, and verify coverage with a lens calculator or on-site mockup before committing to mounts and cable runs.

### Account for Blind Spots and Camera-to-Camera Handoff

A single camera cannot see what is directly beneath it or behind an obstruction, and a row of cameras that each cover a corridor can still leave a gap where an actor moves from one field of view to another without ever being in frame. Critical paths, entries, exits, and choke points should be covered from two angles so that an actor cannot step between coverage zones, and cameras should overlap at the edges so there is no unobserved handoff. Exterior cameras must account for sun position, which can blind a sensor at certain times of day, and for seasonal foliage that grows into the field of view.

The trap is mounting cameras where the cable is easy and assuming the field of view is adequate. The defense is to walk each camera's intended view at install time, to verify overlap at choke points, and to document the sun arc and seasonal obstructions that will change the view over the year.

### Match Resolution, Codec, and Bitrate to the Investigative Need

Resolution determines the raw detail, but the codec and bitrate determine whether that detail survives compression. A 4MP camera starved of bitrate will produce a blocky, smeared image that resolves no more than a well-encoded 2MP camera. H.265 roughly halves the bitrate of H.264 for equivalent quality but requires end-to-end support in the camera, NVR, and client. Constant bitrate produces predictable storage but can drop detail in high-motion scenes, while variable bitrate preserves detail at the cost of unpredictable storage. The right combination is the lowest bitrate that still delivers the target PPF after compression at the expected scene complexity, validated by recording a representative clip and reviewing it.

The trap is trusting the camera's advertised resolution and leaving bitrate at default. The defense is to record representative clips at the intended settings, to review them at the target distance, and to tune codec and bitrate until the investigative objective is met without over-provisioning storage.

### Respect PoE Class, Cable Type, and Distance Limits

Power over Ethernet simplifies camera cabling but imposes limits that are easy to exceed. Standard PoE (IEEE 802.3af) delivers up to 15.4W at the source, PoE+ (802.3at) up to 30W, and PoE++ (802.3bt) up to 60W or 100W, and the camera's draw, including heaters and IR illuminators, must fit within the class after cable loss. Cat6 is preferred over Cat5e for PoE because its lower DC resistance reduces voltage drop over long runs. The 100-meter channel limit applies, and runs approaching it with high-power cameras can drop voltage below the camera's operating range, causing intermittent reboots that are difficult to diagnose. Cable rated for the environment, plenum, riser, or outdoor UV-resistant, is mandatory, and outdoor runs require surge protection at the building entrance.

The trap is pulling long runs of Cat5e to PoE++ PTZ cameras with heaters and expecting them to be reliable. The defense is to use Cat6 for PoE runs, to stay within the 100-meter channel, to budget the camera draw including heaters against the PoE class after cable loss, and to install listed surge protection on outdoor runs.

### Size Storage From Bitrate, Camera Count, and Retention, Then Choose RAID

Storage is the product of per-camera bitrate, the number of cameras, and the retention period, with overhead for motion variability and for the RAID level. A 16-camera system at 4Mbps each for 30 days requires roughly 20 TB of raw storage before overhead, and motion-triggered recording can reduce this but only if the motion detection is reliable, which it often is not at night. RAID 5 trades capacity for single-drive fault tolerance, RAID 6 tolerates two failures at greater capacity cost, and RAID 10 favors performance over capacity. The decision must account for the fact that surveillance storage is written continuously, so drive endurance and the array's rebuild behavior matter as much as raw capacity.

The trap is buying storage by a rough rule of thumb and discovering the system overwrites before retention is met. The defense is to compute storage from actual bitrate measurements, to provision for the target retention plus headroom, and to choose a RAID level that protects against the most likely failure without sacrificing the required capacity.

### Verify Lighting and IR Illumination for Day, Night, and Backlight

A camera can only record the light that reaches its sensor, and the scene that looks fine to the eye at dusk may be a black frame to a camera with inadequate sensitivity or illumination. Day/night cameras switch to monochrome and raise gain in low light, which trades noise for visibility, and integrated IR illuminators extend coverage into total darkness but only to their rated range and only if nothing reflects the IR back into the lens. Backlight from doors, windows, or parking lot lights can silhouette a subject and destroy facial detail, requiring wide dynamic range or repositioning. The lighting design must be verified at night, not assumed from daytime images.

The trap is commissioning the system in daylight and walking away. The defense is to return after dark, to verify that IR coverage reaches the target distances without hot spots or reflection, and to address backlight with WDR settings or camera repositioning before sign-off.

### Select Environmental Housings and Ratings for the Installed Location

A camera rated IP66 in a sheltered soffit will last for years, while the same camera on an exposed pole in a coastal environment will fail in a season. The ingress rating, the operating temperature range, the UV resistance of the housing, and the presence of a heater or blower must match the installed environment. Explosion-proof housings are required in classified areas. Vandal-resistant domes with IK10 impact ratings are appropriate where the camera itself is a target. Condensation inside a housing is a common failure that a blower or desiccant prevents, and a camera that fogs internally is blind at exactly the cool, damp hours when incidents occur.

The trap is using the same indoor-rated dome for every location to standardize the bill of materials. The defense is to match the environmental rating to each location, to add heaters, blowers, or desiccants where condensation is possible, and to use vandal-resistant housings where the camera is reachable.

## Common Traps

### Even Camera Spacing Without PPF Verification

The installer spaces exterior cameras evenly around the building to cover the perimeter and declares the layout complete. The mechanism of the trap is that even spacing optimizes for square footage rather than for the investigative objective, so the cameras at the loading dock, where identification matters, deliver the same low PPF as the cameras over the roof. The false signal is that every camera shows a live image and the perimeter appears covered on the monitor, which proves coverage but not usefulness. The harm is an incident reviewed after the fact where the subject is an unrecognizable blob at the critical location, rendering the entire system useless for its purpose. The defense is to design each camera's lens and placement from the required PPF at the target distance and to verify with a mockup.

### Default Bitrate on a High-Resolution Camera

The installer leaves the camera at its factory bitrate setting, trusting the advertised 4MP resolution to deliver detail. The mechanism of the trap is that resolution is the sensor's capability, but the bitrate caps how much of that detail survives compression, and a high-resolution camera starved of bitrate produces a smeared image no better than a lower-resolution camera. The false signal is that the live image looks sharp on the monitor, which it does because live view is often less compressed than the recording. The harm is a recording that, when reviewed after an incident, lacks the detail the resolution promised, because the bitrate silently threw it away. The defense is to record representative clips at the intended settings and to tune bitrate until the recorded image meets the objective.

### Cat5e PoE++ Run Approaching 100 Meters to a Heated PTZ

To reach a distant parking lot camera, the installer pulls a 95-meter run of Cat5e to a PoE++ PTZ with a heater. The mechanism of the trap is that Cat5e has higher DC resistance than Cat6, and over 95 meters the voltage drop is large enough that the heater's inrush collapses the supply voltage below the camera's operating range, causing reboots that happen only in cold weather when the heater cycles. The false signal is that the camera works fine during summer commissioning, when the heater never energizes. The harm is a camera that drops offline every cold night, a seasonal intermittent that resists diagnosis. The defense is to use Cat6 for PoE runs, to stay within the channel limit, to budget heater inrush against the PoE class, and to consider a local power supply for distant high-draw cameras.

### Storage Sized by Rule of Thumb, Overwriting Before Retention

The installer provisions storage by a rough per-camera rule, enables motion recording to stretch capacity, and hands the system over. The mechanism of the trap is that motion recording only saves space when motion detection is reliable, and at night, with insects, shadows, and foliage, motion detection triggers almost continuously, so the expected savings never materialize and the system overwrites the retention window. The false signal is that the storage graph looks healthy at handover, when only a few days of recording exist. The harm is an incident discovered a week later, by which time the relevant recording has been overwritten. The defense is to compute storage from measured bitrate under continuous recording, to provision for the full retention plus headroom, and to treat motion recording as a bonus, not as the basis for the storage calculation.

### Commissioning in Daylight and Ignoring Night Performance

The installer focuses and aims every camera during the day, confirms a clean image on the monitor, and leaves. The mechanism of the trap is that daytime commissioning never exercises the IR illuminators, the day/night cut filter, or the low-light gain, so problems that only appear after dark, insufficient IR range, IR reflection from a nearby wall, a stuck color filter, go undetected. The false signal is a sharp daylight image on every camera. The harm is a system that is effectively blind during the night hours when most incidents occur, discovered only when a nighttime event yields a black or noisy frame. The defense is to return after dark, verify IR coverage and the day/night switch on every camera, and re-aim or add illuminators as needed.

### Indoor-Rated Dome in an Exposed Outdoor Location

To standardize the bill of materials, the installer uses the same indoor-rated dome for a camera mounted on an exposed exterior wall. The mechanism of the trap is that the indoor housing lacks the ingress and UV resistance for the environment, so moisture infiltrates the dome, UV degrades the plastic, and the camera fogs or fails within months. The false signal is that the camera works at installation, when the housing is new and dry. The harm is progressive image degradation from condensation and eventual failure, plus the cost of a bucket truck to replace a camera that should have been outdoor-rated from the start. The defense is to match the environmental rating to each location and to add condensation control where the environment demands it.

## Self-Check

- Did I define the investigative objective for each camera zone and calculate pixels-per-foot at the target plane to confirm the lens and resolution deliver the required detail?
- Did I verify camera-to-camera overlap at choke points and check for blind spots beneath and behind each camera, accounting for sun arc and seasonal foliage?
- Did I record representative clips at the intended codec and bitrate and confirm the recorded, not just live, image meets the investigative objective?
- Did I use Cat6 for PoE runs, stay within the 100-meter channel, budget camera draw including heaters against the PoE class, and install listed surge protection on outdoor runs?
- Did I compute storage from measured continuous bitrate, provision for the full retention period plus headroom, and choose a RAID level appropriate to the failure tolerance and capacity required?
- Did I return after dark to verify IR coverage, the day/night switch, and backlight handling on every camera, and did I re-aim or add illuminators where night performance fell short?
- Did I match the environmental rating, operating temperature range, and condensation control to each installed location, using vandal-resistant housings where the camera is reachable?
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
