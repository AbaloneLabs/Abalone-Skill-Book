---
name: consent-safety-and-player-wellbeing.md
description: Use when the agent is designing social or multiplayer systems with voice and text chat, handling harassment and toxic behavior, building moderation and reporting tools, designing privacy and data controls, addressing player wellbeing and disordered play, adding spending and playtime limits, or considering the safety of minors and vulnerable players in online spaces.
---

# Consent, Safety, and Player Wellbeing

Online games are not just software; they are populated spaces where real people encounter each other, and like any populated space they can be welcoming, indifferent, or actively dangerous depending on how they are designed, moderated, and governed. The judgment problem is that safety and wellbeing are treated as secondary concerns — "add a report button and a profanity filter" — when in fact they are determined primarily by structural design choices made long before any moderation tooling: who can contact whom, what information is visible, how identity and reputation persist, what behaviors the systems reward, and what friction exists between a hostile actor and their target. Agents tend to miss this because the harms — harassment, stalking, doxxing, grooming, financial exploitation, compulsive play, sleep deprivation — are diffuse, delayed, and disproportionately suffered by players who are not in the design room, while the features that enable them (open voice chat, persistent identifiers, frictionless spending, infinite-session loops) each have legitimate positive uses that make them easy to defend in isolation. This skill covers how to design for safety and wellbeing as structural properties of the game, how to handle the inevitable abuse that social systems attract, and how to protect vulnerable players without infantilizing the whole audience. The agent has latitude to design many social structures, but the obligations around safety-by-design, privacy, and player agency over their own wellbeing are binding.

## Core Rules

### Practice Safety by Design, Not Safety by Moderation

Moderation catches harm after it has occurred; safety by design prevents or limits it structurally. The decision rule: for every social feature, ask how a hostile actor would abuse it — to harass, stalk, doxx, groom, or intimidate — and design the feature so that the abuse is difficult, slow, visible, or impossible, rather than relying on reports to clean up afterward. This includes defaulting to opt-in for contact from strangers, limiting what profile information is visible, providing easy blocking and muting that actually severs contact, rate-limiting communications, and designing reputation and identity systems so that hostile actors cannot simply recreate accounts to continue abuse. Moderation is necessary but it is a fallback; the first line of defense is the architecture.

### Make Consent Granular, Reversible, and the Default

Players should control who can contact them, who can see their information, what data is shared, and what experiences they are exposed to, and these controls should be granular (not all-or-nothing), reversible (easy to change later), and default to the most protective setting. The decision rule: default every social and data-sharing permission to the least exposure, require explicit opt-in for higher exposure, and never bury the controls or make them hard to find. A design that defaults to maximum contact and maximum visibility, then offers controls deep in a settings menu, has chosen convenience for abusers over safety for targets, whether or not it intended to. The burden of opting out of harm should never fall on the person being harmed.

### Treat Player Data as a Liability, Not an Asset

Every piece of personal data the game collects — real name, location, contact list, voice, biometrics, behavioral patterns — is a liability that can be breached, leaked, subpoenaed, sold, or weaponized for harassment, and the more the game collects, the larger the surface for harm. The decision rule: collect the minimum data necessary for the function, retain it only as long as needed, give players visibility into and control over what is held, and design features so that they do not require data whose breach would endanger players. Persistent location, voice recordings, and contact-list access are particularly high-risk and should require compelling justification, granular consent, and short retention. The convenience or monetization value of data is almost always outweighed by the harm potential of its compromise.

### Design Social Systems to Make Abuse Costly and Visible

Hostile actors are rational within their goals, and they abuse the systems that make abuse easy, cheap, and consequence-free. The decision rule: introduce friction at the points where abuse occurs — account creation, initial contact, mass messaging, identity changes — and make abusive behavior visible to the target and to moderators through logging, pattern detection, and transparent reputation. This includes limiting new accounts' ability to contact strangers, slowing or blocking identity changes that abusers use to evade blocks, detecting mass-reporting and brigading patterns, and ensuring that blocks sever contact comprehensively rather than only superficially. The goal is not to punish all players for the actions of abusers but to raise the cost of abuse specifically.

### Address Disordered Play and Overspending as a Design Responsibility

Compulsive play, sleep deprivation, neglect of responsibilities, and overspending are documented harms associated with certain game designs, and dismissing them as purely the player's responsibility ignores that the designs are often built to produce exactly these outcomes. The decision rule: provide players with meaningful tools to manage their own play — playtime tracking and limits, spending tracking and limits, session reminders, cooldowns, and self-exclusion — and design these tools to be genuinely effective rather than performative. A playtime reminder that is easy to dismiss, or a spending limit that is easy to override, is not a wellbeing feature; it is liability theater. Effective tools have friction built into the override, surface the relevant history, and respect the player's prior decision even when they later want to ignore it.

### Protect Minors as a Distinct Population

