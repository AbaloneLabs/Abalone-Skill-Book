---
name: hedge_fund_strategy_classification.md
description: Use when the agent is classifying or comparing hedge fund strategies, assessing long-short equity, event-driven, global macro, arbitrage and relative value, market-neutral and directional approaches, or evaluating how each strategy's risk, correlation, capacity, and return drivers differ.
---

# Hedge Fund Strategy Classification

"Hedge fund" is not a strategy; it is a fee and regulatory structure. Beneath the label lies a wide range of strategies — long-short equity, event-driven, global macro, arbitrage, relative value — that have almost nothing in common in their risk drivers, correlations, capacity, and expected returns. Grouping hedge funds together and judging "the asset class" by an index average hides the fact that a macro fund and a convertible arbitrage fund are entirely different investments. Selecting or allocating to hedge funds without understanding strategy classification leads to owning funds whose risks do not match the portfolio's needs.

Use this skill before answering questions such as "should I invest in hedge funds", "what hedge fund strategy is best", "what is the difference between macro and arbitrage", or "how do hedge fund strategies compare". The goal is to prevent the agent from treating hedge funds as a homogeneous category, and from recommending strategy allocation without matching risk drivers to portfolio objectives.

## Core Rules

### Classify By Return Driver, Not By Label

Each strategy's return comes from a distinct source, and that source determines its risk:

- Long-short equity: stock selection (long undervalued, short overvalued); returns driven by manager skill in picking stocks and sizing net exposure; equity-correlated to varying degrees depending on net exposure.
- Event-driven: corporate events (mergers, restructurings, distressed, activism); returns driven by event outcomes, deal completion, and credit recovery; correlated to equity and credit markets and to M&A activity.
- Global macro: top-down views on currencies, rates, equities, commodities via directional trades; returns driven by macro correctness and timing; highly variable, often uncorrelated to traditional markets.
- Arbitrage and relative value: exploiting small mispricings between related instruments (convertible arb, fixed-income arb, stat arb); returns driven by convergence and carry; low directional exposure but model, liquidity, and leverage risk.
- Managed futures / CTAs: systematic trend-following across futures markets; returns driven by trend persistence; often uncorrelated and crisis-alpha properties.

Identify the return driver first. A fund's name and category tell you less than understanding what actually generates its P&L.

### Match Net Exposure And Directionality To The Objective

Strategies differ in how much directional market risk they take:

- Directional (high net exposure): long-biased equity, macro, certain CTAs; returns move with markets; provide equity-like upside with some downside mitigation.
- Market-neutral (low net exposure): dollar-neutral or beta-neutral long-short, stat arb, many relative value strategies; returns driven by stock selection or convergence, not market direction; lower correlation but also lower expected return.
- Short-biased: explicit bearish exposure; rare, hard to sustain in rising markets; hedge against equity declines.

A portfolio seeking equity downside protection needs low-net or short-biased strategies; one seeking absolute return with low correlation needs market-neutral or macro; one seeking equity-like return with less downside wants directional with hedging. Match the strategy's net exposure to the portfolio objective.

### Assess Correlation, Beta, And Diversification Value

The diversification benefit depends on the strategy's correlation to the rest of the portfolio:

- Low correlation strategies (market-neutral, macro, managed futures): provide genuine diversification; their value is in reducing portfolio volatility and drawdown, not in standalone return.
- Equity-correlated strategies (long-short equity, event-driven): provide less diversification; their returns rise and fall with equities, though often with less downside.
- Crisis behavior: some strategies (managed futures, certain macro) tend to perform in crises ("crisis alpha"); others (relative value, certain arb) can lose in liquidity crises as spreads blow out and leverage unwinds.

Test correlation and crisis behavior across regimes. A strategy that diversifies in normal times but correlates in crises provides less protection than its average correlation suggests.

### Evaluate Capacity, Leverage, And Liquidity Constraints

Strategy characteristics constrain size and risk:

- Capacity: arbitrage and relative-value strategies have limited capacity (mispricings are finite); scaling assets erodes returns. Directional equity and macro have higher capacity.
- Leverage: relative-value and arbitrage strategies use high leverage to magnify small spreads; leverage amplifies losses and creates liquidity and margin risk. Directional strategies use less leverage.
- Liquidity: some strategies hold illiquid or hard-to-trade positions (distressed, certain credit); redemption terms and gating reflect this. Liquid strategies (futures, large-cap equity) allow faster redemption.

