---
name: sound_in_product_context.md
description: Use when the agent is deciding how sound functions within a specific product context, including when sound should be ambient versus foreground, how it fits different devices and environments, accessibility and cognitive load of audio, cultural and acoustic considerations, and how sonic choices support or undermine the product's function, audience, and use setting.
---

# Sound In Product Context

The same sound can be perfect in one product and disastrous in another. A playful pop on a children's game delights; the same sound on a banking app undermines seriousness. A rich ambient layer suits a meditation app and ruins a productivity tool. Sound in products is never evaluated in the abstract; it is evaluated against the product's function, audience, device, environment, and the moment of use. Designers fail at contextual sound when they borrow sonic conventions from one domain and apply them to another without considering what the sound means in this specific product.

Use this skill when defining the role of sound within a particular product, deciding where audio sits in the experience, how it adapts to context, and what it must respect. The goal is to prevent the agent from designing sound in a vacuum and instead to root every sonic decision in the product's real use setting, audience, and constraints.

## Core Rules

### Define The Role Of Sound For This Product

Before designing any individual sound, define what sound is for in this product. Is it functional feedback, ambient atmosphere, brand expression, navigation aid, or emotional enhancement? Different roles demand different treatment, and a product that mixes roles without a clear model produces incoherent audio.

Clarify the role by asking:

- does sound primarily inform, guide, delight, or set mood?
- is sound central to the product, as in music or calls, or peripheral, as in a productivity tool?
- should sound be noticed or felt as absence when missing?
- what is the product's stance toward sound: generous, restrained, or silent by default?

A productivity tool and a meditation app both use sound, but their roles, and therefore their design, are opposites. Define the role before choosing any sound.

### Match Sonic Character To Product Tone And Audience

Sound carries emotional and cultural meaning that interacts with the product's tone. A sound that feels friendly in a casual consumer app can feel unprofessional in a financial or medical product. A sound that feels energetic in a fitness app can feel aggressive in a wellness app. The audience's expectations and the product's promise must align with the sonic character.

Align character by considering:

- the product's brand personality and whether sound should reinforce or soften it;
- the audience's age, culture, and expectations about technology and sound;
- the emotional state the product aims to support, such as calm, focus, urgency, or play;
- the risk of sound feeling childish, cold, alarming, or intrusive for this audience.

Serious, high-stakes products generally warrant restrained, neutral, almost invisible sound. Playful, low-stakes products can afford more character. Mismatching character to context is a common and visible failure.

### Adapt Sound To Device And Environment

Sound behaves differently across devices and environments. A sound designed for headphones may be inaudible on a phone speaker; a sound suited to a quiet home is offensive in an open office; a sound for a mobile app must assume the user may be in public. The product's primary devices and environments must drive sonic decisions.

Adapt by:

- designing and testing on the devices users actually have, especially phone speakers and earbuds;
- accounting for the environments of use, public, private, noisy, quiet, shared;
- considering that mobile users are often in contexts where any sound is unwanted;
- providing vibration or haptics as a discreet alternative on mobile;
- ensuring sounds remain intelligible or appropriately subtle across output devices.

A sound that depends on bass or fine detail to work will fail on the speakers most users actually use.

### Respect Cognitive Load And Attention

Sound competes for attention in a way visuals do not. Every sound the product makes is a claim on the user's cognitive bandwidth, and in tasks that require focus, even pleasant sounds become interruptions. The cognitive cost of sound must be weighed against its value, especially in productivity, learning, and complex tools.

Reduce cognitive cost by:

- minimizing the number of distinct sounds a user must learn;
- using silence as the default in focus-intensive contexts;
- ensuring sounds are distinguishable from each other and from environmental noise;
- avoiding sounds that require interpretation during time-pressured tasks;
- letting users disable all but essential sounds during focused work.

A product that demands attention through sound while the user is trying to concentrate will be muted or abandoned.

### Design For Accessibility Of Sound

Sound design has accessibility dimensions that are often overlooked. Users who are deaf or hard of hearing may miss audio entirely. Users with sensory sensitivities may find certain sounds painful or distressing. Users with cognitive differences may struggle to interpret complex audio cues. Accessible sound design means never making sound a barrier.

