---
name: rapid-prototyping-and-fidelity-choices.md
description: Use when the agent is building prototypes, choosing paper versus digital versus vertical-slice fidelity, deciding what to fake or stub, scoping a prototype to answer a specific design question, or reviewing whether a prototype is testing the right risk before committing production resources.
---

# Rapid Prototyping and Fidelity Choices

A prototype is a tool for answering a specific design question at minimum cost, and the central judgment problem is matching the fidelity of the prototype to the question being asked. Designers miss this because they reach for the highest-fidelity tool available — a full engine, real art, shipped-feeling controls — under the belief that higher fidelity produces more valid answers. It often produces the opposite: a high-fidelity prototype takes so long to build that few questions get asked, the team falls in love with the polish, and the cost of abandoning a bad idea becomes unbearable, so bad ideas survive into production. The judgment problem is that fidelity is not free and is not always better; the right prototype is the cheapest one that convincingly answers the question, whether that is a paper sketch, a gray-block digital mockup, or a single polished mechanic. The harm of misjudging fidelity is either wasted effort (over-building a prototype whose answer was knowable on paper) or invalid answers (under-building a prototype whose question genuinely required feel, timing, or visual context to answer). The agent has wide freedom in tool and medium choice, but it must always begin from the question, not the tool, and it must resist the gravitational pull of polish, which converts a cheap experiment into an expensive commitment.

## Core Rules

### Start From the Question, Not the Tool

Every prototype exists to answer a question — "is this mechanic fun," "can players understand this UI," "does this economy produce the intended tension" — and the question dictates the medium, scope, and fidelity. The rule is to write the question down before building anything, in a form that has a yes/no or measurable answer, and to choose the cheapest medium that can answer it. Decision criterion: if you cannot state the single question the prototype is meant to answer, you are not prototyping, you are building, and you will not know when you are done or whether you succeeded. A prototype without a question is just early production.

### Match Fidelity to What the Question Depends On

Some questions depend on feel and timing and require interactive digital fidelity (does this movement feel good); some depend on logic and can be answered on paper (does this economy balance hold); some depend on visuals and require art context (does this silhouette read as friendly). The rule is to raise fidelity only in the dimensions the question actually depends on and to leave every other dimension at the lowest possible fidelity. Decision criterion: for your question, which dimensions — logic, feel, visuals, audio, narrative — must be real for the answer to be valid? Build those; fake the rest. Over-building unrelated dimensions wastes time and biases the answer.

### Time-Box Every Prototype Against Its Question

A prototype that answers its question in a day is worth ten that take a month, because the value of prototyping is the number of questions you can ask, not the quality of any single artifact. The rule is to set a fixed time budget before starting — a day, a week, a sprint — and to descope ruthlessly to fit it, on the principle that a rough answer now beats a polished answer later, because later the cost of changing direction has multiplied. Decision criterion: at the end of the time box, do you have an answer to the question, even a rough one? If the prototype is not done but the question is answerable, stop; if the time is gone and the question is unanswered, the fidelity was wrong.

### Fake Everything That Does Not Affect the Answer

Real systems, real art, real networking, real save infrastructure — all of these are production concerns disguised as prototype necessities. The rule is to stub, hardcode, script, and fake anything that does not bear on the question: pre-baked levels instead of procedural generation, hardcoded values instead of real tuning systems, a human "wizard of oz" instead of AI. Decision criterion: does the player need to experience this system for the question to be answered? If not, fake it. The trap is building production-grade infrastructure inside a prototype and then being unable to throw the prototype away.

### Build to Throw Away, and Make That Possible

A prototype that becomes the foundation of production is a prototype that was not allowed to fail, and it carries forward every shortcut, stub, and hack that was acceptable in an experiment but fatal in a shipped product. The rule is to treat prototypes as disposable by default and to architect them so that disposal is possible — no shared production code, no irreplaceable art, no team-wide dependencies. Decision criterion: if the prototype's answer is "this idea is bad," can you throw the whole thing away without losing anything you cannot rebuild in a day? If not, the prototype has become a commitment and has lost its experimental value.

### Know When a Question Genuinely Requires High Fidelity

