---
name: input_vat_recovery_and_blocking.md
description: Use when the agent is deciding which input VAT a business can recover versus which is blocked, testing purchases against the recovery conditions, applying partial exemption and pro-rata methods to mixed or overhead expenditure, handling blocked categories such as entertainment and passenger automobiles, or resolving capital goods recovery and adjustment periods and the requirement of valid VAT invoices.
---

# Input VAT Recovery And Blocking

The judgment problem in input VAT recovery is that reclaiming VAT on a purchase looks like the simple inverse of charging VAT on a sale, but it is in fact a permission-based process gated by a set of conditions that agents routinely skip. Output VAT is a classification-and-rating exercise applied to supplies the business makes; input VAT recovery is a conditions-and-evidence exercise applied to purchases the business makes, and the two are governed by different logic. A business that assumes every invoice bearing VAT is reclaimable will overstate its refund and face rejected claims on audit, while a business that misses recoverable input VAT permanently forfeits cash. The recovery decision is layered: first whether the purchase passes the core conditions, then whether it is directly attributable to taxable supplies or must be apportioned, then whether it falls in a statutorily blocked category, and finally whether capital goods rules extend the adjustment over years.

Agents tend to treat the presence of VAT on an invoice as sufficient grounds to reclaim it, forgetting that a valid invoice is necessary but not sufficient — the purchase must also serve the business's economic activity, must relate to taxable rather than exempt supplies, and must not fall in a blocked category. They forget that for a business making exempt supplies alongside taxable ones, residual overhead input VAT must be apportioned through a partial exemption method, and that the recovery percentage must be recomputed each period and often annualized and trued-up later. They forget that blocked categories such as business entertainment and passenger cars are irrecoverable regardless of business nexus, that capital goods carry multi-year adjustment periods during which a change of use can claw back recovered VAT, and that recovery is period-specific and time-limited so a missed deadline permanently forfeits the amount. The harm this prevents is a return that misstates net VAT, a refund rejected for documentation defects, and input VAT permanently lost through wrong-period claims or expired deadlines.

This skill covers which input VAT is recoverable versus blocked — the core recovery conditions, partial exemption and pro-rata methods, blocked categories, capital goods adjustment, and the requirement of valid invoices — primarily under EU and EU-style VAT systems. It is educational guidance, not personalized tax advice; VAT rules are jurisdiction-dependent, vary by EU member state and by country, and change frequently. A qualified tax professional must be consulted for any specific situation.

## Core Rules

### Gate Every Input VAT Line On The Core Recovery Conditions

Before reclaiming any input VAT, test the purchase against the conditions that most VAT systems require in some form: the business must be VAT-registered, the purchase must be for the purpose of the business's economic activity, the goods or services must relate to taxable (not exempt) supplies, and a valid supporting document — typically a VAT invoice meeting the jurisdiction's requirements — must exist. These conditions are cumulative, not alternative, so the failure of any one denies recovery wholly or partly. A valid invoice alone does not rescue a purchase that lacks business purpose or that relates to exempt supplies, and business purpose alone does not rescue a purchase whose invoice is invalid.

For example, a consultancy with a valid invoice for staff entertainment fails recovery in many member states because entertainment is a blocked category, even though the invoice is valid and the spend is business-related. A bank with a valid invoice for consultancy fails recovery because the input VAT relates to its exempt financial supplies. Test each input VAT line against all conditions, and where a purchase serves mixed purposes, apply an apportionment rather than defaulting to 100% or 0%.

### Separate Directly Attributable Input VAT From Residual And Apply Partial Exemption

When a business makes both taxable and exempt supplies, it is partially exempt, and input VAT must be split into three streams: input VAT directly attributable to taxable supplies (fully recoverable), input VAT directly attributable to exempt supplies (not recoverable), and residual or overhead input VAT that cannot be directly attributed (apportioned). The residual is apportioned using a method that reflects how the purchases are used in making taxable versus exempt supplies. The standard apportionment is pro-rata based on the ratio of taxable turnover to total turnover, but member states permit or require special methods — transaction counts, floor space, staff time, sector-based methods — where the pro-rata distorts.

