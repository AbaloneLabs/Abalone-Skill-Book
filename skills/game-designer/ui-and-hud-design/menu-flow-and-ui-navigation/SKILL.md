---
name: menu-flow-and-ui-navigation.md
description: Use when the agent is designing game menus and UI flows, structuring navigation paths between screens, deciding menu depth and organization, or evaluating whether menu navigation is efficient and discoverable or produces excessive clicks, buried options, and player frustration when trying to perform common tasks.
---

# Menu Flow and UI Navigation

Menus are where players spend a surprising fraction of their time — managing inventory, adjusting loadouts, checking objectives, changing settings — and the quality of menu flow determines whether that time feels efficient or feels like fighting the interface. The judgment problem is that menus are often designed by organizing information logically (by category, by system) rather than by task frequency (what the player does most often), and logical organization buries frequent tasks behind the same depth as rare ones. Agents tend to miss this because a menu that is logically structured looks well-organized in review, while the click-count it imposes on frequent tasks only appears in usage, and because the team navigates menus with knowledge of the structure that real players lack. The harm is menus where common actions require many clicks, where settings are buried, where the player cannot find what they need, and where the interface between sessions becomes a source of friction rather than a tool. This skill covers how to structure menu flow by task frequency, manage menu depth, and design navigation that serves the player's actual behavior. The agent has latitude in the menu's organization, but the obligation to make frequent tasks efficient is not optional.

## Core Rules

### Organize Menus by Task Frequency, Not by Logical Category

Players navigate menus to perform tasks, and the tasks they perform most often should be the most accessible, regardless of which logical category they belong to. The decision rule: identify the player's most frequent menu tasks (equip, heal, check objective, adjust one setting), and structure the menu so those tasks are reachable in the fewest clicks, even if it breaks categorical purity. Menus organized purely by category bury frequent tasks at the same depth as rare ones, taxing the player on every common action.

### Minimize Depth for Frequent Tasks and Allow Depth for Rare Ones

Menu depth (number of screens to traverse) should be proportional to task frequency: frequent tasks at depth one or two, rare tasks can be deeper. The decision rule: audit the click-count for each frequent task and reduce any that exceed two or three clicks by surfacing or reorganizing, while allowing rare configuration to live deeper. A menu where every task is at the same depth forces the player to pay the full depth cost for trivial actions, because the structure did not distinguish frequent from rare.

### Provide Direct Paths and Shortcuts for Repeated Operations

Players perform some operations repeatedly (equipping the next item, healing, fast-traveling), and these should have direct paths — hotkeys, quick-menus, radial selectors — that bypass the full menu. The decision rule: identify the operations players repeat most and provide shortcuts that reduce them to one or two inputs, and confirm the shortcuts cover the operations that dominate menu time. Menus without shortcuts force the player to navigate the full structure for every repeated action, converting routine into tedium.

### Ensure Navigation Is Reversible and State-Preserving

Players enter menus by mistake, change their mind, or want to compare options, and the menu must support backtracking without losing state — returning to the previous screen, preserving selections, not resetting to the top of a long list. The decision rule: confirm every menu navigation is reversible, that scroll position and selection are preserved when returning, and that the player never loses their place due to navigation. Menus that reset state on navigation force the player to re-navigate to where they were, doubling the click cost of exploration.

### Make Settings Discoverable and Grouped by Player Need

Settings are where players fix problems (motion sickness, difficulty, controls, audio), and they must be discoverable when the player needs them, grouped by the problem they solve rather than by technical category. The decision rule: organize settings by player-facing need (accessibility, gameplay, audio, video) rather than by engine system, and ensure a player with a specific problem can find the relevant setting quickly. Settings buried under technical categories are unfindable by the player who needs them, because the grouping did not match the player's mental model.

### Validate Menu Flow With Task-Based Usability Testing