Not every question can be answered cheaply. Some design questions — does this combat feel good, does this aesthetic evoke the intended emotion, does this multiplayer netcode hold at scale — require real feel, real art, or real infrastructure, and pretending they can be answered on paper produces confidently wrong answers. The rule is to honestly assess whether the question is about logic (cheap) or about feel/scale/aesthetics (expensive), and to invest in fidelity where it is genuinely load-bearing. Decision criterion: would a paper or gray-block answer mislead the team about this question? If yes, the question demands fidelity, and the cost is justified — but only for that question.

### Separate Prototyping Risk From Production Risk

Prototypes answer design risk (will players find this fun, clear, balanced); they do not answer production risk (can we build this at quality, on budget, at scale). A successful prototype does not mean the game will ship, and a team that conflates the two enters production overconfident. The rule is to be explicit about which kind of risk a prototype is de-risking, and to run separate prototypes or spikes for the other kind. Decision criterion: after this prototype succeeds, do you know the design works, the production is feasible, or both? If only one, name the remaining risk explicitly rather than assuming success transfers.

## Common Traps

### Over-Building Fidelity Because Polish Feels Like Progress

A team builds a beautiful, polished prototype because polish feels productive and demonstrates seriousness, but the question could have been answered in gray blocks in a tenth of the time. The trap is that polish is emotionally rewarding and visually impressive, so it feels like the right thing to do. The false signal is a gorgeous demo that impresses stakeholders; the harm is that few questions get asked because each one is expensive, and the team commits to ideas based on aesthetics rather than validated design.

### Falling in Love With the Prototype and Refusing to Kill It

A prototype works, looks good, and the team becomes attached to it, so when the answer it produces is "this idea is not actually fun," the team rationalizes and ships it anyway. The trap is that sunk cost and affection convert an experiment into a commitment. The false signal is that the prototype "works"; the harm is that bad ideas survive into production because killing them would mean abandoning something the team has invested in and come to love, and the game ships with a flawed core.

### Under-Building a Feel-Dependent Question and Trusting the Paper Answer

A question about whether combat or movement feels good is answered with a paper or logic prototype, and the team concludes the mechanic is sound, when feel questions cannot be answered without interactive fidelity. The trap is that the cheap answer feels rigorous because it is quantitative. The false signal is a clean logical model; the harm is shipping a mechanic that is correct on paper and lifeless in the hand, discovered only in late production when the cost of fixing it is enormous.

### Prototyping the Wrong Risk

The team prototypes whether the core loop is fun (design risk) and declares the game de-risked, then discovers in production that the game cannot be built at scale, on budget, or with the available tech (production risk). The trap is treating a successful design prototype as evidence the whole project is safe. The false signal is a fun, validated prototype; the harm is entering production with unaddressed production risk, leading to crunch, cuts, or cancellation, because the team de-risked the wrong thing.

### Letting the Prototype Become the Codebase

Because the prototype "works," the team builds production on top of it rather than rewriting, and every stub, hack, and shortcut metastasizes into the shipped game as technical debt, bugs, and unmaintainable systems. The trap is that rewriting feels wasteful when something already runs. The false signal is that the prototype is functional; the harm is years of accumulated debt, slow iteration, and bugs that trace back to decisions that were acceptable in a disposable experiment and fatal in a product.

### Prototyping Without a Kill Criterion

A prototype is built to "see if it is fun," with no definition of what would constitute failure, so regardless of the result the team declares success and proceeds, because there was never a condition under which they would stop. The trap is that an unkillable prototype is not an experiment, it is pre-committed production wearing a prototype's clothes. The false signal is the appearance of experimentation; the harm is that the prototyping phase provides no actual de-risking, because no answer would have changed the decision to proceed.

## Self-Check

- Did I write down the single, answerable question the prototype exists to answer before choosing a tool or medium?
- Did I raise fidelity only in the dimensions the question depends on (logic, feel, visuals, audio) and fake every other dimension?
- Did I set a fixed time box, descope to fit it, and stop when the question was answerable rather than when the artifact was polished?
- Is everything that does not bear on the question stubbed, hardcoded, or faked, with no production-grade infrastructure embedded in the prototype?
- If the prototype's answer is negative, can I throw the entire artifact away without losing irreplaceable work?
- For feel-, scale-, or aesthetic-dependent questions, did I honestly invest in the fidelity those questions require rather than trusting a cheaper but invalid answer?
- Have I distinguished whether the prototype de-risks design or production, and named the remaining risk explicitly rather than assuming success transfers?
