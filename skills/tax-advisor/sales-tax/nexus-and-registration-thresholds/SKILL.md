---
name: nexus_and_registration_thresholds.md
description: Use when the agent is determining whether a seller has sales-tax nexus in a state, applying economic-nexus dollar and transaction thresholds, evaluating physical-presence triggers such as employees inventory or warehouses, deciding whether and when to register for a sales-tax permit, or reviewing registration-timing and back-tax exposure across multiple jurisdictions.
---

# Nexus And Registration Thresholds

The judgment problem in sales-tax nexus is that "nexus" is not a single test but a layered set of jurisdiction-specific rules, and an agent who applies one state's logic to another state will produce both false negatives (failing to register where there is an obligation) and false positives (registering where none is required). Nexus splits into two main families — physical-presence nexus and economic nexus — and a seller can satisfy either or both in dozens of states simultaneously, each with its own dollar threshold, transaction-count threshold, measurement window, and lookback period. The registration decision is downstream of the nexus determination, but it carries its own timing exposure: registering late means back tax, interest, and penalties on sales that should have been collected, while registering too early in a state with no obligation wastes compliance cost and can lock the seller into filing zero returns.

Agents tend to over-rely on the Supreme Court's Wayfair decision as if it created a single national rule. It did not. Wayfair authorized economic nexus, but each state wrote its own statute, and the thresholds, effective dates, inclusion or exclusion of sales-for-resale and exempt sales, and marketplace-facilitator interactions all differ. Agents also conflate nexus (a connection that creates an obligation) with registration (the administrative act of getting a permit), and they forget that the obligation to register begins on a specific date tied to when the threshold was crossed — not when the seller "discovers" the nexus. The harm this prevents is a seller that under-collects and faces a multi-year back-tax assessment, or one that over-registers and bleeds compliance cost across states where it has no real obligation.

This skill covers the determination of sales-tax nexus and the registration-timing decision, primarily under U.S. state and local sales-tax law. It is educational guidance, not personalized tax advice; nexus rules, thresholds, and effective dates are jurisdiction-dependent and change frequently as states amend their statutes and issue guidance. A qualified state-and-local tax professional must be consulted for any specific situation.

## Core Rules

### Separate Physical-Presence Nexus From Economic Nexus

Physical-presence nexus arises from tangible connections to a state — employees, offices, warehouses, inventory stored in a fulfillment center, trade-show attendance, or third-party agents acting on the seller's behalf — and historically was the primary basis for a collection obligation. Economic nexus arises from the volume of sales into a state with no physical presence, authorized after Wayfair, and is triggered when a seller's in-state sales exceed a dollar threshold, a transaction-count threshold, or both within a defined measurement period. A seller must be tested against both families because physical presence creates nexus even with negligible sales volume, and economic activity creates nexus even with no physical footprint.

The two are not mutually exclusive and a seller can cross both in the same period. For example, a remote seller with no employees in a state but with $150,000 of annual sales there has economic nexus, while a seller with a single warehouse employee in a state has physical-presence nexus even if its sales are tiny. Run both analyses independently and do not let a clean economic-nexus result substitute for the physical-presence question, because some physical-presence triggers (such as inventory in a third-party fulfillment center) are easy to overlook.

### Apply The Specific State's Economic-Nexus Threshold And Measurement Window

Economic-nexus thresholds are not uniform. Most states set a dollar threshold in the range of $100,000 or $200,000 of gross sales into the state in the current or prior calendar year, and many add a separate transaction-count threshold of 200 transactions that can trigger nexus at a much lower dollar volume. The measurement window varies — some states use the current or prior calendar year, some use a trailing twelve-month period, and some use the prior twelve months only. Whether the threshold counts include exempt sales, sales for resale, and marketplace-facilitator-collected sales differs by state and changes the answer for a borderline seller.

For example, a seller with $90,000 of taxable sales and $20,000 of exempt sales into a state may or may not cross a $100,000 threshold depending on whether exempt sales are counted, and a seller with 210 low-dollar transactions may cross a 200-transaction threshold despite being well under the dollar threshold. Pull the specific state's current statute and its definition of "sales" for threshold purposes rather than applying a generic $100,000 figure. Note also that several states have repealed or suspended their transaction thresholds, so confirm the current rule.

### Identify Physical-Presence Triggers That Are Easy To Miss

