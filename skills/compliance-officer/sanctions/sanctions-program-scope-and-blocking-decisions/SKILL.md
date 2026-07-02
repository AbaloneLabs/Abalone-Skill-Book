---
name: sanctions_program_scope_and_blocking_decisions.md
description: Use when the agent is determining which sanctions programs apply to a transaction, distinguishing comprehensive embargo programs from list-based programs, deciding whether to block or reject a transaction, or assessing sectoral and secondary sanctions exposure across OFAC, EU, UN, and UK regimes.
---

# Sanctions Program Scope And Blocking Decisions

Sanctions compliance fails most often not because a party was mis-screened but because the wrong question was asked. The compliance officer who asks only whether the counterparty appears on a list has reduced a complex program architecture to a single dimension, and has missed that sanctions operate through multiple distinct mechanisms, each with different legal effects. A comprehensive embargo program prohibits all transactions with a jurisdiction regardless of whether any party is listed. A list-based program blocks transactions with specifically designated parties. A sectoral program restricts certain dealings with entire industries without full blocking. A secondary sanctions regime reaches non-US parties for conduct with no US nexus. Each mechanism demands a different response, and confusing them, blocking when rejection was required, applying sectoral limits as if they were embargoes, or ignoring secondary exposure because no US person is involved, produces violations that are strict-liability and often severe.

Use this skill before determining which sanctions programs apply, deciding whether to block or reject, or assessing sectoral and secondary exposure. Sanctions law is complex, jurisdiction-specific, and changes frequently; program scope and blocking decisions must be confirmed with qualified counsel and are not matters of internal discretion. This skill provides a framework for identifying the issues, not a legal determination of what any program requires.

## Core Rules

### Identify All Applicable Programs Before Screening

The first step is to identify every sanctions regime that could apply to the transaction, based on the parties, the goods, the geography, the currency, the clearing, and the nexus to each regime's jurisdiction. A transaction may be subject to OFAC programs because of US dollar clearing or US person involvement, to EU programs because of an EU party or EU nexus, to UN programs because they bind all member states, and to UK programs because of UK persons or conduct. Each regime maintains its own programs with its own scope, and applying only the home-jurisdiction regime misses obligations that attach through other nexuses.

Map the transaction against each regime's applicability. Identify the US nexus, including US persons, US-origin items, US dollar clearing through US financial institutions, and US territory. Identify the EU nexus, including EU persons, EU territory, and EU-established entities. Identify UN obligations, which bind member states and their nationals. Identify UK nexus under post-Brexit OFSI regimes. A transaction with no apparent home-jurisdiction nexus may nonetheless be subject to multiple regimes through clearing, origin, or party connections, and each applicable regime's programs must be assessed.

### Distinguish Comprehensive Embargo From List-Based Programs

The distinction between comprehensive and list-based programs determines the screening approach and the response to a match, and conflating them produces systematic error. A comprehensive embargo program prohibits transactions with an entire jurisdiction or region regardless of whether any specific party is designated. Cuba, Iran, North Korea, Syria, and certain Russian regions under comprehensive measures are examples where the prohibition attaches to the geography, not to a list entry. Screening the named party against a list is insufficient, because the prohibition applies even if the party is not listed. List-based programs, by contrast, prohibit transactions with specifically designated parties, typically on the SDN List or equivalent, and require screening to identify whether a party is designated.

For comprehensive embargo programs, the control is geographic, not name-based. Screen the transaction's parties, origins, destinations, beneficial owners, and routing against the embargoed jurisdictions, and prohibit any nexus regardless of list status. For list-based programs, the control is party-based, screening against the relevant lists. Most institutions face both types, and the program must handle each with the appropriate mechanism. Treating a comprehensive embargo as if it were list-based, screening names but ignoring geography, is a fundamental and common error.

### Apply Sectoral Sanctions By Their Specific Directives

Sectoral sanctions restrict certain types of dealings with defined sectors or entities without imposing a full block, and they operate through specific directives that define what is and is not permitted. The US Sectoral Sanctions Identifications list, for example, imposes directives that may prohibit certain debt transactions, certain equity dealings, or certain oil-sector involvement with listed entities, while permitting other dealings. The restrictions are directive-specific, and applying a blanket prohibition or a blanket permission misstates the obligation.

For each sectoral designation, identify the specific directive and what it prohibits. Some directives prohibit new debt above defined maturities; others prohibit the supply of goods or services to specified sectors. The permitted and prohibited dealings differ by directive, and an entity may be subject to multiple directives with different restrictions. Document the directive analysis so that the dealing decision reflects the actual restriction rather than a generalized assumption that sectoral means blocked or that sectoral means unrestricted.

### Decide Block Versus Reject Based On Program And Transaction Stage

