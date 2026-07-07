---
name: email_deliverability_and_compliance.md
description: Use when the agent is managing email deliverability, diagnosing why emails land in spam, ensuring compliance with email regulations like CAN-SPAM and GDPR, planning authentication and sender reputation, or reviewing whether the email program is reaching the inbox and meeting legal requirements.
---

# Email Deliverability And Compliance

An email that does not reach the inbox might as well not have been sent. Deliverability, the ability to reach the inbox rather than the spam folder or be blocked entirely, is governed by authentication, sender reputation, content, and recipient engagement, and it can collapse silently and suddenly. Compliance, with laws like CAN-SPAM, GDPR, and CASL, is a legal obligation whose violation carries real penalties. The judgment problem is that deliverability and compliance are invisible when they work and catastrophic when they fail, and they are often neglected until a crisis. The most common failure is treating email as a creative and campaign discipline while ignoring the infrastructure and legal foundation that determines whether email works at all. The skill is building and maintaining the deliverability and compliance practices that keep email reaching the inbox and within the law.

Use this skill before launching or scaling an email program, before diagnosing deliverability problems, before ensuring compliance, or when open rates drop suddenly or emails start landing in spam. The goal is to prevent the agent from neglecting deliverability infrastructure, from violating email regulations through oversight, or from treating inbox placement as assumed rather than engineered.

## Core Rules

### [ ] Establish Authentication To Prove Identity

Email authentication, SPF, DKIM, and DMARC, proves to receiving servers that the email genuinely comes from the sending domain. Without authentication, emails are easily spoofed and more likely to be filtered or blocked. Authentication is the foundation of deliverability.

- [ ] Configure SPF, DKIM, and DMARC for all sending domains.
- [ ] Monitor DMARC reports for authentication failures and spoofing.
- [ ] Use a dedicated sending domain for high-volume marketing email if appropriate.
- [ ] Revisit authentication whenever sending infrastructure changes.

### [ ] Build And Protect Sender Reputation

Sender reputation, the score mailbox providers assign based on sending behavior, heavily determines inbox placement. Reputation is built slowly through good practices and lost quickly through spam complaints, bounces, and sudden volume spikes. Protecting reputation is protecting deliverability.

- [ ] Warm up new sending IPs and domains gradually.
- [ ] Avoid sudden volume spikes that trigger spam filters.
- [ ] Monitor reputation across major mailbox providers.
- [ ] Investigate and fix reputation drops immediately.

### [ ] Maintain List Hygiene To Protect Reputation

List quality directly affects reputation. Sending to invalid, inactive, or unengaged addresses produces bounces and low engagement that harm reputation. Regular hygiene, removing invalid and disengaged addresses, protects the ability to reach the engaged majority.

- [ ] Remove hard bounces and invalid addresses promptly.
- [ ] Suppress or sunset disengaged subscribers.
- [ ] Use confirmed opt-in or verification for new subscribers.
- [ ] Audit list quality regularly.

### [ ] Honor Consent And Unsubscribes Promptly

Sending to people who did not consent or who unsubscribed is both a compliance violation and a deliverability killer, generating spam complaints. Consent must be genuine and unsubscribes must be honored immediately and completely.

- [ ] Send only to those who gave genuine consent.
- [ ] Process unsubscribes immediately and across all systems.
- [ ] Make unsubscribe easy, not obstructed.
- [ ] Respect consent scope, do not send types of email the recipient did not agree to.

### [ ] Comply With Email Regulations Fully

Email is regulated by laws including CAN-SPAM (US), GDPR (EU), CASL (Canada), and others, each with specific requirements for consent, identification, unsubscribe mechanisms, and data handling. Compliance is mandatory and must be built into the program, not added after.

- [ ] Identify which regulations apply based on recipient location.
- [ ] Meet each regulation's requirements for consent, identification, and unsubscribe.
- [ ] Include required physical address and sender identification.
- [ ] Maintain records of consent where required.

### [ ] Manage Data And Privacy Obligations

