---
name: cross_border_vat_and_place_of_supply.md
description: Use when the agent is determining the place of supply for cross-border transactions, distinguishing B2B general rule and reverse charge from B2C rules, handling exports and imports, classifying digital services for OSS or IOSS, or applying reverse charge mechanics to intra-EU and international supplies.
---

# Cross-Border VAT And Place Of Supply

The judgment problem in cross-border VAT is that the tax treatment of a transaction hinges on the place of supply, and the place of supply is determined by a set of rules that fork sharply depending on whether the customer is a business (B2B) or a consumer (B2C), what kind of good or service is supplied, and whether the supply crosses a border. Get the place of supply wrong and the entire VAT chain is wrong — the wrong party accounts for the tax, the wrong country's rate applies, the wrong registration or reporting (OSS, IOSS, reverse charge, EC Sales List) is triggered, and the business can simultaneously owe tax in a country where it filed nothing and have filed tax in a country where none was due. Cross-border VAT is the area where small classification errors produce the largest downstream exposure, because the rules interact and a mistake compounds across many transactions.

Agents tend to apply the domestic VAT instinct to cross-border facts, treat B2B and B2C the same, and reach for the general rule without checking the long list of exceptions (real estate, transport, restaurant and catering, admission to events, digital services, intermediaries). They conflate exports (out of the union) with intra-EU supplies, forget that imports trigger import VAT at the border, mishandle the reverse charge by either double-accounting or omitting it, and treat OSS and IOSS as interchangeable. The harm this prevents is a business that zero-rates an intra-EU supply without validating the customer's VAT number, that fails to reverse-charge a service received from abroad (leaving it with an unrecoverable liability), or that sells digital services to consumers without OSS and accumulates registration obligations in multiple member states.

This skill covers cross-border place-of-supply rules, exports and imports, digital services via OSS and IOSS, and reverse charge mechanics, primarily under EU and EU-style VAT systems. It is educational guidance, not personalized tax advice; place-of-supply rules, reverse charge scope, OSS and IOSS mechanics, and import/export treatment are jurisdiction-dependent — they vary by EU member state and by country — and change frequently. A qualified tax professional must be consulted for any specific situation.

## Core Rules

### Determine Place Of Supply First, Because Everything Else Follows

The place of supply is the gateway question: it decides which country's VAT rules and rates apply, who accounts for the tax, and which reporting regime is triggered. Under EU rules, the B2B general rule for services and most cross-border supplies is that the place of supply is where the customer is established, which shifts the VAT accounting to the customer via reverse charge. The B2C general rule for services is typically where the supplier is established, meaning the supplier charges its own domestic VAT unless an exception applies. These general rules are the starting point, not the answer, because dozens of exceptions override them.

For example, a consultancy in one member state providing services to a VAT-registered business in another member state applies the B2B general rule — place of supply is the customer's country, the supply is zero-rated by the supplier, and the customer reverse-charges the VAT in its own return. The same consultancy serving a consumer abroad applies the B2C general rule — the supplier charges its own domestic VAT — unless the service is one where a B2C exception applies (such as services connected to immovable property, which are taxed where the property is located). Always identify whether the customer is a business or consumer, then apply the general rule, then test for exceptions in that order.

### Apply The B2B General Rule And Reverse Charge As The Default For Cross-Border Services

For cross-border B2B supplies of services (and intra-EU supplies of goods), the default under EU rules is that the place of supply is the customer's location and the customer accounts for the VAT under the reverse charge mechanism. The supplier zero-rates the supply, reports it (for example on an EC Sales List or equivalent recapitulative statement), and the customer self-assesses output VAT on the purchase while simultaneously deducting it as input VAT if fully recoverable. The reverse charge is the mechanism that moves the compliance burden to the customer and prevents the supplier from needing to register in the customer's country.

For example, a software company in member state A licensing software to a registered business in member state B does not charge VAT; it issues a zero-rated invoice quoting the customer's valid VAT number, reports the intra-EU supply, and the customer in B reverse-charges VAT at B's rate and deducts it as input VAT. The critical precondition is that the supplier must validate the customer's VAT identification number (via VIES or the equivalent system) and retain evidence that the customer is a taxable person; without that, the supply is treated as B2C and the supplier may owe VAT. Never zero-rate a cross-border B2B supply without proof of the customer's VAT number and status.

### Handle B2C Cross-Border Services And The Digital-Services Exception

