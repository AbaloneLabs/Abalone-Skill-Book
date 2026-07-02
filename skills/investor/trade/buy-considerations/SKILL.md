---
name: stock_buy_considerations.md
description: Use when the agent is evaluating what to consider before buying a stock, adding to an equity position, comparing stock opportunities, or forming a pre-trade checklist for equity investment.
---

# Stock Buy Considerations

Buying a stock is not the same as liking a company, product, theme, founder, technology, or recent price chart. A stock purchase is a decision to exchange cash for a claim on uncertain future cash flows at a specific price, under specific portfolio constraints, with real downside and opportunity cost.

Use this skill before answering broad questions such as "what should be considered before buying stocks", "what should I check before adding an equity position", "how should a stock opportunity be evaluated", or "what matters before buying a company share". The goal is not to predict the next price move. The goal is to prevent the agent from turning a simple story into an investment conclusion without checking valuation, risk, time horizon, position sizing, and portfolio fit.

## Core Rules

### Separate Company Quality From Investment Quality

A great company can be a poor investment at the wrong price. A mediocre company can be a good investment if expectations are low, risks are understood, and the price compensates for uncertainty. Do not let admiration for a product, brand, growth story, or management team replace investment analysis.

Check both layers:

- Business quality: revenue durability, margins, competitive position, reinvestment opportunity, balance sheet strength, governance, and management execution.
- Investment quality: current valuation, expectations embedded in the price, downside if assumptions fail, liquidity, tax impact, and role in the portfolio.

An agent should not say "this company is strong, therefore buy" without asking what the market already believes and what return the current price implies.

### Define The Investment Thesis Before The Trade

Write the thesis before discussing action. A thesis should name the source of return, the time horizon, and the evidence that would make the thesis wrong.

Possible thesis drivers:

- earnings growth faster than expected;
- margin expansion or operating leverage;
- valuation multiple re-rating;
- capital return through dividends or buybacks;
- asset value not reflected in the market price;
- cyclical recovery;
- balance sheet repair;
- market overreaction to temporary bad news.

Avoid vague theses like "AI will grow", "the chart looks good", or "this is a long-term winner". Those statements may be relevant observations, but they do not specify expected return, risk, or falsification.

### Ask What Is Already Priced In

The market may already know the attractive story. A widely discussed growth theme, famous product, or consensus winner can still disappoint if the current price assumes near-perfect execution.

Ask:

- What growth, margin, and multiple assumptions appear embedded in the current price?
- Are analyst expectations rising or falling?
- Is the stock priced for recovery, stability, distress, or perfection?
- What must happen for the investment to produce an acceptable return from here?
- What happens if the company performs well but less well than expected?

Investment analysis should focus on the gap between expectations and plausible outcomes, not on the attractiveness of the story alone.

### Estimate Downside Before Upside

Before considering upside, define what can go wrong and how much capital could be lost. Downside analysis disciplines position size and prevents narrative enthusiasm.

Consider:

- earnings disappointment;
- margin compression;
- valuation multiple contraction;
- debt refinancing risk;
- dilution;
- regulatory change;
- customer concentration;
- product obsolescence;
- fraud, governance, or accounting risk;
- liquidity drying up during stress.

Do not treat "long-term" as a substitute for downside analysis. Long holding periods can help patient investors, but they do not rescue every overpaid or permanently impaired investment.

### Check Portfolio Fit And Concentration

A stock can be attractive in isolation and still be wrong for the portfolio. The decision should account for existing exposures, diversification, liquidity needs, tax position, and emotional tolerance for drawdowns.

Ask:

- How large would the position be after purchase?
- Is the portfolio already exposed to the same sector, factor, geography, currency, commodity, employer, customer base, or macro driver?
- Would a decline in this stock coincide with job income risk, business risk, or other assets falling?
- Is this purchase funded by cash, new savings, or sale of another asset?
- What position size would still allow the investor to make rational decisions after a drawdown?

Do not analyze a buy decision as if the investor has an empty portfolio.

### Match Time Horizon To Evidence

Short-term trades, multi-year investments, income holdings, and speculative positions require different evidence. A long-term thesis should not rely on a one-week chart. A short-term trade should not hide behind a five-year industry story when risk control is absent.

State:

- intended holding period;
- review cadence;
- expected thesis milestones;
- liquidity needs;
- conditions for adding, holding, trimming, or exiting.

If the horizon is unclear, the buy decision is usually underdefined.

### Compare Against Alternatives

Every stock purchase competes with alternatives: cash, broad index exposure, bonds, another stock, debt repayment, or waiting. The agent should ask whether the incremental risk is worth taking.

Compare:

- expected return relative to broad market exposure;
- downside relative to diversified funds;
- complexity relative to position size;
- tax and transaction costs;
- liquidity and flexibility;
- correlation with the rest of the portfolio.

A single-stock purchase needs a reason to prefer concentrated company-specific risk over simpler diversified exposure.

## Common Traps

### Buying The Story Without The Price

Exciting markets create persuasive stories. The trap is to describe a large opportunity and never ask how much of it the stock already discounts. Theme size is not shareholder return.

### Confusing Recent Performance With Safety

A stock that has risen may be gaining because fundamentals improved, because valuation expanded, or because buyers became crowded. Momentum can be useful evidence, but it is not proof of low risk.

### Treating A Small Position As Not Worth Analysis

Small positions can accumulate into a scattered portfolio with no clear risk profile. Even a small speculative buy should have a reason, size limit, and loss tolerance.

### Ignoring Dilution And Capital Structure

Common shareholders may not capture business growth if debt, preferred claims, stock compensation, convertibles, or future capital raises absorb value. Equity analysis must consider the full capital structure.

### Using One Valuation Metric Mechanically

P/E, EV/EBITDA, price-to-sales, dividend yield, and book value can all mislead when used without context. The right metric depends on profitability, cyclicality, growth, leverage, accounting quality, and asset intensity.

### Forgetting Tax And Account Type

The same stock may have different implications in taxable accounts, retirement accounts, margin accounts, or accounts with foreign withholding taxes. Tax should not dominate every decision, but it can change after-tax return and rebalancing flexibility.

## Self-Check

- [ ] The analysis separates company quality from investment quality at the current price.
- [ ] The investment thesis names the expected source of return, time horizon, and falsifying evidence.
- [ ] Current market expectations and what appears priced in were considered.
- [ ] Downside scenarios, permanent impairment risk, and valuation contraction were reviewed before upside.
- [ ] Position size, concentration, liquidity needs, and portfolio fit were checked.
- [ ] The stock was compared against alternatives such as diversified exposure, cash, bonds, or another opportunity.
- [ ] The time horizon, review cadence, and exit or trim conditions are explicit.
- [ ] Leverage, dilution, governance, accounting quality, and capital structure were considered where relevant.
- [ ] Tax, account type, transaction cost, and liquidity constraints were not ignored.
- [ ] The conclusion avoids presenting a stock purchase as suitable without investor-specific goals and risk tolerance.
