---
name: prototype_validation_method.md
description: Use when the agent is planning how to test a prototype, selecting who to test with, deciding what feedback to capture, or designing a prototype test session that produces trustworthy signal rather than polite approval.
---

# Prototype Validation Method

A prototype is only as useful as the test you run on it. The same prototype, tested well, reveals whether users understand a flow; tested poorly, returns polite approval that confirms whatever the team already believed. Method quality, not prototype quality, is what separates signal from flattery.

The judgment problem is that prototype tests are easy to run badly and hard to run well, and the bad version feels just as productive. Teams test with colleagues who are too informed to be representative, ask leading questions that telegraph the right answer, demo the prototype instead of letting users use it, and capture feedback as vague ratings rather than observable behavior. Each of these errors produces positive feedback that feels like validation but is actually noise. Agents tend to treat a few friendly sessions as evidence, to confuse what users say with what they do, and to design the session around proving the design rather than around discovering where it breaks. The harm is designs that ship with confidence they were never earned, and features that fail in the market because the test could only ever return approval.

Use this skill when planning a prototype test, when writing the test script, when selecting participants, and when deciding what to measure. The goal is to design a session that produces trustworthy signal, surfaces real confusion, and resists the team's desire to hear good news.

## Core Rules

### Recruit Participants Who Match The Real User

The validity of prototype feedback depends almost entirely on whether the participants resemble the people who will actually use the product. Testing with colleagues, friends, or the team itself produces feedback that is informed, polite, and unrepresentative, because insiders already understand the domain and the intent.

Define the target user for the question, then recruit participants who match on the attributes that matter: role, experience level, context of use, and motivation. Five to eight well-chosen participants surface most usability problems; dozens of the wrong participants surface nothing useful. Spend more effort on recruitment than on the prototype, because the right participants are the difference between signal and noise.

### Let Users Use It Before You Explain It

The most common session error is demonstrating the prototype and then asking what users think. Once you have shown how it works, you have answered the comprehension question you were trying to test, and all subsequent feedback is contaminated by your explanation. Users will confirm what they were just told.

Start the session by giving users a task and a goal, with no demonstration, and watch what they do. Their first unprompted actions reveal whether the design is comprehensible. Only after they have attempted the task should you ask questions, and even then, ask about their experience, not about whether they liked what you built. The demo is the enemy of the test.

### Ask About Behavior And Confusion, Not About Preference

People are poor at predicting what they would do and excellent at rationalizing preferences in the moment. Asking "do you like this" or "would you use this" invites speculation and politeness. Asking "what were you trying to do just then" or "where did you get stuck" captures observable reality.

Frame questions around the participant's actual experience in the session. "What did you expect to happen when you clicked that?" reveals mental models; "walk me through what you were thinking" surfaces confusion. Avoid leading questions that telegraph the intended answer, such as "was that easy?" Replace them with neutral prompts like "tell me about that." The goal is to discover where the design fails, not to confirm that it succeeds.

### Define What You Are Testing Before The Session

A prototype test should have explicit questions written before any session begins: which comprehension, which flow, which decision points, which moments of confusion you are looking for. Without pre-defined questions, the session becomes a general chat whose findings are selected afterward to support the desired conclusion.

Write two to four specific test questions, such as "do users find the export action without prompting" or "where do users hesitate in the checkout flow." Design the task to exercise those questions, and structure the notes around them. Pre-defined questions turn a subjective conversation into a focused investigation and prevent cherry-picking after the fact.

### Capture Observable Signal, Not Just Opinions

The strongest prototype evidence is what users do, not what they say. Record where they click, where they hesitate, where they backtrack, where they succeed without help, and where they need prompting. These behaviors are more reliable than any rating and more persuasive than any quote.

Take structured notes keyed to the test questions: success without help, success with prompting, failure, and the specific point of confusion. Where possible, record sessions or use an observer, because the moderator cannot both facilitate and capture faithfully. Behavioral signal, aggregated across participants, reveals patterns that individual opinions obscure.

### Resist Confirming The Design You Want To Validate

Prototype tests are biased toward positive results because the team wants the design to succeed and the moderator, often the designer, wants to hear that it works. This bias leaks into tone, phrasing, and the choice of which findings to report. The discipline is to design the session to disconfirm, not to confirm.

Actively look for failure. Treat confusion and errors as the most valuable findings, because they reveal where the design breaks. When a participant struggles, do not rush to help; let the struggle reveal the problem, then explore it. A session that returns only praise has probably been run to please, not to learn, and its conclusions should be treated with suspicion.

### Decide How Many Sessions And When To Stop

A handful of well-run sessions with the right participants surfaces the major usability problems; running dozens adds diminishing returns and delays action. Conversely, running one or two sessions and declaring victory captures too little signal to trust. Match the number of sessions to the stakes and the convergence of findings.

Stop when the same problems recur across participants and few new issues appear, which typically happens within five to eight sessions for a focused question. If findings diverge wildly, the prototype or the recruitment may be off, and it is better to investigate than to average away the disagreement. Report the pattern, including the disagreements, not just the headline.

## Common Traps

### Testing With Colleagues Or Friends

Insiders understand the domain and the intent, so their feedback is informed and polite rather than representative. The trap is a session that feels productive but cannot reveal how a real user would fare. Recruit participants who match the real user.

### Demonstrating Before Testing

Showing how the prototype works answers the comprehension question for the user, contaminating all later feedback. The trap is that the team believes the design is intuitive when users only repeated what they were shown. Let users attempt the task unprompted first.

### Asking Leading Or Preference Questions

"Was that easy?" and "would you use this?" telegraph the desired answer and invite speculation. The trap is collecting agreement that feels like validation but predicts nothing. Ask about behavior and confusion instead.

### No Pre-Defined Test Questions

Without written questions, the session becomes a chat whose findings are cherry-picked afterward. The trap is that the conclusion follows the team's preference rather than the data. Define what you are testing before any session.

### Capturing Opinions Instead Of Behavior

Ratings and quotes are easy to collect and easy to misread. The trap is reporting "users liked it" when the behavioral record shows they could not complete the task. Capture what users do, keyed to the test questions.

### Running The Session To Confirm, Not To Discover

When the moderator wants the design to succeed, the session returns praise. The trap is a test that can only ever approve, whose positive result is meaningless. Design sessions to surface failure.

### One Or Two Sessions Treated As Verdict

Too few sessions capture too little signal to trust, yet teams often declare victory after a friendly pair. The trap is shipping on anecdote. Run enough sessions to see convergence, and report disagreements honestly.

## Self-Check

- [ ] Participants match the real target user on role, experience, context, and motivation, and are not colleagues or friends.
- [ ] Users attempt the task unprompted before any explanation, so comprehension is actually tested.
- [ ] Questions ask about behavior and confusion, not preference, and avoid leading phrasing that telegraphs the intended answer.
- [ ] Two to four specific test questions were written before any session began, and notes are structured around them.
- [ ] Observable behavior, success points, hesitation, and errors are captured, not just opinions and ratings.
- [ ] The session was designed to surface failure and confusion, not to confirm the design the team wants to validate.
- [ ] The number of sessions matches the stakes, and stopping followed convergence of findings rather than a single friendly pair.
- [ ] The reported conclusion includes the pattern of behavior and any disagreements, not just a headline approval.
