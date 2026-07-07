---
name: threat_modeling_and_attack_surface.md
description: Use when the agent is threat-modeling a system before or during design, identifying and ranking security risks across trust boundaries, mapping data flows for adversarial abuse, running STRIDE or similar structured analyses, defining and shrinking the attack surface, deciding which controls are proportionate to which threats, or reviewing an architecture to find what an attacker could exploit that the designers did not anticipate. Also covers the failure mode of shipping a feature that is secure-by-checklist but vulnerable-by-design because no one asked how it could be abused.
---

# Threat Modeling And Attack Surface

Most security failures are not exotic zero-days; they are ordinary features that no one examined from an adversarial perspective. A system can pass every item on a hardening checklist and still be vulnerable-by-design, because the checklist confirms that known controls exist but never asks the question that matters: *given this architecture, what can an attacker actually do, and did we accept that risk on purpose or by accident?* Threat modeling is the structured act of answering that question before code is written or before a change ships. It forces the team to look at the system as an opponent would — tracing data flows, finding the places where trust changes, identifying the assets that matter, and enumerating the ways each component could be abused — rather than as a set of features to implement.

Agents tend to conflate threat modeling with generic security review or with running a scanner. It is neither. A scanner finds known-bad patterns; threat modeling finds design-level weaknesses that no scanner can see, because they are properties of how components are arranged, where trust boundaries are drawn, and which assumptions are implicit. The harm of skipping it appears as vulnerabilities discovered late (expensive to fix, because they are architectural), as controls applied to the wrong threats (heavy investment defending a low-likelihood path while a high-likelihood path is ignored), and as a false sense of security from passing audits that measure process rather than risk. The judgment is to model threats early, structure the analysis so nothing is assumed safe by default, rank threats by real risk rather than by ease of enumeration, and treat the attack surface as something to be deliberately shrunk rather than merely monitored.

## Core Rules

### Model Early, On The Architecture, Not The Code

Threat modeling is highest-leverage before implementation, when design decisions are still cheap to change. Modeling existing code is useful but mostly finds problems too late to fix cheaply. The input to threat modeling is an architecture: the components, the data flows between them, the trust boundaries, and the assets. If you do not have a diagram of these, you are not ready to model.

- **Start from a data-flow or component diagram with explicit trust boundaries.** Draw where data crosses from a less-trusted context to a more-trusted one (client to server, public subnet to private, tenant A to tenant B). Every trust boundary crossing is a candidate threat location.
- **Model on the design as it will actually be built, including third parties and managed services.** A component you do not own is still part of the attack surface; an external identity provider, a webhook receiver, a CDN, or a SaaS dependency all carry threats you must account for.
- **Re-model when the architecture changes materially.** Adding a new public endpoint, a new data sharing path, a new integration, or a new class of user changes the threat profile. Threat models are living documents tied to architecture versions, not one-time artifacts.

### Identify Assets And Adversaries Before Enumerating Threats

A threat is only meaningful relative to an asset and an adversary. Enumerating threats without first naming what you are protecting and from whom produces a long, unranked list that wastes effort.

- **Name the assets and their sensitivity.** Customer PII, credentials and secrets, payment data, internal intellectual property, the integrity of critical configurations, the availability of a revenue-generating service. Different assets warrant different protection levels.
- **Name the adversaries and their capabilities.** An anonymous internet attacker, an authenticated but unprivileged user, a malicious insider with partial access, a compromised dependency, a compromised adjacent tenant. Each adversary sees a different attack surface.
- **Consider the compromised-component adversary.** The most dangerous and most-ignored adversary is one of your own components after it has been compromised. Ask what a single breached service can reach, and whether blast radius is contained.

### Use A Structured Method To Avoid Blind Spots

Unstructured brainstorming finds the obvious threats and misses the systematic ones. A structured method forces coverage across threat categories so nothing is assumed safe by default. STRIDE (Spoofing, Tampering, Repudiation, Information disclosure, Denial of service, Elevation of privilege) is the most common framing because it maps cleanly to properties an architecture should guarantee.

