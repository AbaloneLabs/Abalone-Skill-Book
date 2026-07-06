---
name: social-network-analysis.md
description: Use when the agent is conducting social network analysis, defining nodes and edges in a relational dataset, choosing centrality and clustering measures, interpreting network visualizations, or guarding against reading structural patterns as direct evidence of individual behavior or causation without a clear network theory.
---

# Social Network Analysis

Social network analysis (SNA) studies the structure of relationships among actors rather than the attributes of actors in isolation. Its core insight is that position in a structure shapes outcomes: who you are connected to, and how those connections are arranged, can matter as much as who you are. This structural lens is powerful, but it carries specific pitfalls. Agents often define nodes and edges casually, compute an array of centrality measures, generate attractive visualizations, and then read off findings as if the structure spoke for itself. But a network is only as meaningful as its edge definition, and a centrality score is only as interpretable as the theory linking structure to behavior. The judgment problem is to define the relational data deliberately, to choose measures that match a network theory of the phenomenon, and to resist treating visual patterns or metric rankings as findings without an explanatory argument. An SNA with a vague edge definition and a pile of uninterpreted centralities produces numbers and pictures, not understanding.

Use this skill when conducting social network analysis. The goal is to prevent the agent from defining relations loosely, from computing measures without theoretical purpose, and from reading structure as behavior or cause without justification. The agent has freedom in method and scale, but the data definition, measure selection, and interpretation must be deliberate and theoretically grounded.

## Core Rules

### Define Nodes And Edges Deliberately

The network is defined by what counts as a node and, especially, what counts as an edge. This definition determines everything that follows.

Define by:

- the nodes: people, organizations, documents, or other entities, with clear boundaries;
- the edges: the specific relationship, such as friendship, advice, exchange, communication, or co-occurrence;
- whether edges are directed, valued, or temporal;
- the validity of the chosen relation as a measure of the phenomenon of interest.

An "edge" of co-authorship, friendship, and email exchange are different relations that support different claims. A vague edge definition produces a network whose meaning is unclear and whose measures are uninterpretable.

### Tie Measure Choice To A Network Theory

Centrality, clustering, and other measures are not self-interpreting. Each encodes a specific theory of how structure relates to outcome.

Tie by:

- degree centrality for direct activity or popularity;
- betweenness for brokerage or control of flow;
- closeness for speed of access or information reach;
- eigenvector or PageRank for connection to well-connected others.

Computing every available measure and reporting the rankings is description without theory. The measure must be chosen because it captures the structural mechanism the study cares about.

### Justify The Boundary And Completeness Of The Network

Networks are sensitive to boundary specification and to missing data. These must be addressed, not ignored.

Justify by:

- the boundary rule: how the set of nodes was defined, such as a roster, name generator, or setting;
- whether the network is complete or sampled, and the effect of missing nodes and edges;
- the sensitivity of key measures to missingness;
- acknowledgment that different boundary rules yield different networks.

A network built from a convenience name generator is not the same as one built from a complete organizational roster. The boundary and completeness shape every measure and must be reported.

### Account For Direction, Weight, And Time

Real networks often have directed, weighted, and time-varying edges. Flattening them loses information.

Account by:

- preserving direction where the relation is asymmetric, such as advice-seeking;
- using edge weights where intensity or frequency matters;
- handling temporal networks dynamically rather than collapsing across time;
- justifying any simplification and its consequences.

Treating a directed advice network as undirected, or a weighted exchange network as binary, can invert the meaning of centrality. The structure's richness should be preserved where it matters.

### Interpret Visualizations Critically

Network visualizations are seductive and easily misread. Layout algorithms make aesthetic, not analytic, choices.

Interpret by:

- recognizing that layout is algorithmic and can create or obscure apparent clusters;
- not reading spatial proximity as structural closeness unless the layout encodes it;
- using visualizations as prompts for, not substitutes for, measurement;
- reporting the layout algorithm and key parameters.

A pretty plot where central nodes sit in the middle is a product of the layout, not necessarily of the structure. Visualizations must be read against the measures, not instead of them.

### Avoid The Ecological Fallacy In Network Interpretation

Structural patterns do not directly reveal individual behavior or cause without an explanatory bridge.

Avoid by:

- not inferring individual action from aggregate structure without a mechanism;
- distinguishing structural association from individual-level causation;
- using network theories, such as brokerage or closure, to bridge structure and outcome;
- acknowledging where structural and individual levels may diverge.

A broker position in a network does not by itself prove brokerage behavior; it makes brokerage possible. The link from structure to behavior must be argued.

### Validate The Relational Data

Relational data have their own measurement errors: unreported ties, misrecalled relations, and inconsistent name generators.

Validate by:

- checking the reliability of tie reports, especially in survey-based SNA;
- cross-validating reported ties with other sources where possible;
- assessing the effect of likely missing or false ties on key measures;
- reporting known limitations of the relational data.

Two people may disagree about whether they are friends. Relational data are measurements with error, and that error propagates into structural measures.

### Connect Structural Findings To Substantive Outcomes

SNA earns its place when structure is linked to substantive outcomes: diffusion, performance, inequality, or influence.

Connect by:

- a clear argument for why the structural pattern matters for the outcome;
- evidence linking the measure to the outcome, not just the measure's existence;
- consideration of alternative structural or attribute-based explanations;
- bounding the claim to what the structural evidence supports.

A finding that "this node is central" is a description. The contribution comes from showing what that centrality does or explains in the substantive domain.

## Common Traps

### Loose Edge Definition

A vague or shifting relation makes the network's meaning unclear and its measures uninterpretable.

### Measures Without Theory

Computing every centrality and reporting rankings is description without an explanatory purpose.

### Ignoring Boundary And Missingness

Networks built with different boundary rules or with missing data yield different and potentially wrong structures.

### Flattening Directed Or Temporal Data

Collapsing direction, weight, or time can invert the meaning of structural measures.

### Overreading Visualizations

Treating algorithmic layout as if it revealed structure, or spatial proximity as closeness, misleads interpretation.

### Structure Read As Behavior

Inferring individual action or causation from structural position without a mechanism commits an ecological error.

### Centrality Mistaken For Finding

Reporting that a node is central, without linking structure to a substantive outcome, is description not contribution.

## Self-Check

- Are nodes and, especially, edges defined deliberately, with the relation's validity for the phenomenon justified?
- Is each measure chosen because it captures a specific structural mechanism tied to a network theory?
- Is the network boundary specified, and the effect of missing nodes and edges assessed and reported?
- Are direction, weight, and time preserved where they matter, with simplifications justified?
- Are visualizations interpreted critically against measures, with layout algorithms reported?
- Are structural patterns bridged to individual behavior or causation through an explicit mechanism?
- Is the reliability of relational data assessed, with limitations reported?
- Are structural findings connected to substantive outcomes, not just reported as descriptions?
- Are alternative structural and attribute-based explanations considered?
- Is the claim bounded to what the structural evidence actually supports?
