---
name: quality_metric_application.md
description: Use when the agent is selecting or applying machine translation quality metrics such as BLEU, chrF, COMET, TER, BERTScore, or human direct assessment, interpreting metric scores, comparing engines or runs by metric, setting metric thresholds for acceptance, or deciding which metric fits a given evaluation goal such as ranking engines, tracking regressions, or estimating post-editing effort.
---

# Quality Metric Application

MT quality metrics are tools for turning subjective judgment into comparable numbers, and misusing them produces confident wrong decisions. Each metric measures something specific and ignores the rest. N-gram overlap metrics such as BLEU and chrF reward lexical match against a reference and punish legitimate paraphrase; they cannot detect meaning errors in fluent text and they reward a system that copies the reference style. Embedding-based metrics such as COMET and BERTScore capture semantic similarity and tolerate paraphrase, but they can pass semantically wrong text that happens to sit near the reference in embedding space. Edit-distance metrics such as TER measure post-editing effort against a reference, not quality in the abstract. Human direct assessment captures perceived quality but is noisy, expensive, and sensitive to rater bias and anchoring. The central judgment problem is that no single metric is quality; each is a proxy for one facet of quality, and the facet it captures may not be the facet your decision needs. Agents miss this because a number feels authoritative, and they reach for whichever metric is easy to compute, then draw conclusions the metric cannot support.

The harm this skill prevents is decisions built on the wrong proxy: ranking an engine higher because it matches the reference's surface wording, declaring no regression because a semantic metric held steady while accuracy eroded, or setting an acceptance threshold that passes fluent wrong text. The agent's freedom is to choose and combine metrics, but every choice must be justified against the decision the metric is meant to inform.

## Core Rules

### Match The Metric To The Decision Being Made

Start from the decision, not the metric. Different decisions need different evidence. Ranking candidate engines to pick one for deployment benefits from a combination of an automated semantic metric for breadth and human evaluation for the decisive sample. Tracking regression across retraining cycles benefits from a stable, cheap automated metric applied consistently, with human spot-checks on drift. Estimating post-editing effort benefits from TER or edit distance against post-edited references. Estimating fitness for publication requires human review against source, because no automated metric certifies meaning accuracy. Setting an acceptance gate for raw MT use needs a threshold tied to downstream consequence, not a generic cutoff. State the decision in one sentence, then choose the metric that measures the facet the decision hinges on. A metric chosen because it is convenient, rather than because it fits, produces a defensible-looking but wrong decision.

### Understand What Each Metric Actually Measures And Ignores

You cannot interpret a metric you do not understand. BLEU compares n-gram overlap between candidate and one or more references, rewarding exact lexical match; it is corpus-level, brittle on short segments, and blind to meaning. chrF compares character n-grams and is more robust to morphology and to languages with rich inflection, but it is still an overlap metric. COMET uses multilingual embeddings to score semantic similarity to a reference and source, tolerating paraphrase and catching some meaning errors overlap metrics miss. BERTScore uses embeddings for similarity and is reference-based. TER counts the edits needed to turn the candidate into the reference and proxies post-editing effort. Direct assessment asks humans to rate adequacy or quality on a scale. Know, for every metric you apply, what it rewards, what it punishes, and what it is blind to, and never report a score without that context.

### Never Treat An Automated Metric As Proof Of Accuracy

The most dangerous misuse is treating a high automated score as evidence that the translation is correct. Overlap metrics pass fluent text that copies reference wording and cannot detect hallucinations, omissions, or meaning shifts that happen to use similar words. Embedding metrics can pass semantically wrong text near the reference in vector space. No automated metric in routine use verifies that the target conveys the source meaning completely and correctly. For any decision where accuracy is the stakes, such as publishing, legal, medical, or safety content, automated metrics are at best a triage layer; the decision requires human comparison of target against source. State explicitly, in any report, that automated scores do not certify accuracy.

### Use Multiple Metrics That Capture Different Facets

Single-metric evaluation is fragile because it collapses quality to one dimension. Combine metrics that measure different things: an overlap metric such as chrF for surface match, a semantic metric such as COMET for meaning proximity, and TER or edit distance for effort, alongside human assessment for the facets no automated metric reaches. Divergence between metrics is itself a signal: if overlap drops while a semantic metric holds, the system may be paraphrasing more; if a semantic metric holds while human accuracy scores drop, the system may be producing fluent wrong text that sits near the reference. Report metrics as a panel, not a headline, and explain agreement and disagreement among them.