- **Walk each component and each data flow against each category.** For every trust boundary crossing, ask: can an attacker spoof an identity here, tamper with data in transit or at rest, repudiate an action, disclose information they should not see, deny service, or gain elevated privilege? The structure prevents stopping at the first plausible threat.
- **Prefer the method that fits the system.** STRIDE suits component-and-flow architectures; attack trees suit a single high-value asset; LINDDUN suits privacy-focused systems. The method matters less than that one is applied systematically and completely.
- **Capture assumptions explicitly.** Every "we assume the load balancer terminates TLS" or "we assume the queue enforces ordering" is a load-bearing assumption. Write them down so they can be challenged, because broken assumptions are where architectural vulnerabilities live.

### Rank By Real Risk, Not By Enumeration Ease

A long list of threats is useless if the team defends the easy-to-describe ones and ignores the high-impact ones. Rank by the product of likelihood and impact, adjusted for the controls already in place, and prioritize accordingly.

- **Estimate likelihood realistically, including the adversary's effort and existing controls.** A threat that requires an authenticated insider and a chained exploit is less likely than one exploitable by an anonymous request; a threat already mitigated by an existing control ranks lower than one that is not.
- **Weight impact by asset sensitivity and blast radius.** A threat that exposes public marketing copy is not equivalent to one that exposes customer PII or that allows privilege escalation across tenants. Rank by what actually matters to the organization.
- **Re-rank when controls change.** Adding a control reduces the risk of the threats it addresses; removing one or adding a feature can raise risk elsewhere. Risk ranking is not static.

### Deliberately Shrink The Attack Surface

The attack surface is the set of points an attacker can reach and influence. The safest feature is the one that does not exist; the next safest is the one an attacker cannot reach. Reducing attack surface is almost always more effective than adding detection on top of a large surface.

- **Prefer removing exposure to adding controls.** An endpoint that does not exist cannot be attacked; a port that is not open cannot be scanned; a privilege that is not granted cannot be abused. Default to least exposure.
- **Minimize trust boundary crossings and the data that crosses them.** Every crossing is a place where input must be validated and where trust changes. Fewer crossings, and less sensitive data crossing them, means fewer places to get it wrong.
- **Apply least privilege to every component and identity.** Each service, token, and user should hold the minimum privilege required to function, so that a compromise of any single component has bounded blast radius. A service with broad permissions is a single point of catastrophic failure.
- **Question every new public input.** Any new endpoint, parser, file upload, webhook, or import path is new attack surface. Treat its addition as a threat-modeling event, not a routine feature.

### Choose Controls Proportionate To The Threat

Controls (authentication, authorization, encryption, input validation, rate limiting, logging, isolation) are selected to mitigate specific, ranked threats. Applying controls without mapping them to threats produces security theater — heavy controls on low-risk paths and gaps on high-risk ones.

- **Map each control to the threat it mitigates.** If you cannot say which threat a control addresses, it may be cargo-culted. Conversely, if a high-ranked threat has no control, that is the gap to close.
- **Preventive over detective where feasible.** A control that prevents a threat (input validation that rejects malformed input, authorization that denies access) is stronger than one that detects it after the fact (logging an alert). Detection matters, but it is second-line.
- **Layer controls for high-risk paths.** For threats that are both likely and high-impact, do not rely on a single control; combine prevention, detection, and containment so that the failure of one does not expose the asset.

### Treat Implicit Assumptions And Defaults As Threats

Architectural vulnerabilities usually live in assumptions that were never written down: that a header is trustworthy because it came through a proxy, that internal services are safe because they are not public, that a token's audience is checked because the library issues tokens. These defaults are exactly what an attacker exploits.

