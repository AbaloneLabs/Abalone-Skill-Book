---
name: device_and_account_security.md
description: Use when the agent is securing phones, laptops, and accounts against surveillance or seizure, choosing strong authentication, setting up encrypted communications, protecting against phishing and device compromise, or building baseline digital security practices for reporting work that may attract targeted threats.
---

# Device And Account Security

A reporter's devices and accounts are the keys to their reporting and their sources. They hold notes, drafts, contacts, messages, documents, and credentials, and compromising them compromises the entire body of work and everyone who trusted the reporter with information. The judgment problem is that digital security is treated as a specialist concern or a one-time setup, when in practice it is a continuous discipline whose gaps are invisible until they are exploited. A reporter who uses a strong password but reuses it, who enables encryption but clicks a phishing link, who secures their laptop but carries sensitive source material on an unencrypted phone, has a security posture full of holes they cannot see. Baseline device and account security is not paranoia; it is the minimum that the duty to protect sources and reporting requires.

Use this skill when setting up or auditing device and account security, choosing authentication and encryption tools, protecting against targeted threats like phishing or device seizure, or building security practices for a beat or investigation. The main risk is that an agent treats security as a checklist to complete rather than as a threat model to maintain, applying tools without understanding the specific risks they address.

## Core Rules

### Build Security Around A Threat Model, Not A Checklist

Effective security starts with a threat model: who might target the reporter, what they want, what capabilities they have, and what the consequences of compromise would be. A reporter covering local government faces different threats than one investigating organized crime, a state intelligence service, or a sophisticated private surveillance firm. The threat model determines which protections matter and which are overkill or insufficient.

Revisit the threat model as the reporting evolves. An investigation that seemed routine may, as it exposes powerful interests, attract targeted surveillance that the original security posture cannot withstand. Build the habit of asking, at each stage, "if this reporting attracts serious attention, what would be exposed, and is that exposure acceptable?" Where the answer is no, raise the posture before, not after, the threat materializes.

### Use Strong, Unique Authentication Everywhere

Every account that touches reporting work should have a strong, unique password stored in a reputable password manager, with multi-factor authentication enabled. Reused passwords mean that one breach compromises many accounts; weak passwords invite automated attacks; missing multi-factor authentication leaves accounts vulnerable to credential theft. These are baseline measures, not advanced precautions.

For the most sensitive accounts, use hardware-based multi-factor authentication rather than SMS or app codes, which can be intercepted or socially engineered. Treat email and password manager accounts as the crown jewels, because compromising them compromises everything else, and apply the strongest available protection. Review periodically for accounts that have been created and forgotten, each an unmonitored entry point.

### Encrypt Devices And Backups

Full-disk encryption on laptops and phones protects data if a device is lost, stolen, or seized. Enable it on every device used for reporting work, with strong passphrases that resist guessing. Encryption protects data at rest; it does not protect data if the device is unlocked when seized, so configure reasonable auto-lock timeouts and shut down devices at checkpoints or in seizure-risk situations where the law permits.

Encrypt backups too, and consider where backups live. A cloud backup of an encrypted device, stored under an account with weak authentication, defeats the device encryption. Align backup security with device security, and for the most sensitive material, consider whether backups should exist at all, since each copy is an additional exposure surface.

### Secure Communications End To End

For any communication that could expose a source, an investigation, or the reporter's movements, use end-to-end encrypted channels: encrypted messaging apps, encrypted email, secure voice and video. Verify the identity of the people you communicate with, through safety-number or key verification, to detect man-in-the-middle interception. Understand which channels are encrypted in transit only, where the provider can read content, versus end-to-end, where only the endpoints can.

Match the channel to the risk and to the source's capabilities. A source who cannot safely install an encrypted app may need a different approach than one who is technically fluent. Imposing a tool that creates a record on a monitored device can worsen the source's exposure. See the source-protection skill.

### Defend Against Phishing And Social Engineering

