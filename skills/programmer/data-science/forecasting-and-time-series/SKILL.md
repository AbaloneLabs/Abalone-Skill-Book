---
name: forecasting_and_time_series.md
description: Use when the agent is building or evaluating a forecast (demand, traffic, revenue, capacity, inventory), choosing a time series model (ARIMA, exponential smoothing, state space, Prophet, hierarchical reconciliation, ML/sequence models), handling trend, seasonality, and regime changes, deciding forecast horizon and granularity, evaluating forecast accuracy, or reconciling forecasts across hierarchy levels. Also covers the failure modes of over-fitting to recent history, ignoring structural breaks and regime shifts, naive backtesting that leaks information, single-point forecasts with no uncertainty, reconciling forecasts that do not add up across hierarchy levels, and the recurring mistake of trusting a forecast's point estimate without examining its uncertainty, assumptions, and the regime it was trained on.
---

# Forecasting And Time Series

A forecast is a prediction about the future built on the assumption that the patterns of the past will continue. That assumption is the entire difficulty. Real time series contain trend (a gradual rise or fall), seasonality (weekly, yearly cycles), holidays and events (one-off spikes), noise, and — most dangerously — regime changes (the underlying data-generating process shifts, because of a product change, a market shift, a pandemic). A model that fits the recent past beautifully can fail catastrophically when the future's regime differs, and because the future is unobservable at training time, the only honest evaluation is careful backtesting that mimics the real forecasting situation. The judgment problem is that a forecast produces a number that looks precise, and most of the work is in modeling the right components, evaluating honestly against a holdout that respects time, quantifying uncertainty rather than emitting a single point, and recognizing when the regime has shifted and the past is no longer a reliable guide.

Agents tend to under-invest here because a default model (throw the series at a library, get a forecast) fits the training data and produces plausible-looking output. The harm appears when the forecast is consumed. A model over-fit to recent history misses a seasonal peak and the business under-staffs. A point forecast with no uncertainty interval leads to a plan with no margin, and the actual outcome falls outside it. A naive backtest that trains on data including the "future" leaks information and overstates accuracy, so the forecast is trusted and fails. Forecasts at different hierarchy levels (region vs country) do not reconcile, so different teams plan to inconsistent numbers. The judgment problem is to model trend and seasonality deliberately, evaluate with time-respecting backtests, communicate uncertainty, handle hierarchy and regime change, and never present a forecast as a single certain number.

This skill covers decomposition (trend/seasonality/residual), model selection, honest backtesting, uncertainty quantification, hierarchical reconciliation, and regime detection. It complements the statistical-pitfalls skill (general interpretation), the ab-test skill (experimentation, not forecasting), and the causal-inference skill (causation vs prediction). Here the focus is predicting future values of a time series and the specific ways forecasts mislead.

## Core Rules

### Decompose The Series Into Trend, Seasonality, And Residual Before Modeling

A time series is usually a composite of structural components. Modeling them explicitly (decomposition) is more robust and interpretable than throwing raw data at a black box:

- **Identify and model trend.** Is the series rising, falling, or flat, and is the trend linear, damped, or changing? A trend that is not modeled leaves drift in the residual; a trend modeled as linear when it is saturating produces forecasts that run away.
- **Identify and model seasonality.** Weekly cycles (traffic lower on weekends), yearly cycles (retail peaks in Q4), and daily cycles each recur predictably. Multiple seasonal periods (daily + weekly + yearly) may coexist; a model that captures only one leaves structured pattern in the residual.
- **Handle holidays and known events.** One-off events (a holiday, a launch, a marketing campaign) produce spikes that are not recurring seasonality. Model them as exogenous regressors or holiday effects, or they distort the seasonal estimate.
- **Inspect the residual after decomposition.** A well-decomposed series leaves residual that is noise (unpredictable). Residual that still contains pattern (autocorrelation, remaining seasonality) means the decomposition missed something.

### Choose The Model By The Series' Structure, Not By Habit

