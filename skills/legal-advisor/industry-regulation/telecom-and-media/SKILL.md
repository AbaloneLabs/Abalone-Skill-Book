---
name: telecom_and_media.md
description: Use when the agent is advising on telecommunications, broadcasting, over-the-top media, or online-platform regulation, covering operating licenses, spectrum and numbering, network access and interconnection, net neutrality, content moderation and liability, media-pluralism and concentration rules, privacy and electronic-communications rules, and competition enforcement in digital and communications markets.
---

# Telecom and Media Regulation

Telecommunications and media regulation governs who may operate networks and distribute content, how scarce resources (spectrum, numbering) are assigned, how networks interconnect, what obligations attach to gatekeepers and significant market players, and what rules constrain the content carried. The sector sits at the intersection of economic regulation (competition, access, pricing), social regulation (pluralism, protection of minors, electoral integrity), and data regulation (privacy, electronic-communications confidentiality).

The judgment problem is that the regulatory perimeter shifts with technology. Voice-over-IP, messaging, streaming, social platforms, app stores, search, and generative-AI services are pulled in and out of telecom and media rules as legislatures update statutes. Agents often apply the wrong frame: treating an OTT service as if it were a traditional broadcaster, or assuming a platform has no content obligations because it is "just a conduit." Getting the frame wrong leads to licensing breaches, spectrum or numbering violations, content takedown failures, and competition or platform-regulation enforcement.

This skill is advisory only. Telecom and media rules are jurisdiction-specific and change frequently. Confirm licensing, content, and competition conclusions with qualified counsel in each relevant jurisdiction.

## Core Rules

### Determine whether the service is inside the electronic-communications or broadcasting perimeter

Perimeter analysis is the first step. Is the service an electronic-communications network or service, a broadcasting or audiovisual media service, an on-demand/OTT service, a video-sharing platform, an intermediary service, or a none-of-the-above information service? Definitions are increasingly functional and may capture services that do not own infrastructure.

- A strong analysis tests the service against each statutory definition and any recent reform (e.g., EU EECC, EU Digital Services Act, EU Digital Markets Act, national OTT rules).
- A weak analysis assumes "we don't own a network, so we're not regulated" or "we're an internet company, so broadcasting rules don't apply."
- Some services are regulated at multiple layers simultaneously (e.g., a streaming service may be an audiovisual media service for content rules and an intermediary service for liability).

### Address licensing, spectrum, and numbering obligations

Operating physical networks or providing numbered communications usually requires a general or individual authorization, with rights and obligations. Spectrum use requires assignment (auction, administrative, license-exempt with conditions) and compliance with technical conditions to avoid harmful interference. Numbering resources are allocated and portability rules may apply.

- Confirm whether the planned service needs spectrum, numbering, or interconnection rights, and the authorization class.
- Unlicensed or license-exempt bands still carry power, duty-cycle, and coexistence conditions; "unlicensed" is not "unregulated."

### Apply access, interconnection, and net-neutrality rules

Operators with significant market power, designated gatekeepers, or providers of internet-access services may owe access, interconnection, non-discrimination, and transparency obligations. Net-neutrality rules restrict blocking, throttling, and paid prioritization of internet traffic in many jurisdictions.

- Identify whether the provider is subject to SMP obligations, gatekeeper duties, or general net-neutrality rules, and design product features accordingly.
- Zero-rating, specialized services, and traffic-management practices are often permitted only within defined limits.

### Treat content obligations by service category

Content rules differ sharply by category. Traditional broadcasters face the strictest rules (licensing, quotas, advertising limits, protection of minors, impartiality). Linear and on-demand audiovisual services face a lighter but real set of rules. Intermediary platforms generally benefit from conditional liability exemptions but must operate notice-and-action, transparency reporting, and (for very large platforms) risk assessment and systemic-risk mitigation.

- Do not assume a platform exemption removes all content responsibility; exemptions are conditional on neutral conduct, notice-and-action, and no active promotion of illegal content.
- Election-period, minor-protection, and terrorist-content rules may impose rapid takedown duties independent of general liability rules.

### Map privacy and electronic-communications confidentiality

Telecom and media services sit atop strict rules on confidentiality of communications, metadata, traffic and location data, retention, lawful-intercept capabilities, and cookies/electronic-marketing consent. These rules often apply in addition to general data-protection law.

