---
name: noise-vibration-and-harshness-diagnosis.md
description: Use when the agent is diagnosing vehicle vibrations, noises, or harshness complaints, distinguishing tire, driveline, engine, and exhaust sources, road-testing to isolate frequencies, or deciding whether a vibration is speed-related, engine-speed-related, or load-dependent.
---

# Noise, Vibration, and Harshness Diagnosis

NVH complaints are among the most frustrating and most misdiagnosed problems a technician faces, because the customer feels something the technician cannot always reproduce, and because "it makes a noise" or "it shakes" is a description that could point to twenty different systems. The judgment problem in NVH work is that the human ear and body are poor frequency analyzers: a vibration that the customer calls "a shimmy" may be a tire imbalance, a worn CV joint, a failing engine mount, a warped rotor, or a driveline angle problem, and the technician who guesses based on the most common cause will be wrong often enough to destroy trust. Worse, NVH diagnosis tempts shortcuts — balancing tires, replacing mounts, throwing parts at the complaint — that feel productive but do not isolate the source. This skill covers the disciplined process of reproducing, classifying, and isolating NVH complaints so the repair addresses the actual source instead of masking the symptom.

## Core Rules

### Reproduce the Complaint Under the Customer's Conditions First

An NVH complaint that cannot be reproduced cannot be diagnosed. Before forming any theory, road-test the vehicle under the conditions the customer described: the same speed range, the same road surface, the same engine load, the same ambient temperature. Ask the customer exactly when it happens — accelerating, cruising, coasting, braking, turning, cold start, after warm-up — and reproduce those conditions deliberately. A vibration that only appears between 55 and 62 mph is a different fault from one that appears at any speed proportional to road speed, and both differ from a vibration tied to engine RPM regardless of vehicle speed.

If you cannot reproduce the complaint, do not pretend you did. Document what conditions you tried, what you observed, and return the vehicle to the customer with an honest account. Closing the ticket as "could not duplicate" without genuine effort to reproduce is a credibility failure; claiming to have found and fixed something you never reproduced is worse.

### Classify the NVH by Its Relationship to Speed and Load

The single most diagnostic question in NVH work is: what is the vibration or noise correlated with? Three correlations cover most faults:

- **Vehicle-speed-related** — the vibration frequency rises and falls with road speed, whether the car is in gear, in neutral, or coasting. This points to tires, wheels, wheel bearings, brake rotors, driveshaft, or axle components rotating at wheel speed.
- **Engine-speed-related** — the vibration frequency tracks engine RPM, whether the vehicle is moving or sitting still. Rev the engine in neutral: if the vibration appears, the source is engine, accessory drive, exhaust, or engine mounts.
- **Load-dependent** — the vibration appears or changes under acceleration versus coasting, or turning versus straight-ahead. This points to driveline torque, CV joints, mounts that move under load, or suspension components that shift geometry.

Run these correlation tests deliberately. A road test where you note "vibration at 60 mph" without testing whether it persists in neutral, or whether it tracks engine RPM, has thrown away the data that would have narrowed the source. The disciplined technician spends the first road test collecting correlations, not guessing at causes.

### Measure Frequency When the Ear Cannot Distinguish

Many NVH faults produce distinct frequencies that the ear cannot reliably separate. An electronic vibration analyzer (EVA), a chassis ear, or even a smartphone-based frequency analyzer can convert "a buzz" into "a 42 Hz vibration," which maps to a specific rotating component once you know its rotational speed. First-order tire/wheel vibration at 60 mph is typically in the 10-15 Hz range; driveshaft frequencies are higher; engine-related vibrations at idle are around 10-25 Hz depending on cylinder count. Calculating the expected frequency of a suspected component (RPM divided by 60, times the order) and comparing it to the measured frequency is how a professional confirms the source instead of guessing.

When no analyzer is available, use the process of elimination through controlled changes: shift into neutral at speed to drop driveline load, lightly apply the brakes to load rotors, cut the engine at speed to eliminate engine and accessory sources. Each test changes one variable; the result tells you whether that variable is involved.

### Isolate the Location Before Naming the Component

A vibration felt in the steering wheel points to the front of the vehicle (often front tires, rotors, or front suspension); felt in the seat or floor points to the rear (rear tires, driveshaft, rear axle). A noise from "the front" could be a wheel bearing, a CV joint, a strut mount, a brake pad, or an exhaust heat shield — and chasing the wrong location wastes hours. Use a chassis ear or stethoscope to localize mechanical noises, and use the seat-versus-wheel distinction to localize vibrations. Only after the location is isolated should you name a suspect component, and even then you must test the component before replacing it.

### Distinguish Tire and Wheel Sources From Driveline Sources

Tire and wheel problems — imbalance, radial force variation, out-of-round, belt separation — are the most common source of speed-related vibrations, and they are also the most over-diagnosed. The trap is balancing or replacing tires for every vibration. Before condemning tires, perform the on-car tests: does the vibration change when the tires are swapped front-to-rear (a tire problem will move with the tire)? Does it persist in neutral? Does road-force variation testing reveal a stiff spot? A vibration that does not move when the tires are swapped is not a tire problem, and balancing tires that are already balanced wastes the customer's money without fixing the complaint.

