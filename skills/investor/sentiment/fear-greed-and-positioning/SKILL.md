---
name: fear_greed_and_positioning.md
description: Use when the agent is interpreting fear and greed indices, VIX and volatility, put-call ratios, margin debt, or positioning data, and judging whether extremes signal contrarian opportunity or confirmation of trend before forming an investment view.
---

# Fear, Greed, And Positioning

Sentiment and positioning data describe the crowd's current posture, and at extremes they can signal turning points. But they are among the most misused indicators in investing. An investing agent often treats a high fear-greed reading or an extreme VIX as a mechanical buy or sell signal, ignoring that sentiment can stay extreme for long periods, that positioning data is noisy and lagging, and that the same extreme means different things in different regimes. Sentiment is a context-dependent input, not a standalone timing tool.

Use this skill before answering questions such as "is the fear-greed index a buy signal", "what does a high VIX mean", "is the market too crowded", or "how do I read positioning data". The goal is to prevent the agent from mechanically trading sentiment extremes and to force it to interpret indicators in context, distinguish genuine extremes from trending conditions, and combine sentiment with valuation, fundamentals, and structure.

Sentiment signals are probabilistic and can be early for long stretches. Trading them carries real risk, and conclusions should disclose uncertainty and investor-specific constraints.

## Core Rules

### Understand What Each Indicator Actually Measures

Each sentiment and positioning metric captures a different crowd posture. Know what you are looking at.

Key indicators:

- Fear and greed indices: composite of several inputs, summarizing sentiment in a single number; useful for context but blunt.
- VIX and implied volatility: the market's expectation of near-term volatility, reflecting hedging demand and uncertainty; high VIX signals fear, low VIX signals complacency.
- Put-call ratio: the balance of bearish versus bullish options activity; extremes can signal crowding.
- Margin debt: leverage in brokerage accounts; rising margin fuels rallies, falling margin signals de-risking.
- Fund flows and allocations: where capital is moving; persistent extreme allocations can signal crowding.
- Surveys and investor polls: direct sentiment readings; most useful at multi-year extremes.

No single indicator is sufficient. Each captures one slice of crowd behavior, and they can disagree.

### Interpret Extremes, Not Levels

Sentiment indicators are contrarian mainly at extremes. Moderate readings are noise.

Apply:

- Use multi-year or multi-cycle percentile thresholds to define extremes, not arbitrary levels.
- A fear-greed reading of 60 is not a sell signal; a reading at a multi-year high alongside euphoric positioning may be.
- VIX at 18 is normal; VIX at a cycle low with record short positioning signals complacency.
- The signal strength rises with the extremity of the reading and corroboration from other indicators.

Trading moderate sentiment readings produces whipsaw. Reserve sentiment-based action for genuine, confirmed extremes.

### Combine Multiple Corroborating Signals

A single extreme is a hint. Several extremes aligning across independent measures is evidence.

Build a sentiment mosaic:

- Are surveys, positioning, flows, and options activity all pointing the same way?
- Does price action confirm or diverge from sentiment?
- Is valuation also at an extreme in the same direction?
- Are credit spreads, breadth, and leadership consistent with the sentiment signal?

When sentiment is extreme but valuation, credit, and fundamentals disagree, the signal is weak. When all align, the contrarian case is stronger.

### Distinguish Trend-Confirming From Contrarian Uses

Sentiment can confirm a trend or signal its exhaustion, and the use depends on context.

- In a strong, early-stage trend, rising optimism and inflows can confirm and fuel the move; fading it is premature.
- In a late-stage, overextended trend, euphoric sentiment and crowded positioning signal exhaustion; fading it is contrarian.
- In a crash, maximum fear and capitulation can mark a bottom; buying requires courage and confirmation.

The same sentiment reading confirms a trend early and contradicts it late. The cycle position and the degree of extension determine which interpretation applies.

### Account For Structural Changes In The Indicators

Sentiment and positioning data evolve with market structure, and old benchmarks can mislead.

Consider:

- ETF and passive flows have changed how fund-flow data should be interpreted.
- Zero-commission retail and options activity have altered put-call and volume patterns.
- VIX behavior has shifted with structured products, volatility-selling strategies, and dealer positioning.
- Margin data may not capture all leverage, including derivatives and off-balance-sheet exposure.

Do not compare today's reading to historical benchmarks without asking whether the indicator means the same thing it did a decade ago.

### Separate Positioning From Value

Crowded positioning is not the same as overvaluation, and under-ownership is not the same as cheapness.

Clarify:

- A crowded, expensive asset has two reasons to fall; fading it is higher-conviction.
- A crowded but cheap asset may just be a popular quality stock; the risk is lower.
- An under-owned, expensive asset can keep falling if fundamentals disappoint.
- An under-owned, cheap asset is the classic contrarian opportunity.

Combine positioning with valuation to judge the quality of the contrarian signal.

### Respect That Extremes Can Persist

Sentiment extremes can last far longer than a contrarian expects. "Markets can stay irrational" applies to sentiment directly.

Manage:

- Use sentiment to size and time entries, not to make all-or-nothing bets.
- Pair contrarian sentiment signals with a catalyst or confirmation, since extremes often need a trigger to reverse.
- Accept that being early on a contrarian call can feel like being wrong for an extended period.
- Define invalidation conditions, since sentiment can always get more extreme.

## Common Traps

### Trading Moderate Sentiment Readings

Fading sentiment that is merely optimistic or pessimistic, not extreme, produces constant whipsaw and transaction costs.

### Using One Indicator In Isolation

A single fear-greed reading or VIX level is weak evidence. Confirmation across independent measures is needed.

### Ignoring That Extremes Can Persist

Sentiment can stay extreme for long periods. Acting as if an extreme guarantees an immediate reversal leads to painful early trades.

### Treating Positioning As Valuation

Crowding is not overvaluation. A crowded cheap stock and a crowded expensive stock carry very different risks.

### Applying Old Benchmarks To Changed Market Structure

ETF flows, zero-commission options, and volatility products have changed how sentiment data behaves. Stale benchmarks mislead.

### Fading A Strong Early Trend

In early-stage powerful trends, rising sentiment confirms rather than contradicts the move. Fading it prematurely is costly.

### Ignoring The Need For A Catalyst

Extremes often need a trigger to reverse. Buying maximum fear without any catalyst can mean catching a falling knife.

## Self-Check

- [ ] Each sentiment indicator is understood for what it actually measures, not treated as interchangeable.
- [ ] Extremes are defined by multi-year or multi-cycle percentiles, not arbitrary levels.
- [ ] Multiple independent signals are combined for corroboration.
- [ ] Trend-confirming and contrarian uses are distinguished by cycle position and extension.
- [ ] Structural changes in the indicators are accounted for when using historical benchmarks.
- [ ] Positioning is separated from valuation, and the two are combined to judge signal quality.
- [ ] The analysis acknowledges that extremes can persist and being early can feel like being wrong.
- [ ] A catalyst or confirmation is sought rather than acting on sentiment alone.
- [ ] Position sizing tolerates the signal being early or wrong.
- [ ] The conclusion avoids presenting sentiment as a reliable timing tool and flags uncertainty, investor horizon, and professional advice.
