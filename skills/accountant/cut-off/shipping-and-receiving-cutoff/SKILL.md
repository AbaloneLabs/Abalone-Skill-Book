---
name: shipping_and_receiving_cutoff.md
description: Use when the agent is testing shipping cutoff for sales, testing receiving cutoff for purchases, applying FOB terms to goods in transit, matching shipping documents to revenue, or verifying that inventory and cost of goods sold reflect the correct period for goods movement.
---

# Shipping And Receiving Cutoff

Shipping and receiving cutoff is the specific application of cutoff testing to the physical movement of goods. It confirms that revenue and cost of goods sold are recorded in the period the goods actually move, and that inventory belongs to the period and entity that owns it at the boundary. This is where cutoff theory meets the warehouse floor. A shipment that left the dock on the last day of the month but was invoiced on the first day of the next month creates a cutoff error. A receipt that arrived before period end but was not recorded until the next month leaves inventory and a payable in the wrong period. Agents often rely on invoice dates or system entry dates for goods movement, when the controlling evidence is the shipping document and the receiving document, interpreted through the shipping terms.

Use this skill before testing shipping or receiving cutoff, applying FOB terms to in-transit goods, or matching goods movement documents to revenue and inventory. The goal is to ensure goods movement is recorded in the correct period based on the controlling documents and terms.

This skill covers goods movement cutoff. Specific revenue recognition triggers, consignment arrangements, drop-ship treatment, and bill-and-hold rules depend on the applicable revenue standard. Confirm the governing framework before applying these procedures.

## Core Rules

### Use The Shipping Document Date For Sales Cutoff

For sales of goods, the cutoff trigger is typically the date control of the goods transfers to the customer, which is usually evidenced by the shipping document, the bill of lading, or the delivery confirmation. The invoice date is an administrative date and is not the controlling trigger.

For sales cutoff testing:

- select shipping documents from the last few days of the period and the first few days of the next period;
- trace each shipment to the recorded revenue and confirm it is in the period of shipment;
- confirm shipments after period end are excluded from current-period revenue;
- investigate any shipment recorded in the wrong period.

A shipment shipped before period end must be revenue in that period, and the goods must be removed from inventory, regardless of when the invoice was issued.

### Use The Receiving Document Date For Purchase Cutoff

For purchases, the cutoff trigger is the date the goods are received, evidenced by the receiving report, modified by the shipping terms. Goods received before period end belong in the current period as inventory or expense, with a corresponding payable.

For purchase cutoff testing:

- select receiving documents from the last few days of the period and the first few days of the next period;
- trace each receipt to the recorded payable and inventory or expense;
- confirm receipts before period end are included in the current period;
- investigate any receipt recorded in the wrong period.

A receipt after period end should not be in current-period inventory or payable, even if the invoice arrived earlier.

### Apply FOB Terms To Goods In Transit

Goods in transit at period end belong to the party that holds the risks and rewards of ownership under the shipping terms. This determines the period and the entity for both the inventory and the related sale or purchase.

Apply the terms correctly:

- under FOB shipping point, ownership transfers to the buyer when the goods leave the seller, so in-transit goods are the buyer's inventory and the seller's sale;
- under FOB destination, ownership transfers when the goods reach the buyer, so in-transit goods remain the seller's inventory and no sale is recognized yet;
- confirm the actual terms on the specific shipment, do not assume a default.

Applying the wrong terms puts goods on the wrong balance sheet and recognizes the sale or purchase in the wrong period.

### Match The Shipping Document To The Revenue Entry

The shipping document should tie directly to the revenue entry. A shipment without a matching revenue entry is understated revenue. A revenue entry without a matching shipment may be premature or fictitious.

The matching check confirms:

- every shipment before period end has a corresponding revenue entry in that period;
- no revenue entry exists for goods not yet shipped;
- the quantity and amount on the shipping document agree to the revenue entry.

Discrepancies indicate a cutoff error, a missing entry, or potentially fictitious revenue.

### Match The Receiving Document To The Inventory And Payable

The receiving document should tie to both the inventory or expense recording and the accounts payable entry. A receipt without a matching payable is an understated liability. A payable without a matching receipt may be premature or unsupported.

The matching check confirms:

- every receipt before period end is recorded in inventory or expense and in accounts payable for that period;
- no payable is recorded for goods not yet received, unless terms require;
- the quantity and amount on the receiving document agree to the entries.

### Handle Special Shipping Arrangements Carefully

Certain arrangements require special cutoff attention because the normal shipping-or-receiving trigger does not apply.

Watch for:

- bill-and-hold arrangements, where revenue recognition depends on specific criteria beyond shipping;
- consignment, where goods shipped to a consignee remain the consignor's inventory until sold to the end customer;
- drop-ship, where the seller never takes physical possession but may still recognize revenue on shipment to the end customer;
- returns and allowances, where cutoff of the return reserve affects both revenue and the period.

These arrangements have specific criteria under the revenue standard. Do not apply the default shipping trigger without checking.

### Reconcile Goods Movement To Inventory And Cost Of Sales

Shipping and receiving cutoff ultimately flows into inventory and cost of goods sold. After cutoff testing, confirm that:

- inventory reflects all receipts before period end and excludes all shipments before period end;
- cost of goods sold reflects all shipments before period end;
- the inventory count, where performed, agrees to the perpetual records adjusted for cutoff.

A cutoff error that is not corrected leaves inventory and cost of sales misstated at the balance sheet date.

## Common Traps

### Using The Invoice Date Instead Of The Shipping Date

The invoice date is administrative. A shipment on the last day invoiced on the first day of the next month is current-period revenue. The shipping document controls.

### Assuming A Default FOB Term

Not all shipments use the same terms. Assuming FOB shipping point or FOB destination without checking the specific shipment puts goods in the wrong period and on the wrong books.

### Testing Shipping But Not Receiving

Shipping cutoff affects revenue and cost of sales. Receiving cutoff affects inventory and payables. Both directions must be tested.

### Ignoring Goods In Transit

Goods physically moving at the moment of period end still belong to someone. Failing to identify and properly classify in-transit goods leaves inventory and the related sale or purchase unrecorded.

### Missing The Match Between Document And Entry

A shipment or receipt without a matching entry, or an entry without a matching document, indicates a cutoff error or a missing or fictitious transaction. The match is the evidence.

### Applying The Default Trigger To Special Arrangements

Bill-and-hold, consignment, and drop-ship have specific criteria. Applying the normal shipping trigger without checking the rules can recognize revenue prematurely or defer it incorrectly.

### Failing To Roll Cutoff Into Inventory And COGS

Cutoff testing that does not flow through to the inventory and cost of sales balances leaves the misstatement in place. The correction must reach the balance sheet and income statement accounts.

## Self-Check

- Is sales cutoff based on the shipping document date, the bill of lading, or delivery confirmation, rather than the invoice date?
- Is purchase cutoff based on the receiving document date, with goods received before period end recorded in that period?
- Are FOB shipping point and FOB destination terms applied to each in-transit shipment based on the actual terms, not an assumption?
- Does every shipment before period end trace to a revenue entry, with no revenue for unshipped goods?
- Does every receipt before period end trace to an inventory or expense entry and a payable, with no payable for unreceived goods?
- Are bill-and-hold, consignment, drop-ship, and returns arrangements evaluated under their specific revenue criteria rather than the default trigger?
- After cutoff testing, do inventory and cost of goods sold reflect the corrected goods movement?
- Have the applicable revenue standard and any industry-specific goods movement rules been confirmed before relying on these procedures?