Menu quality is measured by whether players can perform tasks efficiently, not by whether the structure looks organized, and the only reliable test is task-based: give players tasks and measure time, clicks, and failures. The decision rule: run task-based usability tests on the menus, measure the cost of common tasks, and revise where the cost is high. Menus reviewed only for structural elegance ship with tasks that take too many clicks, because the structure that looked organized in review did not serve the tasks the player actually performs.

## Common Traps

### Logical Categorization That Buries Frequent Tasks

The team organizes menus by logical category — all combat options together, all exploration options together — and a frequent task that belongs to a deep category (equipping an item deep in an inventory tree) requires many clicks, while a rare task in a shallow category is quickly accessible. The trap is that categorical organization looks clean and defensible. The false signal is that the menu is well-structured. The harm is that the player pays a high click cost on every frequent task because it happened to belong to a deep category, the menu that looked organized imposes constant friction on common actions, and the player's menu time is dominated by navigating to tasks that should have been surfaced, because the structure optimized logic instead of frequency.

### Uniform Depth That Taxes Trivial Actions

Every menu task sits at the same depth, so a trivial frequent action (heal, equip) requires the same navigation as a rare configuration, and the player pays the full depth cost for routine operations. The trap is that uniform depth is simple and consistent. The false signal is that the menu is fair and predictable. The harm is that the player navigates several screens to perform an action they repeat constantly, the friction accumulates across a session, and the menu that could have been efficient is uniformly slow, because the depth was never differentiated by task importance.

### No Shortcuts for Repeated Operations

The player performs some operations repeatedly — equipping, healing, fast-traveling — and the menu provides no shortcuts, forcing full navigation every time, and the repeated operations dominate menu time. The trap is that full navigation is comprehensive. The false signal is that every operation is reachable. The harm is that the player's menu time is consumed by navigating to the same operations repeatedly, the friction is chronic, and the operations that should be one-click remain multi-screen, because shortcuts were not provided for the actions that dominate usage.

### Navigation That Resets State and Loses the Player's Place

The player navigates into a submenu, backs out, and finds the menu has reset to the top of a long list or a different screen, forcing re-navigation to where they were, and exploration becomes costly. The trap is that stateless menus are simpler to implement. The false signal is that navigation functions. The harm is that every exploration doubles in cost because the player must re-navigate to return, comparison becomes tedious, and the player avoids menu exploration because the cost of backtracking exceeds the value, because the menu did not preserve the state that would make navigation reversible.

### Settings Buried Under Technical Categories

Settings are grouped by engine system rather than by player need, so the player with motion sickness cannot find the field-of-view option (it is under "video advanced") and the player who needs remapping cannot find it (it is under "input scheme"). The trap is that technical grouping matches the implementation. The false signal is that settings are comprehensive. The harm is that the player who needs a setting cannot find it because the grouping does not match their mental model, the problem goes unsolved, and the player churns or suffers through a fixable issue because the settings were organized for the engine rather than the player.

### Validating Menus by Structure Rather Than Task Cost

The team reviews menus for structural elegance — the categories are clean, the hierarchy is logical — and approves them, but never measures the click-cost of actual tasks, so the menus ship with frequent tasks buried. The trap is that structure is the visible review artifact. The false signal is that the menu looks well-organized. The harm is that the player experiences the menu as inefficient despite its elegance, common tasks take too many clicks, and the menu that passed structural review fails task-based usability, because the structure was validated instead of the tasks it was meant to serve.

## Self-Check

- Are menus organized so the player's most frequent tasks are the most accessible, even if it breaks categorical purity?
- Is menu depth proportional to task frequency, with frequent tasks at low depth and rare tasks allowed deeper?
- Do the most repeated operations have shortcuts (hotkeys, quick-menus, radials) that bypass full navigation?
- Is navigation reversible and state-preserving, so the player never loses scroll position or selection?
- Are settings grouped by player-facing need (accessibility, gameplay) rather than by engine system, so problems are findable?
- Did I run task-based usability tests measuring click-count and time for common tasks, rather than reviewing only structure?
- Did I confirm frequent tasks are efficient, not merely that the menu is logically organized?
