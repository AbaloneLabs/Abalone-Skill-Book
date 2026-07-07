---
name: ai_feature_ux.md
description: Use when the agent is designing AI-powered features, generative interfaces, AI assistants, copilots, chat experiences, predictive tools, smart automation, or reviewing how users interact with probabilistic, model-driven, or AI-assisted product capabilities.
---

# AI Feature UX

AI features break the assumptions of traditional interface design. A normal button does exactly what it says, every time. An AI feature does something probabilistic: it is usually right, sometimes wrong, occasionally confident-and-wrong in ways that are hard to predict. Users have no reliable mental model for this, and the interface must compensate. When AI UX is designed like deterministic UX, users either trust it too much, get burned, and never return, or trust it too little and never benefit from it. The design challenge is calibrating trust, surfacing uncertainty, keeping the human in control, and making AI-assisted work genuinely better than the alternative.

Use this skill before designing or reviewing AI assistants, copilots, generative tools, smart compose, predictive inputs, automated workflows, chat interfaces, AI summaries, or any feature where a model produces probabilistic output the user must evaluate or act on. The goal is to prevent the agent from shipping a slick AI surface that hides uncertainty, removes user control, or produces confident errors the user cannot detect or correct.

## Core Rules

### Calibrate Trust To Actual Reliability

Users form trust expectations from the interface, and those expectations must match what the feature can actually deliver. Overclaiming competence ("AI-powered," "smart," "intelligent") sets users up to trust output that fails. Underclaiming prevents adoption.

Match the interface's confidence to the model's reliability:

- high-reliability tasks can be presented more assertively;
- lower-reliability tasks should signal that output needs review;
- never present probabilistic output as if it were deterministic fact;
- adjust framing as the model's actual performance on the task becomes known.

Trust is not maximized; it is calibrated. A user who trusts the feature exactly as much as is warranted gets the most value with the least harm.

### Keep The Human In Control Of Consequential Actions

AI should assist decisions, not silently make them. The interface must preserve the user's ability to review, edit, reject, and override before anything consequential happens.

For consequential outputs:

- show the AI's suggestion as a draft, proposal, or option, not a fait accompli;
- require explicit user confirmation before sending, publishing, paying, deleting, or sharing;
- let the user edit or regenerate before committing;
- preserve a clear path to do the task manually if the AI output is unsatisfactory;
- never let AI initiate irreversible actions without a human gate.

Silent AI action is appropriate only for low-stakes, easily reversible work. The higher the stakes, the more explicit the human checkpoint must be.

### Make Uncertainty And Limitations Visible

AI output often looks polished and confident even when it is wrong. The interface must counter that surface confidence by surfacing uncertainty and known limitations.

Techniques:

- indicate when output is generated, summarized, or inferred rather than factual;
- show confidence or alternative outputs where the model is uncertain;
- disclose known failure modes relevant to the task, such as hallucination, bias, or staleness;
- avoid presenting AI output in the same visual treatment as verified facts;
- provide source or grounding references where they exist.

A user who cannot tell AI output from verified data cannot apply appropriate skepticism, and that is a design failure.

### Design For Correction And Iteration

AI output is rarely perfect on the first attempt. The interface should make correction and iteration easy, not punish the user for the model's limitations.

Support:

- regenerate with the same or modified input;
- edit output directly rather than re-prompting from scratch;
- refine with follow-up instructions or constraints;
- reject and try a different approach;
- give feedback on output quality so the system can improve.

A feature that gives one shot and forces a full restart if the output is wrong wastes the user's time and the model's capability.

### Preserve User Agency Over What The AI Does

Users must be able to understand what the AI is doing on their behalf, turn it off, and control its scope. AI that operates invisibly in the background removes agency and creates anxiety.

- make background AI activity visible or auditable;
- let users disable specific AI capabilities without disabling the whole product;
- let users set boundaries on what the AI can access or act on;
- avoid AI features that cannot be turned off, especially those that use personal data;
- explain what data the AI uses and where it goes.

Agency is the difference between an assistant and a surveillance system that happens to be helpful.

### Set Expectations About What AI Can And Cannot Do

Users arrive with mental models shaped by marketing, media, and other products, and those models are often wrong. The interface must set accurate expectations.

- describe the feature's actual scope and limits in plain language;
- avoid implying general intelligence, consciousness, or omniscience;
- clarify what kinds of tasks it handles well and where it struggles;
- set expectations about latency, cost, or usage limits where relevant;
- correct common misconceptions rather than exploiting them.

A user who expects the AI to be infallible will be disappointed; a user who expects it to be useless will never try. Accurate expectations serve both.

### Handle Errors, Refusals, And Degraded Output Gracefully

AI features fail in ways deterministic features do not: refusals, hallucinations, truncated output, degraded quality under load, and inconsistent behavior across inputs.

Design for:

- clear messaging when the model refuses or cannot complete a task;
- honest handling of hallucinated or fabricated content when detected;
- graceful degradation under load or rate limits;
- recovery paths when output is unsatisfactory or errors occur;
- no silent failures where the user thinks the task succeeded but it did not.

An AI feature that fails silently, or that presents a failure as a success, is more dangerous than one that fails loudly.

### Respect Privacy, Data, And Consent

AI features often process sensitive user input or send data to models. Users must understand and consent to this.

- disclose what user data is processed by AI and where;
- avoid sending sensitive data to models without clear notice;
- let users opt out of AI features that use their data;
- honor data retention and deletion expectations;
- be especially careful with personal, health, financial, and confidential content.

Consent and transparency for AI data use are not optional add-ons; they are part of the feature.

## Common Traps

### Presenting Probabilistic Output As Fact

AI output shown with the same treatment as verified data invites overtrust and hides the need for review. Always signal generated or inferred content.

### Silent Consequential Action

Letting AI send, publish, pay, or delete without a human checkpoint turns a useful assistant into a liability.

### Overclaiming In Marketing Copy

"Intelligent," "smart," and "AI-powered" set expectations the model cannot meet, leading to disappointment and distrust.

### One-Shot Output Without Iteration

Forcing the user to accept or restart from scratch ignores how AI assistance actually works, which is iterative refinement.

### Invisible Background AI

AI that acts on user data or behavior without visibility or an off switch removes agency and breeds anxiety.

### Silent Failures And Confident Errors

Hallucinations or refusals presented as successful output mislead users who cannot detect the problem.

### Exploiting Misconceptions

Letting users believe the AI is more capable, aware, or objective than it is produces misplaced trust and eventual harm.

### Ignoring Privacy And Consent

Processing sensitive input through AI without notice or opt-in violates trust and often regulation.

## Self-Check

- [ ] The interface calibrates user trust to the feature's actual reliability, neither overclaiming nor hiding capability.
- [ ] Consequential actions require explicit human review and confirmation before the AI commits them.
- [ ] AI-generated or inferred output is visually distinguishable from verified facts, with uncertainty and limitations surfaced.
- [ ] Users can regenerate, edit, refine, reject, and give feedback on AI output without restarting from scratch.
- [ ] Users can see what the AI is doing, disable specific capabilities, and set boundaries on access and scope.
- [ ] Expectations about scope, limits, latency, and cost are set accurately in plain language.
- [ ] Errors, refusals, hallucinations, and degraded output are handled with clear messaging and recovery paths.
- [ ] No AI output is presented as a silent success when it may have failed or fabricated content.
- [ ] Users are informed about what data the AI processes and can opt out, especially for sensitive content.
- [ ] The feature avoids exploiting misconceptions about AI capability, awareness, or objectivity.
