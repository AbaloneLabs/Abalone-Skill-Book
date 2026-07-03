---
name: usability_test_planning.md
description: Use when the agent is planning a usability test, writing test tasks, deciding what to test, recruiting participants, or determining how many sessions are needed and what questions to ask without leading the participant.
---

# Usability Test Planning

Usability testing is one of the highest-value, most consistently misapplied research methods. The failure modes are predictable. Teams test too late, when the design is fixed and findings cannot be acted on. They test the wrong participants, who do not represent the real users, so the findings do not transfer. They write leading tasks that tell participants what to do and therefore reveal nothing about whether the interface guides them. Or they run a single session, declare the design validated, and ship. Each of these wastes the effort and produces false confidence.

Good usability test planning starts long before the sessions. It begins with deciding what question the test must answer, choosing the right method and stage, writing tasks that reveal rather than instruct, recruiting participants who match the real user, and deciding in advance how findings will be used. A product manager who plans usability testing well gets actionable signal; one who plans it poorly gets noise that is worse than no data, because it feels like evidence.

Use this skill before commissioning or running a usability test, before writing test tasks, before deciding how many participants to recruit, or when interpreting whether a usability finding is trustworthy. Ask: what specific question is this test trying to answer? Are the participants representative of the users who will actually encounter this design? Do the tasks let participants reveal their understanding, or do they instruct the answer? Is the test happening at a stage where findings can still change the design?

## Core Rules

### Start From A Specific Research Question

A usability test without a sharp question becomes a general exploration that produces scattered observations and no clear conclusion. Before planning anything else, define what you need to learn. Is it whether users can complete a specific task? Whether they understand a new concept? Where they get stuck in a flow? Whether a revised design fixes a known problem? Whether the design works for a new segment?

Frame the question precisely enough that the test can answer it. "Is the onboarding usable" is too broad; "can first-time users complete account setup without help within five minutes" is answerable. The research question determines the method, the tasks, the participants, and the sample size, so getting it wrong corrupts everything downstream. If you cannot state the question in one sentence, you are not ready to plan the test.

### Test At The Right Stage With The Right Fidelity

Usability testing is valuable at multiple stages, but the fidelity of the prototype must match the question. Early concept testing can use rough sketches or wireframes to test whether the approach makes sense, because the question is about direction, not detail. Mid-flow testing needs clickable prototypes to reveal where users stumble in a sequence. Pre-launch testing needs high-fidelity, realistic builds to catch issues that only appear with real data and real interactions.

Testing high-fidelity prototypes too early wastes effort on polish that may be discarded. Testing low-fidelity sketches too late misses interaction and data issues that only surface with realism. Match fidelity to the question and the stage, and do not let the availability of a polished prototype dictate the timing. The most common timing error is testing only at the end, when the design is committed and findings arrive too late to matter.

### Write Tasks That Reveal, Not Instruct

Task writing is the single most consequential skill in usability testing, and the most common point of failure. A leading task tells participants what to do and thereby confirms whatever the tester expected, teaching nothing. A revealing task describes a goal and lets participants show how they would get there, exposing whether the interface supports them.

The discipline is to describe the desired outcome and the motivation, not the steps. Instead of "click the settings icon and turn on notifications", use "you want to be notified when someone replies to your posts; show me how you would set that up". Instead of "use the search to find the report", use "your manager asked for the Q3 revenue report; how would you find it". Avoid terms that appear in the interface, because using them hands participants the answer. After writing each task, check whether a participant who read only the task could guess the interface labels; if so, rewrite it.

### Recruit Participants Who Represent The Real Users

Findings only transfer to the real product if the participants resemble the real users. Testing with colleagues, with the designer's friends, or with a convenience sample of whoever is available produces findings that may not hold. The relevant dimensions depend on the product: prior experience with the domain, technical confidence, device, language, accessibility needs, and the specific context of use.

Match recruitment to the research question. If the question is about first-time users, recruit people who have never used the product. If it is about a power feature, recruit experienced users. If it is about a new market segment, recruit from that segment. Be honest about which dimensions matter and recruit for them, rather than defaulting to whoever is easy to find. A test with the wrong participants generates findings you cannot trust and may act on wrongly.

