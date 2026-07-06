---
name: secure_source_communications.md
description: Use when the agent is setting up secure channels to communicate with sources, choosing tools that match the threat model, explaining secure communication options to sources, handling source outreach that must not leave a traceable record, or reviewing communications practices for exposure risks.
---

# Secure Source Communications

Secure communication with sources is not a single tool choice but a match between the channel, the source's capability, and the threat model, and it fails most often when reporters reach for the most secure tool without considering whether the source can use it, or when they default to convenient channels that leave a permanent, traceable record. The judgment problem is that the most secure channel is the one the source will actually use reliably, and the threat that matters is the one the source actually faces, which may be state surveillance, an employer monitoring corporate devices, an abusive partner with access to a phone, or discovery in litigation. A defensible practice starts from the threat model, selects channels that the source can sustain, and avoids creating records that outlive their usefulness.

Use this skill when setting up secure communications with sources, choosing tools, explaining options to sources, or reviewing communications practices for exposure. The main risk is that an agent recommends technically strong tools that the source cannot use, or defaults to convenient channels that create a permanent, traceable record.

## Core Rules

### Start From The Threat Model, Not The Tool

The choice of secure channel depends on what adversary the source is trying to stay hidden from, and different adversaries require different protections. A source hiding from an employer monitoring a corporate laptop needs a channel that does not touch the corporate device or network. A source facing state-level surveillance needs end-to-end encryption, metadata minimization, and possibly infrastructure outside the state's reach. A source hiding from an abusive partner with physical access to a phone needs a channel that leaves no visible trace on the device. A source worried about future litigation discovery needs a channel that does not create retained records.

Begin by understanding, with the source, who they are afraid of, what access that adversary has to their devices and accounts, and what records that adversary could obtain. Only then select a channel. Recommending Signal to a source whose phone is monitored by an employer who can demand the device is a failure; recommending a secure email to a source whose threat is a subpoena of your mail server is a failure. The threat model dictates the tool.

### Match The Channel To The Source's Capability And Habits

The most secure channel is the one the source will use correctly and consistently. A tool that the source finds confusing, that disrupts their routine, or that requires steps they will skip provides no security, because the source will fall back to convenient, insecure channels. Assess the source's technical comfort, their devices, their daily routine, and their tolerance for friction, and select a channel they can sustain.

This often means accepting a less technically secure channel that the source will use reliably over a more secure one they will abandon. That trade-off must be made deliberately, with eyes open about the residual risk, rather than by defaulting to convenience. Where the threat is high and the source cannot sustain the most secure channel, the options are to coach the source, to find a channel that balances security and usability, or to reconsider whether the story can be reported safely at all. See the digital-security skill.

### Avoid Channels That Create Permanent, Traceable Records

Many convenient channels, email, SMS, voicemail, messaging apps with cloud backup, create records that persist on servers, in backups, and across devices, and that can be obtained by adversaries through legal process, breach, or device seizure. For sensitive sources, default to channels that minimize retained records: ephemeral messaging with disappearing messages, in-person meetings, calls that are not recorded, and channels that do not sync to cloud backups.

Even encrypted channels may retain metadata, who contacted whom, when, and how often, that can reveal the source relationship. For the highest-threat sources, consider channels that minimize metadata, such as in-person contact, or that obscure it. The record a channel creates is part of the threat model: a perfectly encrypted message that sits on a backed-up server still creates exposure.

### Use End-To-End Encryption For Any Sensitive Digital Communication

For any sensitive communication that must occur over a digital channel, use end-to-end encryption, so that the content is protected in transit and at rest on the servers, and only the communicating parties can read it. Verify the encryption is genuine end-to-end, not merely transit encryption, and that the source understands how to confirm they are communicating with the right person, such as verifying safety numbers or keys.

Be aware of the limits of encryption. It protects content, not metadata. It protects the message, not the device, which may be compromised by malware, screen-lock bypass, or physical access. It protects the channel, not the human, who may be coerced, deceived, or surveilled in person. Encryption is necessary for sensitive digital communication but not sufficient on its own.

### Prefer In-Person Contact For The Most Sensitive Discussions

