---
name: secondary_data_and_dataset_use.md
description: Use when the agent is selecting or evaluating an existing secondary dataset such as administrative records, public-use files, survey archives, or repository data, judging whether it fits the research question, understanding the original collection context and its limits, assessing documentation quality, navigating access and licensing restrictions, or harmonizing variables across multiple datasets.
---

# Secondary Data And Dataset Use

Secondary data offers findings without the cost and delay of original collection, and that is exactly why it is misused. When a dataset already exists, the temptation is to start with the data and hunt for a question it can answer, rather than starting with a question and judging whether the data can answer it. The deeper problem is that secondary data were collected by someone else, for some other purpose, under conditions the current researcher did not control. Every variable, code, missing value, and inclusion rule in that dataset carries the imprint of the original study's design, population, timing, and intent. A researcher who treats a secondary file as a neutral table of facts, rather than as an artifact produced under specific and possibly incompatible conditions, will draw conclusions that reflect the data's provenance more than the phenomenon under study.

Use this skill when choosing, evaluating, linking, or analyzing an existing dataset for a research question. The goal is to keep the agent from forcing a question onto unsuitable data, from ignoring the original collection context and its constraints, from trusting undocumented variables, from violating access terms, and from harmonizing incompatible measures as if they were identical. The agent has latitude in source selection, but the fit between dataset and question, and the limits imposed by the data's origin, must be argued explicitly.

## Core Rules

### Start With The Question And Judge The Dataset Against It

The most common secondary-data failure is data-driven questioning: scanning a dataset for interesting variables and reverse-engineering a research question around them. This produces studies that the data can technically support but that answer questions no one prioritized, while the real question goes unaddressed.

Anchor the work:

- define the research question, population, time frame, and constructs first;
- list the variables, granularity, coverage, and quality the answer would require;
- then evaluate candidate datasets against that requirement, gap by gap;
- reject datasets that cannot measure the construct, population, or period the question demands;
- document the specific deficits that make a tempting dataset unsuitable.

A dataset's availability is not a reason to ask the question it can answer. The standard is whether the data can support the intended inference, not whether some inference can be squeezed from the file.

### Reconstruct The Original Collection Context And Its Limits

A secondary dataset is the residue of a primary process. Its variables, coverage, and quality are bounded by how, why, when, and for whom the data were originally gathered. Ignoring that context turns the file into a source of false confidence.

Reconstruct:

- the original purpose, sponsor, and design of collection;
- the target population, sampling frame, and inclusion rules of the source;
- the timing, frequency, and historical conditions during collection;
- the instruments, coding rules, and definitions used by the original collectors;
- known limitations, edits, suppressions, and quality notes published with the data.

Administrative data, for example, reflect the bureaucracy that produced them: who was eligible, what was recorded, what was enforced, and what was ignored. A variable that looks like income or diagnosis may measure eligibility or billing category instead. The collection context explains these gaps and must be understood before analysis.

### Evaluate Documentation Quality Before Trusting The Data

Documentation is the only bridge between the researcher and the original measurement. Weak or missing documentation makes every variable an assumption.

Inspect:

- a codebook or data dictionary defining each variable, its values, and its units;
- the universe and skip patterns determining who was asked or recorded for each variable;
- missingness and edit flags and what they mean, not just blank cells;
- recodes, imputations, and derived variables already applied by the source;
- version history, so the file analyzed is the one documented.

Where documentation is thin, the researcher must either reconstruct meaning from the source study, treat variables as uncertain, or decline to use them. A variable whose definition cannot be verified cannot support a strong claim, however clean its values look.

### Assess Fitness For The Specific Construct And Population

A dataset may cover the right topic and still be unfit, because the construct it measures differs from the one the research requires, or because its population differs from the target.

Assess fitness:

- whether each key variable measures the intended construct or a proxy shaped by the source's purpose;
- whether the population, geography, and time period match the inference target;
- whether the granularity supports the intended unit of analysis or forces aggregation that loses information;
- whether the sample is representative of the target or systematically excludes relevant groups;
- whether changes in definitions, boundaries, or methods over time break comparability.