Different model families suit different series structures. The choice should follow the data, not familiarity with one library:

- **Classical statistical models (ARIMA, exponential smoothing, state space).** Strong for univariate series with clear trend and seasonality, well-understood confidence intervals, and modest data needs. Exponential smoothing (ETS) and ARIMA are robust defaults for many business series.
- **Component-based models (Prophet and similar).** Designed for business series with trend, multiple seasonalities, holidays, and regime changes in trend; interpretable and tolerant of missing data and outliers. A practical default for operational forecasting with human-readable components.
- **Machine learning and sequence models (gradient boosting, neural sequence models, transformers).** Powerful when many exogenous features drive the series and interactions are complex, but they need more data, are prone to overfitting, and offer less interpretable uncertainty. Use when univariate models underfit and exogenous signal is strong.
- **Match model complexity to data volume and stakes.** A simple, robust model on a sparse, high-stakes series (capacity planning) is often better than a complex model that overfits. Reserve complex models for series with enough data to support them and where the extra accuracy is worth the opacity.

### Backtest Honestly, Respecting Time And Avoiding Information Leakage

The only honest evaluation of a forecast is a backtest that mimics the real forecasting situation: train only on data available at each point, forecast the next horizon, and compare. The dominant error is leaking future information into training:

- **Use a rolling or expanding window, never a random split.** Time series must be split by time (train on the past, test on the future), never shuffled. A random split trains on future data and leaks information, overstating accuracy.
- **Re-fit at each backtest fold to mimic production.** In production, the model is retrained as new data arrives; the backtest should re-fit at each step (or on the schedule production uses), not train once and evaluate forward, to reflect real performance.
- **Respect the forecast horizon in evaluation.** Evaluate at the horizons you will actually use (1-step, 7-step, 30-step ahead). A model good at 1-step-ahead may be poor at 30-step; accuracy varies by horizon, and the horizon that matters is the decision's.
- **Compare against a naive baseline.** Always compare to a trivial baseline (the last value, the seasonal naive: "same as last week"). A model that cannot beat the seasonal naive is adding complexity for no gain. Many forecasts lose to a naive baseline when honestly evaluated.

### Quantify And Communicate Uncertainty, Not Just A Point Forecast

A single point forecast invites overconfident planning. Real forecasts must convey uncertainty:

- **Produce prediction intervals, not only point estimates.** An 80% or 95% interval shows the plausible range. Planning to the point estimate with no margin guarantees shortfalls when the outcome is in the tail.
- **Make intervals honest, not too narrow.** Intervals that are too narrow (overconfident) fail to contain the actual outcome at the stated rate. Calibrate intervals on the backtest: an 80% interval should contain roughly 80% of actuals; if it contains only 50%, the intervals are overconfident.
- **Distinguish interval width from horizon.** Uncertainty grows with horizon; a 30-day-ahead interval is wider than a 1-day interval. Communicate intervals per horizon, not a single width.
- **Communicate the forecast as a distribution for downstream decisions.** Capacity, inventory, and staffing decisions should consume the distribution (or scenarios), not the point, so the plan includes margin for the tail.

### Reconcile Hierarchical Forecasts So They Add Up

In organizations, the same quantity is forecast at multiple levels (product, region, country; team, department, company). Independent forecasts at each level do not add up, creating inconsistent plans. Hierarchical reconciliation fixes this:

- **Recognize the hierarchy and its constraints.** Regional forecasts should sum to the country forecast; product forecasts to the category. Independent bottom-up and top-down forecasts violate this.
- **Reconcile rather than forecast once.** Forecast at all levels independently, then reconcile (e.g., MinT) so the forecasts are coherent across the hierarchy while preserving the signal at each level. Reconciliation uses the strengths of each level (top-down captures aggregate trend; bottom-up captures local detail).
- **Decide coherence by the business structure.** Some hierarchies are strictly additive (units); others are not (averages, rates). Reconcile in a way that respects the actual aggregation relationship.

### Detect Regime Change And Know When The Past Stops Applying

