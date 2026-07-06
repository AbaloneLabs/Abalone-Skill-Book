---
name: user-role-permission-and-seat-change-support.md
description: Use when the agent is supporting user invitations, removals, role changes, permission updates, seat assignments, license changes, group membership, admin delegation, access scope, or team membership requests where risks include unauthorized privilege changes, billing impact, data exposure, broken workflows, weak approval evidence, or confusing login support with account administration.
---

# User Role Permission And Seat Change Support

User and permission changes look routine, but they can expose data, change billing, break workflows, and create audit risk. Agents often treat these requests as simple account edits if the requester sounds legitimate, especially when the change is small. This skill helps the agent handle role, permission, and seat changes as governed account administration, not as casual troubleshooting.

Use this skill when a customer asks to add or remove users, change roles, assign seats, grant admin access, revoke access, update teams, transfer a license, modify group membership, or understand why a user can or cannot access a feature. The agent should verify authority, explain impact, and avoid making irreversible changes without evidence.

## Core Rules

### Identify the request type and scope

Clarify exactly what is being requested: invite, deactivation, deletion, role promotion, role downgrade, seat reassignment, group change, license tier change, feature permission, admin delegation, or access troubleshooting. The risk differs by action. Promoting a user to admin is not the same as resending an invite.

Also identify scope: individual user, team, workspace, tenant, billing account, product module, shared project, or external collaborator. A change that appears local may affect many users if permissions inherit from groups or workspace roles.

### Verify requester authority before action

Check whether the requester is an account owner, admin, billing admin, security contact, delegated manager, or authorized support contact. If the requester lacks authority, provide the approved path rather than asking for informal confirmation in the same thread.

Do not grant privileges because the requester is a senior employee, urgent, polite, or copied on an email. Authority must come from the account's governance model, not social pressure.

### Separate identity verification from authorization

Knowing who the requester is does not mean they are allowed to make the change. Identity verification confirms the person; authorization confirms their rights. Support agents often stop after confirming identity and miss whether the person can administer the tenant.

For sensitive changes, confirm both the requester's identity and their administrative authority. If the account has custom approval rules, follow them even if the tool technically allows the edit.

### Explain billing and entitlement impact

Seat changes can affect invoices, renewals, proration, usage limits, plan eligibility, contractual seat minimums, and feature access. Before making or advising on changes, check whether billing or contract rules apply.

Avoid saying a seat change is free, immediate, reversible, or harmless unless the billing and entitlement systems support that claim. If billing impact is unclear, route to billing or account management.

### Protect data and workflow continuity

Removing or downgrading a user may affect owned files, shared dashboards, integrations, automations, API keys, approvals, scheduled reports, tickets, or customer-facing workflows. Ask whether ownership transfer, data retention, or workflow reassignment is needed before access changes.

Do not assume deactivation preserves all business continuity. For departing admins, orphaned assets can become urgent later.

### Use least privilege and time boundaries

When access is needed temporarily, prefer the least privilege that solves the issue and set an expiration or review point where the product supports it. Avoid broad admin grants for convenience.

If a customer asks for a role that exceeds the user's need, explain the narrower option where available. Support should not redesign the customer's governance, but it should flag obvious over-permissioning.

### Document evidence and customer-facing outcome

Record who requested the change, authority evidence, affected user, action taken, timing, billing or data impact disclosed, and any follow-up owner. Keep notes factual and safe. Avoid including unnecessary personal employment details.

If the change cannot be made, document why and what approved path the customer can follow. Future agents should not restart the authorization analysis from scratch.

### Escalate suspicious or high-risk requests

Escalate if the request involves admin takeover, broad user removal, unusual urgency, disputed ownership, departing employee conflict, legal hold, security incident, account takeover suspicion, regulated data, or conflicting instructions from multiple contacts.

High-risk administration requests can be part of fraud, insider risk, or customer governance disputes. Do not resolve them through speed alone.

## Common Traps

- Treating a role change as routine because only one user is affected.
- Verifying identity but not checking whether the requester is authorized for the change.
- Promoting users to admin to quickly solve a feature access issue.
- Removing a user without considering owned data, integrations, automations, or reports.
- Ignoring billing, seat minimum, renewal, or proration impact.
- Accepting approval from someone copied in an email but not authorized in the account.
- Recording sensitive employment or personal details in ticket notes.
- Handling urgent admin-change requests without checking for takeover, dispute, or security signals.

## Self-Check

- Is the exact action and scope of the user, role, permission, or seat change clear?
- Has requester identity been verified where required?
- Has requester authorization been confirmed separately from identity?
- Are billing, contract, entitlement, renewal, or proration impacts checked or routed?
- Could the change affect data ownership, integrations, automations, reports, approvals, or workflows?
- Is least privilege used where possible?
- Are temporary access needs bounded by expiration or review?
- Are authority evidence, action taken, impact disclosed, and follow-up documented?
- Are suspicious, disputed, broad, or security-sensitive requests escalated?
- Would the account owner and audit reviewer understand why the change was or was not made?
