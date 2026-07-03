---
name: trip_brief_and_traveler_fit.md
description: Use when the agent is gathering or interpreting a travel brief, clarifying traveler goals, matching a trip plan to preferences, constraints, budget, pace, group composition, special needs, or turning a vague travel request into a usable planning frame.
---

# Trip Brief And Traveler Fit

Travel planning fails when the agent treats a destination or itinerary as the main problem before understanding who is traveling, why they are going, what constraints are real, and what tradeoffs they will accept. A technically impressive plan can still be wrong if it assumes the wrong pace, budget comfort, food expectations, mobility level, risk tolerance, or group dynamic.

Use this skill before recommending destinations, building an itinerary, comparing travel options, or answering broad requests such as "plan a trip", "where should we go", "make this itinerary better", or "what should I consider". The goal is to convert a vague request into a traveler-fit brief without over-questioning the user or inventing preferences that should remain uncertain.

## Core Rules

### Separate Trip Purpose From Destination Preference

Start by identifying the reason for the trip, not only the place. A traveler asking for Paris may want museums, romance, food, shopping, family memories, a first international trip, or a low-effort city break. Each purpose changes lodging area, daily structure, restaurant choices, reservation needs, and acceptable tradeoffs.

Clarify whether the trip is primarily for:

- rest and recovery;
- sightseeing and landmarks;
- food, nightlife, or cultural experiences;
- nature, hiking, beaches, or wildlife;
- family bonding;
- romance or celebration;
- work plus leisure;
- education, pilgrimage, or heritage;
- adventure and novelty;
- efficient use of limited vacation time.

If the destination is fixed, optimize around the purpose. If the destination is open, use the purpose to narrow options before discussing specific places.

### Capture Hard Constraints Before Optimizing

Some constraints are non-negotiable and should shape every recommendation. Others are preferences that can be traded off. Separate them early.

Hard constraints may include:

- travel dates or school/work holiday windows;
- total budget or maximum per-night lodging cost;
- passport validity, visa, entry eligibility, or documentation limits;
- medical, dietary, disability, pregnancy, age, or mobility needs;
- flight length tolerance;
- climate or seasonal limits;
- safety or political risk tolerance;
- religious, cultural, or ethical restrictions;
- work connectivity needs;
- must-attend events or fixed reservations.

Do not bury these constraints in a generic plan. A single hard constraint can make an otherwise strong destination or route unworkable.

### Understand Traveler Composition And Decision Rights

The best plan for a solo traveler differs from a couple, family, friend group, multigenerational party, or business delegation. Group composition affects pace, lodging layout, transport, meal timing, rest needs, conflict risk, and booking flexibility.

Clarify:

- number of travelers and ages;
- who controls budget and final decisions;
- whether travelers share interests or need split options;
- whether anyone needs accessible rooms, elevators, car seats, quiet downtime, medical refrigeration, or nearby bathrooms;
- whether the group prefers independence or guided structure;
- whether there are first-time travelers, anxious travelers, or experienced travelers;
- how conflict should be handled when interests differ.

A good plan often includes shared anchor activities plus optional branches, rather than forcing every traveler into every activity.

### Define Pace And Energy Budget

Agents tend to pack itineraries with attractive items because each item looks useful in isolation. Travelers experience the combined load: walking distance, transit transfers, jet lag, queues, weather, decision fatigue, meals, and recovery time.

Estimate pace using concrete signals:

- desired start time and bedtime;
- tolerance for one-night stays;
- walking ability and daily step comfort;
- comfort with public transit, taxis, driving, ferries, or small planes;
- preference for slow mornings or full days;
- number of major activities per day;
- need for naps, pool time, remote work, childcare, or medical routines;
- appetite for spontaneous exploration versus planned reservations.

Use "pace" as a design constraint, not a style label. A luxury trip can be fast. A budget trip can be slow. A family trip can include ambitious days if rest is designed around them.

### Translate Budget Into Experience Tradeoffs

Budget is not only a number. Travelers care differently about lodging location, direct flights, food, guides, private transfers, views, flexible cancellation, and comfort. Two trips with the same total budget can feel very different depending on what is protected.

Ask or infer which categories deserve priority:

- shorter travel time;
- central lodging;
- private rooms or apartment-style space;
- memorable meals;
- tours, tickets, and guides;
- beaches, spas, or resort amenities;
- baggage allowance and seat selection;
- flexible cancellation;
- insurance and contingency funds;
- special occasions.

When budget is tight, expose the tradeoff. Do not quietly downgrade safety, unrealistic transit, or basic comfort to preserve a long activity list.

### Make Uncertainty Explicit And Useful

The agent often lacks complete details. Instead of stalling, state assumptions and identify which missing answers would change the plan most.

Useful assumptions include:

- "Assuming passports and entry documents are already valid";
- "Assuming a moderate pace with one anchor activity per day";
- "Assuming comfort with public transit";
- "Assuming budget excludes international flights";
- "Assuming no mobility or dietary restrictions."

If an assumption is high-impact, flag it before committing. Low-impact unknowns can be handled with flexible options.

### Preserve Traveler Agency

Travel planning is personal. Avoid overconfidently deciding taste, risk, or comfort for the user. Offer a small number of meaningful choices when preferences are unclear, but do not create a menu so large that the user must do the planning work again.

Good choices distinguish real tradeoffs:

- "more cities with more transfers" versus "fewer bases with day trips";
- "central but smaller lodging" versus "larger lodging farther out";
- "guided highlights" versus "self-directed wandering";
- "lower cost with early trains" versus "higher cost with direct flights."

Weak choices are cosmetic or exhaustive lists without prioritization.

## Common Traps

### Planning From The Agent's Travel Style

An agent may default to efficient sightseeing, but the traveler may want rest, food, beaches, luxury, nightlife, family logistics, or low stimulation. Fit the traveler, not the plan-maker.

### Treating Budget As Fully Flexible

Plans often drift upward because each upgrade seems reasonable. If budget is a constraint, preserve it category by category and name where the plan is making compromises.

### Ignoring The Least Flexible Traveler

One traveler with mobility, anxiety, childcare, medication, work, or dietary needs can determine the feasibility of the whole plan. Designing for the average traveler may fail the group.

### Asking Too Many Questions Before Helping

Clarification matters, but a user may need momentum. Ask only for details that materially change the plan, or proceed with explicit assumptions and invite correction.

### Confusing Desire With Feasibility

A traveler may want many places, low cost, comfort, and minimal transit. The role of the planner is to make the tradeoffs visible, not pretend all goals can be maximized.

### Forgetting Emotional Stakes

Honeymoons, family reunions, memorial trips, once-in-a-lifetime vacations, and first international trips carry emotional risk. Reliability, comfort, and clear expectations may matter more than novelty.

## Self-Check

- [ ] The trip purpose is clear and not reduced to only the destination name.
- [ ] Hard constraints are separated from preferences and reflected in the recommendations.
- [ ] Traveler composition, decision rights, ages, mobility, dietary, medical, and comfort needs were considered.
- [ ] Pace is described in practical terms such as start times, transfers, walking load, rest, and major activities per day.
- [ ] Budget tradeoffs are explicit across transport, lodging, food, activities, flexibility, and contingency.
- [ ] Assumptions are stated where missing information could change the plan.
- [ ] Group plans include ways to handle divergent interests or optional split activities.
- [ ] Recommendations avoid projecting the agent's own travel style onto the traveler.
- [ ] The plan protects the least flexible traveler rather than optimizing for an average traveler.
- [ ] The output gives useful direction without forcing the user to answer an excessive questionnaire first.
