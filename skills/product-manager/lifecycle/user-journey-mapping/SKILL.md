---
name: user-journey-mapping.md
description: Use when the agent is mapping user journeys, documenting the end-to-end experience across touchpoints, identifying journey pain points and emotional low points, deciding the granularity of a journey map, or using journey maps to find intervention opportunities and align teams around the actual user experience.
---

# User Journey Mapping

A user journey map is a tool for making the actual experience of using a product visible and discussable. Its value is not the artifact itself but the shared understanding it creates: when a team can see the whole journey a user travels, including the parts outside the product's UI, they spot problems and opportunities that are invisible from within any single feature team's view. A good journey map changes how the team thinks. A bad one — built from assumptions, focused only on the happy path, produced once and pinned to a wall — is ceremony that consumes effort and changes nothing. The difference is in how the map is built and how it is used.

This skill covers the judgment needed to build journey maps that genuinely inform decisions: what to include, what to base them on, and how to use them to drive action rather than decoration.

## Core Rules

### Build journey maps from real user evidence, not from internal assumptions

The most common journey mapping failure is building the map from what the team believes users do, rather than from what users actually do. An assumption-based map confirms the team's existing mental model and adds no new understanding; it is a sophisticated form of confirmation bias. A map built from evidence reveals where the team's model is wrong, which is where the value is.

- Base the map on real user data: behavioral analytics showing actual paths, interviews revealing the experience and emotions, support data revealing friction, and observation of real users completing real tasks.
- Where the team's assumptions and the evidence conflict, trust the evidence and investigate the conflict; the conflict is usually a finding worth acting on.
- Acknowledge uncertainty where evidence is thin, rather than filling gaps with plausible-sounding assumptions. A map that honestly marks "we do not know what happens here" is more useful than one that confidently invents.

The map is only as trustworthy as its evidence base. An assumption-based map lends false confidence to decisions made on top of it.

### Include the full journey, including parts outside the product UI

Users' experiences do not begin at the login screen and end at the logout. They begin before the product is involved (the problem or need that leads to it), include touchpoints outside the product (search, evaluation, support, communication, integration with other tools), and continue after use (sharing, recommending, returning). A map that covers only the in-product flow misses the parts of the journey where value is won or lost.

- Map the journey from the trigger (what causes the user to seek the product) through to the outcome (what they ultimately achieve or fail to achieve).
- Include off-product touchpoints: how users discover, evaluate, get help, integrate, and share. These are often where the biggest friction and the biggest opportunities live.
- Include the emotional arc: how the user feels at each stage. Emotional low points are often more diagnostic than functional friction, because they reveal where trust or motivation is lost.

The narrowest maps are the easiest to build and the least useful. The maps that change decisions include the messy reality outside the product.

### Map multiple journeys for distinct segments and use cases, not a single average journey

A single journey map for all users describes an average user who does not exist, and it hides the very real differences between segments and use cases. A journey that is smooth for one segment is full of friction for another, and a single map cannot show this.

- Map distinct journeys for distinct segments (by role, by use case, by experience level) where the journeys meaningfully differ.
- Identify the segments whose journeys matter most: high-value segments, high-growth segments, and segments whose journeys are known to be problematic.
- Avoid mapping so many journeys that the effort exceeds the insight. Prioritize the journeys that will inform the most important decisions.

The right number of maps is enough to capture the meaningful variation without drowning in granularity. Usually this is a small number of strategically chosen journeys.

### Capture both the ideal journey and the actual journey, and study the gaps

There are two journeys that matter: the ideal journey (what the team intends and what the user hopes for) and the actual journey (what really happens, friction and all). Mapping only the ideal produces a document that describes aspirations; mapping only the actual can miss what the experience could be. Mapping both, and studying the gaps, is where the actionable insight lives.

- Map the actual journey from evidence, including the friction, the dead ends, the emotional low points, and the workarounds.
- Map the ideal journey based on the user's goal and the best possible experience, informed by what users say they wanted.
- The gaps between the two are the intervention opportunities: each gap is a place where the experience falls short of what would serve the user, and addressing the largest gaps is the highest-leverage work.