Targeted phishing is the most common way reporters and sources are compromised, because it bypasses technical security by exploiting human trust. Treat unsolicited links, attachments, login prompts, and requests for credentials with suspicion, especially when they arrive with urgency or apparent relevance to current reporting. Verify the sender through a separate channel before acting on anything sensitive. Use hardware security keys, which resist many phishing attacks that app-based and SMS authentication do not.

Social engineering extends beyond email: a friendly contact who asks leading questions, a recruitment approach that seeks to build a relationship, a fake source who offers material to gain access. Maintain awareness that not everyone who approaches a reporter has benign intent, and verify before trusting. Newsrooms should provide phishing training and simulated attacks, because the skill is perishable and the threats evolve.

### Compartmentalize Sensitive Material

Do not store all sensitive material on a single device or account, where one compromise exposes everything. Compartmentalize: separate devices or accounts for the most sensitive investigations, separate communication channels for different sources, and a clean device for travel into seizure-risk environments that carries no unrelated sensitive material. Compartmentalization limits the blast radius of any single compromise.

For the most sensitive investigations, consider dedicated devices and accounts that are used for nothing else, so that compromise of the reporter's routine communications does not expose the investigation. This is overhead, and it is justified only where the threat warrants it, but where it does, the discipline is what keeps sources safe.

### Plan For Device Seizure And Border Search

In many jurisdictions, devices can be searched at borders or seized during arrest, and the reporter may be compelled to unlock them. Plan for this: carry clean devices into seizure-risk situations, store no sensitive source material on travel devices, and have a protocol for what to do if equipment is taken. Understand the legal framework of the jurisdiction, including whether you can be compelled to disclose passwords and what the consequences of refusal are, and consult counsel where the stakes are high.

A seized device may be imaged, surveilled, or returned with monitoring software. Treat any seized device as compromised and rebuild from known-clean media rather than trusting the returned hardware. See the legal-protection skill for the legal dimensions.

## Common Traps

### Reusing Or Weak Passwords

One breach compromises many accounts. Strong, unique passwords in a manager, with multi-factor authentication, are baseline, not optional.

### Securing The Device But Not The Backup

An unencrypted cloud backup of an encrypted device defeats the encryption. Align backup security with device security.

### Using Transit-Only Encryption For Sensitive Content

Channels encrypted only in transit let the provider read content. Use end-to-end encryption for anything that could expose sources or investigations.

### Clicking Targeted Phishing Through Urgency Or Relevance

Phishing works because it exploits trust and urgency. Verify through a separate channel before acting on anything sensitive, and use hardware keys where possible.

### Storing Everything On One Device

A single compromise exposes everything. Compartmentalize sensitive material to limit the blast radius.

### Carrying Sensitive Material Into Seizure Risk

Devices seized at borders or during arrest can expose sources. Carry clean devices and have a protocol for seizure.

### Treating A Seized Device As Trustworthy After Return

A seized device may be imaged or compromised. Rebuild from known-clean media rather than trusting returned hardware.

## Self-Check

Before treating device and account security as sound, verify:

- Security is built around a current threat model that identifies who might target the reporter, what they want, their capabilities, and the consequences of compromise.
- Every reporting-related account uses a strong, unique password in a manager, with multi-factor authentication, and hardware keys on the most sensitive accounts.
- All reporting devices and backups are encrypted, with strong passphrases and reasonable auto-lock timeouts.
- Communications that could expose sources or investigations use end-to-end encryption, with identity verification, matched to the source's capabilities.
- Phishing and social engineering defenses are in place, including separate-channel verification and hardware keys that resist phishing.
- Sensitive material is compartmentalized across devices and accounts to limit the blast radius of any single compromise.
- A plan exists for device seizure and border search, including clean travel devices and a protocol for compromised equipment.
- The threat model is revisited as reporting evolves, with the posture raised before threats materialize.
- No sensitive source material is carried on devices likely to be seized or searched.
- Device and account security supports editorial judgment; significant security decisions for high-risk investigations should involve a responsible editor and, where warranted, specialist security advisors or counsel, recognizing that no security measure is absolute.
