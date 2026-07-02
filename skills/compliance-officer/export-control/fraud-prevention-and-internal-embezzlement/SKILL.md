---
name: fraud_prevention_and_internal_embezzlement.md
description: Use when the agent is preventing internal fraud and embezzlement, controlling account takeover and payment fraud, designing ACH and wire fraud controls, or applying the three lines of defense to fraud risk inside a financial institution.
---

# Fraud Prevention And Internal Embezzlement

Fraud is both a predicate offense for money laundering and a direct loss risk. Internal fraud and embezzlement, where employees or insiders misuse their position to steal or manipulate, are especially damaging because they exploit trust, access, and control weaknesses. External fraud includes account takeover, payment fraud, ACH and wire fraud, check fraud, and social engineering. The BSA, the US AML Act, Sarbanes-Oxley, and prudential regulator expectations require institutions to manage fraud risk through preventive and detective controls, segregation of duties, and the three lines of defense. Fraud programs that focus only on external attacks miss the insider threat, and programs that treat fraud as separate from AML miss the laundering nexus.

Use this skill before designing fraud controls, assessing insider risk, building account takeover and payment fraud defenses, or applying the three lines model to fraud. The goal is to make the agent think about segregation of duties, dual control, override detection, insider red flags, and the controls that stop both external and internal fraud. A fraud program that relies on trust instead of controls will eventually be exploited.

## Core Rules

### Separate Fraud Prevention, Detection, And Response

Fraud risk management should distinguish controls that prevent fraud, controls that detect it, and the response when it occurs. Relying on only one category leaves gaps.

- prevention: segregation of duties, dual control, approval thresholds, access controls, maker-checker, least privilege, and transaction limits;
- detection: transaction monitoring, anomaly detection, override and exception reporting, reconciliation, and audit trails;
- response: investigation, containment, escalation, loss recovery, SAR filing, and law enforcement liaison.

Each category should have owners, evidence, and testing. A program heavy on prevention but weak on detection will miss fraud that bypasses controls; a program heavy on detection but weak on prevention will chase losses instead of stopping them.

### Apply Segregation Of Duties And Dual Control

The single most important internal fraud control is segregation of duties. No single person should be able to initiate, approve, record, and reconcile a transaction. Where segregation is not feasible, dual control or independent review is required.

Segregation areas:

- payment initiation and approval;
- account opening and account maintenance;
- system access provisioning and access reviews;
- reconciliation and transaction processing;
- vendor setup and payment;
- loan origination and disbursement;
- general ledger entries and reconciliation.

Override of segregation, even by senior staff, should be logged, limited, and reviewed. Overrides are a primary insider fraud vector.

### Control Account Takeover And Payment Fraud

External fraud increasingly targets customer accounts through credential theft, phishing, social engineering, and SIM swapping. Controls must protect both the institution and its customers.

Account takeover controls:

- strong customer authentication, including multi-factor for high-risk actions;
- device and behavioral analytics;
- velocity and anomaly checks on logins and transactions;
- out-of-band verification for changes to payees, limits, and contact details;
- customer education and alerts;
- rapid freeze and recovery for confirmed takeover.

Payment fraud controls for ACH, wire, and real-time payments:

- payee verification and positive pay;
- transaction limits and approval thresholds;
- anomaly detection on value, geography, and timing;
- recall and return mechanisms where available;
- monitoring for mule accounts receiving fraud proceeds.

Speed matters: real-time and instant payments compress the window to detect and reverse fraud.

### Detect Internal Embezzlement Patterns

Embezzlement exploits position and access. Patterns vary by role but share red flags.

Insider red flags:

- employee living beyond apparent means;
- reluctance to take leave or share duties;
- unexplained overrides, after-hours activity, or system access anomalies;
- reconciliations that never quite balance or are repeatedly adjusted;
- customer complaints about discrepancies traced to one employee;
- close personal relationships with vendors or customers;
- manual journal entries with weak justification;
- use of suspense, clearing, or dummy accounts.

Mandatory leave and job rotation are classic controls because they force concealment to surface. Monitor overrides, after-hours access, and exception handling.

### Apply The Three Lines Of Defense

Fraud risk, like AML, should be governed through the three lines model.

- first line: business units that own and manage fraud risk in their processes, including preventive controls and initial detection;
- second line: independent risk management and compliance functions that set standards, monitor, challenge, and aggregate fraud risk;
- third line: internal audit that provides independent assurance over the design and effectiveness of fraud controls.

Each line should have a defined mandate, independence appropriate to its role, and access to data. A common failure is the first line both owning and assessing its own risk without second-line challenge.

### Manage Vendor And Third-Party Fraud Risk

Fraud risk extends to vendors, outsourced operations, and third-party payment processors. A compromised or malicious vendor can enable fraud at scale.

Third-party controls:

- due diligence before onboarding;
- contractual fraud and security obligations;
- access controls and monitoring of vendor activity;
- periodic reassessment of high-risk vendors;
- exit and contingency planning.

### Connect Fraud To AML And SAR Obligations

Fraud proceeds are laundered, and fraud itself is a SAR-triggering offense. Coordinate fraud investigation with AML so that confirmed fraud leads to SAR filing, mule account detection, and proceeds tracing. Do not treat fraud losses as a purely operational matter.

## Common Traps

### Trusting Insiders Instead Of Controlling Them

Long tenure, seniority, or perceived reliability are not controls. Insider fraud is most common among trusted, long-serving staff with broad access.

### Weak Segregation In Small Teams

Small teams struggle to segregate duties, but the answer is compensating dual control or independent review, not abandoning segregation.

### Ignoring Overrides

Overrides and exceptions are where controls are bypassed. Unmonitored overrides are an open door for embezzlement.

### Treating Fraud And AML As Separate

Fraud proceeds are laundered, and fraud triggers SAR obligations. Siloing the functions misses the nexus.

### Slow Response On Real-Time Payments

Instant payment rails leave little time to detect and reverse fraud. Controls must be real-time, not next-day.

### First Line Self-Assessment Without Challenge

When the first line assesses its own fraud risk, optimism bias hides gaps. Second-line challenge is essential.

### No Mandatory Leave Or Rotation

Employees who never take leave or rotate duties can sustain concealment indefinitely. Mandatory leave surfaces hidden fraud.

## Self-Check

- Are fraud prevention, detection, and response each covered with owners, evidence, and testing?
- Is segregation of duties applied to payment initiation, account maintenance, access provisioning, reconciliation, and vendor setup?
- Are overrides logged, limited, and independently reviewed?
- Are account takeover controls in place, including strong authentication, behavioral analytics, and out-of-band verification?
- Are ACH, wire, and real-time payment fraud controls calibrated for speed and anomaly detection?
- Are internal embezzlement red flags monitored, including overrides, after-hours access, and exception handling?
- Is mandatory leave and job rotation enforced to surface concealment?
- Does the three lines model assign clear fraud responsibilities with appropriate independence?
- Are vendors and third-party processors subject to fraud due diligence and monitoring?
- Are confirmed fraud cases coordinated with AML for SAR filing and proceeds tracing?
