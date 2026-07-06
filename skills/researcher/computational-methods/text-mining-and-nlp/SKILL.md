---
name: text-mining-and-nlp.md
description: Use when the agent is applying text mining or natural language processing to research text, choosing between dictionary, topic modeling, and embedding methods, validating computational text findings against human reading, or guarding against treating algorithmic output as ground truth without validation against the substantive meaning of the corpus.
---

# Text Mining And NLP

Text mining and natural language processing (NLP) let researchers analyze text at a scale no close reading could match. This power is also the source of its main danger. Agents often run a corpus through topic modeling, sentiment analysis, or embeddings, take the algorithmic output as findings, and report patterns whose substantive meaning was never checked. But an algorithmic topic is a statistical cluster of co-occurring words, not necessarily a meaningful theme; a sentiment score is a model's prediction, not a measured attitude; an embedding encodes distributional patterns that may reflect corpus bias as much as semantic content. The judgment problem is to choose a method whose assumptions fit the text and the question, to validate computational output against human reading of the actual documents, and to treat algorithmic results as hypotheses requiring substantive interpretation rather than as ground truth. A text mining study that reports topics or sentiment without validating what they mean in the corpus produces numbers whose relationship to the phenomenon is unknown.

Use this skill when applying text mining or NLP to research. The goal is to prevent the agent from treating algorithmic output as ground truth, from choosing methods whose assumptions violate the text, and from skipping the validation that connects computation to meaning. The agent has freedom in method and scale, but the link from algorithmic output to substantive finding must be validated and argued.

## Core Rules

### Match The Method To The Question And The Text

Different NLP methods answer different questions and make different assumptions about language. The method must fit.

Match by:

- dictionary methods for measuring known constructs with validated lexicons;
- topic models for exploratory discovery of co-occurrence patterns;
- embeddings and transformer models for semantic similarity and contextual representation;
- supervised classification when labeled training data exist for the target construct.

A sentiment dictionary applied to ironic or domain-specific text, or a topic model used to measure a construct, applies a method outside its assumptions. The method's theory of language must fit the text and the question.

### Validate Algorithmic Output Against Human Reading

Computational output must be checked against the actual documents. Topics, clusters, and scores are not self-interpreting.

Validate by:

- reading representative documents assigned to each topic or cluster;
- checking that high-scoring cases actually exhibit the construct;
- having human coders validate a sample against the algorithmic labels;
- reporting agreement between computational and human coding.

A topic labeled "immigration policy" by the researcher may, on reading, be a cluster of procedural words. Without validation, the label is the analyst's imposition, not the model's finding.

### Do Not Treat Sentiment Or Topic Scores As Ground Truth

Sentiment, emotion, and topic scores are model predictions with error and bias, not measurements of attitude or theme.

Recognize by:

- sentiment models fail on irony, negation, sarcasm, and domain-specific language;
- topic models produce clusters whose meaning must be interpreted, not assumed;
- pretrained models carry biases from their training corpora;
- scores need uncertainty, not just point values.

Reporting that a corpus is "42 percent negative" as if it were a measured fact treats a noisy prediction as a precise measurement. The model's limitations must accompany the result.

### Build And Validate Domain-Specific Lexicons And Models

Off-the-shelf lexicons and models are built on general or other-domain data. They often fail on specialized research corpora.

Build by:

- validating general lexicons against the domain before use;
- developing domain-specific dictionaries where general ones fail;
- fine-tuning or training models on in-domain data where feasible;
- documenting the validation that justifies the chosen resource.

A general sentiment lexicon applied to clinical notes or legal text will misclassify routinely. Domain adaptation and validation are part of the method, not optional extras.

### Preprocess Text Deliberately, Not By Default

Preprocessing choices, lowercasing, stemming, stopword removal, n-grams, materially change results and must be justified.

Preprocess by:

- matching preprocessing to the method and the linguistic features that matter;
- recognizing that stemming or stopword removal can destroy the signal of interest;
- preserving negation, modality, and other function words where they carry meaning;
- reporting preprocessing choices and their rationale.

Default preprocessing pipelines can silently remove the very features that answer the question. Preprocessing is a set of analytic decisions, not a setup step.

### Account For Corpus Composition And Bias

The corpus is a sample of text, and its composition shapes every result. Composition and bias must be analyzed.

Account by:

- what the corpus represents and what it excludes;
- how platform, time period, and selection shape the language observed;
- how demographic and institutional biases in the corpus propagate into findings;
- the gap between corpus and the population of texts or speakers claimed.

A corpus of online posts generalizes to online posters, not to a population. Composition bias, if unanalyzed, makes findings about the corpus look like findings about the world.

### Report Reproducibly With Code, Data, And Parameters

Computational text analysis must be reproducible. Methods sections must allow exact replication.

Report by:

- the exact method, software, version, and parameters;
- preprocessing steps and random seeds;
- the corpus construction and any access limitations;
- code and, where possible, data or a data statement.

A text mining result that cannot be reproduced is a claim, not a finding. Reproducibility is part of the rigor, especially given the many degrees of freedom in the pipeline.

### Interpret Findings Substantively, Not Just Statistically

A topic or pattern is not a finding until it is interpreted in substantive terms.

Interpret by:

- what the computational pattern means in the domain;
- how it connects to theory and prior literature;
- what alternative interpretations the data allow;
- the limits of what the method can and cannot say about the phenomenon.

Topic coherence statistics and silhouette scores describe the model's internal properties. The contribution comes from the substantive interpretation that makes the pattern meaningful.

## Common Traps

### Algorithmic Output As Ground Truth

Treating topics, clusters, or sentiment scores as measured facts rather than model predictions needing validation.

### Method Mismatched To Text

Applying a general sentiment lexicon or off-the-shelf model to domain text where its assumptions fail.

### Unvalidated Labels

Naming topics or clusters without reading the documents to confirm what they actually contain.

### Default Preprocessing Destroying Signal

Routine stemming or stopword removal that silently removes the features of interest.

### Ignored Corpus Bias

Generalizing from a platform- or time-bound corpus to a population it does not represent.

### Irreproducible Pipeline

Reporting results without code, parameters, seeds, or corpus construction details.

### Statistical Without Substantive Interpretation

Reporting coherence or fit statistics without interpreting what the pattern means in the domain.

## Self-Check

- Is the NLP method chosen to fit the question and the text's linguistic properties?
- Is algorithmic output validated against human reading of representative documents?
- Are sentiment, topic, and embedding scores treated as predictions with error, not ground truth?
- Are lexicons and models validated or adapted for the specific domain before use?
- Is preprocessing deliberate and justified, preserving features that carry meaning?
- Is corpus composition and bias analyzed, with generalization bounded to what the corpus represents?
- Is the analysis reported reproducibly with code, parameters, seeds, and corpus construction?
- Are findings interpreted substantively, connecting computational patterns to theory and domain?
- Are alternative interpretations and the method's limits acknowledged?
- Does the validation connect the computation to the substantive meaning of the corpus?