Accessible practices:

- never convey essential information through sound alone;
- provide captions or transcripts for spoken content;
- allow adjustment of volume, and where possible pitch and repetition;
- avoid sudden loud sounds that can distress users with sensory sensitivities;
- provide clear, complete visual and haptic alternatives;
- test with users who rely on alternatives to audio.

Accessibility is not a constraint on good sound design; it is a requirement that often improves the design for everyone.

### Consider Cultural And Acoustic Conventions

Sound carries cultural meaning that varies across regions and communities. A chime associated with celebration in one culture may signal mourning in another. Ascending tones often read as positive and descending as negative in many contexts, but conventions are not universal. Acoustic conventions, such as what a notification should sound like, also evolve and differ across platforms.

Consider:

- whether sonic conventions are universal or culturally specific;
- how the sound will read to users in different regions and languages;
- platform conventions, since users expect iOS and Android sounds to differ;
- generational and subcultural associations with certain sounds;
- whether a sound unintentionally resembles an alarm, siren, or emergency tone.

Sounds that ignore cultural and acoustic conventions can confuse, offend, or alarm users unintentionally.

### Let Context Shift Within The Product

A single product may span contexts that warrant different sonic treatment. A focus mode should sound different from a social mode; a critical alert should cut through ambient audio; a nighttime context should be quieter than daytime. Design sound to adapt to in-product context, not as a single fixed layer.

Adapt within product by:

- reducing or silencing sound in focus, sleep, or do-not-disturb modes;
- escalating sound for genuinely critical events while keeping routine events quiet;
- allowing the user to choose sonic profiles for different activities;
- ensuring transitions between contexts are smooth, not jarring.

### Align Sound With The Product's Broader Sensory Design

Sound does not exist separately from the rest of the product. It must align with the visual motion, haptics, and overall rhythm of the interface. A sound that fires out of sync with its visual counterpart feels broken; a sound whose character contradicts the visual tone feels incoherent.

Align across senses by:

- timing sound precisely with the visual or haptic event it accompanies;
- matching sonic character to motion character, smooth, sharp, playful, or serious;
- ensuring sound, motion, and haptics reinforce rather than compete;
- treating the combination of senses as one designed experience.

## Common Traps

### Borrowing Sonic Conventions Across Domains

A playful consumer sound applied to a serious product undermines credibility; a serious sound in a playful product feels cold.

### Designing Only For Headphones

Sounds that depend on frequency range or detail fail on the phone speakers and laptop speakers most users actually have.

### Ignoring Public And Shared Environments

Sounds that are fine at home are often embarrassing or disruptive in offices, transit, and public spaces.

### Overloading Cognitive Bandwidth

Too many distinct sounds, or sounds during focus tasks, compete for attention and drive users to mute.

### Sound As The Only Channel

Relying on sound alone excludes users who are deaf, hard of hearing, in noisy environments, or on muted devices.

### Cultural Or Acoustic Misalignment

Sounds that unintentionally resemble alarms, or that carry unintended cultural meaning, can confuse or alarm users.

### Fixed Sound Across All Contexts

Treating sound as one unchanging layer ignores that the same product is used in focus, rest, social, and urgent contexts that need different sonic treatment.

## Self-Check

- [ ] The role of sound in this product is defined: functional, ambient, brand, navigational, or emotional.
- [ ] Sonic character matches the product's tone, brand personality, and audience expectations.
- [ ] Sounds were designed and tested on the real devices and in the real environments of use.
- [ ] Cognitive load is respected, with silence as default in focus-intensive contexts and minimal distinct sounds to learn.
- [ ] Essential information is never conveyed by sound alone, and visual, textual, and haptic alternatives are complete.
- [ ] Volume, pitch, repetition, and suddenness are adjustable or controlled for users with sensory sensitivities.
- [ ] Cultural, regional, and platform conventions were considered to avoid confusion or unintended alarm.
- [ ] Sound adapts to in-product context, such as focus, sleep, and critical modes, and aligns with motion and haptics.
