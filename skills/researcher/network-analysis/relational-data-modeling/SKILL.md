---
name: relational-data-modeling.md
description: Use when the agent is modeling relational data, building node and edge schemas for network datasets, handling multilevel and multiplex networks, dealing with one-mode and two-mode affiliation data, or choosing appropriate network statistical models such as ERGMs or SAOMs rather than applying standard regression to dependent observations.
---

# Relational Data Modeling

Relational data, where observations are ties among nodes rather than independent attributes, violate the core assumption of standard statistical methods: independence of observations. Treating network ties as if they were independent rows in a regression produces p-values that are wrong, often dramatically so, because ties in a network are structurally dependent. The judgment problem is to model the relational structure itself: to build a node and edge schema that captures the phenomenon, to handle the specific challenges of multilevel, multiplex, and two-mode data, and to choose statistical models designed for dependent observations, such as exponential random graph models or stochastic actor-oriented models. Agents often import standard regression habits into network data, report significant coefficients, and never realize that the dependence structure invalidates the inference. A relational study with a careless schema and an independence-assuming model produces precise-looking but unreliable results.

Use this skill when modeling relational or network data. The goal is to prevent the agent from violating the independence assumption, from building schemas that distort the phenomenon, and from applying inappropriate models to dependent data. The agent has freedom in model and representation, but the relational nature of the data must govern the modeling choices.

## Core Rules

### Respect The Dependence Structure Of Relational Data

Network ties are not independent. Standard models that assume independence produce invalid inference.

Respect by:

- recognizing that a tie's presence depends on other ties and on the structure;
- not applying ordinary regression or standard errors that assume independence;
- using models designed for dependence, such as ERGMs, SAOMs, or quadratic assignment;
- acknowledging that dependence is the phenomenon, not a nuisance to adjust away.

The dependence among ties is often exactly what the study wants to explain. Treating it as a statistical inconvenience to ignore produces both wrong inference and a missed substantive opportunity.

### Build A Node And Edge Schema That Captures The Phenomenon

The schema determines what the model can represent. A poor schema distorts the phenomenon before modeling begins.

Build by:

- node types and their attributes, clearly defined;
- edge types, whether directed, weighted, or typed;
- whether multiple relations coexist as a multiplex network;
- the temporal granularity if the network evolves.

A schema that collapses distinct relations into one edge, or that omits node attributes central to tie formation, cannot model the mechanisms that actually generate the network. The schema must be designed against the substantive question.

### Handle Two-Mode And Affiliation Data Correctly

Much relational data is two-mode: people affiliated with events, organizations, or documents. These require specific handling.

Handle by:

- preserving the two-mode structure rather than prematurely projecting to one-mode;
- understanding that projection to one-mode invents ties that were not directly observed;
- using methods designed for two-mode or bipartite networks;
- justifying any projection and its consequences for interpretation.

Projecting a person-event network into a person-person network creates co-affiliation ties that may not represent real relationships. The choice to project must be deliberate and its effects understood.

### Model Multiplex And Multilevel Networks Deliberately

Real networks often have multiple relations among the same nodes, or nodes nested across levels.

Model by:

- treating multiplexity, multiple relation types, as part of the structure;
- handling multilevel structure, such as individuals within organizations, explicitly;
- avoiding collapsing levels or relations that should remain distinct;
- using models that can represent the complexity where appropriate.

Collapsing a multiplex network into a single relation, or ignoring the nesting of nodes within higher-level units, loses the structure that the relational perspective was meant to capture.

### Choose Models Designed For Network Dependence

Several model families are built for dependent network data. The choice depends on the question and data.

Choose by:

- ERGMs for cross-sectional network structure and tie dependence;
- SAOMs for longitudinal network evolution through actor-driven processes;
- quadratic assignment procedures for regression-like questions with network autocorrelation;
- the assumptions and data requirements each model imposes.

Each model family encodes specific assumptions about how ties form. The choice must be justified against the substantive theory of tie generation, not by software availability.

### Account For Endogeneity And Network Effects

Networks are shaped by endogenous processes such as reciprocity, transitivity, and preferential attachment.

Account by:

- including structural effects, such as reciprocity or closure, in the model;
- distinguishing endogenous structure from exogenous attribute effects;
- avoiding attributing to attributes what is produced by structure;
- interpreting coefficients in the context of the included structural terms.

A tie may appear driven by homophily when it is actually driven by transitive closure. Models that omit relevant structural terms misattribute the mechanism.

### Assess Model Fit And Degeneracy Carefully

Network models, especially ERGMs, can be degenerate or poorly fitting. Fit must be assessed, not assumed.

Assess by:

- goodness-of-fit checks comparing model-simulated to observed network statistics;
- diagnostics for degeneracy, where the model concentrates probability on near-empty or complete graphs;
- convergence indicators for estimation;
- sensitivity of conclusions to model specification.

A fitted ERGM that looks significant but poorly fits the observed structure explains little. Fit assessment is part of the modeling, not an optional add-on.

### Validate With Simulation And Sensitivity Analysis

Relational models are complex and their behavior can be counterintuitive. Validation builds confidence.

Validate by:

- simulating from the fitted model and comparing to observed features;
- sensitivity analysis to missing ties, boundary choice, and specification;
- comparing alternative model specifications;
- acknowledging uncertainty rather than reporting point estimates as definitive.

A single model specification on a single network can be fragile. Simulation and sensitivity analysis reveal how robust the substantive conclusions are.

## Common Traps

### Independence Assumption Violated

Applying standard regression to network ties produces invalid standard errors and false confidence.

### Premature Two-Mode Projection

Projecting affiliation data to one-mode invents ties that were not observed and distorts the structure.

### Collapsed Multiplex Or Multilevel Structure

Flattening multiple relations or nesting levels loses the complexity the relational view was meant to capture.

### Model Chosen By Software Convenience

Picking a model because it is available, rather than for its assumptions about tie generation, mis-specifies the analysis.

### Omitted Structural Effects

Leaving out reciprocity, transitivity, or other endogenous terms misattributes structure to attributes.

### Unassessed Model Fit

Reporting coefficients without goodness-of-fit or degeneracy checks hides models that explain little.

### Fragile Single Specification

Relying on one model and one network, without simulation or sensitivity analysis, overstates the robustness of conclusions.

## Self-Check

- Is the dependence structure of relational data respected, with models designed for it rather than independence-assuming?
- Does the node and edge schema capture the phenomenon, including direction, weight, and type?
- Is two-mode or affiliation data handled correctly, with projection justified and its effects understood?
- Are multiplex and multilevel structures modeled deliberately rather than collapsed?
- Is the model family chosen for its assumptions about tie generation, justified against the theory?
- Are endogenous structural effects included so mechanisms are not misattributed to attributes?
- Is model fit assessed, including degeneracy and convergence diagnostics?
- Are conclusions validated through simulation and sensitivity to missingness, boundary, and specification?
- Is the interpretation of coefficients grounded in the included structural terms?
- Does the modeling respect the relational nature of the data rather than forcing it into attribute-based habits?
