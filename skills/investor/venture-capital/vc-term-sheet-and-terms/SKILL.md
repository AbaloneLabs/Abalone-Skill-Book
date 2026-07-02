---
name: vc_term_sheet_and_terms.md
description: Use when the agent is analyzing venture capital term sheets, evaluating preferred stock liquidation preferences and participation, anti-dilution and protective provisions, conversion and board rights, pro-rata and veto rights, or assessing how terms shift risk and returns between founders, investors, and later-round participants.
---

# VC Term Sheet And Terms

A venture capital term sheet looks like a short, friendly document, but its terms determine who gets paid first, who controls the company, who gets diluted how, and who can block what — for the entire life of the company. The headline valuation (pre-money) is only one input; the liquidation stack, anti-dilution, board control, and protective provisions can make a "high" valuation a bad deal and a "low" valuation a good one. Reading a term sheet for the price and ignoring the terms is how founders give away control and how investors get stacked under preferences that erase their returns.

Use this skill before answering questions such as "is this term sheet fair", "what is a liquidation preference", "what does participating preferred mean", or "how do VC terms affect returns". The goal is to prevent the agent from treating the valuation as the deal, and from missing the preference, control, and anti-dilution terms that determine economic and governance outcomes.

## Core Rules

### Separate Valuation From The Capital Structure

The pre-money valuation sets the share price, but the terms define what those shares are worth in different outcomes:

- Preferred versus common: investors receive preferred stock with rights senior to common (founders, employees). In a sale or wind-down, preferred is paid before common.
- Liquidation preference: the amount paid to preferred before common. 1x non-participating preferred is standard; higher multiples (2x, 3x) or participating preferred shift more value to investors.
- Participation: participating preferred gets its preference back AND then shares in remaining proceeds pro-rata (double-dipping); non-participating chooses between preference or conversion to common.
- Seniority and stacking: later rounds are often senior to earlier ones; in a down exit, the stack determines who gets paid, and early investors and common can be wiped out.

Model the payout to each class across exit values (low, medium, high). A high pre-money with a 2x participating preference can deliver less to founders than a lower pre-money with 1x non-participating.

### Model Liquidation Preferences And Participation Across Exit Values

The economic impact of preferences is non-linear and only visible across scenarios:

- High exit (great outcome): preferences rarely bind; everyone converts to common and shares pro-rata; valuation dominates.
- Medium exit: preferences begin to bind; participating preferred takes a disproportionate share; common's effective return compresses.
- Low exit (modest sale): the preference stack consumes most or all proceeds; common (founders, employees) may receive nothing even though the company "sold."

Build a waterfall for low, medium, and high exits. This reveals the "preference overhang" — how much the company must sell for before common sees meaningful proceeds. A company with heavy cumulative preferences needs a large exit to reward the team, which can misalign incentives.

### Understand Anti-Dilution And Down-Round Protection

Anti-dilution provisions protect investors in down rounds (later financing at a lower price):

- Broad-based weighted average: adjusts the conversion price moderately based on the new round's price and size; the balanced, common standard.
- Narrow-based weighted average: more aggressive adjustment, more favorable to investors.
- Full ratchet: the conversion price drops to the new (lower) round price regardless of size; extremely punitive to founders and common; rare except in distress.

Full ratchet and narrow-based anti-dilution transfer significant value from common to investors in down rounds. Founders should resist full ratchet; investors should understand what they are asking for.

### Analyze Board Composition, Voting, And Control

Control terms determine who runs the company and who can block decisions:

- Board composition: founders, investors, and independents; investor-controlled boards shift control to investors; balanced boards preserve founder influence.
- Protective provisions: veto rights over major actions (selling the company, raising new capital, changing the charter, hiring/firing executives, budgets). Broad protective provisions give investors effective control even without a board majority.
- Drag-along: allows a majority to force minority shareholders to sell; can be used to exit over founder objection.
- Information and inspection rights: investors' access to financial and operating information.

