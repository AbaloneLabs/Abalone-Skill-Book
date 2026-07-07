---
name: guidance_and_next_actions.md
description: Use when the agent is designing guidance within empty states, error states, onboarding, or task flows, deciding what next action to surface, writing calls to action, choosing between primary and secondary actions, or ensuring users always have a clear path forward.
---

# Guidance And Next Actions

Guidance is the difference between a screen that describes a situation and one that resolves it. Whenever an interface reaches a decision point, an empty state, an error, a completion, or a moment of uncertainty, the user needs to know what to do next. Designs that explain without guiding leave users stranded; designs that guide without explaining breed mistrust. The craft is surfacing the right action, at the right prominence, with the right context.

Use this skill before designing or reviewing any state that requires user direction: empty states, error and failure states, success and completion states, onboarding steps, permission denials, dead ends, and decision points within a task flow. The goal is to prevent the agent from leaving users without a path forward, or from burying the most important action under secondary options and generic help links.

## Core Rules

### Always Provide A Clear Next Action

No screen that the user lands on should be a dead end. Whether the state is empty, errored, complete, or blocked, the user must have at least one concrete action that moves them forward or back to a productive state.

For every such state, ensure there is:

- a primary action that represents the most likely helpful next step;
- a way to go back, cancel, or return to a previous state;
- an escape path when the current state is unrecoverable, such as contact support or start over.

A screen that explains a problem but offers no action forces the user to invent their own recovery, which most cannot do.

### Rank Actions By Likelihood And Consequence

When multiple actions are possible, they should not appear equally weighted. The design should make the most useful next step obvious and demote alternatives.

Ranking principles:

- the most likely helpful action is the primary, visually prominent control;
- secondary alternatives are present but less emphasized;
- destructive or risky actions are separated and require confirmation;
- "learn more" or "contact support" are last resorts, not primary actions;
- the action that resolves the cause ranks above the action that merely explains it.

When every button looks the same, the user must read and compare all of them, which slows everyone and errors the hurried.

### Match The Action To The Cause Of The State

The right next action depends on why the user is in this state. A generic "OK" or "Continue" fails because it does not address the specific situation.

Match action to cause:

- empty first-run state: "Create your first item" or "Add data";
- no search results: "Clear filters" or " broaden your search";
- validation error: "Fix the highlighted field" with the field in focus;
- connection failure: "Try again" or "Check your connection";
- permission denied: "Request access" or "Switch account";
- destructive confirmation: the specific verb, such as "Delete project";
- task complete: "View result" or "Done" leading to a sensible destination.

The action should be the natural answer to "what do I do about this?"

### Make The Primary Action Visually Distinct

The primary next action should be recognizable without reading. Users under stress or in a hurry should find it instantly.

Express prominence through:

- a filled or high-contrast button versus outline or text links for secondary actions;
- consistent placement, such as bottom-right or end of flow;
- larger size and clear labeling for the primary control;
- spacing that separates it from secondary and destructive options;
- restraint in the number of equally prominent buttons.

Two filled buttons of equal weight force a careful read; one primary and one secondary let the user act fast.

### Provide Recovery For Destructive And Blocking States

States that block progress or involve loss need stronger guidance and recovery paths, because the cost of a wrong action is higher.

For blocking and destructive states:

- explain the consequence of each action in plain terms;
- offer undo, restore, or a grace period where possible;
- confirm before irreversible actions, with the specific outcome stated;
- provide a clear way to back out without losing work;
- preserve the user's input and context so they do not restart from zero.

A blocking state with no recovery path turns a recoverable situation into data loss.

### Guide With Context, Not Just Commands

An action label alone may not be enough for the user to decide confidently. Brief context helps the user understand what will happen and why it is the right step.

Add context by:

- a one-line explanation of what the action does or produces;
- the expected result so the user can confirm success;
- any caveat, such as "This cannot be undone" or "This may take a few minutes";
- the reason the action is recommended in this state.

Context builds the confidence to act; a bare command breeds hesitation.

### Anticipate The State After The Action

Designing the next action means also designing what comes after it. The user should not be dropped into an unexpected or empty place.

Plan the post-action state:

- after "Create," land the user on the new item, not a blank list;
- after "Fix errors," return focus to the field that needs correction;
- after "Retry," show clear progress and success or failure;
- after "Complete," go to a sensible next step, not a dead end;
- after "Cancel," return to the previous state with context preserved.

An action whose result is surprising undermines trust in every future action.

### Keep Guidance Honest About Limitations

Sometimes the best action is to tell the user that the current path cannot proceed and point them elsewhere. Pretending a recovery exists when it does not is worse than admitting the limit.

Be honest when:

- a feature is unavailable in the user's plan or region;
- a required integration or permission cannot be granted by the user;
- data is permanently lost and cannot be restored;
- the task requires an action outside the product, such as contacting an admin.

Honest guidance, even when it is "you need to do X elsewhere," respects the user more than a fake recovery that fails.

## Common Traps

### Explanation Without Action

A state that describes the problem but offers no button leaves the user stuck. Every state needs a path forward.

### Equally Weighted Buttons

Multiple filled buttons of the same size force careful reading and invite wrong choices. Rank actions by prominence.

### Generic Labels Like OK Or Continue

"OK" and "Continue" do not tell the user what will happen. Use specific verbs that name the outcome.

### Primary Action That Does Not Match The Cause

Offering "Learn more" as the main action on an error state sends the user to documentation instead of fixing the problem. Match the action to the cause.

### No Recovery For Blocking States

A permission denial or validation error with no way to fix or back out traps the user. Recovery paths are mandatory.

### Surprising Post-Action State

An action that lands the user on an unexpected screen, or discards their context, makes the action feel broken even if it succeeded.

### Fake Recovery

Offering a "Restore" option that does not actually restore, or a "Fix" that cannot fix, destroys trust faster than admitting the limit.

## Self-Check

- [ ] Every empty, error, blocked, and completion state offers at least one clear next action plus a way back or out.
- [ ] Actions are ranked by likelihood and consequence, with the most helpful step visually primary and alternatives demoted.
- [ ] The primary action matches the specific cause of the state, using a specific verb rather than a generic label.
- [ ] The primary action is visually distinct from secondary and destructive actions through fill, size, placement, and spacing.
- [ ] Destructive and blocking states explain consequences, offer undo or restore where possible, and preserve user input and context.
- [ ] Each action includes brief context: what it does, the expected result, and any caveats such as irreversibility or delay.
- [ ] The state after the action was designed: the user lands on a sensible, non-empty destination with context preserved.
- [ ] Guidance is honest about limitations and does not offer fake recovery or actions that cannot succeed.
- [ ] "Learn more" and "contact support" are last-resort secondary actions, not the primary path.
- [ ] The guidance was reviewed from the perspective of a hurried or stressed user who must act without reading everything.