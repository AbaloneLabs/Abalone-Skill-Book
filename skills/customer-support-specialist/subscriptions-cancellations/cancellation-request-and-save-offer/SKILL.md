---
name: cancellation-request-and-save-offer.md
description: Use when the agent is handling a cancellation request, retention offer, save attempt, subscription termination, account closure, downgrade alternative, cancellation confirmation, or customer complaint that cancellation was blocked, delayed, unclear, or handled against their intent.
---

# Cancellation Request And Save Offer

Cancellation support is a trust moment. Customers may be leaving because of cost, missing value, product fit, service failure, privacy concern, or life change. Agents often over-optimize retention and create friction, unwanted charges, complaints, or regulatory risk. This skill helps the agent respect cancellation intent while using retention options only when appropriate and transparent.

## Core Rules

### Confirm the customer's actual intent

Distinguish cancel at end of term, cancel immediately, close account, downgrade, pause, remove add-on, stop renewal, stop marketing, delete data, or request refund. These actions have different effects. Repeat back the action and effective date when stakes are high.

Do not assume a customer asking "cancel" wants a retention conversation before the cancellation path is clear.

### Do not obstruct cancellation

Follow cancellation policy and legal requirements for clear, accessible, and timely cancellation. Do not hide the cancellation path, require unnecessary steps, loop the customer through repeated offers, or delay action to improve retention metrics.

Retention should be an option, not a barrier.

### Use save offers only when relevant and authorized

Save offers may include discount, pause, downgrade, product help, extension, customer success review, or feature education. They should match the customer's reason and be within authority. If the customer says they just want to cancel, proceed without pressure.

Do not offer discounts that create billing confusion or unauthorized precedent.

### Explain effects before final action

Cancellation may affect access date, stored data, team members, integrations, invoices, renewals, warranties, service levels, support tier, discounts, trial status, and refunds. Explain material effects clearly before irreversible action, but do not use them as scare tactics.

If effects vary by plan or region, verify before stating them.

### Separate cancellation from refund eligibility

Canceling stops or changes future service; refunding returns money already charged. A customer may be eligible for one, both, or neither. Handle refund review through policy and evidence, not as an automatic result of cancellation.

Avoid saying "you are all set" if the refund question remains open.

### Watch for failed cancellation signals

Customers may report they tried to cancel earlier, could not find the button, received errors, were blocked by login, contacted support, or thought cancellation was complete. Check logs, prior tickets, emails, renewal notices, and self-service events before denying.

Failed cancellation may require service recovery or compliance routing.

### Protect account authority and shared users

Only authorized roles may cancel organization, family, reseller, marketplace, or enterprise subscriptions. Canceling may affect other users or data. Verify authority and customer impact before action.

Do not disclose or alter subscription status for unauthorized requesters.

### Document final state and promises

Record customer intent, effective date, save offer accepted or declined, cancellation action, access effects, refund path, confirmation sent, and follow-up. The customer should receive a clear confirmation of what changed.

Future agents must know whether cancellation is pending, completed, or failed.

### Verify the system state after cancellation

After submitting cancellation, check the subscription status, renewal flag, effective date, pending invoices, active add-ons, marketplace state, and confirmation delivery where tools allow. A cancellation instruction is not the same as a completed cancellation.

If a backend job or third-party system must finish the change, tell the customer what is pending and set a follow-up instead of implying the process is done.

## Common Traps

- Treating "cancel" as permission for a long retention script.
- Confusing cancel, pause, downgrade, stop renewal, close account, unsubscribe, and data deletion.
- Making cancellation harder than signup or self-service policy allows.
- Offering discounts or pauses without authority or clear billing terms.
- Warning about access loss in a way that feels coercive; saying cancellation includes refund when refund eligibility is separate
- Ignoring evidence that the customer tried to cancel earlier; canceling a shared or enterprise subscription for an unauthorized user
- Failing to send or record cancellation confirmation; leaving the case ambiguous about whether renewal will still happen
- Treating a submitted cancellation request as completed without checking subscription status and renewal flag; forgetting add-ons, marketplace subscriptions, or pending invoices that survive the main cancellation

## Self-Check

- Is the requested action clear: cancel now, end of term, close account, downgrade, pause, remove add-on, stop renewal, stop marketing, delete data, or refund?
- Was cancellation intent respected without unnecessary friction or repeated save attempts?
- Are save offers relevant, authorized, transparent, and optional?
- Are access, data, team, integration, invoice, renewal, warranty, support tier, discount, trial, and refund effects explained accurately?
- Is refund eligibility handled separately from cancellation?
- Were failed cancellation signals checked in logs, prior tickets, emails, renewal notices, and self-service events?
- Is the requester authorized to cancel the subscription affected?
- Are shared, reseller, marketplace, family, and enterprise impacts considered?
- Is final state clear: pending, completed, scheduled, failed, or awaiting customer action?; are intent, effective date, offer, action, effects, refund path, confirmation, and follow-up documented?
- Were subscription status, renewal flag, effective date, add-ons, pending invoices, marketplace state, and confirmation delivery checked where possible?; if cancellation depends on another system, is a follow-up path documented?