Email programs collect and use personal data, triggering privacy obligations under GDPR and similar laws. Recipients have rights to access, correct, and delete their data, and data must be protected and used only for stated purposes. Data practices must be compliant and transparent.

- [ ] Maintain a privacy policy that covers email data use.
- [ ] Honor data subject requests, access, deletion, correction.
- [ ] Protect email data with appropriate security.
- [ ] Use data only for the purposes for which consent was given.

### [ ] Monitor Deliverability Metrics Continuously

Deliverability is not set-and-forget; it must be monitored through inbox placement rates, spam placement, bounces, and spam complaints. Drops in these metrics signal problems before they become crises. Continuous monitoring catches issues early.

- [ ] Track inbox placement across major mailbox providers.
- [ ] Monitor bounce rates, complaint rates, and spam placement.
- [ ] Use inbox placement testing tools before major sends.
- [ ] Investigate any sudden metric change promptly.

### [ ] Design Content To Avoid Spam Filters

Content alone rarely causes spam filtering, but certain patterns, spammy words, excessive images, broken code, all-image emails, can contribute. Content should be designed to be both engaging and filter-friendly, avoiding patterns that trigger filters.

- [ ] Maintain a healthy text-to-image ratio.
- [ ] Avoid spam-trigger words and excessive capitalization.
- [ ] Ensure HTML is valid and renders correctly.
- [ ] Test content against spam filters before sending.

### [ ] Separate Transactional And Marketing Email

Transactional email, receipts, password resets, must reach the recipient regardless of marketing preferences, and mixing it with marketing email risks both compliance issues and deliverability harm. Separating the streams protects transactional delivery.

- [ ] Use separate infrastructure for transactional and marketing email.
- [ ] Ensure transactional email is not subject to marketing unsubscribe.
- [ ] Keep marketing content out of transactional messages where regulations require.
- [ ] Monitor transactional deliverability separately.

### [ ] Plan For Deliverability Recovery

When deliverability drops, recovery requires diagnosis and sustained corrective action, not a single fix. Having a recovery plan, identifying the cause, fixing list or content issues, rebuilding reputation, shortens the crisis. Treating deliverability as unbreakable leaves the team unprepared.

- [ ] Have a diagnostic process for deliverability drops.
- [ ] Identify whether the cause is reputation, content, authentication, or list quality.
- [ ] Take corrective action and monitor recovery over time.
- [ ] Communicate with mailbox providers or deliverability consultants when needed.

## Common Traps

### [ ] No Authentication

Sending without SPF, DKIM, and DMARC, inviting filtering and spoofing.

### [ ] Reputation Neglect

Ignoring sender reputation until inbox placement collapses.

### [ ] Decaying List Quality

Continuing to send to invalid and disengaged addresses, harming reputation.

### [ ] Consent And Unsubscribe Failures

Sending to non-consenting or unsubscribed recipients, generating complaints and violations.

### [ ] Compliance As Afterthought

Treating legal requirements as secondary, risking penalties.

### [ ] Set-And-Forget Deliverability

Assuming inbox placement without monitoring, missing drops until crisis.

### [ ] Filter-Triggering Content

Content patterns that contribute to spam placement.

### [ ] Mixed Transactional And Marketing Streams

Endangering transactional delivery by mixing it with marketing email.

## Self-Check

- [ ] Are SPF, DKIM, and DMARC configured and monitored for all sending domains?
- [ ] Is sender reputation built through warm-up and protected against volume spikes?
- [ ] Is list hygiene maintained, removing invalid and disengaged addresses?
- [ ] Are consent and unsubscribes honored promptly and completely?
- [ ] Does the program comply with CAN-SPAM, GDPR, CASL, and applicable regulations?
- [ ] Are data privacy obligations, access, deletion, protection, met?
- [ ] Are deliverability metrics, inbox placement, bounces, complaints, monitored continuously?
- [ ] Is content designed to avoid spam filter triggers?
- [ ] Are transactional and marketing email streams separated?
- [ ] Is there a plan for diagnosing and recovering from deliverability drops?
