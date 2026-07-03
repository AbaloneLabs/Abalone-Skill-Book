---
name: test_method_selection.md
description: Use when the agent is choosing a discovery or validation method for a hypothesis, weighing interviews versus prototypes versus experiments versus concierge tests, or matching the evidence type to the question and stage.
---

# Test Method Selection

There is no universal best way to test a hypothesis. Each method, interviews, prototypes, fake-door tests, concierge tests, A/B experiments, answers a different kind of question and produces a different kind of evidence. Choosing the wrong method gives a confident answer to the wrong question, and that is worse than having no answer at all.

The judgment problem is that teams pick the method they always use, or the method that feels rigorous, rather than the method that fits the question and the stage. They run an A/B test to find out whether users want a feature, when an A/B test can only measure behavior on a feature that already exists. They run interviews to find out whether users will pay, when interviews measure stated intent, not willingness to pay. They build a full prototype to test a value proposition that a landing page would have tested in an afternoon. Agents tend to conflate the type of evidence a method produces with the type of evidence the question requires, and to reach for experiments because experiments feel scientific, even when the question is about understanding rather than measurement. The harm is confident conclusions that do not actually answer the question, money spent on the wrong test, and decisions made on evidence whose type does not match the risk.

Use this skill when choosing how to test a hypothesis, when matching a method to a stage of discovery, and when a team defaults to one method regardless of the question. The goal is to select the cheapest method that produces evidence of the right type to resolve the specific question at the current stage.

## Core Rules

### Match The Method To The Type Of Evidence The Question Requires

Different questions require different types of evidence, and each method produces a specific type. Questions about why users behave a certain way require explanatory evidence, best from interviews or observation. Questions about whether users can complete a task require behavioral evidence, best from prototypes or usability tests. Questions about whether a change moves a metric require causal evidence, best from experiments. Mismatching method to evidence type answers the wrong question convincingly.

Start from the question and ask what type of evidence would resolve it. If the question is "do users have this problem," you need problem evidence, not solution evidence, so interviews beat prototypes. If the question is "does this design work," you need behavioral evidence, so a prototype beats an interview. If the question is "does this feature improve retention," you need causal evidence, so an experiment beats a prototype. The evidence type, not the team's comfort, drives the method.

### Match The Method To The Stage Of Discovery

The right method also depends on how much is known. Early, when the problem and audience are uncertain, cheap generative methods like interviews and observation are appropriate, because the goal is to understand and to find the right questions. Mid, when the problem is understood but the solution is not, prototypes and fake-door tests probe whether a specific approach resonates. Late, when the solution exists, experiments measure whether it moves the target metric.

Do not jump to experiments early. An A/B test requires a built feature and a specific metric, so it cannot answer questions about whether the problem exists or whether the solution direction is right. Using a late-stage method for an early-stage question either produces no signal or, worse, a precise number that answers a question no one asked. Sequence methods so each stage feeds the next.

### Distinguish Stated Intent From Revealed Behavior

A critical method distinction is whether the evidence captures what users say or what users do. Interviews, surveys, and focus groups capture stated intent, which is useful for understanding motivations and mental models but is a poor predictor of actual behavior. Users routinely say they would use or pay for things they do not. Prototypes, fake-door tests, concierge tests, and experiments capture revealed behavior, which is far more trustworthy for predicting what users will actually do.

Match the evidence type to the stakes. For low-stakes understanding, stated intent is fine. For decisions about whether to build or invest, prefer revealed behavior, because the cost of acting on overstated intent is real. When you must use stated intent, triangulate with at least one behavioral signal to check whether words and actions agree.

### Use The Cheapest Method That Can Resolve The Question

Cost matters because cheaper methods iterate faster and allow more cycles of learning. A concierge test, where you deliver the value manually behind the scenes, can validate willingness and workflow without building software. A fake-door or painted-door test can validate demand by measuring clicks on a non-existent feature. A landing page can validate value-proposition resonance before any product is built. Each is far cheaper than a full build.

Ask what the cheapest test is that could change your mind about the hypothesis. If a landing page could refute the demand assumption, do not build the feature first. If a concierge version could validate the workflow, do not automate it first. The discipline is to spend the minimum that produces trustworthy evidence, reserving expensive methods for questions that genuinely require them.

