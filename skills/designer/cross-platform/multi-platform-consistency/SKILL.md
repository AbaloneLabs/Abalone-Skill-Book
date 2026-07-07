---
name: multi_platform_consistency.md
description: Use when the agent is designing for consistency across platforms such as web, iOS, Android, desktop, tablet, and wearables, balancing brand and interaction consistency against native platform conventions, deciding what must stay consistent versus what should adapt, managing a cross-platform design system, or resolving conflicts between platform guidelines and a unified product experience, and must avoid both rigid pixel-identical copying that ignores platform norms and loose inconsistency that fragments the product.
---

# Multi-Platform Consistency

Cross-platform consistency is a tension, not a target. On one side, a product should feel like one coherent thing wherever a user meets it: the same brand, the same mental model, the same trust in how it behaves. On the other side, each platform has conventions its users have learned and expect, and ignoring those conventions makes the product feel foreign and clumsy. The judgment problem is not "make everything identical" or "make everything native." It is deciding, for each layer of the design, what must stay consistent to preserve the product's identity and what should adapt to respect the platform's users. Agents tend to fail in two opposite directions: rigidly copying one platform's design to every surface, producing an experience that feels wrong on all the others, or letting each platform drift until the product is four different products sharing a name.

Use this skill when designing or reviewing a product across web, mobile, desktop, tablet, wearable, or other platforms, when managing a cross-platform design system, or when resolving conflicts between brand consistency and platform conventions. The goal is a deliberate consistency strategy that knows what to hold constant and what to let adapt.

## Core Rules

### Decide Consistency By Layer, Not As A Single Knob

Consistency is not one decision; it is a set of decisions at different layers, and the right answer differs by layer. Brand identity, such as color, typography, voice, and core iconography, should be highly consistent across platforms because it is the product's recognizable signature. Conceptual model, such as what the core objects are, how they relate, and what the primary actions mean, should be consistent so users transfer their understanding between platforms. Interaction details, such as exact gestures, navigation chrome, and control styling, should often adapt to platform conventions because users have learned them and expect them. Trying to force a single consistency level across all layers produces either a brand that does not register or interactions that feel alien.

Map consistency by layer:

- brand and visual identity: strongly consistent;
- conceptual model and terminology: strongly consistent;
- information architecture and core flows: mostly consistent;
- navigation and chrome: adapt to platform conventions;
- gestures and input: adapt to platform conventions;
- control styling and motion: balance, leaning toward platform where strong conventions exist.

State the strategy per layer so the team makes deliberate choices rather than defaulting to "match the iOS design."

### Preserve The Conceptual Model Across Platforms

The most important consistency is conceptual: the user's mental model of what the product is and does should transfer between platforms. If an object is called a "project" on web and a "workspace" on mobile, if the primary action is "publish" on desktop and "share" on mobile, if the hierarchy is flat on one platform and deep on another, the user cannot transfer their learning and must relearn the product on each surface. Conceptual inconsistency is more damaging than visual inconsistency because it breaks understanding, not just appearance.

Keep consistent:

- the names of core objects and actions;
- the relationships between objects;
- the meaning of primary and secondary actions;
- the overall information architecture;
- the rules that govern state, such as what "saved," "shared," or "archived" means.

Where terminology must adapt to a platform, do so deliberately and document why.

### Respect Strong Platform Conventions Where Users Expect Them

Users arrive at each platform with expectations formed by every other app they use on it. Navigation lives in certain places, back behaves certain ways, sharing uses the system sheet, text editing follows platform rules. Violating strong conventions to enforce cross-platform sameness makes the product feel broken to users who did not ask for consistency with your other platforms; they asked for consistency with their device. Respect conventions that are genuinely strong and widely learned, even when it means the design differs from another platform.

Common conventions to respect:

- system back behavior and navigation gestures;
- share sheets and system-level integrations;
- text selection, editing, and keyboard behavior;
- settings and preferences location;
- permission and alert presentation patterns.

Adapt at this layer rather than imposing a foreign pattern in the name of consistency.

### Distinguish Brand Consistency From Pixel Consistency

A frequent confusion is equating consistency with looking pixel-identical across platforms. Brand consistency means the product is recognizable and coherent; it does not mean every button has the same radius on every device. Pursuing pixel consistency forces platform-inappropriate choices, an iOS-style navigation bar on Android, a web-style menu on mobile, and produces surfaces that look the same but all feel slightly wrong. Aim for recognizable brand expression that uses each platform's idioms, not for photocopy reproduction.