### Account For Reference Quality And Number Of References

Overlap and reference-based semantic metrics depend entirely on the reference. A single reference punishes legitimate alternative translations; multiple references reduce this bias but raise annotation cost. A flawed reference, itself a mistranslation or a non-idiomatic rendering, corrupts every score computed against it. Before trusting metric scores, assess the reference set: is it human, professional, idiomatic, and representative of acceptable variation. For low-resource pairs or specialized domains where good references are scarce, metric scores are correspondingly less trustworthy. Document the reference provenance alongside the scores.

### Set Thresholds Against Baselines, Not Abstract Numbers

A BLEU of 40 means nothing in isolation; it means something only relative to a baseline on the same test set with the same references and the same preprocessing. Set acceptance and regression thresholds relative to a measured baseline, and re-baseline when the test set, references, or metric version change. Beware metric version drift: a COMET score from one model checkpoint is not comparable to one from another. When comparing engines or runs, hold the test set, references, metric version, and preprocessing fixed, and report all of them. Comparisons across different conditions are meaningless and a frequent source of false conclusions.

### Sample And Aggregate Honestly

Metric scores are statistics over samples, and the sample determines what the score means. A score computed on a small or unrepresentative sample does not generalize to the engine's behavior on real content. Stratify test sets to cover the domains, lengths, and content types the engine will face, and report scores by stratum, not only as a corpus aggregate, because a strong aggregate can hide catastrophic performance on a stratum that matters. Report confidence intervals where the metric supports them, and avoid presenting noisy single scores as precise. Honest aggregation protects against over-claiming from small samples.

### Correlate Metrics With Human Judgment Before Trusting Them

Before relying on an automated metric for a decision, confirm it correlates with human judgment in your setting, language pair, and domain. Metrics that correlate well with human judgment on general news may correlate poorly on specialized or creative content. Run a small human evaluation alongside the metric, compute correlation, and treat the metric's authority as bounded by that correlation. A metric with poor human correlation in your setting is a random number for your decision, however respectable it is in the literature.

## Common Traps

### Treating A High Automated Score As Proof Of Accuracy

Overlap and embedding metrics pass fluent and semantically near text that is meaningfully wrong. Automated scores triage; they do not certify accuracy.

### Choosing A Metric Because It Is Easy To Compute

Convenience-driven metric choice produces decisions built on the wrong proxy. Start from the decision, then choose the metric.

### Reporting A Single Headline Number

Single-metric headlines collapse quality to one dimension and hide what the metric is blind to. Report a panel and explain divergence.

### Comparing Scores Across Different Conditions

Different test sets, references, metric versions, or preprocessing make scores incomparable. Hold conditions fixed or do not compare.

### Trusting Scores Against A Flawed Or Single Reference

Reference-based metrics inherit reference flaws and punish legitimate variation. Assess reference quality and use multiple references where feasible.

### Aggregating Over An Unrepresentative Sample

A strong corpus score can hide catastrophic stratum-level performance. Stratify and report by stratum for the domains that matter.

### Ignoring Metric-Human Correlation In Your Setting

A respectable metric with poor human correlation in your pair and domain is noise for your decision. Validate before relying.

### Setting Absolute Thresholds Without A Baseline

Abstract thresholds such as "BLEU above 40" are meaningless without a baseline on the same conditions. Threshold against measured baselines.

## Self-Check

- Was the metric chosen by starting from the decision to be made, and does it measure the facet that decision hinges on?
- For each metric applied, do you know what it rewards, what it punishes, and what it is blind to, and is that context stated in the report?
- Is it explicitly acknowledged that automated metrics do not certify meaning accuracy, with human source comparison required for accuracy-critical decisions?
- Are multiple metrics capturing different facets reported as a panel, with agreement and divergence explained rather than a single headline number?
- Is the reference set assessed for quality, provenance, and representativeness, with multiple references used where legitimate variation matters?
- Are thresholds set against measured baselines on identical conditions, with metric version, test set, references, and preprocessing held fixed for any comparison?
- Is the test set stratified across domains, lengths, and content types, with scores reported by stratum and not only as a corpus aggregate?
- Has the metric's correlation with human judgment been checked in your language pair and domain, and is the metric's authority bounded by that correlation?
- No accuracy-critical decision was made on automated scores alone, and no comparison was drawn across differing conditions.
- The reported scores are accompanied by their conditions, reference provenance, sampling method, and confidence intervals where available.
