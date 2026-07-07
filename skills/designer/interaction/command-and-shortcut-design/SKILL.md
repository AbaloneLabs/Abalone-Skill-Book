---
name: command_and_shortcut_design.md
description: Use when the agent is designing keyboard shortcuts, command palettes, hotkeys, or quick-action systems, deciding which actions deserve shortcuts, choosing key sequences and conflicts, designing discoverability and learnability of shortcuts, supporting power users without alienating new users, handling platform-specific modifier conventions, or ensuring command interactions remain accessible and do not conflict with assistive technology and browser shortcuts.
---

# Command And Shortcut Design

Shortcuts and command palettes are the interaction layer for users who have crossed from novice to expert, and they are where a product either rewards deep use or caps it. A well-designed command system lets experienced users move at the speed of thought, bypassing menus and dialogs to act directly. A poorly designed one conflicts with the operating system, collides with assistive technology, hides its own existence, and traps power users in a slower interface than they could otherwise use. The design problem is not assigning keys; it is deciding which actions deserve shortcuts, making them discoverable and learnable, avoiding conflicts, and ensuring the command layer accelerates experts without excluding or endangering anyone else.

Agents tend to fail shortcut design in predictable ways. They assign shortcuts to every action, creating an unlearnable map of key combinations. They choose key sequences that conflict with the browser, the operating system, or screen readers, breaking the product for the users who need those bindings most. They hide shortcuts so thoroughly that only users who read documentation ever find them. Or they build a command palette as a feature checkbox without curating which commands appear, so the palette becomes a noisy search over an undifferentiated list.

Use this skill before assigning keyboard shortcuts, building a command palette, or designing quick-action systems. The goal is a command layer that is curated, discoverable, conflict-free, accessible, and genuinely faster than the path it replaces.

## Core Rules

### Assign Shortcuts Only To Actions That Warrant Them

Not every action deserves a shortcut. Shortcuts are a finite resource, because memorable key combinations are scarce and assigning one to a rare action wastes a slot a frequent action needs. The discipline is to reserve shortcuts for actions that are both frequent and costly to reach through the primary interface.

Reserve shortcuts for:

- frequent actions that power users repeat many times per session;
- actions buried deep in menus or dialogs, where the shortcut saves meaningful navigation;
- destructive or high-stakes actions that benefit from a fast, deliberate path;
- actions central to the product's core workflow, where speed is a competitive advantage.

Avoid assigning shortcuts to:

- rare actions whose key combination will be forgotten before it is used again;
- actions already trivial to reach through a visible control;
- actions whose key sequence would force a conflict with a more deserving command.

A product with a shortcut for every action has a shortcut for no action, because none can be remembered.

### Choose Key Sequences That Avoid Conflicts

A shortcut that conflicts with the browser, the operating system, or assistive technology is worse than no shortcut, because it either fails silently or hijacks a binding the user depends on. Conflict avoidance is the hardest and most important constraint in shortcut design.

Avoid conflicts systematically:

- avoid single-key shortcuts that conflict with screen reader and browser shortcuts, which often rely on single letters;
- respect platform modifier conventions, using Command on macOS and Control on Windows and Linux, and document where you deviate;
- avoid hijacking critical browser and OS shortcuts, such as those for navigation, accessibility, and developer tools, unless you provide an escape hatch;
- check conflicts against assistive technology, because a binding that works for a sighted mouse user may break a screen reader user's navigation.

When a conflict is unavoidable, document it and provide a fallback path to the same action.

### Make Shortcuts Discoverable And Learnable

A shortcut no one knows about delivers none of its benefit, and shortcuts are notoriously hard to discover because they are invisible by nature. Discoverability must be designed into the interface rather than left to documentation.

Build discoverability:

- display shortcuts inline next to their actions in menus and tooltips, so users learn them through normal use;
- surface recently used or suggested shortcuts in context, so the command system teaches itself;
- provide a searchable command palette or shortcut cheatsheet that lets users find actions by name;
- use consistent patterns, so that once a user learns one shortcut, related ones become predictable.

Consistency is the strongest learnability lever. If "G then I" goes to Inbox, "G then S" should go to Sent, not to Settings.

### Curate The Command Palette As A Product Surface

A command palette is not a dump of every action in the product. It is a curated surface that prioritizes the actions users are most likely to search for, and an uncurated palette degrades into a noisy search that is slower than the menus it was meant to replace.

Curate the palette:

- include actions that are frequent, hard to find, or central to expert workflows;
- exclude trivial, destructive-without-confirmation, or configuration actions that do not belong in a quick-action surface;
- rank results by relevance, recency, and frequency, not alphabetically, so the likely action surfaces first;
- support aliases and natural language, so users can find an action by the name they remember, not the name it was given.

