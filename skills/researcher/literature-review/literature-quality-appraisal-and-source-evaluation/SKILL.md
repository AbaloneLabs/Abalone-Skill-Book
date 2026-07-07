---
name: literature_quality_appraisal_and_source_evaluation.md
description: Use when the agent is judging the credibility of individual studies or sources for a review, selecting a risk-of-bias or critical appraisal tool such as CASP ROB or GRADE for a single study, distinguishing predatory or low-quality venues from legitimate ones, deciding whether to include preprints or grey literature, weighting evidence by quality in a synthesis, or separating peer review from actual credibility.
---

# Literature Quality Appraisal And Source Evaluation

Not every source that looks like a study is one worth building on, and not every credible study is credible in the same way. A literature review is only as trustworthy as the appraisal behind it: if every included source is treated as equal evidence, the synthesis inherits the weaknesses of its weakest study and presents the blend as authoritative. The recurring failure is to conflate several distinct judgments, peer-reviewed versus not, rigorous versus sloppy, relevant versus merely available, and to collapse them into a single yes-or-no on inclusion. The judgment problem is to appraise each source on the dimensions that matter for the question, to recognize that venue and peer review are weak proxies for study quality, and to let the appraisal actually shape how much weight each source carries.

Use this skill when appraising individual studies or sources for a review, choosing an appraisal tool, judging venue credibility, deciding how to handle preprints and grey literature, or weighting evidence by quality. The goal is to keep the agent from treating inclusion as the end of appraisal and from equating peer review with trustworthiness. The agent has latitude in tool choice and weighting, but the appraisal must be explicit, applied consistently, and allowed to affect the synthesis.

## Core Rules

### Appraise Against The Question, Not Against A Universal Standard

Quality is not absolute. A small qualitative study may be high-quality evidence for a "how" question and irrelevant for a "how common" question. Appraisal must be tied to what the source is being used to claim.

For each source, clarify:

- the specific claim or sub-question this source will support;
- the design the source used and whether that design can support that claim type;
- whether the source's population, context, and outcomes match the review's target;
- whether the source's rigor is adequate for the role it will play in the synthesis.

A source appraised as weak for one question may be adequate for another. Tie the appraisal to the inferential role, not to a generic quality score.

### Choose An Appraisal Tool Matched To Design And Question

Different designs fail in different ways, and appraisal tools are built to detect design-specific threats. Using one tool for everything, or none, misses the biases that matter.

Match tools to designs:

- randomized trials benefit from tools that assess randomization, allocation concealment, blinding, and incomplete outcome data, such as Cochrane RoB 2;
- non-randomized quantitative studies benefit from tools that assess confounding, selection, and measurement, such as ROBINS-I;
- observational and diagnostic studies have dedicated appraisal frameworks;
- qualitative studies benefit from tools that assess credibility, dependability, and confirmability rather than positivist validity;
- checklists such as CASP are structured around design-specific questions.

The tool should surface the threats the design is prone to, and the choice of tool should be justified, not defaulted.

### Separate Peer Review From Credibility

Peer review is a filter, not a guarantee. It catches some problems and misses many, and its rigor varies enormously across journals, reviewers, and fields. Treating "peer-reviewed" as synonymous with "credible" is one of the most common appraisal errors.

Recognize that:

- peer review varies in depth, from detailed critique to cursory checks;
- journal rank and impact factor are weak proxies for the quality of an individual study;
- a study can be methodologically flawed and still pass peer review;
- a preprint can be rigorous and still lack peer review;
- retracted or corrected articles may still appear citable.

Judge the study on its design, conduct, and reporting, and treat venue as one weak signal among many, not as the decisive one.

### Detect Predatory And Low-Quality Venues

Predatory and low-quality publishers mimic legitimate venues and can introduce fabricated, plagiarized, or unreviewed material into a review. They are a real threat to synthesis credibility, but the detection requires judgment, because no single marker is conclusive.

Check for:

- verification of peer review claims, not just their presence on the website;
- editorial board legitimacy, with real, identifiable, active members;
- indexing in recognized databases appropriate to the field;
- unusually fast acceptance, aggressive solicitation, or APC structures that signal pay-to-publish;
- transparency about ownership, location, and review process;
- a history of retractions or editorial concerns.

Avoid simplistic heuristics like "no APC means legitimate" or "new journal means predatory." Evaluate the venue's actual practices and the article's actual content.

