---
name: data_breach_detection_and_notification_timing.md
description: Use when the agent is responding to a suspected or confirmed data breach, assessing risk to individuals, managing GDPR 72-hour regulator notification, applying US state breach notification thresholds, or deciding when and how to notify affected individuals.
---

# Data Breach Detection And Notification Timing

A personal data breach is a timed legal event the moment it is detected. Under the GDPR, controllers must notify the supervisory authority within 72 hours of becoming aware, and affected individuals without undue delay when the risk is high. US state laws layer their own thresholds, timelines, and content rules on top. The operational reality is chaotic: incidents are ambiguous, facts emerge slowly, and the clock runs regardless. The most damaging failures are rarely the breach itself; they are late notification, misjudged risk assessment, and missing documentation of decisions.

Use this skill before finalizing a breach response plan, during an active incident, or when reviewing whether a past incident was handled correctly. The goal is to make the agent treat breach response as a disciplined, evidence-generating process with hard deadlines, not as a judgment call to be made under pressure without structure.

## Core Rules

### Define What Counts As A Personal Data Breach

A breach is any security incident leading to accidental or unlawful destruction, loss, alteration, unauthorized disclosure of, or access to personal data. The definition is broader than many teams assume.

Recognize three breach types:

- confidentiality breach: unauthorized access or disclosure, including misdirected emails, exposed databases, or credential theft;
- integrity breach: unauthorized alteration of data, including tampering or corrupting records;
- availability breach: loss of access to or destruction of data, including ransomware, accidental deletion, or system outage affecting personal data.

Ransomware that encrypts personal data is an availability breach and often a confidentiality breach if exfiltration is suspected. Do not assume availability-only incidents are out of scope.

### Establish Awareness And Start The Clock

The 72-hour clock starts when the controller becomes aware, meaning when it has a reasonable degree of certainty that a breach has occurred. Investigating to confirm is permitted, but the clock does not wait for full forensic certainty.

Define awareness operationally:

- a credible alert, report, or finding that personal data may have been affected triggers the response process;
- the incident response team must assess promptly whether a breach is reasonably likely;
- awareness is documented with a timestamp and the basis for the determination;
- prolonged "investigation" that delays awareness beyond a reasonable period is treated as late awareness.

Document the path from detection to awareness. If a regulator challenges timeliness, the record must show when awareness occurred and why.

### Contain And Investigate In Parallel

Containment and investigation must run in parallel with the notification clock. You cannot pause notification while the forensic picture completes.

Run containment by:

- isolating affected systems and revoking compromised credentials;
- preserving evidence before remediation destroys it;
- identifying the scope of data, individuals, and systems affected;
- determining whether data was exfiltrated, altered, or merely made unavailable.

Investigation feeds the risk assessment, which drives notification decisions. Keep investigation notes timestamped and factual, because they become evidence.

### Assess Risk To Individuals Rigorously

Notification to the supervisory authority is required unless the breach is unlikely to result in a risk to individuals. Notification to individuals is required when the risk is high. The risk assessment is the crux of every decision.

Assess risk considering:

- the type and sensitivity of the breached data, with special category and financial data rated higher;
- the identifiability of the data and how easily it can be linked to individuals;
- the severity of consequences, including financial loss, identity theft, reputational harm, discrimination, or physical safety;
- the likelihood of those consequences materializing;
- the number of affected individuals;
- any special characteristics of the affected individuals, such as vulnerability.

Apply encryption and pseudonymization as mitigating factors only where they genuinely reduce risk. Stolen encrypted data with the key compromised is not mitigated.

### Meet The 72-Hour Regulator Notification

Where the risk threshold is met, notify the supervisory authority within 72 hours of awareness. Late notification requires a reasoned explanation of the delay.

Include in the notification:

- the nature of the breach and the categories and approximate number of individuals and records concerned;
- the name and contact of the data protection officer or contact point;
- the likely consequences;
- the measures taken or proposed to address the breach and mitigate its effects.

