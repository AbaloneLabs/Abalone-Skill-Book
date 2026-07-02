---
name: sanctions_ownership_and_control_analysis.md
description: Use when the agent is applying the OFAC 50 percent rule to determine whether an unlisted entity is blocked by aggregation of SDN ownership, analyzing beneficial ownership for sanctions exposure, resolving incomplete or conflicting ownership data, or assessing control-based blocking under EU and UK regimes.
---

# Sanctions Ownership And Control Analysis

A party does not need to appear on a sanctions list to be blocked. Under the OFAC 50 Percent Rule, an entity that is owned 50 percent or more, individually or in the aggregate, by one or more SDNs is itself blocked even if it is not separately listed. EU and UK regimes apply analogous, though not identical, control tests that can block entities under the control of designated persons regardless of the exact ownership percentage. This means that screening only the named contracting party against a list is insufficient, because the entity may be blocked through the ownership or control of its beneficial owners. The compliance challenge is that ownership data is often incomplete, opaque, deliberately concealed, or conflicting, and the aggregation analysis, adding up the ownership stakes of multiple SDNs across layered structures, requires both accurate data and correct arithmetic. An entity that passes a name screen can be fully blocked through a structure that the screening never examined.

Use this skill before applying the 50 Percent Rule, analyzing beneficial ownership for sanctions exposure, or resolving incomplete ownership data. Ownership-based blocking determinations are legal conclusions that depend on regime-specific rules and the specific facts; they must be confirmed with qualified counsel. This skill provides the analytical framework, not a legal determination that any entity is or is not blocked.

## Core Rules

### Apply The 50 Percent Rule Through Aggregation, Not Individual Ownership

The OFAC 50 Percent Rule blocks an entity when the aggregate ownership of one or more SDNs reaches 50 percent or more, not only when a single SDN owns 50 percent. This aggregation principle is the feature that catches layered structures, because no single SDN may hold a majority stake, yet the combined stakes of several SDNs may exceed the threshold. An entity owned 30 percent by one SDN, 15 percent by another, and 10 percent by a third is 55 percent SDN-owned in the aggregate and is blocked, even though no individual owner holds a majority.

Conduct the aggregation across all SDN owners in the ownership chain, not only the direct shareholders. Sum the ownership stakes of every SDN with an interest at any level, accounting for the structure's layering. The arithmetic must be precise, because the difference between 49 and 51 percent aggregate ownership determines whether the entity is blocked. Document the ownership map, each owner's stake, each SDN's identity, and the aggregation calculation, so that the determination is reconstructable and defensible. An aggregation that omits an SDN owner, misreads the structure, or miscalculates the total produces either an unblocked entity that should be blocked or a blocked entity that should be cleared, both of which are errors with consequences.

### Trace Ownership To The Ultimate Beneficial Owners

The 50 Percent Rule and equivalent control tests require tracing ownership to the ultimate beneficial owners, not stopping at the immediate shareholders. An entity may be owned by a holding company that is itself owned by SDNs, and the SDN ownership flows through the structure to the operating entity. Screening only the direct owner misses the beneficial owners whose stakes drive the blocking analysis.

Trace the ownership chain through every layer to the ultimate natural persons or designated entities that hold the economic interest. At each layer, identify the owners, their stakes, and whether any are SDNs or are themselves entities that must be traced further. The depth of tracing required depends on the structure; a simple entity may have one or two layers, while a complex structure may have many layers across multiple jurisdictions designed to obscure ownership. Stop tracing only when the ownership reaches natural persons, listed entities, or parties that can be definitively screened, not when the data becomes inconvenient to obtain. Incomplete tracing produces an incomplete aggregation and an unreliable determination.

### Resolve Incomplete, Opaque, Or Conflicting Ownership Data

The practical obstacle to ownership analysis is that the data is often incomplete. Shell companies, nominee arrangements, jurisdictions with weak ownership disclosure, and deliberate concealment produce ownership maps with gaps, uncertainties, and conflicts. A counterparty may provide ownership data that does not match registry filings, or that stops at a holding company in a jurisdiction that does not disclose beneficial owners. The compliance officer must decide how to proceed when the data needed for a reliable determination is unavailable.

Do not treat incomplete data as sufficient. If the ownership chain cannot be traced to a point where SDN ownership can be ruled out, the entity cannot be reliably cleared, and the risk must be managed through escalation, enhanced diligence, or declination. Request additional ownership information from the counterparty, corroborate against independent sources such as corporate regist and commercial databases, and document the efforts to obtain complete data. Where data conflicts, investigate the discrepancy rather than accepting the more favorable version. A determination based on incomplete data is not a determination; it is an assumption, and assumptions about SDN ownership are exactly what the 50 Percent Rule is designed to prevent.

