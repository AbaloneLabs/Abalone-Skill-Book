---
name: design_feedback_quality.md
description: Use when the agent is writing or giving design feedback, deciding whether feedback is actionable, translating vague reactions into specific observations, or reviewing whether feedback a team receives will actually help a designer improve the work.
---

# Design Feedback Quality

Feedback is the currency of design collaboration, and most of it is spent poorly. The dominant failure is feedback that expresses a reaction without giving the designer anything to act on. "It feels cluttered", "make it more modern", "I do not love it", and "can you make it pop" are all variations of the same problem: the reviewer had an internal response, but instead of diagnosing it, they handed the raw feeling to the designer and expected them to decode it. The designer then guesses, iterates blindly, and returns with something that may or may not address the real concern. This cycle burns time and erodes confidence on both sides.

Good design feedback is specific, goal-oriented, and separable from personal taste. It names what the reviewer observes, explains why it is a problem relative to a user goal or constraint, and leaves the designer room to choose the fix. It also knows when to stop: when feedback has become micromanagement of craft the designer owns, or when it is masking an unspoken effectiveness concern the reviewer has not articulated. A product manager who can write and request high-quality feedback multiplies the value of every design interaction.

Use this skill before writing design feedback, before reviewing the feedback a designer has received from others, or when noticing that design iterations are not converging. Ask: is each piece of feedback specific enough that the designer knows what to change? Does it explain why, relative to a goal? Does it separate observation from directive? Is it the right level of detail for the design stage? And critically, is the feedback actually about the design, or is it expressing an effectiveness concern the reviewer has not yet named?

## Core Rules

### Diagnose Your Reaction Before Delivering It

The first discipline of good feedback is internal. When you react to a design, pause and ask yourself what specifically triggered the reaction. "It feels off" is not feedback; it is a signal that you have noticed something but have not yet identified it. Your job is to convert that signal into a diagnosis before you speak.

Work through it. Is the reaction about hierarchy, because too many elements compete for attention? Is it about flow, because the path through the screen is unclear? Is it about language, because the labels do not match user mental models? Is it about consistency, because the pattern differs from elsewhere in the product? Is it actually about whether the design solves the problem, which is a different and more important concern? Once you can name the specific issue, you have something worth saying. If you cannot, say that explicitly and explore it with the designer rather than issuing a vague directive.

### Make Feedback Specific And Observable

Specific feedback names what you see, where you see it, and what about it is problematic. Vague feedback forces the designer to guess. The difference is not subtle. "The header feels busy" leaves the designer unsure whether to remove elements, resize them, reorganize them, or change their styling. "The three calls to action in the header compete, and I cannot tell which is primary" identifies a specific structural problem the designer can address directly.

A useful test: could another person, reading only your feedback, reproduce the concern you have? If not, the feedback is too vague. Aim to describe the observation precisely enough that the designer can locate it in the design and understand the nature of the problem without further interpretation.

### Tie Every Concern To A Goal Or Constraint

Feedback lands better and is more useful when it is connected to something the team already agrees matters. A concern framed as "this will confuse first-time users who arrive without an account, which our research showed is the highest-friction moment" is far more actionable than the same concern framed as "this is confusing". The first gives the designer the reason and the stakes; the second gives only the verdict.

Connect feedback to user goals, success criteria, known constraints, research findings, accessibility requirements, or business outcomes. When feedback is anchored this way, the designer can weigh it against other considerations and make a reasoned choice. When it is unanchored, it becomes one opinion among many, and the designer has no basis to prioritize it.

### Separate Observation From Prescription

Strong feedback distinguishes between what you noticed and what you think should be done about it. Offer the observation as the primary content, and offer any suggestion as secondary and optional. This respects the designer's ownership of the solution and often produces better fixes than the reviewer would have imagined, because the designer has deeper craft expertise.

A workable pattern is to state the observation and its impact, then offer a direction as a possibility rather than a demand. "The error message appears below the fold on mobile, so users may not see it. One option is to surface it inline near the field." The designer can then solve the problem their own way, possibly better than the suggestion. If you only ever prescribe, you train the designer to implement rather than to think, and the design degrades.

### Match Feedback Density To The Design Stage

Feedback that is appropriate for an early concept is wrong for a polished design, and vice versa. Early in the cycle, feedback should explore whether the direction is right, question assumptions, and open alternatives. At this stage, detailed polish feedback is not only wasted, it is harmful, because it implies a level of finality the design does not yet have and can lock in decisions prematurely.

