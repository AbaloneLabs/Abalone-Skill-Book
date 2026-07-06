---
name: predictive_turnover_and_capacity_modeling.md
description: Use when the agent is building predictive turnover models or workforce capacity models, selecting predictors and validating them, forecasting future headcount and skill supply against demand, modeling scenario impacts on workforce capacity, or avoiding the failures of overfit models, spurious predictors, and capacity plans that ignore skill mix and demand uncertainty.
---

# Predictive Turnover And Capacity Modeling

Predictive turnover modeling estimates who is likely to leave and at what rate, so the organization can intervene before departures occur. Capacity modeling estimates whether the workforce has the right number of people with the right skills to meet future demand. Both are forecasting exercises, and both fail in characteristic ways. Predictive models fail when they overfit historical data, when they rely on predictors that are proxies for protected characteristics, or when they are used to label individuals rather than to direct retention effort. Capacity models fail when they treat headcount as a single number rather than a matrix of skills, when they assume demand is certain, or when they ignore the lead time required to build or acquire capability. The recurring failure across both is to produce a precise-looking forecast and treat it as a prediction rather than as a scenario-dependent estimate with uncertainty, leading to decisions made with false confidence. This skill covers the disciplined construction and use of turnover and capacity models, and the skepticism required to use them well.

Use this skill when building or validating a turnover prediction model, constructing a workforce capacity or supply-and-demand model, or running scenarios for growth, restructuring, or skill transformation. The goal is to make the agent build models that are valid, transparent about uncertainty, and used to inform rather than to replace judgment.

## Core Rules

### Treat Models As Estimates With Uncertainty, Not As Predictions

A workforce model produces a forecast conditioned on assumptions about the future, not a prediction of what will happen. Treating the output as a prediction produces decisions made with false confidence; treating it as an estimate with uncertainty produces decisions that account for what could go differently.

- Express forecasts as ranges or scenarios, not single points, because a single number conceals the uncertainty that determines whether the decision is robust.
- State the assumptions the model depends on (demand growth, attrition rates, hiring lead times, productivity) explicitly, because the forecast is only as good as the assumptions.
- Test the sensitivity of the forecast to changes in the key assumptions, because a forecast that flips under plausible assumption changes is not a basis for commitment.
- Communicate uncertainty to decision-makers, because a forecast presented without uncertainty is a forecast that will be over-trusted and that will produce blame when it is wrong.

### Validate Predictors Against Outcomes, And Watch For Proxies

A predictive turnover model is only as good as its predictors, and predictors must be validated against actual outcomes rather than assumed. The further danger is that some predictors are proxies for protected characteristics, which creates legal and ethical exposure.

- Validate predictors against historical outcomes using holdout samples or out-of-time validation, because a model that fits the training data but does not predict holdout data is overfit and will not generalize.
- Be alert to predictors that are proxies for age, gender, ethnicity, disability, or other protected characteristics (tenure can proxy for age; certain leave patterns can proxy for family status), because using them can constitute unlawful discrimination even without intent.
- Prefer predictors that are actionable (engagement, manager relationship, compensation positioning) over predictors that are not (demographic attributes), because a predictor that cannot be acted on cannot drive retention.
- Re-validate periodically, because the relationships that predicted turnover last year may not predict it this year as conditions change.

### Model Turnover For Intervention, Not For Labeling Individuals

A turnover model's value is in directing retention effort where it will do the most good, not in labeling individuals as flight risks. The latter use creates harm, mistrust, and legal exposure, and often becomes a self-fulfilling label.

- Use the model to identify roles, teams, or segments where retention risk is concentrated, so intervention can be targeted.
- Avoid using individual flight-risk scores to make employment decisions (assignment, promotion, termination), because doing so converts a statistical estimate into an individual judgment the model cannot support and may constitute discrimination.
- Be transparent with employees about what is modeled and why, within the bounds of privacy, because secret scoring erodes trust and may violate data protection law.
- Recognize that an individual flagged as low-risk can leave and one flagged as high-risk can stay; the model describes probability, not destiny.

