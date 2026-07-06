---
name: mt_output_evaluation.md
description: Use when the agent is evaluating machine translation output quality, assessing fluency accuracy and adequacy of MT, detecting machine translation errors such as hallucinations omissions and terminology drift, or deciding whether MT output is fit for purpose raw use light post-editing or full post-editing.
---

# MT Output Evaluation

Machine translation output varies enormously in quality, and evaluating it is a distinct skill from translating or reviewing human work. MT can produce fluent text that is wrong, accurate text that is unreadable, and confident errors that look correct to a non-expert. It can hallucinate content not in the source, omit content that is in the source, drift in terminology across a document, and handle some domains and language pairs far better than others. Evaluating MT output means judging not just whether it reads well but whether it conveys the source meaning accurately, completely, and consistently, and deciding what level of human intervention, if any, the output needs before use. Misjudging MT quality leads to publishing wrong content, over-spending on post-editing that was unnecessary, or under-spending on content that needed full retranslation.

Use this skill when evaluating machine translation output, assessing its fitness for purpose, detecting MT-specific error types, or deciding among raw MT use, light post-editing, and full post-editing. The goal is to judge MT output accurately and route it to the right level of human handling based on its actual quality and the content's requirements.

## Core Rules

### Evaluate Accuracy And Adequacy, Not Just Fluency

Fluency is the most misleading dimension of MT output, because modern MT produces fluent text even when the meaning is wrong. Evaluate accuracy and adequacy first.

Accuracy means the target conveys the source meaning correctly. Adequacy means the target contains the source's information completely, without omission or addition. To assess these, the evaluator must compare target against source, segment by segment, checking that meaning is preserved and nothing is dropped or invented. Fluency, how natural the target reads, matters but is secondary, because fluent wrong text is more dangerous than disfluent correct text.

Never judge MT output by reading the target alone. A target-only read misses the errors that matter most.

### Detect MT-Specific Error Types

MT produces characteristic error types that human translators rarely make. Learn to detect them.

Hallucinations are target content not present in the source, where the model generates plausible-sounding but invented text, especially common with neural MT on ambiguous or out-of-domain input. Omissions are source content dropped from the target, sometimes entire sentences. Additions are extra content inserted. Terminology drift is the same source term rendered differently across segments. Negation errors flip the meaning by mishandling not or no. Number and entity errors mistranslate values, dates, names, and units. Register and formality errors produce inconsistent or inappropriate address. Structural errors mangle syntax in ways that change meaning.

Knowing these types lets you scan MT output efficiently for the defects most likely to occur.

### Assess Fitness For Purpose

MT quality is not absolute. It is relative to purpose. Assess fitness for purpose before deciding on handling.

For gisting, where the reader only needs the gist and accepts imperfection, raw MT may suffice. For internal information sharing, light post-editing may suffice. For published content, full post-editing or retranslation is needed. For high-stakes content such as legal, medical, or safety, MT output should generally not be used without full expert review, and sometimes not at all. The acceptable error rate depends on the consequences of error.

State the purpose explicitly and judge the output against the quality that purpose requires, not against an abstract standard.

### Route Output To The Right Handling Level

Based on quality and purpose, route MT output to the appropriate handling. Define the routing criteria clearly.

Raw MT use applies when the purpose tolerates error and the output meets a minimum adequacy threshold. Light post-editing fixes only errors that impede understanding, accepting imperfect style, and suits internal or informational content. Full post-editing produces publication-quality text, fixing all errors and improving style, and suits external-facing content. Full retranslation applies when MT quality is too low to post-edit efficiently or when the content is too high-stakes for MT. Routing correctly avoids both over- and under-investment in human effort.

Document the routing decision so downstream handlers know what level of work is expected.

### Evaluate Consistency Across The Document

MT quality can vary across a document, with some segments excellent and others badly wrong. Evaluate consistency, not just average quality.

