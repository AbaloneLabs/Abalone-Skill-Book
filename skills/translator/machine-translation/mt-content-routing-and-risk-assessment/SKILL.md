---
name: mt_content_routing_and_risk_assessment.md
description: Use when the agent is deciding which content to route to machine translation versus human translation, assessing the risk of using MT on specific content types, applying content classification to MT routing, or determining where raw MT light post-editing full post-editing or human translation is appropriate based on stakes visibility and quality requirements.
---

# MT Content Routing And Risk Assessment

Machine translation is not a single setting applied uniformly to all content. It is a capability whose appropriate use depends entirely on what content it is applied to, and routing the wrong content to MT is how programs create their most expensive failures. High-stakes content routed to raw MT ships dangerous errors that look fluent. Brand-visible content routed to unreviewed MT damages reputation. Content with placeholders and concatenated structures routed to MT without checking produces broken products. At the same time, routing all content to human translation wastes budget on low-risk volume where MT would have been adequate. Content routing and risk assessment is the discipline of classifying content by the consequence of MT error and routing each class to the production method that balances cost, speed, and acceptable risk. This is the decision layer above the MT engine: even a good engine produces failures on the wrong content, and even a weak engine is acceptable on the right content. Agents who apply MT without classifying the content first are gambling with stakes they have not assessed.

Use this skill when deciding what content to route to MT, assessing MT risk on specific content types, applying content classification to routing, or determining the right production method per content class. The goal is to route content to the method that meets quality requirements at acceptable cost and risk, with high-stakes content protected and low-risk volume leveraged.

## Core Rules

### Classify Content By Consequence Of MT Error

Content routing begins with classification by the consequence of error if MT fails. Classify before routing.

High-consequence content includes legal obligations, medical instructions and dosages, safety warnings, financial figures and disclosures, regulatory compliance text, security and access controls, and any content where an MT error causes harm, liability, or product failure. Medium-consequence content includes procedural instructions, technical documentation, and user-facing guidance where MT errors cause confusion or rework but not harm. Low-consequence content includes internal communications, support knowledge bases, gisting and informational content, and large-volume text where MT errors are tolerable. The consequence class determines the acceptable production method.

Routing without classification treats a medical warning and an internal memo as equivalent, which they are not.

### Match Production Method To Consequence And Visibility

For each content class, match the production method to the consequence of error and the content's visibility. Do not route by convenience.

High-consequence content warrants human translation by qualified linguists, because MT error is unacceptable regardless of speed or cost benefit. Brand-visible and published content warrants MT with full post-editing, combining speed with publication quality, or human translation where brand voice is critical. Internal and informational content may warrant MT with light post-editing, accepting imperfect style for speed. Gisting content, where the goal is only to understand the gist, may warrant raw MT with no post-editing. Match the method to both consequence and visibility: low-consequence but brand-visible content may still warrant fuller handling, while high-consequence content never drops below human translation regardless of volume pressure.

### Assess MT Risk For Specific Content Characteristics

Beyond consequence class, specific content characteristics raise or lower MT risk. Assess them per content type.

Content with dense specialized terminology raises MT risk, because MT often renders such terms inconsistently or incorrectly. Content with idioms, humor, cultural references, or persuasive intent raises MT risk, because MT literalizes what should be adapted. Content with placeholders, variables, markup, and concatenated structures raises MT risk, because MT can break them. Content with numbers, dates, units, and entities raises MT risk, because MT can alter them silently. Content that is short and isolated, like UI strings, raises MT risk, because MT lacks context. Content that is long, general, and informational lowers MT risk. Assess these characteristics to refine the routing beyond the broad consequence class.

### Factor In Language Pair And Engine Reliability

MT quality varies by language pair and domain, and routing must account for where the engine is strong and where it is weak.

An engine strong in a domain and language pair may warrant lighter handling, while the same engine weak in another pair warrants heavier review or human translation. Assess engine reliability per language pair and domain using quality data, not assumptions. Some language pairs, especially those with less training data or greater structural distance from the engine's strong pairs, produce lower-quality MT and require more post-editing or human translation. Factor in that reliability changes over time as engines are tuned, so re-assess periodically.

