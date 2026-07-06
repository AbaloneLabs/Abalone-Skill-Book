---
name: inventory-management-and-ux.md
description: Use when the agent is designing inventory systems and interfaces, planning inventory capacity and organization, building item management UX, or evaluating whether inventory management is efficient and unobtrusive or is tedious, cramped, and a constant friction that pulls players out of the game to wrestle with their items.
---

# Inventory Management and UX

The inventory is the interface between the player and the game's items, and it is also one of the most common sources of friction — the menu the player must constantly wrestle with, the capacity that fills too fast, the organization that cannot find what is needed. The judgment problem is that inventory management must be efficient (so the player spends time playing, not sorting), must be unobtrusive (so it does not pull the player out of the game), and must serve the game's item economy (capacity and organization that match the item flow). Agents tend to miss this because an inventory that looks fine in a static mockup can be miserable in play (too small for the item flow, too disorganized to find things), and because the temptation to use inventory limits as difficulty (encumbrance, carry weight) can produce tedium rather than challenge. The harm is players who spend significant time in menus managing items rather than playing, and who experience the inventory as a constant friction. This skill covers how to design inventory management that is efficient, unobtrusive, and matched to the game's item economy. The agent has latitude in the system, but the obligation to make inventory management serve the play rather than obstruct it is not optional.

## Core Rules

### Match Inventory Capacity to the Game's Item Economy

Inventory capacity must be matched to the game's item flow — a game that showers the player in items needs generous capacity, a game with scarce items can use tight capacity — or the mismatch produces either constant capacity management (too small for the flow) or pointless capacity (too large for the flow). The decision rule: analyze the game's item flow (how many items the player accumulates, how fast), size the inventory to accommodate that flow with reasonable headroom, and avoid capacity that fights the flow. Capacity-item mismatch produces either constant sorting or pointless limits, because the capacity was not matched to the economy.

### Make Item Organization Intuitive and Searchable

The inventory must organize items intuitively — by category, by type, by use — and provide search and sort, so the player can find what they need quickly rather than scrolling through undifferentiated lists. The decision rule: implement intuitive categorization, provide sort and search, and avoid flat inventories that the player must scroll to find anything. Disorganized inventories waste the player's time, because finding items required scrolling rather than navigating.

### Minimize the Time Spent in Inventory Management

Inventory management should consume minimal playtime — quick equips, bulk actions, auto-sort — because time in the inventory is time not playing, and excessive inventory management pulls the player out of the game. The decision rule: minimize the actions required for common inventory tasks (equip, compare, sell, store), provide bulk and auto features, and avoid interfaces that require many steps for simple tasks. Time-heavy inventory management pulls the player out, because the management competed with the play for time.

### Provide Clear Item Comparison and Information

When the player considers an item, the inventory must provide clear comparison (how does this compare to what I have) and full information (stats, effects, value), so the decision is informed rather than guesswork. The decision rule: implement clear item comparison (stat diffs against equipped), display full item information, and avoid interfaces that hide relevant data. Hidden comparison produces uninformed decisions, because the player could not assess the item.

### Use Inventory Limits as Design, Not as Tedious Difficulty

If inventory limits (carry weight, slot caps) are used, they should serve a design purpose (forcing prioritization, managing economy) rather than existing as tedious difficulty that produces constant sorting, and the limits should be calibrated to serve the purpose without excessive friction. The decision rule: if using limits, define the design purpose, calibrate the limit to serve it with minimal friction, and avoid limits that produce constant sorting without purpose. Purposeless limits produce tedium, because the sorting served no design end.

### Ensure Inventory Does Not Block Core Gameplay Flow

The inventory must not block the core gameplay flow — the player should not need to open the inventory mid-combat to swap items, should not be stopped by a full inventory mid-action — and the inventory interactions should be paced to not interrupt the core play. The decision rule: identify inventory interactions the core flow requires, ensure they are accessible without flow interruption (quick-slots, radial menus), and avoid full-inventory stops mid-action. Inventory that blocks the flow frustrates, because the management interrupted the play.

## Common Traps

### Capacity Mismatched to Item Economy

The team sizes the inventory without analyzing the item flow, and the capacity is too small (constant sorting to make room) or too large (pointless limits that serve nothing). The trap is that a fixed capacity is simpler. The false signal is that the inventory has limits. The harm is that the mismatched capacity either fills constantly (forcing tedious sorting) or is so generous the limits are pointless, the player either manages inventory instead of playing or ignores the limits entirely, and the capacity serves neither purpose, because it was not matched to the economy.

### Disorganized Inventory That Wastes Time

The team designs a flat or poorly-categorized inventory without sort or search, and the player scrolls through undifferentiated lists to find items, wasting time. The trap is that a flat list is simpler to build. The false signal is that the inventory shows all items. The harm is that the player cannot find items quickly, the inventory management consumes excessive time, the player is pulled out of the game to wrestle with the list, and the inventory becomes a friction point, because the organization was not designed for efficient retrieval.

### Time-Heavy Inventory Management

The team designs inventory interactions that require many steps — equip requires multiple menus, selling requires one-by-one, no bulk actions — and the inventory management consumes excessive playtime. The trap is that the interface is thorough. The false signal is that the inventory has features. The harm is that the player spends significant time in menus rather than playing, the inventory pulls the player out of the game, the engagement that playtime should provide is lost to management, and the inventory is resented, because the interactions were not minimized.

### Hidden Comparison Producing Uninformed Decisions

The team designs the inventory without clear item comparison, and the player cannot assess whether an item is better than what they have, making equip decisions guesswork. The trap is that the inventory shows item stats. The false signal is that the information is available. The harm is that the player cannot compare effectively, the equip decision is uninformed, the player may equip inferior items or miss upgrades, and the inventory fails to support the decisions it exists to enable, because the comparison was not clear.

### Purposeless Inventory Limits as Tedious Difficulty

The team adds inventory limits — carry weight, slot caps — without a design purpose, and the limits produce constant sorting that serves no end. The trap is that limits feel like a survival mechanic. The false signal is that the inventory has constraints. The harm is that the player sorts constantly to stay under the limit, the sorting serves no design purpose, the tedium accumulates without engagement, and the player resents the inventory, because the limits existed without a reason.

### Inventory Blocking the Core Flow

The team designs inventory interactions that interrupt the core flow — mid-combat item swaps requiring full menu, full-inventory stops mid-action — and the inventory blocks the play. The trap is that inventory is menu-based. The false signal is that the player can access items. The harm is that the player is pulled out of the flow to manage inventory, the combat or action is interrupted, the engagement that the flow should provide is disrupted, and the player resents the inventory, because it blocked rather than served the play.

## Self-Check

- Is inventory capacity matched to the game's item flow, avoiding constant sorting or pointless limits?
- Is item organization intuitive, with categorization, sort, and search for quick retrieval?
- Are common inventory tasks (equip, compare, sell, store) minimized in steps, with bulk and auto features?
- Does the inventory provide clear item comparison and full information for informed decisions?
- If inventory limits are used, do they serve a design purpose without producing tedious constant sorting?
- Does the inventory avoid blocking the core flow, with quick-access options for in-action item use?
- Did I confirm the player spends time playing, not wrestling with inventory management?
