---
name: destructive_action_design.md
description: Use when the agent is designing delete, remove, discard, overwrite, reset, revoke, irreversible, or bulk destructive actions, confirmation dialogs, undo, trash and soft-delete flows, and deciding how much friction a destructive operation needs.
---

# Destructive Action Design

A destructive action is one whose effects are hard or impossible to reverse: deleting data, discarding work, overwriting a record, revoking access, removing a member, or wiping a configuration. The design challenge is not adding a confirmation dialog. It is calibrating the right amount of friction for the consequence, making the scope unmistakable, and providing a recovery path when one is possible. Too little friction causes irreversible mistakes; too much friction trains users to click through confirmations without reading them, which is worse than having no confirmation at all.

Use this skill before finalizing any delete, remove, discard, reset, revoke, or bulk operation, and before designing the confirmation, warning, or undo that surrounds it. The goal is to prevent the agent from applying a one-size-fits-all confirmation modal, from using destructive styling on reversible actions, or from making irreversible actions look routine.

## Core Rules

### Match Friction To Consequence, Not To Mechanism

The amount of confirmation an action needs is a function of its consequences, not of the verb in the button. Deleting a draft comment and deleting a production database are both "delete", but they demand wildly different friction.

Calibrate along consequence dimensions:

- reversibility (undo exists, trash exists, soft-delete, or truly permanent);
- scope (one item, a selection, a whole account, an organization);
- blast radius (affects only this user, collaborators, customers, or the entire system);
- recoverability of the underlying work (can it be recreated, re-imported, or is it unique);
- frequency (a rare destructive action warrants more friction than a frequent one).

A reversible action with undo can have low friction. A frequent, low-scope, reversible action may need none. A rare, high-scope, irreversible action needs typed confirmation, clear scoping, and possibly a delay.

### Make The Scope And Target Unmistakable

Users confirm the wrong thing when the confirmation does not tell them exactly what will be destroyed. "Are you sure?" is the failure case. The confirmation must name the target and the scope.

Strong confirmation copy includes:

- the verb in plain terms ("Delete", "Remove", "Discard", not "Confirm");
- the specific target ("Delete project 'Apollo Launch Plan'?");
- the scope when relevant ("This will remove 3 members from the workspace");
- the consequences that are not obvious ("All shared links will stop working", "This cannot be undone");
- the count when bulk ("Delete 47 selected items?").

Avoid generic pronouns. "Delete this file" is weaker than "Delete quarterly-report-final.pdf". The more specific the target, the less likely a user confirms the wrong item.

### Prefer Undo Over Confirmation Where Possible

Confirmation asks the user to predict; undo lets them act and correct. For reversible operations, undo is almost always better than a confirmation dialog, because it removes friction from the common case while preserving safety.

Undo works well when:

- the action is technically reversible (soft-delete, restore, re-send);
- the undo window is meaningful (seconds to days, not microseconds);
- the user can discover the undo affordance (toast, banner, or persistent control);
- the system can reliably reverse the side effects, including notifications already sent.

Undo does not work when the action has external consequences that cannot be unwound (an email already sent and read, a payment already captured, a notification already pushed). In those cases, confirmation before the action is the correct pattern.

### Separate Destructive Actions Visually And Spatially

Destructive actions should not look like primary actions, and they should not sit adjacent to actions a user reaches for frequently. Visual and spatial separation prevents accidental activation.

Practices:

- use a distinct destructive color or treatment only for genuinely destructive actions;
- place destructive actions away from primary actions (below a divider, in a secondary menu, on a settings page rather than the main toolbar);
- do not cluster a destructive button next to a frequently used one;
- avoid putting "Delete" and "Save" or "Delete" and "Edit" as immediate neighbors;
- in lists, keep per-row destructive actions behind a menu rather than always visible.

Reserve destructive styling exclusively for destructive actions. If every secondary button is red, the warning value disappears.

### Design Bulk And Cascading Destruction Carefully

Bulk destructive actions multiply consequence and multiply the chance of including an item the user did not mean to select. They need stronger scoping and clearer review.

For bulk operations:

- show the count and, where feasible, the list of items to be affected before confirming;
- warn explicitly about cascading effects (children, dependencies, shared resources);
- consider a two-step flow: select, then review, then confirm;
- never bury a "select all" near a bulk delete without a clear confirmation of scope;
- distinguish "delete these selected items" from "delete all items matching this filter", which can be far larger than the visible selection.

