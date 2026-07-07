---
name: sensor_data_reliability_and_calibration.md
description: Use when the agent is designing sensor data acquisition, processing, or analytics for IoT or embedded systems; dealing with calibration (factory calibration, field calibration, re-calibration), sensor drift over time or temperature, sensor fusion, outlier and anomaly detection, noise filtering and signal conditioning, timestamp and sample alignment across devices, units and semantic metadata for measurements, or quantifying confidence and uncertainty in readings; building pipelines that consume temperature, humidity, pressure, vibration, gas, GPS, accelerometer, or other sensor data; or diagnosing analytics that produce wrong conclusions because the underlying sensor values drifted, were miscalibrated, were affected by environmental conditions, or were trusted as ground truth. Covers why raw sensor values are not measurements, calibration lifecycle, drift modeling, fusion, and confidence-aware data design.
---

# Sensor Data Reliability And Calibration

A sensor reading is not a measurement. It is a raw value shaped by calibration, temperature, aging, wiring, mounting, and the local environment — and it silently degrades from the moment it leaves the factory. Agents trained on web data, where a number in a JSON payload is the number, routinely treat sensor output as ground truth: they store the raw value, compute analytics on it, trigger alerts from it, and report it to users as fact. Then the analytics drift, the alerts misfire, and nobody can tell whether the anomaly is a real event or a sensor that drifted 8% over a year of heat and humidity. The judgment problem is that turning raw readings into trustworthy measurements is a lifecycle and modeling problem — calibration, drift compensation, fusion, validation, and uncertainty — and it must be designed into the pipeline, not assumed away.

The harm of trusting raw values is insidious: the system looks correct in the lab, where sensors are new and the environment controlled, and then silently produces wrong answers in the field, where every sensor drifts and the environment fights you.

## Core Rules

### Treat Every Reading As Raw, And Manage Calibration And Drift As A Lifecycle

A sensor's output is meaningful only relative to a calibration that maps raw counts to engineering units, and that mapping is not permanent. Calibration has a lifecycle: factory calibration (valid for a stated accuracy under stated conditions), field calibration (at install, against a reference), and periodic re-calibration (because every sensor drifts). Drift is the rule, not the exception: electrochemical gas sensors drift and deplete, humidity sensors drift with contamination, pressure sensors shift with temperature. Temperature is the most common drift driver — a sensor calibrated at 20 °C reads differently at 50 °C — and vendors often provide a compensation curve. Build compensation in where the model is known, and where it is not, capture the conditions alongside the reading (temperature at the sensor, supply voltage, time since calibration) so drift can be detected downstream. The recurring failure is treating calibration as a one-time factory step and recording only the primary measurement, discarding the context that would explain why a reading shifted — so a real event becomes indistinguishable from a sensor that got hot or old. A sensor with a calibration date, an expiry, a re-calibration procedure, and its conditions recorded is a measured value; one with none of that is a guess with a number on it.

### Use Sensor Fusion And Redundancy Where One Sensor Cannot Be Trusted

Many quantities are measured better by combining multiple imperfect sensors than by trusting one expensive one. A single GPS fix is noisy; fusing GPS with inertial and wheel-odometry is robust. A single temperature sensor cannot tell you it is drifting, but two co-located sensors that diverge tell you one is wrong. Fusion is not just averaging — it is weighting sensors by their expected error, their current health, and their failure mode (a sensor that fails stuck-high vs one that fails random), and rejecting readings that disagree beyond a plausible bound. Decide where a single sensor is acceptable (low-cost, low-stakes, drift-tolerant) and where fusion or redundancy is required (safety, billing, anything that triggers an irreversible action). The trap is treating fusion as an optimization when it is often the only way to bound the error of a measurement you cannot otherwise trust — and averaging redundant sensors without checking for divergence lets a failed sensor that reads stuck-high silently corrupt the average.

### Detect Outliers And Noise Deliberately, Never With A Magic Threshold

