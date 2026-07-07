---
name: recommendation_and_suggestion_design.md
description: Use when the agent is designing recommendation systems, suggested content, product recommendations, autocomplete, smart suggestions, personalized feeds, "for you" surfaces, or reviewing how the product proposes options, content, or actions to users based on inference.
---

# Recommendation And Suggestion Design

Recommendations look like a simple feature: show the user something they might like. In practice, every recommendation is a claim about the user's intent, taste, and context, and those claims are often wrong. A recommendation system shapes what users see, what they buy, what they believe, and how they spend their attention, often more than the user realizes. Designed well, suggestions save time and surface value the user would have missed. Designed carelessly, they narrow exposure, erode agency, manipulate behavior, and train users to ignore or distrust every suggestion.

Use this skill before designing or reviewing recommendation carousels, "for you" feeds, suggested products or content, autocomplete and smart suggestions, personalized rankings, upsell and cross-sell surfaces, or any system that proposes options to users based on inference. The goal is to prevent the agent from treating recommendations as a ranking problem alone and ignoring diversity, agency, transparency, manipulation risk, and the long-term effect of narrowing what users encounter.

## Core Rules

### Match The Suggestion To The User's Actual Intent

A recommendation that ignores the user's current goal feels irrelevant at best and intrusive at worst. The same user wants different suggestions when browsing, searching, comparing, buying, or killing time.

Before designing suggestions, define:

- what intent the recommendation serves: discovery, completion, comparison, reminder, or persuasion;
- whether the user is in a goal-directed or exploratory mode;
- whether the suggestion helps the user's goal or the product's goal, and whether those align;
- how the suggestion fits the current screen and task without interrupting.

Suggestions that serve the product's conversion goal while ignoring the user's intent feel manipulative and reduce trust even when they are clicked.

### Balance Relevance, Diversity, And Serendipity

A naive recommendation engine optimizes for predicted engagement and produces a narrowing feed: more of the same, forever. That feels relevant short-term and stale, predictable, and manipulative long-term.

Balance three properties:

- relevance: the suggestion matches the user's evident interest;
- diversity: the set of suggestions is not all the same type, source, or viewpoint;
- serendipity: some suggestions introduce the user to something new rather than confirming existing patterns.

A feed that only confirms what the user already likes becomes an echo chamber. A feed that is too random feels useless. The right balance depends on the product, but pure engagement optimization reliably produces the wrong one.

### Make Suggestions Distinguishable From Organic Content

Users have a right to know what is recommended versus what is editorially chosen, chronologically ordered, or organically relevant. Blurring the line manipulates the user's assessment of what they are seeing.

- label recommendations clearly, such as "suggested," "recommended for you," or "sponsored" where applicable;
- separate recommendation surfaces from organic ones where the distinction matters;
- avoid disguising ads or paid placements as organic recommendations;
- keep the visual treatment honest about why something is being shown.

Hidden recommendations that look organic exploit the user's trust in the surrounding content.

### Give Users Control Over What Drives Suggestions

Recommendations inferred from behavior can feel invasive, especially when they reveal that the product is tracking more than the user realized. Users should understand and influence the inputs.

Provide:

- visibility into what signals drive recommendations, in plain terms;
- the ability to remove or downweight specific signals, such as a past purchase or search;
- the ability to dismiss, "not interested," or correct a suggestion;
- controls to reset or adjust personalization;
- an option to see non-personalized or general recommendations.

A user who cannot shape what drives their feed has no agency over a system that increasingly shapes their choices.

### Avoid Manipulative Persuasion Patterns

Recommendations are powerful, and that power can be misused. Patterns that exploit cognitive biases to drive engagement or purchase erode trust and often cross ethical lines.

Avoid:

- fake social proof, such as inflated counts or fabricated testimonials;
- manufactured scarcity or urgency on suggested items;
- dark patterns that make rejecting or dismissing a suggestion difficult;
- targeting vulnerable states, such as grief, anxiety, or financial stress;
- recommendations designed to maximize time-on-product at the user's expense.