For example, a training provider selling both standard-rated courses and exempt educational services recovers residual input VAT in proportion to taxable turnover, and may need a special method if turnover-based pro-rata is misleading because the exempt side uses disproportionate overheads. Partial exemption is the single most error-prone area because the residual pool and recovery percentage must be recomputed each period and often annualized and adjusted later, so never treat it as a one-time setup. Document the method and be ready to defend why it gives a fair and reasonable result.

### Exclude Blocked Categories Regardless Of Business Purpose

Many VAT systems block recovery of input VAT on specific categories by statute, regardless of how genuinely business-related the spend is. Common blocked categories include business entertainment and hospitality, passenger cars available for private use, and goods or services given away as gifts beyond a de minimis value. The blocking rule overrides the ordinary conditions test, so a purchase that would otherwise pass can still be irrecoverable. Identify the member state's blocked categories before claiming, because assuming business nexus is enough will produce over-claimed and rejected refunds.

For example, input VAT on a company car used by an employee with private use is commonly irrecoverable in several member states, while input VAT on a commercial van used only for business is recoverable. Input VAT on client dinners is frequently blocked even though the dinners are genuinely for business development. Always run the blocked-category test after the core conditions test, and exclude any blocked input VAT from the recovery figure.

### Handle Capital Goods With Extended Adjustment Periods

Capital goods — such as buildings and other immovable property, aircraft, ships, and high-value equipment — are not treated as a one-time recovery. Many VAT systems impose an extended adjustment period during which the initially recovered input VAT can be clawed back or adjusted if the use of the asset changes, typically several years for immovable property and shorter periods for other capital items. The rationale is that a large input-VAT recovery should reflect the asset's actual use over time, not just its use in the year of purchase.

For example, a business that buys a building, recovers the input VAT, and later shifts the building toward exempt activity must claw back part of the recovered VAT over the capital goods adjustment period. Conversely, a shift toward taxable use may permit additional recovery. Flag every capital goods purchase for ongoing adjustment, record the initial recovery and the intended use, and revisit the adjustment at each interval the member state requires, rather than treating the recovery as final at acquisition.

### Require A Valid VAT Invoice As The Evidentiary Floor

A valid VAT invoice is the evidentiary precondition for recovery, and it must meet the member state's content requirements — typically the supplier's and customer's details, a unique invoice number, the date, a description and quantity of the goods or services, the taxable amount, the VAT rate, and the VAT amount. A defective invoice, a pro-forma, or a receipt that does not meet the invoice requirements can deny recovery even when the underlying purchase qualifies. The right to deduct generally arises in the period the valid invoice is held, not necessarily the period of supply, which interacts with the time-limit rules.

For example, a business that reclaims input VAT from a simple receipt rather than a compliant VAT invoice may have the recovery denied on audit. A business that holds a valid invoice in a later period than the supply must claim in the period of holding, subject to the deadline. Build a process to obtain compliant invoices at the point of purchase and to capture them in the correct period, because recovery without a valid invoice is undefendable.

### Respect Period-Specific Recovery And The Absolute Time Limit

Input VAT recovery is period-specific and time-limited. Most systems require the claim in the period the valid invoice is held, or by a deadline shortly after, and impose an absolute time limit — commonly several years, varying by member state — after which input VAT cannot be reclaimed at all. Late claims, missed periods, and invoices discovered after the deadline can permanently forfeit the recovery. Some systems allow a late claim under specific conditions, but relying on that is risky and jurisdiction-dependent.

For example, a business that finds a valid supplier invoice from two years prior must check whether the member state's time limit and late-claim rules still permit recovery, and if so whether it corrects via an amended return or a separate claim. Build a review process to capture input VAT in the correct period and to surface missed invoices before the absolute deadline, because forfeited input VAT is a permanent cash loss that no later netting can recover.

### Route Cross-Border Input VAT Through The Correct Refund Channel

Input VAT incurred in a member state where the business is not registered generally cannot be netted against the domestic return of the member state where it is registered. It must be reclaimed through the relevant cross-border refund mechanism, such as the EU refund directive for businesses established in the union reclaiming from another member state, or the equivalent process for non-established businesses. The refund process is evidence-dependent and has its own deadlines and forms, distinct from domestic netting.

