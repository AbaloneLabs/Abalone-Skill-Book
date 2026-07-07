---
name: cart_and_purchase_flow.md
description: Use when the agent is designing shopping carts, cart pages, mini-carts, saved items, wish lists, multi-step purchase flows, cart persistence, or reviewing how shoppers gather, review, edit, and commit to items before checkout.
---

# Cart And Purchase Flow

The cart is the bridge between browsing and buying, and it is where many shoppers pause, compare, edit, abandon, and return. Unlike a product page, which serves a single decision, the cart holds a growing, changing set of decisions that the shopper revisits over hours, days, or devices. A cart that treats itself as a static summary misses the reality that shoppers use it as a shortlist, a budgeting tool, a comparison workspace, and a holding area. Designing the cart well means supporting all of those behaviors while keeping the path to purchase clear.

Use this skill before designing or reviewing shopping carts, mini-carts, cart drawers, saved-for-later lists, wish lists, multi-item purchase flows, bundle or subscription cart states, or any surface where shoppers gather and revise items before committing. The goal is to prevent the agent from treating the cart as a simple line-item table and ignoring persistence, editing, variant handling, cross-device behavior, and the shopper's real cart-using habits.

## Core Rules

### Treat The Cart As A Living Workspace, Not A Static Summary

Shoppers do not just collect items and buy. They add, remove, change quantities, switch variants, save for later, compare, and reconsider. The cart must support all of these without friction.

Support:

- inline quantity editing with immediate total updates;
- easy removal with a clear undo, since reflexive deletes are common;
- variant changes without forcing a return to the product page;
- moving items between cart, saved-for-later, and wish lists;
- reordering or grouping items by shipment, seller, or category when relevant.

A cart that requires navigating away to change anything treats the shopper's edits as exceptions rather than the core activity.

### Make The Total Honest And Live

The cart total is the shopper's running budget check. It must reflect reality as items change, and it must not hide costs that will appear later.

- update totals immediately when quantities or items change;
- itemize unit price, line totals, discounts, shipping estimates, and tax where knowable;
- distinguish what is fixed from what is estimated;
- show applicable discounts, promo codes, and their effects clearly;
- avoid totals that exclude known costs to look lower.

A total that jumps at checkout because shipping or tax was hidden trains the shopper to distrust the cart itself.

### Handle Multiple Sellers, Shipments, And Fulfillment Paths

Many carts contain items from different sellers, warehouses, or fulfillment methods. Treating them as one undifferentiated list creates confusion about cost, timing, and recourse.

When relevant:

- group items by seller, shipment, or delivery date;
- show per-group shipping cost and delivery estimate;
- clarify return and recourse differences when sellers differ;
- handle backorder, pre-order, and made-to-order states explicitly;
- avoid merging incompatible items, like digital and physical, into one confusing total.

A shopper who cannot tell when each part of their order arrives, or who is responsible for each item, is a shopper one step from abandoning.

### Preserve The Cart Across Sessions And Devices

Carts are rarely completed in one session. Shoppers add on mobile, review on desktop, and return days later. Persistence is a core feature, not a nice-to-have.

- persist cart contents for logged-in users across devices and sessions;
- preserve carts for guest users as long as feasible, with a clear re-entry path;
- handle stock and price changes that occurred since the item was added;
- notify or flag when a saved item changed price, went out of stock, or became unavailable;
- avoid silently dropping items without explanation.

A cart that empties on session end, or that shows stale prices as if current, breaks the shopper's trust in the whole flow.

### Support Saving, Comparing, And Deferring

Not every item in a cart is ready to buy now. Shoppers use carts to hold options they are considering. The flow should support deferral without losing the item.

- allow saving items for later without deleting them;
- support wish lists or favorites separate from the active cart;
- let shoppers move items between active and saved states easily;
- preserve notes, selected variants, or customization across saves;
- make it easy to return to a saved item and complete it later.

Forcing a binary buy-or-delete decision ignores how shoppers actually use carts to manage uncertainty.

### Make Editing Safe And Recoverable

Cart edits are frequent and often reflexive. A shopper who accidentally removes an item, or who changes a quantity by mistake, should not lose their work.

- confirm or allow undo for destructive actions like remove or clear;
- prevent accidental empty-cart actions, especially on mobile;
- preserve selected variants and customizations when quantities change;
- handle invalid states, like zero quantity or unavailable items, with clear guidance.

A cart where one mis-tap empties everything is a cart the shopper will fear using.

### Keep The Path To Checkout Clear Without Pressuring

The cart should always offer a clear path to checkout, but it should not pressure the shopper into hasty decisions. The cart's job is to support the shopper's decision, not to rush it.

- keep the checkout action visible and consistent;
- avoid manipulative urgency, like fake stock counts, on items the shopper is still considering;
- let the shopper edit and save without being pushed toward immediate purchase;
- surface helpful information, like delivery estimates or promo eligibility, without nagging.

Pressure tactics in the cart may lift short-term conversion and increase returns, complaints, and churn.

### Handle Empty, Error, And Recovery States

Carts are not always full and healthy. The flow must handle the states where things go wrong.

- design a useful empty-cart state with clear paths back to shopping;
- handle pricing or availability errors with explanation and recovery;
- support restoring a cart after session loss, login, or device switch;
- explain why an item cannot be checked out, like region restrictions or stock.

An empty or error state that is a dead end wastes the shopper's return visit.

## Common Traps

### Cart As A Static Line-Item Table

A cart that only displays items and a total, without supporting editing, variant changes, or saving, ignores how shoppers actually use carts.

### Hidden Or Static Totals

Totals that do not update with edits, or that exclude shipping and tax to look lower, erode trust and cause checkout surprises.

### Lost Carts On Session Or Device Change

Carts that empty on logout, session end, or device switch discard the shopper's accumulated work and intent.

### Stale Prices And Stock Presented As Current

Showing prices or availability from when the item was added, without flagging changes, misleads the shopper and causes checkout failures.

### Forcing Buy-Or-Delete

A cart that offers no way to defer or save items forces premature decisions and loses shoppers who were still considering.

### Accidental Destructive Actions

One-tap empty-cart or hard-to-undo removes, especially on mobile, create fear and data loss.

### Manipulative Urgency In The Cart

Fake stock counts, countdowns, or pressure copy on items the shopper is still evaluating increases returns and damages trust.

### Unexplained Unavailable Items

Items that cannot be checked out, with no reason or alternative, leave the shopper stuck and likely to abandon the whole cart.

## Self-Check

- [ ] The cart supports inline editing of quantities, variants, and removals with immediate, honest total updates.
- [ ] Removals and destructive actions are recoverable through undo or confirmation.
- [ ] Totals itemize unit price, line totals, discounts, shipping estimates, and tax, distinguishing fixed from estimated costs.
- [ ] Items from multiple sellers, shipments, or fulfillment paths are grouped with per-group cost, timing, and recourse clarity.
- [ ] Cart contents persist across sessions and devices for both logged-in and guest users, with a clear re-entry path.
- [ ] Price, stock, and availability changes since items were added are flagged rather than presented as current.
- [ ] Shoppers can save, defer, or move items between cart, saved-for-later, and wish lists without losing variants or customizations.
- [ ] The path to checkout is clear and consistent without manipulative urgency or pressure tactics.
- [ ] Empty, error, unavailable, and recovery states offer clear next steps rather than dead ends.
- [ ] Invalid states, such as zero quantity, out-of-stock, or region-restricted items, are explained with guidance.
