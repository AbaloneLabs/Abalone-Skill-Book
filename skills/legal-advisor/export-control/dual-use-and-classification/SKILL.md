---
name: dual_use_and_classification.md
description: Use when the agent is advising on export control classification of goods, software, and technology, dual-use item determination, encryption controls, technology and source-code release, intangible technology transfer, end-use and end-user risk, and license-exception eligibility under export-control regimes.
---

# Dual-Use and Classification

Export control starts with classification. Before any cross-border movement of goods, software, or technology, the exporter must determine whether the item is controlled, under which jurisdiction's list (e.g., the EAR Commerce Control List, the EU Dual-Use Regulation Annex I/IV, Wassenaar Arrangement and regime lists), and what license requirements or exceptions apply. The same physical or digital item can be uncontrolled, controlled for one reason, or controlled for many reasons depending on its technical parameters and the destination, end-use, and end-user.

The judgment problem is that classification is technical, parameter-driven, and easy to get wrong in both directions. Over-classification (treating an uncontrolled item as controlled) blocks legitimate trade and creates cost; under-classification (missing a control) is the far more dangerous error and the source of most enforcement actions. Agents frequently overlook that "technology" and "source code" are themselves export-controlled items even when never physically shipped, that encryption is controlled almost everywhere, and that classification must be revisited when parameters change.

This skill is advisory only. Export-control lists are technical and jurisdiction-specific. Confirm classifications with the exporter's internal empowered-official or qualified trade-compliance counsel before relying on them.

## Core Rules

### Determine the controlling jurisdiction(s)

The controlling jurisdiction is generally the jurisdiction of export (where the item is located when exported) plus the jurisdictions whose re-export controls apply (origin country of the item, especially US-origin items which carry worldwide re-export reach). A single transaction can be subject to multiple regimes.

- A strong analysis identifies every jurisdiction whose controls may apply, including the extraterritorial reach of US EAR for US-origin items and items with more than a de minimis US content.
- A weak analysis assumes only the seller's home jurisdiction matters.

### Classify the item against the correct control list

Classification is performed by matching the item's technical parameters to the entries on the dual-use (and, where relevant, military) control list. Lists are organized by category and product group with alphanumeric control entries (e.g., Export Control Classification Numbers under the EAR; dual-use entries under the EU Annex).

- Classification must be based on the item's actual technical parameters and functionality, not its marketing name or intended use.
- Software and technology are classified by what they do or enable, and may be controlled even when embedded in an uncontrolled product.
- Record the rationale and the evidence (technical specifications) supporting each classification decision.

### Treat technology and source code as controlled items

"Technology" (technical assistance, know-how, design documents, blueprints, models, formulas) and "source code" are themselves export-controlled items. They can be exported intangibly by email, cloud access, oral disclosure, or visual inspection, not only by physical shipment.

- Identify all forms in which controlled technology is released: documentation, source-code repositories, design reviews, technical support, training, and access to production systems.
- A release of technology to a foreign person located in the country of origin is itself an export (deemed export) to that person's nationality.

### Apply encryption controls carefully

Encryption and cryptographic items are controlled under nearly all regimes, with specific entries for mass-market, open-source, and administrative-use exemptions. The classification of an encryption item depends on its algorithm, key length, and end-use function, and often requires a classification request or annual self-classification report to the authorities.

- Do not assume "standard encryption" is exempt; many items require a license or notification even for commercial use.
- Open-source encryption may still trigger notification or review obligations.

### Assess end-use, end-user, and catch-all controls

Even uncontrolled items may require a license under "catch-all" controls if the end-use or end-user raises proliferation, military, or human-rights concerns. End-use and end-user screening (against denied-party, entity, and military-end-user lists) is required regardless of classification.

- A "no license required" classification does not dispense with end-user screening or catch-all analysis.
- Red flags (unusual purchasing patterns, reluctance to provide end-use information, shipping through intermediaries) trigger a duty to inquire further.

### Determine license requirements and exception eligibility

If the item is controlled, determine whether a license is required for the specific destination, end-use, and end-user, and whether a license exception applies. Exceptions are narrow, conditional, and recordkeeping-heavy; misuse is a violation.

- Document the specific license-exception citation and the conditions met, and retain records for the statutory period.
- When in doubt, apply for a license rather than stretch an exception.

### Maintain classification records and an ECCN/register

Maintain a product-classification register tying each product, software release, and technology set to its classification, rationale, and review date. Re-classify when parameters change.

- A classification is a point-in-time determination; a firmware update or feature addition can change it.

## Common Traps

### Classifying by product name or marketing category

Control lists are parameter-driven. A product name like "industrial sensor" tells you nothing; the sensing range, accuracy, and radiation-hardening determine control. Classify from the technical specifications.

### Missing intangible technology transfers

The most common enforcement risk is the release of controlled technology by email, shared repository, or technical support to a foreign person, treated as an export. Engineering and support teams often export technology daily without realizing it. Map all technology-release channels.

### Assuming encryption is exempt because it is "standard"

Mass-market and administrative exemptions exist but have conditions (algorithm, key length, function, end-use). Many commercial encryption products require classification requests, notifications, or licenses. Default to classifying, not assuming.

### Ignoring re-export and de minimis rules

Items containing US-origin controlled content above the de minimis ratio remain subject to US EAR worldwide, even when re-exported between third countries. Assuming "we bought it locally, so it's ours to resell" ignores re-export controls. Trace origin.

### Treating "no license required" as "no compliance required"

Even when no license is required, the exporter must record the classification, screen the end-user, assess catch-all end-use, and retain records. The absence of a license does not remove the compliance program.

### Stale classifications

A classification done at product launch becomes stale after feature updates, parameter changes, or list revisions. Control lists are amended regularly; re-classify on a schedule and on material change.

### Stretching a license exception

License exceptions are narrow and conditional. Using an exception to avoid a license-review timeline, when the conditions are not fully met, is a violation with significant penalties. When conditions are uncertain, apply for a license.

## Self-Check

- For each item, have I identified all controlling jurisdictions, including re-export and de minimis reach of origin-country controls?
- Is the classification based on documented technical parameters matched to the specific control-list entry, with the rationale and evidence recorded?
- Have I identified all channels through which controlled technology or source code may be released intangibly, and controlled them as exports?
- For encryption items, have I determined the correct classification and any notification, classification-request, or licensing obligation?
- Have I screened the end-user against denied-party, entity, and military-end-user lists, and assessed catch-all end-use red flags, regardless of classification?
- Where relying on a license exception, have I documented the specific citation, the conditions met, and the recordkeeping required?
- Is there a product-classification register with review dates, and a process to re-classify on material change or list revision?
- Have I flagged that classifications are technical and jurisdiction-specific and must be confirmed with the empowered-official or qualified trade-compliance counsel?