Routing that ignores engine reliability per pair routes the same content identically across languages, over-handling strong pairs and under-protecting weak ones.

### Protect Confidential And Sensitive Content

MT routing must consider confidentiality, because routing sensitive content to an external MT engine can constitute a data breach. Assess data sensitivity before routing.

Content containing personal data, trade secrets, unpublished financial information, legal-privileged material, health information, or security-sensitive content must not be routed to external MT engines that retain or train on input. Use secure on-premises or contractually protected engines for sensitive content, or route to human translation under confidentiality controls. The convenience of MT never overrides the obligation to protect sensitive content, and a confidentiality failure causes harm no quality benefit can offset.

### Define Routing Rules And Automate Where Possible

Content routing should be systematic, not ad hoc per segment. Define routing rules and automate them where the content pipeline allows.

Define rules that map content classes, identified by metadata, content type, or domain tags, to production methods. Automate routing in the content management or translation management system so content flows to the right method without manual decision each time. Automation enforces consistency and prevents the human error of routing the wrong content to MT. Where automation is not possible, document the rules so routing decisions are consistent across the team.

Ad hoc routing, where each project decides anew, produces inconsistency and the inevitable routing of high-stakes content to MT under deadline pressure.

### Build In Review Gates Appropriate To The Route

Each route needs review gates proportional to its risk. A route with no gate ships unverified content.

Raw MT routes need at least sampling review to detect systematic failure. Light post-editing routes need review focused on meaning errors. Full post-editing routes need review against publication standards. Human translation routes need the standard review process. The gate depth matches the consequence class: high-consequence content needs the deepest gates regardless of production method, because even human translation can err. Define what each gate checks and who owns it.

A route without a gate is an assumption that the production method is infallible, which no method is.

### Re-Assess Routing As Content And Engine Evolve

Content and MT engines both evolve, and routing that was correct can become wrong. Re-assess periodically.

As content changes domain, visibility, or sensitivity, re-classify and re-route. As the MT engine is tuned, retrained, or replaced, re-assess its reliability per pair and domain and adjust routing. As quality data accumulates, use it to identify routes that under-perform and adjust them. Static routing decays, because the conditions that made it correct change.

## Common Traps

### Routing All Content To MT Uniformly

Applying MT without classification routes high-stakes content to a method whose errors are unacceptable.

### Routing By Convenience Or Cost Alone

Choosing MT for speed or cost on high-consequence content trades a small saving for large risk.

### Ignoring Content Characteristics That Raise MT Risk

Specialized terminology, idioms, placeholders, and isolated strings raise MT risk beyond the broad consequence class.

### Assuming Uniform Engine Quality Across Pairs

MT quality varies by language pair and domain; routing that ignores this over-handles strong pairs and under-protects weak ones.

### Routing Sensitive Content To External Engines

Sending confidential content to engines that retain or train on input constitutes a data breach regardless of quality benefit.

### Ad Hoc Routing Per Project

Routing decided anew each time is inconsistent and routes high-stakes content to MT under deadline pressure.

### Routes Without Review Gates

A route with no gate assumes the production method is infallible and ships unverified content.

### Static Routing That Never Re-Assesses

Content and engines evolve; routing that is never re-assessed decays and routes content to methods that no longer fit.

## Self-Check

Before approving MT content routing for a program or content batch, verify:

- Content is classified by consequence of MT error, with high-stakes content identified and protected.
- Production method per class matches consequence and visibility, with high-consequence content never below human translation.
- Content characteristics that raise MT risk, terminology, idioms, placeholders, numbers, isolated strings, are assessed and refine the routing.
- Engine reliability per language pair and domain is factored in, with weaker pairs receiving heavier handling.
- Confidential and sensitive content is routed only to secure engines or human translation, never to external engines that retain input.
- Routing rules are defined and automated where possible, with documentation where automation is not feasible.
- Review gates proportional to risk are defined for each route, with gate depth matching consequence class.
- Routing is re-assessed periodically as content, engine reliability, and quality data evolve.
- No high-stakes content is routed to raw or lightly handled MT, and no sensitive content is sent to an unsecured engine.
- Routing balances cost and speed against acceptable risk, protecting what matters while leveraging MT where it is safe.
