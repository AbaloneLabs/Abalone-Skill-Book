---
name: labels_and_naming.md
description: Use when the agent is naming interface elements, features, products, menu items, navigation labels, form fields, settings, status values, or any named concept that users must recognize, recall, and distinguish from alternatives.
---

# Labels And Naming

A name is the handle by which a user finds, recognizes, and trusts a feature. Bad naming does not show up as a crash; it shows up as confusion, support tickets, missed features, and wrong actions. When two different things share a name, or one thing has several names, users cannot build a stable mental model, and every later interaction costs more effort.

Use this skill before naming or reviewing names for navigation items, menu labels, feature names, product names, settings, form fields, status values, categories, tags, roles, plans, or any concept the user must identify repeatedly. The goal is to prevent the agent from choosing names that are clever, internal, ambiguous, or inconsistent, and that erode comprehension over time.

## Core Rules

### Name For Recognition And Recall, Not For Originality

Users scan labels quickly and must recognize what a name refers to without study. A distinctive, clever, or branded name may be memorable as a word but fail as a signpost if it does not signal its function.

Strong names:

- use familiar words the user already knows;
- describe the object or action clearly;
- are short enough to scan but specific enough to distinguish;
- avoid novelty that obscures meaning;
- match the user's vocabulary, not the team's internal jargon.

"Inbox" is recognizable; "Message Hub 360" is not. Originality serves recognition, never the reverse.

### Ensure Each Name Maps To Exactly One Concept

Ambiguity is the most damaging naming failure. If the same label refers to two different things, users cannot trust what they are looking at.

Audit for:

- two features with overlapping names, such as "Projects" and "Workspaces" that behave similarly;
- one concept called by multiple names across screens, such as "folder", "collection", and "group";
- a name whose meaning shifts by context without a clear signal;
- a generic term, like "Items" or "Records", that could mean several things;
- status values that overlap, such as "Pending" and "In progress" with no defined boundary.

Each distinct concept deserves one stable name; each name should point to one concept.

### Distinguish Names By What The User Needs To Differentiate

Users compare names against alternatives. If two names are too similar, users pick the wrong one. Names should differ at least as much as the things they label differ in consequence.

Make distinctions clear when:

- the difference is destructive, such as "Delete" versus "Delete permanently";
- the difference affects cost, such as plan tiers or billing actions;
- the difference affects visibility, such as "Private" versus "Shared";
- the difference affects scope, such as "This project" versus "All projects";
- names share a root or prefix and need a distinguishing suffix.

Similar names for different outcomes is a recipe for wrong actions.

### Match The Name To The User's Mental Model

The right name depends on who the user is and what they already understand. A name that is precise to an expert may be opaque to a novice, and a name that is friendly to a consumer may be insulting to a professional.

Consider:

- the user's domain vocabulary, such as medical, legal, financial, or technical terms;
- the user's experience level and tolerance for jargon;
- regional and cultural differences in word meaning;
- whether the term is already established in the user's workflow or industry;
- whether a simpler term would sacrifice necessary precision.

Do not impose the engineering or business team's internal vocabulary on users.

### Keep Names Stable Over Time And Across Surfaces

Once a user learns a name, changing it forces relearning and breaks documentation, memory, and habit. Names should be chosen for longevity.

Protect stability by:

- avoiding trendy or time-bound terms;
- not renaming features for marketing campaigns while keeping the function the same;
- keeping the name consistent across navigation, help, emails, and marketing;
- planning for feature evolution so the name still fits after the feature grows;
- communicating and documenting any unavoidable rename clearly.

A name is a small contract with the user. Renaming without need breaks it.

### Plan For Name Length And Display

Names live in constrained spaces: navigation bars, buttons, table headers, mobile menus, and localized strings. A name that is perfect in a spec may break in the interface.

Design names to:

- fit common containers without truncation;
- remain distinct even when shortened;
- survive translation expansion;
- work in lists, breadcrumbs, and tabs;
- avoid words so long they dominate a compact control.

If a name must be long, plan an accepted abbreviation, but keep the full name consistent where space allows.

### Align Names With Status, State, And Action Verbs

Names interact with the verbs and states around them. A label should combine naturally with the actions users take on it and the states it can be in.

Check combinations:

- does the name read naturally with "Create", "Edit", "Delete", and "Share"?
- do status values, such as "Active", "Draft", and "Archived", make sense for the named object?
- does the name imply a state that conflicts with the actual states available?
- are role and permission names clear about what the role can do?

Names that fight their verbs and states create cognitive friction on every interaction.

## Common Traps

### Internal Jargon As User-Facing Labels

Terms coined inside the team, such as "Dossier", "Entity Graph", or "Sync Bucket", mean nothing to users. Internal names leak out when no one translates them.

### Cute Or Branded Names For Functional Features

Naming a feature "Spark" or "Nexus" may feel distinctive but tells the user nothing. Branded names need a clear functional label beside them.

### Generic Names That Cannot Be Distinguished

"Items", "Records", "Details", and "Settings" are so broad they could mean anything. Specificity is what makes a name useful.

### Synonyms Used Interchangeably

"Remove" and "delete", "group" and "team", "plan" and "subscription" used for the same concept force users to wonder if the difference is meaningful.

### Names That Differ By Only One Letter Or Word

"Project" and "Projects", or "Share" and "Share with team", may refer to different actions but look nearly identical in a menu. Small differences need clear visual or positional separation.

### Renaming For Marketing Without Migration Care

Changing "Folders" to "Spaces" for a campaign breaks help docs, user memory, and support scripts. Renames must be planned, communicated, and supported.

### Names That Imply Wrong Scope Or Permanence

Calling a temporary draft a "Document", or a shared item "Private", misleads users about consequence and visibility. Names should not contradict behavior.

## Self-Check

- [ ] Each label uses familiar, descriptive words that aid recognition rather than clever or branded terms that obscure function.
- [ ] Every distinct concept has one stable name, and no single name refers to two different concepts.
- [ ] Names that could be confused are distinguished clearly, especially when the difference is destructive, costly, or scope-changing.
- [ ] Labels match the user's domain vocabulary and experience level rather than internal team jargon.
- [ ] Names are consistent across navigation, help, marketing, emails, and all surfaces where they appear.
- [ ] Names were chosen for longevity and will still fit as the feature evolves, avoiding trendy or time-bound terms.
- [ ] Labels fit their containers without truncation and remain distinct when shortened or translated.
- [ ] Each name combines naturally with the create, edit, delete, share, and status verbs that apply to it.
- [ ] Generic labels like "Items" or "Details" were replaced with specific names where ambiguity could cause errors.
- [ ] Any rename was planned with documentation, communication, and user migration in mind, not done casually.