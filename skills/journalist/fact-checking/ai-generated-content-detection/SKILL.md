---
name: ai_generated_content_detection.md
description: Use when the agent is assessing whether text, images, audio, or video may be AI-generated, deciding how to verify the authenticity of digital content in an era of generative AI, applying detection tools and provenance checks, or determining what disclosure and skepticism are appropriate when AI generation is suspected or confirmed.
---

# AI Generated Content Detection

Generative AI has fundamentally changed the verification problem, because the default assumption that media is real unless proven otherwise no longer holds. Text, images, audio, and video can now be synthesized that is convincing enough to pass casual inspection, and the tools are improving rapidly while detection tools lag behind. The judgment problem is that traditional verification relied on the difficulty of fabrication: a real-looking photo was probably real because faking it was hard. Now fabrication is cheap and easy, so the evidentiary value of "it looks real" has collapsed, and verification must rely on provenance, context, and technical signals rather than on the appearance of authenticity. The skill is approaching digital content with calibrated skepticism appropriate to the generative era, using provenance and context as the primary verification tools, treating detection tools as aids rather than oracles, and being honest about the limits of what can be detected.

The harm this skill prevents is the journalist being fooled by synthetic content, which undermines the core function of verification, and the parallel harm of false accusations that real content is AI-generated, which can be used to dismiss genuine evidence as fabrication. Both errors are serious and both are increasing. A newsroom that publishes AI-generated images as real betrays its readers and becomes a vector for manipulation. A newsroom that dismisses authentic evidence as "deepfake" without justification helps bad actors evade accountability by providing a ready-made denial. The agent must help the journalist apply detection methods appropriate to the content type, anchor verification in provenance rather than appearance, understand the limitations of detection tools, and maintain honest uncertainty when detection is inconclusive.

Use this skill when verifying the authenticity of digital content, when AI generation is suspected, when deciding whether to publish content of uncertain provenance, or when someone claims real content is a deepfake. The goal is to prevent the agent from being fooled by synthetic content or from falsely dismissing real content, and to ensure verification reflects the realities of the generative era.

## Core Rules

### Anchor Verification In Provenance, Not Appearance

In the generative era, appearance is no longer reliable evidence. Content that looks real may be synthetic, and verification must therefore rest on provenance, where the content came from and whether its origin supports its authenticity, rather than on how convincing it looks.

Anchor in provenance by:

- tracing the content to its original source, not the instance that went viral;
- identifying who created or first published it and their credibility;
- checking whether the source was in a position to capture or create the content authentically;
- looking for a chain of custody from the event to the publication;
- treating content with no traceable provenance as suspect regardless of how real it appears.

A real-looking image from an anonymous account with no connection to the event is suspect because it could be synthetic. A real-looking image from a verified journalist at the scene is far more trustworthy because provenance supports authenticity. Provenance, not pixels, is the foundation.

### Apply Type-Appropriate Detection Methods

Different content types have different detection approaches, and the journalist must match the method to the medium. Text, images, audio, and video each have distinct signals and tools, and applying the wrong approach produces false confidence or false alarms.

Apply by type:

- images: examine for anatomical inconsistencies, warped details, nonsensical text, and unnatural patterns; use reverse image search to find earlier versions;
- video: analyze frame-by-frame for temporal inconsistencies, unnatural motion, and blending artifacts; verify audio-video sync;
- audio: listen for unnatural prosody, robotic artifacts, and inconsistencies; compare to known samples of the purported speaker;
- text: assess for stylistic consistency, factual verifiability, and context; note that text detection is especially unreliable.

No method is definitive, and each has false positives and negatives. Use multiple methods and weigh them together rather than relying on any single signal.

### Treat Detection Tools As Aids, Not Oracles

Automated detection tools that label content as AI-generated or real are useful but unreliable, with significant false positive and false negative rates. Treating a tool's verdict as definitive is a serious error, because the tools can be fooled and their confidence is often overstated.

Use tools carefully by:

- treating detection tool output as one signal among many, not as a verdict;
- understanding that tools have false positives, labeling real content as synthetic, and false negatives, missing synthetic content;
- recognizing that detection tools lag behind generation tools and may not catch current models;
- using multiple tools and looking for agreement or disagreement;
- never publishing a claim of AI generation based solely on a tool output without corroborating evidence.

A tool that says "eighty percent likely AI-generated" is not a finding; it is a prompt for further investigation. The journalist must do the investigation, not relay the score.

### Verify Through External Corroboration

Because intrinsic detection is unreliable, the strongest verification is external corroboration, evidence from independent sources that the content depicts a real event. If multiple independent accounts, angles, or records confirm the event, the content is likely authentic regardless of detection uncertainty.

