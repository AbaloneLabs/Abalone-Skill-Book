---
name: multiple_testing_and_error_control.md
description: Use when the agent is analyzing data with many tests or comparisons, controlling the family-wise error rate or false discovery rate, addressing the multiple comparisons problem, deciding on correction methods such as Bonferroni or Benjamini-Hochberg, interpreting results after correction, or diagnosing false positive inflation due to uncontrolled multiple testing.
---

# Multiple Testing And Error Control

Every statistical test carries a chance of a false positive, a significant result that arises from chance rather than a real effect. At the conventional five-percent threshold, one in twenty tests on true nulls will appear significant by accident. When a study conducts many tests, comparing multiple outcomes, multiple groups, multiple time points, or exploring many variables, the chance that at least one false positive appears rises rapidly, and without correction, the analysis will report "significant" findings that are pure noise. This is the multiple comparisons problem, and it is one of the most common sources of false findings in research. Analyses with dozens or hundreds of comparisons, common in genomics, neuroimaging, and exploratory data analysis, can generate numerous false positives if error control is not applied. The judgment problem is recognizing when multiple testing is occurring, choosing appropriate error control methods, and interpreting corrected results honestly, rather than conducting many tests and reporting the significant ones as if each were independently tested.

Use this skill when analyzing data with multiple tests, choosing correction methods, addressing the multiple comparisons problem, or diagnosing false positive inflation. The goal is to prevent the agent from conducting many tests without correction, from choosing correction methods that do not fit the analysis, from reporting uncorrected significant results as findings, and from misunderstanding what corrected p-values mean.

## Core Rules

### Recognize When Multiple Testing Is Occurring

The first discipline is recognizing that multiple testing is happening, because it is often invisible. A study with one primary outcome and one test is not a multiple testing problem; a study with several outcomes, subgroup analyses, or repeated measures is. Exploratory analyses with many variables are heavily multiple-testing-laden.

Recognize by:

- counting the number of tests conducted, including all outcomes, groups, time points, and subgroups;
- recognizing that testing many predictor-outcome pairs is multiple testing even if framed as one analysis;
- including tests conducted but not reported, which still inflate error rates;
- distinguishing the pre-specified primary analysis from exploratory and post hoc analyses;
- being honest about the total number of comparisons, not just the number reported.

An analysis that reports three significant results out of fifty tests conducted is reporting noise unless error control accounts for the fifty.

### Distinguish Family-Wise Error Rate From False Discovery Rate

Two main approaches to error control exist, and they answer different questions. Family-wise error rate, FWER, control limits the probability of any false positive among all tests, and is appropriate when making a single strong claim that must not include any error. False discovery rate, FDR, control limits the expected proportion of false positives among the significant results, and is appropriate when making many claims and tolerating some false positives among them.

Distinguish by:

- using FWER control, such as Bonferroni, when any false positive would undermine the conclusions;
- using FDR control, such as Benjamini-Hochberg, when many findings are expected and some error is tolerable;
- recognizing that FWER is more stringent, reducing power, while FDR is less stringent, allowing more findings;
- matching the error rate to the analytic goal, not applying one universally.

Bonferroni correction on a genomics study with ten thousand tests would find almost nothing; FDR is appropriate there. FWER is appropriate for a clinical trial's primary comparisons.

### Choose Correction Methods Appropriate To The Analysis

Correction methods differ in their assumptions, stringency, and power. Bonferroni is simple and conservative, controlling FWER under any dependence structure but losing power. Holm and Hochberg are less conservative FWER methods. Benjamini-Hochberg controls FDR under independence or positive dependence. Permutation-based methods handle complex dependence. Choose based on the analysis structure.

Choose by:

- using Bonferroni or Holm for FWER control where simplicity and robustness matter;
- using Benjamini-Hochberg for FDR control where many findings are expected;
- using permutation-based methods where test statistics are correlated or the structure is complex;
- accounting for the dependence structure among tests, which simple corrections may not;
- documenting the method choice and its rationale.

Applying Bonferroni where FDR is appropriate sacrifices power and misses real effects; applying FDR where FWER is needed allows false positives into strong claims.

### Pre-Specify The Primary Analysis To Limit Multiplicity

The cleanest way to handle multiplicity is to limit it through pre-specification. A study with a single pre-specified primary outcome and analysis has no multiplicity problem in its primary inference. Multiplicity arises when many outcomes, analyses, and subgroups are examined, often post hoc. Pre-specification contains the problem.

Pre-specify by:

- designating a single primary outcome and analysis before data examination;
- limiting secondary outcomes and specifying how they will be corrected;
- pre-registering the analysis plan to make pre-specification verifiable;
- treating exploratory and post hoc analyses as hypothesis-generating, not confirmatory;
- recognizing that analyses conducted after seeing the data require extra caution.

A study with a pre-specified primary analysis can make a clean claim; a study with fifty post hoc analyses cannot, regardless of correction.