### Support Both Novice And Expert Paths

A command system that serves only experts alienates new users, and one that serves only novices caps the speed of experts. The two paths must coexist, with the visible interface as the novice path and the command layer as the expert acceleration.

Support both:

- ensure every shortcut-accessible action is also reachable through the visible interface, so no action is hidden behind a binding the user may not know;
- let users graduate naturally, with shortcuts revealed through use rather than required upfront;
- avoid forcing shortcuts on new users, such as modal traps that demand a keypress, which exclude users who do not know it;
- provide personalization where it adds value, letting users remap bindings, but do not make personalization a substitute for good defaults.

### Design For Memorability And Error Resistance

Shortcuts are recalled from memory under cognitive load, and a shortcut system that is hard to remember or easy to trigger by mistake causes errors. Memorability and error resistance are design properties, not user responsibilities.

Design for recall and safety:

- use mnemonic key choices where possible, so the binding hints at the action;
- group related shortcuts under consistent modifiers or chords, so the structure aids memory;
- separate destructive shortcuts from benign ones, so a slip does not trigger an irreversible action;
- require confirmation or an undo for destructive shortcut-triggered actions, because shortcuts are especially prone to accidental activation.

### Handle Platform And Input Variability

Shortcuts behave differently across platforms and input methods, and a command system designed only for a desktop keyboard fails on other configurations. The system must adapt while preserving its underlying model.

Adapt across platforms:

- map modifiers per platform, Command on macOS, Control elsewhere, and document the mapping;
- provide pointer and touch equivalents for environments without keyboards, such as tablets;
- ensure the command layer is operable by switch devices and other alternative inputs, not only by full keyboards;
- preserve the command model across platforms, so users transfer their learning even as the bindings differ.

### Ensure Accessibility Of The Command Layer

The command layer must be accessible, not merely available. Shortcuts and palettes that are not announced to screen readers, not operable by keyboard alone, or not reachable by users with motor differences exclude the users who often benefit most from efficient input.

Make the command layer accessible:

- ensure command palettes are operable by keyboard and announced by screen readers, with results navigable via standard keys;
- avoid trapping focus or relying on timing that excludes users who input slowly;
- provide announcements when shortcut-triggered actions occur, so assistive technology users know what happened;
- never make a shortcut the only path to an action, because some users cannot operate it.

## Common Traps

### Shortcuts For Every Action

Assigning a binding to every action creates an unlearnable map and wastes memorable combinations on rare commands.

### Conflicts With Browser, OS, Or Assistive Technology

A binding that hijacks a critical shortcut breaks the product for the users who depend on that binding, especially screen reader users.

### Invisible Shortcuts

Shortcuts that are never displayed or taught deliver none of their speed benefit and cap expert users at novice speed.

### An Uncurated Command Palette

A palette that dumps every action becomes a noisy search slower than the menus it replaced.

### Expert-Only Actions With No Visible Path

Actions reachable only by shortcut exclude users who do not know the binding and violate accessibility expectations.

### Inconsistent Binding Patterns

Shortcuts that follow no pattern cannot be learned from each other, so users must memorize each in isolation.

### Destructive Shortcuts Without Confirmation Or Undo

Shortcuts are prone to slips, and a destructive binding without a safety net causes irreversible errors.

### Desktop-Only Design

A command system designed only for a desktop keyboard fails on tablets, switch devices, and other input methods.

## Self-Check

- [ ] Shortcuts are reserved for frequent, deep, or high-stakes actions, not assigned to every action in the product.
- [ ] Key sequences avoid conflicts with browser, OS, and assistive technology bindings, with fallbacks where conflict is unavoidable.
- [ ] Shortcuts are discoverable through inline display in menus and tooltips, contextual suggestions, and a searchable palette or cheatsheet.
- [ ] The command palette is curated, ranked by relevance, and supports aliases and natural language, not an alphabetical dump.
- [ ] Every shortcut-accessible action is also reachable through the visible interface, so no action is hidden behind a binding.
- [ ] Bindings follow consistent and mnemonic patterns that aid memorability, with destructive shortcuts separated from benign ones.
- [ ] Destructive shortcut-triggered actions require confirmation or offer undo, because shortcuts are prone to slips.
- [ ] Modifiers are mapped per platform, with pointer and touch equivalents for keyboard-less environments, while preserving the underlying model.
- [ ] The command layer is operable by keyboard alone, announced to screen readers, and reachable by switch and alternative inputs.
- [ ] No action is reachable only by shortcut, so users who cannot operate the binding are not excluded.