For cross-border B2C supplies of services, the general rule is that the place of supply is where the supplier is established, so the supplier charges its own domestic VAT. The major exception is electronically supplied services (digital services), telecommunications, and broadcasting supplied to consumers in other member states, where the place of supply is the consumer's location and the supplier must account for VAT there — either by registering in each member state where it has consumers or by using the One Stop Shop (OSS) to declare and pay all of it through a single member state.

For example, a business selling streaming subscriptions to consumers across the EU must charge each consumer's member-state rate, and can use OSS to report and pay all of it in one return rather than registering in every member state. The distinction between digital and non-digital services matters: a downloaded e-book is a digital service (consumer-location rule, OSS), while an in-person training course is not (supplier-location rule). Classify the service correctly before deciding the place of supply, because misclassifying a digital service as a standard service leaves the supplier charging the wrong country's VAT.

### Distinguish Exports, Intra-EU Supplies, And Imports

Cross-border movement of goods splits into three regimes that must not be conflated. An export is a supply of goods dispatched out of the union (to a third country); under EU rules exports are generally zero-rated, subject to evidence of export (customs declaration, proof of transport out of the union). An intra-EU supply is a dispatch of goods from one member state to another between taxable persons; it is zero-rated by the supplier (with VAT-number validation and reporting) and the acquisition is reverse-charged by the acquiring business in the destination member state. An import is the entry of goods into the union from a third country; import VAT is charged at the border (and is generally recoverable as input VAT by the importer, subject to conditions).

For example, a manufacturer exporting goods to a customer outside the union zero-rates the supply and retains the export evidence; the same manufacturer selling to a registered business in another member state zero-rates the intra-EU supply and reports it, with the customer reverse-charging the acquisition; and a business importing components from a third country pays import VAT at customs and reclaims it as input VAT. Each regime has distinct documentation, rate, and reporting requirements, so identify the goods movement type before determining the treatment.

### Use OSS And IOSS Correctly For B2C Cross-Border Digital And Distance Sales

The One Stop Shop (OSS) and Import One Stop Shop (IOSS) are simplification regimes that let a supplier declare and pay VAT in multiple member states through a single registration and return, avoiding the need to register in each member state. OSS covers B2C supplies of digital services, telecommunications, and broadcasting across the union, and (under the post-2021 rules) B2C intra-EU distance sales of goods and certain domestic supplies by non-established suppliers. IOSS covers the distance sale of goods imported into the union from third countries to consumers, up to a consignment value threshold (commonly a low-value threshold), allowing import VAT to be declared via IOSS rather than collected at customs.

For example, a non-union business selling low-value imported goods to EU consumers can use IOSS to charge destination VAT at checkout and declare it in one IOSS return, so the goods clear customs without VAT being collected at the border, which speeds delivery. A union business selling digital services union-wide uses OSS to report all destination VAT centrally. The trap is treating OSS and IOSS as interchangeable or optional in a way that ignores the consignment-value threshold and the split between goods and services. Confirm which regime fits the supply, whether the consignment value brings it within IOSS, and whether marketplace or intermediary rules shift the obligation.

### Apply The Reverse Charge Carefully To Avoid Double Counting Or Omission

The reverse charge mechanism requires the customer to self-assess output VAT on a cross-border supply received and, simultaneously, to deduct that same amount as input VAT (if recoverable), so that for a fully recoverable business the net cash effect is zero but both entries must appear on the return. Errors fall into two patterns: omitting the reverse charge entirely (leaving the supply unaccounted for and creating an unrecoverable exposure), or double-counting by having both supplier and customer account for the VAT. Reverse charge also appears in specific domestic anti-fraud regimes (such as certain high-value goods or construction-sector domestic reverse charges), which are distinct from the cross-border reverse charge but use the same mechanism.

For example, a business receiving marketing services from a supplier abroad must post both the output VAT (reverse charge) and the input VAT recovery on its return; if it posts only the input recovery without the matching output self-assessment, the return is wrong and the input recovery may be denied. Always pair the reverse-charge output entry with the corresponding input recovery entry, confirm the customer's VAT number was valid at the time of supply, and check whether any domestic reverse-charge regime applies to the transaction type.

### Validate Customer Status And Retain Place-Of-Supply Evidence

Because the entire cross-border treatment depends on whether the customer is a business or consumer and where they are located, evidence of customer status and location is a legal precondition, not a nice-to-have. For B2B, this means validating the customer's VAT identification number (via VIES for EU customers) at the time of supply and retaining the validation. For B2C digital services, this means collecting and retaining non-contradictory evidence of the customer's location (such as billing address, payment address, IP address, or similar data points) to justify applying a particular member state's rate.

