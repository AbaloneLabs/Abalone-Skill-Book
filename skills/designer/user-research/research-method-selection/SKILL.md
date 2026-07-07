---
name: research_method_selection.md
description: Use when the agent is choosing a user research method, deciding between generative and evaluative research, selecting interviews versus surveys versus usability tests versus analytics, planning a research approach for a product question, scoping qualitative versus quantitative work, or justifying why a given method fits the decision and constraints at hand.
---

# Research Method Selection

Choosing a research method is not picking a favorite technique. It is matching the kind of question being asked to the kind of evidence the decision needs, under real constraints of time, access, participants, and confidence. The same product question can be answered weakly or well depending on the method, and a mismatch between question and method produces confident-looking answers that do not actually support the decision.

Agents tend to fail here by reaching for the method they can execute fastest, by confusing what a method can and cannot tell them, or by running a method whose results will be ignored because they do not speak to the decision the team actually faces.

## Core Rules

### Start From The Decision, Not The Method

The first step is not "what method should we use" but "what decision is this research meant to inform." State the decision explicitly.

For each study, define the decision context:

- what choice is pending;
- who owns it;
- what would change it;
- by when it must be made.

A research plan that is not tied to a decision becomes a report that no one acts on. If there is no concrete decision, the research is exploratory and should be labeled as such, with the understanding that its output is direction rather than proof. Tying method to decision also reveals when no research is needed: if the decision will be made the same way regardless of findings, the research is waste.

### Match The Question Type To The Method Family

Different methods answer fundamentally different kinds of questions, and using a method outside its family produces misleading results.

Match question to method family:

- generative or exploratory questions, "what is the problem" or "who are our users," call for field studies, contextual inquiry, diary studies, and open interviews;
- descriptive questions, "how many" or "how often," call for surveys, analytics, and log analysis;
- evaluative questions, "can users do this" or "where do they fail," call for usability tests and first-click studies;
- comparative questions, "which is better," call for A/B tests and benchmark usability studies with enough power to detect a difference.

A survey cannot tell you why users behave a certain way; an interview cannot tell you how many users share a behavior. Confusing these is the most common method-selection error.

### Be Honest About What Each Method Cannot Establish

Every method has a blind spot, and the research plan should name it before the study runs.

Know the characteristic blind spots:

- self-reported data from surveys and interviews suffers from recall bias and social desirability bias, so people misreport what they do and why;
- behavioral data from analytics shows what happened but not why;
- usability tests with small samples reveal severe problems reliably but cannot prove their absence or measure prevalence precisely;
- A/B tests measure outcomes but can miss long-term effects and cannot explain mechanisms.

State the limitations of the chosen method before running it, so that findings are interpreted within their actual evidentiary bounds rather than overclaimed.

### Size The Sample To The Claim

The number of participants must match the strength of the claim being made.

Match sample to claim type:

- qualitative discovery research needs enough participants to reach saturation, the point where new sessions stop revealing new themes, often around five to twelve for a focused question but more for diverse populations;
- quantitative claims about prevalence or difference need samples large enough to support the statistical inference being drawn;
- a survey of thirty people cannot support a claim about "most users."

When the sample cannot support a strong claim, weaken the claim rather than overreading the data. A small qualitative study is valid for discovering problems; it is not valid for estimating their frequency.

### Account For Participants, Access, And Recruitment Reality

A method is only viable if the right participants can be reached. Before committing to a method, confirm feasibility.

Confirm the following before committing:

- the target users are recruitable in the time available;
- you can get the access the method requires, whether home visits, screen sharing, or production data;
- incentives and logistics are feasible.

Consider who is excluded by the recruitment method: online panels over-represent certain populations, in-person studies exclude remote and disabled users, and English-only studies exclude non-English speakers. The sample you can conveniently get is rarely the sample you need.

### Sequence Methods To Cover Each Other's Blind Spots

Single-method research is fragile. Strong research plans combine methods so that the weakness of one is covered by the strength of another. A common productive sequence is qualitative discovery to find the problems, followed by quantitative measurement to size them, followed by evaluative testing to confirm a solution works.

Triangulation, reaching the same conclusion through independent methods, is far more convincing than any single study. Plan the sequence deliberately rather than running isolated studies whose results cannot be combined.

### Decide The Standard Of Evidence The Decision Requires

Not every decision needs the same rigor. Match the investment in research to the cost of being wrong.

Let research investment follow risk:

- a risky, expensive, hard-to-reverse decision, like a major navigation restructure or a pricing change, deserves stronger evidence;
- a low-risk, easily-reversed choice needs less.

State the cost of error explicitly and let it drive the research investment. Over-researching a trivial decision wastes time; under-researching a consequential one invites a costly mistake.

## Common Traps

### Choosing The Method You Can Run Fastest

Picking the method based on speed rather than fit produces data that does not answer the question and a decision made on weak evidence.

### Using A Survey To Answer A "Why" Question

Surveys describe what and how many, not why. Using a survey to infer motivation produces confident numbers around guessed causes.

### Generalizing From A Small Qualitative Sample

Five interviews can reveal a problem exists; they cannot establish how common it is. Do not attach percentages to small qualitative findings.

### Ignoring Recruitment Bias

The users you can easily recruit are often not the users who matter. Online panels, volunteers, and insider testers all skew the sample.

### Confusing Self-Report With Behavior

People are poor witnesses to their own behavior. Treat what users say they do as a hypothesis about what they do, to be checked against behavioral data.

### Running Research Untied To A Decision

Research with no decision attached becomes a deliverable that is filed and forgotten. If nothing will change based on findings, do not run the study.

### Overclaiming From A Single Method

One method's findings are always provisional. Claiming certainty from a single study, especially one whose limitations were not stated, misleads the team.

## Self-Check

- [ ] The decision the research is meant to inform is stated explicitly, with its owner and deadline.
- [ ] The question type, generative, descriptive, evaluative, or comparative, is matched to the correct method family.
- [ ] The limitations and blind spots of the chosen method are stated before the study runs.
- [ ] The sample size matches the strength of the claim; qualitative studies do not claim prevalence, and quantitative claims have adequate power.
- [ ] Recruitment is feasible for the target population, and recruitment bias has been considered.
- [ ] Methods are sequenced or combined to triangulate, covering each other's blind spots where the decision warrants it.
- [ ] The research investment is proportional to the cost of being wrong.
- [ ] No finding is overclaimed beyond what its method and sample can support.
- [ ] Self-reported data is treated as a hypothesis to be checked against behavior, not as established fact.
- [ ] If the decision would not change regardless of findings, the research has been reconsidered or dropped.
