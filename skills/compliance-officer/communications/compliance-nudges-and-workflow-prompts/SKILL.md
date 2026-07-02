---
name: compliance_nudges_and_workflow_prompts.md
description: Use when the agent is designing just-in-time compliance prompts at decision points, embedding nudges into business systems and workflows, using choice architecture to make compliant behavior the easy default, or placing compliance guidance where employees make risky decisions rather than only in annual training.
---

# Compliance Nudges And Workflow Prompts

Most compliance failure happens at a specific moment of decision: the click that approves an unusual payment, the email that attaches a customer list, the form that waives a required check. Annual training is nowhere near these moments. A nudge or workflow prompt placed exactly at the decision point can prevent the failure in the second it occurs, while a training taken months earlier has been forgotten. The power of just-in-time prompts is that they meet the employee at the moment of risk with the smallest possible useful intervention. The danger is that poorly designed prompts become noise that people click through reflexively, or worse, prompts that are technically present but never seen because they sit in the wrong system, at the wrong step, in the wrong wording.

Use this skill before adding compliance prompts to a business system, designing a just-in-time nudge, reviewing why a control failed despite a prompt existing, or deciding where in a workflow to intervene. The goal is to make the agent use choice architecture and contextual prompts as precision controls, not as decorative warnings.

## Core Rules

### Locate Prompts At The Genuine Decision Point, Not Anywhere Convenient

A prompt is only useful if it appears at the exact moment and place the decision is made. A data-privacy reminder on the intranet homepage does not help an employee about to email a customer list; a prompt that appears in the mail client when an external recipient and an attachment are detected does.

For each risk, map the decision journey:

- which system or tool the employee uses;
- which step in the process the risky decision occurs;
- what data, action, or trigger signals that the risk is present;
- where a prompt can be inserted that the employee will actually see before acting;
- what the smallest useful intervention is at that point.

Place the prompt at the trigger, not upstream where it will be forgotten or downstream where the decision is already made. If the genuine decision point is inside a system compliance does not control, securing placement there is the real work, not posting a reminder elsewhere.

### Make The Compliant Choice The Default

Choice architecture is more powerful than warnings. Making the compliant option the default, the easiest path, or the pre-selected choice reduces failure far more than telling people to be careful.

Apply defaults and architecture by:

- pre-selecting the compliant option in forms and approvals;
- requiring affirmative action to choose a riskier path, such as a justification field that activates only when an exception is selected;
- making the compliant path require fewer clicks than the non-compliant one;
- surfacing relevant policy or guidance inline rather than requiring the employee to find it;
- blocking, not just warning, when an action is clearly prohibited and cannot be justified;
- routing high-risk actions through an approval that is easy to request and fast to complete.

Defaults respect the employee's autonomy while steering toward the right outcome. Warnings alone, layered over a process that still makes the wrong path easiest, will fail.

### Keep Prompts Minimal, Specific, And Actionable

A prompt that is long, legalistic, or generic will be dismissed without reading. The most effective prompts are short, specific to the moment, and tell the user exactly what to do.

Design prompts so that each one:

- is short enough to read in the time the user has at that step;
- names the specific risk present at this moment, not compliance in general;
- states the action expected in plain language;
- offers the next step, such as a link to the policy, a help contact, or an approval form;
- avoids repeated identical wording across many screens, which trains users to dismiss them;
- appears only when the risk trigger is present, not on every action.

Prompts that fire constantly become invisible. Reserve them for moments that matter, and vary them so attention is maintained.

### Distinguish Hard Blocks From Soft Prompts By Risk

Not every risky action should be merely prompted. Some should be blocked entirely, and the distinction must be deliberate.

Use a tiered intervention model:

- informational prompts for low-severity risks where the user needs awareness;
- confirmation prompts for medium-severity risks where the user must affirm they have checked;
- justification prompts for exceptions, requiring a reason that is recorded and reviewable;
- soft blocks that require a second person or approval to proceed;
- hard blocks for actions that are prohibited and cannot be justified, with a clear escalation path if the block is wrong.

Choosing the wrong tier is a control failure. A hard block on something legitimate frustrates users and gets bypassed or removed; a mere prompt on something truly prohibited lets the failure through.

### Integrate With The Systems Where Work Actually Happens

