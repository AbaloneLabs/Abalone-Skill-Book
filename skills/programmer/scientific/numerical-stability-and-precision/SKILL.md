---
name: numerical_stability_and_precision.md
description: Use when the agent is writing floating-point arithmetic, implementing numerical algorithms (linear solvers, ODE integrators, matrix factorizations, FFTs, statistical reductions), choosing between single and double precision, summing many numbers, comparing floats for equality, diagnosing loss of precision, catastrophic cancellation, ill-conditioned problems, or results that differ between runs or platforms; or deciding whether to use a stable algorithm over a faster unstable one. Covers IEEE 754 semantics, conditioning, cancellation, accumulation error, and the tradeoffs of precision versus performance.
---

# Numerical Stability And Precision

Floating-point arithmetic is not real arithmetic, and the gap between them is where numerical bugs live. IEEE 754 doubles are finite, rounded, and non-associative: `(a + b) + c` does not in general equal `a + (b + c)`, ten thousand additions of a small number to a large one can lose all the small contributions, subtracting two nearly-equal numbers can wipe out most of the significant digits, and the same algorithm can produce different results on different runs because of summation order or parallel reduction. A program that is correct in real-number arithmetic can be wildly wrong in floating-point, and the error often does not raise an exception — it silently produces a plausible-looking wrong answer. The difficulty is that the failure modes are invisible without understanding both the algorithm's stability (does it amplify rounding error?) and the problem's conditioning (does the answer depend sensitively on the input?).

Agents tend to write numerical code by transcribing textbook formulas directly, treating floats as if they were reals, and assuming that double precision is a cure-all. The judgment problem is deciding, for each computation, whether the straightforward formula is numerically stable, whether the problem is well-conditioned enough that stability matters, what precision is actually required, and how to verify the result against the rounding error rather than against an idealized real-number answer. Getting this wrong produces simulations that diverge, statistics that are wrong in the last digits, solvers that fail to converge, and results that differ between the developer's machine and production — all without any error message.

## Core Rules

### Distinguish Conditioning (The Problem) From Stability (The Algorithm)

These are different concepts that are constantly conflated, and the distinction governs everything.

- **Conditioning** is a property of the problem: how sensitively the exact answer depends on the input. A problem is ill-conditioned if a tiny change in the input produces a large change in the answer. The condition number of matrix inversion is roughly the product of the matrix's norm and its inverse's norm; if it is 10^10, you lose about 10 digits of precision in the result regardless of the algorithm. Conditioning cannot be improved by a better algorithm — it is intrinsic to the problem.
- **Stability** is a property of the algorithm: does the algorithm's computed result (with rounding) correspond to the exact result of a nearby problem? A stable algorithm on a well-conditioned problem gives an accurate answer. A stable algorithm on an ill-conditioned problem gives an answer accurate to (input precision minus log of condition number) digits — which may be zero. An unstable algorithm can lose accuracy even on a well-conditioned problem.

The implication: if a result is inaccurate, first ask whether the problem is ill-conditioned (in which case no algorithm helps — reformulate the problem, regularize, or accept the precision limit) or whether the algorithm is unstable (in which case a different algorithm helps). Computing the condition number — or estimating it — is part of diagnosing numerical error.

### Prefer Stable Algorithms Over Naive Textbook Formulas

Many textbook formulas are numerically unstable even though they are mathematically correct. The classic examples: the naive formula for the roots of a quadratic (`(-b ± sqrt(b²-4ac)) / 2a`) loses precision when `b² >> 4ac` because `b` and `sqrt(b²-4ac)` are nearly equal and their subtraction cancels; the stable form rationalizes to compute the root that would cancel via a different expression. Gaussian elimination without pivoting is unstable for matrices where a pivot is small; partial pivoting makes it stable in practice. The naive recurrence for sample variance can produce a negative variance from rounding; Welford's algorithm is stable.

