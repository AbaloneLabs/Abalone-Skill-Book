---
name: feedback-routing-and-action.md
description: Use when the agent is routing feedback to the right teams, closing the loop with customers, deciding which feedback triggers product changes versus documentation, managing feedback ownership, or ensuring collected customer input actually reaches the people who can act on it.
---

# Feedback Routing And Action

Collecting and synthesizing feedback produces nothing of value unless it reaches the people who can act on it and unless customers see that speaking up had consequences. The gap between "we listened" and "we acted" is where most feedback programs fail. Feedback that is gathered, summarized, presented once, and then forgotten is worse than no program at all, because it teaches customers that their input disappears into a void and teaches the team that feedback is noise to be managed rather than signal to be used.

This skill covers the operational judgment of moving feedback from insight to action: who owns it, where it goes, how it is prioritized against other demands, and how the loop is closed with the people who provided it.

## Core Rules

### Define ownership before feedback arrives

The most common routing failure is that feedback is everyone's responsibility and therefore no one's. A survey result lands in a shared inbox, gets discussed in a meeting, and then drifts because no one was assigned to do anything with it. Before you scale up collection, define clear ownership.

- Name a single accountable owner for each major feedback channel. This person is responsible for triage, routing, and ensuring nothing falls through the cracks. They are not responsible for fixing every issue, only for making sure each item reaches a decision or an owner.
- Define routing destinations: which team handles bugs, which handles feature requests, which handles billing or policy issues that are not product problems at all. Misrouted feedback (a feature request sent to engineering as a bug, or a policy complaint sent to design) dies in the wrong queue.
- Establish a service-level expectation for triage. Feedback that sits unread for weeks teaches customers that no one is listening, and teaches the team that feedback is low priority.

Ownership is boring and essential. Without it, the synthesis work is wasted.

### Triage by decision type, not by topic

When routing synthesized feedback, the natural impulse is to sort by feature area or theme. But the actionable sorting is by what kind of decision the feedback demands.

- **Defects and regressions** route to engineering with severity and reproduction context. These need fast triage, not roadmap debate.
- **Missing capabilities** route to product as candidate backlog items. These need prioritization against other opportunities, not immediate action.
- **Confusion and discoverability problems** may route to design, documentation, or in-product education. Often the cheapest fix is not a code change but clearer messaging.
- **Policy, pricing, or expectation mismatches** route to the relevant function (support, success, billing) and may indicate a communication problem rather than a product problem.

Sorting by decision type prevents the trap of sending everything to engineering as "the product team will figure it out." Much feedback is best resolved outside the build pipeline.

### Separate routing from prioritization

A common mistake is to treat routing as commitment. Routing a feature request to the backlog is not a promise to build it; it is a promise to consider it. Be explicit about this distinction, both internally and with customers.

- Routing decides where the feedback goes and ensures it is not lost.
- Prioritization decides whether and when it gets acted on, weighed against everything else.

Conflating them creates two failures. Internally, teams feel obligated to act on every routed item, leading to a bloated, unfocused roadmap driven by whoever shouts loudest. Externally, customers interpret "we have passed this to the team" as "we will build this," and lose trust when nothing appears. Make the boundary clear: feedback is heard and routed; action is a separate, deliberate decision.

### Close the loop with customers who spoke up

Closing the loop is not a courtesy; it is what makes the feedback channel sustainable. Customers who took time to provide input and then see no acknowledgment learn that the channel is one-way and stop participating. The customers most likely to stop are often the thoughtful, moderate ones whose signal you most need.

- For individual, identifiable feedback (interviews, support-escalated cases, account-level surveys), close the loop personally and specifically. Acknowledge what you heard.
- For aggregate feedback (mass surveys, widget comments), close the loop publicly and honestly. Share what themes emerged and what the team is doing or not doing about them.
- Be truthful about outcomes. If the feedback did not change anything, say so and explain why, rather than implying action that will not come. Customers respect honest "we considered it and here is the tradeoff" far more than silence or false promises.

Closing the loop is also a data-gathering opportunity. When you tell customers what you decided, their reaction tells you whether your synthesis was right.

### Route the negative feedback especially carefully

Positive feedback is easy to route; it gets shared in all-hands meetings and pinned to dashboards. Negative feedback is harder and more valuable, and it is also more likely to be suppressed, softened, or buried. Build explicit handling for it.