The hardest part of workflow prompting is technical integration. Prompts that live only in compliance-owned channels miss the real decisions, which happen in finance, sales, procurement, engineering, and HR systems.

To integrate effectively:

- partner with the system and product owners, not around them;
- build prompts into the native workflow rather than a separate compliance tool;
- use existing trigger data, such as vendor bank changes, external recipients, or high-value transactions;
- respect system performance, latency, and user experience;
- plan for system upgrades and changes that break integrations;
- test that prompts actually fire on the intended triggers in production;
- monitor whether prompts are being bypassed, dismissed, or ignored at high rates.

A prompt that exists in a design document but not in the live system is not a control. Verify deployment and continued operation.

### Measure Whether Prompts Change Behavior

A prompt that is shown but does not change behavior is wasted screen space. Measure the intervention, not just its presence.

Track for each prompt:

- how often it fires;
- how often users proceed, modify, or abandon the action after seeing it;
- whether the rate of risky actions or exceptions decreased after deployment;
- whether users are dismissing it reflexively, indicated by near-instant proceed rates;
- whether the prompt is generating help-line calls, approvals, or reports as intended;
- whether bypass or workaround patterns are emerging.

If a prompt shows no behavior change, redesign it, reposition it, or replace it with a harder control. Do not leave an ineffective prompt in place just to show a control exists.

### Keep Prompts Current With Policy And Risk Changes

Prompts are often deployed once and forgotten. When the policy, threshold, or risk changes, the prompt becomes misleading.

Maintain prompts by:

- tying prompt content to the controlling policy version;
- reviewing prompts on the same cycle as the policy they support;
- updating triggers when workflows or systems change;
- retiring prompts for risks that no longer apply;
- documenting ownership so someone is accountable for each live prompt.

A stale prompt that contradicts current policy is worse than no prompt, because it creates a false sense of control.

### Preserve User Trust And Avoid Nag Fatigue

Over-prompting destroys the effectiveness of all prompts. Users who see constant low-value warnings develop banner blindness and dismiss even important ones without reading.

Protect trust by:

- prompting only when a genuine risk trigger is present;
- consolidating overlapping prompts from different functions;
- making prompts genuinely useful so users value them;
- removing prompts that data shows are ineffective;
- respecting the user's time and workflow.

A workforce that trusts prompts as helpful will heed them; a workforce that sees them as noise will ignore all of them.

## Common Traps

### Prompt In The Wrong Place

A reminder far from the decision point is forgotten before it matters. Place prompts at the genuine trigger.

### Warning Instead Of Default

Telling people to be careful over a process that still makes the wrong path easiest will fail. Use defaults and architecture.

### Long, Generic, Or Repeated Prompts

Users dismiss prompts that are long, vague, or identical across screens. Keep them short, specific, and varied.

### Wrong Intervention Tier

A mere prompt for a prohibited action, or a hard block for a legitimate one, is a control design failure. Match tier to risk.

### Prompt Exists In Design But Not In The Live System

A prompt that is not deployed or is broken by a system change is not a control. Verify production operation.

### Unmeasured Prompts

Showing a prompt without tracking behavior change leaves ineffective controls in place. Measure effect.

### Stale Prompts After Policy Change

A prompt that contradicts current policy misleads users. Tie content to policy version and review on cycle.

### Nag Fatigue From Over-Prompting

Too many low-value warnings create banner blindness. Prompt only at genuine risk triggers.

## Self-Check

- Is each prompt located at the genuine decision point in the system where the risk is realized, not merely somewhere convenient?
- Is the compliant choice made the default or easiest path, with riskier options requiring affirmative action or justification?
- Are prompts short, specific to the moment, actionable, and varied rather than long, generic, or identically repeated?
- Is the intervention tier, informational, confirmation, justification, soft block, or hard block, matched to the severity of the risk?
- Are prompts integrated into the native systems where work happens, in partnership with system owners, with production deployment verified?
- Is each prompt measured for fire rate, user response, behavior change, reflexive dismissal, and emerging workarounds?
- Are prompts tied to controlling policy versions and reviewed when policy, risk, or workflows change?
- Has over-prompting and nag fatigue been controlled by prompting only at genuine triggers and removing ineffective prompts?
- Are overlapping prompts from different functions consolidated to avoid a noisy user experience?
- Could the organization defend that the prompt changed behavior at the moment of risk rather than merely existing on the screen?
