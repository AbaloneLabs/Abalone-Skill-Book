---
name: marketing_channel_content.md
description: Use when the agent is translating or transcreating marketing content tailored to specific channels such as email social media paid ads landing pages push notifications and SMS, adapting length tone and format to each channel's constraints, or coordinating message consistency across channels and markets without losing channel-specific effectiveness.
---

# Marketing Channel Content

Marketing channels are not neutral pipes that carry the same message identically; each channel has its own conventions, constraints, audience expectations, and success metrics. An email subject line, a social post, a paid ad headline, a push notification, and an SMS are governed by different rules of length, tone, urgency, and format, and content that wins on one channel can flop or even break on another. Channel-aware translation is therefore not the repetition of one rendering across surfaces but the disciplined adaptation of a message to each channel's physics while keeping the campaign coherent. Agents miss this because the source often arrives as a content matrix, a grid of strings by channel, and the temptation is to translate each cell fluently and move on. But a subject line has deliverability and open-rate dynamics, a push notification has character and lock-screen constraints, a paid ad has bid and relevance dynamics, and social content has platform-specific tone and hashtag behavior. Treating them as interchangeable produces content that is individually correct but collectively ineffective and inconsistent. The harm this prevents is channel-blind localization: campaigns that underperform because each channel's content was translated rather than adapted to how that channel actually works.

Use this skill when translating or transcreating email campaigns, social media posts, paid search and social ads, landing pages, push notifications, SMS, and other channel-specific marketing content, or when coordinating a message across multiple channels and markets. The goal is content that performs on each channel according to its own rules while remaining coherent as one campaign.

## Core Rules

### Adapt To Each Channel's Constraints And Physics

Every channel imposes hard and soft constraints that shape how content performs, and the translation must be built to those constraints, not merely rendered fluently. Identify each channel's governing rules before translating its content.

Email subject lines face length limits for preview, deliverability sensitivity to spam-trigger words, and the need to drive opens; the translated subject line must avoid local spam triggers, which differ by language and provider. Push notifications face strict character limits and lock-screen truncation, so the core message must land in the first visible characters. SMS faces encoding and segment-length limits, where character-set choices can multiply cost. Paid search ads face headline and description character limits and keyword-relevance dynamics. Social posts face platform tone, hashtag behavior, and link-card rendering. Map these constraints per channel and localize within them. A translation that ignores channel physics may be correct and still fail to deliver, display, or convert.

### Match Tone To Channel And Audience Expectation

Tone is not uniform across channels; what reads as engaging on social may read as unprofessional in email, and what works in a push may feel intrusive in a newsletter. Calibrate tone per channel to the audience's expectation for that surface.

Social media often permits, and rewards, informality, humor, and trend-aware language, but the acceptable register differs by platform and market. Email spans transactional, promotional, and lifecycle tones, and mixing them confuses recipients. Push notifications demand brevity and often urgency but must not feel harassing. Paid ads must align tone with the keyword intent that triggered them. Decide the tone register per channel in line with the brand voice, and do not assume the source's tone maps directly, because channel tone norms differ across markets. A social tone that is casual in one market may need to be more reserved in another where the platform's culture is formal.

### Preserve Campaign Coherence Across Channels

While each channel adapts, the campaign must read as one campaign. The core message, offer, value proposition, and brand voice should remain consistent across all channels in a market.

Establish the campaign invariants: the offer, the key benefit, the brand voice, and any tagline or theme. These hold across email, social, ads, push, and SMS even as length and tone adapt. Without this anchor, channel-by-channel adaptation fragments into several mini-campaigns that confuse the audience and dilute the message. Build a campaign brief or message house before localizing the channel matrix, and check every channel's content against it. Coherence does not mean repetition; it means every channel is recognizably executing the same strategy.

### Handle Channel-Specific Format Elements

Channels carry format elements that have functional meaning and must be handled correctly, not translated blindly. These include variables, links, hashtags, emojis, mentions, and dynamic fields.

Email contains merge fields, unsubscribe links, preheader text, and alt text, all of which must be preserved and often repositioned for target-language syntax. Social content contains hashtags, which may need localized equivalents, and mentions or handles that must not be translated. Paid ads contain keyword insertion variables and display URLs with length limits. Push and SMS contain deep-link parameters and personalization fields. Protect all functional elements exactly, reorder them safely when target grammar requires, and verify counts and positions. A translated email that breaks a merge field or a social post that mistranslates a handle is a production failure, not a stylistic one.