### Handle Preprints And Grey Literature Deliberately

Preprints, reports, theses, and grey literature can be valuable, timely, and sometimes the only available source. Excluding them can introduce publication bias; including them without appraisal can introduce unvetted material. The decision should be reasoned, not reflexive.

For preprints and grey literature:

- state whether they are eligible and why, tied to the review's purpose;
- appraise them on the same design-specific criteria as peer-reviewed work;
- check whether a preprint has since been peer-reviewed, revised, or retracted;
- track versions so that a later published version supersedes an earlier preprint;
- note that absence of peer review is a flag to examine, not grounds for automatic exclusion.

In fast-moving fields, a preprint may be the most current evidence; in others, relying on it may be premature. The appraisal, not the format, should drive the weight.

### Let Appraisal Actually Weight The Synthesis

Appraisal that does not affect the synthesis is theater. If every source is appraised but all are then treated as equal, the appraisal has added effort without value. The findings must shape how much each source contributes.

Weight by:

- giving more influence to sources with lower risk of bias and better fit to the question;
- down-weighting or excluding sources with fatal flaws for the claim they support;
- flagging where the conclusion rests on weak studies rather than strong ones;
- performing sensitivity analyses that test whether the conclusion holds when weak studies are removed;
- narrating the direction of bias, since a body of flawed studies can be systematically misleading rather than merely noisy.

A conclusion that survives removing the weakest studies is far more credible than one that depends on them.

### Appraise Consistently And Document The Process

Appraisal is only defensible if it is applied consistently across sources and documented well enough to be reproduced. Selective or undocumented appraisal is itself a source of bias.

Ensure:

- the same tool and criteria are applied to all eligible sources of a given design;
- judgments are recorded with the reasoning, not just a score;
- where dual appraisal is feasible, disagreements are discussed and resolved;
- the appraisal process and any exclusions based on quality are reported transparently;
- limitations of the appraisal, such as poor reporting in source studies, are acknowledged.

A reader should be able to see not only which sources were appraised as strong or weak, but why, and how that affected the synthesis.

## Common Traps

### Equating Peer Review With Credibility

Peer review is a variable and imperfect filter. A peer-reviewed study can be flawed, and an unreviewed preprint can be sound; the design and conduct decide.

### Using One Generic Checklist For All Designs

A single tool applied across randomized, observational, and qualitative studies misses the design-specific threats that determine whether each can be trusted for its claim.

### Appraising But Not Weighting

Recording risk of bias and then treating every source as equal evidence makes the appraisal decorative. The appraisal must shape the synthesis.

### Defaulting To Journal Rank As A Quality Proxy

Impact factor and rank reflect citation dynamics and field practices, not the rigor of an individual study. They are weak proxies and often misleading.

### Reflexively Excluding Or Including Preprints

Excluding all preprints can bias a review toward older published work; including them uncritically can import unvetted claims. The decision should be reasoned and appraised.

### Simplistic Predatory-Journal Heuristics

No single marker proves a venue is predatory or legitimate. Real evaluation examines peer-review practice, editorial legitimacy, indexing, and the article itself.

### Ignoring The Direction Of Bias

A body of studies sharing the same flaw is systematically biased, not merely imprecise. The synthesis should report the likely direction of that bias, not just average across studies.

### Undocumented Or Selective Appraisal

If the appraisal criteria, judgments, and their effect on the synthesis are not recorded, the process cannot be reproduced or challenged, and may hide selective treatment of sources.

## Self-Check

- Is each source appraised against the specific claim or sub-question it is meant to support, rather than against a universal standard?
- Is the appraisal tool matched to the source's design and the review's question, with the choice justified?
- Is peer review treated as one weak signal rather than as proof of credibility?
- Are predatory and low-quality venues evaluated on their actual practices, not on simplistic heuristics?
- Are preprints and grey literature handled deliberately, appraised on the same criteria, and version-tracked?
- Does the appraisal actually weight the synthesis, with stronger sources carrying more influence and weak sources down-weighted or excluded?
- Has the direction of bias across the body of evidence been considered, not just averaged away?
- Is the appraisal applied consistently across sources of each design, with judgments and reasoning documented?
- Are sensitivity analyses used to test whether the conclusion depends on the weakest studies?
- Are the appraisal process, exclusions, and its limitations reported transparently enough to be reproduced?
