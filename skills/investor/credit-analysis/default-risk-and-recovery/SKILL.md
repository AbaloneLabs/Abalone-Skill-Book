---
name: default_risk_and_recovery.md
description: Use when the agent is assessing default probability, credit ratings, recovery rates, collateral and seniority, expected loss, or the difference between senior and subordinated debt before forming a view on credit risk or a distressed investment.
---

# Default Risk And Recovery

Credit investing is fundamentally about two questions: will the borrower pay, and if not, how much will I recover? Expected loss is the product of default probability and loss-given-default, where loss-given-default is one minus the recovery rate. An investing agent often fixates on default probability and the credit rating while ignoring recovery, treating all bonds of an issuer as equally risky. This is a serious error. Two bonds of the same issuer can have identical default probability but radically different recovery because of seniority, collateral, and covenants. The expected loss, and therefore the fair yield, differ accordingly.

Use this skill before answering questions such as "how risky is this bond", "what is the recovery rate", "is this high-yield bond fairly priced", or "how do seniority and collateral affect credit risk". The goal is to prevent the agent from reducing credit risk to a single default probability and to force it to assess both default probability and recovery, understand seniority and collateral, compute expected loss, and distinguish investment-grade from distressed analysis.

Credit analysis involves real default and loss risk. Distressed investing is especially complex and illiquid. Conclusions should disclose uncertainty and recommend professional advice for high-risk or distressed situations.

## Core Rules

### Separate Default Probability From Loss Given Default

Expected loss has two independent components, and conflating them produces wrong pricing and wrong risk assessment.

Decompose:

- Default probability: the likelihood the issuer fails to pay on time.
- Loss given default: the fraction of par lost if default occurs, equal to one minus the recovery rate.
- Recovery rate: the fraction of par recovered after default, through restructuring, sale, or liquidation.
- Expected loss equals default probability times loss given default.

A high-yield bond with 5 percent default probability and 40 percent recovery has an expected loss of 3 percent. A subordinated bond of the same issuer with the same default probability but 10 percent recovery has an expected loss of 4.5 percent. The yield must compensate for the higher expected loss, and the risk profile is different even though default probability is identical.

### Assess Default Probability Through Multiple Lenses

Default probability is driven by the issuer's ability and willingness to pay. Use several lenses.

Assess:

- Financial strength: leverage, coverage ratios, cash flow stability, and liquidity.
- Business quality: cyclicality, competitive position, and earnings volatility.
- Capital structure and maturity profile: refinancing risk and debt burden.
- Industry and macro conditions: tailwinds or headwinds to cash generation.
- Management and governance: willingness to protect creditors versus shareholders.
- Event risks: acquisitions, leveraged buyouts, litigation, or regulatory action that could impair credit.

No single ratio predicts default. Combine quantitative leverage and coverage metrics with qualitative business and management assessment, and stress-test through a downturn.

### Understand Seniority And Its Effect On Recovery

Where a bond sits in the capital structure largely determines recovery. Seniority is the primary recovery driver.

The priority waterfall:

- Secured debt, backed by specific collateral, recovers most, often 70 percent or more.
- Senior unsecured debt recovers moderately, often 40 to 60 percent historically.
- Subordinated debt recovers less, often 20 percent or less.
- Preferred and hybrid securities recover even less.
- Common equity is last and often recovers nothing in distress.

Within an issuer, moving down the capital structure increases expected loss through lower recovery even at the same default probability. Price the additional risk with additional yield, and recognize that subordinated debt has more equity-like, asymmetric downside.

### Evaluate Collateral And Security

Secured debt is backed by specific assets, and the quality and realizability of that collateral drives recovery.

Assess:

- What assets secure the debt, and what is their market value, not book value?
- How liquid and realizable is the collateral in distress?
- Are there prior liens or intercreditor issues that reduce the effective claim?
- Is the collateral perishable, specialized, or location-bound, reducing fire-sale value?
- How much debt is secured ahead of the bond in question?

Collateral that is illiquid, specialized, or encumbered provides less protection than the security label suggests. Value the collateral at distress-sale prices, not going-concern book values.

