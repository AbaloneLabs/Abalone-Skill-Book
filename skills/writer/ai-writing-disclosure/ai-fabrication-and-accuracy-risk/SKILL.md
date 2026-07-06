---
name: ai_fabrication_and_accuracy_risk.md
description: Use when the agent is verifying AI-generated text for accuracy, guarding against hallucinated sources quotes and facts, handling the special verification burden of model output, or preventing fabricated material from reaching readers.
---

# AI Fabrication And Accuracy Risk

AI models share a dangerous property: they produce fluent, confident text whether or not it is true. Unlike a human writer who signals uncertainty through hedging or a database that returns nothing when it has no answer, a language model will often generate plausible-sounding facts, citations, quotes, and sources that simply do not exist. This is fabrication, sometimes called hallucination, and it is the central accuracy risk of AI-assisted writing. The text looks exactly like verified information, which means the usual signal that something is unreliable, vagueness or hedging, is absent. A reader, and often the writer, has no surface clue that the material is invented.

The judgment problem is that the verification burden for AI-generated content is higher, not lower, than for human-drafted content, and in the opposite direction from what intuition suggests. Because the output reads so well, writers tend to trust it, skim it, and publish it, treating fluency as evidence of accuracy. An agent that applies ordinary editorial standards to AI output, checking only what looks suspicious, will miss fabricated material, because nothing about it looks suspicious. The craft is treating every factual element in AI-generated text as unverified until confirmed against an independent source, and understanding the specific ways models fabricate, so that the verification effort is targeted where the risk concentrates.

Use this skill when checking AI-generated drafts for accuracy, verifying sources and citations produced by a model, or deciding what verification AI-assisted content requires before publication. The goal is preventing fabricated material from reaching readers under a human's name.

## Core Rules

### Treat All AI Factual Claims As Unverified

The starting principle is that no factual claim, citation, quote, statistic, or source produced by an AI model carries any presumption of accuracy. Each must be independently verified before publication. This is a higher standard than human-drafted text warrants, because the failure mode of models, fluent fabrication, is invisible to casual reading. Adopt the stance that AI output is a set of hypotheses to check, not a set of facts to use.

Verify independently:

- every statistic, date, and quantitative claim;
- every citation and reference, which may be entirely invented;
- every direct quote and attribution, which may be fabricated or distorted;
- every named person, organization, study, or event;
- every causal or legal claim, where errors carry high stakes.

### Understand How Models Fabricate

Effective verification requires knowing the specific shapes fabrication takes. Models do not usually produce obvious nonsense; they produce material that fits the form of real information. They invent citations with plausible author names, journal titles, and page numbers. They generate quotes that sound like what a person might have said. They conflate real entities into nonexistent combinations. They fill gaps with confident-sounding detail. Knowing these patterns tells you where to look hardest.

Common fabrication patterns:

- citations with real-sounding authors and journals that do not exist;
- quotes attributed to real people that they never said;
- studies that synthesize real-sounding methodology around nonexistent data;
- conflation of two real sources into one false hybrid;
- confident specifics, dates, numbers, and titles, where the model is guessing.

### Verify Citations And Sources With Special Rigor

Citations are the most frequently and convincingly fabricated element in AI output, and they are the most damaging when published, because they lend false authority and send readers on fruitless searches. A model can produce a perfectly formatted reference to a paper that does not exist. Treat every citation as fabricated until you have located the actual source and confirmed it says what the text claims.

For each citation:

- locate the real source independently, not via the model's description;
- confirm the source exists and is accurately described;
- verify the cited claim actually appears and is represented fairly;
- check that quotes match the source verbatim;
- confirm dates, authors, and publication details.

If a source cannot be found, assume it is fabricated and remove the claim.

### Do Not Trust Confident Phrasing As Evidence

Models phrase fabricated content with the same confidence as accurate content. Words like "according to," "research shows," and "as X stated" appear whether or not the underlying claim is real. Do not let confident framing substitute for verification. The fluency of the sentence is evidence of the model's language ability, not of the truth of its content. Build the habit of separating how something is said from whether it has been confirmed.

