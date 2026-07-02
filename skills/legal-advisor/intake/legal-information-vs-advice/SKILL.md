---
name: legal_information_vs_advice.md
description: Use when the agent is deciding whether a request is general legal information or case-specific legal advice, assessing unauthorized practice of law risk, explaining the difference between a paralegal and a licensed attorney, defining a limited scope or consulting role, or routing a user toward qualified counsel.
---

# Legal Information Versus Advice

The line between legal information and legal advice is the line between a permitted explanation and the unauthorized practice of law. Crossing it can expose the speaker to criminal penalties, civil liability, professional sanctions, and harm to the person who relied on the answer. Agents are particularly prone to crossing this line because they can produce fluent, specific, confident-sounding answers, and because users often phrase a request for advice as a request for information. The deeper judgment problem is that the same words can be information or advice depending on context: who is asking, whether the answer is applied to their specific facts, whether it recommends a course of action, and whether the recipient is reasonably relying on it as tailored guidance.

Use this skill before answering a legal question, drafting explanatory content, building a self-help legal tool, designing a chatbot intake, supporting a paralegal or non-attorney advisor, or deciding whether to route a user to counsel. The goal is to make the agent classify the request, stay on the information side of the line when no attorney is involved, use conditional and general framing, and escalate when the request is really advice in disguise. This skill does not authorize any agent to give legal advice; it helps the agent recognize when advice is being requested and avoid providing it.

## Core Rules

### Classify The Request Before Answering

Before responding, determine what is actually being asked.

- General legal information explains the law, a process, or a concept without applying it to a specific person's situation.
- Legal issue spotting identifies categories of issues a reader should consider, usually conditionally.
- Legal analysis applies law to a specific set of facts to reach a conclusion.
- Legal advice recommends what a specific person should do in their situation.

Only the first two are generally safe for a non-attorney to provide. Analysis applied to a specific user's facts, and any recommendation about what that user should do, is advice. When in doubt, treat the request as advice and route it.

### Apply The Specificity And Reliance Tests

Whether content is advice often turns on two factors. First, specificity: is the answer tailored to the user's particular facts, documents, jurisdiction, deadlines, and goals? The more specific, the more it looks like advice. Second, reliance: would a reasonable person understand the answer as guidance they can act on for their own situation? A general explanation of small claims procedure is information; telling a specific user that they should file in small claims court by Friday is advice. A strong answer stays general, conditional, and disclaimed; a weak answer quietly becomes specific and directive.

### Recognize Unauthorized Practice Of Law Risk

The unauthorized practice of law is providing legal advice or services without a license, and it is regulated by jurisdiction, often through statutes, court rules, and bar enforcement. The definition varies, but the core is providing individualized legal judgment. Penalties can include injunctions, fines, contempt, and in some places criminal charges. Non-attorneys, paralegals, legal technicians, document preparers, software tools, and AI agents all face this risk. Even an attorney licensed in one jurisdiction can commit unauthorized practice by advising in a jurisdiction where they are not licensed.

### Distinguish Roles And Their Scope

Different roles carry different authority.

- A licensed attorney may provide legal advice and representation within their licensed jurisdictions and practice areas.
- A paralegal or legal assistant supports an attorney but does not independently provide advice.
- A limited license legal technician or non-attorney advocate may provide specific permitted services defined by their jurisdiction, such as form preparation or procedural guidance, but not full representation.
- A compliance officer, HR professional, or business advisor may explain internal policy and general rules but should not give case-specific legal advice.
- A document preparer may fill in forms but should not recommend strategy or interpret the user's rights.

Identify the speaker's role and stay within it. When the request exceeds the role, escalate.

### Use Limited Scope And Consulting Framing

When a non-attorney is permitted to assist, the assistance should be bounded by a clear, written scope. A limited scope engagement states exactly what the advisor will and will not do, for example form preparation only, general explanation only, or procedural coaching without strategic advice. The advisor must then resist scope creep when the user asks follow-up questions that cross into advice. Documenting the scope protects both sides and makes the boundary enforceable.

### Watch For The Advice In Disguise Patterns

Users rarely ask directly for advice. Common patterns that are actually advice requests include "what should I do," "do I have a case," "is this legal," "can they do that to me," "will I win," "should I sign this," and "how do I get out of this." Any answer that evaluates the user's specific situation, predicts an outcome, or recommends action is advice. A safe response reframes these into general considerations and recommends consulting a licensed attorney for the specific decision.

### Use Disclaimers Correctly, Not As A Shield

Disclaimers matter, but they do not transform advice into information. A paragraph saying "this is not legal advice" attached to a specific recommendation does not cure unauthorized practice. The substance controls: if the content is specific, tailored, and directive, the disclaimer is ineffective. Use disclaimers honestly by making the content genuinely general and conditional, then adding the disclaimer to reinforce it.

### Respect Jurisdiction And Confidentiality Limits

Legal information that is correct in one jurisdiction can be wrong in another. An information-only answer should identify the jurisdiction it describes and avoid asserting that the rule applies everywhere. Additionally, a non-attorney does not hold attorney-client privilege, so users may disclose sensitive information without the protection they expect. Intake tools should warn users not to share confidential facts until they are speaking with counsel.

## Common Traps

### Answering "Do I Have A Case" With A Probability

Evaluating the strength of a specific user's claim is advice. Reframe to general factors that affect such claims and recommend an attorney consultation.

### Letting Fluency Create Authority

A confident, well-structured answer feels authoritative even when it is information. Fluency can cause a user to rely on it as advice, which is exactly the harm the line is meant to prevent.

### Treating "Not Your Lawyer" As A Magic Phrase

The label does not control. If the substance is specific advice, the disclaimer does not protect the speaker from unauthorized practice.

### Providing Jurisdiction-Specific Answers Without Confirming The Jurisdiction

A correct general rule stated as if it applies to the user's state can mislead. Confirm jurisdiction or keep the answer explicitly general and conditional.

### Encouraging Disclosure Of Sensitive Facts In A Non-Privileged Channel

A chatbot or intake form that elicits detailed case facts without an attorney in the loop can create a record that is not privileged and that the user wrongly believes is confidential.

### Drifting From Form Preparation To Strategy

Document preparers and limited-scope helpers often drift into advising which form to choose or how to answer a question strategically. Each drift crosses the line.

### Assuming An Out-Of-State Attorney Can Advise Anywhere

A licensed attorney advising a user in a state where they are not admitted can still commit unauthorized practice. Confirm the attorney's admission in the relevant jurisdiction.

## Self-Check

- Is the request classified as general information, issue spotting, analysis, or advice, and is the response kept within the permitted categories for the speaker's role?
- Does the answer avoid applying law to the user's specific facts, documents, deadlines, and goals, and avoid recommending a specific course of action?
- Is unauthorized practice of law risk recognized, and is the speaker's role, license, and jurisdiction confirmed before answering?
- For non-attorney roles, is the scope of permitted services defined and documented, and is scope creep resisted?
- Are "what should I do" style questions reframed into general considerations with a referral to counsel rather than answered directly?
- Is any disclaimer supported by genuinely general and conditional content, rather than used to mask specific advice?
- Is the jurisdiction identified or kept general, and are users warned not to disclose confidential facts in non-privileged channels?
- Does the output avoid predicting outcomes, evaluating the strength of a specific claim, or advising whether to sign, file, or settle?
- Does the agent escalate to qualified counsel whenever the request is advice in substance, regardless of how it is phrased?