For example, a business registered in one member state that incurs VAT on a trade fair in another member state must reclaim that VAT through the cross-border refund portal, not by netting it on its domestic return. Identify where input VAT was incurred and route foreign input VAT to the correct refund channel, because netting it domestically is an error that will be disallowed.

## Common Traps

### Assuming A Valid Invoice Makes Input VAT Automatically Recoverable

The symptom is that the business reclaims the full amount because a proper VAT invoice exists. The trap is that a valid invoice is necessary but not sufficient — the purchase must also serve business purpose, relate to taxable supplies, and avoid blocked categories. The direction is to test all recovery conditions, not invoice validity alone.

### Reclaiming Input VAT Linked To Exempt Supplies

The symptom is that a bank or insurer reclaims overhead input VAT like a trader would. The trap is that input VAT relating to exempt supplies is not recoverable, and for mixed businesses the residual must be apportioned, often drastically reducing recovery. The direction is to apply partial exemption before netting.

### Claiming Blocked Entertainment Or Automobile Input VAT

The symptom is that the business reclaims VAT on client dinners or a director's car because the spend seems business-related. The trap is that entertainment and passenger cars are commonly blocked-input categories regardless of business nexus. The direction is to identify blocked categories and exclude them before claiming.

### Forgetting Apportionment On Mixed-Use Purchases

The symptom is that a laptop or phone used partly privately is claimed at 100%. The trap is that the private-use portion fails the business-purpose condition and must be apportioned, with periodic adjustments in some systems. The direction is to apportion mixed-use purchases and document the method.

### Treating Capital Goods Recovery As Final

The symptom is that a business recovers input VAT on a building and closes the file. The trap is that capital goods have extended adjustment periods, and a later change of use toward exempt activity triggers a claw-back over the adjustment window. The direction is to flag capital goods for ongoing adjustment and revisit recovery at each required interval.

### Netting Foreign Input VAT Against The Domestic Return

The symptom is that VAT incurred abroad is netted on the home member state's return. The trap is that cross-border input VAT usually cannot be netted domestically and must be reclaimed through the relevant refund mechanism. The direction is to route foreign input VAT to the correct cross-border refund channel.

### Claiming Input VAT In The Wrong Period Or After The Deadline

The symptom is that input VAT is claimed in a later period or after the absolute time limit, and recovery is denied. The trap is that recovery is period-specific and time-limited, so a missed deadline can permanently forfeit the VAT. The direction is to capture input VAT in the correct period and review for missed invoices before the deadline.

### Using A Receipt Or Pro-Forma In Place Of A Valid VAT Invoice

The symptom is that the business reclaims input VAT from a simple receipt. The trap is that a non-compliant document does not satisfy the evidentiary requirement and recovery can be denied on audit. The direction is to obtain and retain compliant VAT invoices meeting the member state's content requirements.

## Self-Check

- [ ] Has every input VAT line been tested against all core recovery conditions (registration, business purpose, relation to taxable supplies, valid invoice) rather than invoice validity alone?
- [ ] Where the business makes exempt supplies, has partial exemption been applied — separating directly attributable, exempt-linked, and residual input VAT and apportioning the residual with a defensible method?
- [ ] Have blocked categories (entertainment, passenger cars, gifts) been identified and excluded from recovery regardless of business nexus?
- [ ] Have capital goods purchases been flagged for adjustment over the relevant period, rather than treated as final one-time recovery?
- [ ] Has mixed-use expenditure been apportioned and documented, rather than claimed at 0% or 100% by default?
- [ ] Has each reclaim been supported by a valid VAT invoice meeting the member state's content requirements, rather than a receipt or pro-forma?
- [ ] Has input VAT been claimed in the correct period, with missed invoices reviewed against the member state's time limit and late-claim rules?
- [ ] Has cross-border input VAT been routed to the correct refund mechanism rather than netted against the domestic return?
- [ ] Has the partial exemption recovery percentage been treated as a recomputed, often annualized figure rather than a one-time setup?
- [ ] Has the agent noted that this is general VAT information, not personalized tax advice, and recommended consulting a qualified tax professional for the specific situation?
