---
name: sourcing_rules_and_marketplace_facilitator.md
description: Use when the agent is determining whether a sale is sourced to the origin or destination for sales-tax rate purposes, applying destination sourcing for remote sales, handling local tax and special district add-ons, deciding whether a marketplace facilitator or the underlying seller must collect, or reviewing single-vs-multiple transaction and return liability under marketplace facilitator rules.
---

# Sourcing Rules And Marketplace Facilitator

The judgment problem in sales-tax sourcing is that the rate applied to a sale depends on where the sale is "sourced," and sourcing is not simply the seller's location or the customer's location — it is a rule that varies by state and by the type of sale (in-state origin sourcing versus remote destination sourcing), and it determines not only the state rate but the local and special-district rates that stack on top. Layered on top of sourcing is the marketplace-facilitator regime, which in many states shifts the collection obligation for marketplace sales from the underlying seller to the platform, but only for sales made through the marketplace and only where the platform meets the state's definition. An agent that conflates origin and destination sourcing, or that assumes the marketplace always or never collects, will apply the wrong rate to the wrong party.

Agents tend to assume all sales are sourced to the customer's location (destination sourcing), but several states use origin sourcing for in-state sales, meaning the seller's location rate applies. They also underestimate the complexity of local taxes — some states administer local taxes centrally, some require separate local registration, and the local rate can vary block by block. On marketplace facilitation, agents often apply a binary "the marketplace collects everything" rule, missing that the shift applies only to marketplace sales, that the underlying seller may still owe tax on direct sales, and that the marketplace's collection does not relieve the seller of registration if it has independent nexus. The harm this prevents is a seller or platform that charges the wrong rate, collects in the wrong jurisdiction, or double-collects or under-collects because the sourcing and facilitator roles were misassigned.

This skill covers sourcing-rule determination and marketplace-facilitator collection obligations, primarily under U.S. state and local sales-tax law. It is educational guidance, not personalized tax advice; sourcing rules, local-tax administration, and marketplace-facilitator statutes are jurisdiction-dependent and change frequently. A qualified state-and-local tax professional must be consulted for any specific situation.

## Core Rules

### Determine Origin Versus Destination Sourcing For The Transaction Type

Sourcing rules dictate which jurisdiction's rate applies. Origin sourcing applies the rate of the location where the sale originates (typically the seller's place of business) and is used by some states for in-state sales of tangible personal property. Destination sourcing applies the rate of the location where the customer receives the good (typically the ship-to address) and is the default for most remote and online sales and is used by a growing number of states for all sales. The choice changes both the state base rate and the local add-ons.

For example, a sale by a seller in an origin-sourcing state to an in-state customer may be taxed at the seller's location rate, while the same seller's sale to an out-of-state customer is destination-sourced to the customer's address. Confirm whether the state is an origin-sourcing or destination-sourcing state, and for destination states confirm the exact ship-to jurisdiction including any local or special-district boundaries, because the rate can differ across a street.

### Stack State, Local, And Special-District Rates And Confirm Administration

The applicable rate is often a stack of state, county, city, and special-district (transit, stadium, public safety) taxes. The state rate is usually uniform, but local and district rates vary by jurisdiction and can be substantial. Critically, the administration differs: some states administer all local taxes through a single state return, while others require the seller to register and file separately with each local jurisdiction, which materially changes the compliance burden.

For example, a destination in a special-district boundary may carry an extra percentage or two on top of the state and city rates, and a seller sourcing to that destination must collect the full stack. Determine the full stacked rate for the destination jurisdiction, and confirm whether local taxes are state-administered (one return) or self-administered (separate local registration and filing). Do not quote a "state rate" without the local and district components.

### Treat Remote And Online Sales As Destination-Sourced By Default

Post-Wayfair, the overwhelming majority of states apply destination sourcing to remote and online sales, meaning the rate is determined by the customer's ship-to address, not the seller's location. This requires the seller to determine the correct jurisdiction for each destination address, including resolving address boundaries where postal city does not match taxing jurisdiction. Rate databases and address-validation tools are essential because manual rate lookup is error-prone.

For example, a customer address with a city name that spans multiple taxing jurisdictions must be resolved to the correct jurisdiction, often by ZIP+4 or geolocation, to apply the right rate. Use a current, boundary-accurate rate engine for destination-sourced sales, and do not rely on ZIP-code-level rates, which can cross jurisdiction lines and produce the wrong rate.

### Define The Marketplace And The Marketplace Facilitator Correctly

A marketplace facilitator is a platform that contracts with third-party sellers to facilitate their sales, and the marketplace-facilitator regime shifts the collection obligation for those facilitated sales from the underlying seller to the platform. But the definition of a marketplace and a facilitator varies by state, and not every platform qualifies. A platform must typically list goods for sale, collect payment, and transmit the order, and some states add volume or other thresholds. The determination of whether a given platform is a facilitator under a given state's law is jurisdiction-specific.