### Understand What Each Method Can And Cannot Prove

Every method has a blind spot, and choosing without knowing it leads to overconfidence. Interviews cannot predict behavior. Surveys cannot establish causality. Prototypes cannot prove market demand, because a user who likes a prototype may never use the real thing. Fake-door tests prove interest but not value, because a click is not usage. Concierge tests prove willingness but not scalability. A/B tests prove a metric moved but not why, and only for the population tested.

Name the blind spot of the chosen method before relying on its result. If the blind spot overlaps the riskiest assumption, either choose a different method or run a complementary method to cover the gap. A single method's confidence is always narrower than it feels, and combining methods that cover each other's blind spots is how strong evidence is built.

### Avoid Using Experiments To Answer Understanding Questions

A common error is reaching for an A/B test when the question is about understanding rather than measurement. Experiments are powerful for measuring the causal effect of a specific change on a specific metric, but they cannot tell you why users behave as they do, whether the problem is real, or whether the solution direction is right. They assume the feature exists and the metric is defined.

Reserve experiments for late-stage measurement questions where the feature is built and the goal is to quantify impact. For earlier questions about problem, value, and direction, use generative and behavioral methods. Using an experiment to answer an understanding question produces a number that is precise about the wrong thing and obscures the real uncertainty.

### Triangulate When The Stakes Are High

For high-stakes decisions, no single method is sufficient, because each has blind spots. Triangulate by combining methods that produce different evidence types: an interview to understand the why, a prototype to test comprehension, a fake-door or concierge test to test willingness, and an experiment to measure impact. Convergent evidence across methods is far stronger than any single result.

Triangulation is not running the same method repeatedly; it is combining methods whose blind spots differ. Three interviews all measure stated intent and share the same weakness. An interview, a behavioral prototype, and a demand test measure different things and cover each other. Reserve triangulation for decisions where the cost of being wrong justifies the extra effort.

## Common Traps

### Defaulting To The Method The Team Always Uses

Reaching for experiments because they feel scientific, or for interviews because they are familiar, ignores whether the method fits the question. The trap is a confident answer to the wrong question. Choose the method from the question, not from habit.

### Using Experiments For Understanding Questions

An A/B test cannot reveal whether the problem is real or why users behave as they do. The trap is a precise number that measures the wrong thing while the real uncertainty stays hidden. Reserve experiments for measurement.

### Trusting Stated Intent For Build Decisions

Users say they would use and pay for things they do not. The trap is building on interview optimism and discovering the gap after launch. Prefer revealed behavior for decisions about investment.

### Jumping To The Most Rigorous-Looking Method

The most rigorous method is not always the right one; a full experiment is overkill for a question a landing page could answer. The trap is spending heavily to resolve a cheap question while a lethal assumption waits. Use the cheapest method that works.

### Ignoring A Method's Blind Spot

Every method fails to prove something, and relying on it as if it were complete produces overconfidence. The trap is acting on evidence whose type does not cover the risk. Name the blind spot before trusting the result.

### Over-Building Before Testing Demand

Building the feature to test whether users want it inverts the order and commits resources before the demand assumption is resolved. The trap is sunk cost that biases the team toward shipping regardless of signal. Test demand cheaply first.

### Treating A Single Method As Sufficient For High Stakes

For consequential decisions, one method's blind spot can hide the fatal flaw. The trap is confidence built on evidence that does not actually cover the risk. Triangulate methods whose blind spots differ.

## Self-Check

- [ ] The chosen method produces the type of evidence, explanatory, behavioral, or causal, that the question requires.
- [ ] The method matches the discovery stage: generative early, prototypes and demand tests mid, experiments late.
- [ ] For build or investment decisions, the primary evidence is revealed behavior, not stated intent.
- [ ] The cheapest method that could change the team's mind about the hypothesis was chosen, and expensive methods are reserved for questions that require them.
- [ ] The blind spot of the chosen method is named, and it does not overlap the riskiest assumption without a complementary method.
- [ ] No experiment is used to answer an understanding question, and no interview is used as the sole evidence for a build decision.
- [ ] For high-stakes decisions, multiple methods with different blind spots are triangulated rather than one method repeated.
- [ ] Demand and willingness were tested cheaply before the full feature was built, not after.
