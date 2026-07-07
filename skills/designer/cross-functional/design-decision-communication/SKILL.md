---
name: design-decision-communication.md
description: Use when the agent is communicating design decisions to non-designers, writing design rationale, presenting design work to engineering product or business audiences, defending decisions with evidence, or ensuring that the reasoning behind design choices is understood and trusted by collaborators who must implement and support them.
---

# Design Decision Communication

Design work that cannot be understood by the people who must implement, approve, and support it does not survive. A design embodies many decisions — why this flow, why this layout, why this tradeoff — and those decisions must be communicated to non-designers whose cooperation the design requires. The judgment problem is that designers understand their own reasoning implicitly and tend to present the design as an artifact ("here is the design") rather than as a set of decisions with rationale ("here is what we decided and why"). The result is that collaborators cannot evaluate the design on its merits, cannot distinguish deliberate choices from unfinished ones, and cannot trust decisions they do not understand, leading to rejection, erosion, or misimplementation.

This skill helps the agent communicate design decisions as reasoned, evidence-grounded arguments that non-design collaborators can understand, evaluate, and trust, rather than as artifacts that must be taken on faith.

## Core Rules

### Communicate decisions and rationale, not just the design artifact

The design file is not the communication; it is the output. What collaborators need is the set of decisions the design embodies and the reasoning behind each. For every significant decision, communicate what was decided, the alternatives considered, and why the chosen option won. A design presented without its rationale is opaque; collaborators either accept it blindly (and then question it later) or reject it because they cannot evaluate it. Make the reasoning as visible as the pixels.

### Ground decisions in evidence, not preference

Non-designers rightly resist design decisions that appear to be matters of taste, because taste is not auditable and everyone has different taste. Ground decisions in evidence: user research, usability findings, analytics, constraints, established principles, and business goals. When a decision is backed by evidence, collaborators can evaluate it against shared standards; when it is backed only by preference, it becomes a contest of opinions that design usually loses to more senior or louder voices. Not every decision needs formal evidence, but significant ones should be traceable to something beyond preference.

### Tailor the argument to the audience's concerns and vocabulary

Engineering, product, and business stakeholders evaluate designs through different lenses and vocabularies. Engineering cares about feasibility, effort, and edge cases; product cares about user outcomes and scope; business cares about impact, risk, and investment. Communicate the same decision in the vocabulary and against the concerns of the audience you are addressing. A rationale framed in design language that ignores the audience's concerns fails to land, even when the decision is sound.

### Connect design decisions to user and business outcomes

Design decisions are most persuasive when connected to the outcomes the organization cares about: does this decision improve task completion, reduce errors, increase conversion, reduce support load, serve an underserved segment. Translate design reasoning into user and business impact, so collaborators see the decision as serving shared goals rather than design preferences. A decision framed as "this reduces checkout abandonment" lands differently than the same decision framed as "this improves visual hierarchy."

### Make alternatives and tradeoffs visible

A decision presented in isolation, with no sense of what was considered and rejected, looks arbitrary. Showing the alternatives that were weighed, and the tradeoffs that led to the chosen option, makes the decision legible and defensible. Collaborators can see that the team considered their concern, understand why a different option was not chosen, and trust that the decision was deliberate. Alternatives also give stakeholders who favored a different option a clear accounting of why their preference was not selected.

### Distinguish firm decisions from open questions

Presenting everything in a design as equally firm or equally open confuses collaborators. Mark which decisions are settled and ready to build, which are open and need input, and which depend on unresolved factors. This lets collaborators focus their input where it is wanted and avoids both challenges to settled decisions and silent acceptance of unresolved ones. Clarity about decision status keeps review productive.

### Communicate constraints and how they shaped the decisions

Many design decisions are responses to constraints: technical limitations, brand requirements, accessibility obligations, content realities, or deadlines. Communicating these constraints explains why decisions took the shape they did, and prevents collaborators from proposing alternatives that violate the same constraints. Constraints are not excuses; they are part of the reasoning, and making them visible turns "why did you do it this way" into an answered question.

### Invite scrutiny through the reasoning, not defensiveness

The goal of communicating rationale is to invite collaborators to scrutinize the reasoning, not to defend the design against all challenge. When rationale is clear and evidence-grounded, good challenges improve the design by revealing flaws in the reasoning. Approach communication as making the decision auditable, so that scrutiny strengthens rather than threatens the work. Defensiveness signals that the reasoning cannot withstand examination.

## Common Traps

### Presenting the artifact without the rationale

A design file without communicated decisions forces collaborators to take it on faith; communicate what was decided and why.

### Grounding decisions in preference

Taste-based decisions lose to louder or more senior opinions; ground significant decisions in evidence and shared standards.

### One-size-fits-all communication

Engineering, product, and business evaluate through different lenses; tailor the argument to the audience's vocabulary and concerns.

### Disconnecting design from outcomes

Design reasoning disconnected from user and business impact reads as preference; connect decisions to outcomes the organization cares about.

### Presenting decisions in isolation

A decision with no visible alternatives looks arbitrary; show what was considered and the tradeoffs that led to the choice.

### Treating all decisions as equally firm or open

Confusing settled and open decisions wastes review on the settled and leaves the open unresolved; mark decision status clearly.

### Hiding the constraints that shaped the design

Unstated constraints lead collaborators to propose alternatives that violate them; make constraints part of the communicated reasoning.

### Defensiveness against scrutiny

Treating challenge as threat signals the reasoning cannot withstand examination; invite scrutiny through clear, evidence-grounded rationale.

## Self-Check

- Are significant decisions communicated with what was decided, the alternatives considered, and why the chosen option won?
- Are decisions grounded in evidence — research, analytics, constraints, principles — rather than preference?
- Is the argument tailored to the audience's concerns and vocabulary, whether engineering, product, or business?
- Are design decisions connected to user and business outcomes the organization cares about?
- Are alternatives and tradeoffs visible, so decisions read as deliberate rather than arbitrary?
- Is it clear which decisions are firm, which are open, and which depend on unresolved factors?
- Are the constraints that shaped the decisions communicated, so collaborators understand the boundaries?
- Is the communication structured to invite scrutiny of the reasoning rather than provoke defensiveness?