Ethical recommendation design asks whether the user would consent to being shown this suggestion if they understood the reasoning.

### Handle Cold Start, Sparse Data, And New Users Honestly

Recommendation systems struggle when they know little about the user. Pretending to have personalized insight when the data is thin produces poor suggestions and erodes trust.

For cold-start situations:

- use sensible defaults or popular, high-quality options rather than random or empty;
- be honest that suggestions are general rather than personalized early on;
- gather preference signals through explicit, low-friction choices rather than silent inference;
- improve visibly as the user engages, without overclaiming sudden personalization.

A "for you" feed that is obviously generic on day one teaches the user that the personalization claim is not credible.

### Design For Correction And Feedback Loops

Recommendations will be wrong. The interface must let users correct mistakes and must actually use that feedback.

- let users indicate "not interested," "stop suggesting," or "I already have this";
- apply corrections promptly and visibly, so the user sees the effect;
- avoid re-suggesting dismissed items, or explain why they reappear;
- use feedback to improve future suggestions rather than ignoring it;
- handle the case where the user's preferences change over time.

A recommendation system that ignores user corrections trains users to stop giving feedback, which degrades the system further.

### Consider The Effect On Exposure And Fairness

Recommendations shape what users see, which shapes what creators, sellers, and viewpoints get exposure. A system optimized only for engagement can systematically disadvantage new, niche, or minority-serving content.

Consider:

- whether the system gives new or less-popular items a chance to be seen;
- whether it amplifies extreme, sensational, or harmful content because it engages;
- whether it creates feedback loops that concentrate exposure on a few winners;
- whether it is fair across creators, sellers, and viewpoints that the product claims to support.

Exposure effects are often invisible in short-term metrics and significant over time.

## Common Traps

### Pure Engagement Optimization

Optimizing only for clicks, watch time, or purchase produces a narrowing, sensational, or manipulative feed that feels relevant and degrades trust and wellbeing.

### Disguised Ads And Paid Placement

Blurring recommendations with ads or paid placements exploits the user's trust in organic content and is often a regulatory risk.

### Echo Chambers And Filter Bubbles

Showing only what confirms existing preferences narrows exposure and can polarize, misinform, or bore the user over time.

### Invasive Personalization Without Control

Recommendations that reveal surprising tracking, with no way to understand or adjust the inputs, feel creepy and reduce trust.

### Ignored Feedback

A "not interested" button that does not change future suggestions teaches users that feedback is pointless.

### Cold Start Pretending To Be Personalized

Generic suggestions labeled "for you" on day one undermine the credibility of personalization when it actually improves.

### Manipulative Urgency And Social Proof

Fake scarcity, inflated counts, or pressure tactics on suggested items exploit bias and erode trust when noticed.

### Exposure Concentration

Systems that funnel all attention to a few popular items starve new, niche, or diverse content and weaken the ecosystem.

## Self-Check

- [ ] Each recommendation matches the user's current intent and serves their goal, not only the product's conversion metric.
- [ ] The suggestion set balances relevance with diversity and serendipity, avoiding a pure echo chamber.
- [ ] Recommendations are clearly distinguishable from organic content, with honest labeling of sponsored or paid placements.
- [ ] Users can see, in plain terms, what signals drive recommendations and can remove, downweight, or reset them.
- [ ] Users can dismiss, correct, or stop suggestions, and that feedback visibly affects future recommendations.
- [ ] No manipulative patterns, such as fake scarcity, inflated social proof, or dark dismissal patterns, are used.
- [ ] Cold-start and sparse-data situations use honest defaults rather than pretending to be personalized.
- [ ] The system gives new, niche, and diverse items a fair chance at exposure rather than concentrating on a few winners.
- [ ] Recommendations avoid amplifying harmful, extreme, or sensational content purely because it engages.
- [ ] The user would consent to being shown these suggestions if they understood the reasoning behind them.