Late in the cycle, the direction should be settled, and feedback should focus on execution and consistency. Reopening fundamental questions at this stage destabilizes the work and threatens the timeline. Calibrate your feedback to the stage, and if you are unsure what stage the design is in, ask. Giving the wrong type of feedback at the wrong time is one of the most common and costly feedback mistakes.

### Distinguish Effectiveness Feedback From Craft Feedback

Effectiveness feedback asks whether the design solves the problem, meets the constraints, handles edge cases, and is accessible. This is the product manager's core responsibility, and pushing back hard here is appropriate. Craft feedback asks about visual hierarchy, spacing, typography, interaction details, and the felt quality of the experience. This is the designer's domain, and the PM should tread lightly.

The trap is that vague discomfort with craft is often actually an unspoken effectiveness concern. Before commenting on craft, ask yourself whether you are really reacting to an effectiveness problem you have not articulated. If the design "feels wrong" and you cannot say why in effectiveness terms, keep digging rather than translating the discomfort into craft feedback the designer cannot use.

### Prioritize And Synthesize Feedback

A designer who receives forty scattered comments cannot act effectively. Feedback is more useful when it is prioritized and grouped. Separate must-address effectiveness issues from should-address improvements from optional suggestions. Group related comments into themes so the designer sees patterns rather than a flat list. Identify the few changes that would most improve the design, rather than treating every comment as equal.

If you are consolidating feedback from multiple reviewers, this synthesis is especially valuable. Different reviewers will contradict each other, and the designer should not have to resolve those contradictions alone. The PM and design lead should reconcile, prioritize, and present a coherent set of directions.

### Close The Loop On Feedback Given

Feedback that is given and then ignored teaches the team that feedback does not matter. Conversely, feedback that is treated as mandatory teaches the team that every comment must be implemented. Neither is healthy. The healthy pattern is that feedback is considered, some is acted on, some is deliberately rejected with reasoning, and the reasoning is visible.

When you give feedback, follow up to understand how it was received and what was decided. When you receive feedback, acknowledge it and explain what you did with it. This turns feedback from a one-way dump into a dialogue that improves both the design and the team's feedback skills over time.

## Common Traps

### Delivering Raw Reactions As Feedback

Handing a designer your unprocessed feeling forces them to do the diagnostic work you skipped. The trap is believing you have given feedback when you have only expressed a mood.

### Prescribing Instead Of Observing

When feedback is only a list of changes, the designer stops thinking and starts executing. The trap is mistaking detailed prescriptions for helpful feedback, when they actually bypass the designer's expertise.

### Feedback That Cannot Be Prioritized

A flat list of equal-weight comments gives the designer no signal about what matters most. The trap is treating thoroughness as quality, when unprioritized feedback is harder to act on than less feedback.

### Mistimed Feedback

Polish feedback on an early concept, or directional feedback on a near-final design, both damage the work. The trap is assuming all feedback is equally valid regardless of stage.

### Craft Feedback Masking Effectiveness Concerns

Commenting on visuals when the real issue is whether the design works deflects from the harder question. The trap is that craft feedback feels safer to give than effectiveness feedback, so it gets substituted.

### Feedback Without Follow-Through

Feedback given and then never revisited teaches the team it is performative. The trap is assuming the act of giving feedback is the deliverable, rather than the dialogue it should start.

## Self-Check

- [ ] Each piece of feedback was diagnosed before delivery, with the specific trigger identified rather than a raw reaction handed over.
- [ ] Feedback is specific and observable enough that another person could reproduce the concern from the description alone.
- [ ] Every concern is tied to a user goal, success criterion, constraint, research finding, or accessibility requirement.
- [ ] Observations are separated from prescriptions, and suggestions are offered as possibilities rather than demands.
- [ ] Feedback density and type match the design stage (directional early, polish late).
- [ ] Effectiveness feedback, where the PM is accountable, is distinguished from craft feedback, where the designer leads.
- [ ] Feedback is prioritized and grouped into themes, with must-address issues separated from optional suggestions.
- [ ] Contradictions between multiple reviewers were reconciled before reaching the designer.
- [ ] A follow-up loop exists so the designer can explain what was acted on and what was rejected, with reasoning.
- [ ] No vague directives like "make it pop", "clean it up", or "it feels off" remain in the final feedback.
