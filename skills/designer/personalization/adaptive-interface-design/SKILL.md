---
name: adaptive_interface_design.md
description: Use when the agent is designing adaptive interfaces, deciding what changes automatically based on context, building rule-based personalization, designing layouts that respond to device or environment, or reviewing whether adaptive behavior helps or confuses users.
---

# Adaptive Interface Design

Adaptive interfaces change themselves based on signals: device, time, location, role, usage history, input method, connection quality, or inferred intent. The judgment problem is not whether the interface can change. It is whether each change reduces the user's effort without stealing their sense of control, predictability, and trust.

Adaptation is powerful when it removes friction the user would otherwise fix by hand. It is harmful when it moves controls, reorders content, hides familiar paths, or changes meaning based on signals the user cannot see, verify, or override. A user who cannot predict what the interface will do next cannot form habits, teach others, or recover from mistakes.

Use this skill before designing or reviewing interfaces that adapt by rules, context, history, or inferred state. The goal is to prevent the agent from shipping adaptation that feels clever in a demo but erodes trust, hides functionality, or changes behavior in ways the user cannot explain or control.

## Core Rules

### Separate Adaptation From Personalization

Adaptation is rule-based and context-driven: the same inputs produce the same change for everyone. Personalization is user-specific and history-driven: the change depends on who the user is and what they did before. Both can be valuable, but they have different risks.

Adaptation risks:

- the rule is wrong for the user's current situation;
- the signal is noisy or delayed;
- the change breaks a learned workflow;
- the user cannot tell why the interface changed.

Personalization risks:

- the inferred profile is stale or wrong;
- the user is locked into a narrow view;
- sensitive inferences leak private information;
- the user cannot reset or inspect what the system learned.

State clearly which kind of change is happening before deciding how aggressive it should be. Context-driven adaptation can be bolder when the signal is reliable and the change is reversible. History-driven personalization should be more conservative and always overridable.

### Make Each Adaptive Change Earn Its Place

Every adaptive behavior adds complexity, test surface, and unpredictability. Do not add adaptation because it is possible. Add it only when it removes a real, repeated cost.

Strong reasons to adapt:

- the user repeatedly performs the same adjustment;
- the environment genuinely changes what is useful;
- a device constraint forces a different layout;
- the user's role changes what actions are available;
- input method changes what controls are reachable.

Weak reasons to adapt:

- it looks impressive in a demo;
- the data is available, so it feels wasteful not to use it;
- a competitor does it;
- it increases an engagement metric that does not reflect user value.

For each proposed adaptation, name the user problem it solves and the failure mode if the signal is wrong. If you cannot name both, the adaptation is probably decorative.

### Prefer Stable Defaults Over Aggressive Inference

Users build muscle memory, teach coworkers, write documentation, and form expectations around a stable baseline. Adaptation that constantly rearranges the interface undermines all of these.

Prefer:

- a strong, predictable default that works for most users;
- optional adaptation that the user can turn on;
- adaptation that adds or emphasizes rather than removes;
- changes that are easy to notice and easy to reverse.

Be cautious with:

- hiding controls the user used recently;
- reordering navigation based on inferred frequency;
- changing labels, icons, or meanings based on context;
- removing fields or options because they were rarely used.

A safe pattern is to promote likely-useful content while keeping everything else reachable through a stable path. The user gains a shortcut without losing the map.

### Expose The Signal And Let The User Correct It

Adaptation fails silently when the user cannot tell what the system thinks it knows. If the interface adapts based on location, role, history, or inferred intent, the user should be able to see the basis and correct it.

Expose:

- what context the system detected;
- what the system inferred from history;
- what the current adapted state is;
- how to change or reset it.

This is not only a courtesy. It is how users recover from wrong adaptation. If the interface shows the wrong location-based content and the user cannot tell why, they may assume the product is broken. If they can see and correct the detected location, the same behavior becomes useful.

Correction paths should be lightweight. Forcing a user through deep settings to undo a wrong inference is worse than no adaptation.

