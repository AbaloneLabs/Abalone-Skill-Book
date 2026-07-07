---
name: ai_evaluation_and_benchmarking.md
description: Use when the agent is evaluating or benchmarking an LLM, agent, or generative system — building offline golden datasets and metrics, running LLM-as-judge rubrics, regression-testing across model or prompt upgrades, designing human evaluation, measuring open-ended generation quality, or deciding whether a change shipped safely. Also covers the failure modes of eyeballing a few examples, LLM-as-judge bias and self-preference, position effects in pairwise scoring, metrics that do not match the task, regression slipping through a model upgrade, and the recurring mistake of treating evaluation as a one-time check rather than a continuous guardrail that gates every release.
---

# AI Evaluation And Benchmarking

A generative system has no compiler and no passing test suite. Its behavior is statistical — correct on most inputs, wrong on a slice you did not check — so the only way to know whether a change is safe to ship is to measure it systematically against a representative set with defined metrics. The judgment problem is that "it looks good on the examples I tried" is not evaluation; it is overfitting to a handful of data points and declaring victory. Agents reach for a vibe check because building a real evaluation harness feels like overhead, and the harm is invisible until a model upgrade or a prompt change silently regresses the system on the long tail, in production, with no rollback signal because no one was measuring.

This skill is distinct from prompt-engineering-and-evaluation, which covers the craft of writing and tuning a single prompt. Here the focus is the evaluation and benchmarking *system*: the golden datasets, the metrics, the judge design, the regression pipeline, and the human-evaluation strategy that together tell you whether a model, agent, or generative feature is actually good and whether a change made it better or worse. Agents tend to miss that evaluation is a first-class engineering artifact with its own quality bar — a biased judge, a metric that does not match the task, or a golden set that misses edge cases each produce a number that looks authoritative and misleads every downstream decision.

The harm appears in characteristic ways. A golden set of easy, hand-picked examples reports 98% accuracy and hides a 40% regression on rare inputs. An LLM-as-judge prefers whichever answer it wrote (self-preference) or whichever came first (position bias), so the "objective" metric is just the judge's bias. A model upgrade is shipped because the aggregate score held, while a critical slice silently broke. An open-ended generation task is graded by a metric designed for classification, producing a number unrelated to quality. The judgment problem is to treat evaluation as a continuous, calibrated, slice-aware guardrail that gates every release — not a one-time check performed once and forgotten.

## Core Rules

### Build A Golden Dataset That Mirrors Real Input Distribution

The evaluation set is the ground truth your decisions rest on, and its composition determines what you can see. A set of easy head cases reports high scores and hides tail regressions:

- **Sample the real distribution, not the convenient one.** Include common cases, edge cases (empty, very long, ambiguous, multilingual, adversarial), and the long tail — the inputs where systems actually fail. A golden set drawn only from "what the developer tried" is biased toward what works.
- **Size it to the variance.** Three examples cannot distinguish a real improvement from noise; aim for tens to hundreds of graded items so a metric move is statistically meaningful, not coincidental.
- **Keep golden labels current and versioned.** Inputs drift, expected answers change, and labelers disagree. Version the dataset, record labeling decisions, and re-label when the task definition evolves. A stale golden set measures the wrong thing.
- **Grade on multiple axes, not one.** A single "is it correct" label collapses distinct failures (factually wrong, wrong format, partially right, harmful). Use graded judgments (relevant/irrelevant/irrelevant-but-plausible) and per-axis scores where the task has more than one quality dimension.

### Choose Metrics That Match The Task, Not Metrics That Are Easy To Compute

The metric must measure what the task actually rewards. A common failure is reaching for a familiar metric (exact match, BLEU, ROUGE) because it is easy, even when it does not correlate with quality:

- **Match the metric to the output type.** Classification: precision, recall, F1 against labels. Extraction/structured output: schema validity, field accuracy, exact-match on key fields. Retrieval-grounded answers: factual accuracy against references, citation correctness. Open-ended generation: task-specific rubrics, often LLM-as-judge or human grading, because lexical overlap does not capture quality.
- **Beware lexical metrics on semantic output.** BLEU/ROUGE reward word overlap; a correct paraphrase scores low and a fluent wrong answer can score high. Use them only where surface form is the task (translation, summarization fidelity), not for reasoning or open-ended answers.
- **Track pass/fail and quality together.** For structured output, track both "did it parse" (a hard floor) and "was it correct" (quality). A system that always parses but is often wrong looks fine on the first and broken on the second.
- **Define the metric before you optimize for it.** Once a number exists, behavior converges on it; a poorly chosen metric optimizes the wrong thing invisibly.

### Use LLM-as-Judge, But Calibrate Its Biases

LLM-as-judge is the practical way to grade open-ended output at scale, but a judge model is a stochastic, biased evaluator, not an oracle. Its biases, if uncorrected, become your metric:

- **Measure and mitigate self-preference.** Judges tend to prefer outputs from their own family. If the judge and the system-under-test are the same model, the score is inflated. Use a different (ideally stronger) model as judge, or cross-check a sample with a second judge.
- **Control position and order bias.** In pairwise comparison, judges prefer whichever answer is presented first (or last, depending on the model). Randomize or balance the order, and average over both orderings.
- **Use a rubric, not a vibes score.** A judge prompted to "rate this answer 1-10" produces an uncalibrated number. Give it explicit criteria (accuracy, completeness, adherence to format, safety) and ask for per-criterion scores plus a justification, which is more stable and auditable.
- **Calibrate against human labels.** Run a sample through both the judge and human graders and measure agreement; if agreement is low, the judge is not measuring what you think. Report judge-human agreement as part of the metric's credibility.
- **Watch for judge drift.** A judge model upgrade changes the metric silently; pin the judge version and re-calibrate when it changes, just as you would the system-under-test.