### Use the map to find intervention opportunities, not just to document

A journey map that is produced, presented, and filed has consumed effort and produced no change. The value of the map is in the decisions and interventions it drives. Build the map with the intervention question in mind: what would we do differently because of this?

- For each friction point and emotional low point in the map, identify the possible intervention: a product change, a communication, a process change, a self-service resource.
- Prioritize interventions by the size of the gap and the number of users affected, using the map to make the prioritization visible and discussable.
- Assign owners and track the interventions to completion, so the map drives action rather than decorating a wall.

### Keep the map living, not frozen

A journey map is a snapshot of the experience at a point in time, and the experience changes as the product, the users, and the context evolve. A map frozen at one moment becomes stale and misleading, used to justify decisions based on conditions that no longer hold. Keep the map living.

- Revisit and update the map when the product changes substantially, when new evidence emerges, or on a regular cadence.
- Treat the map as a hypothesis that is tested and refined, not a fixed truth. When real user behavior contradicts the map, update the map.
- Version the map so that changes are visible and the team can see how understanding has evolved.

### Use the map to create shared understanding across siloed teams

One of the highest values of a journey map is that it gives teams that own different parts of the experience a shared view of the whole. The team that owns onboarding, the team that owns the core feature, the team that owns support, and the team that owns billing each see only their segment. The map shows them how their segments connect and where the handoffs fail.

- Use the map in cross-functional reviews to make the whole journey visible to every team that touches it.
- Identify the handoff points between teams, where users often fall through gaps because no team owns the transition.
- Build shared accountability for the journey outcomes, not just for each team's segment metrics.

## Common Traps

### Assumption-based maps that confirm the existing mental model

A map built from what the team believes adds no understanding and lends false confidence. Build from real user evidence and investigate where assumptions and evidence conflict.

### Maps covering only the in-product flow

The journey outside the product UI — discovery, evaluation, support, integration — is often where the biggest friction and opportunity live. Map the full journey, including off-product touchpoints.

### A single average journey for all users

An average journey describes no real user and hides segment differences. Map distinct journeys for segments and use cases that meaningfully differ.

### Documenting without driving intervention

A map that is produced and filed changes nothing. Build the map to find and prioritize interventions, and track them to completion.

### Mapping only the ideal or only the actual

The ideal alone describes aspirations; the actual alone misses the potential. Map both and study the gaps, which are the intervention opportunities.

### Frozen maps that become stale

A map frozen at one moment misleads as the experience evolves. Keep the map living, updated as the product and evidence change.

### Granularity that exceeds the insight

Mapping too many journeys or too many steps produces effort that exceeds the decisions it informs. Prioritize the journeys and the level of detail that will inform the most important decisions.

### The map owned by one team, invisible to others

A map that lives with the research team and is not used by the teams that own the experience creates no shared understanding. Use the map cross-functionally to build accountability for the whole journey.

## Self-Check

- Is my journey map built from real user evidence (analytics, interviews, observation, support data), or from internal assumptions?
- Does the map cover the full journey from trigger to outcome, including off-product touchpoints and the emotional arc?
- Have I mapped distinct journeys for segments and use cases that meaningfully differ, rather than a single average journey?
- Did I map both the ideal and the actual journey, and am I using the gaps between them to find intervention opportunities?
- For each friction point and emotional low point, have I identified and prioritized an intervention, with an owner and a plan?
- Is the map living — updated as the product and evidence change — or frozen at a moment that has passed?
- Is the level of detail calibrated to the decisions the map needs to inform, without granularity that exceeds the insight?
- Is the map used cross-functionally to build shared understanding of handoffs and gaps between teams?
- Where the map and real user behavior conflict, do I update the map, or do I defend the map?
- If I showed this map to a user it claims to describe, would they recognize their experience, or would they describe something different?
