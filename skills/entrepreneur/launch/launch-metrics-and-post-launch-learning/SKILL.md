---
name: launch_metrics_and_post_launch_learning.md
description: Use when the agent is defining what success looks like for a product launch, choosing leading versus lagging launch metrics, distinguishing launch noise from durable signal, deciding what to learn in the weeks after launch, or diagnosing why a launch generated a spike that produced no retained users.
---

# Launch Metrics And Post-Launch Learning

A launch produces a spike of attention, and a spike feels like success. Founders watch the charts rise on launch day, report the numbers to investors, and conclude the launch worked. But a launch spike is not adoption; it is curiosity. The users who arrive on launch day are explorers, not customers, and most of them never return. The real measure of a launch is not how many people showed up but how many stayed, and that measure is only visible weeks later, by which point the team has often moved on to the next thing. The judgment problem is defining launch success in terms that distinguish durable signal from transient noise, instrumenting the metrics before launch so they can be measured, and resisting the temptation to declare victory on the strength of a chart that will revert to baseline within a week.

Use this skill when defining launch success metrics, when instrumenting measurement before launch, when interpreting launch results, or when diagnosing why a launch that looked successful produced no lasting growth. The goal is to prevent the agent from optimizing for launch-day vanity metrics and from missing the post-launch learning window that determines whether the spike becomes a curve.

## Core Rules

### Separate Launch Noise From Durable Signal

A launch generates two kinds of traffic. The first is launch noise: visitors driven by the announcement, the press, the community, and the novelty, who arrive once and rarely return. The second is durable signal: users who discover the product through the launch, find value, and return or convert independently of the launch event. These must be measured separately, because blending them produces a number that flatters the launch and hides whether anything durable happened.

Separate by:

- tagging launch-driven traffic distinctly from organic traffic;
- measuring return behavior, such as day-7 and day-30 retention, separately for launch cohorts;
- distinguishing one-time curiosity visits from repeat usage;
- watching what happens to traffic in the weeks after the launch spike subsides.

A launch that produces a spike followed by a return to baseline produced attention but not adoption. A launch that produces a spike followed by an elevated baseline produced durable acquisition.

### Choose Leading Metrics That Predict Lagging Outcomes

Launch-day metrics are leading indicators; revenue and retention are lagging outcomes that take weeks or months to resolve. The team needs leading metrics that genuinely predict the lagging ones, so it can act before the lagging results arrive. Vanity metrics, such as total signups or page views, are leading metrics that predict nothing.

Choose leading metrics by:

- identifying the lagging outcome that defines success, such as retained active users or paying customers;
- tracing backward to the user behaviors that predict that outcome, such as activation, second-session return, or key-action completion;
- instrumenting those behaviors so they are measurable from launch day;
- resisting metrics that count activity without connection to the outcome.

A leading metric is only useful if a change in it reliably precedes a change in the lagging outcome. Total signups do not predict retention; activation rate does.

### Define Activation Before Launch, Not After

Activation is the moment a user first experiences the core value of the product. It is the single most important post-launch metric, because users who do not activate never retain. Yet activation is often undefined at launch, measured inconsistently, or confused with signup. Define activation explicitly before launch so it can be instrumented and measured from the first user.

Define activation by:

- identifying the specific action or state that means the user has experienced the core value;
- ensuring the definition is measurable from product telemetry, not inferred;
- setting a time window, such as activation within the first session or first day;
- distinguishing activation from signup, from feature use, and from engagement.

A product where activation is undefined cannot be improved, because the team cannot tell whether users are reaching the value the product exists to deliver.

### Instrument Measurement Before Launch, Not After

The most common post-launch regret is discovering that the data needed to evaluate the launch was never collected. Once launch traffic arrives and dissipates, the opportunity to measure it is gone. Instrumentation must be in place before launch, tested, and verified to capture the metrics that will be needed.

Instrument before launch by:

- listing every metric that will be needed to evaluate success and diagnose failure;
- verifying each metric is captured correctly in a staging environment;
- ensuring cohort tagging distinguishes launch-driven from organic traffic;
- confirming event tracking covers activation, retention, and conversion funnels;
- testing that dashboards render the data before launch traffic arrives.