Real sensor streams are noisy, and "noise" and "anomaly" overlap: a spike might be a glitch (drop it), a real event (keep and alarm on it), or the onset of sensor failure (flag and investigate). A naive pipeline that filters everything outside a fixed range will silently swallow real events; one that passes everything will drown analytics in noise. The disciplined approach: know your sensor's noise floor and failure modes, filter or smooth to remove noise above the floor (moving average, Kalman filter, median filter — median is robust to the spikes a mean would distort), and separately detect anomalies against a model of normal behavior that can evolve, not a static threshold set once in the lab. A static threshold tuned in the lab is wrong in the field, where normal shifts with season, usage, and environment. Make filtering and anomaly detection adaptive and reviewable, and never let a filter silently hide the rare event the system exists to detect.

### Carry Timestamps, Units, Metadata, And Confidence With Every Measurement

A number without a timestamp, a unit, a meaning, and a confidence is not data — it is a future bug. Timestamps must be captured at sample time (not arrival time, which is delayed by sleep cycles, buffering, and network latency) and ideally in a shared time base across devices, or multi-device correlation is impossible. Units must be explicit (a "25" that is Celsius in one payload and Fahrenheit in another is a silent error). Semantic metadata — what sensor, what quantity, what calibration — must travel with the value. And every measurement has uncertainty — from accuracy, calibration error, drift, and the compensation model — that downstream decisions should see: "25 °C ± 2 °C" supports a different decision than "25 °C ± 0.1 °C," and a threshold crossed with low confidence should be handled differently than one crossed with high confidence. Propagate uncertainty through the pipeline (fused uncertainty is a function of the inputs; an aggregate's confidence reflects them). The failure mode is a pipeline of bare numbers that discards uncertainty at the sensor and reports a single value everywhere, so a decision-maker cannot distinguish a reliable reading from a near-guess. Make every measurement self-describing and confidence-aware.

## Common Traps

### Trusting Raw Sensor Values As Ground Truth

Storing and acting on raw counts or uncalibrated values as if they were true measurements, then discovering the analytics were wrong because the sensors were never calibrated or had drifted. Calibrate, track calibration state, and treat raw values as raw until proven.

### One-Time Factory Calibration, Never Revisited

Assuming factory calibration holds for the device's lifetime, so a fleet drifts unmonitored and data degrades silently over months and years. Model drift, capture conditions, and schedule re-calibration.

### Discarding The Conditions That Explain Drift

Recording only the primary measurement and throwing away temperature, voltage, and time-since-calibration, so when a reading shifts you cannot tell a real event from a sensor that got hot or old. Record the context with the measurement.

### Static Outlier Thresholds Tuned In The Lab

A fixed range that filters out real events or swallows noise because field behavior differs from the lab. Make filtering and anomaly detection adaptive to the evolving normal, and separate noise filtering from anomaly detection.

### Filtering Away The Event You Exist To Detect

A filter aggressive enough to remove noise that it also removes the rare anomaly the system was built to catch, or averaging redundant sensors without detecting divergence so a failed sensor corrupts the result. Review what the filter discards and detect failing sensors explicitly.

## Self-Check

- [ ] Every sensor reading is treated as raw until calibrated: the pipeline records calibration state (factory vs field, date, expiry) per sensor, and re-calibration is scheduled or triggered, not assumed permanent.
- [ ] Drift is modeled and compensated: temperature and aging effects are compensated where the model is known, and the conditions at sample time (temperature, voltage, time since calibration) are recorded with the measurement so drift can be detected downstream.
- [ ] Where a single sensor cannot meet the required trust, sensor fusion or redundancy is used, with weighting by expected error and health and explicit divergence detection that rejects or down-weights a failing sensor.
- [ ] Noise filtering and anomaly detection are separate and adaptive: filtering targets the known noise floor (with robust methods like median where spikes matter), and anomaly detection uses an evolving model of normal, not a static lab-tuned threshold.
- [ ] Every measurement is self-describing: a sample-time timestamp (not arrival time) in a shared time base, explicit units, sensor identity, and calibration reference travel with the value.
- [ ] Confidence or uncertainty is quantified at the sensor and propagated through the pipeline, so downstream decisions and alerts can distinguish a high-confidence reading from a near-guess.
- [ ] Multi-device correlation was considered: timestamps are aligned to a shared time base (or clock skew handled) so readings from different devices can be meaningfully compared and fused.
- [ ] A drift/failure simulation was run: a sensor was deliberately miscalibrated, drifted, or made to fail, and the pipeline correctly flagged it rather than silently propagating wrong values into analytics and alerts.