Brand consistency is achieved through:

- shared color, typography, and voice;
- consistent iconography style and meaning;
- consistent use of imagery and illustration tone;
- coherent motion principles, even if specific animations differ.

These register the brand without requiring pixel-level uniformity.

### Manage A Cross-Platform Design System Deliberately

A cross-platform design system must express the consistency strategy: shared tokens for brand-level decisions, platform-specific tokens for adaptive layers, and clear documentation of what is shared versus what varies. A system that tries to share everything produces components that fit no platform well; a system that shares nothing produces fragmentation. Design the system's abstraction boundaries to match the consistency strategy decided by layer.

Structure the system around:

- shared foundation tokens: color, type scale, spacing rhythm, voice;
- shared component contracts: what a button, card, or input must do and mean;
- platform-specific implementations: how each platform renders and interacts;
- documented exceptions where a platform convention overrides the shared pattern.

The system encodes the strategy; without it, consistency decisions are made ad hoc and drift.

### Handle Feature And Content Parity Intentionally

Users expect that what they can do on one platform, they can do on others, at least for core tasks. Feature and content gaps breed frustration: a user who can edit on web but only view on mobile, or who finds a setting on desktop that is absent on mobile, loses trust. That said, true parity is not always possible or desirable; some tasks suit certain platforms. The discipline is deciding parity deliberately: which tasks must be fully available everywhere, which are platform-appropriate, and how to communicate gaps rather than silently omit features.

Decide for each capability:

- must be fully available on all platforms, because it is core;
- appropriate to some platforms, with a clear path or explanation elsewhere;
- not yet available, with honest communication rather than silent absence.

Silent feature omission is worse than an explained limitation, because the user assumes the product is broken.

### Reconcile Conflicts Between Brand And Platform Explicitly

Conflicts between brand consistency and platform convention are inevitable, and they must be resolved explicitly, not absorbed silently. When the brand wants one pattern and the platform expects another, document the decision and its rationale: which side won, why, and what the tradeoff is. Unresolved, unspoken conflicts produce inconsistent inconsistency, where the product breaks brand on some surfaces and breaks convention on others with no governing logic.

## Common Traps

### Pixel-Identical Copying Across Platforms

Forcing one platform's design onto every surface produces experiences that feel wrong everywhere. Aim for brand consistency, not photocopy reproduction.

### Letting Each Platform Drift Independently

Without a consistency strategy, each platform becomes a different product sharing a name. Decide what stays constant by layer.

### Conceptual Inconsistency Between Platforms

Different names, models, or hierarchies across platforms break the user's transferred understanding. Keep the conceptual model consistent.

### Violating Strong Platform Conventions

Ignoring widely learned conventions to enforce sameness makes the product feel broken to users who expect their device's patterns.

### Equating Consistency With Uniformity

Recognizable brand expression does not require identical pixels. Use brand tokens and platform idioms together.

### Silent Feature And Content Gaps

Omitting features without explanation makes users think the product is broken. Decide parity deliberately and communicate limitations.

### Ad Hoc Consistency Decisions

Without a governing strategy and system, consistency choices drift and contradict each other. Encode the strategy in the design system.

### Unresolved Brand-Platform Conflicts

Letting conflicts absorb silently produces inconsistent inconsistency. Resolve each explicitly with documented rationale.

## Self-Check

- [ ] Consistency was decided by layer, with brand and conceptual layers held constant and interaction layers allowed to adapt.
- [ ] The conceptual model, terminology, object relationships, and primary actions are consistent across all platforms.
- [ ] Strong platform conventions such as navigation, back, sharing, and editing are respected rather than overridden.
- [ ] Brand consistency is pursued through shared tokens and voice, not through pixel-identical reproduction.
- [ ] The cross-platform design system encodes what is shared versus what varies, with platform-specific implementations.
- [ ] Feature and content parity is decided deliberately, with core tasks fully available and gaps communicated honestly.
- [ ] Conflicts between brand and platform conventions are resolved explicitly with documented rationale.
- [ ] No platform silently omits a feature the user would expect, without explanation.
- [ ] The product feels like one coherent thing while respecting each platform's users.
- [ ] The consistency strategy is written down, not just held in one designer's head.