### Model Capacity As Skill Mix, Not Just Headcount

A capacity model that treats the workforce as a single headcount number will conclude the organization has enough people when it lacks the right skills, or that it is overstaffed when it has too many of the wrong people. Capacity is a matrix of skills, levels, and locations against demand.

- Decompose capacity into the skill and level dimensions that matter for the work, because aggregate headcount conceals the skill gaps that determine whether demand can be met.
- Model current supply by skill and level, accounting for attrition, internal movement, and development pipelines, because supply is not static.
- Model demand by skill and level, recognizing that different scenarios imply very different skill requirements, because a growth scenario and a transformation scenario demand different capabilities.
- Identify the gaps — where supply will not meet demand — and the lead time to close them, because the lead time determines how early action must begin.

### Incorporate Demand Uncertainty Through Scenarios

Demand is not known; it is estimated, and capacity decisions made against a single demand estimate are fragile. Scenario modeling tests whether the capacity plan is robust across plausible futures.

- Build scenarios that span the plausible range of demand (growth, stability, contraction, transformation) rather than a single central case.
- Test the capacity plan against each scenario, identifying where it is robust and where it breaks, because a plan that works only in the central case is a plan that assumes away uncertainty.
- Identify no-regret moves that are valuable across scenarios (building transferable skills, reducing single-points-of-failure) versus bets that pay off only in specific scenarios.
- Avoid committing to irreversible capacity decisions (large hires, large separations) on the basis of a single demand scenario, because the cost of being wrong is asymmetric.

### Account For Lead Times In Building And Acquiring Capability

Capability cannot be instantiated instantly. Hiring, developing, and redeploying all have lead times, and a capacity plan that ignores them will commit to timelines that cannot be met.

- Estimate realistic lead times for hiring (sourcing, selection, notice, onboarding), development (time to competence), and redeployment (retraining, role transition).
- Build the lead times into the plan, so that action to close a gap begins early enough to close it by the time it is needed.
- Recognize that lead times lengthen under scarcity, because the time to hire a scarce skill increases when the market is tight.
- Distinguish build (develop internally, longer lead time, more durable) from buy (hire externally, shorter lead time, less durable) strategies, and choose based on lead time, scarcity, and strategic importance.

### Govern Model Use And Prevent Misuse

Models are tools that can be misused, and governance is what keeps their use aligned with their purpose. Ungoverned models drift toward uses they were not designed for and cannot support.

- Define the permitted uses of the model (directing retention effort, planning capacity) and the prohibited uses (individual employment decisions), and enforce the boundary.
- Review the model's inputs, predictors, and outputs for bias and proxy risk on a rhythm, because drift in data or relationships can introduce bias over time.
- Document the model's methodology, validation, and limitations, so that users understand what it can and cannot support.
- Subject consequential decisions informed by the model to human review, because a model is an input to judgment, not a replacement for it.

## Common Traps

### The Overfit Model

The model is tuned to predict historical turnover with high accuracy on the data it was trained on, and is deployed with confidence in its predictive power. The trap is that a model that fits historical data perfectly often captures noise rather than signal, because it has learned the idiosyncrasies of the training period rather than the relationships that generalize, so it predicts the past beautifully and the future poorly. The precision on historical data produces false confidence, and the model is trusted to forecast turnover it cannot actually forecast, leading to retention effort directed at the wrong people and gaps that appear where the model said they would not. Out-of-sample and out-of-time validation is what distinguishes a model that generalizes from one that memorizes, because a model that has not been tested on data it did not train on is a model whose predictive power is unverified, however impressive its fit.

### Predictors That Are Proxies For Protected Characteristics

The model uses tenure, leave patterns, commute distance, or other predictors that correlate with turnover, without recognizing that these same predictors are proxies for age, family status, disability, or other protected characteristics. The trap is that using such predictors can produce disparate impact — the model systematically flags members of a protected group as higher flight risk — which constitutes unlawful discrimination regardless of intent, and which exposes the organization to legal liability, regulatory action, and the reputational damage of algorithmic bias. The absence of direct demographic inputs does not make a model fair, because proxies carry the same information and produce the same disparate outcomes. Auditing predictors for proxy risk and disparate impact is what prevents a retention tool from becoming a discrimination instrument, because the legality of a model depends on its effects, not on the labels of its inputs.