Control terms are often more contested than valuation. A founder-friendly valuation with investor-friendly control can leave founders running a company they no longer control.

### Evaluate Pro-Rata, Preemptive, And Follow-On Rights

These rights affect who participates in future rounds:

- Pro-rata rights: existing investors' right to maintain their ownership percentage in subsequent rounds. Valuable to defend against dilution in up-rounds.
- Super pro-rata: the right to invest more than the pro-rata share; favored by large funds; dilutes others.
- Preemptive rights: right to participate in new issuances; broader than pro-rata.
- Pay-to-play: investors who do not participate in down rounds lose preferences or other rights; forces support in tough times.

Pro-rata rights are how investors maintain exposure to winners. Their absence or limitation affects an investor's ability to follow on into breakout companies.

### Assess Vesting, Option Pool, And Founder Alignment

Founder and employee equity terms affect alignment and retention:

- Founder vesting: founders' shares typically vest over 4 years with a 1-year cliff; protects against early departure with large unearned equity.
- Option pool (ESOP): the reserve of options for employees; the size and whether it is pre- or post-money affects dilution. A larger pre-money pool dilutes founders more.
- Acceleration: single/double trigger acceleration of vesting on change of control; protects employees in an acquisition.
- Reverse vesting and repurchase: company right to repurchase unvested shares on departure.

Vesting and option pool terms determine how equity is distributed over time and who bears the dilution. A "pre-money option pool" effectively lowers the pre-money valuation for founders.

### Read The Whole Stack, Not Just The Current Round

Each round's terms interact with every prior and future round:

- Stacking preferences: multiple rounds of senior preferred create a deep stack that determines payouts.
- Pay-to-play and down rounds: later rounds can reset preferences, restructure the cap table, or impose new terms on holdouts.
- Conversion and automatic conversion: preferred converts to common (typically at IPO), changing the economics.

Analyze the full cap table and preference stack, not just the current term sheet. A new investor buying into a heavily stacked cap table may find their returns compressed by prior preferences.

## Common Traps

### Focusing On Valuation And Ignoring Preferences

A high valuation with participating preferred or high multiples can deliver less to common than a lower valuation with clean 1x non-participating.

### Overlooking The Preference Overhang

Heavy cumulative preferences require a large exit before common sees meaningful proceeds, misaligning the team and deterring effort.

### Accepting Full Ratchet Anti-Dilution

Full ratchet is extremely punitive to founders in down rounds and can wreck the cap table. Resist unless in deep distress.

### Ignoring Control And Protective Provisions

Board composition and veto rights can shift effective control to investors regardless of ownership. Read control terms as carefully as economic terms.

### Missing Pro-Rata Rights For Follow-On

Without pro-rata, investors cannot maintain ownership in breakout winners, which is where most VC returns concentrate (see power-law skill).

### Reading One Round In Isolation

Each round stacks on prior rounds. Analyze the full cap table and preference stack to understand who gets paid in each outcome.

## Self-Check

- [ ] Valuation is separated from the capital structure, and preferred/common distinctions are explicit.
- [ ] Liquidation preferences and participation are modeled across low, medium, and high exit values, and the preference overhang is identified.
- [ ] Anti-dilution (broad-based, narrow-based, full ratchet) is assessed for its dilutive impact in down rounds.
- [ ] Board composition, protective provisions, drag-along, and control terms are analyzed for governance impact.
- [ ] Pro-rata, preemptive, super pro-rata, and pay-to-play rights are evaluated for follow-on and dilution effects.
- [ ] Founder vesting, option pool (pre- versus post-money), acceleration, and repurchase terms are assessed for alignment.
- [ ] The full cap table and preference stack across all rounds are analyzed, not just the current term sheet.
- [ ] The conclusion avoids treating valuation as the deal and references the party-specific objectives (founder versus investor) and risk tolerance.
