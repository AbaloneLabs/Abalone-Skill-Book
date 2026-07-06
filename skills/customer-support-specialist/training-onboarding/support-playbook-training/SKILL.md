---
name: support-playbook-training.md
description: Use when the agent is creating, reviewing, or delivering support playbook training for workflows, escalation paths, troubleshooting, billing, account access, incidents, complaints, QA, macros, or other repeatable support work where agents need judgment, not rote memorization.
---

# Support Playbook Training

A playbook is only useful if agents know when to use it, when to stop, and when to escalate. Playbook training often becomes rote steps, which fails in messy customer cases. This skill helps the agent train support workflows as decision systems with boundaries, examples, and feedback.

## Core Rules

### Teach intent before steps

Start with what the playbook is for, what risk it controls, what customer need it serves, and what outcome it should create. Steps make more sense when agents know the judgment problem.

Do not teach a flow as button clicks alone.

### Define entry and exit criteria

Agents need to know when the playbook applies, when it does not, when to escalate, and when the case is complete. Include exclusions and stop conditions for safety, privacy, legal, fraud, access, and billing risk.

Most playbook failures happen at the boundaries.

### Use realistic branching cases

Training should include messy examples: missing evidence, angry customer, partial eligibility, conflicting records, unverified requester, prior promise, outage signal, and cross-team dependency. Ask agents to explain decisions.

Rote examples do not prepare agents for real queues.

### Teach recognition, not memorization

Agents must learn the signals that make a playbook relevant: customer intent, account state, product state, risk markers, prior contact history, contractual terms, and missing facts. If the training only teaches the playbook name and sequence, agents will force the wrong workflow onto cases that merely look familiar.

Include near-miss cases where the playbook should not be used. For example, a password issue may be ordinary access help, account takeover risk, enterprise SSO misconfiguration, or a privacy request depending on evidence. Training should make those differences visible.

### Connect tools, policy, and communication

A playbook should show which tools to check, which policy source applies, what to document, what to tell the customer, and what to avoid saying. Agents need the whole workflow, not isolated instructions.

Tool steps without communication guidance create customer confusion.

### Include handoff and documentation standards

Train what an escalation packet, internal note, customer update, and closure note should contain. A playbook that ends at "escalate" is incomplete.

The next owner should inherit context, not a mystery.

### Practice judgment under constraints

Include time pressure, queue volume, missing tool access, customer emotion, and ambiguous policy. Agents should learn what to do safely when the ideal path is blocked.

Real support work rarely follows the ideal version.

### Include failure-mode drills and update playbooks from feedback

Practice should include what to do when the playbook breaks: evidence conflicts, a tool is down, the customer cannot complete verification, the article is stale, an upstream team misses its SLA, an automation sends the wrong message, or the customer reports harm that the normal process does not address.

The drill should identify the safe next action, the customer update, documentation, escalation owner, and any promise that must be avoided. This matters because agents often over-rely on the playbook exactly when the case requires judgment.

Use QA findings, repeat contacts, escalations, incident reviews, product launches, policy changes, and agent questions to improve playbooks. Track version changes and retire old guidance.

A playbook that is not maintained becomes a source of defects.

### Verify training transfer and define ownership and version control

Check whether agents apply the playbook correctly in live cases through nesting, QA, side-by-side review, and outcome metrics. Completion of training does not prove behavior change.

Training succeeds when customer outcomes improve.

A playbook needs a named owner who can approve changes, resolve contradictions, and retire outdated versions. Support, product, policy, legal, operations, and QA may all influence the content, but agents need one trusted source of truth.

Version control should show what changed, when it applies, whether old cases are grandfathered, and which macros, bot flows, QA criteria, and help center articles were updated. If playbooks are copied into local notes or outsourced-team documents, include a way to reconcile those copies.

## Common Traps

- Teaching only tool clicks and not customer intent.
- Omitting entry, exit, and escalation criteria.
- Using only clean examples; failing to teach signals that distinguish similar-looking workflows
- Skipping near-miss cases where the playbook should not be used; failing to explain what agents should say to customers
- Ending playbooks at "send to another team."; ignoring documentation quality
- Assuming agents can improvise safely when tools are unavailable; practicing only the ideal flow and not what to do when tools, evidence, or upstream teams fail
- Leaving ownership split across teams so contradictions persist; leaving outdated versions available
- Measuring training success only by attendance or quiz completion; not using QA and repeat-contact data to improve the playbook

## Self-Check

- Does training explain the playbook's intent, risk, customer need, and desired outcome?
- Are entry, exit, exclusion, stop, and escalation criteria clear?
- Are branching examples realistic and messy?
- Are recognition signals and near-miss non-use cases included?
- Do agents practice explaining their decisions?
- Are tools, policy, documentation, customer communication, and forbidden wording connected?
- Are handoff, escalation packet, internal note, customer update, and closure standards included?
- Does training cover blocked ideal paths, time pressure, emotion, volume, and ambiguity?; do drills cover conflicting evidence, unavailable tools, failed verification, stale guidance, missed upstream SLAs, automation mistakes, and customer harm?
- Are QA, repeat contacts, escalations, incidents, launches, policy changes, and agent questions used to update the playbook?; is there a named playbook owner and version history for changed guidance?
- Are old versions retired?; is live application verified beyond training completion?
