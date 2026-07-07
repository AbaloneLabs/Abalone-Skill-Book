---
name: configuration_complexity.md
description: Use when the agent is designing complex configuration flows, multi-step setup wizards, integration configuration, workspace onboarding, rule builders, permission matrices, or any surface where many interdependent options must be configured without overwhelming the user.
---

# Configuration Complexity

Configuration is where products gain power and lose users at the same time. A small set of options can unlock real workflows; a tangle of interdependent fields can make setup feel like a test the user is expected to fail. The design challenge is not hiding complexity but structuring it so that each decision feels obvious, each dependency is visible, and the user always knows whether they are done.

Use this skill before designing or reviewing integration setup, workspace or team onboarding, rule and automation builders, permission and role matrices, billing plan configuration, deployment or environment setup, API key and webhook configuration, or any flow with many conditional, dependent, or interdependent options. The goal is to prevent the agent from producing a single long form, an unguided wizard, or a brittle rule builder that breaks the moment a user's case is slightly unusual.

## Core Rules

### Separate Mandatory From Optional From Advanced

Not every field carries the same weight. A configuration surface should make the priority of each decision explicit.

- Mandatory fields block completion and must be gathered before anything else.
- Optional fields improve the setup but are not required to finish.
- Advanced fields tune behavior for specialized cases and should be collapsed or deferred.

When everything looks equally required, users treat everything as optional and skip the parts that matter. When everything looks optional, users feel they must read every field to find the one that is actually required. Rank fields visually and structurally so the user knows where to spend effort.

### Order Decisions By Dependency And Consequence

Configuration has a natural sequence. A choice early in the flow can determine which later fields even appear. Order decisions so that each one unlocks or constrains the next, and so that the user never has to backtrack because a later field invalidated an earlier answer.

Ask:

- Which decisions determine which other decisions exist?
- Which decisions are expensive to change later, such as region, currency, or identity provider?
- Which decisions are cheap to change and can be deferred?
- Which decisions depend on information the user may not have yet?

Put expensive, gating, or foundational choices first. Defer choices that depend on later context. Never make a user commit to a final value before they have the information needed to choose it.

### Make Dependencies And Side Effects Visible

When one field changes another, the user must see it. Hidden dependencies produce silent misconfiguration: the user fills everything in, saves, and discovers later that two fields contradicted each other.

Techniques:

- show or hide dependent fields as their trigger changes;
- disable and explain fields that are not yet applicable;
- preview the resulting behavior when feasible;
- warn when a combination is invalid, redundant, or risky;
- summarize the effective configuration before save.

A rule builder that lets users set mutually exclusive conditions without warning is a bug, not a feature.

### Provide Sensible Defaults And Prefill Where Possible

Long configuration flows lose users. Every field you can prefill or default is a field the user does not have to think about.

Prefill from:

- earlier choices in the same flow;
- existing account, workspace, or profile data;
- common values for the user's segment or plan;
- the previous configuration when editing.

When you prefill, make it clear that the value is a suggestion and let the user change it. A prefill the user cannot override is worse than no prefill because it removes agency.

### Validate Continuously, Not Only At The End

Waiting until the final save to reveal errors punishes users who filled in early fields correctly but hit a problem near the end. Validate as the user moves through the flow:

- mark invalid fields as soon as the value is committed or the field loses focus;
- explain what is wrong and how to fix it;
- prevent progression only when the error actually blocks the next step;
- distinguish hard errors from soft warnings.

Do not block the user from moving forward to read later fields just because an early field is invalid. Let them explore, then fix.

### Show Progress And Completion State

Complex configuration should feel finite. Users need to know how many decisions remain, which are done, and whether the configuration as a whole is valid.

Use:

- a step indicator for linear flows;
- a checklist or status summary for non-linear flows;
- a final review screen that shows the effective configuration;
- a clear distinction between "ready to save" and "saved and active."

Without a completion signal, users either abandon early or save incomplete configurations and blame the product later.

### Support Editing As Well As Initial Setup

Most configuration is edited far more often than it is created. A flow that is pleasant the first time but painful to modify later is half-designed. Editing must preserve context, show the current values clearly, and explain what changes when a value is updated.

Avoid forcing the user to re-run the entire wizard to change one field. Allow targeted edits, and surface the downstream effects of each change.

### Account For Partial, Invalid, And Inherited States

Real configuration is rarely clean. Users inherit settings from a template, a plan, an admin policy, or a previous owner. They start, stop, and resume. They save half-finished work.

Design for:

- resuming an incomplete configuration without losing progress;
- inherited or locked values that the user can see but not change, with an explanation;
- partial validity, where some sections are active and others are not;
- migration when a plan change or feature removal invalidates existing config.

## Common Traps

### The Single Giant Form

One long form with every field visible overwhelms users and hides dependencies. It signals that no one decided what matters first. Break it into ordered, prioritized sections.

### The Wizard With No Memory

A multi-step wizard that loses entered values when the user goes back, or that forces a full restart after an error, trains users to avoid configuration. Preserve state across navigation and validation.

### Invisible Dependencies

When changing field A silently changes field B, the user's mental model breaks. They save, walk away, and discover the mismatch later. Surface dependencies explicitly.

### Treating All Fields As Equal

When mandatory, optional, and advanced fields look identical, users cannot tell where to focus. Equal visual weight produces equal confusion.

### Validation Only At Save

Revealing all errors at the end forces users to re-read the entire flow to fix problems. Continuous validation keeps the user oriented and reduces abandonment.

### No Way To Compare Before And After

Editing configuration without showing what the change will do invites regret. Especially for rules, permissions, and automation, let users preview the effective behavior before committing.

### Assuming The Happy Path Only

Configuration built only for the common case breaks on inherited settings, plan limits, partial saves, and resumed sessions. Design for the messy states, not just the ideal one.

## Self-Check

- [ ] Mandatory, optional, and advanced fields are visually and structurally distinguished.
- [ ] Decisions are ordered so dependencies and expensive choices come first, with deferrable choices later.
- [ ] Dependencies and side effects between fields are visible, and invalid combinations are warned about before save.
- [ ] Sensible defaults and prefill reduce effort without removing the user's ability to override.
- [ ] Validation is continuous, with clear fixes, and does not block exploration of later fields unnecessarily.
- [ ] Progress and completion state are visible, including a final review of the effective configuration.
- [ ] Editing existing configuration is as easy as initial setup and shows downstream effects of changes.
- [ ] Partial, inherited, locked, and migrated states are handled with explanations rather than silent failures.
- [ ] The flow preserves entered values across navigation, errors, and resume.
- [ ] The configuration can be previewed or reviewed before it becomes active.