For example, an intra-EU supply zero-rated by the supplier can be reclassified and taxed if the supplier cannot prove the customer's VAT number was valid, turning a zero-rated sale into a taxable one with back VAT. A digital-services supplier that cannot evidence the consumer's location may be forced to default to the highest rate or face disputes. Build customer-status validation and location-evidence retention into the billing process, because without it the place-of-supply determination is undefendable.

## Common Traps

### Treating B2B And B2C The Same For Place Of Supply

The agent applies one place-of-supply rule regardless of customer type. The trap is that the B2B general rule (customer location, reverse charge) and the B2C general rule (supplier location, unless an exception) produce opposite results. Always determine business versus consumer status first.

### Zero-Rating A B2B Supply Without Validating The Customer's VAT Number

The supplier zero-rates an intra-EU or cross-border service assuming the customer is a business. The trap is that without a valid VAT number and status evidence, the supply is treated as B2C and the supplier owes VAT. Validate the customer's VAT number (via VIES) and retain the proof.

### Conflating Exports, Intra-EU Supplies, And Imports

The business treats all cross-border goods the same. The trap is that exports (out of the union, zero-rated with export evidence), intra-EU supplies (between member states, zero-rated with reverse charge), and imports (into the union, import VAT at the border) have distinct rules and documentation. Identify the goods-movement type before determining treatment.

### Omitting Or Double-Counting The Reverse Charge

The customer recovers input VAT on a cross-border service without posting the matching reverse-charge output entry, or both parties account for the VAT. The trap is that the reverse charge requires a paired output self-assessment and input deduction; omitting it can deny recovery and double-counting overpays. Always pair the entries and confirm only one party accounts.

### Treating OSS And IOSS As Interchangeable

The business uses the wrong single-window regime or assumes one covers everything. The trap is that OSS covers union B2C digital and distance sales while IOSS covers imported low-value goods to consumers, with a consignment-value threshold. Confirm which regime fits and whether the value threshold applies.

### Applying The General Rule And Skipping The Exceptions

The agent applies the general place-of-supply rule and moves on. The trap is that exceptions for immovable property, passenger transport, restaurant and catering, admission to events, and digital services override the general rule and change the country of taxation. Test every cross-border supply against the relevant exceptions.

### Assuming Import VAT Is The Same As Domestic VAT

The importer treats import VAT casually or expects it to be handled by the supplier. The trap is that import VAT is charged at the border, is a separate event from the domestic supply, and is recoverable as input VAT only subject to conditions and evidence. Account for import VAT at customs and reclaim it correctly.

### Failing To Retain Customer Location Evidence For Digital Services

The digital-services supplier charges a member state rate without evidence of the consumer's location. The trap is that location evidence is a legal precondition for applying a destination rate, and its absence can force a default to the highest rate or create disputes. Collect and retain non-contradictory location evidence.

## Self-Check

- [ ] Has the place of supply been determined as the first step, before rate, registration, or reporting decisions?
- [ ] Has the customer been classified as business (B2B) or consumer (B2C), since the general rules diverge sharply between the two?
- [ ] For cross-border B2B supplies, has the reverse charge been applied with the supplier zero-rating, the customer self-assessing output VAT and deducting input VAT, and the paired entries posted on the return?
- [ ] Has the customer's VAT identification number been validated (via VIES or equivalent) at the time of supply and the validation retained, before zero-rating a B2B supply?
- [ ] For B2C cross-border services, has the service been classified as digital (consumer-location rule, OSS) versus non-digital (supplier-location rule) before deciding the treatment?
- [ ] Have exports (out of the union, zero-rated with export evidence), intra-EU supplies (between member states, zero-rated with reverse charge), and imports (into the union, import VAT at the border) been distinguished rather than conflated?
- [ ] For B2C digital and distance sales, has the correct regime — OSS for union supplies and IOSS for imported low-value goods, respecting the consignment-value threshold — been selected?
- [ ] Has every cross-border supply been tested against place-of-supply exceptions (immovable property, transport, restaurant and catering, admission, digital services) rather than stopping at the general rule?
- [ ] For digital services to consumers, has non-contradictory customer location evidence been collected and retained to justify the destination rate?
- [ ] Has the agent noted that this is general VAT information, not tax advice, and recommended consulting a qualified tax professional for the specific situation?
