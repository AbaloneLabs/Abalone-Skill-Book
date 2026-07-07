---
name: personalization_ethics.md
description: Use when the agent is designing personalization, recommendations, inferred preferences, adaptive content, profiling, or any feature that uses user data to shape what a person sees, and must weigh privacy, fairness, manipulation, transparency, and user autonomy.
---

# Personalization Ethics

Personalization is not neutral. When a product uses data about a person to decide what they see, what is recommended, what is hidden, and what is emphasized, it exercises power over that person's attention, choices, and self-image. Ethical personalization is not personalization that avoids harm by accident. It is personalization designed with an explicit understanding of what can go wrong for the user and for affected groups, and with safeguards that make the system worthy of trust.

The judgment problem is that personalization creates value and risk at the same time. The same data that lets a product surface relevant content can reveal sensitive facts the user did not consent to share. The same model that recommends useful options can narrow a user's world, amplify bias, or steer decisions in ways the user would not endorse if they understood them. The agent's job is to design personalization that captures the value while making the risks visible, bounded, and overridable.

Use this skill before designing or reviewing recommendation engines, inferred preferences, adaptive feeds, personalized search, dynamic pricing, targeted content, profile-based defaults, or any system that learns from user behavior. The goal is to prevent the agent from shipping personalization that is technically impressive but ethically careless, opaque, discriminatory, manipulative, or impossible for the user to inspect or escape.

## Core Rules

### Name The Sensitive Inferences Before You Use Them

Many useful personalization signals are also sensitive. Location, purchases, health searches, financial behavior, reading history, social connections, and even inferred attributes like pregnancy, sexuality, religion, or financial distress can be derived from seemingly innocuous data. Before building on a signal, name what it could reveal.

For each signal, ask:

- could it reveal health, sexuality, religion, politics, immigration status, financial hardship, or other protected or stigmatized attributes?
- could it become sensitive in combination with other signals?
- could it embarrass or endanger the user if exposed, breached, or shown to others?
- is the inference accurate enough to act on, and what happens when it is wrong?

Sensitive inferences are not forbidden, but they require stronger consent, tighter access control, shorter retention, clearer explanation, and an easier opt-out. Treat a derived sensitive attribute with the same care as data the user typed directly.

### Separate Personalization That Helps The User From Personalization That Targets Them

Personalization can serve the user or serve the system's goals against the user's interest. The same technique feels very different depending on whose side it is on.

Helps the user:

- surfacing content they already wanted;
- reducing repetitive configuration;
- adapting to accessibility needs;
- respecting stated preferences;
- protecting them from irrelevant noise.

Targets the user:

- exploiting cognitive biases to extend sessions;
- steering toward options more profitable for the platform;
- using vulnerability signals to adjust pricing or terms;
- narrowing choices to manipulate perceived options;
- personalizing persuasion rather than information.

Be honest about which kind a feature is. If a personalization feature primarily benefits the platform at the user's expense, the ethical path is to redesign it, limit it, or not ship it, regardless of how well it performs on engagement metrics.

### Make Inference Transparent And Correctable

Users cannot trust personalization they cannot see. If the system infers interests, builds a profile, or ranks content based on learned behavior, the user should be able to learn what the system thinks it knows and correct it.

Provide, where feasible:

- a view of inferred interests or attributes;
- the reason a piece of content was recommended or deprioritized;
- the ability to remove or correct a wrong inference;
- the ability to say a recommendation was not useful and have it matter;
- a way to reset or clear the learned profile.

Transparency is not a wall of technical detail. It is enough plain-language explanation for the user to form a mental model of why the system behaves as it does and to intervene when it is wrong. A recommendation the user cannot challenge is a recommendation the user cannot trust.

### Give The User A Meaningful Escape

Personalization should be opt-out-able, not inescapable. Users differ in how much they want their past to shape their present. Some want a tightly tailored experience; others want a fresh, unprofiled view; others want personalization off entirely.

Provide:

- a non-personalized default or fallback experience that is still good;
- a way to turn personalization off without breaking core features;
- a way to use the product without an account or history where feasible;
- a way to start fresh without losing access.

If the only good experience is the personalized one, and the only way to escape personalization is to stop using the product, the design is coercive. A user who opts out should still get a coherent, usable product.

### Watch For Discriminatory And Exclusionary Outcomes

Personalization trained on historical data can reproduce and amplify existing inequities. Recommendations, rankings, dynamic pricing, eligibility, and targeted content can systematically disadvantage people by race, gender, age, disability, language, location, or economic status, even when those attributes are not explicit inputs.

