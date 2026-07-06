---
name: interview_synthesis_and_insights.md
description: Use when the agent is synthesizing findings from multiple customer interviews, identifying patterns across qualitative data, turning interview notes into actionable insights, or deciding what a body of research actually proves about customer needs.
---

# Interview Synthesis And Insights

Synthesis is where research earns or loses its value. A pile of interview notes is not an insight; it is raw material. Synthesis is the disciplined work of finding patterns across respondents, distinguishing signal from noise, separating what is true from what one person said, and translating findings into claims that can drive decisions. Done well, it turns conversations into a shared, accurate picture of the customer that the whole team can act on. Done poorly, it cherry-picks vivid quotes, overweights the most recent interview, and produces conclusions that feel obvious but rest on one or two stories.

The harm this skill prevents is research that collects a lot of data and then discards most of it through sloppy synthesis. Teams remember the memorable interview, quote the respondent who said what they wanted to hear, and declare a pattern after three conversations. The result is decisions based on narrative rather than on what the full body of evidence actually shows.

Use this skill before answering questions such as "what did we learn from these interviews", "what are the key insights", "how do we synthesize this research", or "what should we do with these findings". The goal is to prevent the agent from producing confident insights that are not actually supported by the data collected.

## Core Rules

### Synthesize Across All The Data, Not The Memorable Subset

The mind weights vivid, recent, and emotionally resonant interviews far more than quiet, contradictory ones. After a dozen interviews, the team remembers the two dramatic stories and forgets the ten unremarkable ones that pointed a different way. Synthesis must deliberately re-engage the full body of notes, including the interviews that did not fit the emerging story. A pattern that survives only when you ignore the dissenting cases is not a pattern.

Use a structured method to force completeness: affinity mapping of every quote, a spreadsheet of behaviors per respondent, or a matrix of needs against segments. The structure exists to counter the memory's tendency to cherry-pick. Review the full dataset again before finalizing any claim.

### Distinguish Universal Patterns From Segment-Specific Ones

A behavior that appears across all respondents means something different from one that appears only in a subset. Before claiming an insight, check whether it holds for the whole sample or only for a particular segment. An insight that is true for enterprise buyers but not for self-serve users is still valuable, but only if labeled correctly; presented as universal, it misleads.

Map findings to the respondent attributes that matter: segment, sophistication, geography, company size, tenure with the product. This reveals where patterns are robust and where they are conditional. Conditional insights are useful when the team knows the condition.

### Separate Needs From Solutions And From Feature Requests

Customers describe their world in terms of solutions and feature requests, because that is how they think about products. Synthesis must translate these into the underlying needs and jobs. Ten respondents asking for ten different features may all be reaching for the same underlying need; one respondent's specific request may obscure a need shared by many who articulated it differently.

The work of synthesis is to abstract upward from stated solutions to the problems they imply. Quote the request, but record the need it points to. A feature backlog built from raw requests misses the customers whose need was the same but whose imagined solution was different or unspoken.

### Weight Evidence By Quality And Triangulation

Not all interview findings are equally strong. A claim supported by specific, recent, observed behavior across many respondents is strong. A claim based on one respondent's hypothetical preference is weak. Weight findings by the quality of the underlying evidence: behavioral detail beats stated opinion, multiple independent mentions beat a single vivid story, and triangulation across data sources beats interviews alone.

Make the evidence strength visible in how insights are presented. Distinguish "respondents consistently described doing X" from "one respondent mentioned wishing for X". Conflating strong and weak evidence lets weak findings ride along under the credibility of strong ones.

### Look Actively For Disconfirming Evidence

Synthesis biased toward confirmation notices every quote that fits the emerging story and explains away those that do not. Counter this by deliberately searching the data for the strongest case against the leading insight. If the synthesis cannot account for the disconfirming interviews, the insight is incomplete or wrong. The disconfirming cases are often where the most important learning hides.

State, for each major insight, what the data showed that pushed against it and how that was reconciled. An insight with no acknowledged counter-evidence has not been tested against the full dataset.

### Translate Insights Into Decisions, Not Just Findings

Research that ends with a list of insights has stopped short. Each insight should connect to a decision it informs: what to build, whom to target, what to stop doing, what assumption to test next. If an insight changes no decision, it is interesting but not actionable, and the team should say so rather than imply it carries weight. Link insights explicitly to the roadmap, positioning, or further research questions they raise.

This translation also tests whether the insight is real. An insight that does not change any decision may be too vague to be useful, or may not actually be supported strongly enough to act on.

## Common Traps

### The Vivid Quote Trap

Building the synthesis around one or two memorable stories and treating them as representative. The trap is that vividness is unrelated to prevalence, and the dramatic interview often is the outlier.

### Premature Pattern Closure

Declaring a pattern after three interviews because the early answers align. The trap is that three aligned interviews feel like convergence but may reflect sampling or interviewer bias rather than a real pattern.

### Confirmatory Cherry-Picking

Noticing quotes that support the hypothesis and quietly setting aside those that do not. The trap is a synthesis that confirms the team's prior belief by construction.

### Reporting Requests As Needs

Recording feature requests verbatim and treating them as the finding. The trap is a backlog of literal requests that misses the underlying needs and serves customers who articulated solutions rather than those who shared problems.

### Conflating Segment Findings With Universal Truths

Presenting a segment-specific pattern as if it applied to everyone. The trap is misdirected product investment aimed at a need that only part of the market actually has.

### Insights That Change Nothing

Delivering a list of insights with no connection to decisions. The trap is research that feels complete but exerts no influence, because the link to action was never made.

## Self-Check

- [ ] Synthesis re-engages the full body of notes, including disconfirming and unremarkable interviews, not only the memorable ones.
- [ ] Findings are mapped to respondent attributes to distinguish universal patterns from segment-specific ones.
- [ ] Feature requests and stated solutions have been translated into underlying needs and jobs.
- [ ] Each insight is weighted by evidence quality, and strong and weak evidence are distinguished in how findings are presented.
- [ ] The strongest case against each leading insight was actively sought and reconciled.
- [ ] Each insight is connected to a specific decision it informs, or explicitly labeled as non-actionable.
- [ ] The synthesis distinguishes what the data proves from what it merely suggests.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