- **Challenge every trust assumption, especially convenient ones.** "The gateway validates input, so the service does not need to" fails when the gateway is bypassed or misconfigured. Validate at the trust boundary that enforces it, and do not rely on upstream guarantees you do not control.
- **Assume components will be compromised.** Design so that a single compromised service cannot pivot to everything. Network segmentation, scoped credentials, and mutual isolation limit blast radius under compromise.
- **Examine what happens when a dependency or default fails.** A library that silently downgrades, a fallback path that skips authorization, a feature flag that opens a broader surface — these failure modes are threats that structured modeling should surface.

## Common Traps

- **Threat modeling as a one-time compliance artifact.** Producing a document once to satisfy a process, then never updating it as the architecture evolves, so the model no longer reflects the real system. Tie models to architecture versions and re-model on material change.
- **Enumerating threats without ranking.** A long unranked list that causes the team to defend easy-to-describe low-risk threats while high-risk ones go unaddressed. Rank by likelihood times impact, adjusted for existing controls.
- **Modeling only the happy path of data flow.** Tracing how legitimate data moves while ignoring how an attacker could inject, replay, tamper, or exceed expected volumes. Model adversarial inputs, not just intended ones.
- **Ignoring the compromised-component adversary.** Assuming your own services are trustworthy, so that a single breach allows lateral movement to everything. Model what a compromised internal component can reach.
- **Relying on upstream guarantees you do not control.** Trusting a header, token, or validation because "the gateway handles it," then being exploited when the gateway is bypassed or misconfigured. Validate at the boundary you own.
- **Confusing scanner output with threat modeling.** Treating dependency scans and SAST results as the complete security picture, while design-level weaknesses (broken trust boundaries, missing isolation, overbroad privilege) go unexamined. Scanners find known-bad patterns; threat modeling finds architectural flaws.
- **Adding controls without mapping to threats.** Applying encryption, logging, or rate limiting because a checklist demands it, without knowing which threat each control mitigates, leaving the actual high-risk path uncontrolled. Map controls to ranked threats.
- **Expanding attack surface without a threat-modeling event.** Adding a public endpoint, parser, or integration as a routine feature, never asking what an attacker could do with the new input. Treat new public inputs as threat-modeling triggers.
- **Assuming internal equals trusted.** Treating internal network location or service identity as proof of trustworthiness, so that an attacker who reaches the internal network faces no further controls. Apply least privilege and defense in depth regardless of network position.
- **Heavy controls on low-risk paths, gaps on high-risk ones.** Investing in elaborate controls for visible-but-low-impact threats while the high-impact path (tenant isolation, privilege escalation, secret access) is underdefended. Let ranking drive investment.

## Self-Check

- Did I threat-model on the architecture (components, data flows, trust boundaries, assets) before or during design, rather than only on existing code after the fact?
- Have I explicitly named the assets (and their sensitivity) and the adversaries (and their capabilities), including the compromised-component adversary, before enumerating threats?
- Did I apply a structured method (e.g., STRIDE) systematically across every component and trust boundary crossing, rather than stopping at the first plausible threat?
- Are threats ranked by realistic likelihood times impact, adjusted for existing controls, and is investment prioritized by that ranking rather than by ease of description?
- Have I deliberately shrunk the attack surface — removing unnecessary exposure, minimizing trust boundary crossings and the data crossing them, applying least privilege to every component and identity — rather than only adding detection on top of a large surface?
- Is each control mapped to a specific threat it mitigates, with preventive controls preferred over detective ones and layered controls on high-risk paths?
- Have I challenged the implicit assumptions and defaults — trusted headers, internal-equals-trusted, library defaults, fallback paths — that architectural vulnerabilities usually hide in?
- Is the threat model tied to the architecture version and scheduled for re-modeling on material change (new public endpoint, new integration, new data sharing path, new user class)?
- Did I verify the highest-risk cases — a compromised internal component's blast radius, a trust boundary crossing with no validation, an overprivileged identity, a new public input never threat-modeled — rather than only the clean checklist-compliant path?