### Labeling Individuals As Flight Risks

The model produces individual flight-risk scores, and managers use them to make assignment, development, or termination decisions, treating a statistical estimate of leaving as a judgment about the person. The trap is that a flight-risk score is a probability conditional on current data, not a destiny, and using it to make individual employment decisions converts a population-level estimate into an individual verdict the model cannot support, often producing self-fulfilling prophecies (the employee labeled high-risk is denied opportunities, becomes disengaged, and leaves) and frequently disparate impact on protected groups. The model's value is in directing retention effort toward where risk concentrates, not in adjudicating individuals, because the model cannot know what it has not measured and the individual is more than the model's variables. Restricting use to population and segment-level intervention is what keeps a turnover model a retention tool rather than a discrimination mechanism.

### Headcount Capacity Without Skill Mix

The capacity model concludes the organization has the headcount it needs, based on aggregate numbers against aggregate demand, while the skill composition is wrong — too many of one skill, too few of another — so the organization cannot meet demand despite appearing fully staffed. The trap is that aggregate headcount conceals the skill structure that determines whether work can actually be done, and a plan built on headcount alone will hire or retain the wrong people, producing the appearance of adequate capacity and the reality of unmet demand. Capacity is a matrix of skills, levels, and locations against demand decomposed the same way, because ten people with the wrong skills do not equal ten people with the right skills, and only skill-level modeling reveals the gaps that determine whether the work gets done.

### Single-Scenario Demand Planning

The capacity plan is built against a single central demand forecast and commits to hiring or separation accordingly. The trap is that demand is uncertain, and a plan optimized for one scenario breaks in the others, because the hires that were right under growth are excess under contraction, and the separations that were right under contraction create shortages under growth. A single-scenario plan assumes away the uncertainty that defines the planning problem, and the organization commits to irreversible decisions on the basis of a future that does not arrive. Scenario-based planning that tests the capacity strategy across plausible futures, identifies no-regret moves, and avoids irreversible commitments on single-scenario assumptions is what produces a plan robust to the uncertainty it will actually face, because the cost of being wrong is borne in the scenarios the plan did not consider.

### Ignoring Lead Times

The capacity plan identifies a skill gap that will emerge in eighteen months and schedules action to begin in twelve, on the assumption that capability can be built quickly. The trap is that hiring, developing, and redeploying all have lead times measured in months, and lead times lengthen under scarcity, so action begun at twelve months produces capability at twenty, missing the gap that opened at eighteen and forcing a reactive external hire at premium cost or an unmet demand. The lead time to build or acquire capability is the constraint that determines how early planning must begin, and a plan that ignores it commits to timelines that cannot be met, producing the gaps it was meant to prevent. Building realistic lead times into the plan, and starting action early enough to close gaps by the time they emerge, is what makes capacity planning effective rather than aspirational.

## Self-Check

- [ ] Are model forecasts expressed as ranges or scenarios with stated assumptions, rather than single-point predictions presented without uncertainty?
- [ ] Have predictors been validated out-of-sample or out-of-time, and audited for proxy risk and disparate impact on protected groups?
- [ ] Is the turnover model used to direct retention effort at segments and roles, not to label individuals or make employment decisions about specific people?
- [ ] Does the capacity model decompose supply and demand by skill, level, and location, rather than treating headcount as a single number?
- [ ] Is demand modeled across scenarios spanning the plausible range, with no-regret moves identified and irreversible commitments avoided?
- [ ] Are realistic lead times for hiring, development, and redeployment built into the plan, with action started early enough to close gaps?
- [ ] Are permitted and prohibited uses of the model defined and enforced, with methodology, validation, and limitations documented?
- [ ] Are consequential decisions informed by the model subject to human review, with the model treated as an input to judgment rather than a replacement for it?