### Check Quotes And Attributions Against Originals

Quotes are particularly risky because a model can produce text that sounds like a real quote from a real person but was never said, or can slightly distort a real quote into something different. Every direct quote in AI-generated text must be checked against the original source, word for word. Misattributed or invented quotes can defame, mislead, and damage credibility, and they are among the hardest fabrications to detect by reading alone.

Check that:

- the quoted person actually said or wrote those exact words;
- the quote is not paraphrased inside quotation marks;
- the attribution names the correct source and context;
- the quote is not a composite or reconstruction presented as verbatim.

### Verify Quantitative Claims And Data

Statistics, percentages, dates, and figures produced by models are frequently wrong or invented, even when embedded in otherwise reasonable prose. Numbers carry particular danger because they appear precise and authoritative. Verify every quantitative claim against a primary or reputable source, and watch for numbers that are plausible but unverifiable, which often indicate fabrication.

### Apply Heightened Verification To High-Stakes Domains

Some content carries consequences that make fabrication especially harmful: medical claims, legal guidance, safety instructions, financial information, and material about identifiable people. In these domains, a fabricated claim can cause real-world injury, legal liability, or reputational damage. Apply the strictest verification to high-stakes content, and where you cannot verify, remove the claim rather than publish it with a hedge.

### Prefer Removing Unverifiable Material Over Hedging It

When AI-generated material cannot be verified, the safe choice is to remove it, not to publish it softened with a hedge. A fabricated citation does not become acceptable because you add "reportedly"; an invented quote does not become honest because you attribute it loosely. Hedging fabricated content still presents it to readers as information. If you cannot confirm it independently, cut it.

### Keep A Verification Trail

For AI-assisted work, maintain a record of what was verified and how, so that accuracy claims can be supported and corrections can be made precisely if problems emerge later. A verification trail also disciplines the process: the act of recording what you checked reveals what you skipped. Where the medium allows, note the verification status of key claims.

## Common Traps

### Trusting Fluency As Accuracy

The most pervasive trap. Because AI text reads smoothly and confidently, writers treat it as reliable. Fluency is a property of the model's language generation, not of the content's truth.

### Skipping Citation Verification

Citations look authoritative and are tedious to check, so they are often accepted on trust. They are also the most commonly fabricated element. Unverified citations are the leading source of published AI fabrication.

### Accepting Plausible-Sounding Quotes

A quote that sounds like something a person might have said is not evidence they said it. Models generate plausible quotes routinely. Check every quote against the original.

### Hedging Fabricated Content Instead Of Removing It

Softening an unverifiable claim with "reportedly" or "it is said" does not make it publishable. Hedged fabrication still misleads. Remove what you cannot confirm.

### Treating Confident Numbers As Precise

Specific figures appear authoritative, but models generate wrong and invented numbers as confidently as correct ones. Quantitative claims need the same verification as qualitative ones.

### Publishing First, Correcting Later

Relying on post-publication correction is especially dangerous with AI fabrication, because the fabricated material may spread before anyone catches it, and some harms cannot be undone by a correction.

### Checking Only What Looks Suspicious

Fabricated content does not look suspicious; that is what makes it dangerous. Verification must be systematic, targeting the categories most prone to fabrication, not just what catches the eye.

## Self-Check

Before treating AI-generated content as accurate enough to publish, verify:

- Every factual claim, statistic, and date has been confirmed against an independent source.
- Every citation refers to a real source that has been located and checked.
- No reference is fabricated, conflated, or inaccurately described.
- Every direct quote matches an original source verbatim and is correctly attributed.
- Quantitative claims have been verified, not accepted for appearing precise.
- High-stakes claims in medical, legal, safety, or financial domains received heightened verification.
- Unverifiable material has been removed rather than hedged into publication.
- You have not trusted confident phrasing as a substitute for checking; verification targeted the fabrication-prone categories, not only what looked suspicious
- A record exists of what was verified and how, for key claims; no invented person, study, organization, or event remains in the text
- The piece would survive a rigorous fact-check of its most specific claims; you could defend every factual element as independently confirmed, not as model-generated
