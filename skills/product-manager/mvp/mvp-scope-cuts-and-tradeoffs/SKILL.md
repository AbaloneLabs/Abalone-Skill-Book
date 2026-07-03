---
name: mvp_scope_cuts_and_tradeoffs.md
description: Use when the agent is deciding what to cut from an MVP, weighing reversible versus committed scope, managing scope creep from stakeholders, or choosing what to defer without losing the core value proposition.
---

# MVP Scope Cuts And Tradeoffs

Once an MVP has a learning goal, the hard work is the cutting. Every feature, integration, and edge case is a candidate for removal, and the wrong cuts either hollow out the core value or leave the build too heavy to iterate. Cutting is a series of tradeoffs, not a single act of trimming.

The judgment problem is that cuts are negotiated under pressure from stakeholders who each see their own requirement as essential, and from engineers who see the clean full solution as obviously correct. In that environment, the cuts that survive are the politically easy ones, not the ones that serve the learning goal. Agents tend to cut the visible polish that users need and keep the invisible infrastructure that does not serve the question, or to defer the wrong things and discover too late that a deferred dependency was actually load-bearing. The harm is an MVP that ships late, tests a broken experience, or commits the team to an architecture that constrains every future iteration.

Use this skill when negotiating scope, when a stakeholder demands a feature, and when deciding what to defer versus what to build now. The goal is to make cuts deliberately, protect the core, and keep the MVP reversible and fast.

## Core Rules

### Cut Against The Learning Goal, Not Against Effort

The criterion for cutting is whether a feature serves the learning question, not whether it is easy or hard to build. A hard feature that is essential to the core value stays; an easy feature that does not serve the question goes. Sorting by effort produces a scope optimized for build convenience rather than for learning.

For each feature, ask what would happen to the learning signal if it were removed. If the answer is nothing, cut it. If the answer is that the experience breaks or the signal becomes untrustworthy, keep it. This test is stricter than it sounds, because stakeholders will argue that many non-essential features are "expected" by users. The learning goal is the tiebreaker.

### Distinguish Reversible From Committed Scope

Some cuts are about reversibility. A feature that can be added later without rework is reversible and safe to defer; a decision that locks in a data model, an API contract, or an architecture is committed and hard to undo. MVPs should maximize reversible scope and minimize committed scope, because committed scope is where premature decisions cause lasting damage.

Review each kept item and ask whether building it now commits the team to a path. If a feature forces a database schema, a permission model, or a public API that will be expensive to change, treat it as high-risk and consider whether a simpler, more reversible version answers the same question. The goal is to keep options open while still learning.

### Protect The Core Value Proposition Absolutely

The core value proposition is the one thing the MVP must deliver for the learning signal to be trustworthy, and it is the last thing to cut. Cutting into the core produces a false negative, where users reject a hollow experience and the team concludes the idea was bad when only the execution was.

Identify the core explicitly and treat it as non-negotiable scope. When pressure to cut arrives, cut around the core first: polish, settings, integrations, edge cases, scale, and admin tooling. If you cannot deliver the core at the current scope, the MVP is not feasible as specified, and you need a different form, such as a manual or concierge version, rather than a thinner build that breaks the value.

### Manage Scope Creep By Redirecting To The Roadmap

Scope creep is not always wrong; stakeholders raise real needs. The mistake is absorbing those needs into the MVP instead of capturing them elsewhere. Every vision feature that does not serve the learning question belongs in the roadmap or a backlog, not in this build.

When a stakeholder asks for a feature, acknowledge the need, then ask which learning question it serves. If it serves the vision, log it in the roadmap and explain that the MVP is a probe, not version one. This reframing turns a refusal into a deferral and preserves the relationship while protecting the scope. Document the deferred items so stakeholders see their needs are captured, not dismissed.

### Defer Dependencies Without Breaking Load-Bearing Ones

