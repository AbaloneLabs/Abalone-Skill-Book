---
name: reviewer_feedback_integration.md
description: Use when the agent is a translator or post-editor receiving review feedback, deciding which reviewer suggestions to accept reject or query, reconciling conflicting reviewer comments, responding to feedback without defensiveness, updating terminology and style based on review, or managing the loop where reviewer findings are integrated back into the translation before delivery.
---

# Reviewer Feedback Integration

Reviewer feedback is useful only if the translator integrates it well. Integration is the act of receiving review findings, evaluating each one, deciding to accept, reject, or query, applying the accepted changes, and reconciling disagreements, all before the translation is delivered. Agents often mishandle this stage in two directions. They either accept every reviewer change uncritically, treating the reviewer as infallible, which lets reviewer errors and preferences overwrite correct translator decisions and can introduce new defects. Or they reject feedback defensively, treating the reviewer as an adversary, which preserves translator errors and erodes the review's value. Both fail in the same way: they skip the evaluation that integration actually requires. Good integration is a judgment act. The translator has context the reviewer may lack, the reviewer has distance the translator lacks, and the goal is a final text that is better than either could produce alone. The harm of poor integration is that review effort is wasted, the delivered text is either reviewer-dominated or translator-stubborn rather than collaboratively best, and the feedback loop that should build quality instead breeds friction.

Use this skill when receiving review feedback, deciding on each finding, reconciling conflicting comments, responding to a reviewer, or establishing how translators should process review input. The goal is to integrate reviewer findings through informed judgment so the final text improves without importing reviewer errors or preserving translator errors.

## Core Rules

### Evaluate Each Finding On Its Merits, Not By Source

Every reviewer finding must be evaluated on its merits, regardless of who raised it. A senior reviewer can be wrong; a junior reviewer can be right. Evaluate against the source meaning, the termbase, the style guide, the target language's norms, and the segment's context. Accept findings that genuinely improve correctness or fluency, reject findings that are wrong or that are preferences presented as errors, and query findings where the reviewer may lack context the translator has. Evaluating by source authority rather than by merit produces a text shaped by hierarchy rather than by quality, and it lets reviewer errors through when the reviewer outranks the translator. The reviewer's role is to catch what the translator missed, not to override by status.

### Distinguish Errors From Preferences Before Deciding

Before deciding on a finding, classify it. An error is a deviation from source meaning, the termbase, the style guide, or language norms that affects correctness; a preference is an alternative rendering that is not more correct, only different. Errors demand a fix; preferences are optional and should not be imposed. Many reviewer comments are preferences framed as corrections, and accepting them all clutters the text with unnecessary changes and demoralizes the translator. Classifying first keeps the decision grounded: fix the errors, consider the preferences on their contribution, and do not let preferences masquerade as mandatory. This also feeds back into reviewer calibration, because patterns of preference-as-error should be raised with the reviewer or lead.

### Use The Translator's Context To Judge Reviewer Findings

The translator often holds context the reviewer does not: a decision made earlier in the file, a client instruction, a terminology choice forced by a constraint, a source ambiguity that was resolved deliberately. Use that context to judge findings. A reviewer flagging an apparent inconsistency may not know the client required it; a reviewer suggesting a term may not know the termbase mandates another. Where context changes the answer, apply it, and where the reviewer would benefit, share the context in the response so the next review is better informed. Ignoring one's own context and deferring to the reviewer discards the translator's most valuable contribution; wielding context defensively to dismiss all feedback discards the reviewer's. The productive middle is context applied to judgment and shared to improve the loop.

### Reconcile Conflicting Reviewer Comments Deliberately

When two reviewers disagree, or when one reviewer's comments conflict across segments, reconcile deliberately rather than picking arbitrarily. Identify the principle each side reflects: one may prioritize source fidelity, the other target fluency; one may follow the termbase strictly, the other may favor naturalness. Decide which principle governs for this content and this segment, and apply it consistently. Document the decision and its rationale so the same conflict does not recur unresolved. Unreconciled conflicts produce a text that flips style or terminology by segment, which is itself a defect class. Escalate to a lead or to the client only when the conflict reflects a genuine ambiguity in the brief that the translators cannot resolve.