### Regression-Test Across Model And Prompt Upgrades

A model upgrade or prompt change is the highest-risk moment for a generative system, because behavior shifts statistically with no compile error. Regression testing is what catches it:

- **Run the full golden set on every change.** A change that improves the cases you were staring at and regresses fifty others is a regression; only the full set reveals it. Gate releases on the aggregate and the slices.
- **Inspect slices, not just the aggregate.** An aggregate score that holds can hide a sharp regression on a minority input class (a language, a length band, an input type). Slice by every dimension where behavior might differ and alarm on per-slice regressions even when the aggregate is flat.
- **Pin the (model, prompt, settings) tuple under test.** A regression test is meaningless if the model version floated between runs. Version the system-under-test the same way you version the golden set.
- **Diff before and after, not just scores.** For open-ended output, inspect actual output diffs on a sample; a held score can mask a style or safety shift that matters to users.

### Add Human Evaluation For High-Stakes And Open-Ended Tasks

Automated metrics and LLM-as-judge are proxies; for high-stakes, safety-critical, or genuinely open-ended tasks, human judgment is the ground truth that validates the proxies:

- **Use humans where the cost of error is high.** Medical, legal, financial, safety, and brand-sensitive outputs warrant human grading even when an automated metric exists.
- **Define the rubric and train graders.** Untrained graders applying vague criteria produce noisy labels. Write an explicit rubric, calibrate graders against a shared set, and measure inter-rater agreement; low agreement means the rubric (or the task) is ambiguous.
- **Sample, do not grade everything.** Human grading is expensive; grade a representative, stratified sample large enough to detect the regressions you care about, and use it to calibrate the automated judge rather than as the sole signal.

## Common Traps

### Eyeballing A Few Examples Instead Of Systematic Evaluation

Declaring a system good after reading three or five outputs, which overfits to the head and misses tail regressions. Build a golden set mirroring the real distribution and run it on every change.

### Golden Set Of Easy, Hand-Picked Cases

A dataset of only the easy examples the developer tried, which reports high scores and hides failures on rare, long, ambiguous, or adversarial inputs. Include edge cases and the long tail.

### LLM-as-Judge With Self-Preference Or Position Bias

Using the same model to judge its own outputs, or presenting answers in a fixed order, so the score reflects the judge's bias rather than quality. Use a different/stronger judge, randomize order, and calibrate against humans.

### Metric That Does Not Match The Task

Applying a lexical metric (BLEU/ROUGE, exact match) to semantic or reasoning output where surface overlap does not capture quality, producing a number unrelated to real performance. Match the metric to what the task rewards.

### Shipping A Model Upgrade On The Aggregate Alone

Approving a model or prompt change because the overall score held, while a critical slice silently regressed. Inspect per-slice metrics and output diffs, and gate on slices.

### Unpinned Judge Model That Drifts The Metric

Letting the judge model float to "latest," so a judge upgrade changes the metric without anyone noticing, making scores non-comparable across time. Pin the judge version and re-calibrate on change.

### Single Pass/Fail Label Collapsing Distinct Failures

Grading only "correct or not," so factual errors, format violations, partial answers, and safety issues are indistinguishable. Grade on multiple axes with a rubric.

### One-Time Eval Never Re-Run On Changes

Running evaluation once at launch and never again, so subsequent model or prompt changes ship unmeasured. Make evaluation a continuous release gate.

## Self-Check

- [ ] A golden dataset exists that mirrors the real input distribution (head, tail, edge, adversarial, multilingual where relevant), is sized to make metric moves meaningful, and is versioned with current, multi-axis graded labels.
- [ ] Metrics match the output type (classification F1, structured-output schema/field accuracy, retrieval factual/citation accuracy, open-ended rubric grading), and lexical metrics are used only where surface form is the task.
- [ ] LLM-as-judge uses a different or stronger model than the system-under-test, order is randomized/balanced to control position bias, scoring is rubric-based with justifications, and judge-human agreement is measured and reported.
- [ ] The judge model version is pinned and re-calibrated on change, so scores are comparable across time.
- [ ] The full golden set is run on every model or prompt change, with per-slice inspection (not just the aggregate), and releases are gated on slice-level regressions.
- [ ] The (model, prompt, settings) tuple under test is pinned and versioned alongside the golden set, and before/after output diffs are inspected on a sample for open-ended tasks.
- [ ] High-stakes or open-ended tasks include human evaluation with an explicit rubric, calibrated graders, measured inter-rater agreement, and stratified sampling.
- [ ] Pass/fail and quality are tracked separately for structured output (parse rate vs correctness), and distinct failure modes are not collapsed into one label.
- [ ] The highest-risk cases were verified — eyeballed examples, easy-only golden sets, self-preferencing or position-biased judges, mismatched metrics, aggregate-only upgrade approval, drifting judge versions, collapsed labels, and one-time evals — not only the few outputs that looked good.