### Preserve Predictability For Critical And Repeated Work

Some work must be predictable even when the rest of the interface adapts. Safety-critical actions, financial confirmations, administrative operations, frequent professional workflows, and accessibility paths should not be silently reordered, hidden, or relabeled.

Predictability matters most when:

- the user is under time pressure;
- the action is destructive or irreversible;
- the user learned the workflow from someone else;
- the user returns after a long absence;
- the user has a disability that depends on stable structure.

For these cases, keep placement, naming, and ordering stable. Adaptation can add shortcuts, but it should not remove the canonical path.

### Handle Missing, Conflicting, And Stale Signals

Real signals are imperfect. Location may be wrong, role may be ambiguous, history may reflect a temporary task, and connection quality may fluctuate. Adaptive design must define behavior when the signal is absent, weak, or contradictory.

Define explicitly:

- the fallback when no signal is available;
- the behavior when signals conflict;
- how often stale data is refreshed;
- what happens when the signal changes mid-task;
- whether a started task is preserved when adaptation triggers.

Never let a missing signal produce a broken or empty interface. The default state must always be usable.

### Test Adaptation Across The Full State Space

Adaptive interfaces have far more states than static ones. A layout that adapts on three signals has many combinations, and most teams only test the common paths. The failures live in the rare combinations: the user on a slow connection, with a stale role, mid-task, on an unusual device.

Enumerate the states that matter:

- first use versus returning use;
- strong signal versus weak signal versus no signal;
- signal that changes during a task;
- conflicting or rapidly fluctuating signals;
- users who deliberately override the adaptation.

Decide which combinations must be graceful and which can degrade. Document the decision so reviewers and engineers share the same expectation.

## Common Traps

### Adapting To Increase Engagement Rather Than Value

Adaptation tuned to clicks, dwell time, or session length can surface content the user finds hard to ignore but does not actually want. This erodes trust and can feel manipulative. Tie adaptation to user-stated or task-completed value, not to attention metrics.

### Hiding What The User Just Used

Frequency-based personalization often hides controls the user used once and did not need again, or promotes a control used during a temporary task and then never removes it. Both feel arbitrary. Distinguish temporary patterns from durable preferences.

### Silent Reordering That Breaks Muscle Memory

Moving a frequently used action because a different action became slightly more frequent can break a workflow the user performs dozens of times a day. Small frequency shifts should not trigger large structural changes.

### Assuming The Signal Is Correct

Location, role, device, and history are all fallible. Designing as if the signal is always accurate produces interfaces that work in testing and fail in the field. Always design the correction and fallback path.

### Letting Adaptation Obscure The Default

When the adapted view becomes the only view, new users cannot tell what the baseline is, and users who want the standard experience cannot find it. Keep the canonical, un-adapted experience reachable.

### Changing Behavior The User Cannot Explain

If a user opens a support ticket asking why their screen looks different from a colleague's, and the honest answer is opaque inference, the adaptation has failed. Users should be able to understand, at a high level, why the interface behaves the way it does.

## Self-Check

- [ ] Each adaptive change is justified by a specific user problem and has a defined failure mode when its signal is wrong.
- [ ] The design distinguishes context-driven adaptation from history-driven personalization and treats their risks differently.
- [ ] A strong, stable default exists and remains usable and reachable when adaptation is off or unavailable.
- [ ] The user can see what signal or inference drove an adaptation and can correct or reset it without deep navigation.
- [ ] Critical, destructive, repeated, and accessibility-sensitive workflows remain predictable and are not silently reordered or hidden.
- [ ] Behavior is defined for missing, weak, conflicting, and stale signals, including changes that occur mid-task.
- [ ] The adapted state and the canonical default are both testable, and rare state combinations have defined graceful behavior.
- [ ] Adaptation adds or emphasizes useful content rather than removing the user's familiar map of the interface.
- [ ] No adaptation relies on engagement metrics that conflict with the user's actual goals or sense of control.
- [ ] A user who sees unexpected behavior can form a reasonable explanation of why it happened and how to change it.
