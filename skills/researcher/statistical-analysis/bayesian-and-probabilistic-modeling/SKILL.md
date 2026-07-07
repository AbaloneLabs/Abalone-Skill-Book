---
name: bayesian_and_probabilistic_modeling.md
description: Use when the agent is deciding whether to use Bayesian methods, specifying priors, interpreting posterior distributions, diagnosing MCMC convergence, weighing Bayesian versus frequentist approaches for a specific question, or reporting Bayesian results in a way that honestly conveys the influence of prior choices and computational assumptions.
---

# Bayesian And Probabilistic Modeling

Bayesian methods are powerful, and that is precisely why they are misused. They offer direct probability statements about quantities of interest, incorporate prior evidence, and handle complex hierarchical structures that defeat default frequentist tools. But the same flexibility that makes them attractive makes them easy to deploy without understanding. A researcher supplies a default prior, runs a sampler until it stops complaining, reads a posterior interval, and reports a Bayesian conclusion, never noticing that the prior quietly dominated the data, that the chains never actually converged, or that a simpler frequentist analysis would have answered the question with less machinery and fewer hidden assumptions. Bayesian inference is not a more authoritative version of frequentist inference; it is a different framework whose conclusions are conditional on inputs, especially the prior, that frequentist methods do not require. Using it without justifying those inputs produces confident-sounding answers whose dependence on unexamined choices is invisible.

Use this skill when deciding whether to adopt a Bayesian approach, specifying priors, fitting and diagnosing probabilistic models, interpreting posteriors, and reporting Bayesian results. The goal is to keep the agent from reaching for Bayesian machinery when it adds complexity without benefit, from using default priors without examining their influence, from accepting MCMC output without convergence evidence, and from reporting posterior summaries as if they were prior-free. The agent has latitude in model and software, but the prior, the computation, and the interpretation must each be justified and reported.

## Core Rules

### Choose Bayesian Methods For A Substantive Reason, Not For Prestige

Bayesian approaches earn their cost when they answer questions frequentist methods answer poorly. They are not automatically better, and deploying them where a standard analysis suffices adds assumptions and computation without payoff.

Use Bayesian methods when:

- the question is naturally about the probability of a hypothesis, a parameter range, or a decision;
- prior evidence should formally inform the estimate, as in synthesis of previous studies;
- the model is hierarchical, with partial pooling across groups that frequentist mixed models handle awkwardly;
- samples are small and weakly informative, where shrinkage from a prior stabilizes estimates;
- inference is sequential, with beliefs updated as data accrue;
- the quantity of interest is a complex derived function of parameters with non-standard uncertainty.

When none of these apply, a Bayesian model adds a prior and a sampler to a problem a confidence interval already solves. Match the framework to the question, not the question to a fashionable framework.

### Specify Priors Deliberately And Justify Them

The prior is the defining input of Bayesian analysis, and it is the input most often supplied by default. A prior is not neutral; it encodes beliefs about plausible parameter values and can dominate the data when the data are weak.

Specify priors by:

- choosing informative priors when genuine prior evidence exists, and documenting its source;
- using weakly informative priors to regularize without imposing strong beliefs, and explaining the regularization intended;
- avoiding diffuse or default priors presented as objective, since they can be improperly influential on transformed scales or in complex models;
- checking the prior's implied predictions on the outcome scale, not only on parameters;
- reporting the prior for every parameter and the reasoning behind it.

A prior that looks vague on the parameter scale can imply absurd predictions on the data scale, and vice versa. The prior must be examined where its consequences are felt, which is on the quantities the model predicts.

### Test How Sensitive The Conclusion Is To The Prior

Because the prior influences the posterior, a defensible Bayesian analysis reports how much the conclusion depends on that choice. A result that holds across a range of reasonable priors is far more credible than one that hinges on a single specification.

Probe sensitivity by:

- re-fitting under alternative priors, including more and less informative versions;
- reporting how the posterior mean and interval shift as the prior changes;
- comparing posterior conclusions under informative versus weakly informative priors;
- flagging cases where the prior dominates the data, which signals weak evidence rather than strong findings.

If a defensible change in the prior reverses the substantive conclusion, the data are not decisive, and the conclusion must be reported as prior-dependent. Hiding this dependence overstates what was learned.

### Diagnose Computation Before Trusting The Posterior

Bayesian models for nontrivial problems rely on MCMC or variational methods, and these can fail silently. A posterior summary from a sampler that did not converge is not an estimate; it is a number with no guaranteed relationship to the target distribution.

Diagnose by:

- inspecting trace plots for mixing, stationarity, and the absence of trends or stuck chains;
- checking convergence diagnostics such as split R-hat near one across multiple chains;
- examining effective sample size, which must be adequate for stable posterior summaries;
- assessing divergences, energy diagnostics, and other failure signals specific to the sampler;
- verifying that posterior predictive checks show the model reproduces key features of the data.

A model that reports no errors is not necessarily a model that converged. Convergence is a property to be demonstrated with diagnostics, not assumed because the software returned a number.

### Interpret Posteriors Correctly And Honestly

The posterior is the central Bayesian output, and its interpretation differs from frequentist intervals. Confusing the two produces incoherent claims.

Interpret by:

- stating posterior intervals as probability statements about the parameter given the data and the model, which is the quantity many readers actually want;
- remembering the posterior is conditional on the prior and the model, so it is not a model-free truth;
- reporting the full posterior or its key summaries, not only the point estimate;
- distinguishing posterior probability of a hypothesis from the probability under a single null;
- avoiding the error of treating a posterior interval as if it were a confidence interval with different syntax.

The directness of Bayesian probability statements is a strength, but it is a strength that carries the prior and model assumptions into every conclusion. State those conditions alongside the probability.

### Respect The Computational Cost And Complexity

Bayesian models can be expensive to fit and hard to debug. That cost is justified only when it buys inferential value the simpler approach cannot.

Weigh cost by:

- comparing the Bayesian model's conclusions to a simpler frequentist baseline, to confirm the complexity is earning its keep;
- recognizing that more complex hierarchical or non-linear models require more diagnostic effort, not less;
- ensuring reproducibility through fixed seeds, recorded software versions, and saved model fits;
- planning for the time and expertise to specify, debug, and validate the model.

A Bayesian model whose results mirror a straightforward regression, at ten times the computational and interpretive cost, has added assumptions without adding insight. Complexity must be paid for in clarity of conclusion.

### Report Bayesian Results With Their Full Conditions

Bayesian reporting must expose the inputs that shape the output. Omitting the prior and the diagnostics turns a conditional inference into an apparently objective one.

Report:

- the model specification and the rationale for its structure;
- the priors for every parameter and the reasoning behind each;
- the sensitivity of conclusions to alternative priors;
- the convergence diagnostics and effective sample sizes;
- posterior summaries, intervals, and posterior predictive checks;
- software, version, seed, and enough detail to reproduce the fit.

A Bayesian result reported without its prior and diagnostics cannot be evaluated, because the reader cannot see what produced it.

## Common Traps

### Bayesian For Prestige Rather Than Fit

Choosing a Bayesian model because it sounds advanced, when a simpler analysis answers the question, adds assumptions and cost without benefit.

### Default Priors Treated As Objective

Presenting software defaults as neutral hides prior influence, especially on transformed or derived scales where vague priors can be improperly informative.

### Prior Dominating Weak Data

When data are sparse, an unexamined prior can drive the posterior, producing a confident conclusion that reflects belief rather than evidence.

### Trusting Unconverged Chains

Accepting posterior summaries without trace plots, R-hat, and effective sample size risks reporting numbers unrelated to the target distribution.

### Confusing Posterior And Confidence Intervals

Treating a Bayesian interval as a frequentist one, or vice versa, conflates two different conditional quantities and misleads interpretation.

### Complexity Without Payoff

Building an elaborate probabilistic model whose conclusions duplicate a simple regression wastes effort and obscures the analysis with unnecessary assumptions.

### Undisclosed Prior And Diagnostics

Reporting posterior results without priors, sensitivity analysis, and convergence checks prevents readers from judging what shaped the conclusion.

### Overstating Direct Probability

Presenting posterior probabilities as model-free truths hides their dependence on the prior and model specification that produced them.

## Self-Check

- Is the Bayesian approach chosen for a substantive reason, such as prior synthesis, hierarchical structure, small samples, or direct probability questions, rather than for prestige?
- Is every prior specified deliberately, with the source of informative priors documented and the regularization intent of weakly informative priors explained?
- Has the prior's implied predictions been checked on the outcome scale, not only on parameters?
- Is a sensitivity analysis reported showing how conclusions shift under alternative, defensible priors?
- Are convergence diagnostics, including trace plots, R-hat, effective sample size, and divergences, inspected and reported?
- Does a posterior predictive check confirm the model reproduces the key features of the data?
- Are posterior intervals interpreted as probability statements conditional on the prior and model, not as model-free truths or as frequentist confidence intervals?
- Is the Bayesian conclusion compared against a simpler baseline to confirm the complexity earns its keep?
- Are the model, priors, sensitivity analysis, diagnostics, posterior summaries, software, version, and seed reported in enough detail to reproduce the fit?
- Would a reader be able to see how much the conclusion depends on prior and modeling choices rather than on the data alone?