The most dangerous forecasting failure is a regime shift — the data-generating process changes, and the model trained on the old regime produces confidently wrong forecasts. Detecting and responding to this is essential:

- **Monitor forecast error over time.** A sudden, persistent increase in error signals that the series has shifted and the model is stale. Alert on error, not only on producing a forecast.
- **Distinguish noise from structural break.** A few large errors may be noise (an event); a sustained shift in the error mean or in the series' level is a structural break. Structural breaks require retraining and possibly re-specifying the model.
- **Incorporate exogenous drivers of regime change.** Product launches, policy changes, market shifts, and macro events change the series in ways history alone cannot predict. Where such drivers are known, include them as exogenous variables; where they have occurred, do not trust a model trained before them.
- **Be honest when the future is not like the past.** Some regimes (a pandemic, a market disruption) break the forecasting assumption entirely. In such cases, the honest output is wide uncertainty or a scenario range, not a precise forecast from a model whose assumptions no longer hold.

## Common Traps

### Over-Fitting To Recent History

A model that fits the recent past beautifully but captures noise as pattern, failing when the future differs. Prefer models whose complexity matches the data and the signal, and validate on an honest holdout.

### Naive Backtesting That Leaks Information

A random split or a single train-once-evaluate-forward that trains on future data, overstating accuracy. Use a rolling/expanding window, re-fit per fold, evaluate at the real horizon, and compare to a naive baseline.

### A Point Forecast With No Uncertainty

Emitting a single number that invites zero-margin planning, so the actual outcome falls outside the plan. Produce calibrated prediction intervals per horizon and let downstream decisions consume the distribution.

### Overconfident Intervals

Intervals that are too narrow because they ignore parameter or model uncertainty, failing to contain actuals at the stated rate. Calibrate intervals on the backtest and widen them for longer horizons.

### Forecasts That Do Not Reconcile Across Hierarchy

Independent forecasts at region, product, and total levels that do not add up, producing inconsistent plans. Reconcile hierarchically so forecasts are coherent while preserving each level's signal.

### Ignoring A Structural Break Or Regime Change

A model trained on an old regime producing confidently wrong forecasts after the series shifts. Monitor forecast error, distinguish noise from structural breaks, incorporate exogenous drivers, and retrain or re-specify when the regime changes.

### Trusting A Forecast Whose Assumptions No Longer Hold

Continuing to emit precise forecasts during a disruption (a pandemic, a market shift) where the past is not a guide to the future. Recognize when the forecasting assumption is broken and communicate wide uncertainty or scenarios instead.

## Self-Check

- [ ] The series was decomposed into trend, seasonality, holidays/events, and residual before modeling, multiple seasonal periods were captured where present, and the residual was inspected for leftover pattern.
- [ ] The model family was chosen by the series' structure (classical for univariate trend/seasonality, component-based for business series with regime changes, ML/sequence when exogenous signal is strong), with complexity matched to data volume and stakes.
- [ ] Backtesting is honest: time-respecting rolling/expanding split (no shuffling), re-fit per fold to mimic production, evaluation at the real forecast horizon, and comparison against a naive baseline the model actually beats.
- [ ] Uncertainty is quantified and communicated: prediction intervals (not only point estimates), calibrated on the backtest (an 80% interval contains ~80% of actuals), widening with horizon, and consumed as a distribution by downstream decisions.
- [ ] Hierarchical forecasts are reconciled so they add up across levels (region to country, product to category), using a method that preserves each level's signal and respects the actual aggregation relationship.
- [ ] Regime change is monitored (forecast error tracked and alerted), structural breaks are distinguished from noise, exogenous drivers of regime change are incorporated, and the forecast is honest (wide uncertainty or scenarios) when the past stops applying.
- [ ] The highest-risk cases were verified — a model that lost to a naive baseline under honest backtest, overconfident intervals that missed actuals, unreconciled hierarchy, and a regime shift that broke the forecast — not only the clean in-sample fit.
