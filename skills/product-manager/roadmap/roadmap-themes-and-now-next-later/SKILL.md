---
name: roadmap_themes_and_now_next_later.md
description: Use when the agent is structuring a roadmap by themes instead of features, building now-next-later or horizon-based roadmaps, communicating roadmap uncertainty, moving away from date-bound Gantt roadmaps, or deciding how much detail to commit to in each timeframe.
---

# Roadmap Themes And Now-Next-Later

A feature-by-feature, date-by-date roadmap is a contract the team cannot keep and a fiction the stakeholders will punish it for breaking. The more specific and time-bound a roadmap becomes, the more it converts every estimate into a promise and every change into a failure. Thematic and now-next-later roadmaps solve a real communication problem: how to show direction and near-term commitment without pretending to knowledge of the distant future that no one has. The judgment problem is choosing the right structure for the audience, defining themes that are meaningful rather than vacuous, and deciding how much certainty to project into each horizon without either overcommitting or appearing evasive.

Use this skill before building or revising a roadmap, before presenting direction to executives or customers, before deciding whether to commit to dates, and before translating strategy into a visual roadmap format. The goal is to prevent the agent from producing a roadmap that is either a brittle Gantt chart doomed to miss or a vague cloud of themes that communicates nothing.

## Core Rules

### Choose The Structure For The Audience And Horizon

Different audiences need different roadmap structures, and the same product may need several views. Match structure to need.

Common structures and their fit:

- now-next-later: communicates commitment gradient without false dates, good for broad audiences;
- theme-based: groups work under strategic outcomes, good for leadership and narrative;
- horizon-based (H1/H2/H3): separates committed, probable, and exploratory work;
- outcome-based: shows target outcomes per period rather than features;
- date-bound milestone: appropriate only when dates are real and externally fixed.

A single roadmap rarely serves the engineering team, the executive team, and the customer equally. Plan for multiple views from one source of truth.

### Define Themes As Outcomes, Not Buckets

A theme like "platform reliability" is a label; a theme like "reduce checkout failure rate and eliminate the top three causes of support tickets" is a direction. Vague themes become dumping grounds where unrelated work hides behind a strategic-sounding name.

Strong themes:

- connect to a named outcome or problem area;
- reference a strategic bet or customer job;
- are specific enough to reject work that does not belong;
- span multiple features that would otherwise seem disconnected;
- are stable across quarters, providing continuity.

If any feature can be placed under any theme, the themes are not doing work.

### Commit With Descending Confidence Across Horizons

The defining insight of now-next-later is that commitment should decrease as the horizon extends. Now is committed and resourced; next is planned and probable; later is directional and exploratory.

Calibrate each horizon:

- now: in progress or starting imminently, resourced, scoped, with high confidence;
- next: next in line, problem defined, solution likely but not locked, medium confidence;
- later: directionally intended, problem or solution may shift, low confidence, no commitment.

The error is treating all three horizons as equally committed, which collapses the structure back into a fixed plan.

### Avoid False Dates In The Distant Horizon

The further out a date is, the less anyone can know it. Putting a date on a Q3 item in January is mostly theater, yet stakeholders will hold the team to it. Reserve specific dates for the now horizon and use ranges or sequencing for later horizons.

When dates are demanded:

- use ranges that reflect real uncertainty;
- separate committed dates from forecast dates visually;
- attach the assumptions the date depends on;
- define what would cause the date to move.

A roadmap that refuses to date the uncertain future is more honest, not less ambitious.

### Make Dependencies And Sequencing Visible

A flat list of themes hides the fact that some work must precede other work. Even a thematic roadmap should show enablement relationships so stakeholders understand why something is in later rather than now.

Show:

- which themes unlock other themes;
- which require platform or infrastructure foundation;
- which depend on external partners, data, or regulatory events;
- which can run in parallel without contention.

Sequencing logic, not just ordering, makes the roadmap defensible.

### Build A Review And Update Cadence

A roadmap is a living document. Without a cadence it either fossilizes or drifts. Establish when and how it is revised.

Define:

- the update frequency, monthly, quarterly, or per planning cycle;
- who has authority to move items between horizons;
- what triggers an out-of-cycle change;
- how changes are communicated and why;
- the archive of what moved and why, for trust and learning.

Stakeholders tolerate change when it is explained; they revolt when it is silent.

### Use The Roadmap To Drive Alignment Conversations

The roadmap's highest value is not as an artifact but as the conversation that produces and revises it. A roadmap built by one person and presented as final misses the alignment it is meant to create.

Use roadmap reviews to:

- surface conflicting priorities between teams;
- test whether themes still match strategy;
- reveal capacity and dependency tensions;
- align sales, marketing, and support on what is coming;
- create shared ownership of tradeoffs.

## Common Traps

### The Gantt In Disguise

Calling it now-next-later while internally treating every item as date-committed reproduces the brittleness the structure was meant to avoid.

### Vacuous Themes

Themes so broad that any feature fits provide no prioritization power and become catch-alls for unrelated work.

### Equal Confidence Across Horizons

Presenting later items with the same certainty as now items sets up inevitable disappointment and erodes credibility.

### Dating The Uncertain Future

Specific dates on distant items create promises the team cannot keep and stakeholders will enforce.

### Hiding The Why

Moving items without explaining the reason reads as chaos and destroys the trust the thematic structure was meant to build.

### One Roadmap For Everyone

A single view cannot serve engineering detail, executive strategy, and customer expectation simultaneously.

### Set-And-Forget

A roadmap written once and never revised becomes fiction within a quarter as reality changes.

## Self-Check

- [ ] The roadmap structure matches the audience and horizon, with multiple views from one source of truth where needed.
- [ ] Themes are defined as outcomes or problem areas specific enough to reject non-belonging work, not as broad buckets.
- [ ] Commitment decreases across horizons, with now resourced and scoped, next probable, and later directional.
- [ ] Specific dates are reserved for the now horizon, with ranges and assumptions for more distant work.
- [ ] Dependencies, sequencing, and enablement relationships between themes are visible.
- [ ] A review and update cadence is defined, with clear authority and change-communication rules.
- [ ] The roadmap was built through alignment conversations, not authored unilaterally and presented as final.
- [ ] Changes are explained with reasons rather than applied silently.
- [ ] The structure avoids collapsing back into a brittle date-bound Gantt chart.
- [ ] Later-horizon items are presented with appropriately low confidence.
