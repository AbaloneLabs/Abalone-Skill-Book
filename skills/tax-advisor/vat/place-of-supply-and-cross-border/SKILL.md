---
name: place_of_supply_and_cross_border.md
description: Use when the agent is determining which country VAT applies to a transaction by applying place-of-supply rules, distinguishing the B2B general rule with reverse charge from the B2C destination rule, classifying digital services for OSS or IOSS, separating goods regimes such as intra-EU acquisitions versus imports versus exports, or validating customer status and location evidence for cross-border supplies.
---

# Place Of Supply And Cross-Border

The judgment problem in place-of-supply analysis is that the entire VAT chain — which country's rate applies, who accounts for the tax, and which registration or reporting regime is triggered — hinges on a single gateway determination that forks sharply depending on customer type, supply type, and border crossing. Get the place of supply wrong and everything downstream is wrong at once: the wrong party accounts for tax, the wrong country's rate applies, the wrong registration or reporting (reverse charge, OSS, IOSS, EC Sales List) is triggered, and the business can simultaneously owe tax in a country where it filed nothing and have filed tax in a country where none was due. The determination is layered because the general rules diverge by customer type, dozens of exceptions override them by supply type, and goods and services follow different regimes entirely.

Agents tend to apply the domestic VAT instinct to cross-border facts, treat B2B and B2C identically, and reach for the general rule without testing the long list of exceptions — services connected to immovable property, passenger transport, restaurant and catering, admission to events, and digital services. They conflate exports out of the union with intra-EU supplies between member states, forget that imports trigger import VAT at the border as a separate event, mishandle the reverse charge by either double-accounting or omitting it, and treat OSS and IOSS as interchangeable. They also fail to treat customer-status validation and location evidence as legal preconditions rather than administrative niceties. The harm this prevents is a business that zero-rates an intra-EU supply without validating the customer's VAT number, that fails to reverse-charge a service received from abroad and is left with an unrecoverable liability, or that sells digital services to consumers without OSS and accumulates registration duties in multiple member states.

This skill covers place-of-supply rules that determine which country's VAT applies — the B2B general rule with reverse charge versus the B2C destination rule, digital services via OSS and IOSS, goods versus services, and intra-EU acquisitions versus imports versus exports — primarily under EU and EU-style VAT systems. It is educational guidance, not personalized tax advice; VAT rules are jurisdiction-dependent, vary by EU member state and by country, and change frequently. A qualified tax professional must be consulted for any specific situation.

## Core Rules

### Determine The Place Of Supply Before Rate, Registration, Or Reporting

The place of supply is the gateway question and must be settled first, because it decides which country's VAT rules and rates apply, who accounts for the tax, and which reporting regime is triggered. Under EU rules the B2B general rule for services and most cross-border supplies is that the place of supply is where the customer is established, which shifts VAT accounting to the customer via reverse charge. The B2C general rule for services is typically where the supplier is established, meaning the supplier charges its own domestic VAT unless an exception applies. These general rules are the starting point, not the answer, because dozens of exceptions override them by supply type.

For example, a consultancy in one member state providing services to a VAT-registered business in another applies the B2B general rule — place of supply is the customer's country, the supply is zero-rated by the supplier, and the customer reverse-charges VAT in its own return. The same consultancy serving a consumer abroad applies the B2C general rule and charges its own domestic VAT, unless a B2C exception such as services connected to immovable property applies. Always identify customer type first, apply the general rule, then test for exceptions in that order.

### Apply The B2B General Rule And Reverse Charge As The Cross-Border Default

For cross-border B2B supplies of services and intra-EU supplies of goods, the default under EU rules is that the place of supply is the customer's location and the customer accounts for VAT under the reverse charge mechanism. The supplier zero-rates the supply and reports it on a recapitulative statement such as the EC Sales List, and the customer self-assesses output VAT on the purchase while simultaneously deducting it as input VAT if fully recoverable. The reverse charge moves the compliance burden to the customer and prevents the supplier from needing to register in the customer's country. The critical precondition is that the supplier must validate the customer's VAT identification number at the time of supply and retain the proof.

For example, a software company licensing software to a registered business in another member state issues a zero-rated invoice quoting the customer's valid VAT number, reports the intra-EU supply, and the customer reverse-charges VAT at its own rate and deducts it as input VAT. Without proof of the customer's VAT number and taxable-person status, the supply is treated as B2C and the supplier may owe VAT. Never zero-rate a cross-border B2B supply without validating and retaining evidence of the customer's VAT number.

