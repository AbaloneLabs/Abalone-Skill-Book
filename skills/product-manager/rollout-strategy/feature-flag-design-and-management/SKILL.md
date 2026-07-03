---
name: feature_flag_design_and_management.md
description: Use when the agent is designing feature flags for a rollout, deciding flag granularity and targeting rules, managing flag lifecycle and kill switches, or preventing flag sprawl and technical debt from long-lived flags.
---

# Feature Flag Design And Management

Feature flags decouple deployment from release, and that decoupling is what makes safe, gradual rollout possible. But flags are also a source of growing technical debt: every flag adds a branch in the code, a state to test, a configuration to manage, and a path that eventually has to be removed. Flag design is an architecture decision with a lifecycle, not a toggle you add and forget.

Agents miss this because adding a flag feels trivial and removing one feels like cleanup that can wait. They add flags liberally during rollout, never define when they should retire, and within a year the codebase is threaded with stale flags whose behavior no one fully remembers. The harm is combinatorial complexity in testing, surprising interactions between old flags, configuration drift across environments, and outages caused by a flag someone forgot was still set. The opposite failure is over-engineering flags for changes that did not need them, adding ceremony and configuration overhead to simple releases.

Use this skill before answering broad questions such as "should we use a feature flag for this", "how should we design the flag", "what targeting rules do we need", "how do we manage flag lifecycle", or "how do we prevent flag sprawl". The goal is to prevent the agent from treating flags as free and forgetting that every flag is debt that must eventually be paid.

## Core Rules

### Decide Whether A Flag Is Needed At All

Not every change needs a flag. A flag is justified when the team needs runtime control over exposure, a kill switch for risk, a gradual rollout, an experiment, or a way to decouple deploy from release for coordination reasons. A change that is low-risk, reversible, and ships in one step does not need a flag and is simpler without one.

Ask what control the flag provides that cannot be achieved by a deploy. If the answer is nothing, skip the flag. Each unnecessary flag adds configuration surface and testing branches for no benefit. Default to no flag, and add one when the control it provides is worth the debt it creates.

### Choose The Flag Type To Match The Purpose

Flags serve different purposes and should be designed accordingly. Release flags control exposure of unfinished or ramping features and should be short-lived. Experiment flags assign users to variants for a defined test period and are removed when the experiment concludes. Ops or kill-switch flags toggle behavior in production to mitigate risk and may be long-lived but should be few and named clearly. Permission flags gate entitlements based on plan or role and are part of the product, not the rollout.

Match the type to the purpose, because the lifecycle and management rules differ. A release flag treated like a permission flag lingers forever; a permission flag treated like a release flag gets removed and breaks entitlements. Name and group flags by type so their intended lifetime is obvious.

### Design Targeting Rules Around The Real Population

Flag targeting defines who gets the feature, and poorly designed targeting produces skewed exposure, surprised users, and confounded experiments. The targeting rules should match the population the team actually wants to reach, whether that is a percentage, a list of accounts, a segment, or a set of attributes.

Define targeting in terms of stable identifiers and clear attributes, and avoid rules that depend on volatile state. Consider whether the assignment should be sticky (the same user always sees the same variant) or dynamic, and choose deliberately, because stickiness affects experiment validity and user experience. Document the intended population so the flag's behavior is predictable rather than emergent from accumulated rules.

### Make The Kill Switch Fast, Clear, And Tested

A flag used as a kill switch must be operable under pressure. If turning it off requires finding the right config, waiting for a cache, or chasing a dependency, it is not a kill switch, it is a hope. The team should know exactly how to disable the feature, how long it takes, and who has permission to do it.

Test the kill switch before relying on it. Many teams assume they can flip a flag and discover under incident pressure that the flag does not cover all entry points, that the off state has its own bug, or that the change takes minutes to propagate. Verify the off path the same way you verify the on path.

### Define The Flag's Lifecycle And Removal Criteria Up Front

Every short-lived flag should have a defined end. The team should know, when the flag is created, what condition triggers its removal: rollout complete, experiment concluded, feature generally available, or a fixed date. Without a removal criterion, flags accumulate indefinitely.

Record the lifecycle in the flag itself or in a registry: owner, purpose, creation date, intended removal trigger, and current status. Schedule removal as a follow-on task, not an aspiration. A flag without a planned death is permanent debt by default.

### Prevent And Periodically Clean Up Flag Sprawl

Flag sprawl is the accumulated technical debt of long-lived, overlapping, and forgotten flags. It increases testing complexity, because every flag multiplies the states the code can be in, and it increases operational risk, because old flags interact in ways no one remembers.

Run periodic flag audits. Identify flags past their intended lifetime, flags that are always on or always off, flags with no owner, and flags whose purpose is no longer clear. Remove the ones that are safe to remove, and document or refactor the ones that must stay. Treat flag cleanup as routine maintenance, not an optional cleanup project that never gets prioritized.

### Test The Flagged Combinations That Can Actually Occur

A flag multiplies the possible states of the code, but most teams only test the default path. The combinations that matter are the ones that can occur in production, including flag interactions where multiple flags are on or off together.

Identify the realistic combinations, especially interactions between flags that touch the same code path, and test those. A defect that only appears when two flags are in a specific combination will surface in production if it is not surfaced in testing. Do not assume the default path is representative of what users will experience.

## Common Traps

### Adding A Flag For Every Change

Treating flags as default adds configuration overhead and testing branches to changes that would be simpler shipped directly. Flags have a cost; use them when the control is worth it.

### Mixing Flag Types

A release flag managed like a permission flag, or an experiment flag left in place after the test, creates confusion about lifetime and behavior, and the flag lingers long after its purpose ended.

### Untested Kill Switch

Assuming a flag can disable a feature without verifying it under pressure leads to the discovery, during an incident, that the off path is broken, slow, or incomplete.

### No Removal Criterion

Flags created without a defined end condition accumulate. Within a year, the codebase carries flags whose purpose and safe state no one remembers.

### Targeting Rules That Drift

Accumulated targeting rules produce emergent behavior where the actual exposed population is not what anyone intended, skewing rollouts and confounding experiments.

### Ignoring Flag Interactions

Testing only the default path misses defects that appear only when multiple flags combine in production-realistic ways, and those defects surface at the worst time.

### Treating Cleanup As Optional

Flag audits that never happen let debt compound until the configuration surface itself becomes a source of outages and a brake on every future change.

## Self-Check

- [ ] A flag is used only when it provides runtime control worth the debt it creates, not added by default to every change.
- [ ] Each flag is typed by purpose (release, experiment, ops, permission) and managed according to that type's lifecycle.
- [ ] Targeting rules are defined in terms of stable identifiers, documented, and produce the intended population without drift.
- [ ] Any flag used as a kill switch has been tested for speed, completeness, and reliability under pressure.
- [ ] Every short-lived flag has a recorded owner, purpose, and removal criterion at creation time.
- [ ] Flag removal is scheduled as a task, not left as an aspiration, and the registry reflects current status.
- [ ] Periodic flag audits identify and remove stale, always-on, or ownerless flags.
- [ ] Realistic flag combinations, especially interactions between flags on the same code path, are tested, not just the default path.
- [ ] Stickiness and assignment behavior are chosen deliberately to support both user experience and experiment validity.
- [ ] The configuration surface is treated as maintained infrastructure, not an append-only pile of toggles.