For each numerical computation, ask whether the standard formula is the stable one or the unstable one, and prefer the stable variant even when it is less obvious. Linear algebra libraries (LAPACK, BLAS) use the stable variants (pivoted LU, QR, SVD); use them rather than hand-rolling. The cost of an unstable algorithm is silent wrong answers, not an exception.

### Watch For Catastrophic Cancellation In Subtraction

When two nearly-equal floating-point numbers are subtracted, the leading digits cancel and the result has far fewer significant digits than the inputs. This is catastrophic cancellation, and it is the single most common source of precision loss. The quadratic formula is one example; computing the variance as `E[X²] - E[X]²` is another (when the mean is large relative to the variance, both terms are large and nearly equal, and their difference loses most of the digits); computing a small angle as the difference of two nearly-equal large angles is another.

Detect cancellation by looking for subtractions of quantities that can be nearly equal, and restructure to avoid them: use the rationalized quadratic form, use Welford's recurrence for variance, compute small differences via `atan2` or `sin`/`cos` identities rather than subtracting angles. A subtraction that loses 8 digits of precision in double gives you only 8 digits of meaningful result; in single (float32), it can leave you with nothing.

### Sum In A Stable Order, Or Use A Compensated Sum

Naive summation `((a0 + a1) + a2) + ...` accumulates error proportional to the number of terms, and the error is worst when summing many small numbers into a large running total (each small addend is rounded to the running total's precision and can be lost entirely). Summing a million numbers this way can lose several digits. The error depends on summation order, which is why the same computation gives different results in different orders (and why parallel reduction, which changes the order, changes the result).

Mitigations, in increasing order of cost: sum from smallest to largest magnitude (so small contributions accumulate before the total is large), use pairwise (binary tree) summation (which most BLAS libraries do by default), or use the Kahan or Neumaier compensated sum (which tracks the lost rounding error and adds it back, giving near-double precision at the cost of a few extra operations per term). For reductions over large arrays, use the library's pairwise sum rather than a naive loop; the difference can be orders of magnitude in error.

### Choose Precision Deliberately, Not By Default

Single precision (float32) has about 7 decimal digits and roughly half the throughput and memory bandwidth cost of double (float16, on hardware that supports it, has about 3 digits and is faster still). For many computations — graphics, signal processing, machine learning inference — single is sufficient and the performance win is large. For others — long-time simulations, accumulation over many terms, ill-conditioned linear solves, financial calculations — single loses so much precision that the result is wrong, and double is required.

The decision should be deliberate: identify the precision the result needs (how many digits are meaningful), estimate the precision the algorithm loses (condition number plus accumulation), and choose the format with margin. Do not default to double "to be safe" when single would do (you give up 2x performance for no benefit), and do not default to single "for speed" when the algorithm loses more digits than single provides. Mixed precision (single for the bulk, double for the sensitive accumulation) is often the right answer.

### Never Compare Floats For Equality Without A Tolerance

Because floating-point arithmetic is rounded, two computations that are mathematically equal often produce slightly different bit patterns. `0.1 + 0.2 != 0.3` in IEEE 754. Direct equality comparison (`==`) of floats is almost always a bug; it works in tests on one platform and fails on another, or works for one input and fails for a nearby input.

Compare with a tolerance: `abs(a - b) < eps` for an absolute tolerance, or `abs(a - b) < eps * max(abs(a), abs(b))` for a relative tolerance. Choose the tolerance against the precision of the computation and the scale of the values — an absolute tolerance of `1e-9` is meaningless for values around `1e15`. For tests, compare against a reference with a documented tolerance, not with `==`. The same applies to convergence checks in iterative solvers: the residual must fall below a tolerance scaled to the problem, not exactly zero.

### Verify Against Rounding Error, Not Against An Idealized Answer

A numerical result is "correct" if it is accurate to the precision the algorithm and the problem allow, not if it matches the real-number answer exactly. A linear solve with condition number 10^8 in double precision is correct to about 8 digits; expecting 15-digit agreement with a reference is wrong. A simulation run for a long time will diverge from a reference because of accumulated rounding, and that is expected, not a bug.

Verify results by: comparing against a higher-precision reference (compute the same thing in quad or arbitrary precision and check the double result agrees to the expected digits), checking invariants the answer should satisfy (residual norms, conservation laws), and running convergence studies (does the answer improve as precision or step size increases, at the expected rate?). A result that matches a reference to all digits is suspicious; a result that matches to the precision the problem allows and diverges predictably beyond that is trustworthy.

## Common Traps

### Transcribing The Textbook Formula Directly

Implementing the quadratic formula, sample variance, or Gaussian elimination straight from the textbook, getting an unstable algorithm that loses precision on the inputs that matter. Use the stable variant; the textbook form is often the unstable one.

### Subtracting Nearly-Equal Quantities And Losing Digits

Computing a small result as the difference of two large nearly-equal quantities (variance as `E[X²] - E[X]²`, a small angle as a difference of large angles), losing most of the significant digits to cancellation. Restructure to compute the small quantity directly.

### Summing In Arbitrary Order And Getting Different Results

Using a naive `for` loop to sum a large array, then getting a different answer from a parallel reduction or a different-platform library because summation order changed. Use pairwise or compensated summation for reductions over many terms.

### Defaulting To Double "To Be Safe" Or Single "For Speed"

Choosing precision by habit rather than analysis — double everywhere (wasting 2x performance where single would do) or single everywhere (losing precision where double is needed). Identify the precision the result needs and the precision the algorithm loses; choose with margin.

### Comparing Floats With `==`

Writing `if (x == 0.3)` or asserting `result == reference` in a test, then failing on a different platform or a nearby input because of rounding. Compare with a tolerance scaled to the precision and magnitude.

### Ignoring The Condition Number

Blaming the algorithm for inaccuracy when the problem is ill-conditioned, or expecting more digits than the condition number allows. Estimate the condition number; if it is large, no algorithm gives an accurate answer and the problem needs reformulation or regularization.

### Assuming Determinism Across Runs Or Platforms

Expecting the same floating-point result on different hardware, compilers, or parallel configurations, then chasing a "bug" that is actually summation-order or FMA-contraction difference. Floating-point results depend on evaluation order and hardware; specify the order or accept the variation.

### Treating A Small Residual As Proof Of Correctness

Checking that a solver's residual is small and assuming the answer is accurate, when an ill-conditioned problem can have a small residual and a large error. Check both the residual and the condition number; a small residual in a well-conditioned problem is meaningful, in an ill-conditioned one it is not.

## Self-Check

- [ ] For each numerical computation, the conditioning of the problem (condition number or estimate) and the stability of the algorithm were considered separately; instability was addressed by a stable algorithm, ill-conditioning by reformulation or regularization.
- [ ] Stable algorithm variants are used (pivoted LU/QR from LAPACK, rationalized quadratic, Welford variance, etc.) rather than naive textbook formulas that lose precision.
- [ ] Subtractions of nearly-equal quantities were identified and restructured to avoid catastrophic cancellation; no computation loses most of its digits to a cancellation that could be avoided.
- [ ] Reductions over many terms use pairwise or compensated summation (or the BLAS/library default), not a naive loop whose error depends on order.
- [ ] Precision (single, double, mixed) was chosen deliberately against the digits the result needs and the digits the algorithm loses, with margin — not by habit.
- [ ] No float is compared with `==` for correctness; comparisons use absolute or relative tolerances scaled to the precision and magnitude, including in tests and convergence checks.
- [ ] Results were verified against a higher-precision reference or by checking invariants (residuals, conservation laws), with agreement expected to the precision the problem allows — not to all digits.
- [ ] Cross-platform or cross-run variation in floating-point results was anticipated (summation order, FMA, hardware) and either controlled by specifying order or accepted as expected, not chased as a bug.