Cascading destruction, where deleting a parent removes children, is especially dangerous. Make the cascade explicit and, where possible, offer to reassign or archive children instead of destroying them.

### Provide A Recovery Path When One Exists

If an action is reversible through trash, soft-delete, archive, or a retention window, the recovery path should be discoverable, not hidden in an admin setting.

Design recovery so that:

- deleted items are restorable within a stated window ("Items remain in trash for 30 days");
- the user is told, at deletion time, how and for how long they can recover;
- the trash or archive is reachable from a predictable location;
- recovery restores related data (comments, attachments, history), not just the bare record;
- permanent deletion after the window is clearly communicated.

A recovery path that exists but is undiscoverable functions as no recovery path at all.

### Handle The "Last Copy" And "Dependency" Cases

Some deletions are irreversible in practice even if technically reversible, because they destroy the only copy of unique work or break dependencies. These deserve elevated friction even when the system supports undo.

Elevate friction when:

- deleting the only owner or admin of a workspace;
- deleting a record that other records depend on;
- deleting content with no backup, no version history, and no export;
- removing access for a user who is the sole holder of critical context;
- destroying data subject to legal, compliance, or audit retention requirements.

In these cases, consider requiring explicit typed confirmation, a delay before the action completes, or a secondary check by another person.

### Never Make Irreversible Look Routine

The visual and interaction treatment of an irreversible action should signal its weight. A permanent deletion that looks like a normal button trains users to click it as if it were reversible. If an action cannot be undone, the interface should make that fact inescapable at the moment of commitment.

## Common Traps

### "Are You Sure?" As The Only Confirmation

A generic confirmation with no target, scope, or consequence forces the user to guess what they are agreeing to and trains them to click yes automatically.

### Over-Confirming Reversible Actions

Adding a confirmation dialog to every minor reversible action teaches users to dismiss confirmations without reading, which destroys the protection for the actions that actually need it.

### Destructive Styling Everywhere

If red buttons are used for cancel, secondary actions, and warnings as well as true destruction, the destructive signal is meaningless and users stop respecting it.

### Destructive And Primary Actions Side By Side

Placing Delete next to Save or next to a frequently used edit control guarantees accidental clicks, especially on mobile and in dense tables.

### Bulk Delete Without Scope Review

Allowing a bulk destructive action to proceed without showing the count or the affected items lets a mistaken selection become a large irreversible loss.

### Hidden Or Undiscoverable Recovery

Supporting trash, archive, or undo but never surfacing it at deletion time means users who would have recovered instead assume the loss is permanent.

### Undo For Actions With External Effects

Offering undo for an action whose side effects (sent emails, captured payments, pushed notifications) cannot be reversed gives false assurance and causes real harm.

### Treating "Delete" And "Archive" As Interchangeable

Conflating permanent deletion with reversible archiving in copy and UI leads users to expect recovery where none exists, or to fear loss where the item is safely archived.

## Self-Check

- [ ] The friction applied to each destructive action matches its reversibility, scope, blast radius, recoverability, and frequency, rather than being uniform.
- [ ] Confirmation copy names the specific target, scope, count, and non-obvious consequences, and avoids generic "Are you sure?" phrasing.
- [ ] Reversible operations prefer undo or trash over pre-confirmation, and undo is only used where side effects can genuinely be reversed.
- [ ] Destructive actions are visually and spatially separated from primary and frequently used actions, and destructive styling is reserved for genuinely destructive actions.
- [ ] Bulk and cascading destructive operations show the affected count and items, make cascading effects explicit, and distinguish visible selection from full-filter scope.
- [ ] Where recovery exists (trash, soft-delete, archive, retention window), it is discoverable at deletion time, restorable from a predictable location, and restores related data.
- [ ] "Last copy", sole-owner, dependency, and retention-bound cases receive elevated friction such as typed confirmation, delay, or secondary review.
- [ ] Irreversible actions are visually and interactionally distinct from routine actions so users cannot commit to them accidentally or casually.
- [ ] The destructive flow was tested with mistaken selections, bulk operations, and mobile or dense contexts where accidental activation is most likely.
