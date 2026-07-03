---
name: throwaway_vs_evolutionary_prototyping.md
description: Use when the agent is deciding whether a prototype is disposable or destined to evolve into production code, managing tech debt from prototypes that ship, or setting expectations about what a prototype is and is not.
---

# Throwaway Versus Evolutionary Prototyping

Every prototype carries a hidden decision about its afterlife: will it be thrown away, or will it grow into the product? That decision is rarely made deliberately, and the default, especially under deadline pressure, is that the prototype becomes the product whether anyone intended it or not. The cost of that silent default is measured in years of tech debt.

The judgment problem is that throwaway and evolutionary prototypes are built for opposite purposes, and confusing them corrupts both. A throwaway prototype is built to learn fast, with shortcuts, fake data, and no concern for maintainability, because it will be discarded once the question is answered. An evolutionary prototype is built to become production, so it needs real architecture, real tests, and real attention to quality. When a team builds a throwaway prototype and then, under pressure to ship, promotes it to production, they inherit all the shortcuts as permanent debt. When they build an evolutionary prototype and treat it as disposable, they waste the production-quality effort and rebuild from scratch. Agents tend to leave the lifecycle unstated, assume the best case, and discover the mismatch only when the prototype is already load-bearing. The harm is products built on prototype foundations that were never meant to last, and teams trapped maintaining code that was supposed to be thrown away.

Use this skill before building a prototype, before deciding whether to ship prototype code, and before telling stakeholders what the prototype represents. The goal is to make the lifecycle decision deliberately, set expectations honestly, and prevent the silent promotion of throwaway work into production.

## Core Rules

### Decide The Lifecycle Before You Build

The single most important decision is whether the prototype is throwaway or evolutionary, and it must be made before construction, not after. The decision determines the build approach, the quality bar, the architecture, and what the team is allowed to shortcut. Deferring the decision guarantees a mismatch between how the prototype is built and how it will be used.

State the lifecycle explicitly and share it with the team and stakeholders. If throwaway, build for speed and learning, and plan the discard. If evolutionary, build for production from the start, accepting that this is slower. The unstated default is always evolutionary-by-accident, which is the worst of both worlds, so make the choice conscious.

### Build Throwaway Prototypes To Learn, Then Discard

A throwaway prototype exists to answer a question, and its value is the learning, not the artifact. Build it with whatever is fastest: hardcoded data, mocked services, no tests, no error handling, and tools chosen for speed rather than durability. The shortcuts are legitimate because the code will not survive.

The discipline is to actually throw it away. Once the question is answered, discard the code and rebuild the production version with proper architecture, informed by what the prototype taught. The learning transfers; the code does not. Keeping the throwaway code "to save time" imports every shortcut as permanent debt, and the time saved up front is paid back many times over in maintenance.

### Build Evolutionary Prototypes To Production Quality From The Start

An evolutionary prototype is destined to become the product, so it must be built as production from the first commit. Real architecture, real data models, real tests, real error handling, and real attention to performance and security. The cost is higher up front, but it avoids the rebuild that a throwaway-then-promoted prototype would require.

Be honest about the time this takes. Evolutionary prototyping is slower than throwaway prototyping, and pretending otherwise leads to cutting corners that defeat the purpose. If the schedule cannot support production-quality building, the prototype is not actually evolutionary; it is a throwaway pretending to be one, and the lifecycle decision was wrong.

### Resist The Pressure To Ship Throwaway Code

The most common and damaging failure is shipping throwaway prototype code to production because the deadline arrived. Under pressure, "we will clean it up later" becomes "we never clean it up," and the shortcuts harden into the foundation. The team then maintains prototype-quality code for years, because rewriting working software is always lower priority than new features.

Set the expectation early that throwaway code will be rebuilt before it ships, and protect time for that rebuild in the plan. If the deadline cannot accommodate the rebuild, the honest options are to build evolutionary from the start, to descope, or to delay. Shipping throwaway code is not a shortcut; it is a debt conversion that trades weeks of speed for years of drag.

### Set Expectations With Stakeholders About What The Prototype Is