Physical-presence nexus is not limited to offices and employees. Common but overlooked triggers include inventory stored in a third-party fulfillment center (such as a marketplace's FBA-style warehouse), consignment inventory, drop-ship arrangements where the seller holds title briefly in-state, independent sales representatives or agents operating in the state, installation or repair crews, trade-show or pop-up presence even for a few days, and affiliate or "click-through" relationships. Each of these can create nexus independent of sales volume.

For example, a seller that uses a marketplace's fulfillment network may unknowingly have inventory in a dozen states, creating physical-presence nexus in each regardless of whether the economic threshold is met. Map the seller's entire physical footprint — employees, contractors, inventory locations, and event presence — before concluding there is no physical nexus, because these triggers are the most frequently missed and can generate back-tax exposure even when the economic analysis looks clean.

### Treat The Registration Effective Date As The Anchor For Back-Tax Exposure

When nexus is established, the obligation to register and collect begins on a jurisdiction-specific effective date — typically the first day of the month after the threshold is crossed, or a set number of days after crossing. If a seller discovers it had nexus in a prior period, it faces back tax on sales that should have been collected, plus interest and penalties, and generally cannot retroactively collect that tax from customers. The remediation path (voluntary disclosure, amnesty, or late registration) varies by state and affects the penalty and lookback period.

For example, a seller that crossed an economic threshold eighteen months ago and never registered owes back tax on eighteen months of sales, and because it cannot pass that tax to past customers, the cost typically falls on the seller. Quantify the back exposure before registering, and consider whether a voluntary disclosure agreement can limit the lookback period (often to a defined number of prior years) and waive penalties. Treat the effective date as the fixed anchor for both the first return and any back-tax computation.

### Account For Marketplace-Facilitator Interaction With Nexus

Many states shift the collection obligation for marketplace sales to the marketplace facilitator rather than the underlying seller, which can mean that sales made through a marketplace do not count toward the seller's own economic-nexus threshold, or that the seller has no independent collection obligation on those sales even if it has nexus. The rules vary by state in how they define a marketplace, whether marketplace sales are excluded from the threshold count, and whether a seller with its own nexus still must register for non-marketplace sales.

For example, a seller whose entire in-state volume flows through a registered marketplace facilitator may have no independent collection obligation in that state, but if it also sells direct through its own website, it must register and collect on the direct sales once its own threshold is met. Do not assume that marketplace facilitator collection eliminates all of the seller's obligations; analyze the marketplace sales and the direct sales separately.

### Distinguish Nexus From Registration And Filing Frequency

Nexus is the legal connection; registration is the administrative permit; and filing frequency (monthly, quarterly, annually) is assigned by the state based on expected tax volume. A seller can have nexus but not yet be registered, and once registered it is assigned a filing frequency that can change as its volume grows. Registration also creates an ongoing filing obligation — many states require a return even for a zero-tax period — so registering prematurely in a state with no real obligation creates a perpetual filing burden.

Confirm that nexus exists before registering, then register promptly to avoid late-registration penalties, and track the assigned filing frequency because missing a frequency change (for example, being bumped from quarterly to monthly) causes late filings. Treat registration as a commitment with ongoing cost, not a one-time formality.

## Common Traps

### Treating Wayfair As A Single National Threshold

The symptom is applying one dollar figure as the universal economic-nexus threshold. The trap is that each state sets its own threshold, transaction count, and measurement window, so a single figure is wrong for most states. Always pull the specific state's current statute and definition of includable sales.

### Counting Or Excluding Exempt Sales Inconsistently With The State's Rule

The symptom is a borderline seller that just clears or just misses a threshold. The trap is that states differ on whether exempt sales, sales for resale, and marketplace sales count toward the threshold, so the same revenue can cross or not cross depending on the state's inclusion rule. Confirm the state's definition of includable sales before concluding.

### Missing Inventory In A Third-Party Fulfillment Center

The symptom is a remote seller with a clean economic-nexus analysis but unreported physical presence. The trap is that inventory stored in a marketplace or third-party warehouse creates physical-presence nexus independent of sales volume, and it is the most commonly missed trigger. Map all inventory locations before concluding there is no physical nexus.

### Registering Late And Assuming Back Tax Can Be Collected From Customers

The symptom is a seller that discovers prior-period nexus and expects to invoice past customers for the uncollected tax. The trap is that sales tax on past sales generally cannot be passed on after the fact, so the cost falls on the seller along with interest and penalties. Quantify back exposure early and use voluntary disclosure to limit the lookback.

### Letting A 200-Transaction Threshold Trigger Nexus Unexpectedly

The symptom is a seller with many small-dollar sales that assumes it is safe because its revenue is low. The trap is that some states maintain a transaction-count threshold that triggers nexus regardless of dollar volume, so a high-volume low-price seller can be obligated. Check both the dollar and transaction thresholds for each state, noting that some states have repealed the transaction count.

### Assuming Marketplace Facilitator Collection Eliminates All Obligations

The symptom is a seller that concludes it need not register because the marketplace collects. The trap is that marketplace collection typically covers only marketplace sales, so any direct sales through the seller's own site still require registration and collection once the seller's own threshold is met. Analyze marketplace and direct sales separately.

### Registering Prematurely And Creating A Permanent Filing Burden

The symptom is a seller registering in a state where it has no real obligation "just to be safe." The trap is that registration creates an ongoing filing obligation, often including zero returns, and deregistering is not always simple. Confirm nexus before registering and treat registration as a long-term commitment.

## Self-Check

- [ ] Has physical-presence nexus been analyzed separately from economic nexus, covering employees, inventory, contractors, trade-show presence, and third-party fulfillment locations?
- [ ] Has the specific state's economic-nexus dollar threshold, transaction-count threshold (if any), and measurement window been confirmed against current law rather than a generic figure?
- [ ] Has the state's definition of includable sales for threshold purposes (taxable, exempt, resale, marketplace) been verified before concluding whether the threshold is crossed?
- [ ] Have easily-missed physical-presence triggers, especially inventory in third-party fulfillment centers, been mapped across all states?
- [ ] Has the registration effective date been anchored to the threshold-crossing date, and has back-tax exposure (with interest and penalties) been quantified for any late registration?
- [ ] Has voluntary disclosure or amnesty been considered to limit the lookback period and waive penalties where prior-period nexus exists?
- [ ] Has the marketplace-facilitator interaction been analyzed so that marketplace sales and direct sales are treated separately for collection obligations?
- [ ] Has the assigned filing frequency been confirmed, and is the ongoing filing obligation (including zero returns) understood before registering?
- [ ] Has the agent noted that this is general sales-tax information, not personalized tax advice, and recommended consulting a qualified state-and-local tax professional for the specific situation?
