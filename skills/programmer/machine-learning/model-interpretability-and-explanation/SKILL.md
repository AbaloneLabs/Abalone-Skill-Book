---
name: model_interpretability_and_explanation.md
description: Use when the agent is building or evaluating model interpretability or explanations (feature importance, SHAP, LIME, counterfactuals, partial dependence, attention or saliency for deep models), deciding what level of explainability a model needs, explaining an individual prediction to a user or auditor, validating whether an explanation is faithful to the model, or balancing interpretability against accuracy for regulated or high-stakes domains. Also covers the failure modes of explanations that are misleading or unfaithful to the model, treating feature importance as causation, global vs local explanation confusion, saliency maps that highlight noise, trusting an explanation method without validating it, and the recurring mistake of bolting on an explanation method after deployment to satisfy a requirement rather than designing the model and its explanations together.
---

# Model Interpretability And Explanation

Many models that are accurate on aggregate are deployed into decisions where someone is entitled to ask "why?" — a loan applicant denied credit, a patient flagged as high-risk, a user whose content was removed, a regulator auditing a hiring model. The judgment problem is that "why did the model produce this output?" is a distinct and harder question than "what did the model predict," and the tools that answer it (feature importance, SHAP values, counterfactuals, saliency) are easy to run and easy to misread. An explanation that is not faithful to the model describes a model that does not exist. A feature-importance ranking read as causation leads to interventions that do not work. A global explanation ("age is the most important feature overall") is conflated with a local one ("age drove this specific prediction"). A saliency map highlights pixels that look plausible but reflect noise. The discipline is to decide what kind of explanation is actually needed (global understanding, local justification, debugging, compliance), choose a method whose faithfulness is validated, and communicate the explanation with its limits — never presenting a post-hoc rationalization as proof of how the model reasons.

Agents tend to under-invest here because the tooling produces a clean output (a SHAP plot, an importance bar chart) that looks like an answer, and because interpretability is often treated as a compliance checkbox added after the model is built. The harm appears when the explanation is consumed. An unfaithful explanation misleads a user about why they were denied, and a regulator about whether the model is fair. A feature-importance score read causally ("income is most important, so raising income thresholds will improve outcomes") drives an intervention that fails, because importance is not causation. A local explanation is mistakenly applied globally. An explanation method is trusted without validation, and its outputs diverge from the model's actual behavior. The judgment problem is to match the explanation to the question, validate faithfulness, distinguish correlation from causation, and design interpretability into the model and its deployment rather than bolting it on.

This skill covers the types of explanation and when each matters, faithfulness and validation, the correlation-vs-causation trap, explanation design for users and auditors, and the interpretability-vs-accuracy tradeoff. It complements the model-evaluation-and-metrics skill (overall model quality), the drift-detection skill (when explanations change as the model drifts), and the training-data-quality skill (bias that explanations surface). Here the focus is explaining model behavior faithfully and appropriately.

## Core Rules

### Decide What Kind Of Explanation Is Needed Before Choosing A Method

"Explainability" is not one thing. Different stakeholders and decisions need different kinds of explanation, and the method must match the question:

- **Global explanation: how does the model behave overall?** Which features drive predictions across the whole dataset, what are the learned relationships (a partial dependence curve, global feature importance). Useful for model understanding, debugging, and auditing for systematic bias.
- **Local explanation: why this specific prediction?** Which features drove a particular decision for a particular instance (a SHAP or LIME explanation for one applicant). Required for individual justifications ("why was I denied?") and for case-by-case review.
- **Counterfactual explanation: what would have to change?** "You would have been approved if your income were X higher." Often the most actionable explanation for an affected user, and increasingly required by regulation.
- **Match the method to the stakeholder.** A data scientist debugging wants global relationships; an affected user wants a counterfactual or a local reason; a regulator wants evidence of fairness and absence of protected-attribute reliance. Do not serve a global importance chart to a user asking why they were denied.

### Validate That An Explanation Is Faithful To The Model

An explanation method produces output regardless of whether it reflects the model; an unfaithful explanation describes a model that does not exist. Faithfulness must be validated, not assumed:

- **Prefer exact methods where available.** For tree models, SHAP has an exact polynomial-time algorithm; for linear models, the contribution is exact. Prefer these over approximate methods where the model admits them, because exactness removes a source of unfaithfulness.
- **Test explanation faithfulness empirically.** Remove or perturb the features an explanation says are important and confirm the prediction changes accordingly; if removing the "most important" feature does not change the prediction, the explanation is not faithful. Correlate explanation importance with actual sensitivity.
- **Beware methods that disagree.** Different explanation methods (SHAP, LIME, gradients, permutation importance) can produce different rankings for the same model; large disagreement is a signal that none should be trusted blindly. Investigate the disagreement rather than picking the explanation that suits the story.
- **Do not trust saliency for deep models without scrutiny.** Saliency and attention maps for neural models can highlight noise or be unfaithful to the actual decision path; treat them as hypotheses about the model, not ground truth, and validate where possible.

### Never Confuse Feature Importance With Causation

The most damaging misreading of an explanation is treating "feature X is important to the model" as "feature X causes the outcome." Importance reflects correlation the model uses, not causation:

- **Importance means the model relies on the feature, not that the feature causes the label.** A model may rely heavily on a feature that is correlated with the label via a confounder, and intervening on that feature will not change the outcome (see the causal-inference skill). "Zip code is the most important feature" does not mean changing zip code changes creditworthiness.
- **Do not derive interventions from importance alone.** "Income is important, so we should raise the income threshold" assumes income causes repayment, which the model did not establish. Interventions require causal evidence, not predictive importance.
- **Explain the distinction to consumers of the explanation.** Stakeholders and users will read importance causally by default; state explicitly that the explanation describes what the model uses, not what causes the outcome, to prevent misinformed action.

