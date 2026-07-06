---
name: secure_communications_setup.md
description: Use when the agent is setting up secure communications with a sensitive source, choosing between encrypted messaging apps, secure email, secure drop platforms, or in-person meetings, explaining secure tools to non-technical sources, or building a communication channel that protects a source whose exposure would cause harm.
---

# Secure Communications Setup

Secure communication with a sensitive source is not a single tool choice; it is the design of an entire channel that protects the source from first contact through publication and beyond. The judgment problem is that reporters often focus on the encryption of a single app while ignoring the surrounding exposure: how the first contact was made, what metadata the channel leaks, what records persist on the source's monitored device, how the relationship will be maintained, and what happens if any link in the chain is compromised. A channel is only as secure as its weakest point, and the weakest point is usually not the encryption but the human, procedural, and metadata exposures around it. Designing secure communications means thinking in systems, not in tools.

Use this skill when establishing a secure channel with a sensitive source, choosing among encrypted apps, secure email, secure drop, or in-person meeting, explaining tools to a non-technical source, or maintaining a sensitive channel over time. The main risk is that an agent imposes a sophisticated tool that the source cannot safely use, or that secures content while leaking the fact of communication.

## Core Rules

### Design The Whole Channel, Not Just The App

A secure channel runs from first contact through ongoing communication to the handling of the material shared. Each stage has exposures. The first contact may have been made through an unencrypted, monitored channel that already revealed the relationship. The chosen app may encrypt content but log metadata, contact lists, and timestamps that expose who is talking to whom. The source's device may be monitored by an employer, a spouse, or a state, so that installing an encrypted app creates a visible record. The material shared may be stored in ways that expose the source if the reporter's device is seized.

Design the channel end to end. Ask how first contact was made and whether it left a trace, what the chosen tool reveals beyond content, what records persist on both ends, how ongoing communication will be maintained without creating patterns, and what happens to each of these exposures if any link is compromised. A tool that secures content while exposing the relationship has not protected the source in contexts where the relationship itself is dangerous.

### Match The Tool To The Source's Real Environment

The most secure tool is useless if the source cannot or will not use it safely. A source on a workplace-monitored phone cannot safely install a new encrypted app; a source in a repressive state may face legal risk merely for possessing encryption tools; a source who is not technically fluent may misuse a tool in ways that expose them. Match the tool to the source's actual environment, capabilities, and risk, not to the reporter's preferred toolkit.

Sometimes the safest channel is the least technological: an in-person meeting in a location free of surveillance, with no devices carried, and notes taken on paper. Sometimes it is a secure drop platform that accepts anonymous submissions. Sometimes it is an encrypted app the source already uses for routine communication, so that its presence is not itself a signal. Choose based on what reduces the source's exposure, not on what demonstrates sophistication.

### Understand What Each Tool Protects And What It Reveals

Different tools protect different things and reveal different things. End-to-end encrypted messaging protects content from the provider but typically reveals that two accounts are in contact, when, and how often. Encrypted email protects content but exposes metadata through email headers and provider logs. Secure drop platforms can protect identity but may require technical sophistication the source lacks. Phone calls, even encrypted ones, may reveal location and contact patterns through billing records. In-person meetings leave no digital trace but may be observed.

For each tool under consideration, understand precisely what it protects and what it does not, and weigh both against the source's threat model. A tool that protects content while exposing the fact of communication is appropriate for some threats and catastrophic for others. There is no universally safest tool; there is only the tool that fits the specific source's specific risks.

### Minimize Metadata And Persistent Records

Metadata, who contacted whom, when, from where, and how often, often exposes sources even when content is encrypted. Phone billing records, email logs, app contact lists, device location history, and cloud backups can all reveal a relationship that the participants believed was protected. Minimize metadata exposure by choosing channels that collect less of it, by avoiding channels tied to real identities, and by being deliberate about timing and patterns that could reveal the relationship.

Persistent records are another exposure. A conversation that disappears from the reporter's app may persist on the source's monitored device, in the source's employer's logs, or in a backup. Discuss with the source what records the channel creates on their end, and configure tools to minimize persistence where the source's environment allows. A channel that is secure on the reporter's end but leaves exposing records on the source's end has not protected the source.