### Respond To Feedback Without Defensiveness

The manner of response matters as much as the decision. Explain rejections with reasons tied to source, termbase, or context, not with dismissal. Acknowledge accepted findings. Query unclear findings constructively, asking what the reviewer intended rather than assuming. Defensive responses, terse rejections, or silent overrides poison the loop: the reviewer stops engaging, the translator stops benefiting, and quality suffers. Professional, reasoned responses keep the loop productive and build the trust that makes future review more effective. Treat the reviewer as a collaborator whose findings are inputs to a shared decision, not as an opponent to be defeated.

### Apply Accepted Changes And Re-Verify

Integration is not complete when decisions are made; it is complete when accepted changes are applied and the affected segments are re-verified. A late change can introduce a new inconsistency, break an agreement with another segment, or disturb a placeholder. After applying accepted changes, re-read the affected segments and their neighbors, re-run automated QA, and confirm that the change did not create a new defect. Overconfidence that a small change is safe is how late-stage defects enter. Treat post-integration verification as a mandatory step, not an option.

### Capture Learning For Future Work

Review feedback that reveals a recurring pattern, a terminology preference, a style convention, or a common mistranslation trap, should be captured for future work, not just applied to the current file. Update the termbase, the style guide, or a personal note from accepted findings so the same improvement persists. Feedback applied only to one file and forgotten forces the reviewer to raise the same point on the next project. The integration loop's long-term value is cumulative learning; capturing it turns each review into a permanent quality gain.

### Close The Loop With The Reviewer

Integration ends with closure, not silence. Confirm to the reviewer which findings were accepted, which were rejected with reasons, and which were queried, so the reviewer knows their input was considered and can calibrate future review. A loop that ends when the translator stops responding leaves the reviewer guessing whether their findings landed, which degrades calibration and morale. Closure also gives the reviewer the chance to escalate a rejected finding they consider critical, before delivery rather than after.

## Common Traps

### Accepting Every Reviewer Change Uncritically

Treating the reviewer as infallible lets reviewer errors and preferences overwrite correct decisions and can introduce new defects.

### Rejecting Feedback Defensively

Treating the reviewer as an adversary preserves translator errors, erodes the review's value, and breeds friction.

### Evaluating Findings By Authority Not Merit

Letting hierarchy decide produces a text shaped by status rather than quality and lets senior reviewer errors through.

### Preferences Treated As Mandatory Errors

Accepting preference comments framed as corrections clutters the text and demoralizes the translator.

### Discarding Translator Context

Deferring to the reviewer without applying one's own context discards the translator's most valuable contribution to the joint decision.

### Unreconciled Conflicting Comments

Picking arbitrarily between disagreeing reviewers produces a text that flips style or terminology by segment, itself a defect.

### Applying Changes Without Re-Verification

Assuming a small accepted change is safe skips the re-read and re-QA that catches defects introduced by late edits.

### Feedback Applied But Not Captured

Fixing the current file without updating the termbase, style guide, or notes forces the reviewer to raise the same point next time.

## Self-Check

Before treating reviewer feedback as integrated and the translation as ready, verify:

- Each finding was evaluated on its merits against source, termbase, style guide, and context, not accepted or rejected by reviewer authority.
- Findings were classified as error or preference before deciding, with preferences treated as optional and not imposed as mandatory.
- The translator's context, earlier decisions, client instructions, and constraints, was applied to judge findings and shared with the reviewer where relevant.
- Conflicting reviewer comments were reconciled by identifying the governing principle and applying it consistently, with the decision documented.
- Responses to the reviewer were reasoned and non-defensive, explaining rejections with evidence and querying unclear findings constructively.
- Accepted changes were applied and the affected segments and neighbors were re-read and re-checked with automated QA to catch introduced defects.
- Recurring patterns from accepted findings were captured in the termbase, style guide, or notes so the improvement persists across projects.
- The loop was closed with the reviewer, confirming accepted, rejected, and queried findings so calibration and trust are maintained.
- No finding was accepted uncritically or rejected defensively, and no accepted change was left unverified.
- The final text reflects informed integration, better than either translator or reviewer alone could produce, rather than reviewer domination or translator stubbornness.