### Handle B2C Cross-Border Services And The Digital-Services Destination Rule

For cross-border B2C supplies of services the general rule is that the place of supply is where the supplier is established, so the supplier charges its own domestic VAT. The major exception is electronically supplied services — digital services — plus telecommunications and broadcasting supplied to consumers in other member states, where the place of supply is the consumer's location and the supplier must account for VAT there, either by registering in each member state where it has consumers or by using the One Stop Shop to declare and pay all destination VAT through a single member state. Classifying the service as digital or non-digital is therefore decisive.

For example, a business selling streaming subscriptions to consumers across the union must charge each consumer's member-state rate and can use OSS to report and pay all of it in one return. A downloaded e-book is a digital service subject to the consumer-location rule and OSS, while an in-person training course is a non-digital service subject to the supplier-location rule. Misclassifying a digital service as a standard service leaves the supplier charging the wrong country's VAT and missing OSS.

### Distinguish Intra-EU Acquisitions, Imports, And Exports For Goods

Cross-border movement of goods splits into three regimes that must never be conflated. An intra-EU supply is a dispatch of goods from one member state to another between taxable persons; it is zero-rated by the supplier with VAT-number validation and reporting, and the corresponding intra-EU acquisition is reverse-charged by the acquiring business in the destination member state. An export is a supply of goods dispatched out of the union to a third country; exports are generally zero-rated subject to evidence of export such as a customs declaration and proof of transport out of the union. An import is the entry of goods into the union from a third country; import VAT is charged at the border as a separate event and is generally recoverable as input VAT by the importer subject to conditions.

For example, a manufacturer selling to a registered business in another member state zero-rates the intra-EU supply and reports it, with the customer reverse-charging the acquisition; the same manufacturer exporting to a third country zero-rates the export and retains the export evidence; and a business importing components pays import VAT at customs and reclaims it as input VAT. Each regime has distinct documentation, rate, and reporting requirements, so identify the goods-movement type before determining the treatment.

### Use OSS And IOSS Correctly For B2C Digital And Distance Sales

The One Stop Shop and Import One Stop Shop are simplification regimes that let a supplier declare and pay VAT in multiple member states through a single registration and return, avoiding registration in each member state. OSS covers B2C supplies of digital services, telecommunications, and broadcasting across the union, and under the post-2021 rules also B2C intra-EU distance sales of goods and certain domestic supplies by non-established suppliers. IOSS covers the distance sale of goods imported into the union from third countries to consumers, up to a consignment-value threshold for low-value consignments, allowing import VAT to be declared via IOSS rather than collected at customs. The two regimes are not interchangeable.

For example, a non-union business selling low-value imported goods to EU consumers can use IOSS to charge destination VAT at checkout and declare it in one return, so goods clear customs without VAT collected at the border and delivery is faster. A union business selling digital services union-wide uses OSS to report all destination VAT centrally. Confirm which regime fits the supply, whether the consignment value brings it within IOSS, and whether marketplace or intermediary rules shift the obligation to the platform.

### Apply The Reverse Charge As A Paired Entry To Avoid Omission Or Double Counting

The reverse charge mechanism requires the customer to self-assess output VAT on a cross-border supply received and, simultaneously, to deduct that same amount as input VAT if recoverable, so that for a fully recoverable business the net cash effect is zero but both entries must appear on the return. Errors fall into two patterns: omitting the reverse charge entirely, which leaves the supply unaccounted for and can create an unrecoverable exposure, or double-counting by having both supplier and customer account for the VAT. Reverse charge also appears in specific domestic anti-fraud regimes such as certain high-value goods or construction-sector domestic reverse charges, which are distinct from the cross-border reverse charge but use the same mechanism.

For example, a business receiving marketing services from a supplier abroad must post both the output VAT reverse charge and the input VAT recovery on its return; posting only the input recovery without the matching output self-assessment makes the return wrong and can deny the recovery. Always pair the reverse-charge output entry with the corresponding input recovery entry, confirm the customer's VAT number was valid at the time of supply, and check whether any domestic reverse-charge regime applies to the transaction type.

### Validate Customer Status And Retain Location Evidence As A Legal Precondition

Because the entire cross-border treatment depends on whether the customer is a business or consumer and where they are located, evidence of customer status and location is a legal precondition, not an administrative nicety. For B2B this means validating the customer's VAT identification number via VIES or the equivalent system at the time of supply and retaining the validation. For B2C digital services it means collecting and retaining non-contradictory evidence of the consumer's location — such as billing address, payment address, and IP address — to justify applying a particular member state's rate. Without this evidence the place-of-supply determination is undefendable.

