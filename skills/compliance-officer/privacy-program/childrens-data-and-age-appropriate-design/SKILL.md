---
name: childrens_data_and_age_appropriate_design.md
description: Use when the agent is building services that may be used by children, deciding age assurance methods, applying the children's code or age-appropriate design, handling parental consent under COPPA, or reviewing marketing and profiling directed at minors.
---

# Children's Data And Age-Appropriate Design

Children merit heightened protection under virtually every privacy regime, but the obligations differ by jurisdiction, age threshold, and service type. Treating children's data like ordinary personal data is a serious error: consent rules change, profiling is restricted, marketing is constrained, and design defaults must favor the child. The hardest part is not knowing the rules; it is detecting that a child is present and applying the right posture across a service that may be mixed-use.

Use this skill before launching a service likely to attract children, adding features such as personalization or social sharing, configuring age gates, or reviewing advertising and profiling for a young audience. The goal is to make the agent identify the applicable regime, choose a defensible age assurance approach, and apply design defaults that protect minors rather than exploit their developmental stage.

## Core Rules

### Identify The Applicable Regime And Age Threshold

Children's privacy rules vary by law and by the age at which a person is treated as a child. Do not assume one global threshold.

Map the regimes that apply:

- COPPA in the United States: parental consent generally required for under-13s, with knowledge of child-directed status triggering duties;
- GDPR Article 8: information society services offered to a child require consent from the holder of parental responsibility below the national age (13 to 16 depending on member state);
- UK Age Appropriate Design Code (Children's Code): applies to online services likely to be accessed by children under 18;
- California Age-Appropriate Design Code: coverage for businesses whose services are likely to be accessed by children;
- PIPL and LGPD: special provisions for minors, generally requiring guardian consent.

Determine which thresholds apply to your service in each market, and apply the strictest where a single global posture is operationally necessary.

### Detect Whether The Service Is Likely To Be Accessed By Children

You cannot apply children's protections if you do not know children are present. The duty often attaches when the service is "likely to be accessed" by children, which is broader than "directed to" children.

Assess likelihood using:

- subject matter, themes, and visual design;
- marketing channels and audience targeting;
- third-party reports, app store categorization, and reviews;
- known user demographics and account data;
- presence of features attractive to children such as games, avatars, stickers, or influencer content.

Document the assessment. If children are likely to access the service, the design code obligations attach even if the service is not primarily for children.

### Choose A Proportionate Age Assurance Method

Age assurance sits on a spectrum from declaration to robust verification. The method must be proportionate to the risk of the processing and the rights at stake.

Consider, from least to most intrusive:

- self-declaration of age at account creation;
- age gating that blocks features below a threshold;
- account holder attestation that a child is present;
- age estimation from behavior or voice;
- hard age verification using identity documents or credit checks.

Apply data minimization to the assurance itself: do not collect identity documents to gate a low-risk feature. Stronger assurance is justified for higher-risk processing such as targeted advertising, location tracking, or content that could harm minors.

### Obtain Verifiable Parental Consent Where Required

Under COPPA and GDPR Article 8, processing a child's data generally requires consent from a parent or guardian, and that consent must be verifiable.

Acceptable COPPA mechanisms include:

- signed consent form returned by mail, fax, or scanned document;
- credit or debit card plus real-time verification;
- toll-free call with trained staff;
- videoconference with trained staff;
- government ID checked against a database, provided the ID is deleted after verification.

Avoid mechanisms that collect more data than necessary or that depend on the child's own device. Keep records of how and when parental consent was obtained.

### Apply Age-Appropriate Design Defaults

The children's codes require defaults that protect children, not defaults that maximize data collection. The default state of a child's account should be the safest, not the most permissive.

Apply high-privacy defaults:

- geolocation off by default;
- profiling for advertising off by default;
- targeted advertising based on profiling off by default;
- nudge techniques that encourage children to weaken privacy disabled or restrained;
- visibility to other users restricted by default;
- data collection limited to what is necessary to deliver the core service.

Justify any default that departs from high privacy. The burden is on the service to show the less protective default is in the child's best interest.

### Restrict Profiling, Marketing, And Behavioral Advertising

Profiling children for behavioral advertising, or targeting them with advertising based on inferred characteristics, is heavily restricted or prohibited.

Restrict:

- behavioral advertising to children;
- profiling that has legal or similarly significant effects on a child;
- use of nudge techniques to drive children to provide more data or weaken settings;
- recommender systems that amplify harmful content to minors;
- commercial messages that exploit children's inexperience or credulity.

Contextual advertising that does not rely on profiling is generally a safer alternative.

### Provide Clear, Child-Suitable Information

Notices and choices must be understandable to the child at their developmental stage, in addition to information provided to parents.

Provide:

- concise, age-appropriate explanations of what data is collected and why;
- prominent, simple tools to exercise rights and change settings;
- bite-sized just-in-time prompts before data collection;
- information reinforced at points where significant choices are made.

Do not rely solely on a long privacy policy that a child will not read.

### Handle Special Category And Sensitive Data Cautiously

Children's data is often sensitive by context even when not formally special category: location, biometrics, photos, school records, and health-adjacent data all warrant extra care. Apply enhanced safeguards, minimize collection, and avoid secondary uses such as training shared models on children's inputs without a strong, documented basis.

## Common Traps

### Assuming The Service Is Not For Children

Many services used heavily by children, such as games and social platforms, were not designed for them. The "likely to be accessed" test captures these services regardless of intent.

### Using A Weak Age Gate For High-Risk Processing

A simple checkbox age gate is defensible for low-risk services but inadequate when the service exposes children to profiling, location tracking, or harmful content.

### Treating Parental Consent As A One-Time Event

Parental consent must be verifiable and, where the processing changes materially, refreshed. A one-time click is not verifiable consent.

### Defaulting To Personalized Advertising

Default-on behavioral advertising to children is a frequent enforcement target. The safer default is off, with contextual alternatives.

### Ignoring Nudge And Dark Pattern Risk

Designs that push children to share more data, weaken settings, or make in-app purchases are increasingly treated as unfair and unlawful.

### Collecting Identity Documents Without Deleting Them

Robust age verification can be necessary, but retaining the identity documents creates new risk. Delete verification data once the check is complete unless retention is justified.

### Forgetting That Children Grow Up

A child's account may need to transition to an adult account at the age of majority, with new choices offered. Failing to handle this transition can leave an adult locked into child settings or a child's data retained longer than appropriate.

## Self-Check

- Is the applicable children's regime identified per market, including COPPA, GDPR Article 8, the UK Children's Code, and California's Age-Appropriate Design Code?
- Has the service been assessed for whether it is likely to be accessed by children, with the assessment documented?
- Is the age assurance method proportionate to the risk of the processing, and does it minimize data collection?
- Where parental consent is required, is it verifiable under COPPA or GDPR Article 8, with records retained?
- Are high-privacy defaults applied for geolocation, profiling, and behavioral advertising?
- Is profiling for behavioral advertising to children restricted, with contextual alternatives used instead?
- Are notices and choices presented in child-suitable language at the point of collection?
- Are nudge techniques that exploit children's inexperience avoided?
- Is special category and sensitive data handled with enhanced safeguards and minimized collection?
- Is there a process to transition a child's account at the age of majority and to avoid indefinite retention of childhood data?
