---
name: cyber-physical-risk-and-logistics-scam-response.md
description: Use when the agent is handling logistics scams, cyber-physical fraud, phishing that affects shipments, fraudulent carrier changes, fake invoices, compromised emails, payment redirection, shipment diversion, TMS/WMS access risk, or incident response across cyber and operations teams.
---

# Cyber Physical Risk And Logistics Scam Response

Modern logistics scams often begin digitally and end physically. A compromised email can redirect a shipment, a fake carrier portal can harvest credentials, a fraudulent invoice can change bank details, or stolen load data can support a fictitious pickup. Agents often separate cybersecurity from transportation operations, leaving gaps between IT alerts and freight controls. This skill helps the agent recognize cyber-physical risk and coordinate response across logistics, finance, security, IT, and providers.

## Core Rules

### Treat Digital Changes As Physical Risk

Shipment diversion, carrier substitution, pickup appointment change, bank detail change, delivery address change, document resend, password reset, or urgent access request can all be fraud signals. If a digital message changes who gets freight, who gets paid, or who can see load details, verify through trusted channels before acting.

Do not rely on email continuity. A thread can be compromised. Use known phone numbers, carrier portals, TMS records, contract contacts, or separate verified channels. For high-value loads, require dual approval for changes to carrier, destination, pickup, or payment information.

### Control System Access And Data Visibility

TMS, WMS, yard systems, carrier portals, load boards, EDI/API credentials, customer portals, and shared drives may expose shipment data. Limit access by role and remove inactive users quickly. Use multi-factor authentication where available, strong credential practices, and vendor access controls. Temporary workers, contractors, and 3PL users should not keep access after their need ends.

Load data should be treated as sensitive. Commodity, value, pickup time, trailer number, route, customer, and delivery address can enable theft. Avoid exporting broad spreadsheets or forwarding detailed load lists unnecessarily. Monitor unusual access or download patterns where systems support it.

### Recognize Common Scam Patterns

Common patterns include fake carrier onboarding, double brokering, spoofed emails, lookalike domains, changed bank instructions, fraudulent factoring notices, fake proof of delivery, fake rate confirmations, fake insurance certificates, altered bills of lading, phishing links to load portals, malware attachments, and urgent requests near cutoff times. Scams often exploit pressure and after-hours coverage.

Train teams to pause. A slightly different domain, new phone number, unusual grammar, unexpected urgency, too-good rate, changed remittance, or request to bypass process should trigger verification. Finance, transportation, warehouse, and customer service need shared signals because scams cross boundaries.

### Coordinate Incident Response Across Functions

If compromise or fraud is suspected, involve IT/security, logistics, finance, legal, risk, carrier management, warehouse, customer service, and affected providers. Immediate actions may include freezing payments, stopping pickups, holding freight, disabling accounts, changing passwords, preserving emails, contacting carriers through verified channels, alerting facilities, and notifying customers where appropriate.

Define severity. A phishing email with no click differs from a compromised mailbox that changed delivery instructions for active high-value loads. Prioritize shipments at risk, payments at risk, and accounts with broad access. Time-sensitive freight may require manual verification until systems are trusted.

### Preserve Evidence And Avoid Contamination

Keep suspicious emails, headers, attachments, rate confirmations, invoices, bank-change requests, portal logs, access logs, call notes, shipment records, video, gate logs, and payment records. Do not delete messages or overwrite account access without IT guidance if investigation is needed. Document who discovered the issue and what actions were taken.

External communication should be controlled. Accusing a provider or customer prematurely can damage relationships and legal position. Use factual language: suspected unauthorized change, verification required, shipment on hold, payment under review, or account access being investigated.

### Build Preventive Controls Into Daily Workflow

Prevention includes verified carrier onboarding, bank-change callback procedures, domain monitoring, access reviews, MFA, least-privilege roles, dual approval for sensitive changes, suspicious-change flags in TMS, facility release controls, staff training, and provider security expectations. Controls must be usable during peak volume and after hours.

Run drills or tabletop scenarios: compromised dispatcher email, fake carrier at pickup, changed bank details, TMS credential theft, fraudulent delivery address change, or malware at a 3PL. The best time to discover unclear ownership is before a live load disappears.

## Common Traps

- Treating phishing and invoice fraud as IT issues with no impact on active freight.
- Trusting a reply within an existing email thread without considering mailbox compromise.
- Approving bank, carrier, pickup, or delivery changes through email only.
- Giving broad portal or spreadsheet access to people who only need limited shipment data.
- Missing scam signals because transportation, warehouse, finance, and IT each see only part of the pattern.
- Deleting suspicious emails or changing accounts without preserving evidence.
- Letting urgent cutoff pressure override verification.
- Failing to alert facilities when a fraudulent carrier or pickup may occur.
- Adding controls that work in office hours but fail after hours or during peak season.

## Self-Check

- Are digital changes to carrier, destination, pickup, delivery, payment, and access treated as potential physical risk?
- Are trusted-channel verification rules defined for sensitive changes?
- Are TMS, WMS, portals, load boards, shared drives, and EDI/API credentials limited by role and reviewed regularly?
- Is load data exposure minimized for commodity, value, route, pickup, and delivery details?
- Are staff trained on spoofing, lookalike domains, fake carriers, bank changes, fake PODs, and phishing links?
- Is there a cross-functional incident path involving IT/security, logistics, finance, legal, warehouse, and providers?
- Are active shipments and payments triaged quickly during suspected compromise?
- Is evidence preserved across email, logs, documents, calls, gate records, and payments?
- Are external communications factual and controlled?
- Are preventive controls usable during peak volume and after-hours operations?