### Interpret Corrected Results With Attention To Power

Error control reduces false positives but also reduces power, the ability to detect real effects. A study with many tests and stringent correction may fail to detect real effects because the correction has made the threshold too high. Interpreting corrected results requires attention to whether non-significant findings reflect no effect or insufficient power after correction.

Interpret with attention to power by:

- recognizing that correction raises the bar for significance, reducing power;
- distinguishing non-significant corrected results from evidence of no effect;
- examining effect sizes and confidence intervals, not just significance after correction;
- considering whether the analysis was powered for the corrected threshold;
- avoiding the interpretation that corrected non-significance proves no effect.

A study that finds nothing significant after Bonferroni correction of one hundred tests may have real effects it was underpowered to detect after correction, not absence of effects.

### Address Multiplicity In Subgroup And Sensitivity Analyses

Subgroup analyses, examining whether effects differ across groups, are a common source of hidden multiplicity. A study that tests the effect in ten subgroups has conducted ten tests, and the significant subgroup finding may be noise. Sensitivity analyses, conducted to test robustness, add further tests. Address multiplicity in these analyses explicitly.

Address by:

- limiting subgroups to those pre-specified and motivated by prior evidence;
- treating subgroup findings as exploratory unless pre-specified and corrected;
- recognizing that significant subgroup effects often fail to replicate;
- using interaction tests for subgroup differences, not separate within-subgroup tests;
- being honest that many subgroup analyses constitute multiple testing.

A significant subgroup finding from twenty tested is likely noise, and presenting it as a confirmed finding is misleading.

### Report All Tests Conducted, Not Just Significant Ones

Selective reporting of significant tests, among many conducted, produces a distorted picture indistinguishable from conducting few tests. The error rate applies to tests conducted, not tests reported, so reporting only significant results hides the multiplicity. Honest reporting includes all tests.

Report honestly by:

- reporting the number of tests conducted, including non-significant ones;
- presenting a table or plot of all results, not just selected significant ones;
- distinguishing pre-specified from exploratory analyses in reporting;
- avoiding the practice of running many tests and reporting the significant few as if pre-planned;
- recognizing that selective reporting is a form of publication bias within the analysis.

An analysis that ran fifty tests and reports three significant ones has reported three of fifty, and the error rate must account for the fifty.

### Separate Confirmatory From Exploratory Analysis

Confirmatory analyses test pre-specified hypotheses and can support claims; exploratory analyses generate hypotheses and cannot. Mixing them, conducting exploratory analyses and presenting significant results as confirmatory, inflates false positives while claiming rigor. Separate them clearly.

Separate by:

- designating pre-specified analyses as confirmatory and reporting them with full error control;
- designating post hoc and data-driven analyses as exploratory and framing them as hypothesis-generating;
- not promoting exploratory findings to confirmatory status because they are significant;
- requiring replication of exploratory findings before they support claims;
- being honest in reporting about which analyses were pre-specified and which were exploratory.

An exploratory finding presented as confirmatory is a hypothesis dressed as a conclusion, and it will often fail to replicate.

## Common Traps

### Invisible Multiple Testing

Many tests conducted without recognizing multiplicity. Count all tests, including unreported ones.

### Wrong Error Rate For The Goal

FWER where FDR is appropriate, or vice versa. Match the error rate to the analytic goal.

### Correction Method Mismatched To Structure

Ignoring dependence or complexity. Choose methods that fit the analysis structure.

### Uncontrolled Post Hoc Analyses

Many post hoc analyses without pre-specification or correction. Pre-specify and separate exploratory work.

### Interpreting Corrected Non-Significance As No Effect

Correction reduces power. Non-significance after correction may reflect power, not absence of effect.

### Hidden Multiplicity In Subgroup Analyses

Many subgroup tests treated as single tests. Limit subgroups and treat findings as exploratory.

### Selective Reporting Of Significant Tests

Reporting significant tests among many conducted. Report all tests and the total number conducted.

### Exploratory Findings Presented As Confirmatory

Post hoc significant results presented as pre-specified. Separate and label analyses honestly.

## Self-Check

- [ ] All tests conducted are counted, including unreported ones, and multiplicity is recognized.
- [ ] The error rate, FWER or FDR, is matched to the analytic goal, not applied universally.
- [ ] The correction method fits the analysis structure, including dependence among tests.
- [ ] A primary analysis is pre-specified to limit multiplicity in confirmatory inference.
- [ ] Corrected results are interpreted with attention to power, distinguishing non-significance from no effect.
- [ ] Subgroup and sensitivity analyses are limited, pre-specified, and treated as exploratory unless corrected.
- [ ] All tests conducted are reported, including non-significant ones, with the total count stated.
- [ ] Confirmatory and exploratory analyses are separated and labeled clearly in reporting.
- [ ] No exploratory finding is presented as confirmatory without replication.
- [ ] Effect sizes and confidence intervals are examined alongside significance after correction.