---
name: security_and_privacy_ux.md
description: Use when the agent is designing user-facing security and privacy experiences, including authentication, login and recovery flows, permissions and consent, data sharing and deletion, multi-factor authentication, password and credential management, cookie and tracking consent, privacy settings, and making safe behavior the easy and understandable path for users.
---

# Security And Privacy UX

Security and privacy are user experience problems before they are technical ones. The strongest cryptography fails if the login flow is so frustrating that users reuse passwords or disable multi-factor authentication. The most rigorous privacy policy is meaningless if the consent screen is designed to extract agreement rather than inform. Most security failures and most privacy harms are not breaks in the system; they are predictable consequences of interfaces that make safe behavior hard, confusing, or invisible.

Use this skill when designing any flow where users authenticate, grant access, share data, adjust privacy settings, or make security-relevant decisions. The goal is to prevent the agent from treating security as friction to minimize or privacy as a compliance overlay, and instead to design experiences where the secure and private choice is also the easy, understandable, and respectful one.

## Core Rules

### Make The Secure Path The Easy Path

The central principle of security UX is that users optimize for convenience. If the secure behavior is harder than the insecure one, users will choose the insecure one, every time, and the design is responsible for the resulting risk. Good security UX does not add friction; it makes the safe action the path of least resistance.

Apply this by:

- defaulting to the more secure setting rather than asking users to opt in;
- offering password managers, passkeys, and biometrics that reduce memory burden;
- making multi-factor authentication setup guided and recoverable, not a maze;
- avoiding security questions with guessable or publicly known answers;
- providing session management and logout that are easy to find and use.

When security requires friction, make it proportional, explained, and rare.

### Design Authentication For Humans, Not For Systems

Login and recovery flows are where security and usability collide most visibly. Rules imposed for security, complexity requirements, frequent reauthentication, opaque error messages, often produce the opposite of safety by driving users to write down passwords, reuse them, or abandon the flow.

Design authentication around real human behavior:

- allow paste and password manager use; never block it;
- explain password requirements before submission, not after failure;
- show strength feedback that is honest and actionable;
- offer passkeys and biometrics to reduce reliance on memorized secrets;
- make recovery robust without making it easy to hijack;
- avoid locking users out for minor mistakes with no recovery path.

A login flow that treats users as adversaries produces adversaries, or at least ex-users.

### Make Consent Meaningful, Not Extracted

Consent is the foundation of privacy, and most consent UI is designed to manufacture it rather than earn it. Pre-checked boxes, dark patterns, buried options, and asymmetric button styling extract agreement without informing. Meaningful consent requires that the user actually understands and chooses.

Design consent so that:

- choices are unbundled; unrelated permissions are not grouped into one accept;
- the default is not consent for non-essential data use;
- declining is as easy as accepting, with equal prominence;
- the purpose of each data use is explained in plain language at the point of choice;
- consent can be withdrawn later as easily as it was given.

Consent that is designed to be hard to refuse is not consent, and regulators as well as users increasingly recognize this.

### Explain Permissions And Access At The Moment Of Request

Permissions requested out of context feel intrusive and suspicious. A flashlight app asking for contacts, or a request appearing before its purpose is clear, trains users to either deny everything or accept everything, both of which are bad outcomes. Explain what access enables and why, before or at the moment of asking.

Explain access by:

- stating what the permission enables in concrete terms;
- distinguishing essential from optional permissions;
- requesting permissions in context, when the feature that needs them is used;
- making clear what the permission does not enable, to counter common fears;
- providing a path to grant later if initially denied.

Contextual, explained permission requests earn trust and produce better-informed decisions.

### Design For Data Visibility, Control, And Deletion

Privacy is not only about collection; it is about what users can see, change, and remove. Users trust products that let them inspect their data, correct it, export it, and delete it. Designs that hide these capabilities signal that the product considers the data its property, not the user's.

Provide clear paths to:

- view what data is held and how it is used;
- correct inaccurate information;
- export data in a usable format;
- delete data and understand what deletion covers and its limits;
- manage sharing with third parties and revoke it.

Deletion that is technically impossible to complete should be honest about retention obligations rather than pretending erasure is immediate.

### Communicate Security Events Clearly And Without Alarm

Security events, suspicious login attempts, password changes, data breaches, new device sign-ins, need to reach users in a way that is clear, actionable, and calm. Vague alerts cause panic or are ignored; alarming but unactionable notices train users to disregard real warnings.

Effective security communication:

- states specifically what happened and when;
- explains whether action is required and what action;
- provides a verifiable path to legitimate resolution, not a link in a suspicious email;
- avoids crying wolf with low-value notifications;
- is distinguishable from phishing, so users can trust the channel itself.

The design of security notifications is itself a security feature, because users who cannot tell real alerts from fake ones are vulnerable to both.

### Avoid Dark Patterns That Undermine Safety

Many patterns that improve conversion do so by undermining user safety and privacy: hiding the logout, making account deletion a multi-step maze, using confirm-shaming language on decline buttons, or burying privacy controls. These patterns extract short-term gains at the cost of trust and often of compliance.

Avoid:

- asymmetric styling that pushes users toward data sharing;
- confirm-shaming copy such as "No thanks, I prefer to be unsafe";
- hiding deletion, downgrade, or opt-out behind support contact;
- pre-consenting to sharing or marketing;
- using urgency or fear to push security-relevant decisions.

### Design For Vulnerable And High-Risk Users

Some users face elevated risk: people fleeing abuse, activists, journalists, people in hostile environments, minors. Standard privacy defaults may be inadequate for them. Consider whether the product serves users for whom exposure could be dangerous, and design escape hatches, pseudonymity, and stronger defaults accordingly.

Consider:

- whether accounts can be used pseudonymously;
- whether contact information is exposed to other users;
- whether location or activity is visible by default;
- whether there are paths to block, report, and disappear quickly;
- whether sensitive data is minimized and protected by default.

## Common Traps

### Blocking Paste And Password Managers

Preventing paste in password fields, supposedly for security, actively weakens security by driving users to simple, memorized, reused passwords.

### Dark-Pattern Consent

Pre-checked boxes, bundled permissions, and hard-to-find decline options extract agreement without informed consent and increasingly violate regulation.

### Permissions Out Of Context

Requesting access before explaining its purpose trains users to accept blindly or deny reflexively, both of which are insecure outcomes.

### Vague Or Alarming Security Alerts

Notifications that do not explain what happened or what to do cause panic or are ignored, undermining the value of real warnings.

### Hidden Deletion And Downgrade Paths

Making it easy to sign up and hard to leave signals that the product needs to trap users, which destroys trust.

### Friction Masquerading As Security

Genuine security is usually low-friction. Excessive, unexplained friction often indicates poor design rather than real protection.

### Ignoring High-Risk Users

Defaults that are fine for average users can endanger those facing stalking, harassment, or surveillance, for whom exposure is a safety issue.

## Self-Check

- [ ] The secure behavior is the easy and default behavior, not an opt-in burden.
- [ ] Authentication supports password managers, paste, passkeys, and robust recovery without adversarial friction.
- [ ] Consent is unbundled, informed, equally easy to grant or decline, and withdrawable.
- [ ] Permissions are requested in context with a plain-language explanation of what they enable and do not enable.
- [ ] Users can view, correct, export, and delete their data through clear, honest paths.
- [ ] Security events are communicated specifically, calmly, and actionably, distinguishable from phishing.
- [ ] No dark patterns push users toward sharing, prevent leaving, or use fear and shame to extract decisions.
- [ ] The design accounts for high-risk users who need pseudonymity, minimization, and stronger defaults.