### Decide Sample Size Based On The Goal

The question of how many participants to test is often answered with a number inherited from folklore, usually five. The real answer depends on what you are trying to learn and how you will use the findings. For identifying major usability problems in a flow, a small number of participants is often enough, because severe issues surface quickly and repeatedly. For estimating the prevalence of a problem, for comparing two designs, or for making a confident ship decision, more participants are needed.

Distinguish between problem discovery, where a handful of sessions reveals the major issues, and measurement, where you need enough data to trust a number. Be explicit about which you are doing. A test designed for discovery cannot support a claim that "eighty percent of users succeed", and a test designed for measurement should not be treated as exhaustive problem discovery. Matching the sample to the goal prevents overclaiming from small samples and underclaiming from large ones.

### Decide In Advance How Findings Will Be Used

A usability test produces observations, and observations become decisions only if there is a process to convert them. Before running the test, agree on how findings will be synthesized and acted on. Who will watch the sessions? How will issues be prioritized? What is the threshold for blocking a release versus deferring a fix? Who decides?

Without this, the test produces a report that sits unread, or a list of issues that gets debated endlessly. With it, the test feeds directly into design and release decisions. Plan the synthesis as deliberately as the sessions: schedule a debrief, assign ownership for acting on findings, and define what a blocking issue looks like. A test whose findings have a clear path to action is worth far more than a polished test whose report goes nowhere.

### Separate Usability Problems From Preferences

Participants will offer opinions, suggestions, and preferences during a test, and these are easy to mistake for findings. A participant saying "I would prefer a darker theme" is not evidence of a usability problem; it is a preference. A participant struggling to find the submit button, hesitating, and asking for help is evidence of a usability problem. The distinction matters because preferences should not drive design changes the way problems should.

Record observations of behavior separately from expressions of preference. Behavior, what participants do and where they struggle, is the primary signal in usability testing. Stated preference is secondary and should be weighted accordingly. Conflating the two leads to design changes that chase opinions rather than fix real friction.

## Common Traps

### Testing Only At The End

Testing when the design is committed means findings arrive too late to change anything. The trap is treating usability testing as a release gate rather than a design input.

### Leading Tasks That Confirm Expectations

Tasks that use interface terms or describe steps tell participants the answer. The trap is that the test appears to validate the design while revealing nothing.

### Wrong Participants

Testing with people who do not match the real users produces non-transferable findings. The trap is treating convenience recruitment as good enough and then generalizing the results.

### Overclaiming From Small Samples

A few sessions can reveal major problems but cannot support quantitative claims. The trap is reporting "most users succeeded" from a sample too small to support it.

### Chasing Preferences Instead Of Fixing Problems

Acting on stated preferences as if they were usability findings produces design churn without reducing friction. The trap is that preferences are easier to collect than behavioral observations.

### No Path From Findings To Action

A test whose report goes unread teaches the team that research does not matter. The trap is investing in sessions while neglecting synthesis and decision-making.

## Self-Check

- [ ] The test is anchored to a specific, answerable research question, not a vague exploration.
- [ ] The prototype fidelity and test stage match the question, and the test is early enough that findings can change the design.
- [ ] Tasks describe goals and motivations, not steps, and avoid interface terms that would hand participants the answer.
- [ ] Participants were recruited to match the real users on the dimensions that matter for the question.
- [ ] Sample size was chosen based on whether the goal is discovery or measurement, not inherited as a fixed number.
- [ ] A synthesis and decision process was planned before the sessions, with clear ownership and blocking criteria.
- [ ] Behavioral observations were separated from stated preferences, and design changes target problems, not opinions.
- [ ] Severe issues were distinguished from minor ones, and fixes were prioritized by impact on the user goal.
- [ ] The findings were actually used to inform design or release decisions, not filed and forgotten.
- [ ] The limitations of the test, including sample size and participant match, were acknowledged when interpreting results.