### Compute Expected Loss And Compare To The Yield Spread

The purpose of decomposing expected loss is to judge whether the yield spread compensates for the risk.

Calculate:

- Expected loss as default probability times loss given default.
- The yield spread over the risk-free rate.
- Whether the spread exceeds expected loss by an adequate margin of safety.

If the spread barely exceeds expected loss, the bond is not compensating for the risk. If the spread far exceeds expected loss, there may be value, or the market may be pricing a default probability or recovery worse than your estimate. The gap between spread and expected loss is the credit risk premium, and judging whether it is adequate is the core of credit investing.

### Distinguish Investment-Grade From Distressed Analysis

The analytical focus differs sharply between investment-grade and distressed credit.

Investment-grade:

- Focus on rating stability, migration risk, and the issuer's ability to maintain quality through a cycle.
- Default probability is low; the main risk is spread widening from downgrade or macro stress.
- Recovery is less central because default is unlikely.

Distressed:

- Default or restructuring is plausible or expected; the analysis centers on recovery and the restructuring process.
- The bond trades on recovery value, not on yield to maturity.
- Legal process, collateral, covenants, and intercreditor dynamics dominate.
- The outcome depends on bankruptcy law, negotiation, and the restructuring plan.

Applying investment-grade yield analysis to distressed debt, or recovery analysis to investment-grade, misframes the opportunity. Match the method to the credit quality.

### Incorporate The Restructuring And Legal Process

Recovery in distress depends on the restructuring or bankruptcy process, which is legal, negotiated, and jurisdiction-specific.

Consider:

- The governing bankruptcy or insolvency law and its treatment of creditors.
- Whether the process is likely to be a negotiated restructuring or a liquidation.
- The role of collateral, covenants, and intercreditor agreements in allocating value.
- The timeline, which can be years, during which capital is locked.
- The potential for debt-to-equity swaps, which convert a bond claim into an equity stake of uncertain value.

Distressed investing requires understanding the legal process, not just the financials. Recovery is determined in court and negotiation, not on a spreadsheet.

## Common Traps

### Reducing Credit Risk To Default Probability Alone

Expected loss depends on recovery too. Two bonds with the same default probability can have very different expected losses.

### Treating Credit Ratings As Guarantees

Ratings are opinions that lag, change, and miss risks. They are one input, not a promise. Use them as a starting point, not a conclusion.

### Ignoring Seniority And Collateral

Where a bond sits in the capital structure and what secures it largely determine recovery. Ignoring these factors misprices the risk.

### Valuing Collateral At Book Value

Collateral realizes at distress-sale prices in default, not at book or going-concern value. Specialized or illiquid collateral recovers far less.

### Applying Yield Analysis To Distressed Debt

Distressed debt trades on recovery value, not yield to maturity. Yield analysis misframes the opportunity when default is expected.

### Overlooking The Restructuring Process And Timeline

Recovery is determined through a legal, negotiated process that can take years. Ignoring the process and timeline misestimates the outcome.

### Underestimating Event And Refinancing Risk

Acquisitions, buyouts, and maturity walls can impair credit suddenly. Static analysis misses the event risks that cause defaults.

## Self-Check

- [ ] Default probability and loss given default are separated, and expected loss is computed as their product.
- [ ] Default probability is assessed through financial, business, capital-structure, and qualitative lenses, with stress testing.
- [ ] Seniority and its effect on recovery are understood, with the bond's place in the priority waterfall identified.
- [ ] Collateral quality and realizability are assessed at distress-sale values, not book values.
- [ ] Expected loss is compared to the yield spread to judge whether compensation is adequate.
- [ ] Investment-grade and distressed analysis are distinguished, with the method matched to credit quality.
- [ ] The restructuring and legal process, including timeline and jurisdiction, are incorporated for distressed situations.
- [ ] Event and refinancing risks are considered, not just static ratios.
- [ ] Credit ratings are used as one input, not as guarantees.
- [ ] The conclusion discloses default and loss risk, flags the complexity and illiquidity of distressed investing, and recommends professional advice.