## Common Traps

### Balancing Tires for Every Speed-Related Vibration — The customer reports a vibration at highway speed, the technician assumes tire imbalance, and the tires are balanced or replaced. The trap mechanism is that balancing is cheap, fast, and often "fixes" the complaint temporarily because the technician disturbed the wheels and re-torqued them, masking a loose wheel bearing or a bent hub runout that was the real source. The false signal is that the vibration "felt better" after the balance, which the technician reads as confirmation. The harm is that the underlying fault — a worn wheel bearing, a hub with excessive runout, a brake rotor with lateral runout, or a bent wheel — returns within weeks, the customer returns angry, and the shop has charged for a balance that did not address the cause. The disciplined technician measures wheel and hub runout and road-force variation before balancing, and recognizes that a vibration that does not improve after a verified balance is not a balance problem.

### Attributing a Noise to the Most Recently Replaced Part — A customer returns with a new noise shortly after a repair — new tires, new brakes, new struts — and both technician and customer assume the new part is the cause. The trap mechanism is recency bias: the brain links the noise to the most recent change. The false signal is that the noise "started right after" the service. The harm is that the technician tears apart the recent repair, re-does the work, possibly damages good parts, and never investigates the unrelated fault that actually developed — a heat shield that rusted loose, a sway bar link that finally failed, an exhaust hanger that let go. The disciplined technician treats a post-repair noise as a new complaint to be diagnosed on its own merits, verifying whether the recent work could plausibly cause it before assuming it must have.

### Diagnosing a Wheel Bearing by Sound Alone — Wheel bearing failures produce a characteristic growl or hum that changes with cornering load, and experienced technicians often identify them by ear. The trap mechanism is that tire noise — especially from chopped or cupped tires, or aggressive tread patterns — mimics bearing growl almost perfectly, and the only reliable distinction is whether the noise changes when the tire's load is transferred in a sweeping turn. The false signal is a growling noise that "sounds like a bearing." The harm is that the technician replaces a good wheel bearing, the noise is unchanged because it was the tire, and the customer pays for a misdiagnosis. The disciplined technician confirms a bearing with a loaded-and-unloaded turn test, measures play, and checks for heat after a drive before condemning the bearing — and always considers that cupped tires from a worn strut or alignment problem can produce identical noise.

### Assuming an Engine Vibration Is a Misfire Without Confirming — A rough idle or vibration is often assumed to be an ignition misfire, leading to plug and coil replacement. The trap mechanism is that many non-ignition faults produce the same sensation: a vacuum leak leaning out one cylinder, a dirty fuel injector, a worn engine mount transmitting normal combustion pulses into the body, a broken harmonic balancer, or even a variable-displacement engine running on fewer cylinders by design. The false signal is that the engine "feels like it's misfiring." The harm is that ignition parts are replaced, the vibration persists because the cause was a mount or a vacuum leak, and the misfire data that would have pointed to the real cause was never read. The disciplined technician reads misfire counter data on the scan tool, checks fuel trim, and tests mounts before replacing ignition components.

### Chasing the Noise the Customer Hears Instead of the Source — The customer hears a rattle "from the dashboard," and the technician disassembles the dash looking for a loose component. The trap mechanism is that sound travels through structure, and a rattle from a loose exhaust heat shield, a worn sway bar link, or a loose catalytic converter bracket transmits through the body and is perceived by the occupant as coming from the interior. The false signal is the customer's localized perception of the sound. The harm is hours of interior disassembly that finds nothing, while the actual source — accessible underneath the vehicle — is never inspected. The disciplined technician reproduces the noise with a helper outside the vehicle, uses a chassis ear to localize the source, and checks the underbody and exhaust before disassembling interior trim.

## Self-Check

- Did I road-test under the customer's exact conditions (speed, load, temperature, surface) before forming any theory?
- Did I classify the NVH as vehicle-speed-related, engine-speed-related, or load-dependent through deliberate tests (neutral coast, in-gear, revving in park)?
- Did I note whether the vibration is felt in the steering wheel, the seat, or the whole vehicle, and use that to localize front versus rear?
- Before balancing or replacing tires, did I confirm the vibration moves when tires are swapped, or measure hub and wheel runout?
- For a suspected wheel bearing, did I perform a loaded/unloaded turn test and check for play and heat, rather than diagnosing by sound alone?
- For an engine vibration, did I read misfire counters and fuel trim before replacing ignition parts?
- For a localized noise complaint, did I check underbody, exhaust, and suspension sources before disassembling interior trim?
- Did I use a frequency analyzer or chassis ear when the ear could not distinguish the source, instead of guessing?
- Did I document the correlations I found (speed, RPM, load, location) so the next technician does not start from scratch if the complaint returns?
