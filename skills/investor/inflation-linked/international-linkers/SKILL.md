---
name: international_linkers.md
description: Use when the agent is evaluating foreign inflation-linked government bonds, assessing currency and inflation-definition differences, tax and withholding treatment, liquidity, and cross-country correlation of real-yield and inflation exposure.
---

# International Linkers

Inflation-linked bonds exist outside the US — UK linkers, German Bunds, French OATi/OAT€i, Japanese JGBi, Canadian Real Return Bonds, and emerging-market linkers. They offer access to other countries' real yields and inflation experiences, which can diversify a US-centric inflation hedge. But foreign linkers layer on currency risk, different inflation-index definitions, withholding tax, thinner liquidity, and differing correlations. Treating a foreign linker as "a TIPS in another currency" hides first-order differences.

Use this skill before answering questions such as "should I buy foreign inflation-linked bonds", "how do UK linkers differ from TIPS", or "do international linkers diversify inflation risk". The goal is to prevent the agent from recommending foreign linkers on real-yield comparisons alone, and from missing currency, index-definition, tax, and liquidity effects that dominate returns.

## Core Rules

### Treat Currency As The Dominant Risk For Unhedged Foreign Linkers

A foreign linker's return to a US investor is its local-currency real return plus foreign-currency movement. Currency volatility is typically far larger than real-yield or inflation-adjustment differences.

- An unhedged foreign linker is mostly a currency bet with an inflation overlay, not a clean inflation hedge.
- Currency hedging removes exchange risk but adds hedging cost (driven by interest-rate differentials) that can erase the real-yield advantage.
- Hedged foreign linkers isolate the foreign real yield and inflation experience; unhedged ones mix in currency.

Decide explicitly whether the investor wants foreign real-yield exposure (hedge currency) or foreign-currency-plus-inflation exposure (unhedged). These are different investments.

### Verify The Inflation Index Definition And Lag

Each country links to its own CPI/RPI/HICP measure, and the definitions differ materially:

- UK linkers use RPI, which has known methodological quirks (including a "formula effect") that make it run higher than CPIH; this is priced into UK linker yields.
- Euro-area linkers (OATi, Bundi) use euro-area HICP ex-tobacco, a supranational index.
- Japanese JGBi use core CPI excluding fresh food.
- Indexation lags differ (typically 2-3 months), affecting how quickly the bond responds to inflation prints.
- Some linkers have a deflation floor; some do not.

Do not assume all linkers track the same inflation. An investor hedging US CPI with a UK linker is hedging RPI, not US CPI — the inflation exposures are different.

### Compare Real Yields On A Like-For-Like, Currency-Adjusted Basis

Foreign real yields can look attractive, but the comparison must be fair:

- Compare hedged real yields (after currency-hedging cost), not headline local real yields.
- Adjust for the different inflation index: a 2% real yield on RPI is not directly comparable to a 2% real yield on US CPI.
- Account for the deflation floor: a linker without a floor is riskier in deflation and should offer a higher real yield.
- Consider sovereign credit quality: most developed linkers are high-quality, but EM linkers carry default and restructuring risk.

A "higher real yield abroad" often shrinks or reverses after hedging cost and index differences.

### Assess Liquidity And Market Structure

Foreign linker markets vary widely in depth:

- UK and French linker markets are deep and actively traded.
- German and Japanese linker markets are smaller and less liquid.
- EM linkers can be illiquid, with wide spreads and limited dealer support.
- On-the-run versus off-the-run liquidity gaps are larger than in nominal markets.

Thinner liquidity means wider bid-ask, harder exit in stress, and larger liquidity premia embedded in real yields. An apparent real-yield pickup may be a liquidity premium, not a genuine return opportunity.

### Model Tax And Withholding Treatment

Foreign linkers create cross-border tax complexity:

- Withholding tax on interest (and sometimes on the inflation adjustment) at the source country's rate, partly recoverable via treaty depending on the investor's residency.
- Phantom income tax on the inflation accrual in the investor's home country (for US investors, the inflation adjustment on a foreign linker is generally taxable as it accrues).
- Foreign tax credit limitations and account-type interactions.

For US taxable investors, foreign linkers combine phantom income with withholding and foreign-tax-credit complexity, often making them inefficient in taxable accounts. Tax-advantaged accounts are usually preferable, but withholding may still apply.

### Evaluate Diversification And Correlation Benefits

Foreign linkers can diversify because inflation regimes and real yields differ across countries:

- Different inflation shocks (commodity, wage, fiscal) affect countries differently.
- Real-yield curves are not perfectly correlated; foreign linkers add spread of real-yield outcomes.
- However, in global inflation shocks (e.g., energy-driven), linkers correlate and the diversification benefit shrinks.

Test the diversification claim across regimes, including common-shock scenarios. Diversification that holds in normal times can fail in the very inflationary shock the investor is hedging against.

### Consider Sovereign Credit And Political Risk

Most developed-market linkers are issued by high-quality sovereigns, but:

- EM linkers carry default, restructuring, and capital-controls risk; the inflation adjustment is only as good as the sovereign's willingness and ability to pay.
- Some sovereigns have a history of interfering with indexation, changing CPI methodology, or restructuring linker debt in distress.
- Political risk can affect both the inflation index and the payment.

For EM linkers, sovereign credit analysis is as important as the inflation mechanics. A high real yield can be compensation for default and indexation risk.

## Common Traps

### Comparing Foreign Real Yields To TIPS Without Currency Hedging

Unhedged foreign real yields are not comparable to US real yields because currency dominates the return. Compare hedged, or state that the position is a currency bet.

### Assuming All Linkers Track The Same Inflation

RPI, HICP, core CPI, and US CPI differ materially. Hedging US inflation with a foreign linker hedges a different index and may not perform as expected.

### Ignoring Phantom Income And Withholding Tax

Foreign linkers layer home-country phantom income tax with source-country withholding. After-tax real return can be poor, especially in taxable accounts.

### Chasing High Foreign Real Yields That Are Liquidity Or Credit Premia

A high real yield in a thin or lower-quality market often compensates for illiquidity or default risk, not a genuine return opportunity.

### Treating Foreign Linkers As A Clean US Inflation Hedge

Unhedged foreign linkers hedge foreign inflation plus currency, not US CPI. They diversify but do not cleanly protect US purchasing power.

### Overlooking Deflation-Floor Differences

Some linkers lack a maturity deflation floor, increasing downside in deflation. A real-yield comparison that ignores the floor understates risk.

## Self-Check

- [ ] Currency risk is addressed explicitly, with a stated choice between hedged (foreign real-yield exposure) and unhedged (currency-plus-inflation exposure).
- [ ] The specific inflation index (RPI, HICP, core CPI, US CPI) and indexation lag are identified, and cross-index comparability is questioned.
- [ ] Real yields are compared on a hedged, index-adjusted, floor-adjusted, and credit-adjusted basis, not headline local yields.
- [ ] Liquidity and market structure are assessed, and apparent real-yield pickups are checked for liquidity premia.
- [ ] Tax treatment (phantom income plus withholding plus foreign tax credit) is modeled, and tax-advantaged accounts are preferred where appropriate.
- [ ] Diversification and correlation benefits are tested across regimes, including common global inflation shocks.
- [ ] Sovereign credit and political/indexation risk are assessed, especially for EM linkers.
- [ ] The conclusion avoids presenting foreign linkers as a clean US inflation hedge and references investor-specific currency view, tax situation, and risk tolerance.
