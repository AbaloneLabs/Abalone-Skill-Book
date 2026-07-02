---
name: knowledge_base_and_feedback_loop.md
description: Use when the agent is creating or reviewing support knowledge base articles, help center content, macros, internal notes, self-service flows, support documentation, or feedback loops from customer issues into product and operations.
---

# Knowledge Base And Feedback Loop

Support knowledge is a product surface. A help article, macro, internal note, troubleshooting guide, or self-service flow can reduce customer effort and improve consistency. It can also mislead customers, expose outdated policy, hide product defects, or train support teams into poor answers. The support specialist must keep knowledge accurate, findable, customer-centered, and connected to product feedback.

Use this skill before writing support documentation, updating a help center article, creating a macro, summarizing recurring tickets, proposing self-service content, or turning support patterns into product feedback. The goal is to prevent documentation from becoming a static dumping ground.

## Core Rules

### Start From Real Customer Questions

Knowledge content should answer questions customers actually have. Use ticket patterns, search logs, failed searches, support tags, onboarding friction, release changes, and repeated misunderstandings.

Ask:

- what customers ask;
- what words they use;
- where they get stuck;
- what they tried first;
- what outcome they need;
- whether the issue is confusion, product defect, policy, or missing capability;
- whether self-service is appropriate.

Do not create articles only because an internal team wants to publish information. Start from customer need.

### Choose The Right Knowledge Format

Different problems need different formats.

Possible formats:

- step-by-step article;
- troubleshooting guide;
- FAQ;
- policy explanation;
- known issue note;
- release note;
- macro;
- internal-only runbook;
- video or screenshot guide;
- decision tree;
- escalation checklist.

Public content should be safe, clear, and customer-actionable. Internal content can include routing, diagnostic details, and policy nuance that should not be exposed publicly.

### Keep Instructions Accurate And Testable

Support documentation should be verified against the current product or process.

Check:

- UI labels and navigation;
- screenshots;
- permissions;
- plan differences;
- region differences;
- device or browser differences;
- policy dates;
- billing behavior;
- edge cases;
- failure modes;
- when to contact support.

Do not publish steps that support has not tested or confirmed with the owning team.

### Write For Customer Action

Customers come to support content to solve something. The article should help them decide whether it applies, take the next step, and know what to expect.

Include:

- who the article applies to;
- prerequisites;
- clear steps;
- expected result;
- common errors;
- what to do if it does not work;
- links to related articles;
- support contact path when self-service is not enough.

Avoid long internal explanations before the customer can act.

### Maintain Version And Ownership

Knowledge decays. Assign ownership and review rhythm.

Track:

- article owner;
- subject matter reviewer;
- last reviewed date;
- product version or release dependency;
- policy dependency;
- analytics and feedback;
- deprecation or replacement article;
- internal versus public status.

If nobody owns the content, it will become stale.

### Use Macros Carefully

Macros increase consistency and speed, but they can make support sound careless when used blindly.

Good macros:

- include placeholders that must be filled;
- adapt to customer context;
- avoid unsupported promises;
- include safe information only;
- route sensitive cases correctly;
- stay updated with policy and product changes.

Macros should remind agents to think, not remove judgment.

### Measure Whether Knowledge Actually Helps

Publishing an article is not proof that it works. Review whether customers find it, understand it, and avoid needing another contact.

Useful signals include:

- search terms with no useful result;
- article views followed by support contact;
- negative article feedback;
- repeated macro edits by agents;
- unresolved tickets after self-service;
- time saved for customers or support;
- reduction in duplicate questions.

If content is frequently viewed but customers still contact support, the article may be unclear, incomplete, outdated, or solving the wrong problem.

### Close The Product Feedback Loop

Repeated support contacts are evidence. They may show unclear UI, broken flows, missing features, weak onboarding, policy confusion, or product defects.

Turn patterns into:

- product feedback;
- bug reports;
- documentation updates;
- onboarding changes;
- UI copy improvements;
- support training;
- operational process fixes.

Include ticket volume, customer impact, examples, affected segment, workaround, and business risk. Product teams need evidence, not only complaints.

## Common Traps

### Writing For Internal Organization

Customers do not care which internal team owns a feature. Organize content by customer task and language.

### Publishing Workarounds As Permanent Fixes

If many customers need a workaround, product or operations may need to address the root cause.

### Letting Screenshots Age

Outdated screenshots quickly reduce trust. Use them when they help, and review after UI changes.

### Macro Overuse

A correct macro used in the wrong context can feel insulting or fail to solve the problem.

### No Search Language

If the article does not use customer terms, customers may not find it.

### Feedback Without Evidence

Product feedback that says "customers are confused" is weaker than one with ticket counts, examples, impact, and proposed root cause.

### Measuring Article Views As Success

High views can mean the article is useful, but it can also mean customers are repeatedly forced to look up a confusing workflow. Pair views with resolution and follow-up contact signals.

## Self-Check

- [ ] The content is based on real customer questions, ticket patterns, search behavior, or known friction.
- [ ] The chosen format fits the problem: article, FAQ, troubleshooting guide, macro, internal runbook, known issue, or decision tree.
- [ ] Instructions were checked against current product behavior, policy, permissions, plans, regions, and edge cases.
- [ ] The article states who it applies to, prerequisites, steps, expected result, errors, and what to do if it fails.
- [ ] Public content avoids internal-only details, sensitive diagnostics, and unsafe disclosure.
- [ ] Content has owner, reviewer, review date, and dependency on product or policy changes where relevant.
- [ ] Macros require context adaptation and avoid unsupported promises.
- [ ] Repeated support issues are converted into product feedback, documentation updates, training, or operational fixes.
- [ ] Product feedback includes volume, examples, impact, segment, workaround, and risk.
- [ ] Knowledge performance is reviewed with findability, resolution, feedback, and repeat-contact signals.
- [ ] Knowledge content reduces customer effort rather than hiding product or process problems.