- Determine whether the service is a provider of electronic-communications services triggering sectoral confidentiality and intercept obligations.
- Metadata retention and access by authorities are politically and legally sensitive; design minimally and document the legal basis.

### Address media-pluralism, concentration, and must-carry

Ownership concentration in media is scrutinized for pluralism and democratic discourse, sometimes beyond general merger control. Must-carry rules may require carriage of certain channels, and prominence rules may require discoverability of public-service or general-interest content.

- Media M&A may require separate media-pluralism clearance in addition to general competition clearance.
- Platform prominence and "must-offer/must-carry" rules affect product design for set-top boxes and smart TVs.

### Identify gatekeeper and platform-regulation duties

Designated gatekeepers or very large online platforms face additional duties: self-preferencing bans, interoperability, data portability, sideloading, business-user protection, advertising-transparency, and risk-based content moderation. These duties can override commercial product decisions.

- Check whether the service meets designation thresholds and prepare for the obligations before designation, not after.
- Compliance may require architectural changes (APIs, data separation) that take time; start early.

### Coordinate with competition enforcement

Telecom and digital markets are intensively supervised by competition authorities for exclusionary conduct, abusive pricing, mergers, and information exchange. Sector regulators and competition authorities may both act.

- Treat coordination between product, regulatory, and antitrust teams as continuous, not transactional.

## Common Traps

### "We are an OTT service, so telecom rules don't apply"

OTT voice, messaging, and messaging-based payments are increasingly brought inside electronic-communications or financial rules. Number-independent services may still trigger emergency-access, numbering, or intercept obligations. Confirm the perimeter rather than relying on the label.

### Assuming platform immunity is absolute

Conditional liability exemptions for intermediaries require neutral conduct, effective notice-and-action, and no active knowledge of illegality. Recommending, curating, or monetizing illegal content can forfeit the exemption. Exemptions also do not apply to primary obligations such as transparency reporting or systemic-risk duties.

### Treating unlicensed spectrum as unregulated

License-exempt bands carry power, emission, and coexistence limits. Interference with licensed services can trigger enforcement and shutdown. Design hardware to the regional technical rules, not a global default.

### Ignoring sectoral confidentiality for communications metadata

Communications metadata (location, traffic, contacts) is often protected separately from general personal data, with stricter retention and access limits. Reusing metadata for analytics or advertising without a valid basis is a frequent enforcement target.

### Applying broadcasting content rules to platforms, or vice versa

Mismatched framing leads to either over-compliance (treating a platform as a broadcaster and censoring lawful content) or under-compliance (treating a broadcaster as a platform and skipping pluralism and quota rules). Classify the service correctly first.

### Net-neutrality workarounds

Zero-rating, fast lanes, and bundled specialized services are often restricted. Designing a product around a perceived loophole can attract enforcement and consumer backlash. Test against the jurisdiction's specific rules.

### Media M&A cleared only on competition grounds

A merger that passes general competition review may still fail media-pluralism clearance or require structural remedies. Engage media-regulatory counsel early in M&A planning.

### Prominence and must-carry treated as UI choices

Discoverability of public-service or must-carry content, and equal prominence for accessibility services, are increasingly mandated. Treat them as regulatory requirements in product design, not aesthetic choices.

## Self-Check

- Have I classified the service against the electronic-communications, audiovisual/broadcasting, on-demand, platform/intermediary, and information-service definitions, and recorded the basis for each jurisdiction?
- If the service uses spectrum, numbering, or interconnection, have I identified the authorization class and the technical and operational conditions?
- Have I determined whether net-neutrality, access, interconnection, or gatekeeper duties apply to the planned product features (zero-rating, prioritization, self-preferencing)?
- For content distribution, have I mapped the correct content obligations for the service category, including minor-protection, electoral, and rapid-takedown duties?
- Have I identified sectoral electronic-communications confidentiality and lawful-intercept obligations in addition to general data-protection law?
- For any media M&A or platform-designation scenario, have I considered pluralism, must-carry, prominence, and gatekeeper duties separately from general competition analysis?
- If relying on an intermediary liability exemption, have I confirmed the conditions (neutral conduct, notice-and-action, transparency, no active promotion) are met operationally?
- Have I flagged that perimeters and platform rules change frequently and that conclusions must be confirmed with qualified counsel in each jurisdiction?