- Ensure critical or angry feedback reaches decision-makers unfiltered. A support tier that summarizes away the anger also summarizes away the urgency.
- Route patterns of negative feedback to the owning team with enough context to understand the user's actual experience, not just the sanitized summary.
- Resist the impulse to explain away negative feedback ("they just don't understand the feature"). Even when the root cause is a misunderstanding, the misunderstanding is itself a product problem worth routing to design or education.

### Make feedback visible where decisions are made

Feedback that lives in a separate "voice of customer" tool, visited only by the research team, has no influence. Route feedback into the systems where prioritization and planning actually happen: the backlog, the roadmap review, the sprint planning artifacts. A theme that does not appear where decisions are made does not exist for decision-making purposes.

This also means translating feedback into the language of those systems. A raw quote is moving but not actionable in a prioritization framework. Translate themes into problem statements, impact estimates, and affected segments so they can be compared alongside telemetry and business goals.

### Measure whether the feedback program is working

A feedback program is itself a product with users (your customers) and stakeholders (your team). Measure whether it works.

- Track loop closure: what fraction of feedback themes resulted in a clear decision (built, declined, or deferred with rationale)?
- Track customer perception: do customers believe their feedback matters? This is measurable through periodic surveys of the feedback experience itself.
- Track time-to-decision: feedback that takes six months to route and decide has lost its relevance.

If these metrics are poor, the problem is not the customers or the synthesis; it is the routing and action machinery, and that is what needs investment.

## Common Traps

### The feedback black hole

Feedback flows in and nothing visible flows out. Internally, teams stop submitting or trusting it; externally, customers stop providing it. The black hole forms silently and is only noticed when response rates collapse. The fix is not more collection; it is visible, consistent closing of the loop.

### Routing everything to engineering

Because feedback is "about the product," it tends to default to engineering queues. But much feedback is best resolved by documentation, support training, pricing changes, or expectation-setting. Defaulting to engineering overloads the build pipeline with items that would be solved faster and cheaper elsewhere, and frustrates engineers who receive requests that are not really engineering problems.

### Promising action to secure feedback

To get users to participate in interviews or surveys, there is a temptation to imply that their input will directly shape the product. This secures participation in the short term and destroys trust in the long term when the implied promise is not kept. Be honest that feedback informs but does not dictate, and that many good ideas will not be built because of tradeoffs. Honest framing attracts the customers whose feedback is most useful.

### The loudest customer sets the roadmap

When routing is reactive, the customers who escalate, complain publicly, or have the largest accounts dominate the action queue. This optimizes the product for escalation skill rather than for value. Counter it by routing based on synthesized patterns and impact, not on volume of individual pressure, and by deliberately seeking out the signal from customers who do not escalate.

### Feedback that dies in prioritization

Feedback is routed correctly, enters the backlog, and then never gets prioritized because it competes with strategic initiatives and never wins. This is a form of the black hole, more subtle because the feedback appears to have been received. The fix is to periodically review the feedback-derived backlog explicitly and decide deliberately to build, decline, or defer each item, rather than letting it age silently.

### Treating loop closure as marketing

When closing the loop publicly, the temptation is to highlight only the positive outcomes and the feedback that led to shipped features. This reads as self-congratulation and erodes trust among customers whose feedback was declined. Honest loop closure includes what was heard, what was decided, and why some things were not acted on. Credibility comes from the honesty about the no's, not the celebration of the yes's.

## Self-Check

- Is there a single accountable owner for each feedback channel, with defined routing destinations and triage service levels?
- Am I triaging by decision type (defect, capability, confusion, policy) rather than dumping everything into one queue?
- Have I clearly separated routing (where it goes) from prioritization (whether and when to act), both internally and in customer communications?
- Am I closing the loop with customers, personally for identifiable feedback and publicly for aggregate feedback, with honest outcomes including declines?
- Is negative feedback reaching decision-makers unfiltered, or is it being softened away in summarization?
- Does feedback appear in the systems where prioritization actually happens, translated into the language of those systems?
- Am I measuring whether the program works: loop closure rate, customer perception, and time-to-decision?
- Is the loudest customer or largest account overriding synthesized patterns and impact in setting action priorities?
- When I close the loop publicly, do I include honest declines and tradeoffs, or only the wins?
- Would a customer who provided feedback six months ago believe, based on evidence, that their input was heard and considered?