Minors are not just smaller adults; their judgment, impulse control, and vulnerability to manipulation are developmentally different, and games accessible to minors carry heightened obligations. The decision rule: where the audience includes or may include minors, apply stricter defaults — stronger privacy, no targeted advertising, no high-pressure monetization, no unrestricted contact with adults, robust parental controls — and do not rely on age gates that everyone knows are ineffective. The design question is not "is this legal for minors in our jurisdiction" but "is this safe for the minors who will actually use it," and the answer must account for the certainty that minors will access the product regardless of nominal age restrictions.

### Build Moderation and Support as Infrastructure, Not Afterthought

A social game without resourced moderation and player support is a social game that will harm its players, because abuse is inevitable in any unmoderated populated space. The decision rule: budget for human moderation, player support, and trust-and-safety engineering as core infrastructure from launch, not as a team to hire when problems emerge, and staff them at levels calibrated to actual abuse volume rather than to wishful projections. The cost of adequate moderation is a cost of operating a social product; treating it as optional is treating player safety as optional, and the eventual reputational, legal, and human costs of under-moderation always exceed the savings.

## Common Traps

### The Report Button as Safety Theater

The team ships a report button and considers the safety problem solved, but the button feeds an under-resourced queue, reports are not acted on promptly, repeat abusers face no escalating consequence, and targets receive no feedback or protection. The trap is that the feature's presence reads as responsibility. The false signal is that the report button exists and is used. The harm is that targets learn reporting is futile, abusers learn there is no consequence, and the abuse continues with the team's implicit blessing.

### Default-Open Social Architecture

The game defaults to allowing anyone to contact anyone, voice chat is on by default, profiles expose identifying information, and blocking is buried or incomplete, on the theory that openness fosters community. The trap is that openness does foster community for the majority while exposing the minority to relentless abuse. The false signal is that most players have positive social experiences. The harm is that the players who are targeted — disproportionately women, marginalized players, and minors — are driven out of the game, and their absence is invisible because they leave quietly.

### Data Collection Driven by Possibility Rather Than Need

The team collects data because it might be useful someday, because the analytics team wants it, or because a future feature might need it, accumulating a store of sensitive information whose breach would devastate the affected players. The trap is that data feels like an asset and collection feels free. The false signal is that richer data enables better decisions. The harm is a growing liability that, when breached or misused, causes harms — stalking, identity theft, exposure — that the game's value could never justify, and that the team has no plan to prevent.

### Performative Wellbeing Tools

The game ships playtime reminders and spending limits that are trivial to dismiss or override, then points to them as evidence of caring about wellbeing. The trap is that the tools exist and can be cited in defense. The false signal is that the wellbeing features are present in the product. The harm is that the tools provide no real protection, they exist primarily to deflect criticism, and they may even increase harm by giving players a false sense of safety while the underlying exploitative design continues uninterrupted.

### Frictionless Spending and Infinite Sessions

The game removes all friction from spending and from continuing to play — one-tap purchases, saved payment methods, no session limits, mechanics that punish logging off — maximizing revenue and engagement while maximizing the harm to players prone to overspending or compulsive play. The trap is that the frictionlessness produces strong metrics. The false signal is that revenue and engagement rise. The harm is concentrated in the vulnerable minority who are the explicit target of the design, and the team externalizes that harm while internalizing the revenue.

### Ignoring the Disproportionate Harm to Marginalized Players

The team evaluates safety by average experience and concludes the game is safe, when the average conceals that women, players of color, LGBTQ+ players, and players with disabilities face dramatically higher rates of harassment and are driven out at higher rates. The trap is that the aggregate looks fine. The false signal is that overall retention and satisfaction metrics are healthy. The harm is that the game systematically sheds exactly the players whose presence would make it more diverse and welcoming, entrenching a homogeneous community that is itself a barrier to broader inclusion.

## Self-Check

- For every social feature, have I threat-modeled how a hostile actor would abuse it for harassment, stalking, doxxing, or grooming, and designed structural friction rather than relying on reactive moderation?
- Are all social and data-sharing permissions granular, reversible, and defaulted to the most protective setting, with the burden of opting into exposure on the person who wants it rather than on the person being exposed?
- Have I minimized the personal data collected, retained it only as long as needed, and justified each high-risk category — location, voice, contacts, biometrics — against the harm potential of its compromise?
- Have I introduced friction at abuse points — account creation, initial contact, mass messaging, identity changes — and made abusive behavior visible to targets and moderators through logging and pattern detection?
- Are the playtime and spending wellbeing tools genuinely effective, with real friction on overrides and respect for the player's prior decisions, rather than dismissible reminders that function as liability theater?
- Where minors can access the product, have I applied stricter defaults for privacy, monetization, and contact, rather than relying on ineffective age gates?
- Is trust and safety resourced as core infrastructure — human moderation, player support, safety engineering — calibrated to actual abuse volume and operating proactively rather than only reactively to incidents?
