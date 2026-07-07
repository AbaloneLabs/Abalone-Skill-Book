---
name: convertible_term_analysis.md
description: Use when the agent is reviewing a convertible bond's term sheet, evaluating call and put schedules, reset and refix provisions, conversion price adjustments, mandatory conversion features, or assessing how specific terms shift value between issuer and investor and affect the instrument's risk and return profile.
---

# Convertible Term Analysis

The headline numbers of a convertible bond, coupon, conversion premium, and maturity, describe only the surface. The detailed terms in the indenture and term sheet determine who benefits when the stock rises, falls, or stays flat, who can force whose hand, and how value shifts under contingent events. Two convertibles with identical coupons and premiums can have vastly different risk and return profiles because one has a soft call that caps upside, a reset that protects investors in down markets, and a put that floors value, while the other has none of these. Investors who read the summary and skip the terms, or who assume "standard" terms, misjudge the instrument and are surprised when provisions activate. Term analysis is the discipline of reading the contingencies that govern the convertible's actual behavior.

Use this skill when reviewing a convertible's term sheet, comparing convertibles on terms beyond coupon and premium, or assessing how specific provisions (calls, puts, resets, mandatory conversion, contingent conversion) affect value. The goal is to prevent the agent from treating the convertible's terms as boilerplate and from missing the provisions that materially shift economics between issuer and investor.

## Core Rules

### Read The Call Schedule And Call Protection

The issuer's call right is the most important term affecting investor upside:

- Hard call: the issuer can redeem at any time after a protection period, usually at a small premium to par that declines over time.
- Soft call (provisional call): the issuer can call only if the stock trades above a threshold (often 130-150% of conversion price) for a sustained period, forcing conversion and capping investor upside.
- Call protection: the initial period during which the convertible cannot be called, giving investors time to capture upside.

A convertible with an early soft call effectively caps the investor's participation once the stock runs; the issuer calls, forces conversion, and the investor holds common stock instead of the bond's downside protection. Value the call as a short option the investor has sold to the issuer.

### Evaluate The Put Schedule As Investor Protection

The investor's put right provides downside protection and is especially valuable for weaker credits:

- Put dates and prices: specific dates (often years 3 or 5) on which the investor can put the bond back to the issuer at a set price (often par or slightly above).
- Effect on floor: a put at par effectively raises the bond floor to par on the put date, protecting against credit and rate deterioration before then.

A put transforms the convertible's risk profile by guaranteeing an exit at a known price, which matters enormously if the issuer's credit weakens. Ignoring the put undervalues convertibles that carry it and overstates their credit risk.

### Analyze Reset And Refix Provisions

Resets adjust the conversion price based on the stock's performance, usually to the investor's benefit in down markets:

- Downward-only reset: if the stock falls below a threshold by a reset date, the conversion price drops (conversion ratio rises), increasing equity participation at a lower effective price.
- Repeated resets: some convertibles (common in certain Asian markets) reset multiple times, compounding investor protection in sustained declines.
- Effect on dilution: resets increase potential dilution for existing shareholders and transfer value from issuer to investor.

A convertible with a reset is a different instrument than one without: it carries built-in downside protection that a static analysis misses. Always model the conversion ratio under reset scenarios, not just the original terms.

### Check Mandatory And Contingent Conversion Features

Some convertibles constrain when conversion can occur:

- Mandatory convertibles (PEPS, PERCS, DECS): automatically convert at maturity to a variable number of shares based on the stock price, with no bond floor at maturity; they are equity-like from the start and offer higher coupon as compensation.
- Contingent conversion: conversion is allowed only if the stock exceeds a threshold, limiting investor flexibility to convert into a falling stock.

Mandatory convertibles trade and behave differently from optional convertibles: they have little bond floor, high delta from inception, and are essentially forward equity purchases with enhanced coupon. Treating them like optional convertibles misstates their risk.

### Review Conversion Price Adjustment Triggers

The conversion price adjusts on corporate actions to protect the conversion ratio's economic value:

- Stock splits and dividends: the conversion ratio adjusts to preserve value.
- Special distributions, rights offerings, and recapitalizations: defined adjustment formulas apply.
- Change of control: may trigger a put or an adjustment, affecting value in M&A scenarios.

Failing to apply adjustments on corporate actions distorts the conversion value. Read the adjustment provisions to understand how the convertible behaves in mergers, spin-offs, and special distributions.

### Assess The Interaction Of Terms, Not Each In Isolation

Terms interact. A convertible with a soft call and a reset has a complex payoff: the reset protects in down markets, but the soft call caps upside in up markets, creating a "band" of stock prices where the investor captures the most value. A convertible with a put and contingent conversion has downside protection at the put date but constrained conversion flexibility. Model the combined effect of provisions across stock price scenarios rather than evaluating each term separately.

### Compare Terms Across The Capital Structure

A convertible's terms should be read alongside the rest of the issuer's capital structure:

- Seniority: convertibles are typically unsecured and subordinated, sitting below secured debt in bankruptcy.
- Covenants: restrictive covenants on the issuer may protect the convertible, while loose covenants permit actions that harm it.
- Other convertibles and preferreds: multiple convertible issues create a stack that affects recovery and dilution.

A convertible is not analyzed in isolation; its terms interact with the issuer's broader obligations and covenants.

## Common Traps

### Reading Only Coupon And Premium

The summary metrics describe the surface. The call, put, reset, and conversion provisions determine the actual risk and return.

### Ignoring The Soft Call That Caps Upside

A soft call lets the issuer force conversion once the stock runs, limiting investor participation in the biggest upside scenarios. Valuing the convertible as if upside is uncapped overstates its worth.

### Overlooking The Put As Downside Protection

A put at par on an intermediate date materially raises the floor and protects against credit deterioration. Ignoring it understates the convertible's safety.

### Missing Reset Provisions

Resets transfer value to investors in down markets. A static conversion ratio analysis undervalues convertibles that reset.

### Treating Mandatory Convertibles Like Optional Ones

Mandatory convertibles have little bond floor and high delta from inception. Applying optional-convertible logic misstates their equity-like risk.

### Forgetting Conversion Price Adjustments

Corporate actions trigger adjustments that preserve conversion value. Failing to apply them distorts the conversion value and dilution analysis.

### Evaluating Terms In Isolation

Terms interact. A convertible's payoff depends on the combined effect of calls, puts, resets, and conversion features across scenarios, not each provision alone.

## Self-Check

- Is the call schedule analyzed, distinguishing hard and soft calls, and is the call valued as a short option that caps investor upside?
- Is the put schedule identified and valued as downside protection that raises the effective floor, especially for weaker credits?
- Are reset and refix provisions modeled, including their effect on the conversion ratio and dilution in down-market scenarios?
- Are mandatory and contingent conversion features distinguished from optional conversion, with mandatory convertibles treated as equity-like?
- Are conversion price adjustment triggers (splits, dividends, change of control) reviewed for their effect on conversion value in corporate actions?
- Is the interaction of terms modeled across stock price scenarios, rather than each provision evaluated in isolation?
- Is the convertible's seniority, covenants, and position in the capital structure assessed alongside its terms?
- Does the conclusion compare convertibles on the full term profile, not just coupon and premium, and reference the provisions that shift value between issuer and investor?
