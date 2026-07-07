---
name: convertible_bond_mechanics.md
description: Use when the agent is analyzing convertible bonds, evaluating conversion premium and ratio, assessing coupon and yield-to-maturity versus conversion value, modeling the bond floor and equity upside, or understanding how convertibles behave across equity, credit, and interest rate scenarios for a corporate issuer or investor.
---

# Convertible Bond Mechanics

A convertible bond is a hybrid instrument that combines a fixed-income bond with an embedded equity call option. It pays coupons and returns principal like a bond if the issuer's stock does not rise, but it can be converted into shares if the stock appreciates, allowing participation in equity upside. Because it spans debt and equity, its behavior changes dramatically with the underlying stock price, the issuer's credit quality, and interest rates. Investors and analysts who treat a convertible as "a bond with a bonus" or "cheap equity" misprice it: the conversion premium, the bond floor, the reset features, and the credit component all interact, and the instrument's risk profile shifts across regimes in ways plain bonds and plain stocks do not.

Use this skill before answering questions such as "is this convertible cheap or expensive", "what happens to this convertible if the stock falls", "what is the conversion premium", or "how does a convertible behave in a down market". The goal is to prevent the agent from reducing a convertible to a single yield or a single equity projection, and from missing the bond floor, the conversion terms, the dilution, and the credit risk that together determine its value and risk.

## Core Rules

### Understand The Two Components And How They Combine

A convertible bond's value comes from two components that interact, not add linearly:

- Bond component (bond floor): the present value of the remaining coupons and principal, discounted at a rate reflecting the issuer's credit spread and prevailing interest rates. This is what the bond is worth if conversion never happens.
- Equity component (conversion option): the right to convert into a fixed number of shares. Its value rises with the stock price, volatility, and time to maturity.

The total value is the bond floor plus the value of the embedded option, but the option's value depends on the bond's terms (conversion ratio, reset, call features). An investor buying a convertible is buying a bond plus a call option on the issuer's stock, and the price should reflect both.

### Model The Conversion Ratio, Conversion Price, And Premium

The conversion mechanics define the equity participation:

- Conversion ratio: the number of shares received per bond upon conversion, fixed at issuance (unless reset).
- Conversion price: the effective price per share paid via conversion, equal to par divided by the conversion ratio.
- Conversion premium: how much the conversion price exceeds the current stock price, usually expressed as a percentage at issuance (often 20-40%).

The conversion premium is the "cost" of the optionality. A high premium means the stock must rise substantially before conversion pays off; a low or negative premium means the convertible trades close to or below its conversion value and behaves more like equity. Always compute the current conversion value (conversion ratio times stock price) and compare it to the bond's market price.

### Identify The Bond Floor And Its Credit Sensitivity

The bond floor is the convertible's downside protection, but it is not fixed. It moves with:

- Interest rates: rising rates lower the present value of fixed coupons and principal, dropping the floor.
- Credit spread: if the issuer's credit deteriorates, the discount rate rises and the floor falls. A "busted convertible" trades near a depressed floor when the stock collapses and the credit worsens simultaneously.

An investor relying on "par at maturity" as the floor ignores that the bond floor erodes with credit and rate moves. In distress, the floor can fall far below par, and the convertible behaves like impaired debt, not like protected principal.

### Map Behavior Across Equity Regimes

A convertible's risk profile is regime-dependent:

- Deep in-the-money (stock well above conversion price): the convertible trades like equity, moving nearly dollar-for-dollar with the stock; the bond floor provides little protection because conversion value dominates.
- At-the-money (stock near conversion price): the convertible exhibits convexity, gaining more from stock rises than it loses from falls; this is where the embedded option is most valuable.
- Out-of-the-money / busted (stock well below conversion price): the convertible trades like a bond, near its floor; equity upside is minimal and credit risk dominates.

Modeling only one regime misleads. An investor who buys for the "bond floor" but holds a deep in-the-money convertible is holding equity risk; an investor who buys for "equity upside" but holds a busted convertible is holding credit risk.