For the most sensitive discussions, especially initial contact with a high-risk source and any conversation that could expose the source to serious harm, prefer in-person contact in a location where the meeting itself does not reveal the relationship. In-person contact leaves no digital record, resists legal process, and allows the reporter to assess the source's circumstances directly. It has costs in time and logistics, but for high-threat situations it is often the only defensible channel.

Choose meeting locations that do not connect the source to the reporter or to the story, avoid surveillance, and allow private conversation. Consider whether the source's travel to the meeting creates exposure, and whether the meeting should occur away from both parties' normal locations. See the source-meeting-security skill.

### Explain The Channel And Its Limits To The Source

A secure channel protects only if the source uses it correctly, and correct use requires understanding what the channel protects and what it does not. Explain to the source, in terms they can act on, what the channel protects against, what records it creates or avoids, what they should and should not do, such as not backing up the channel to a personal cloud, and what to do if they are pressured to reveal the channel or its contents.

Do not overstate the protection. A source who believes a channel makes them invulnerable may take risks the channel does not cover, such as discussing the source relationship on an insecure channel or in person where overheard. Be honest about residual risk and about what the channel cannot do, so the source can make informed decisions about their own safety.

### Review Communications Practices For Exposure Regularly

Secure communications practices drift. A source who starts on a secure channel may revert to convenient ones under pressure or forgetfulness. A reporter may accumulate contacts across channels that create inconsistent protection. Tools change, vulnerabilities are discovered, and threat models evolve. Review communications practices regularly, with the source where possible, to confirm the channel is still being used, still appropriate to the threat, and still understood.

This review is part of the ongoing source relationship, not a one-time setup. A channel that was appropriate at first contact may become inadequate as the story develops, the source's circumstances change, or the adversary's attention increases. See the source-relationship-security skill.

## Common Traps

### Reaching For The Most Secure Tool Without Considering Usability

A tool the source cannot use reliably provides no security. Match the channel to the source's capability and habits, accepting deliberate trade-offs between security and usability.

### Defaulting To Convenient Channels That Create Permanent Records

Email, SMS, voicemail, and cloud-backed messaging create records that can be obtained by adversaries. For sensitive sources, default to channels that minimize retained records.

### Assuming Encryption Covers Everything

End-to-end encryption protects content in transit and at rest, not metadata, not the device, not the human. It is necessary for sensitive digital communication but not sufficient.

### Ignoring Metadata

Even encrypted channels reveal who contacted whom, when, and how often. For high-threat sources, consider channels that minimize or obscure metadata.

### Overstating Protection To The Source

A source who believes a channel makes them invulnerable takes risks the channel does not cover. Be honest about residual risk and the channel's limits.

### Treating Channel Setup As A One-Time Event

Secure communications practices drift. Review regularly to confirm the channel is still used, still appropriate, and still understood as the threat evolves.

## Self-Check

Before treating secure source communications as sound, verify:

- The channel was selected from the threat model, with the source, identifying who they fear, what access that adversary has, and what records could be obtained, not from a generic tool recommendation.
- The channel matches the source's capability and habits, so they will use it correctly and consistently, with any security-usability trade-off made deliberately.
- The channel minimizes retained records appropriate to the threat, avoiding email, SMS, voicemail, and cloud backups for sensitive sources where they create exposure.
- Any sensitive digital communication uses genuine end-to-end encryption, with the source able to verify they are communicating with the right person.
- The most sensitive discussions occur in person, in locations that do not reveal the relationship, with the source's travel considered for exposure.
- The source understands what the channel protects, what it does not, what they should and should not do, and what to do if pressured, without overstated protection.
- Communications practices are reviewed regularly, with the source where possible, to confirm the channel remains appropriate as the story and threat evolve.
- The channel does not create metadata exposure that defeats the source's protection, considering who contacted whom, when, and how often.
- The channel choice is documented in a form that does not itself expose the source, and is consistent across the reporting team.
- Secure source communications support editorial judgment; channel decisions for high-risk sources should involve a responsible editor and, where the source faces legal or state threats, legal counsel or a digital security specialist, recognizing that no channel eliminates risk and that the source's safety may require foregoing or delaying the story.