Cutting a feature sometimes removes a dependency that other kept features rely on, and discovering this late causes rework or a broken build. Before deferring, map which kept features depend on the deferred one, and confirm the kept scope still functions.

For each proposed cut, trace its dependencies forward. If the cut removes something the core value depends on, either keep that dependency or restructure the kept scope so it no longer needs it. This is especially important for shared infrastructure: authentication, data ingestion, and payment flows are often load-bearing even when they feel like background plumbing.

### Choose Build Quality To Match The MVP's Lifespan

The quality bar for an MVP should match how long it will live and how many users it serves. Over-engineering a throwaway probe wastes time; under-engineering a build that will become production creates debt that haunts every later iteration. The trap is applying a single quality standard to all MVPs.

Decide upfront whether the MVP is disposable, transitional, or destined to evolve, and set the quality bar accordingly. A disposable concierge MVP can use spreadsheets and manual steps. A transitional build should be clean enough to iterate but not polished for scale. Be explicit about the lifespan, because the lifespan determines which corners are acceptable and which are not.

### Make Cuts Visible And Justified

Every cut should be recorded with its reason, so that the team and stakeholders can see what was deferred and why. Invisible cuts breed resentment and lead to rediscovery, where someone re-requests a cut feature because no one remembers it was considered. A visible cut log turns scope decisions into a shared record.

Maintain a deferred-items list tied to the learning goal. For each item, note what question it would answer in a future build and what evidence from this MVP would justify revisiting it. This list becomes the input to the next iteration and prevents the team from relitigating the same cuts.

## Common Traps

### Cutting The Visible Polish Users Need

Polish and onboarding often feel expendable, but removing them can break comprehension and produce a false negative. The trap is shipping an MVP so rough that users cannot figure out the core value, then concluding the idea failed. Keep enough polish that users can experience the core.

### Keeping Invisible Infrastructure That Serves No Question

Teams often keep backend, scale, and admin work that does not serve the learning goal because it feels like good engineering. The trap is that this scope slows the build and commits architecture without advancing learning. Cut infrastructure that the question does not require.

### Deferring A Load-Bearing Dependency

Cutting a feature without checking what depends on it can break the kept scope. The trap is discovering the dependency only at integration time, causing rework or a broken demo. Trace dependencies before deferring.

### Absorbing Vision Features To Appease Stakeholders

Saying yes to a stakeholder's vision feature to avoid conflict pulls the MVP toward version one. The trap is that scope balloons and the build slows, defeating the purpose. Redirect vision features to the roadmap.

### Applying One Quality Bar To All MVPs

Treating every MVP as either throwaway or production-ready ignores the lifespan. The trap is either wasting effort on a probe or saddling a future product with prototype-quality code. Set the quality bar to the MVP's intended lifespan.

### Invisible Cuts That Get Relitigated

When cuts are not recorded, stakeholders re-request the same features and the team re-argues the same decisions. The trap is wasted energy and eroded trust. Keep a visible deferred-items list with reasons.

### Cutting By Political Ease Rather Than Learning Value

The cuts that survive negotiation are often the ones no one fought for, not the ones that serve the question. The trap is a scope shaped by politics rather than by the learning goal. Judge every cut against the question.

## Self-Check

- [ ] Every kept and cut feature was tested against the learning goal, not against effort or political ease.
- [ ] Reversible scope is maximized and committed scope, such as data models and public APIs, is minimized or simplified.
- [ ] The core value proposition is identified, protected, and not hollowed out by cuts.
- [ ] Vision features raised by stakeholders were redirected to the roadmap rather than absorbed into the MVP.
- [ ] Dependencies of each cut feature were traced, and no load-bearing dependency was removed from kept scope.
- [ ] The build quality bar was set to match the MVP's intended lifespan, not applied as a single blanket standard.
- [ ] A visible deferred-items list records each cut with its reason and the evidence that would justify revisiting it.
- [ ] The cuts would still be defensible if the learning result came back opposite to what the team hoped.