Capacity, leverage, and liquidity interact. A high-capacity strategy may be lower-return; a high-return arbitrage may be capacity-constrained and highly levered. Match the strategy's constraints to the investor's size and liquidity needs.

### Distinguish Discretionary From Systematic

The decision process affects consistency and risk:

- Discretionary: manager judgment drives trades; performance depends on the manager's skill, with key-person risk and variability; can adapt to novel situations but can also make large errors.
- Systematic (quant): rules and models drive trades; consistent and scalable, but model risk (the model can be wrong, especially in regime changes) and capacity limits; can fail in unprecedented conditions.
- Hybrid: combines judgment and models.

Discretionary funds carry key-person risk; systematic funds carry model and regime-change risk. Both can lose; the failure modes differ.

### Separate True Alpha From Beta Disguised As Skill

Many hedge fund returns are beta (market or factor exposure) dressed up as alpha:

- Equity beta: long-short equity funds with persistent net long exposure are earning equity beta, not stock-picking alpha.
- Factor exposures: value, momentum, quality, and other factor tilts explain much of long-short equity performance.
- Credit beta: event-driven and distressed funds carry credit beta.
- Option-like payoffs: some strategies sell tail risk (earning steady small gains, occasional large losses) that looks like alpha until the tail hits.

Decompose returns into beta, factor, and alpha components. A fund earning equity beta at 2-and-20 fees is an expensive way to own equities. True alpha is rare and should be the justification for the fees.

### Apply Strategy Allocation To Portfolio Objectives

Hedge fund allocation should be driven by what the portfolio needs:

- Equity downside protection: allocate to low-net equity, macro, or managed futures that perform in equity drawdowns.
- Diversification and absolute return: allocate to market-neutral and relative-value strategies with low correlation.
- Income and carry: allocate to credit and arbitrage strategies that generate steady carry, accepting tail risk.
- Inflation and macro hedging: allocate to macro and commodity strategies.

Do not allocate to "hedge funds" generically. Allocate to specific strategies whose risk and correlation profiles serve a defined portfolio role, and diversify across strategies to avoid single-strategy risk.

## Common Traps

### Treating Hedge Funds As A Homogeneous Asset Class

A macro fund and a convertible arbitrage fund are entirely different investments. Strategy classification drives risk and return.

### Confusing Beta For Alpha

Persistent net-long equity exposure, factor tilts, and credit beta are not skill. Decompose returns before paying alpha fees.

### Ignoring Capacity, Leverage, And Liquidity

High-return arbitrage strategies are often capacity-constrained, highly levered, and illiquid. Returns do not scale, and risks compound.

### Overlooking Crisis Correlation

Strategies that diversify in normal times can correlate in crises. Test crisis behavior, not just average correlation.

### Allocating To "Hedge Funds" Without A Portfolio Role

Allocation should serve a defined objective (protection, diversification, income). Generic hedge fund allocation has no clear purpose.

### Assuming Past Strategy Performance Will Persist

Strategy returns cycle with market conditions. Arbitrage returns compress as capital enters; trend-following works in trending, not choppy, markets.

## Self-Check

- [ ] Each fund is classified by its return driver (long-short equity, event-driven, macro, arbitrage, relative value, managed futures), not by label.
- [ ] Net exposure and directionality are matched to the portfolio objective (protection, absolute return, income).
- [ ] Correlation, beta, and crisis behavior are tested across regimes, not just in normal times.
- [ ] Capacity, leverage, and liquidity constraints are assessed for the strategy and the investor's size.
- [ ] Discretionary versus systematic process and its failure modes (key-person versus model/regime risk) are evaluated.
- [ ] Returns are decomposed into beta, factor, and alpha, and fees are justified by true alpha.
- [ ] Strategy allocation serves a defined portfolio role, with diversification across strategies.
- [ ] The conclusion avoids treating hedge funds as homogeneous and references investor-specific objectives, correlation needs, and risk tolerance.
