---
name: form_layout_and_flow.md
description: Use when the agent is structuring a form, deciding field order and grouping, choosing one-page versus multi-step flows, placing labels and actions, handling long forms, or designing the path a user takes through a form so completion is fast, errors are avoidable, and abandonment is minimized.
---

# Form Layout And Flow

Form layout and flow is the decision of how a user moves through a set of fields to reach completion. It is not arranging inputs on a page; it is sequencing decisions, grouping related questions, choosing where effort is concentrated, and deciding when the user can see the whole task versus one piece of it. Agents tend to dump every field on one screen because it looks complete, or break a simple form into needless steps because progressive disclosure sounds sophisticated. The harm is abandonment: users who see an intimidating wall of fields and leave, or users who lose patience clicking through steps that added no clarity, or users who reach the end and discover an earlier answer changed what came after.

Use this skill before deciding field order, grouping, page breaks, label and action placement, or whether a form should be one step or many. The goal is to prevent the agent from structuring a form around the data model rather than the user's decisions, from splitting or merging steps without reason, or from placing actions and labels in ways that slow completion and invite errors.

## Core Rules

### Sequence Fields By How The User Decides, Not By The Data Model

Field order shapes how easily a user moves through a form. Ordered by the data model, a form asks for internal identifiers before the things the user actually thinks about. Ordered by the user's decision flow, it starts with what they know, moves to what they must decide, and ends with confirmation. The user's logic, not the schema, should drive the sequence.

Sequence by user logic:

- lead with fields the user already knows or has ready (identity, basics);
- group decisions that depend on each other so earlier answers inform later ones;
- place conditional or consequence-bearing choices where their effects are visible;
- put confirmation, agreement, and submission last.

A form that asks for an account number before a name forces the user to context-switch to lookup before they have even introduced themselves.

### Group Related Fields Into Logical Sections

A long form as a flat list is exhausting; the same fields grouped into sections become scannable and approachable. Grouping reflects how the user thinks about the information and lets them complete one cluster of thought before moving to the next. Group by the user's mental model, not by backend entities.

Group effectively by:

- clustering fields that answer one sub-question (contact details, address, payment);
- giving each group a clear, user-language heading;
- keeping groups small enough to complete as a unit;
- ordering groups so earlier ones set up later ones.

A registration form grouped as "About you", "How we reach you", and "Security" reads far better than thirty sequential fields with no structure.

### Choose One-Page Versus Multi-Step By Cognitive Load

Whether a form should be one page or several steps is a decision about cognitive load and commitment, not about length alone. One page lets users see the whole task and scan before committing; multiple steps reduce overwhelm and create momentum but hide the total effort. Each has failure modes.

Choose one-page when:

- the form is short and the whole task fits comfortably in view or scroll;
- users benefit from seeing the full scope before starting;
- fields are independent and order is flexible.

Choose multi-step when:

- the form is long enough that one page overwhelms;
- later steps depend on earlier answers (conditional flow);
- progress creates useful commitment and momentum;
- each step is a coherent unit of decision.

Never split a short, independent form into steps to seem guided, and never cram a long conditional form onto one page to seem simple.

### Make Conditional Logic Visible And Well-Sequenced

When fields appear, disappear, or change based on earlier answers, the form's structure is dynamic, and that dynamism must be handled deliberately. Conditional logic that surprises users (fields jumping in unexpectedly) or that strands them (a later step rendered irrelevant by an earlier answer) breaks flow and trust.

Handle conditionals by:

- placing conditional fields after the answers that trigger them, never before;
- making appearance and disappearance smooth and noticeable, not jarring;
- ensuring earlier answers that invalidate later steps update those steps clearly;
- avoiding deep branching that makes the form unpredictable.

A user who answers "no" and then sees a whole section become irrelevant should understand why, not wonder if the form broke.

### Place Labels And Actions To Support Scanning

