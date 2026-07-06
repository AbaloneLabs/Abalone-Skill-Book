---
name: customer-environment-change-risk.md
description: Use when the agent is advising a customer to change settings, permissions, integrations, DNS, API keys, browser configuration, mobile settings, device permissions, network rules, admin controls, production configuration, or third-party app setup during support where risks include causing downtime, weakening security, making unsupported changes, losing data, disrupting other users, or telling a customer to alter an environment the agent does not understand.
---

# Customer Environment Change Risk

Support advice often asks customers to change their own environment: toggle a setting, clear data, rotate a key, change permissions, reinstall an app, adjust DNS, reconnect an integration, or modify a production workflow. These instructions can solve the issue, but they can also create downtime, security exposure, data loss, audit gaps, or impact users who are not in the ticket. Agents often underestimate the blast radius because the step looks small from the product side. This skill helps the agent assess change risk before giving instructions.

## Core Rules

### Identify the environment and blast radius

Before recommending a change, determine whether the customer is in a personal account, shared workspace, enterprise tenant, production system, sandbox, mobile device, admin console, integration, network layer, or third-party platform. Ask who else depends on the setting.

A browser cache reset affects one user. A DNS change, SSO setting, API key rotation, permission change, webhook update, or integration reconnect may affect an organization. The agent should not treat these as equivalent.

### Confirm authority and change ownership

The person in the ticket may not be authorized to change billing, security, admin roles, integrations, production configuration, compliance settings, data retention, or identity-provider settings. Confirm role, admin status, and internal ownership when policy requires it.

If the customer says "my IT team handles this," provide a handoff-ready explanation rather than pressuring the current user to improvise.

### Prefer reversible and low-risk diagnostics first

Use read-only checks, status pages, logs, screenshots, test accounts, sandbox reproduction, temporary filters, or non-destructive refresh steps before asking for persistent configuration changes. When a change is needed, choose the smallest reversible step that can prove or fix the issue.

Do not start with destructive steps such as deletion, reset, reinstallation, key rotation, permission broadening, or production integration rebuild unless the risk is understood and justified.

### Explain consequences before the customer acts

Instructions should state what the change does, who may be affected, whether it is reversible, expected downtime, data risk, security implications, and how to roll back. For high-impact changes, advise the customer to involve their admin, IT, security, developer, or business owner.

Avoid terse commands such as "disable SSO and retry." The customer needs enough context to decide whether the step is acceptable.

### Avoid weakening security to solve convenience problems

Support shortcuts may include disabling MFA, broadening permissions, turning off security tools, sharing admin credentials, making API keys less restrictive, bypassing SSO, relaxing CORS, exposing ports, or using public links. Treat these as high risk.

If a temporary security relaxation is approved by the customer and policy allows it, define the time limit, compensating controls, and rollback owner. Do not normalize insecure settings as the final fix.

### Protect data during reset, deletion, and sync actions

Clearing cache, reinstalling, disconnecting integrations, deleting records, reimporting data, reprocessing webhooks, or rebuilding sync state can remove local data, duplicate records, trigger notifications, or overwrite customer changes. Confirm backup, export, idempotency, and expected side effects where relevant.

If data loss is possible, pause and escalate rather than relying on hopeful instructions.

### Match advice to customer capability

Some customers can safely follow technical instructions; others need their administrator or vendor. Adjust language, but do not oversimplify away risk. If the action requires CLI commands, DNS records, identity-provider settings, API credentials, or production deploys, make ownership explicit.

Good support helps the right person take the right action, not just any person take fast action.

### Document what changed and why

Record recommended changes, customer confirmation, timestamps, expected effect, rollback steps, and unresolved risk. If the customer performs the change outside the session, ask them to confirm outcome.

This record protects future troubleshooting and avoids repeated risky instructions.

## Common Traps

- Giving production configuration instructions based on a screenshot without understanding the tenant or integration architecture.
- Treating admin, security, DNS, SSO, webhook, API key, and permission changes like ordinary preference toggles.
- Asking a non-admin user to perform actions owned by IT, security, finance, or developers.
- Recommending irreversible deletion, reset, or reinstall before lower-risk diagnostics.
- Disabling security controls as a quick test without time limit, approval, or rollback.
- Forgetting that reconnecting an integration can duplicate, reprocess, or overwrite data.
- Not warning that a change may affect other users, customers, automations, or audit logs; providing steps with no rollback path
- Failing to document what was changed during support; leaving the customer with a temporary workaround that becomes a risky permanent configuration

## Self-Check

- Has the agent identified whether the environment is personal, shared, enterprise, sandbox, or production?
- Is the likely blast radius understood before recommending the change?
- Is the requester authorized to make the change, or should an admin, IT, security, developer, or owner be involved?
- Have low-risk, read-only, or reversible diagnostics been tried first where practical?
- Does the instruction explain consequences, reversibility, downtime, data risk, and rollback?
- Are security-weakening steps avoided or tightly bounded by policy, approval, time limit, and rollback?
- Are deletion, reset, reinstall, reconnect, reimport, and sync actions checked for data-loss or duplication risk?
- Is the technical complexity matched to the customer's capability and ownership?
- Are recommended and performed changes documented with timestamps and rationale?
- Has the agent prevented a temporary risky workaround from becoming the final configuration?
