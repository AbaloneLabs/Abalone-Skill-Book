---
name: guided-setup-and-onboarding-support.md
description: Use when the agent is guiding a customer through initial setup, onboarding, account configuration, migration, first-use activation, integration setup, admin rollout, workspace creation, or early product adoption where sequencing, permissions, risk, and customer readiness matter.
---

# Guided Setup And Onboarding Support

Setup support shapes the customer's first real experience of the product. A fast answer that skips sequencing can create broken integrations, wrong permissions, duplicate accounts, billing surprises, failed migrations, or poor adoption. Agents often treat onboarding as a checklist of steps, but customers need a safe path from current state to useful operation. This skill helps the agent guide setup with attention to prerequisites, dependencies, and handoff.

## Core Rules

### Establish starting state and success criteria

Before guiding setup, determine whether the customer is new, migrating, expanding a team, replacing another tool, testing a trial, configuring an integration, or recovering from failed setup. Ask what success looks like: first login, imported data, live payments, invited team, configured security, completed order flow, or launched workflow.

The right setup path depends on the customer's starting point and desired end state.

### Sequence dependencies deliberately

Some setup steps must happen in order: domain verification before SSO, billing before activation, admin roles before invites, data cleanup before import, sandbox test before production, inventory before fulfillment, or permissions before automation.

Do not let the customer perform visible or irreversible steps before prerequisites are ready.

### Clarify roles and ownership

Onboarding often involves account owner, admin, billing contact, IT, developer, finance, operations, legal, vendor, customer success, or end users. Identify who must act and who must approve.

If the customer asking support lacks authority, provide guidance they can share rather than encouraging unsafe workarounds.

### Surface risk before activation

Activation may send emails, charge payment methods, publish content, expose data, invite users, sync records, change DNS, trigger workflows, or affect customers of the customer. Explain these effects before the customer clicks.

For production changes, recommend testing, backup, rollback, or stakeholder notice where appropriate.

### Adapt to customer capability

Some customers need brief confirmation; others need plain-language walkthroughs, accessibility adjustments, developer handoff, or customer success involvement. Match depth to the customer's context without patronizing.

If setup requires technical work beyond support scope, identify the handoff and what information that owner needs.

### Prevent duplicate or fragmented setup

Watch for duplicate accounts, duplicate workspaces, multiple trials, wrong region, wrong marketplace purchase, old billing owner, partial migration, or competing integrations. Fragmentation creates long-term support problems.

Pause and reconcile before adding more configuration on top of a bad foundation.

### Track blockers and next owners

Onboarding can stall on verification, missing permissions, payment failure, legal approval, import error, unsupported requirement, or external vendor. Record the blocker, owner, due date, and customer expectation.

Do not leave setup cases with "waiting" but no accountable next step.

### Confirm first value

The goal is not merely completed configuration. Confirm that the customer can perform the first meaningful task: send a test message, view imported data, complete checkout, invite a user, run a report, or use the configured integration.

First value is the evidence that setup worked.

### Prepare rollback and recovery and bridge from setup to adoption

Setup changes can fail after partial progress. Before higher-risk steps, identify what can be undone, what cannot, and who can help restore a safe state. This matters for migrations, domain changes, SSO, payment activation, user invites, integrations, automations, DNS, and bulk imports.

If rollback is not available, slow down and confirm the customer understands the risk. For production workflows, recommend a test account, sandbox, small batch, backup export, or maintenance window when supported.

After first value, customers often need operational habits: who monitors failures, how users get help, how admins review settings, where logs live, and when to contact support. Provide light handoff guidance so setup does not become an abandoned configuration.

For larger accounts, identify whether customer success, implementation, training, or an admin owner should take over.

## Common Traps

- Giving generic setup steps without asking starting state or success criteria.
- Letting customers activate production workflows before prerequisites are complete.
- Ignoring who has authority to configure billing, security, users, domains, or integrations.
- Failing to warn about emails, charges, data exposure, publishing, sync, or customer-facing effects.
- Treating all customers as equally technical or equally resourced; building on duplicate accounts, wrong regions, wrong plans, or partial migrations
- Sending the customer to IT, finance, or customer success without a handoff packet; recording blockers without owner or date
- Considering onboarding complete when settings are saved but no useful workflow has succeeded; starting migrations, SSO, imports, DNS, or production integrations without rollback or recovery thinking
- Leaving the customer with a configured feature but no adoption owner or monitoring habit; hiding unsupported requirements until late in setup

## Self-Check

- Is the customer's starting state clear: new, migrating, expanding, trialing, integrating, or recovering?
- Are success criteria defined in operational terms?
- Are setup dependencies sequenced before irreversible or visible actions?
- Are account owner, admin, billing, IT, developer, finance, legal, vendor, customer success, and end-user responsibilities clear?
- Are activation risks such as emails, charges, publishing, data exposure, sync, workflows, and customer-facing effects explained?
- Is guidance adapted to the customer's technical ability and accessibility needs?
- Are duplicate accounts, workspaces, trials, regions, marketplace purchases, billing owners, and integrations checked?
- Are blockers recorded with owner, due date, and customer expectation?
- Is the handoff packet sufficient if another team must act?; has first value been confirmed rather than only configuration completion?
- Are rollback, recovery, backup, sandbox, small-batch, or maintenance-window options considered before high-risk setup steps?; is there a bridge from initial setup to ongoing adoption, monitoring, admin ownership, or customer success follow-up?
