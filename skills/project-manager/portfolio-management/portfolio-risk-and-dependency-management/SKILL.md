---
name: portfolio_risk_and_dependency_management.md
description: Use when the agent is managing risk and dependencies across projects in a portfolio, surfacing cross-project resource contention, mapping cascading failure modes, or resolving conflicts that only become visible when multiple projects are viewed together at the portfolio level.
---

# Portfolio Risk and Dependency Management

Individual project risk registers are necessary but insufficient. The risks that most often sink portfolios are not the ones inside any single project; they are the ones that emerge between projects, where one project's delay starves another, where several projects compete for the same scarce specialist, or where a shared dependency, platform, or vendor fails and takes half the portfolio with it. Portfolio risk and dependency management is the discipline of seeing the portfolio as a connected system and managing the failure modes that only exist at that system level.

The judgment problem is how to model cross-project dependencies and shared risks without drowning in detail, how to detect contention and cascading failure before it materializes, and how to allocate scarce shared resources across competing projects fairly and transparently. Agents tend to manage each project's risks in isolation, to treat dependencies as line items rather than failure pathways, and to resolve resource contention reactively by whoever escalates loudest.

## Core Rules

### Model the Portfolio as a Connected System, Not Independent Projects

The first move is to stop thinking of projects as separate. Build a dependency map that shows which projects consume which shared resources, which depend on which deliverables from other projects, and which share common external dependencies such as a platform, a vendor, or a regulatory milestone. Risks that look small inside one project can be systemic when the same dependency underpins many. The portfolio view is what reveals these shared failure modes, and it must exist before contention can be managed.

### Distinguish Within-Project Risk From Portfolio-Aggregated Risk

A risk that is acceptable in one project may be unacceptable when ten projects carry it simultaneously. Aggregate risks by category, dependency, and resource so that the portfolio-level exposure becomes visible. Ten projects each with a moderate schedule risk on the same critical vendor is a high portfolio risk even if no single project flags it as red. Aggregate before you judge severity, because the portfolio consequence is what the organization actually feels.

### Identify and Govern Shared Critical Resources Proactively

Most portfolio failures trace to a small number of bottleneck resources: a key architect, a specialized test environment, a single integration team, a critical vendor. Identify these shared critical resources early, forecast demand across all consuming projects, and govern allocation explicitly. Reactive allocation, where the resource goes to whoever escalates hardest, produces a portfolio optimized for political weight rather than value. Make the allocation rule visible and tie it to strategic priority.

### Map Dependencies as Failure Pathways, Not Just Links

A dependency is not just a connection; it is a pathway along which a delay or failure propagates. For each critical dependency, ask what happens to the downstream project if the upstream deliverable is late, incomplete, or wrong. Model the cascade: which projects stall, which workarounds exist, and where the cascade stops. Dependencies documented only as arrows on a chart do not reveal their blast radius; you must trace the consequence of each one breaking.

### Run Cross-Project Risk Reviews, Not Just Project Risk Reviews

Project-level risk reviews cannot see portfolio risk by construction. Establish a regular cross-project review where risks, dependencies, and resource demands are compared across the portfolio, and where contention and cascading exposure are surfaced. This is where shared risks get owned, because no single project manager owns a risk that spans projects. Without a cross-project forum, systemic risks fall through the gaps between project boundaries.

### Assign Clear Ownership for Cross-Cutting Risks

A risk that spans multiple projects often has no owner, because each project assumes another is handling it. Explicitly assign ownership for each cross-cutting risk and shared dependency to a named role at the portfolio level, with the authority to act across projects. Unowned systemic risks are the ones that compound silently until they become crises. Ownership must be assigned, not assumed.

### Use Scenarios to Stress-Test the Portfolio

Dependencies and resource contention are hard to see in steady state and obvious under stress. Run scenarios: what if the critical vendor slips two months, what if the lead architect leaves, what if the regulatory milestone moves. Stress testing reveals hidden single points of failure and over-concentration that normal reporting misses. Make scenario review a periodic discipline, not a one-off exercise, because the portfolio's fragility changes as projects enter and exit.

### Sequence Projects to Reduce Contention Where Possible

Some contention is structural, but some is created by poor sequencing. Where multiple projects need the same scarce resource, sequence them rather than running them in parallel and forcing constant contention. Sequencing is a portfolio-level decision that individual project managers cannot make, because they only see their own urgency. Deliberate sequencing trades a little schedule on individual projects for large gains in portfolio throughput and reliability.

## Common Traps

### Managing Risks Project-by-Project

Each project manages its own risks and no one sees the aggregated or shared exposure, so systemic failures arrive unannounced. The trap is that project-level risk management feels complete. Aggregate and review cross-project.

### Reactive Resource Allocation by Escalation Volume

Shared resources go to whoever complains loudest or is most senior, rather than to the highest-value use. The trap is that escalation feels like priority. Govern allocation by visible rules tied to strategy.

### Dependencies as Arrows Without Blast Radius

Dependencies are drawn as links but never traced for what breaks downstream, hiding cascading failure modes. The trap is that the diagram feels sufficient. Model the consequence of each dependency failing.

### No Owner for Cross-Cutting Risks

Risks that span projects fall between boundaries because each project assumes another owns them. The trap is that shared risks feel covered. Assign explicit portfolio-level ownership.

### Over-Concentration on a Single Dependency

Many projects depend on one vendor, platform, or person, and the portfolio is fragile to that single point of failure. The trap is that reuse feels efficient. Stress-test concentration and diversify where the blast radius is large.

### Ignoring Aggregated Severity

Each project rates a risk as moderate, but aggregated across ten projects it is severe, and no one escalates. The trap is that individual ratings feel accurate. Aggregate before judging severity.

### Parallel Projects Forced Into Constant Contention

Projects that could be sequenced are run in parallel, manufacturing resource conflicts and delays. The trap is that parallelism feels faster. Sequence deliberately to reduce contention.

### Scenario Thinking Treated as a One-Off

Stress tests are run once at initiation and never repeated, so new fragility goes unseen as the portfolio changes. The trap is that the initial exercise feels durable. Repeat scenario review periodically.

## Self-Check

- [ ] Is there a portfolio-level dependency map showing shared resources, cross-project deliverables, and common external dependencies?
- [ ] Are risks aggregated by category, dependency, and resource so that portfolio-level severity is visible, not just per-project severity?
- [ ] Are shared critical resources identified early with demand forecast across all consuming projects and allocation governed by visible rules?
- [ ] Are critical dependencies traced as failure pathways with their downstream blast radius, not just drawn as links?
- [ ] Is there a regular cross-project risk review where systemic risks and contention are surfaced and owned?
- [ ] Does each cross-cutting risk and shared dependency have a named owner with authority to act across projects?
- [ ] Are scenario and stress tests run periodically to reveal single points of failure and over-concentration?
- [ ] Are projects sequenced deliberately to reduce contention on scarce resources rather than run in parallel by default?
- [ ] Could you name the top three portfolio-level failure modes that no single project risk register would reveal?
- [ ] Is resource contention resolved by strategic priority and transparent rules, not by who escalates loudest?