Where labels and actions sit determines how fast a user can move through a form. Labels above inputs scan fastest and adapt to long labels and localized text; actions at the end should be ordered by intent. Misplaced labels and actions add friction to every field.

Place for speed and clarity:

- prefer top-aligned labels for scannability and length flexibility;
- keep the primary action at the end, in the path of completion;
- order actions by intent (primary, secondary, destructive), with clear visual hierarchy;
- avoid placing submit where users can hit it accidentally before finishing.

Left-aligned labels work in dense expert forms where vertical space is costly; right-aligned labels rarely justify their scanning cost.

### Show Progress And Scope In Multi-Step Forms

In a multi-step form, users need to know how much remains and where they are. Hiding the total scope makes the form feel endless; showing it lets users pace themselves and decide whether to continue. Progress is a commitment and reassurance tool.

Communicate progress by:

- showing the current step and total steps, or a progress indicator;
- labeling steps meaningfully so users know what is coming;
- allowing review of completed steps and navigation back without losing data;
- making the final step clearly final.

A form that says "Step 2 of 5" with labeled upcoming steps respects the user's need to budget effort; one that says only "Continue" feels like a trap.

### Preserve Data Across Steps, Errors, And Navigation

Users move back and forth, encounter errors, and sometimes leave and return. A form that loses entered data at any of these points punishes the user and causes abandonment. Data preservation must be robust across the full lifecycle of the form.

Preserve data by:

- retaining field values when navigating between steps or correcting errors;
- restoring partial entries on return where appropriate and secure;
- not clearing fields on validation failure except where necessary;
- handling session expiry gracefully, with a path to resume.

### Match Form Length To The Value Exchange

Users tolerate form effort in proportion to the value they receive. A form that asks for a great deal to deliver little feels exploitative; a form that asks for little to deliver high value feels respectful. Length should be justified by the outcome.

Right-size the form by:

- removing fields that are not strictly needed for the immediate purpose;
- deferring optional or nice-to-have data to later, lower-friction moments;
- explaining why sensitive or unusual information is required;
- aligning the ask with the perceived value of completing it.

## Common Traps

### Sequencing By The Data Model

Forms ordered by internal schema ask for identifiers and internals before the basics the user has ready, forcing context-switching and lookup.

### Flat Lists With No Grouping

Long forms presented as ungrouped sequences overwhelm users and make the task feel larger than it is.

### Splitting Short Forms Into Needless Steps

Breaking a simple, independent form into multiple steps adds clicks without aiding comprehension and increases abandonment.

### Cramming Long Conditional Forms Onto One Page

Fitting a long, branching form onto one screen hides the dynamic logic and overwhelms users with irrelevant fields.

### Conditional Fields That Surprise

Fields that appear or vanish without clear connection to the triggering answer make the form feel broken or unpredictable.

### Misplaced Labels And Actions

Right-aligned labels, hidden submit buttons, or primary actions placed where they can be hit early all add friction and invite errors.

### Hidden Progress And Scope

Multi-step forms that do not show how much remains make the task feel endless and erode the user's willingness to continue.

### Losing Data On Errors Or Navigation

Forms that clear entered values on validation failure or step navigation punish users and cause abandonment at the worst moment.

## Self-Check

- [ ] Fields are sequenced by how the user decides, leading with what they know and ending with confirmation, not by the data model.
- [ ] Related fields are grouped into small, clearly headed sections that reflect the user's mental model.
- [ ] One-page versus multi-step was chosen by cognitive load and conditional logic, not by length alone or a desire to seem guided.
- [ ] Conditional fields appear after their triggers, change smoothly, and update later steps clearly when earlier answers change relevance.
- [ ] Labels are top-aligned for scannability, and actions are ordered by intent with the primary action in the path of completion.
- [ ] Multi-step forms show current step, total steps, meaningful labels, and allow back-navigation without data loss.
- [ ] Entered data is preserved across steps, validation errors, and return, with graceful session-expiry handling.
- [ ] Form length is right-sized to the value exchange, with non-essential fields deferred or removed.
