---
name: success_and_confirmation_feedback.md
description: Use when the agent is designing success messages, confirmation dialogs, save confirmations, completion states, undo prompts, receipts, and the feedback that tells users an action succeeded, what changed, and whether it is reversible, including when to confirm before acting and when to confirm after.
---

# Success And Confirmation Feedback

Telling a user that something worked is harder than it looks. Too little confirmation, and the user submits the same form three times because they are not sure it went through. Too much confirmation, and every routine action demands a modal, wearing down the user until they stop reading any of it. The judgment problem is deciding, for each action, how much confirmation is enough: enough to make the user confident and safe, but not so much that the interface becomes a wall of interruptions. Confirmation design is really about calibrating friction to stakes, and most products get the calibration wrong in one direction or the other.

Use this skill before designing success toasts, save confirmations, completion screens, undo prompts, receipts, and pre-action confirmation dialogs. The goal is to prevent the agent from either leaving users uncertain whether their action succeeded or burying them in confirmations they learn to dismiss without reading.

## Core Rules

### Calibrate Confirmation Friction To Stakes And Reversibility

The central decision in confirmation design is how much friction to place in front of an action, and the answer depends almost entirely on the stakes and whether the action is reversible. Low-stakes, easily reversible actions, such as toggling a setting or liking a post, need little or no pre-confirmation; they should feel instant, with light after-the-fact feedback. High-stakes, irreversible actions, such as deleting data, sending a payment, or publishing publicly, need clear pre-action confirmation that makes the consequence explicit.

Map each action on two axes, consequences and reversibility, and choose the confirmation weight accordingly. The goal is to add friction only where the cost of an accidental action is high, and to remove it where the action is safe. Friction in the wrong place slows routine work; missing friction in the right place causes disasters.

### Prefer Undo Over Confirmation For Reversible Actions

For actions that are reversible, an undo mechanism is usually better than a pre-action confirmation. Confirmation interrupts every user to catch the few who are about to err; undo lets everyone proceed quickly and rescues the few who need to. This is why deleting an email with an undo toast feels better than a "are you sure?" dialog before every delete.

Use undo for reversible, moderate-stakes actions such as delete, move, archive, or send. Reserve pre-action confirmation for truly irreversible or very high-stakes actions where even undo cannot help, such as permanent deletion, account closure, or irreversible financial transactions.

### Make Success Specific, Not Generic

"Success" tells the user nothing. A good success message names what happened and, where useful, what changed: "Invoice 1042 sent to acme@example.com," "3 files uploaded," "Your profile is now public." Specific confirmation lets the user verify that the right thing happened, which is especially important for actions with consequences such as sending, publishing, or charging.

Generic confirmations force the user to infer or double-check, which defeats the purpose of the feedback. Include the object, the action, and any key detail the user needs to trust the outcome.

### Confirm What The User Cannot See

Confirmation matters most when the result is not visible. If a user submits a form and the data disappears into a backend with no visible change, they need explicit confirmation that it was received and stored. If a user sends a message and it leaves their view, they need to know it went. When the result is immediately visible, such as an item appearing in a list after creation, the success is self-evident and additional confirmation can be minimal.

Audit each action for whether the user can see the outcome. Where they cannot, make the confirmation prominent and specific. Where they can, keep it light to avoid redundancy.

### Destructive And Irreversible Actions Need Explicit Confirmation

Actions that destroy data, spend money, or cannot be undone deserve confirmation that makes the consequence unambiguous. The confirmation should state what will be lost or changed, use clear language rather than euphemism, and require a deliberate action to proceed. For the most severe actions, consider typed confirmation, where the user must type the name of the thing they are destroying, or a two-step review.

The confirmation should not be easy to dismiss by accident. At the same time, it should not be so elaborate that it is used for routine actions, because then users develop a habit of clicking through without reading, which defeats the protection entirely.

### Show Success At The Right Place And Time

Success feedback should appear where the user's attention is, soon after the action, and persist long enough to be noticed but not so long that it clutters. A toast that appears near the action, stays for a few seconds, and allows the user to act on it, such as clicking undo, is a common effective pattern. A success message buried at the top of a long page, or one that vanishes before the user looks, fails its job.

For important successes, such as a completed purchase or a submitted application, a dedicated confirmation screen or receipt is appropriate, because the user needs a moment to register and possibly save the outcome.

### Provide A Record For Important Actions

For actions with lasting consequences, such as payments, submissions, bookings, or legal filings, the confirmation should be more than a fleeting message. Provide a record the user can return to: a receipt, a confirmation email, an entry in history, or a downloadable summary. This protects both the user, who can prove the action occurred, and the product, which can demonstrate it happened.

A success that leaves no trace is fragile; if the user later doubts whether the action went through, there is nothing to check.

### Avoid Confirmation Fatigue

Every confirmation is a small tax on the user's attention. When the product confirms every routine action, users learn to dismiss confirmations reflexively, which means the confirmations that matter most, the ones protecting against real harm, get dismissed along with the rest. This is the core danger of over-confirmation.

Reserve prominent confirmation for actions where it genuinely protects the user, and keep routine success feedback light and dismissible. The rarity of a confirmation is part of what makes it effective.

## Common Traps

### Confirming Every Routine Action

Pre-action dialogs on safe, reversible actions train users to click through without reading, which undermines the confirmations that actually matter.

### Generic "Success" Messages

Confirmations that do not say what succeeded force users to verify the outcome themselves, defeating the purpose of the feedback.

### Missing Confirmation For Invisible Results

Actions whose outcomes the user cannot see, such as a form submission with no visible change, leave users unsure whether anything happened.

### No Undo For Reversible Actions

Forcing confirmation before every reversible delete, instead of offering undo, adds friction to routine work without adding real protection.

### Weak Confirmation For Irreversible Harm

Destructive actions protected only by an easy-to-miss dialog fail to prevent the accidents they exist to stop.

### Success That Vanishes Too Fast

Toasts that disappear before the user notices them, especially for important actions, leave users uncertain the action completed.

### No Lasting Record

Important actions with no receipt, history entry, or email leave users unable to verify the outcome later.

### Confirmation In The Wrong Place

Success messages far from the action, or at the top of a long page, are missed by users whose attention is elsewhere.

## Self-Check

- [ ] Confirmation friction is calibrated to stakes and reversibility: minimal for safe reversible actions, explicit for irreversible or high-stakes ones.
- [ ] Reversible actions offer undo rather than pre-action confirmation wherever practical.
- [ ] Success messages are specific, naming the object, the action, and key details the user needs to trust the outcome.
- [ ] Actions whose results are not visible have prominent, specific confirmation.
- [ ] Destructive and irreversible actions have explicit, unambiguous confirmation requiring deliberate intent.
- [ ] Success feedback appears where the user's attention is, persists long enough to notice, and offers actions such as undo where relevant.
- [ ] Important actions provide a lasting record such as a receipt, history entry, or confirmation email.
- [ ] Routine actions use light, dismissible feedback to avoid confirmation fatigue.
- [ ] No confirmation is so routine that users learn to dismiss it without reading, and none is so weak that it fails to protect against real harm.