For example, an intra-EU supply zero-rated by the supplier can be reclassified and taxed if the supplier cannot prove the customer's VAT number was valid, turning a zero-rated sale into a taxable one with back VAT. A digital-services supplier that cannot evidence the consumer's location may be forced to default to the highest rate or face disputes. Build customer-status validation and location-evidence retention into the billing process.

## Common Traps

### Treating B2B And B2C The Same For Place Of Supply

The symptom is that one place-of-supply rule is applied regardless of customer type. The trap is that the B2B general rule (customer location, reverse charge) and the B2C general rule (supplier location, unless an exception) produce opposite results. The direction is to determine business versus consumer status first and apply the matching general rule.

### Zero-Rating A B2B Supply Without Validating The Customer's VAT Number

The symptom is that an intra-EU or cross-border service is zero-rated assuming the customer is a business. The trap is that without a valid VAT number and status evidence the supply is treated as B2C and the supplier owes VAT. The direction is to validate the customer's VAT number via VIES at the time of supply and retain the proof.

### Conflating Intra-EU Acquisitions, Imports, And Exports

The symptom is that all cross-border goods are treated the same. The trap is that intra-EU supplies (between member states, zero-rated with reverse charge), exports (out of the union, zero-rated with export evidence), and imports (into the union, import VAT at the border) have distinct rules and documentation. The direction is to identify the goods-movement type before determining treatment.

### Omitting Or Double-Counting The Reverse Charge

The symptom is that input VAT on a cross-border service is recovered without the matching output entry, or both parties account for the VAT. The trap is that the reverse charge requires a paired output self-assessment and input deduction; omitting it can deny recovery and double-counting overpays. The direction is to always pair the entries and confirm only one party accounts.

### Treating OSS And IOSS As Interchangeable

The symptom is that the wrong single-window regime is used or one is assumed to cover everything. The trap is that OSS covers union B2C digital and distance sales while IOSS covers imported low-value goods to consumers, with a consignment-value threshold. The direction is to confirm which regime fits and whether the value threshold applies.

### Applying The General Rule And Skipping The Exceptions

The symptom is that the general place-of-supply rule is applied and the analysis stops. The trap is that exceptions for immovable property, passenger transport, restaurant and catering, admission to events, and digital services override the general rule and change the country of taxation. The direction is to test every cross-border supply against the relevant exceptions.

### Misclassifying A Digital Service As A Standard Service

The symptom is that an electronically supplied service is treated like an in-person service. The trap is that digital services follow the consumer-location rule and OSS, so misclassification charges the wrong country's VAT and misses the simplification regime. The direction is to classify the service as digital or non-digital before deciding the place of supply.

### Failing To Retain Customer Location Evidence For Digital Services

The symptom is that a member-state rate is charged to a consumer without location evidence. The trap is that location evidence is a legal precondition for applying a destination rate, and its absence can force a default to the highest rate or create disputes. The direction is to collect and retain non-contradictory location evidence.

## Self-Check

- [ ] Has the place of supply been determined as the first step, before any rate, registration, or reporting decision?
- [ ] Has the customer been classified as business (B2B) or consumer (B2C), since the general rules diverge sharply between the two?
- [ ] For cross-border B2B supplies, has the reverse charge been applied with the supplier zero-rating, the customer self-assessing output VAT and deducting input VAT, and the paired entries posted on the return?
- [ ] Has the customer's VAT identification number been validated via VIES or equivalent at the time of supply and the validation retained, before zero-rating a B2B supply?
- [ ] For B2C cross-border services, has the service been classified as digital (consumer-location rule, OSS) versus non-digital (supplier-location rule) before deciding the treatment?
- [ ] Have intra-EU acquisitions (between member states, reverse charge), exports (out of the union, zero-rated with evidence), and imports (into the union, import VAT at the border) been distinguished rather than conflated?
- [ ] For B2C digital and distance sales, has the correct regime — OSS for union supplies and IOSS for imported low-value goods, respecting the consignment-value threshold — been selected?
- [ ] Has every cross-border supply been tested against place-of-supply exceptions (immovable property, transport, restaurant and catering, admission, digital services) rather than stopping at the general rule?
- [ ] For digital services to consumers, has non-contradictory customer location evidence been collected and retained to justify the destination rate?
- [ ] Has the agent noted that this is general VAT information, not personalized tax advice, and recommended consulting a qualified tax professional for the specific situation?
