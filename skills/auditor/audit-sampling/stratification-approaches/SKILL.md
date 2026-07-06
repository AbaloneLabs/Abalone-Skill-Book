---
name: stratification-approaches.md
description: Use when the agent is stratifying a population for audit sampling, deciding on stratification criteria, evaluating how stratification affects sample size and coverage, combining 100% key-item examination with sampling of residual strata, or assessing whether the stratification genuinely improves efficiency without creating coverage gaps.
---

# Stratification Approaches

Stratification is the technique of dividing a population into subgroups (strata) that are separately sampled or examined. Done well, it concentrates effort on the items that matter most (large, unusual, high-risk) and uses sampling efficiently for the homogeneous remainder, often dramatically reducing the total sample size for the same assurance. Done poorly, it creates coverage gaps — a stratum that no one tests, or a boundary that lets material items slip through — and produces a false sense of efficiency. The discipline is to stratify deliberately, to cover every stratum, and to confirm the stratification actually matches the risk structure of the population.

## Core Rules

### Stratify to reduce variability within strata, not just to sort items

The purpose of stratification is to make each stratum internally homogeneous so that a smaller sample represents it well. Effective stratification puts similar items together (e.g., all large balances in one stratum, all small recurring balances in another) so that the variability that drives sample size is contained within strata rather than spread across the population. Ineffective stratification sorts items into groups that are still internally variable, providing no efficiency gain. Test whether the chosen criteria actually reduce within-stratum variability; if they do not, the stratification is cosmetic.

### Use value-based stratification with a 100% top stratum where appropriate

The most common and powerful stratification is by monetary value: examine 100% of items above a threshold (the key-item or top stratum), and sample the residual. This works because a small number of large items often holds most of the monetary value, and examining them 100% removes sampling risk for that value. Confirm:

- the threshold is set so the top stratum captures a high proportion of total value (commonly 70–90% or more);
- the top stratum is genuinely examined 100%, not sampled;
- the residual stratum is sampled with a size appropriate to the value and risk remaining.

This combination is often the single most efficient sampling design for monetary populations.

### Stratify by risk characteristics, not only by value

Value is not the only relevant dimension. Stratify by risk when the population contains subgroups with different misstatement likelihood:

- related-party vs third-party balances;
- old vs new items (ageing for receivables, vintage for inventory);
- items subject to estimation vs items at fixed amounts;
- items from systems or locations with different control environments;
- items involving new products, customers, or vendors vs established ones.

Sample higher-risk strata more heavily, and consider 100% examination of the highest-risk subgroup. Risk-based stratification catches errors that value-based stratification alone would miss.

### Ensure every stratum is covered — no untested gaps

The most dangerous stratification error is the orphan stratum: a subgroup that is defined but never sampled, because attention concentrated on the top and bottom strata. For every stratum defined, confirm there is a testing plan: 100% examination, a sample, or an analytical procedure. Map strata to procedures explicitly so no value and no risk falls through the cracks. A stratification that leaves a material stratum untested has reduced effort by reducing assurance — a bad trade.

### Set stratum boundaries deliberately and defend them

The boundaries between strata (the value threshold for the top stratum, the risk criteria for risk strata) determine efficiency and coverage. Set them deliberately:

- a top-stratum threshold set too high leaves large items in the residual stratum, requiring a larger residual sample;
- a threshold set too low makes the 100% stratum unwieldy;
- risk boundaries should reflect genuine differences in misstatement likelihood, not arbitrary splits.

Document why the boundaries were chosen and confirm they match the population's value and risk distribution.

### Evaluate each stratum separately and aggregate carefully

Each stratum is sampled and evaluated on its own terms: project misstatement within the stratum, compare to the stratum's tolerable misstatement, and conclude on the stratum. Then aggregate stratum conclusions to the population. Two cautions:

- do not offset a misstatement in one stratum against a clean result in another without considering whether the aggregation is meaningful;
- when aggregating statistically, combine precisions correctly; a judgemental aggregation of statistical stratum results loses the statistical properties.

State the evaluation method at stratum and population level so the conclusion is defensible.

### Re-confirm stratification when the population changes

Stratification that fit last year's population may not fit this year's. Re-examine the population's value distribution and risk structure each period:

- has the value concentration shifted (more large items, or fewer)?
- have new risk subgroups emerged (new products, acquisitions, new systems)?
- have prior strata become immaterial?

Rolling forward last year's stratification without re-examination can leave new concentrations under-covered or waste effort on now-immaterial strata.

### Use stratification to enable the right procedure for each subgroup

Stratification is not only about sample size; it lets the auditor apply the most efficient procedure to each subgroup:

- 100% confirmation of large receivables;
- sampling and recalculation for mid-size items;
- analytical procedures for small homogeneous items;
- specific procedures for related-party or estimate items.

Match the procedure to the stratum's characteristics. This is where stratification delivers both efficiency and effectiveness: the right evidence for each part of the population.

### Consider negative (zero or small) populations explicitly

Populations often contain many zero, small, or negative items (dormant accounts, credit balances, reversed entries). Decide deliberately how to handle them: include them in the residual stratum (they may indicate completeness or classification issues), exclude them with documented rationale, or examine them separately for unusual patterns. Excluding them silently is a coverage gap; many frauds and errors hide in the tails of the distribution.

## Common Traps

- **Defining strata that do not reduce within-stratum variability**, providing no efficiency gain — stratification in form only.
- **Leaving an orphan stratum untested**, a coverage gap that trades assurance for effort savings.
- **Setting the top-stratum threshold too high**, leaving large items in the residual and inflating the residual sample.
- **Stratifying only by value, missing risk subgroups** (related-party, aged, estimated) that hold disproportionate error likelihood.
- **Aggregating stratum results incorrectly**, offsetting misstatements across strata or losing statistical precision in judgemental aggregation.
- **Rolling forward last year's stratification** without re-examining whether the population's value and risk distribution has changed.
- **Applying the same procedure to every stratum**, missing the efficiency of matching procedure to stratum characteristics.
- **Silently excluding zero, small, or negative items**, leaving the tails of the distribution — where frauds and errors often hide — unexamined; **Failing to document stratum boundaries and rationale**, making the design indefensible if challenged

## Self-Check

- Does my stratification reduce within-stratum variability, so each stratum is internally homogeneous and a smaller sample represents it well?
- Did I set a top stratum examined 100% that captures a high proportion of total value, and size the residual sample appropriately?
- Did I stratify by risk characteristics (related-party, ageing, estimates, systems) in addition to value, and sample higher-risk strata more heavily?
- Is every stratum covered by a defined procedure — no orphan strata with no testing plan?
- Are stratum boundaries set deliberately and documented, matching the population's value and risk distribution?
- Did I evaluate each stratum separately and aggregate conclusions correctly, without offsetting misstatements or losing statistical precision?
- Did I re-examine the population this period rather than rolling forward last year's stratification?
- Did I match the procedure to each stratum (confirmation, recalculation, analytics, specific procedures) rather than applying one method everywhere?
- Did I handle zero, small, and negative items deliberately — included, excluded with rationale, or examined separately — rather than silently dropping them?
