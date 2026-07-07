---
name: design-to-development-handoff.md
description: Use when the agent is managing the cross-functional workflow between design and development, integrating design into engineering processes, establishing shared tooling and design-dev collaboration, handling QA and design review during build, or improving the ongoing partnership between design and engineering teams across a product lifecycle.
---

# Design To Development Handoff

The relationship between design and development is not a single handoff event but an ongoing cross-functional collaboration that spans the whole product lifecycle. While artifact-level specification (annotating prototypes, defining states and interactions) is covered elsewhere, this skill addresses the broader workflow and partnership: how design integrates into engineering processes, how shared tooling and single sources of truth are established, how design is reviewed and maintained during and after the build, and how the two disciplines collaborate rather than throw work over a wall. The judgment problem is that a clean spec is necessary but insufficient; without an integrated workflow, the build drifts from the design, design decisions made during implementation are lost, and the two teams accumulate friction that degrades every subsequent release.

Agents tend to focus on the spec artifact and neglect the workflow that surrounds it: who reviews the build against design, how design changes during implementation are captured, how shared tooling keeps design and code in sync, and how the relationship is maintained over time. This skill helps the agent build the cross-functional process that makes design intent actually survive into the shipped product, release after release.

## Core Rules

### Treat design-development as an ongoing partnership, not a one-way handoff

The moment of handing specifications to engineering is one event in a continuous relationship, not its conclusion. Design decisions continue through implementation: engineers surface constraints and edge cases, content changes, technical limitations force tradeoffs, and the build reveals problems the design hid. If the relationship ends at handoff, these decisions are made without design input and the product drifts. Establish the partnership as ongoing: design stays involved through implementation, reviews the build, and participates in resolving build-revealed problems. A healthy design-dev relationship is collaborative throughout, not transactional at the start.

### Integrate design into the engineering workflow, not parallel to it

When design and development run as separate tracks that meet only at handoff, the work is misaligned: design produces specs that ignore engineering constraints, and engineering builds without design review until too late. Integrate design into the engineering workflow: involve engineers in design early to surface constraints and feasibility, bring design into sprint planning and technical discussions, and embed design review into the definition of done. Integration catches problems when they are cheap to fix, rather than after they are built.

### Establish shared tooling and a single source of truth

Design and code drift apart when they live in separate systems with no enforced connection. Shared tooling — design tokens that compile into code, component libraries shared between design and engineering, design tools that sync with code repositories — creates a single source of truth that keeps the two aligned. Invest in the bridges: token pipelines, shared component documentation, and tooling that makes divergence visible. Without shared tooling, every alignment is manual and eventually fails.

### Define clear roles, ownership, and decision rights across the boundary

Ambiguity about who owns what at the design-development boundary produces friction and gaps. Define who owns the design system, who decides when a design requires a new component, who has final say on tradeoffs between design intent and engineering constraints, and who reviews the build against design. Clear ownership prevents both duplicated work and dropped responsibilities. When a decision falls in the gap between the two disciplines and no one owns it, it is made by default, usually poorly.

### Build design review into the build process, not only at the end

Design is often reviewed against the build only at the end, when divergence is expensive to fix. Build design review into the process continuously: review components as they are built, check design implementation in pull requests or QA, and catch drift early. Early review catches problems when they are a small fix rather than a rework. Treat design implementation review as a first-class part of the definition of done, not a final cosmetic pass.

### Capture design decisions made during implementation

During the build, many design decisions are made or modified in response to constraints, and these decisions are often lost, leading to drift between the documented design and the shipped product, and to repeated debates later. Capture implementation-time decisions: document why a design changed, update the design source to reflect what was built, and record tradeoffs. A design that is not updated to reflect the built reality becomes wrong over time and misleads future work.

### Manage the tradeoff between design intent and engineering constraints deliberately

Engineering realities — performance, technical constraints, deadlines, legacy systems — inevitably pressure design intent, and these pressures are resolved either deliberately or by default. When resolved by default, the constraint silently wins and the design erodes. Make tradeoffs explicit: surface the constraint, evaluate the cost to the design and the user, and decide deliberately with both design and engineering input. Some tradeoffs rightly favor engineering; many silently favor engineering only because no one advocated for the design.

### Maintain the design after release, not just up to it

Design work does not end at release. The shipped product needs ongoing maintenance: components evolve, new states are discovered, analytics reveal problems, and the design system changes. Without continued design ownership after release, the product drifts from the design over time. Establish who maintains the design of shipped features, how it evolves with the system, and how post-release issues are addressed. Abandoning design at release guarantees gradual erosion.

## Common Traps

### Treating handoff as the end of design involvement

When design disengages at handoff, build-revealed problems are resolved without design input and the product drifts from the intent.

### Parallel tracks that meet only at handoff

Design and engineering running separately produce misaligned work; integrate design into the engineering workflow early.

### Separate systems with no enforced connection

Design and code in unconnected systems drift apart; invest in shared tooling and a single source of truth.

### Ambiguous ownership at the boundary

Decisions that fall between design and engineering are made by default when no one owns them; define roles and decision rights.

### Design review only at the end

Late review catches divergence when it is expensive; build continuous design review into the build process.

### Losing implementation-time design decisions

Decisions made during the build that are not captured cause drift between documented and shipped design and repeated debates.

### Silent erosion of design under engineering pressure

Constraints that silently override design intent erode the product; make tradeoffs explicit and deliberate.

### Abandoning design at release

Without post-release design maintenance, the shipped product drifts from the design over time.

## Self-Check

- Is the design-development relationship treated as an ongoing partnership through implementation, not ending at handoff?
- Is design integrated into the engineering workflow early, involved in planning and technical discussions?
- Is there shared tooling and a single source of truth connecting design and code, with divergence made visible?
- Are roles, ownership, and decision rights at the design-development boundary clearly defined?
- Is design implementation review built continuously into the build process, not only at the end?
- Are design decisions made during implementation captured, with the design source updated to reflect built reality?
- Are tradeoffs between design intent and engineering constraints made explicitly with both disciplines involved?
- Is there continued design ownership and maintenance of shipped features after release?