Before shipping, consider:

- does the model or rule produce different outcomes for different groups?
- could it exclude, overcharge, under-serve, or stereotype any group?
- does it depend on proxies for protected attributes?
- what is the fallback when data is sparse for a user or group?
- who is harmed most when the personalization is wrong?

Design for the users with the least data and the most at stake, not only the typical user. Test outcomes across plausible subgroups, and build in monitoring rather than assuming fairness will hold.

### Bound Persuasion And Vulnerability-Based Targeting

Some personalization is designed to persuade: ads, prompts, upgrade offers, reminders. Persuasion becomes exploitative when it targets moments or attributes of vulnerability: grief, financial stress, illness, addiction, low literacy, or emotional distress. Designing to maximize conversion in those moments can cause serious harm.

Set explicit limits:

- do not target based on inferred vulnerability without strong safeguards;
- do not time persuasive content to exploit distress;
- do not use addiction-like reinforcement patterns against users who have signaled they want less;
- provide friction and cooling-off for high-stakes decisions;
- honor expressed preferences to reduce contact or intensity.

When in doubt, choose the design that would still feel acceptable if the user understood exactly how and why they were targeted.

### Minimize, Protect, And Bound The Data That Powers Personalization

Ethical personalization respects the data it depends on. Collect what is necessary, protect it, retain it only as long as it serves the user, and give the user control over its life cycle.

Practice data minimization:

- collect the least data that achieves the personalization goal;
- avoid joining datasets in ways that create new sensitive inferences;
- prefer on-device or ephemeral processing where feasible;
- set retention limits and delete or anonymize on schedule;
- scope access tightly and log unusual access.

A personalization feature that requires hoarding sensitive data indefinitely is usually not worth the risk to the user. Prefer designs that achieve the value with less data, and be willing to ship a weaker feature to protect the people it serves.

## Common Traps

### Optimizing For Engagement And Calling It Personalization

Features tuned to maximize time-on-product are often framed as relevance. If the metric being optimized is attention rather than user-stated value, the personalization is likely serving the platform, not the person.

### Inferring Sensitive Attributes From Innocuous Data

Seemingly neutral signals (purchases, searches, location, app use) can reveal pregnancy, illness, sexuality, or financial trouble. Treating these as ordinary inputs exposes users to harm and the product to legal and reputational risk.

### Opaque Recommendations The User Cannot Challenge

When users cannot see why something was shown or removed, they cannot correct errors, build trust, or protect themselves from manipulation. Opacity feels neutral but shifts all risk onto the user.

### Personalization That Punishes Opt-Out

If turning personalization off produces a degraded, ad-heavy, or broken experience, the opt-out is not real. Users who decline profiling should not be penalized for protecting themselves.

### Amplifying Bias Through Historical Data

Models trained on past behavior inherit past inequities. Without explicit fairness checks, personalization can systematically under-serve or overcharge the users with the least power and the most to lose.

### Targeting Vulnerability Because It Converts

Moments of distress convert well. That is precisely why targeting them is dangerous. Conversion rate is not ethical justification.

### Retaining Data Indefinitely Because It Might Be Useful

Infinite retention creates breach exposure, profiling risk, and user distrust. The possibility of future value does not outweigh the present risk to the people whose data it is.

## Self-Check

- [ ] Sensitive inferences (health, sexuality, religion, finances, vulnerability) have been identified and given stronger consent, access, retention, and opt-out treatment.
- [ ] Each personalization feature is assessed honestly as user-serving versus platform-serving, and platform-serving features targeting the user are redesigned, limited, or cut.
- [ ] Users can see what the system inferred about them, why content was recommended or hidden, and can correct or reset wrong inferences.
- [ ] A meaningful non-personalized or opt-out path exists and does not degrade core features or punish the user for using it.
- [ ] Discriminatory and exclusionary outcomes have been considered across subgroups, including users with sparse data and high stakes, with monitoring in place.
- [ ] Persuasive and targeted features have explicit limits against exploiting vulnerability, distress, or addiction-like reinforcement.
- [ ] Data collection is minimized, access is scoped and logged, retention is bounded, and the feature achieves its value without hoarding sensitive data indefinitely.
- [ ] The design would still feel acceptable to a user who understood exactly how and why they were being targeted.
- [ ] Recommendations and rankings have a fallback when data is missing or wrong, and the fallback is a coherent experience rather than an empty or broken one.
- [ ] The personalization respects expressed user preferences, including requests for less, and does not override them in pursuit of engagement.
