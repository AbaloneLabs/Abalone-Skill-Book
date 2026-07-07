---
name: status_and_system_messaging.md
description: Use when the agent is designing system messages, status banners, error messages, warnings, notifications, maintenance notices, account and permission messages, and the persistent or transient messaging that tells users about the state of the system, their account, or the world the product operates in.
---

# Status And System Messaging

System messages carry the relationship between the product and the user at its most strained moments: when something is broken, when access is denied, when maintenance is happening, when an account is at risk, or when the rules have changed. These messages are usually written and designed under deadline pressure, in response to a problem, and as a result they tend to be vague, blameful, or technical in ways that alienate users. The judgment problem is that system messaging is where trust is most easily destroyed, because a user who is already dealing with a problem receives a message that either hides the cause, mocks their confusion, or tells them nothing useful.

Use this skill before designing error states, status banners, permission and account messages, maintenance notices, policy and compliance messages, and any persistent or transient communication about system state. The goal is to prevent the agent from shipping messages that are technically accurate but humanly useless, or that protect the system's dignity at the cost of the user's understanding.

## Core Rules

### Write For The User's Situation, Not The System's State

The user reading a system message is usually in one of three situations: they tried to do something and it did not work, they need to know something changed, or they need to take action. The message should speak to that situation, not to the internal state of the system. "The payment service returned a 503" describes the system; "We could not process your payment right now. Please try again in a few minutes" speaks to the user.

Before writing any system message, identify what the user was trying to do, what went wrong from their perspective, and what they need to know or do next. Write from that vantage point. Technical detail can be available on demand for those who want it, but the primary message must be human and situational.

### Always Pair The Problem With A Path Forward

A message that states a problem without offering a next step leaves the user stranded. Every system message, especially errors and warnings, should answer three questions: what happened, why it matters to the user, and what they can do now. The path forward might be retry, contact support, wait, try a different approach, or accept that the action is not possible, but it must be present.

Messages that only describe failure, with no action, force the user to figure out the solution alone, which generates support load and frustration. Even when there is no good fix, saying so honestly, and pointing to help, is better than silence.

### Be Specific About What Went Wrong And Why

Vague messages, such as "Something went wrong" or "Error," give the user no way to understand or fix the problem. Specific messages, such as "Your password must be at least 12 characters" or "This file is too large; the limit is 10 MB," let the user act. The right level of specificity is enough for the user to understand the cause and choose a response, without dumping internal stack traces on them.

Where the cause is genuinely unknown, say so, rather than pretending or guessing. "We are not sure what happened, but your changes were not saved. Please try again" is more honest and more useful than a confident but false explanation.

### Choose Severity And Tone Deliberately

System messages carry an emotional weight through their severity and tone. An error banner that uses alarming red and urgent language for a minor inconvenience makes the product feel fragile and the user feel panicked. A casual, friendly tone for a serious data-loss risk trivializes the danger. Match the visual severity and the language to the actual stakes.

Map messages to severity levels, such as info, success, warning, error, and critical, and apply consistent visual and verbal treatment to each. Reserve the strongest treatment for genuinely critical situations so that users learn to calibrate their response to the signal.

### Distinguish Transient From Persistent Problems

Some problems are temporary: the network blipped, a service is restarting, a rate limit will clear in a minute. Others are persistent: the user lacks permission, the feature is not in their plan, the data they seek does not exist. The messaging should make clear which kind of problem it is, because the right response differs entirely.

For transient problems, encourage retry and set expectations about timing. For persistent problems, explain the constraint and offer alternatives, such as upgrading, requesting access, or contacting an administrator. Conflating the two, by treating a permanent limit like a temporary glitch or vice versa, misleads the user about what they can do.

### Handle Permission And Account State With Care

Messages about permissions, account status, and access are among the most sensitive, because they touch on identity, payment, and trust. A user told they lack permission needs to understand why, whether it is because of their role, their plan, an administrative setting, or a policy, and what, if anything, they can do about it. A user whose account is suspended, expired, or flagged needs clear information and a path to resolution.

Avoid blameful or accusatory language in these messages. State the situation factually, explain the impact, and offer the path to restore or change access. How the product communicates about access shapes whether the user feels respected or policed.

### Make Maintenance And System-Wide Status Visible

When the product itself is degraded, in maintenance, or experiencing an outage, users deserve to know before they encounter unexplained failures. Proactive status messaging, such as a banner announcing scheduled maintenance or a known issue, prevents users from blaming themselves or retrying uselessly. It also reduces support load, because users can see the problem is known.

Surface system-wide status prominently but calmly, distinguish planned from unplanned, and update it as the situation resolves. Hiding outages forces users to discover them through failure.

### Respect Attention And Avoid Alert Fatigue

System messages compete for attention with the user's actual work. A product that shows banners, toasts, modals, and notifications constantly trains users to dismiss them all, which means the messages that matter get ignored. Audit the volume and prominence of system messaging, and reserve high-attention treatments for messages that genuinely require action or carry real consequences.

Persistent low-priority status, such as a read-only mode or a minor degradation, can be shown subtly and remain available without demanding attention. The goal is for users to trust that when the product does demand attention, it matters.

### Keep Messaging Consistent And On-Brand

System messages are part of the product's voice. Inconsistent tone, terminology, or severity treatment across messages makes the product feel disjointed and untrustworthy. Establish voice and severity guidelines for system messaging, and apply them so that an error in one part of the product reads like an error in any other.

## Common Traps

### Technical Or Internal Language

Messages that describe system internals, such as error codes or service names, instead of the user's situation leave people confused and unable to act.

### Problem Without A Path

Messages that state what went wrong but offer no next step strand the user and generate avoidable support load.

### Vague "Something Went Wrong"

Generic errors give users no way to understand or fix the problem and erode trust through their uselessness.

### Severity Mismatch

Alarming treatment of minor issues, or casual treatment of serious ones, miscalibrates the user's response and undermines the messaging system.

### Conflating Transient And Persistent Problems

Treating a permanent limit like a temporary glitch, or a temporary outage like a permanent block, misleads users about what they can do.

### Blameful Permission Messages

Accusatory or unexplained access messages make users feel policed rather than informed.

### Hiding System-Wide Outages

Failing to surface known degradation forces users to discover problems through repeated unexplained failures.

### Alert Fatigue

So many banners, toasts, and modals that users dismiss them all, causing the messages that matter to be ignored.

## Self-Check

- [ ] System messages are written from the user's situation, describing what happened and why it matters to them, not internal system state.
- [ ] Every message, especially errors and warnings, pairs the problem with a clear path forward or next action.
- [ ] Messages are specific enough for the user to understand the cause and choose a response, without dumping technical detail.
- [ ] Visual severity and tone match the actual stakes, with consistent treatment across info, success, warning, error, and critical levels.
- [ ] Transient and persistent problems are distinguished, with appropriate guidance for retry versus accepting a constraint.
- [ ] Permission and account messages are factual, non-blameful, and offer a path to resolution.
- [ ] System-wide maintenance, degradation, and outages are surfaced proactively and updated as they resolve.
- [ ] The volume and prominence of system messaging avoid alert fatigue, reserving high-attention treatment for messages that require action.
- [ ] Voice, terminology, and severity treatment are consistent across all system messages.
