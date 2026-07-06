---
name: explanation_and_concept_writing.md
description: Use when the agent is writing explanatory technical content, building mental models, using analogies for technical subjects, writing concept guides, or producing the Diátaxis explanation mode that deepens understanding without teaching a task.
---

# Explanation And Concept Writing

Explanation is the mode of technical writing that builds understanding. It is neither a tutorial, which teaches by doing, nor a reference, which documents what is. Explanation answers the question "why" and "what is really going on here." A reader leaves an explanation not with a new ability but with a new mental model: they can now reason about the system, anticipate its behavior, and connect it to what they already know.

Agents tend to miss what makes explanation hard because it looks like ordinary prose. The temptation is to dump everything the author knows about a topic, or to write a feature tour disguised as concept. But good explanation is selective, opinionated, and oriented around the reader's confusion, not the author's expertise. It chooses one controlling insight and builds outward, sacrificing breadth for the clarity that makes a concept click.

You have wide latitude in voice, analogy, and structure. The obligation is that the reader genuinely understands the concept more deeply after reading, and that the explanation is honest about where the model simplifies.

## Core Rules

### Identify The Controlling Insight

An explanation should be governed by a single insight the reader needs to grasp. "What is caching" is a topic; "why a cache trades freshness for speed, and when that trade breaks down" is a controlling insight. Before writing, name the one understanding the reader should walk away with. If you cannot state it in a sentence, the explanation is not yet focused.

Every section should serve that insight. Material that is interesting but does not advance the controlling understanding is a candidate for a different document. Explanation is not the place to be comprehensive; it is the place to make one important thing clear.

### Start From The Reader's Current Model

Explanation works by modifying an existing mental model. Find out what the reader already believes, including what they believe incorrectly, and start there. A reader who thinks a database is "just a spreadsheet" can be moved toward transactions and isolation levels from that starting point. A reader who already knows relational theory needs a different entry.

Name the common misconceptions explicitly and treat them with respect, not condescension. "It is tempting to think X, because Y; here is where that intuition breaks" is more effective than "X is wrong." Explanation that ignores the reader's starting model lectures past them.

### Choose Analogies That Preserve Structure

Analogies are the most powerful and most dangerous tool in explanation. A good analogy maps the structure of a familiar thing onto the structure of the unfamiliar concept, so the reader can reason by transfer. A bad analogy matches only surface features and misleads.

Test an analogy by asking which relationships it preserves. If you compare a process scheduler to a restaurant host, the analogy is useful only if the reader can correctly infer new things: requests wait in a queue, priority affects ordering, starvation is possible. State explicitly where the analogy holds and where it breaks. Every analogy fails somewhere; honest explanation names the failure rather than letting the reader overextend it.

### Build From Concrete To Abstract

Readers grasp abstractions best when they are anchored in concrete examples first. Show a specific request, a specific failure, a specific data shape, then generalize. Do not open with definitions of abstract terms and expect the reader to hold them in suspension while you explain.

Layer the abstraction: a concrete instance, then the pattern it illustrates, then the general principle, then a second concrete instance that tests whether the reader can apply the principle. The movement between concrete and abstract is what makes a concept stick, rather than slide off.

### Explain Why, Not Just What

Reference material tells the reader what a feature is. Explanation tells the reader why it exists, what problem it solves, and what the designers were trading off. A reader who understands why a system behaves as it does can predict its behavior in situations the documentation never covers.

Surface the forces in tension: consistency versus availability, simplicity versus flexibility, safety versus speed. Naming the tradeoff helps the reader see the concept as a response to a real problem, not an arbitrary design choice. Avoid presenting contingent decisions as inevitable.

### Connect To What The Reader Already Knows

New concepts are learned by attachment to existing ones. Explicitly link the concept to things the reader likely already understands: related systems, familiar patterns, prior explanations in the same series. "You already understand X; this is like X in these ways and different in these ways" is one of the most efficient explanatory moves.

Build a small, consistent vocabulary across related explanations so concepts reinforce each other rather than each inventing their own terms. Cross-link to other explanations and to the relevant reference and tutorial material, so the reader can move between understanding and action.

### Be Honest About Simplification

Every explanation simplifies. The skill is in being honest about it. If you describe a system as if it has no exceptions, the reader will be confused when they meet the exceptions later. If you collapse a multi-stage process into a single step for clarity, say that you have done so and point to where the full picture lives.

Mark opinion as opinion. Explanation often requires taking a stance on which mental model is most useful, and that is legitimate, but the reader should know when they are receiving the author's recommended framing versus the system's authoritative behavior. Confusing the two undermines both explanation and reference.

### Use Diagrams And Examples Deliberately

A diagram can convey structure that prose struggles with: relationships, layers, flows, boundaries. But a diagram that tries to show everything shows nothing. Choose diagrams that illustrate the controlling insight, label them clearly, and refer to them in the text rather than dropping them in as decoration.

Pair every diagram with a concrete example that the reader can trace through it. A diagram of a request flow is far more useful when the reader can follow a specific request through each stage. Avoid diagrams that encode the author's mental model without translating it for the reader.

## Common Traps

### The Feature Tour Disguised As Explanation

Walking through every capability of a system is reference, not explanation. Explanation that tries to cover everything ends up explaining nothing deeply and reads like marketing.

### Analogy That Misleads

An analogy that matches surface features but not structure sends the reader off in the wrong direction. Always test what the reader would correctly infer, and mark where the analogy breaks.

### Starting With Definitions

Opening with a glossary of terms before the reader has any reason to care makes the explanation feel like a textbook the reader did not ask for. Earn the definitions by establishing the problem first.

### Explaining Everything At Once

Breadth is the enemy of depth in explanation. A reader who is given fifteen concepts in one piece retains none. Choose one controlling insight and defer the rest to linked explanations.

### Presenting Simplification As Fact

When an explanation omits complexity without acknowledging it, readers build on a false foundation and are blindsided later. Honesty about simplification preserves trust.

### Ignoring The Reader's Misconceptions

If the explanation does not engage with what the reader likely already believes, including their wrong beliefs, it talks past them. The most useful explanations often correct a specific, common misunderstanding.

### No Connection To Action

Explanation that never links to how the concept is used leaves the reader understanding but unable to apply. Cross-link to tutorials and reference so understanding becomes capability.

## Self-Check

Before treating the explanation as complete, verify:

- The piece is governed by a single controlling insight that can be stated in one sentence.
- The explanation starts from the reader's likely current model, including common misconceptions.
- Every analogy preserves the structure it is meant to convey, and its limits are named.
- Concrete examples precede and anchor the abstractions.
- The explanation answers why the system behaves as it does, including the tradeoffs involved.
- The concept is connected to things the reader already knows, with consistent vocabulary.
- Simplifications and opinions are honestly marked, not presented as authoritative fact; diagrams, if used, illustrate the controlling insight and are traced with concrete examples
- Breadth has been sacrificed for depth where the two conflict; the piece is clearly separated from tutorial and reference, with links to each
- A reader who finishes would reason about the system better than before, not just know more facts about it; nothing in the explanation contradicts the system's actual behavior
- The piece does not lapse into a feature tour or marketing tone; the ending leaves the reader with the intended mental model, not a list of takeaways
