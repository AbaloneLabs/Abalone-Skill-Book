---
name: organization-workspace-configuration-support.md
description: Use when the agent is helping customers configure organizations, workspaces, teams, domains, SSO settings, groups, shared settings, tenant preferences, feature toggles, environments, workspace limits, or admin-level product configuration where risks include changing settings with broad user impact, confusing support guidance with implementation ownership, breaking integrations, exposing data, or making configuration recommendations without understanding customer governance.
---

# Organization Workspace Configuration Support

Organization and workspace settings shape how many users experience the product. A configuration answer can change permissions, notifications, data visibility, integrations, billing, security, and administrative control. Agents often answer from feature knowledge alone and miss tenant-wide impact. This skill helps the agent support organization configuration with the right caution, context, and ownership.

Use this skill when customers ask about workspace setup, org settings, domain management, SSO-related configuration, team structures, shared templates, feature toggles, groups, environments, notification defaults, or admin preferences. The agent should explain options and risks without taking over customer governance unless authorized.

## Core Rules

### Identify configuration level and blast radius

Determine whether the setting affects one user, team, workspace, organization, domain, environment, integration, or all future users. A small toggle in an admin panel may change visibility, workflow, or access for a large population.

Before advising a change, explain who will be affected and whether the change is reversible. If the blast radius is uncertain, route to a specialist or recommend testing in a limited scope first.

### Clarify the customer's operating model

Configuration should fit how the customer manages teams, security, billing, data, and workflows. Ask who administers the workspace, how teams are structured, whether external collaborators exist, whether SSO or domain verification is used, and whether compliance or audit requirements apply.

Do not recommend a neat structure based only on product simplicity. A small business, enterprise tenant, school, agency, or regulated customer may need different governance.

### Separate support explanation from implementation decision

Support can explain what settings do, prerequisites, known effects, and safe steps. The customer usually owns the decision about how to configure their organization. Avoid making policy choices for them, such as who should be admin, which departments should see data, or whether external sharing should be allowed.

If the customer wants implementation design, route to onboarding, customer success, professional services, or documentation where appropriate.

### Preserve security and identity boundaries

Settings involving SSO, SCIM, domain verification, MFA enforcement, session duration, external sharing, API access, or default permissions can create security impact. Do not advise changes without checking prerequisites, authority, and escalation rules.

For identity-provider changes, be careful about lockout risk. Encourage backup admin access, test user validation, rollback path, and scheduled change windows where policy supports it.

### Check downstream integrations and automations

Workspace settings can affect integrations, webhooks, automations, reports, API tokens, approval flows, notifications, data sync, or third-party apps. Ask whether the customer relies on connected systems before changing shared settings.

If impact is unclear, advise a test or staged rollout rather than a broad immediate change. Breaking a workflow can create more harm than the original configuration issue.

### Use examples without overfitting

It is useful to explain common patterns: separate workspaces by department, environment, client, region, or security boundary. But do not imply there is one best configuration for every customer.

Provide tradeoffs. More workspaces can improve separation but increase administration. Fewer workspaces can simplify collaboration but broaden visibility. The customer needs to choose based on risk and operating style.

### Document configuration guidance and decisions

Record the setting discussed, customer goal, guidance given, risks explained, owner decision, and any follow-up. If the customer changes a setting themselves, document that support advised but did not perform the change where relevant.

Good notes help later agents understand why a configuration exists and prevent contradictory guidance.

### Consider whether users need notice

Some configuration changes alter user experience immediately: login method, notifications, sharing defaults, workspace names, required fields, feature availability, or access paths. Ask whether admins plan to notify affected users before the change. Support should not send internal customer communications without permission, but it should flag when silence will create confusion or repeat tickets.

### Escalate broad or irreversible changes

Escalate when the request involves tenant-wide access, data deletion, domain takeover, SSO lockout risk, regulated data, production integrations, legal hold, account dispute, or unclear admin authority. Broad configuration changes should not be handled as ordinary how-to questions.

If an admin asks for a risky change, the answer may still be yes, but the support path should include appropriate warning, confirmation, and ownership.

## Common Traps

- Treating admin settings as simple how-to guidance without checking blast radius.
- Recommending a workspace structure without understanding customer governance.
- Making customer policy choices under the guise of support advice.
- Ignoring SSO, SCIM, domain, MFA, or external-sharing security implications.
- Advising a tenant-wide change without testing, rollback, or customer admin confirmation.
- Forgetting that integrations, automations, reports, and API tokens may depend on current settings.
- Giving one "best practice" without explaining tradeoffs.
- Ignoring whether affected users need notice before a visible configuration change; failing to document configuration advice, causing later contradictory answers

## Self-Check

- Is the configuration level and affected population clear?
- Has the agent explained blast radius, reversibility, and likely side effects?
- Is the customer's team structure, security model, external collaboration, and admin ownership understood?
- Is support explaining product behavior rather than choosing customer governance for them?
- Are identity, domain, SSO, MFA, API, and external-sharing risks checked?
- Are integrations, automations, webhooks, reports, and notifications considered?
- Are options presented with tradeoffs rather than a universal answer?
- Is broad change tested or staged where appropriate?
- Has user notice or admin communication been considered for visible changes?
- Are guidance, warnings, customer decision, and owner documented?; are tenant-wide, irreversible, disputed, or security-sensitive settings escalated?
