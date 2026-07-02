---
name: government_bond_considerations.md
description: Use when the agent is evaluating sovereign or government bonds, assessing credit quality and currency risk of foreign sovereigns, comparing nominal versus real yields, weighing safe-haven demand, or deciding duration and sizing for a government-bond allocation.
---

# Government Bond Considerations

Government bonds are treated as the risk-free benchmark, but "government" is not synonymous with "risk-free." Sovereigns default, restructure, impose capital controls, inflate away debt, and print currency. The risk profile of a government bond depends on which government, which currency, what inflation regime, who the marginal buyer is, and what the investor actually needs the bond to do in the portfolio.

Use this skill before answering questions such as "are government bonds safe", "should I hold foreign government bonds", "do I need a government-bond allocation", or "what matters when buying sovereign debt". The goal is to prevent the agent from treating all government bonds as interchangeable safe assets, and to prevent the assumption that "risk-free" means "no loss possible."

## Core Rules

### Distinguish Credit Quality Across Sovereigns

Not all governments borrow in their own fiat currency, and not all that do are equally credible.

- Reserve-currency sovereigns (e.g., US, UK, Japan, in their own currency) have essentially zero nominal default risk because they can create the currency. Their real risk is inflation and currency depreciation, not missed payments.
- Sovereigns that borrow in a foreign currency, or that are part of a monetary union without full fiscal backing (e.g., some euro-area members), carry genuine credit and restructuring risk.
- Emerging-market sovereigns can and do default or restructure; spreads and ratings reflect this.

Ask whether the sovereign prints the currency it owes. That single fact changes the nature of the risk from default to inflation/currency.

### Treat Currency As A First-Order Risk For Foreign Sovereigns

A foreign government bond's return to a given investor is the bond's local-currency return multiplied by currency movement. Currency can dominate.

- A "safe" foreign sovereign bond can lose a large fraction of its value in the investor's home currency if the foreign currency depreciates.
- Hedging currency removes exchange risk but adds hedging cost that can erase the yield advantage of higher-yielding foreign bonds.
- Unhedged foreign bonds add a currency bet that may or may not diversify.

Never compare foreign sovereign yields to domestic yields without stating whether the comparison is currency-hedged and what the hedging cost implies.

### Compare Nominal Yield Against Inflation To Get Real Return

The coupon matters less than the real yield — nominal yield minus expected inflation.

- A high nominal yield in a high-inflation country may offer a negative real return.
- A low nominal yield with low, stable inflation may preserve real purchasing power better.
- For inflation protection, inflation-linked government bonds (e.g., TIPS) provide a contracted real yield and should be compared on a real-yield basis.

Do not recommend a government bond by its headline coupon without checking whether inflation is expected to erode it.

### Define What Role The Bond Plays In The Portfolio

Government bonds can serve several distinct purposes, and the right choice depends on the purpose:

- Capital preservation / liquidity reserve: short-duration, high-quality, domestic sovereigns; accept low yield for stability and instant liquidity.
- Diversification against equity drawdowns: long-duration high-quality sovereigns that tend to rally in flight-to-quality; the diversification benefit depends on the rate regime and can break in inflationary stress.
- Income: longer or higher-yielding sovereigns, accepting more rate and reinvestment risk.
- Real-return hedge: inflation-linked sovereigns.

A bond that is excellent for capital preservation may be poor for diversification, and vice versa. State the role before choosing the instrument.

### Recognize That The Equity-Diversification Benefit Is Regime-Dependent

The classic case for long government bonds is their negative correlation to equities during growth shocks — they rally when stocks fall. But this correlation is not stable.

- In inflation-driven sell-offs (e.g., 2022), both stocks and long bonds can fall together, removing the diversification.
- When starting yields/real yields are very low, bonds have limited room to rally and more room to fall.
- The diversification benefit is strongest from high-quality, long-duration sovereigns, and weakest from short or low-quality bonds.

Backtest the diversification claim across regimes, including inflationary ones, not just the recent low-rate period.

### Assess Demand And Supply Technicals

Government-bond prices are heavily influenced by structural buyers and issuance.

- Central bank QE/QT directly changes demand.
- Banks, pension funds, and insurers have regulation-driven demand at specific maturities.
- Foreign reserve managers and safe-asset demand concentrate in the highest-quality sovereigns.
- Heavy issuance can cheapen a curve even when fundamentals are stable.

Supply and demand technicals can move yields independently of growth and inflation views. Factor them in for timing, especially at long maturities.

### Check Tax, Withholding, And Account Treatment

Some sovereigns (e.g., certain munis, some government-backed instruments) have tax advantages. Foreign sovereign bonds may carry withholding tax that is partly recoverable depending on the investor's residency and account type. These affect after-tax yield and should be checked, not assumed.

## Common Traps

### Equating "Government Bond" With "Risk-Free"

Only the highest-quality sovereigns in their own fiat currency are close to default-free, and even then only in nominal terms. Real and currency risk remain. Many governments are not in this category.

### Chasing Foreign Yield Without Hedging Cost

Higher foreign sovereign yields often disappear once currency hedging costs (driven by interest-rate differentials) are included. The "free" extra yield is frequently an illusion.

### Assuming Long Bonds Always Diversify Equities

The negative stock-bond correlation is a feature of specific disinflationary regimes. In inflationary regimes the correlation can turn positive and both assets fall together.

### Ignoring Real Yield When Inflation Is Elevated

A 5% nominal yield with 6% inflation is a losing real return. Headline nominal yields mislead when inflation is high or unanchored.

### Treating All Maturities As The Same Asset

Short T-bills, intermediate notes, and long bonds have very different risk and diversification profiles. "I hold government bonds" is not a complete description of risk.

### Overlooking Negative-Yielding Or Very-Low-Yield Risk

At very low or negative yields, bonds have asymmetric downside: limited room to rally, meaningful room to fall, and a guaranteed nominal loss to maturity if held (when negative). The risk/reward is structurally poor.

## Self-Check

- [ ] The sovereign's credit quality is assessed, including whether it borrows in its own fiat currency versus foreign currency or monetary union.
- [ ] Currency risk for foreign sovereigns is addressed, including whether the comparison is hedged and the hedging cost's effect on yield.
- [ ] Real yield (nominal minus expected inflation) is computed, not just nominal coupon, especially when inflation is elevated.
- [ ] The bond's role in the portfolio (preservation, diversification, income, real-return hedge) is stated and the instrument matches that role.
- [ ] The equity-diversification claim is tested across regimes, including inflationary periods, not assumed from recent history.
- [ ] Demand and supply technicals (QE/QT, issuance, structural buyers) are considered for maturity and timing choices.
- [ ] Tax, withholding, and account-type effects on after-tax yield are checked.
- [ ] The conclusion avoids presenting government bonds as universally safe and references investor-specific horizon, currency, and risk tolerance.
