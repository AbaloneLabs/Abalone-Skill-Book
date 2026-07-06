---
name: admin-change-and-tenant-governance-risk.md
description: Use when the agent is handling admin replacement, lost admin access, departing admin, tenant owner change, disputed administrator control, emergency admin grant, domain-based admin claims, governance lockout, or requests to override tenant administration where risks include account takeover, insider conflict, unauthorized data access, business disruption, legal dispute, weak approval trail, or support granting control to the wrong party.
---

# Admin Change And Tenant Governance Risk

Admin change requests are among the highest-risk account administration cases because they decide who controls the tenant. A requester may be a legitimate employee locked out, a former admin, a new manager, an attacker, a customer in dispute, or a person caught in an internal governance conflict. Agents often want to restore access quickly and may underestimate the harm of granting control to the wrong party. This skill helps the agent handle tenant governance changes with conservative verification and escalation.

Use this skill when a customer asks to replace an admin, recover a tenant after admin departure, grant emergency admin access, resolve lost owner access, change tenant owner, claim a domain, or override existing administrator control. The agent should prioritize authority evidence and auditability over speed.

## Core Rules

### Treat admin control as a sensitive asset

Admin access may allow viewing data, changing billing, inviting users, deleting records, exporting information, modifying security settings, changing integrations, or locking out others. A support-granted admin change can transfer control of the customer's business environment.

Do not treat admin change as ordinary login support. Even if the requester is verified as a person, the question is whether they are entitled to control the tenant.

### Identify the governance scenario

Classify the situation: sole admin left company, admin lost MFA, admin unavailable, company renamed, domain changed, acquisition, reseller-managed account, agency-client split, legal dispute, security incident, or internal power conflict. The scenario determines evidence and escalation.

Avoid one-size-fits-all recovery. A lost MFA case for the existing admin differs from a new employee asking to replace a departed founder.

### Require strong authority evidence

Evidence may include existing admin approval, verified billing owner approval, contractual support contact, domain control proof, company officer documentation, account manager confirmation, legal documents, or company policy path depending on product rules. The evidence must match the risk and policy.

Do not accept business cards, email signatures, urgency, senior title, or public LinkedIn profile as sufficient by themselves. These can help context but are weak authority evidence.

### Watch for dispute and takeover signals

Red flags include request to remove other admins secretly, urgent pressure, inability to contact existing admins, recent employee departure, conflicting requests, unusual domain changes, security incident, unpaid invoices, hostile language, legal threats, or a requester using a personal email for a company tenant.

When red flags appear, pause normal handling and escalate to security, legal, privacy, account management, or leadership as policy requires.

### Avoid revealing sensitive governance details

When a requester is not yet authorized, do not disclose admin names, email addresses, billing details, user lists, security settings, or internal notes. Provide the approved path for verification without exposing information that could help an attacker.

Even confirming that a specific person is or is not an admin can be sensitive in some contexts.

### Preserve existing admins where possible

If an existing admin can approve the change, use that path. If multiple admins exist, avoid overriding governance unless policy allows and evidence supports it. Support should not shortcut customer governance because one requester finds it inconvenient.

If the issue is lost MFA for a current admin, follow account recovery and identity verification. If the issue is organizational authority dispute, use the governance escalation path.

### Document every decision point

Record requester identity, authority evidence, affected tenant, current admin situation as policy permits, red flags, approvals, escalations, decision, changes made, and customer communication. Admin changes should be auditable.

Do not leave critical approval evidence only in chat, call memory, or personal email. The next reviewer must understand why control changed.

### Communicate conservative timelines

Admin governance cases may take longer than normal support because evidence and escalation are required. Explain that the process protects the account and data. Avoid promising same-day resolution before the required review is complete.

Customers may be frustrated, but speed cannot override tenant control safeguards.

## Common Traps

- Treating admin replacement as a normal access reset.
- Confusing verified identity with authority to control the tenant.
- Accepting senior title, email signature, or urgency as enough evidence.
- Revealing existing admin or account details to an unauthorized requester.
- Removing admins or granting new admin access during an internal customer dispute.
- Handling former employee or acquisition scenarios without escalation.
- Making emergency access changes without an audit trail.
- Promising fast resolution before security, legal, or account review is complete.

## Self-Check

- Is the request recognized as tenant governance, not ordinary login support?
- Is the specific governance scenario classified?
- Has requester identity been verified where required?
- Is authority evidence strong enough for the requested admin change?
- Are takeover, dispute, employee departure, domain, legal, or security red flags checked?
- Has the agent avoided revealing admin, billing, user, or security details before authorization?
- Can existing admins approve the change, and has that path been considered first?
- Are security, legal, privacy, account management, or leadership escalations used where required?
- Are evidence, approvals, decision, action, and communication documented for audit?
- Is the customer given a conservative timeline tied to account protection rather than convenience?