When a transaction implicates a sanctions prohibition, the required response depends on the program and the transaction stage, and the wrong response can itself be a violation. Blocking applies where the property or funds are subject to a blocking program, typically because the counterparty is an SDN or the transaction involves a comprehensively embargoed jurisdiction. Blocked property must be frozen, placed in a blocked account, reported, and held; it may not be returned, transferred, or used until authorized. Rejecting applies where the transaction is prohibited but there is no blockable interest, for example a payment to an embargoed jurisdiction that has not yet been received, or a transaction that cannot be processed but does not involve frozen property. Rejected transactions are refused and reported, but no property is held.

The distinction is critical because returning blocked funds is itself a separate violation. Operations staff under pressure may reverse or unwind a payment to resolve an alert, and if the funds were blockable, the reversal compounds the violation. Train operations to recognize that a suspected blocking match freezes the funds pending review and that unwinding the transaction to avoid the problem is prohibited. The block-versus-reject decision must follow the program's requirements, documented and escalated, not the operational convenience of clearing the alert.

### Assess Secondary Sanctions Exposure For Non-US Parties

Secondary sanctions reach non-US persons for specified conduct involving sanctioned parties or jurisdictions, even where no US person, US item, or US nexus is involved. This is the mechanism by which the US imposes sanctions risk on foreign companies that engage in certain dealings with Iran, Russia, or other targets. A non-US company may face no primary US sanctions exposure, because it is not a US person and has no US nexus, but still face secondary sanctions risk that can result in being cut off from the US financial system or the US market.

Assess secondary sanctions exposure for non-US parties in the transaction, including counterparties, beneficial owners, and intermediaries. Identify whether the conduct falls within a secondary sanctions scope, such as significant transactions with specified sectors or parties. Secondary sanctions exposure affects risk decisions even where no legal prohibition applies to the company directly, because it affects the reliability and continuity of counterparties. A counterparty that incurs secondary sanctions may become unable to clear US dollars or to transact with US-connected institutions, creating commercial and operational risk that prudent compliance assesses.

### Maintain Current List And Program Awareness Across Regimes

Sanctions programs and lists change frequently and sometimes intraday. OFAC, the EU, the UN, and the UK publish designations and program changes on different schedules, and a designation issued today creates obligations today, not when the next periodic screen runs. A screening engine operating on a stale list, or a program assessment based on outdated guidance, misses current obligations.

Maintain awareness of program changes, not only list updates. New comprehensive measures, new sectoral directives, and changes to general licenses alter what is permitted. Subscribe to regime-specific updates and establish a process to assess the impact of each change on current transactions and existing positions. Record the list version and program guidance against each decision so that the state of the sanctions landscape at the time of decision is reconstructable. A decision that was correct under last month's program may be a violation under this week's, and only current awareness prevents that gap.

## Common Traps

### Reducing Sanctions To Name Screening

Asking only whether the party is listed, missing comprehensive embargo programs that prohibit dealings with a jurisdiction regardless of list status.

### Single-Regime Tunnel Vision

Applying only the home-jurisdiction sanctions regime, missing OFAC, EU, UN, or UK obligations that attach through clearing, origin, party, or territorial nexus.

### Treating Sectoral Sanctions As Full Blocks

Applying a blanket prohibition to sectoral designations when the directive permits some dealings, or treating them as unrestricted when the directive prohibits others.

### Returning Blocked Funds

Unwinding or reversing a payment to resolve an alert when the funds are blockable, creating a separate violation on top of the original exposure.

### Ignoring Secondary Sanctions For Non-US Parties

Dismissing sanctions risk because no US nexus exists, missing secondary sanctions exposure that can cut non-US counterparties off from the US system.

### Stale Lists And Program Awareness

Running screens against outdated lists or relying on outdated program guidance, missing same-day designations and program changes.

### Generalizing Across Regimes

Assuming that EU, UN, UK, and OFAC programs are equivalent, when their scope, lists, and mechanisms differ in ways that matter for specific transactions.

## Self-Check

- Has every applicable sanctions regime been identified based on parties, goods, geography, currency, clearing, and jurisdictional nexus, rather than only the home regime?
- Are comprehensive embargo programs handled through geographic controls distinct from the name-based screening used for list-based programs?
- Are sectoral sanctions applied according to their specific directives, with permitted and prohibited dealings documented per directive rather than generalized?
- Is the block-versus-reject decision based on the program and transaction stage, with blocked funds frozen and never returned or transferred pending review?
- Are blocking and rejection reports filed within required timeframes, with annual blocked property reporting maintained?
- Is secondary sanctions exposure assessed for non-US parties, recognizing risk that affects counterparty reliability even without primary prohibition?
- Are lists kept current across all regimes with documented update cadence, and is the list version recorded against each decision?
- Are program changes, including new comprehensive measures, sectoral directives, and general licenses, monitored and assessed for impact on current and pending transactions?
- Would the program catch a transaction prohibited by geography under a comprehensive embargo even if no party is listed?
- Is the distinction between regimes reflected in distinct controls rather than a single generalized sanctions process?