A launch without instrumentation produces anecdotes, not evidence, and the learning window cannot be replayed.

### Watch Retention And Conversion, Not Just Acquisition

Acquisition metrics peak on launch day and decline after. Retention and conversion metrics start low and reveal their truth over weeks. A launch evaluated only on acquisition looks successful regardless of what actually happened. The metrics that matter emerge after the spike.

Watch, in the weeks after launch:

- cohort retention: do launch-day users return on day 7, day 30;
- activation rate: what fraction of acquired users reached the core value;
- conversion: what fraction of acquired users became paying or committed users;
- organic acquisition: is the product acquiring users independently of the launch after the spike subsides.

A launch that acquired many users who did not activate or retain acquired curiosity, not customers.

### Treat The Post-Launch Window As A Learning Sprint

The weeks after launch are the highest-information period in a product's early life. Users are arriving, encountering the product fresh, and revealing where activation fails, where value is unclear, and where friction blocks conversion. This window must be treated as a structured learning sprint, not as a passive wait for numbers.

Run the post-launch sprint by:

- reviewing activation and retention funnels daily for the first two weeks;
- interviewing new users about their first experience and where they got stuck;
- watching for the drop-off points where users abandon before reaching value;
- making rapid fixes to the activation path based on observed friction;
- distinguishing product problems, where value is unclear, from distribution problems, where the wrong users arrived.

A team that launches and then waits a month to review results wastes the window in which the product's weaknesses are most visible.

### Define Launch Success Criteria In Advance

Without pre-defined success criteria, any result can be declared a success, and the launch evaluation becomes a story rather than an assessment. Define, before launch, what success and failure look like in measurable terms, so the post-launch interpretation is honest.

Define criteria by:

- setting targets for activation rate, retention, and conversion, not just acquisition;
- setting a baseline expectation for post-spike organic traffic;
- defining the threshold below which the launch is considered to have failed to produce durable adoption;
- committing to the actions that follow each outcome, such as iterating, pivoting, or scaling.

Success criteria defined after the results are known are rationalizations.

## Common Traps

### Declaring Victory On The Spike

Launch-day traffic is curiosity, not adoption. A spike that reverts to baseline produced attention without retention.

### Vanity Metrics As Leading Indicators

Total signups and page views do not predict retention or revenue. Choose leading metrics that genuinely predict the lagging outcome.

### Undefined Activation

If activation is not defined and instrumented before launch, the team cannot tell whether users reached the core value, and cannot improve the product.

### No Instrumentation Before Launch

A launch without pre-installed measurement produces anecdotes, and the learning window cannot be replayed once traffic dissipates.

### Evaluating Only Acquisition

Acquisition peaks on launch day. Retention and conversion reveal the truth over weeks and are the metrics that determine whether the launch worked.

### Waiting Instead Of Learning

The post-launch window is the highest-information period. A team that waits passively for a month wastes the chance to fix activation friction while it is visible.

### Success Criteria Defined After The Fact

Without pre-committed criteria, any result is declared a success, and the evaluation becomes a story.

## Self-Check

- [ ] Launch-driven traffic is tagged and measured separately from organic traffic, so durable signal is distinguishable from launch noise.
- [ ] Leading metrics were chosen because they predict the lagging outcome, not because they are easy to count.
- [ ] Activation is explicitly defined, measurable from telemetry, and instrumented before launch.
- [ ] All metrics needed to evaluate success and diagnose failure were instrumented and tested before launch traffic arrived.
- [ ] Cohort retention, activation rate, and conversion are being watched in the weeks after launch, not just acquisition.
- [ ] The post-launch period is being run as a learning sprint with daily funnel review and new-user interviews.
- [ ] Launch success criteria, including retention and conversion targets, were defined before launch, not after.
- [ ] The team can distinguish a launch that produced durable adoption from one that produced only a spike.
- [ ] Activation friction identified in the post-launch window is being fixed rapidly while it is visible.
- [ ] No launch is being declared successful on acquisition metrics alone without evidence of retention and conversion.