For example, a platform that lists goods and processes payment but does not fulfill orders may or may not qualify as a facilitator depending on the state's definition. Confirm whether the platform meets the state's marketplace and facilitator definitions before relying on it to collect, because if it does not qualify, the underlying seller remains responsible.

### Limit The Facilitator Collection Shift To Marketplace Sales Only

The marketplace-facilitator collection obligation applies only to sales facilitated through the marketplace, not to the seller's direct sales through its own website or other channels. A seller that also sells direct must still register and collect on those direct sales if it has nexus, and the marketplace's collection on facilitated sales does not relieve the seller of its independent obligations on non-marketplace sales. Conversely, a pure marketplace seller with no direct sales may have no independent collection obligation in states where the facilitator collects.

For example, a seller with both marketplace and direct website sales must treat the two streams separately — the marketplace collects on marketplace sales, and the seller collects on direct sales where it has nexus. Do not let marketplace collection create a false sense that all obligations are covered; analyze each sales channel independently.

### Address Facilitator Liability, Returns, And The Seller's Residual Role

When the facilitator collects, it is generally liable for the tax on the marketplace sales and files the returns, but the underlying seller may still have reporting obligations, and disputes can arise over who is liable for errors, exemptions, and audits. Some states hold the facilitator solely liable once it collects, while others retain residual liability for the seller if the facilitator fails to collect correctly. The seller must understand its residual exposure and ensure the facilitator is properly registered and remitting.

For example, if a facilitator under-collects due to a rate error, some states hold the facilitator liable while others may look to the seller for the shortfall. Clarify the allocation of liability under each state's facilitator statute, confirm the facilitator is registered and remitting, and retain records of facilitator-collected sales for reconciliation and audit defense.

## Common Traps

### Assuming All Sales Are Destination-Sourced

The symptom is applying the customer's address rate to every sale. The trap is that some states use origin sourcing for in-state sales, so the seller's location rate applies, and applying destination logic over- or under-collects. Confirm the state's sourcing rule for the transaction type.

### Quoting A State Rate Without Local And Special-District Components

The symptom is a seller charging only the state base rate. The trap is that local and special-district rates can add substantial percentage points, and omitting them under-collects. Determine the full stacked rate for the destination jurisdiction.

### Relying On ZIP-Code-Level Rates For Destination Sourcing

The symptom is a seller using a five-digit ZIP to set the rate. The trap is that ZIP codes can cross taxing jurisdiction lines, producing the wrong rate for boundary addresses. Use boundary-accurate address resolution, not ZIP-level rates.

### Assuming Any Platform Is A Marketplace Facilitator

The symptom is treating a platform as a facilitator that must collect. The trap is that the state's definition of marketplace and facilitator varies, and a platform that does not meet the definition leaves the seller responsible. Confirm the platform qualifies under the state's statute.

### Letting Marketplace Collection Mask The Seller's Direct-Sale Obligations

The symptom is a seller concluding it has no obligations because the marketplace collects. The trap is that facilitator collection covers only marketplace sales, so direct website sales still require registration and collection where the seller has nexus. Analyze marketplace and direct sales separately.

### Assuming The Facilitator's Collection Fully Relieves The Seller Of Liability

The symptom is a seller assuming zero residual exposure once the facilitator collects. The trap is that some states retain residual seller liability if the facilitator under-collects, and the seller may need records to defend itself. Clarify the liability allocation and retain facilitator-collection records.

## Self-Check

- [ ] Has the transaction been sourced correctly — origin sourcing for in-state sales in origin states, destination sourcing for remote and online sales — based on the specific state's rule?
- [ ] Has the full stacked rate (state, county, city, special-district) been determined for the applicable jurisdiction, rather than quoting a state base rate alone?
- [ ] Has local-tax administration been confirmed — state-administered single return versus separate local registration and filing — to capture the true compliance burden?
- [ ] For destination-sourced sales, is a boundary-accurate rate engine used rather than ZIP-code-level rates that can cross jurisdiction lines?
- [ ] Has the platform been confirmed to meet the state's marketplace and facilitator definitions before relying on it to collect?
- [ ] Are marketplace sales and direct sales analyzed separately, with the seller collecting on direct sales where it has independent nexus?
- [ ] Has the allocation of liability between facilitator and seller been clarified under each state's statute, with facilitator-collection records retained for reconciliation and audit defense?
- [ ] Has the agent noted that this is general sales-tax information, not personalized tax advice, and recommended consulting a qualified state-and-local tax professional for the specific situation?