Where information is not yet available within 72 hours, provide it in phases without further delay. Phased notification is acceptable; silence is not.

### Decide Individual Notification On High Risk

When the breach is likely to result in a high risk to individuals, notify them without undue delay. Individual notification is not optional where the high-risk threshold is met.

Provide individuals with:

- a clear, plain-language description of the breach;
- the likely consequences;
- the measures taken or proposed;
- guidance on what they can do to protect themselves, such as changing passwords or monitoring accounts.

A communication exception exists where the controller has implemented appropriate technical and organizational protections (such as strong encryption rendering data unintelligible), or where notification would involve disproportionate effort followed by a public communication. Do not stretch these exceptions to avoid notifying.

### Reconcile GDPR With US State And Sector Laws

US state breach laws add their own definitions, harm triggers, and timelines. Sector laws such as HIPAA add specific duties for protected health information.

Reconcile by:

- mapping each US state's trigger, which often turns on the type of data (such as Social Security numbers, financial account data, or credentials) and the risk of harm;
- tracking HIPAA's 60-day individual notification duty and its breach risk assessment framework;
- identifying any state-specific attorney general notification duties and content requirements;
- applying the strictest applicable timeline rather than guessing.

A single incident can trigger dozens of overlapping duties. Build a notification matrix keyed to the data types and jurisdictions affected.

### Document Every Decision And Justification

Breach response is heavily scrutinized after the fact. Every decision, especially a decision not to notify, must be documented with its reasoning.

Document:

- detection and awareness timestamps;
- containment and investigation actions;
- the risk assessment and its inputs;
- notification decisions, including decisions not to notify and the basis;
- communications sent and when;
- post-incident remediation.

## Common Traps

### Treating Ransomware As Availability Only

Ransomware increasingly involves exfiltration. Assuming data was only encrypted, not stolen, leads to under-notification that is exposed when stolen data appears online.

### Letting Investigation Delay Awareness

Prolonged investigation used to defer the awareness determination is treated as late notification. Awareness is a reasonable-certainty standard, not full proof.

### Understating Risk To Avoid Notification

Judging risk as low to avoid the 72-hour clock, without a documented assessment, is a frequent and serious error.

### Missing The 72-Hour Window On Phased Reporting

Phased reporting is allowed, but the initial notification must still land within 72 hours. Waiting to assemble a complete picture before any notification breaches the deadline.

### Overusing The Encryption Exception

Claiming data was encrypted to avoid individual notification, when keys were also compromised or encryption was weak, is not defensible.

### Ignoring US State And Sector Overlap

Notifying under GDPR while missing US state attorney general duties or HIPAA timelines creates separate violations.

### Communicating Late Or Vaguely To Individuals

Vague breach notices that omit consequences and protective guidance fail the duty even if sent on time.

### Losing Evidence During Remediation

Wiping systems to restore service before preserving forensic evidence destroys the ability to determine scope and exfiltration.

## Self-Check

- Is the breach definition applied to confidentiality, integrity, and availability incidents, including ransomware with suspected exfiltration?
- Is awareness defined operationally with a timestamp, and does the 72-hour clock start at reasonable certainty rather than full forensic proof?
- Are containment and investigation run in parallel with the notification clock, with evidence preserved before remediation?
- Does the risk assessment weigh data sensitivity, identifiability, severity, likelihood, number of individuals, and vulnerability?
- Where the risk threshold is met, is the supervisory authority notified within 72 hours with nature, consequences, contact, and measures?
- Where high risk exists, are individuals notified without undue delay with clear consequences and protective guidance?
- Are GDPR duties reconciled with US state breach laws and HIPAA through a notification matrix keyed to data types and jurisdictions?
- Are encryption and pseudonymization claimed as mitigations only where they genuinely reduce risk, not where keys are compromised?
- Are communication exceptions applied narrowly rather than used to avoid individual notification?
- Is every decision, including decisions not to notify, documented with timestamps, inputs, and reasoning?