### Distinguish Global From Local Explanations And Do Not Substitute One For The Other

A global explanation describes the model's average behavior; a local explanation describes one prediction. They can disagree, and conflating them misleads:

- **A globally important feature may not drive a specific prediction.** "Age is the most important feature overall" does not mean age drove this particular applicant's denial; for that individual, a different feature may dominate. Do not justify a specific decision with a global ranking.
- **A locally important feature may be globally minor.** A feature that rarely matters but dominates a specific edge case will not appear in global importance but is the correct local explanation. A global-only view misses these cases.
- **Use the right scope for the question.** "Is the model fair across the population?" is global; "why was this user denied?" is local. Serving the wrong scope produces an answer to a different question.

### Design Explanations For The Audience That Consumes Them

An explanation is communication, and its usefulness depends on whether the audience can understand and act on it:

- **Translate to the user's vocabulary.** A SHAP value or a log-odds contribution is meaningless to a denied applicant; translate to a reason ("your recent account history contributed to this decision") and, where useful, a counterfactual ("a higher score in X would have changed the outcome").
- **Provide actionable explanations where possible.** "Feature 47 was important" is not actionable; "reducing your debt-to-income ratio would improve your chances" is. Counterfactuals are often the most useful form for affected users.
- **Limit the explanation to what matters.** A full SHAP decomposition of hundreds of features is noise to a user; surface the top contributing reasons. Over-explaining is as unhelpful as under-explaining.
- **Be honest about uncertainty and limits.** An explanation is a simplification of a complex model; communicate that it is an approximation of the model's reasoning, not a complete causal account, and that it may not capture every factor.

### Treat Interpretability As A Design Constraint, Not A Post-Hoc Patch

The hardest interpretability problems arise when a black-box model is built first and explanations are demanded later. Designing for interpretability from the start is often more effective:

- **Consider an inherently interpretable model when explanations are required.** For regulated or high-stakes decisions, a model whose reasoning is transparent by construction (a sparse linear model, a small decision tree, a rule-based model with an auditable rule set) may be preferable to a black box with a post-hoc explainer, even at some accuracy cost.
- **Weigh the interpretability-accuracy tradeoff against the stakes.** A complex model's accuracy advantage must justify the opacity and the risk that post-hoc explanations are unfaithful or contested. For low-stakes predictions, accuracy may dominate; for high-stakes, individual decisions, interpretability may be worth the accuracy cost.
- **Validate explanations during development, not after deployment.** Build explanation faithfulness checks into model evaluation, so an unfaithful or misleading explainer is caught before it reaches users or auditors.
- **Document the explanation method and its limits.** Record which explanation method is used, its validated faithfulness, and what it does and does not establish, so consumers (and future maintainers) understand the explanation's scope and reliability.

## Common Traps

### Unfaithful Explanations That Mislead

Running an explanation method and trusting its output without validating that it reflects the model, so the explanation describes a model that does not exist. Validate faithfulness (perturbation tests, method agreement) and prefer exact methods where available.

### Feature Importance Read As Causation

Treating "feature X is important to the model" as "feature X causes the outcome," leading to interventions that fail because importance is correlation, not causation. State the distinction explicitly; derive interventions from causal evidence.

### Global Vs Local Confusion

Justifying a specific decision with a global feature ranking, or assessing population fairness with a single local explanation. Match the explanation's scope (global vs local) to the question being asked.

### Saliency Maps Highlighting Noise

Treating saliency or attention maps for deep models as ground truth about the decision, when they may highlight noise or be unfaithful. Treat them as hypotheses and validate where possible.

### Trusting An Explanation Method Without Validation

Adopting SHAP, LIME, or permutation importance and assuming the output is correct, when methods can disagree and individual outputs can be unfaithful. Investigate disagreement and test faithfulness empirically.

### Bolting Explanations On After Deployment

Building a black-box model first and demanding explanations later to satisfy a requirement, when an inherently interpretable model designed for the stakes would have been more appropriate. Design interpretability in; validate during development.

### Over-Explaining Or Under-Explaining For The Audience

Dumping a full feature decomposition on a user who needs one actionable reason, or giving a vague global statement to someone asking why their specific decision was denied. Translate to the audience's vocabulary and surface the relevant, actionable reasons.

## Self-Check

- [ ] The kind of explanation needed was identified (global for model understanding/auditing, local for individual justification, counterfactual for actionable "what would change"), and the method matches the stakeholder and question.
- [ ] Explanation faithfulness was validated (perturbation tests confirming important features actually affect predictions, method-agreement checked, exact methods preferred where the model admits them), and saliency for deep models is treated as a hypothesis, not ground truth.
- [ ] Feature importance is not read as causation: the explanation describes what the model uses, not what causes the outcome, and interventions are derived from causal evidence, not predictive importance; this distinction is communicated to consumers.
- [ ] Global and local explanations are distinguished and used for the right questions: a specific decision is justified with a local explanation, population fairness assessed globally, and the two are not substituted for each other.
- [ ] Explanations are designed for the audience: translated to the user's vocabulary, actionable (counterfactuals where useful), limited to the relevant reasons, and honest about being an approximation with limits.
- [ ] Interpretability was treated as a design constraint: an inherently interpretable model was considered for high-stakes decisions, the interpretability-accuracy tradeoff was weighed against the stakes, faithfulness was validated during development, and the method and its limits are documented.
- [ ] The highest-risk cases were verified — an unfaithful explanation caught by perturbation, an importance score misread as causal, a global ranking misapplied to a local decision, and a black box whose post-hoc explainer disagreed with the model — not only the clean aggregate importance chart.