### Localize Offers, Dates, And Locale-Sensitive Content

Marketing content frequently contains offers, dates, times, prices, and locale-specific references that must be localized correctly or they mislead the user and create compliance risk.

Convert or localize dates, times, and time zones to the target market's format and zone, and confirm whether the campaign's validity dates even apply to the target market. Localize prices and currency per the brief, and never invent or alter an offer; if the source offer does not apply in the target market, flag it rather than shipping a misleading promotion. Watch for locale-specific references, holidays, seasons, and events that may not map; a summer campaign launched in a southern-hemisphere market may need seasonal reframing, and a holiday reference may need a local equivalent or removal. These are content decisions, not translation decisions, so flag them for the requester.

### Coordinate Across Markets For Cross-Market Consistency

Global campaigns run across many markets and channels at once, and uncoordinated localization produces markets that contradict each other or diverge from the global concept. Coordinate decisions across markets.

Share terminology, tagline handling, offer rules, and tone decisions across the markets working on the same campaign, so that a multinational audience encounters a coherent brand. Where a master concept or key visual carries globally, align localized copy to it. Coordinate launch timing and content versioning so markets do not ship stale or premature content. For assets reused across markets, maintain a shared glossary and style guide so consistency does not depend on each market rediscovering the same decisions. Cross-market coordination is invisible when done well and conspicuously absent when neglected.

### Plan For Channel Measurement And Iteration

Channels are measured differently, and the localized content will be judged by channel-specific metrics, open rates, click-through, engagement, conversion. Localize with measurement and iteration in mind.

Understand how each channel's content will be evaluated, because the metric shapes what good localization looks like. A subject line is judged by open rate, a push by tap-through, an ad by click-through and conversion, social by engagement. Provide variants for testing where traffic justifies it, and be prepared to revise based on performance. Accept that the highest-performing localization may not be the most literal or elegant one, because channel performance is governed by audience behavior, not by fidelity to source wording. Frame channel localization as optimizable, not final.

## Common Traps

### Treating All Channels As Interchangeable

Translating one rendering across email, social, ads, push, and SMS ignores that each channel has distinct length, tone, and format physics. Channel-blind localization underperforms even when individually correct.

### Breaking Functional Format Elements

Mistranslating or misplacing merge fields, links, hashtags, handles, or keyword variables breaks production. Functional elements must be protected exactly and reordered safely.

### Fragmenting The Campaign Across Channels

Adapting each channel without anchoring to campaign invariants produces several incoherent mini-campaigns. Coherence across channels must be enforced deliberately.

### Inventing Or Altering Offers And Locale Content

Adding, changing, or failing to localize offers, prices, dates, and seasonal references misleads users and can breach compliance. Flag locale mismatches rather than shipping them.

### Ignoring Deliverability And Spam Dynamics

Subject lines and SMS content with local spam triggers or wrong encoding hurt delivery and cost. Channel physics like deliverability must drive localization choices.

### Mismatching Tone To Channel Or Market Norms

A tone that works on one platform or in one market may flop or offend on another. Tone must be calibrated per channel and per market, not carried over.

### Failing To Coordinate Across Markets

Uncoordinated parallel localization lets markets contradict each other and diverge from the global concept. Share decisions and assets across markets.

## Self-Check

Before approving channel-specific marketing content, verify:

- Each channel's constraints, length, format, deliverability, and platform behavior were identified and the content localized within them.
- Tone is calibrated to the channel and the target market's expectation for that surface, consistent with brand voice.
- Campaign invariants, offer, benefit, voice, and theme, hold across all channels so the campaign reads as one.
- Functional elements, merge fields, links, hashtags, handles, keyword variables, and personalization, are preserved exactly and reordered safely.
- Offers, dates, times, prices, currency, and locale-specific references are correctly localized or flagged, with nothing invented or misleadingly carried over.
- Cross-market coordination shared terminology, tone, and concept handling so markets stay coherent with the global campaign.
- Content is framed for channel-specific measurement, with variants provided for testing where justified and openness to performance-based revision.
- No channel was treated as interchangeable, no functional element was broken, and no offer or locale content was invented or mislabeled.
- Encountered by a target-market user on each channel, the content performs that channel's job, opens, taps, clicks, engagement, as the source did in its market.
