---
name: power_law_and_portfolio_construction.md
description: Use when the agent is constructing a venture capital portfolio, modeling power-law return distributions, sizing positions across stages, planning diversification and follow-on reserves, and estimating expected returns given that a few winners drive most of the fund's returns.
---

# Power-Law And Portfolio Construction

Venture capital returns follow a power law: a small number of portfolio companies generate the vast majority of returns, while most investments fail or return little. This is the opposite of a normal distribution, and it breaks the intuition that "more deals at smaller size" or "average outcomes" describe the portfolio. Constructing a VC portfolio without explicitly modeling the power law — the base rates of failure, the probability of a winner, the ownership needed in winners, and the follow-on capital required to maintain it — leads to portfolios that cannot return the fund even if a winner emerges.

Use this skill before answering questions such as "how many companies should a VC fund invest in", "how do I size VC investments", "what is the power law in venture", or "how do I construct a venture portfolio". The goal is to prevent the agent from applying normal-distribution diversification logic to power-law returns, and from building portfolios that cannot generate fund-returning outcomes.

## Core Rules

### Internalize The Power-Law Distribution Of Outcomes

VC outcomes are not bell-curved; they are power-law distributed:

- Most companies fail or return less than invested: typically 40-60% of deals return 0-1x.
- A few return 1-5x: moderate winners that recover capital.
- A tiny few return 10-100x+: the "fund returners" that generate most of the fund's total return.

In a typical fund, 1-3 companies may produce the majority of the total value. The portfolio's return depends almost entirely on capturing these outliers, not on the average deal. Construction must optimize for exposure to outliers, not for average deal quality.

### Size The Portfolio For A Meaningful Probability Of A Winner

Because winners are rare, the portfolio must hold enough positions to have a reasonable chance of including one:

- Too few deals (e.g., 5-8): high risk of missing winners entirely; the fund depends on near-certain success of a handful.
- Diversified (e.g., 20-40 for seed; 10-20 for Series A): improves the probability of capturing a winner; the standard range for fund-returning odds.
- Too many deals (e.g., 100+ at tiny ownership): ownership in any single winner is too small to return the fund, even if a winner emerges.

There is a tension between diversification (more deals to catch a winner) and ownership (larger stakes so a winner returns the fund). Construction balances these against the fund size and stage.

### Compute The Ownership Needed For A Fund Return

Determine what ownership in a winner is required to return the fund:

- Fund return definition: often 3x net MOIC or a target IRR; "returning the fund" means 1x DPI from a single deal.
- Required ownership = fund size / (winner exit valuation × dilution factor). For a $100M fund, returning the fund from one deal requires owning ~10% of a $1B exit (after dilution).
- Dilution: ownership at exit is lower than at investment due to future rounds; model typical dilution (often 50%+) from entry to exit.

Construction must ensure that initial ownership (plus follow-on) in potential winners is large enough that a single breakout can return the fund. Tiny ownership in many deals cannot do this.

### Reserve Follow-On Capital For Winners

Winners raise more capital in later rounds; maintaining ownership requires follow-on:

- Reserve allocation: a portion of the fund (often 30-50%) held back to follow on into breakout companies.
- Concentration in winners: follow-on capital should concentrate in companies that are winning, doubling down on power-law outcomes.
- Pro-rata defense: exercising pro-rata rights to maintain ownership in up-rounds; failing to do so dilutes exposure to the very winners that matter.
- Opportunity cost: every dollar reserved is not deployed to new deals; the reserve-versus-new balance is a core construction decision.

A fund that deploys all capital into initial positions cannot follow on into winners, surrendering ownership in the companies that drive returns. Reserve design is central to power-law construction.

### Model Expected Returns From Base Rates

Construction should be tested against base-rate expected returns:

- Failure rate: probability a deal returns 0-1x (typically 40-60%).
- Moderate win rate: probability of 1-5x (20-30%).
- Outlier rate: probability of 10x+ (a few percent per deal).
- Expected value per deal: probability-weighted return; the portfolio's EV is the sum across deals.
- Fund return sensitivity: how many outliers, at what ownership, are needed to hit the target return?

Modeling reveals whether the portfolio can plausibly return the fund. A portfolio of 30 deals at 2% ownership each, with base-rate failure rates, may mathematically require multiple outliers to return the fund — an unlikely outcome. Construction must make the math work.

### Account For Stage, Vintage, And Sector Concentration

Construction choices interact with stage and concentration:

- Stage: seed has higher failure rates but lower entry valuations and more diversification; Series A/B have lower failure but higher valuations requiring larger checks; growth has lower failure but needs very large exits.
- Vintage: deploying across multiple vintages (for evergreen or multi-fund programs) smooths entry conditions; a single vintage concentrates market-timing risk.
- Sector concentration: thematic concentration (e.g., all AI, all fintech) increases dispersion; sector diversification smooths but may dilute edge.

Match construction to the stage's base rates and the investor's edge. Seed portfolios need more diversification; growth portfolios need larger ownership and fewer, higher-conviction bets.

### Plan For Losses, Capital Calls, And Liquidity

VC portfolios face chronic negative cash flow and high loss rates:

- Loss tolerance: a VC portfolio will have many write-offs; the investor must be able to absorb these without abandoning the strategy.
- Capital call timing: commitments are called over years; the LP must hold liquid capital to meet calls while receiving no distributions for many years.
- Liquidity and J-curve: early years show negative returns (fees, write-offs, unrealized losses); distributions come years later, concentrated in winners.
- Long horizon: VC capital is locked for 10+ years; only capital that can be locked up that long is suitable.

VC is unsuitable for capital the investor may need, cannot afford to lose, or cannot commit for a decade. Construction includes the investor's liquidity and loss tolerance, not just the deal math.

## Common Traps

### Applying Normal-Distribution Diversification Logic

VC returns are power-law distributed. "Many small bets average out" does not work; ownership in winners is too small to return the fund.

### Owning Too Little To Return The Fund

Tiny ownership in many deals cannot generate a fund return even if a winner emerges. Compute the ownership needed and ensure the portfolio can achieve it.

### Failing To Reserve For Follow-On

Without follow-on reserves, the fund cannot maintain ownership in winners through later rounds, surrendering the returns that matter.

### Over-Diversifying Into Insignificance

Too many deals at tiny ownership dilute exposure to any single winner below the threshold needed to return the fund.

### Ignoring Base Rates And Required Outliers

Construction must be tested against base rates. A portfolio that mathematically requires multiple outliers to succeed is unlikely to do so.

### Deploying Capital The Investor Cannot Lock Up

VC requires decade-long lockups and high loss tolerance. Capital needed for liquidity or that the investor cannot afford to lose is unsuitable.

## Self-Check

- [ ] The power-law distribution of outcomes is internalized, and construction optimizes for outlier exposure, not average deal quality.
- [ ] Portfolio size balances diversification (catching a winner) against ownership (a winner returning the fund), appropriate to the stage.
- [ ] The ownership needed for a fund return is computed, accounting for dilution from entry to exit.
- [ ] Follow-on reserves are allocated to concentrate in winners and defend pro-rata in up-rounds.
- [ ] Expected returns are modeled from base rates (failure, moderate win, outlier rates), and the portfolio's plausibility of returning the fund is tested.
- [ ] Stage, vintage, and sector concentration are matched to base rates and the investor's edge.
- [ ] Loss tolerance, capital-call liquidity, the J-curve, and the decade-long horizon are factored into suitability.
- [ ] The conclusion avoids applying normal-distribution logic and references investor-specific fund requirements, liquidity, and risk tolerance.