Fitness is a construct-by-construct, population-by-population judgment, not a global property of the dataset. A file can be fit for one question and unfit for another using the same variables.

### Respect Access, Licensing, And Confidentiality Restrictions

Secondary data carry terms. Public-use files, restricted-use files, licensed data, and data enclaves impose different obligations, and violating them can terminate access and breach participant confidentiality.

Clarify:

- the access tier: public-use, licensed, restricted, or enclave-only;
- the permitted analyses, publications, and derivative uses;
- re-identification and disclosure risks, especially for small cells and linked data;
- data residency, deletion, and sharing obligations;
- whether institutional review or a data use agreement is required before access.

Aggregating or linking datasets can re-identify individuals even when each source is anonymized. Disclosure review is part of using secondary data responsibly, not an optional add-on.

### Decide Whether Linking And Harmonization Are Defensible

Combining datasets can answer questions no single source can, but linking and harmonization introduce their own errors and assumptions that are easy to underestimate.

When linking or harmonizing:

- confirm that the linkage keys are valid and that match rates are high enough to avoid linkage bias;
- analyze who did and did not link, since unmatched records may differ systematically;
- check whether variables measure the same construct across sources or merely share a name;
- document recodes, category mappings, and unit conversions applied to align variables;
- test whether harmonized relationships hold within each source before pooling.

Harmonizing two income variables with different reference periods, definitions, or top-coding is not creating a comparable measure; it is creating a composite whose meaning is uncertain. Harmonization decisions must be explicit and sensitivity-tested.

### Carry The Source's Limitations Into The Analysis And Report

Secondary analysis inherits every limitation of the source and adds its own. These limitations must shape the analysis and be reported, not buried.

Carry forward:

- the sampling and coverage limits of the original collection;
- measurement differences between the source's constructs and the study's constructs;
- missingness, imputation, and editing already applied upstream;
- the temporal and geographic scope the data actually represent;
- the gap between the question asked and the question the data can answer.

A secondary analysis that reports only its own methods, as if the data were pristine and self-evident, hides the chain of assumptions on which its conclusions rest.

## Common Traps

### Data-Driven Questioning

Starting from an available dataset and manufacturing a question it can answer inverts the research logic and produces convenient but unimportant findings.

### Ignoring Original Collection Purpose

Treating administrative or survey data as neutral facts hides how the source's purpose shaped who and what was recorded.

### Trusting Undocumented Variables

Analyzing variables whose definitions, universes, or edits are unknown builds claims on unverifiable assumptions.

### Proxy Treated As Construct

Assuming a source variable measures the research construct because it shares a label, without checking validity, misrepresents the finding.

### Harmonizing Incompatible Measures

Pooling variables with different definitions, periods, or coding as if identical creates a composite of uncertain meaning.

### Overlooking Linkage Bias

Ignoring who failed to match when linking datasets hides systematic differences between linked and unlinked records.

### Violating Access Terms

Using restricted or licensed data outside its permitted scope breaches confidentiality and agreements and can invalidate publication.

### Silent Inherited Limitations

Reporting only the secondary analysis while omitting the source's sampling, measurement, and coverage limits overstates what the data can support.

## Self-Check

- Does the research question precede dataset selection, with candidate data evaluated against required variables, population, period, and quality?
- Is the original collection context, including purpose, design, population, timing, and instruments, reconstructed and understood?
- Is documentation quality assessed, with variables whose definitions cannot be verified treated as uncertain or excluded?
- Is each key variable judged for construct, population, granularity, and temporal fitness against the specific inference?
- Are access tier, licensing, permitted uses, re-identification risk, and any required data use agreements or disclosure review confirmed?
- If datasets are linked, are match rates, linkage bias, and the meaning of linkage keys analyzed?
- If variables are harmonized, are category mappings, unit conversions, and cross-source comparability documented and sensitivity-tested?
- Are the source's sampling, coverage, measurement, missingness, and editing limitations carried into the analysis and reported?
- Is the gap between the question asked and the question the data can answer stated explicitly?
- Are conclusions bounded to the population, time, and constructs the secondary data actually represent?