Corroborate by:

- seeking other media or accounts of the same event from independent sources;
- checking whether other journalists or witnesses reported the same thing;
- verifying details in the content against known facts about the event, location, and time;
- looking for official records, statements, or data that confirm or contradict;
- treating content that cannot be externally corroborated as lower confidence.

Content that shows a unique event with no other witnesses or records is harder to verify and warrants higher skepticism, because fabrication cannot be ruled out by corroboration. Content that aligns with multiple independent accounts is far more trustworthy.

### Maintain Honest Uncertainty

A defining feature of the generative era is that some content cannot be definitively classified as real or synthetic with available tools. The honest response to inconclusive verification is to say so, rather than to force a verdict that the evidence does not support. False certainty in either direction causes harm.

Maintain honest uncertainty by:

- stating clearly when verification is inconclusive and why;
- distinguishing high confidence, moderate confidence, and unable to verify;
- not publishing content as verified when it is merely unverified;
- not claiming content is AI-generated when detection is uncertain;
- updating assessments as better tools or evidence emerge.

The audience can handle uncertainty when it is honestly conveyed. What damages credibility is false certainty that later collapses, in either direction.

### Apply Heightened Skepticism To High-Stakes Content

The effort and sophistication of fabrication scale with the stakes, so content that would have high impact if believed warrants correspondingly higher skepticism. A viral image of a celebrity is low-stakes; a leaked video purporting to show a leader making damaging statements is high-stakes and a prime candidate for sophisticated fabrication.

Apply heightened skepticism by:

- reserving the highest verification standards for content that would significantly influence public opinion, elections, or conflicts;
- considering who would benefit from the content being believed or disbelieved;
- being especially cautious of content that appears at politically consequential moments;
- recognizing that sophisticated actors will target high-stakes moments with fabricated content;
- requiring stronger provenance and corroboration for high-impact claims.

A leaked damaging video released days before an election deserves far more scrutiny than a casual clip, because the incentive and capability to fabricate are both higher.

### Handle Deepfake Denial Of Real Content

The mirror image of being fooled by synthetic content is the weaponization of deepfake claims to dismiss authentic content. Bad actors increasingly claim real evidence is AI-generated to evade accountability, and journalists must be able to distinguish legitimate detection from cynical denial.

Handle deepfake denial by:

- applying the same rigorous verification to claims that content is fake as to the content itself;
- not accepting a bare assertion of deepfake without supporting evidence;
- looking for the provenance and corroboration that support the content's authenticity;
- noting when denial is self-serving and unsupported by technical evidence;
- being prepared to affirm content's authenticity when verification supports it, against cynical denial.

The rise of deepfakes does not make all real content deniable. When verification supports authenticity, the journalist should say so clearly, rather than letting unsupported denial create false doubt.

## Common Traps

### Trusting Appearance

Assuming content that looks real is real, when generative tools can produce convincing fakes. The trap is the collapse of appearance as evidence.

### Treating Detection Tools As Definitive

Publishing a tool's verdict as fact without corroborating investigation. The trap is overconfidence in unreliable technology.

### False Positives As Accusations

Claiming real content is AI-generated based on weak signals, which can be weaponized to dismiss genuine evidence. The trap is the harm of false fabrication claims.

### Ignoring Provenance

Focusing on pixels and detection while neglecting where the content came from. The trap is missing the strongest verification signal.

### Forcing A Verdict

Insisting on a definitive real-or-synthetic classification when the evidence is inconclusive. The trap is false certainty in either direction.

### Insufficient Skepticism For High-Stakes Content

Applying casual verification to content with high public impact. The trap is being fooled exactly when it matters most.

### Accepting Deepfake Denial At Face Value

Letting unsupported claims of fabrication create doubt about authentic content. The trap is enabling evasion through cynical denial.

## Self-Check

- [ ] Verification is anchored in provenance, tracing the content to its source, rather than relying on how real it appears.
- [ ] Detection methods are matched to the content type, with the limitations of each method understood.
- [ ] Detection tool outputs are treated as one signal among many, not as definitive verdicts.
- [ ] External corroboration from independent sources was sought to confirm or contradict the content.
- [ ] Honest uncertainty is conveyed when verification is inconclusive, rather than forcing a verdict.
- [ ] Heightened skepticism and stronger verification were applied to high-stakes, high-impact content.
- [ ] Claims that real content is a deepfake were subjected to the same verification rigor as the content itself.
- [ ] No content was published as verified when its authenticity could not be established.
- [ ] No accusation of AI generation was made based solely on detection tool output without corroboration.
- [ ] The coverage honestly conveys what was verified, what remains uncertain, and what detection cannot determine.