### Account For Call, Put, And Reset Provisions

Convertibles carry embedded options that affect both issuer and investor:

- Call provision: the issuer can redeem the bond early, usually above par, which caps investor upside and forces conversion when the stock is high. Soft calls (only exercisable if the stock exceeds a threshold) and hard calls differ.
- Put provision: the investor can put the bond back to the issuer at a set price on specific dates, providing downside protection; valuable when credit or rates deteriorate.
- Reset / refix features: the conversion price adjusts downward if the stock falls below a threshold by a reset date, increasing the conversion ratio and transferring value to investors; common in Asian markets and in weaker credits.

Ignoring call features overvalues the convertible by assuming unlimited upside; ignoring resets undervalues convertibles that have downside protection built into the terms.

### Assess Dilution And Capital Structure Impact

Conversion creates new shares, diluting existing shareholders. The size of potential dilution depends on the conversion ratio and the number of convertibles outstanding. For the issuer, convertibles are a way to raise equity at a premium (the conversion price) while paying a lower coupon than straight debt. For investors in the common stock, an outstanding convertible represents overhanging dilution that affects per-share value, especially as the stock approaches the conversion price.

### Separate Investment View From Arbitrage View

Two very different investor types hold convertibles:

- Straight investors: buy for the combination of downside protection and upside participation, holding the bond to capture coupons and potential conversion.
- Convertible arbitrageurs: buy the convertible and short the underlying stock to isolate and trade the embedded option's mispricing, delta-hedging as the stock moves.

The same convertible can look attractive to one and unattractive to the other. Know which perspective the analysis serves, because the relevant metrics (yield, premium, delta, gamma, implied volatility) differ.

## Common Traps

### Treating The Convertible As "A Bond With A Bonus"

Reducing the convertible to its coupon and ignoring the conversion terms, premium, and option value misprices the instrument. The optionality is the central feature, not a side benefit.

### Assuming Par Is The Floor

The bond floor erodes with rising rates and widening credit spreads. In distress, the floor can fall well below par, and the convertible behaves like impaired debt.

### Modeling A Single Equity Scenario

Projecting one stock price outcome ignores the regime-dependent behavior. A convertible's value and risk differ sharply between in-the-money, at-the-money, and busted scenarios.

### Ignoring Call Features That Cap Upside

An issuer's call right limits investor upside and can force conversion at unfavorable terms. Valuing the convertible as if it can never be called overstates its worth.

### Overlooking Reset Provisions

Conversion price resets transfer value to investors when the stock falls. Ignoring resets undervalues convertibles that carry this protection.

### Forgetting The Credit Component

The bond floor depends on the issuer's credit. A deteriorating credit drops the floor and can turn a "safe" convertible into a distressed position.

### Confusing Straight Investment With Arbitrage Analysis

Metrics that matter to a delta-hedging arbitrageur (implied volatility, gamma) differ from those that matter to a straight investor (yield, premium, floor). Applying the wrong lens produces poor conclusions.

## Self-Check

- Are the bond component (floor) and equity component (conversion option) both identified, and is the total value understood as their interaction rather than a simple sum?
- Is the conversion ratio, conversion price, and current conversion premium computed, with conversion value compared to market price?
- Is the bond floor modeled with sensitivity to interest rates and credit spread, rather than assumed to be par?
- Is the convertible's behavior mapped across in-the-money, at-the-money, and busted equity regimes, rather than a single scenario?
- Are call, put, and reset provisions accounted for, including their effect on upside, downside protection, and value transfer?
- Is the potential dilution from conversion assessed for its impact on the capital structure and existing shareholders?
- Is the analysis perspective (straight investment versus arbitrage) explicit, with the appropriate metrics used for each?
- Does the conclusion avoid treating the convertible as either pure debt or pure equity, and reference the credit, rate, and equity factors that drive its value?