### Distinguish Ownership From Control Across Regimes

The OFAC 50 Percent Rule is an ownership test, but EU and UK regimes include control-based tests that can block entities under the control of designated persons regardless of the ownership percentage. Control may arise through voting rights, board appointment rights, contractual arrangements, or other means of direction that do not require a 50 percent economic stake. An entity that is 40 percent owned by an SDN but controlled by that SDN through board dominance or contractual rights may be blocked under EU or UK rules while not blocked under the OFAC ownership test alone.

Apply the correct test for each applicable regime. For OFAC, assess aggregate ownership. For EU and UK, assess both ownership and control, recognizing that control can block at lower ownership thresholds. The control analysis requires examining governance, voting agreements, board composition, and any contractual or informal arrangements through which a designated person could direct the entity. An analysis that applies only the ownership test misses control-based blocking under regimes that reach it, and an analysis that treats all regimes as identical misapplies tests that differ in their thresholds and mechanisms.

### Screen The Ownership Chain Continuously, Not Only At Onboarding

Ownership changes. An entity that was clear at onboarding may become blocked if its ownership structure changes to include an SDN, through acquisition, inheritance, restructuring, or designation of an existing owner. One-time ownership screening at onboarding is insufficient because both the ownership and the SDN status of owners can change over the relationship.

Rescreen the ownership chain on a periodic risk-based cycle and on triggers, including ownership changes reported by the counterparty, list updates that designate a previously clear owner, and material corporate events. Maintain the ownership map current so that a change in ownership or in an owner's designation status triggers reassessment. A relationship that was compliant at inception can become a violation through ownership changes that no one monitored, and continuous ownership screening is the control that catches it.

### Document The Determination And Its Basis

An ownership-based blocking determination, whether the entity is blocked or cleared, must be documented with the ownership map, the screening results for each owner, the aggregation calculation, the control analysis where applicable, and the basis for the conclusion. Regulators reconstruct these determinations from records, and a determination that cannot be supported by documentation is indefensible regardless of whether it was correct.

Document both the data and the reasoning. Record where ownership data was obtained, where it was corroborated, where gaps existed and how they were addressed, and why the conclusion follows from the evidence. For cleared entities, document why SDN ownership was ruled out. For blocked entities, document the aggregation or control basis. A determination that is correct but undocumented cannot be distinguished from one that was guessed, and in enforcement the distinction matters.

## Common Traps

### Screening Only The Named Entity

Running a name screen on the contracting party without tracing and screening the beneficial owners, missing SDN ownership that blocks the entity under the 50 Percent Rule.

### Failing To Aggregate Across SDN Owners

Checking whether any single SDN owns 50 percent rather than summing across all SDN owners, missing blocked entities where no individual SDN holds a majority.

### Stopping Trace At The Direct Shareholder

Tracing ownership only one layer deep, missing SDN ownership that flows through holding companies and layered structures to the operating entity.

### Accepting Incomplete Ownership Data

Treating gaps, conflicts, or opaque ownership as sufficient for clearance, when the inability to rule out SDN ownership means the entity cannot be reliably cleared.

### Applying The Ownership Test Where Control Applies

Using only the OFAC 50 percent ownership test under EU or UK regimes that also reach control, missing entities blocked through governance or contractual control at lower ownership.

### One-Time Ownership Screening

Screening ownership only at onboarding, missing subsequent ownership changes or owner designations that render a previously clear entity blocked.

### Undocumented Determinations

Reaching a blocked or cleared conclusion without recording the ownership map, screening results, aggregation, and basis, leaving the determination indefensible if questioned.

## Self-Check

- Is the 50 Percent Rule applied through aggregation across all SDN owners in the ownership chain, with the total precisely calculated and documented?
- Is ownership traced through every layer to the ultimate beneficial owners, stopping only at natural persons or definitively screened parties?
- Is incomplete, opaque, or conflicting ownership data treated as insufficient for clearance, with escalation, enhanced diligence, or declination rather than assumption?
- Is the control test applied under EU and UK regimes in addition to the OFAC ownership test, recognizing that control can block at lower ownership thresholds?
- Is the ownership chain rescreened on a periodic cycle and on triggers including ownership changes and list updates, not only at onboarding?
- Is each determination, whether blocked or cleared, documented with the ownership map, screening results, aggregation calculation, control analysis, and basis?
- Are ownership data sources recorded, including where obtained, how corroborated, and how gaps were addressed?
- Would a regulator reconstructing the determination find complete evidence supporting the conclusion rather than gaps or assumptions?
- Is the distinction between ownership-based and control-based blocking reflected in the analysis for each applicable regime?
- Are cleared entities supported by documented evidence ruling out SDN ownership rather than by the absence of identified SDNs in incomplete data?