Check for terminology consistency, because MT often renders the same term differently in different segments. Check for register consistency, because formality can drift. Check for coherence across segments, because MT translates segments somewhat independently and can produce text that is locally fine but globally inconsistent. A document with high average quality but a few catastrophic segments is riskier than one with uniformly mediocre quality, because the catastrophic segments are hard to spot.

Sample evaluation across the document, including beginning, middle, and end, catches inconsistency that a small sample misses.

### Account For Language Pair And Domain Effects

MT quality depends heavily on language pair and domain. Account for these when evaluating.

High-resource language pairs with abundant training data, such as English to and from major European languages, generally produce better MT than low-resource pairs. Domains well-represented in training data, such as general news, produce better MT than specialized domains such as law, medicine, or niche technology. MT between distant languages, or involving languages with complex morphology or different syntax, tends to have more errors. Adjust quality expectations and evaluation thoroughness to the pair and domain.

Do not assume MT quality observed in one pair or domain transfers to another.

### Use Structured Evaluation Methods

Structured evaluation produces more reliable judgments than impressionistic reading. Use established methods where appropriate.

Adequacy and fluency scales rate each segment on defined criteria. Error classification catalogs errors by type and severity, useful for diagnosing MT weaknesses. Scalar quality metrics such as DAQP provide comparable scores. For larger evaluations, professional metrics such as BLEU, COMET, or human evaluation frameworks apply, though these serve MT development more than operational routing. For operational decisions, segment-level human review against source remains the most reliable method.

Choose the method that matches the decision being made, and apply it consistently.

### Flag Where MT Should Not Be Used

Some content should not be processed by MT at all, regardless of output quality. Recognize these cases.

Confidential content should not be sent to public MT services that may store or reuse it. High-stakes content such as legal, medical, safety, or regulatory text generally should not rely on MT without full expert review, and often should be human-translated. Creative and literary content, where MT cannot capture voice and effect, should be human-translated. Content where the source is itself flawed or ambiguous should be resolved before MT, because MT amplifies source problems.

Flag these cases and recommend alternatives rather than evaluating MT output that should not have been produced.

## Common Traps

### Judging MT By Fluency Alone

Fluent wrong text is the most dangerous MT output, because it looks correct without comparison to source.

### Missing Hallucinations And Omissions

Invented or dropped content is invisible in a target-only read and is a hallmark MT defect.

### Using Raw MT For High-Stakes Content

Publishing legal, medical, or safety content from raw MT creates risk of harm and liability.

### Over-Post-Editing Good MT

Spending full post-editing effort on output that needed only light editing wastes resources.

### Under-Post-Editing Poor MT

Accepting MT output that needed full editing publishes low-quality or wrong content.

### Evaluating A Small Unrepresentative Sample

MT quality varies across a document; small samples miss catastrophic segments and inconsistency.

### Assuming Quality Transfers Across Pairs And Domains

MT strong in one pair or domain may be weak in another; adjust evaluation accordingly.

### Sending Confidential Content To Public MT

Using public MT services for sensitive content risks data exposure and reuse.

## Self-Check

Before approving an MT evaluation or routing decision, verify:

- Accuracy and adequacy were assessed by comparing target against source, not by reading target alone.
- MT-specific error types, hallucinations, omissions, additions, terminology drift, negation, numbers, and register, were checked systematically.
- Fitness for purpose was assessed against the content's actual requirements and acceptable error rate.
- Output is routed to raw use, light post-editing, full post-editing, or retranslation based on quality and purpose, with the decision documented.
- Consistency was evaluated across the document, including terminology, register, and coherence, not just average quality.
- Language pair and domain effects on MT quality were accounted for in expectations and evaluation thoroughness.
- A structured evaluation method appropriate to the decision was applied consistently.
- Cases where MT should not be used, confidential, high-stakes, creative, or flawed-source content, were flagged with alternatives recommended.
- No segment was judged acceptable without source comparison, especially fluent segments that may conceal meaning errors.
- The routing decision matches the quality observed and the risk the content carries.
