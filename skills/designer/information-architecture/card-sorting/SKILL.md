---
name: card_sorting.md
description: Use when the agent is planning or running a card sort, choosing between open and closed card sorting, deciding on the number of participants, designing the cards and categories, interpreting results to inform information architecture, or validating a proposed site structure through tree testing.
---

# Card Sorting

A card sort is a research method for discovering how users naturally group and label content. It looks like a simple exercise of sorting cards into piles, but it is really a structured way to surface user mental models that should drive navigation, taxonomy, and content structure. Agents tend to treat card sorting as a checkbox activity, run it with too few participants, choose cards that bias the results, or read the output as a prescription rather than evidence. The harm is an information architecture built on assumptions that users then struggle to navigate, with no clear data showing why.

Use this skill before running a card sort or when interpreting its results to inform structure. The goal is to prevent the agent from designing a sort that cannot answer the question, from over-interpreting weak data, or from treating participant groupings as a finished sitemap rather than as input to a broader design process.

## Core Rules

### Start From The Specific IA Question, Not The Method

A card sort is not self-justifying. Before running one, state the question it is meant to answer, because the question determines the method, the cards, and the analysis. Running a sort because "we should do research" produces data no one acts on.

Define the question first:

- are we trying to discover how users group content we have not yet structured (open sort)?
- are we testing whether a proposed structure works (closed sort)?
- are we comparing structures or label variants (hybrid or reverse sort)?
- what decision will the results inform, and who owns it?

A sort untied to a decision becomes a report that is filed and forgotten. If the structure will be the same regardless of findings, the sort is waste.

### Choose Open, Closed, Or Hybrid Based On The Question

The three card sort types answer different questions, and choosing the wrong type produces results that do not speak to the decision.

Match type to question:

- open sorts, where participants create and name their own categories, suit discovery: you do not yet know how users group content and want their mental models to emerge;
- closed sorts, where participants place cards into predefined categories, suit evaluation: you have a proposed structure and want to test whether content lands where users expect;
- hybrid sorts, where participants can create new categories alongside predefined ones, suit refinement: you have a structure but suspect it has gaps.

An open sort cannot validate a proposed structure. A closed sort cannot discover unknown groupings. Match the type to where you are in the design process.

### Select Cards That Represent The Content Without Biasing Results

The cards are the stimulus, and their selection shapes everything. Too few cards miss the structure; too many overwhelm participants. Cards that are too abstract or too similar force arbitrary groupings. Cards chosen to confirm a hypothesis bias the outcome.

Choose cards carefully:

- select forty to sixty cards for a typical sort: enough to reveal structure, few enough to complete in a reasonable session;
- choose cards that represent real content users would seek, not internal categories or admin pages;
- use user-facing language on the cards, not internal jargon;
- avoid cards so similar that participants cannot distinguish them, and so different that grouping is trivial;
- include cards that span the full content scope so the structure is not skewed toward one area.

Card selection is where bias most easily enters. Have someone outside the design team review the card set for balance and neutrality.

### Size The Participant Group To The Strength Of The Claim

Card sort results are only as reliable as the sample behind them. A handful of participants can suggest patterns; they cannot prove a structure. Agents often run a sort with five people and present the groupings as definitive.

Match sample to claim:

- for qualitative discovery (open sorts), fifteen to twenty participants across distinct user segments is a practical range to see consistent patterns emerge;
- for quantitative evaluation (closed sorts), larger samples produce more reliable agreement statistics;
- segment participants by user type, because different audiences often group content differently, and averaging across segments hides real conflict;
- do not claim a structure is validated by users if the sample cannot support that claim.

If the sample is small, present findings as directional insight, not as a prescription.

### Analyze For Patterns, Not For A Single Answer

Card sort data is messy. Participants group differently, create idiosyncratic categories, and disagree. The goal of analysis is to find the patterns strong enough to act on, not to force a single consensus structure that no individual actually produced.

Analyze deliberately:

- use similarity matrices and dendrograms (for open sorts) to see which cards consistently group together;
- look for cards that scatter widely across categories, which signal ambiguity or a labeling problem;
- compare groupings across user segments to find where mental models diverge;
- treat outlier groupings as signals to investigate, not noise to discard.

A card sort does not produce a sitemap. It produces evidence about how users think, which the designer synthesizes into a structure alongside other inputs.

### Follow A Sort With Tree Testing To Validate The Resulting Structure

A card sort reveals how users group content in isolation, but a real navigation requires users to find specific items within a structure. A structure that sorts well can still fail when users must navigate it under task pressure. Tree testing closes this gap.

Sequence the methods:

- run the card sort to inform the proposed structure;
- build a candidate navigation from the findings;
- run a tree test where users find specific items in the proposed structure, without visual design;
- measure first-click accuracy, directness, and time to find.

Tree testing catches problems a card sort cannot: labels that work in isolation but fail in context, categories that are too deep, and items placed where users do not expect them.

### Report Limitations Alongside Findings

Card sorts have known limitations, and overclaiming from them misleads the team. Participants sort in a vacuum without the context of real tasks, visual design, or prior knowledge. Their groupings are preferences, not behavior.

State the limitations:

- card sorts show how users would group content, not how they actually navigate;
- the card set constrains the possible findings; missing content cannot appear in the results;
- groupings reflect the participants recruited, who may not represent all users;
- findings are input to design, not a prescription to implement verbatim.

Honest limitation reporting keeps the findings credible and prevents the team from over-indexing on one method.

## Common Traps

### Running A Sort Without A Defined Question

A card sort untied to a specific IA question and decision produces data no one acts on; define the question before choosing the method.

### Choosing The Wrong Sort Type

An open sort cannot validate a proposed structure, and a closed sort cannot discover unknown groupings; match the type to where you are in the design process.

### Biasing Results Through Card Selection

Cards that are too abstract, too similar, or chosen to confirm a hypothesis skew the findings toward the designer's assumptions.

### Overclaiming From A Small Sample

Five participants can suggest patterns but cannot validate a structure; size the sample to the strength of the claim and segment by user type.

### Treating Groupings As A Finished Sitemap

A card sort produces evidence about mental models, not a prescription; the designer must synthesize findings into a structure and validate it separately.

### Skipping Tree Testing

A structure that sorts well can fail under task navigation; follow the sort with tree testing to validate findability in context.

### Ignoring Cross-Segment Differences

Averaging groupings across different user types hides real conflicts in mental models that should inform distinct navigation strategies.

### Presenting Findings Without Limitations

Overclaiming from a method with known constraints misleads the team and undermines the research's credibility.

## Self-Check

- [ ] The specific IA question and the decision it informs are stated before the sort is designed.
- [ ] The sort type (open, closed, hybrid) matches the question and the stage of the design process.
- [ ] The card set (typically forty to sixty cards) represents real content in user language, spans the full scope, and was reviewed for bias.
- [ ] The participant sample is sized to the claim, segmented by user type, and not over-interpreted if small.
- [ ] Analysis looks for patterns and divergences rather than forcing a single consensus structure.
- [ ] Cards that scatter widely are investigated as signals of ambiguity or labeling problems, not discarded.
- [ ] The resulting structure is validated through tree testing, not assumed correct from the sort alone.
- [ ] Findings are reported with their limitations, as input to design rather than a prescription.