Stakeholders see a working prototype and assume the product is nearly done, which creates pressure to ship it as-is. This pressure is the mechanism by which throwaway prototypes become production. Manage expectations by being explicit about what the prototype represents: a proof of concept, a learning artifact, or the beginning of production.

Label the prototype clearly and explain its lifecycle in the demo. "This is a throwaway to validate the flow; we will rebuild it properly before it ships" sets a different expectation than "look, it works." When stakeholders understand the prototype is disposable, they do not treat its existence as a reason to skip the production build.

### Isolate Prototype Code From Production Systems

When a prototype touches production systems, the boundary between disposable and permanent blurs, and the prototype's shortcuts can leak into shared infrastructure. Fake data, mocked auth, and hardcoded values are dangerous the moment they connect to real systems or real users.

Keep throwaway prototypes isolated from production data, production services, and production users. Use sandboxed environments, synthetic data, and clearly separated codebases. If the prototype must touch production to answer its question, treat that integration as a signal that the prototype may need to be evolutionary, and raise the quality bar accordingly.

### Plan The Transition Or The Discard Explicitly

Both lifecycles need an explicit endpoint. A throwaway prototype needs a discard plan: when the learning is captured, the code is archived or deleted, and the production build begins. An evolutionary prototype needs a transition plan: what must be true, in terms of tests, performance, security, and documentation, before it is considered production-ready.

Write the endpoint criteria before building. For throwaway, name the question and the discard date. For evolutionary, name the production-readiness checklist. Without an explicit endpoint, throwaway prototypes linger in the codebase as zombies and evolutionary prototypes ship without ever crossing a quality bar.

## Common Traps

### Leaving The Lifecycle Unstated

When no one decides whether the prototype is throwaway or evolutionary, the default is evolutionary-by-accident. The trap is that shortcuts intended for a throwaway become permanent, and the team inherits debt they never chose to take on. Decide the lifecycle before building.

### Shipping Throwaway Code Under Deadline Pressure

The deadline arrives, the throwaway works, and "clean it up later" becomes never. The trap is weeks of speed bought with years of maintenance drag on prototype-quality foundations. Protect time for the rebuild, or build evolutionary from the start.

### Treating Evolutionary Prototypes As Disposable

Building production-quality work and then discarding it wastes the effort. The trap is a team that rebuilt from scratch what they had already built well, because no one agreed the prototype was meant to survive. Align the lifecycle with the build quality.

### Letting Stakeholders Assume The Prototype Is The Product

A working prototype looks done, and stakeholders push to ship it. The trap is that this pressure is exactly how throwaway code reaches production. Set expectations explicitly about what the prototype is and is not.

### Connecting Throwaway Prototypes To Production Systems

Mocked auth and hardcoded data become dangerous when they touch real users or real services. The trap is shortcuts leaking into shared infrastructure or causing real harm. Isolate throwaway prototypes from production.

### No Discard Plan, So The Prototype Lingers

Without a discard date, throwaway code stays in the codebase as a zombie that no one owns and no one removes. The trap is accumulated cruft from prototypes that outlived their question. Plan the discard explicitly.

### No Production-Readiness Bar For Evolutionary Prototypes

Without a checklist, evolutionary prototypes ship without ever being held to production quality. The trap is code that was always "going to be cleaned up" reaching users with prototype-grade tests and security. Define the transition criteria before building.

## Self-Check

- [ ] The prototype's lifecycle, throwaway or evolutionary, was decided and communicated before construction began.
- [ ] Throwaway prototypes are built for speed and learning, with a plan to discard the code and rebuild for production.
- [ ] Evolutionary prototypes are built to production quality from the first commit, with real architecture, tests, and error handling.
- [ ] No throwaway prototype code was shipped to production without a rebuild, and time for that rebuild was protected in the plan.
- [ ] Stakeholders were told explicitly what the prototype represents and were not allowed to treat its existence as near-completion.
- [ ] Throwaway prototypes are isolated from production data, services, and users, with no leaking of shortcuts into shared infrastructure.
- [ ] An explicit endpoint exists: a discard date and question for throwaway, or a production-readiness checklist for evolutionary.
- [ ] The lifecycle decision would still hold if the prototype worked well and the deadline were tight, rather than being reversed under pressure.
