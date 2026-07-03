---
name: activation_and_onboarding.md
description: Use when the agent is improving user activation, designing onboarding flows, defining the activation event or aha moment, diagnosing why new users drop off before reaching value, or planning activation experiments.
---

# Activation And Onboarding

Activation is the bridge between acquisition and retention, and it is where most growth leaks happen invisibly. A user who signs up but never reaches the product's core value was expensive to acquire and contributes nothing, and the gap between sign-up and value is almost always a design problem, not a marketing one. Onboarding is not a tour of features; it is the deliberate teaching of a new user how to reach the outcome they came for, in the smallest number of credible steps.

Use this skill before designing or diagnosing onboarding, defining the activation event, mapping time-to-value, or planning activation experiments. The goal is to prevent the agent from confusing sign-up with activation, from building onboarding that shows features instead of teaching outcomes, from optimizing the wrong step in the funnel, or from measuring activation with a denominator and time window that hide the real drop-off. Use it when the team is asking why new users churn early, how to define the aha moment, what setup is truly required versus nice-to-have, or how onboarding should differ across segments.

## Core Rules

### Define Activation As Reaching Core Value, Not Signing Up

Sign-up is an input; activation is an outcome. The activation event is the moment a user experiences the core value the product exists to deliver, the aha moment after which retention becomes likely. Defining activation as account creation or first login flatters the funnel while hiding the real failure point where most users actually drop off.

A strong activation event is:

- tied to the product's core value, not to an administrative step;
- observable in behavior, not just self-report;
- predictive of retention, validated against later cohorts;
- specific enough to instrument, not so vague it captures everyone.

Find the behavior that separates users who stay from users who leave, and define activation around that behavior rather than around a convenient metric.

### Map Time-To-Value And The Friction Points

Activation is a function of how fast and how easily a user reaches value. Every step between sign-up and the aha moment is a chance to lose them, and most onboarding problems are not missing features but accumulated friction. Mapping the actual path to value exposes where the drop-off really happens.

Map and measure:

- each required step from entry to first value;
- the time and effort each step demands;
- where users stall, skip, or abandon;
- which steps are essential and which are optional;
- where context, data, or permissions are required.

Attack the steps that add time without adding perceived progress toward value. A long time-to-value is usually a sequence of small frictions, not one big wall.

### Treat Onboarding As Teaching, Not A Tour

A feature tour tells users what buttons exist; teaching shows them how to get the outcome they want. Users do not care about the product's capabilities in the abstract; they care about whether it solves their problem. Onboarding that lists features educates without activating.

Design onboarding as progressive teaching:

- start from the user's goal, not the product's structure;
- introduce capability at the moment it is needed;
- use empty states to teach by suggesting the first meaningful action;
- make the first-run experience produce a real result, not a demo;
- defer advanced capability until the core habit is established.

If a user finishes onboarding knowing the menu layout but having achieved nothing, the onboarding has failed even if it felt polished.

### Separate Must-Do Setup From Nice-To-Have

Onboarding bloat happens when every team adds its preferred setup step, and the new user is asked to configure, connect, invite, and personalize before reaching any value. Each step feels reasonable in isolation; together they form a wall. The discipline is to distinguish what is truly required for value from what can be deferred.

For each setup step ask:

- does the user need this to reach the aha moment;
- can it be deferred until after first value;
- can a sensible default replace the input;
- can the step be made optional with a clear benefit for completing it later.

Move anything that is not essential to first value out of the critical path, and let users earn capability as they go deeper.

### Measure Activation With The Right Denominator And Window

Activation rate is only meaningful when its denominator and time window are defined honestly. Measuring activation as activated users divided by all sign-ups, with no time bound, hides whether activation is fast or slow and obscures cohort effects. The wrong denominator makes a leaking funnel look healthy.

Define activation measurement explicitly:

- the denominator: who counts as a candidate for activation;
- the event: the specific behavior that counts as activated;
- the time window: how long users have to activate;
- the segment: whether the rate is measured overall or by cohort.

Compare like with like. A denominator that includes bots, test accounts, or low-intent sign-ups understates the real activation problem.

### Distinguish Activation From Retention

Activation and retention are related but distinct, and conflating them produces confused strategy. Activation is about reaching first value; retention is about returning to value. A user can activate and still churn, and a user can fail to activate yet be counted as retained by a loose metric.

Keep the two separate:

- activation measures first-value attainment;
- retention measures repeat return to value;
- improving activation without retention leaks users who try once and leave;
- improving retention without activation is rare, because users who never reach value rarely return.

Diagnose which problem you actually have before choosing where to invest.

### Onboard For Different Segments And Intents

A single onboarding path assumes all new users want the same thing, which is almost never true. A solo user, a team admin, an enterprise evaluator, and a free-trial prospect have different goals, different authority, and different definitions of value. One onboarding flow optimized for the average serves no segment well.

Segment onboarding by:

- intent or use case signaled at entry;
- role and authority, such as individual versus admin;
- plan or trial context;
- prior experience with the category.

Route each segment to the shortest credible path to its own aha moment rather than forcing everyone through a generic flow.

### Run Activation Experiments On The Right Step

Activation work tempts teams to optimize whichever step is easiest to measure, but improving a late step that few users reach does not fix an early step that loses most of them. Experimentation should target the step with the largest drop, not the step that is most convenient to test.

Before experimenting, identify where the funnel actually loses users, and prioritize the step with the highest marginal return. Optimizing the wrong step produces statistically significant wins that move no business metric.

## Common Traps

### Equating Sign-Up With Activation

Counting account creation as activation inflates the number while hiding the real failure. The trap is reporting a healthy activation rate that masks the fact that most users never reach value.

### Building A Feature Tour Instead Of Teaching Value

Showing the interface educates users about capability without moving them toward an outcome. The trap is a polished onboarding that wins design reviews and fails to activate real users.

### Stuffing Every Setup Step Into First Run

Asking for configuration, integrations, invites, and personalization up front creates a wall before any value. The trap is each team adding one reasonable step until the cumulative friction is fatal.

### Optimizing The Wrong Funnel Step

Polishing a late step that few users reach produces clean experiments and no real lift. The trap is choosing what to test by ease rather than by where users actually drop.

### Using A Denominator That Hides The Leak

Including low-intent or test sign-ups in the denominator makes activation look better than it is. The trap is a metric that reassures instead of diagnoses.

### Conflating Activation With Retention

Treating first-use as retention hides users who try once and never return. The trap is declaring growth fixed because activation improved while retention silently leaks.

### One Onboarding Flow For All Segments

A generic path optimized for the average underserves every real segment. The trap is refusing to branch onboarding because a single flow is simpler to maintain.

## Self-Check

- [ ] The activation event is defined as reaching core value, not as sign-up or first login.
- [ ] The activation event is validated as predictive of retention, not chosen for convenience.
- [ ] Time-to-value is mapped step by step, with friction points identified and prioritized.
- [ ] Onboarding teaches the user's outcome through progressive disclosure, not a feature tour.
- [ ] Setup steps are separated into must-do for first value versus deferrable nice-to-have.
- [ ] Activation rate uses an explicit, honest denominator and time window, with segments compared like for like.
- [ ] Activation and retention are measured and discussed as distinct problems.
- [ ] Onboarding branches by segment, intent, role, or plan where those meaningfully change the path to value.
- [ ] Activation experiments target the step with the largest actual drop, not the easiest step to test.
- [ ] The first-run experience produces a real result for the user, not a demo or an empty tour.