### Verify Identity Without Exposing It

Encrypted channels can be intercepted by man-in-the-middle attacks if the participants do not verify each other's identities. Verify identity through safety numbers, key fingerprints, or out-of-band confirmation, so that the encryption actually protects the intended parties. But verify in a way that does not itself expose the source: do not send a safety number through an unencrypted channel, do not ask the source to take steps that create records on a monitored device, and do not require the source to identify themselves in ways that defeat the anonymity the channel is meant to provide.

This is a genuine tension, and it must be navigated deliberately. For some sources, identity verification is essential and the channel must support it safely; for others, such as anonymous secure drop submitters, verification may be impossible and the reporting must proceed on the strength of the material itself. See the confidential-sources skill.

### Maintain The Channel Securely Over Time

A channel set up securely degrades if not maintained. Apps update and may change security properties; devices are replaced and may carry different exposures; the source's circumstances change and may alter what is safe; patterns of communication accumulate and may become identifiable. Maintain the channel deliberately: revisit the tool choices periodically, confirm that the source's environment has not changed in ways that create new exposure, and vary communication patterns to avoid creating a detectable signature.

For long-term sensitive relationships, build in periodic security reviews with the source, conducted in a way that does not itself expose the relationship. A channel that was secure at setup may be insecure a year later because the threat environment changed while the channel did not.

### Plan For Compromise Of Any Link

No channel is perfectly secure, and a defensible setup plans for the possibility that any link, the source's device, the reporter's device, the tool itself, or the network, is or becomes compromised. Plan what happens if a device is seized, if a tool is found to have a vulnerability, if metadata is subpoenaed, or if the source is identified through other means. Minimize what each compromise would expose through compartmentalization, and have a protocol for notifying sources and editors if a compromise occurs.

Planning for compromise is not pessimism; it is the recognition that security is probabilistic. A channel designed assuming no link will fail is fragile; one designed assuming some link may fail limits the damage when it does. See the source-protection skill.

## Common Traps

### Imposing A Sophisticated Tool The Source Cannot Safely Use

A tool that the source cannot install, understand, or use without creating exposure on a monitored device worsens their risk. Match the tool to the source's real environment.

### Securing Content While Leaking The Relationship

End-to-end encryption protects content but often reveals that communication occurred. In contexts where the relationship itself is dangerous, metadata exposure defeats the encryption.

### Ignoring Persistent Records On The Source's End

A channel secure on the reporter's end may leave exposing records on the source's monitored device. Discuss and minimize what persists on both ends.

### Skipping Identity Verification

Encrypted channels can be intercepted without identity verification. Verify through safety numbers or out-of-band confirmation, in a way that does not itself expose the source.

### Letting The Channel Degrade Over Time

A secure setup degrades as tools, devices, circumstances, and patterns change. Maintain the channel deliberately and revisit it periodically.

### Assuming No Link Will Fail

Security is probabilistic. A channel that assumes no compromise is fragile; one that plans for compromise of any link limits the damage when it occurs.

## Self-Check

Before treating secure communications setup as sound, verify:

- The whole channel was designed end to end, from first contact through ongoing communication to material handling, not just the encryption of a single app.
- The tool was matched to the source's real environment, capabilities, and risk, including whether its presence on the source's device creates exposure.
- What each tool protects and what it reveals, including metadata and persistent records, is understood and weighed against the source's threat model.
- Metadata and persistent records are minimized on both ends, with deliberate attention to what the channel creates on the source's monitored device.
- Identity is verified through safety numbers or out-of-band confirmation, in a way that does not itself expose the source.
- The channel is maintained over time, with periodic security reviews and adaptation as tools, devices, circumstances, and patterns change.
- A plan exists for compromise of any link, including device seizure, tool vulnerability, metadata subpoena, or source identification, with compartmentalization limiting the damage.
- The channel fits the specific source's specific risks rather than reflecting the reporter's preferred toolkit.
- No tool choice prioritizes sophistication over the source's actual safety.
- Secure communications setup supports editorial judgment; sensitive channel design should involve a responsible editor and, for high-risk sources, specialist security advisors, recognizing that no channel is perfectly secure and that the source's safety may require legal counsel input on the limits of protection.
