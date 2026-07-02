---
name: simulation_design_and_validation.md
description: Use when the agent is building a physics, fluid, agent-based, or numerical simulation, choosing a time integration scheme, setting the timestep, validating simulation results against analytical or experimental data, performing convergence or sensitivity analysis, propagating uncertainty, or diagnosing a simulation that diverges, blows up, produces non-physical results, or fails to conserve invariants.
---

# Simulation Design And Validation

A simulation is a model of a physical or abstract system evolved forward in time by a numerical scheme, and its value depends entirely on whether its output reflects reality rather than the artifacts of its own machinery. The trap is that a simulation always produces output — it always runs, always prints numbers, always renders a picture — and the output can be plausible, detailed, and confidently wrong. A fluid solver that does not conserve mass produces elegant vortices that slowly drain the domain. An agent-based model whose timestep is too large misses the collisions it is supposed to capture. A structural simulation that has not converged produces stresses that look reasonable and are off by an order of magnitude. The difficulty is that the failure modes are not exceptions; they are silent drift in quantities that look fine until you check the right invariant, and by then the result has been trusted.

Agents tend to build a simulation by transcribing the governing equations, picking a timestep by guess, running it until it produces a picture, and treating convergence and validation as optional polish. The judgment problem is deciding whether the numerical scheme is stable and accurate for the problem, whether the timestep and resolution are fine enough to resolve the phenomena that matter, whether the result is converged (insensitive to further refinement) or merely the output of one arbitrary discretization, and whether the simulation has been validated against anything independent of itself. Getting this wrong produces simulations that are internally consistent and externally meaningless — results that look scientific and have no demonstrated connection to the system they claim to model.

## Core Rules

### Choose The Integration Scheme Against Stability, Accuracy, And Conservation

The time integration scheme — how the simulation advances from one state to the next — governs whether the simulation is stable (does not blow up), accurate (matches the true solution to the expected order), and conservative (preserves the invariants the physics requires). These are different properties, and optimizing one can sacrifice another.

- **Explicit schemes** (forward Euler, explicit Runge-Kutta) compute the next state directly from the current state. They are simple and cheap per step, but conditionally stable: the timestep must be below a problem-dependent limit (the CFL condition for advection, the diffusion limit for heat equation) or the solution oscillates and diverges. Explicit schemes are the default when the stability limit is tolerable.
- **Implicit schemes** (backward Euler, implicit Runge-Kutta) solve an equation for the next state that depends on itself, requiring a linear or nonlinear solve per step. They are unconditionally stable (large timesteps do not diverge) but more expensive per step and can be less accurate at large timesteps (numerical damping). Use implicit schemes for stiff problems (fast and slow dynamics together, like chemical reactions or structural mechanics) where the explicit stability limit would force impractically small timesteps.
- **Symplectic schemes** (leapfrog, Verlet) preserve the geometric structure of Hamiltonian systems and conserve energy over long times, where generic schemes slowly drain or inject energy. Use them for long-running orbital, molecular, or structural dynamics where energy conservation matters.

Match the scheme to the problem's character. An explicit scheme on a stiff problem diverges; an implicit scheme on a non-stiff problem wastes the solve cost; a non-symplectic scheme on a long orbital simulation slowly spirals the orbit inward.

### Set The Timestep And Resolution Against The Phenomena That Must Be Resolved

The timestep and spatial resolution determine what the simulation can capture. A timestep too large for the fastest relevant dynamics aliases them away or destabilizes the scheme; a resolution too coarse for the smallest relevant feature smears it out. The simulation can only be trusted for phenomena that are resolved, and everything faster or smaller than the grid is either lost or produces numerical artifacts.

Choose the timestep against the fastest dynamics that must be captured, not the fastest dynamics in the system if those are irrelevant (a stiff system where the fast modes are physical but uninteresting is the case for implicit methods). Choose the spatial resolution against the smallest feature that must be resolved (the boundary layer in a fluid, the wavelength of the highest-frequency wave). Then verify both by a convergence study (below): if halving the timestep or doubling the resolution changes the result meaningfully, the original was not resolved.

### Verify Convergence Before Trusting Any Result

A simulation result is trustworthy only if it is converged — insensitive to further refinement of the discretization. A result that changes substantially when the timestep halves or the grid refines is not a result; it is an artifact of the current resolution, and the "true" answer is somewhere else. Convergence is the difference between a simulation that models the physics and one that models its own grid.

Perform a convergence study: run the simulation at a sequence of timesteps (or resolutions) and confirm the quantity of interest approaches a limit as the discretization refines, at the rate the scheme's order predicts (a second-order scheme should roughly quarter the error when the step halves). If the result does not converge, or converges at the wrong rate, the scheme is wrong, the resolution is too coarse, or there is a bug. A single-resolution run with no convergence check is a guess dressed as a computation.

### Validate Against Independent Data, Not Against Itself

Verification (is the simulation solving the equations correctly?) is necessary but not sufficient. Validation (do the equations and their solution match reality?) requires comparison against data that is independent of the simulation — analytical solutions, experimental measurements, or a trusted reference simulation. A simulation verified to converge to the wrong answer is precisely converged nonsense.

Sources of validation data:

- **Analytical solutions** for simplified cases (a known flow, a test problem with a closed-form answer). If the simulation cannot reproduce the analytical solution on a case where one exists, it is wrong, and the discrepancy localizes the error.
- **Experimental or observational data** for realistic cases. This is the ultimate validation: does the simulation match what the real system does? Agreement within the experimental uncertainty is the standard; perfect agreement is suspicious (it may indicate the validation data was used to tune the model).
- **Code comparison** against a trusted, independent implementation of the same physics (the method of manufactured solutions, or comparison to a established code). Agreement between two independent codes catches bugs that are invisible within one.

Document what was validated against what, and the level of agreement. A simulation presented without validation is a hypothesis, not a result.

### Check Conservation Laws And Invariants At Every Step

Physical systems conserve quantities — mass, momentum, energy, charge — and a simulation that violates these conservation laws is producing non-physical results, even if they look plausible. Conservation is a cheap, powerful check: compute the conserved quantity at each step and confirm it does not drift (beyond the expected truncation error). Drift indicates a bug, a scheme that is not conservative, or a boundary condition that leaks.

Some schemes conserve exactly (finite-volume methods for mass, symplectic methods for energy), and some do not (generic finite-difference schemes may slowly violate conservation). If conservation matters for the application, choose a conservative scheme or correct the drift explicitly. A fluid simulation that loses 10% of its mass over the run is producing results in a domain that is quietly emptying, and no amount of visual plausibility makes that correct.

### Propagate Uncertainty And Perform Sensitivity Analysis

A simulation takes inputs (parameters, initial conditions, boundary conditions) that are uncertain, and propagates them to outputs that are therefore uncertain. A single simulation run with "best guess" inputs produces a point estimate with no indication of how much it would change if the inputs were different. For any result that informs a decision, the uncertainty must be quantified.

- **Sensitivity analysis** varies each input across its plausible range and measures the output's response, identifying which inputs the result depends on most. This tells you where to invest in better input data and where the result is robust.
- **Uncertainty propagation** (Monte Carlo sampling of the inputs, or polynomial chaos for expensive simulations) produces a distribution of outputs, not a point estimate. A result reported as "X plus or minus Y given the input uncertainties" is honest; a result reported as "X" conceals the uncertainty and invites overconfidence.

### Use Adaptive Timestepping For Problems With Varying Dynamics

Many simulations have dynamics that vary in time — quiescent periods interspersed with fast events, or regions that stiffen as the system evolves. A fixed timestep sized for the worst case wastes computation during the quiescent periods; a fixed timestep sized for the average case diverges during the fast events. Adaptive timestepping adjusts the step based on an error estimate, taking small steps when the dynamics are fast and large steps when they are slow.

Adaptive schemes (embedded Runge-Kutta, adaptive BDF) estimate the local truncation error per step and shrink or grow the timestep to keep it below a tolerance. This concentrates computation where it is needed and is essential for problems with stiffening or event-driven dynamics. Set the tolerance against the accuracy the result needs; too loose and the adaptive scheme takes large inaccurate steps, too tight and it wastes computation. Verify that the adaptive result is converged against the tolerance.

## Common Traps

### Running One Resolution And Calling It A Result

Producing a single simulation run with no convergence study, so the result is an artifact of the chosen discretization rather than the physics. Perform a convergence study; a result that changes with refinement is not converged.

### Validating The Simulation Against Itself

Checking that the simulation is self-consistent and calling it validated, when validation requires independent data (analytical, experimental, or a second code). Verification is not validation; a precisely converged wrong answer is still wrong.

### Ignoring Conservation Law Drift

Letting mass, energy, or momentum drift over the run because the scheme is not conservative or a boundary leaks, producing plausible-looking non-physical results. Check conserved quantities at every step.

### Timestep Too Large For The Dynamics

Choosing a timestep that aliases or destabilizes the fastest relevant dynamics, so the simulation misses the phenomena it is meant to capture or diverges. Size the timestep against the dynamics that must be resolved; verify by convergence.

### Reporting A Point Estimate With No Uncertainty

Running the simulation once with best-guess inputs and reporting the output as the answer, concealing how much it depends on uncertain inputs. Propagate uncertainty; report results as distributions or ranges.

### Using An Explicit Scheme On A Stiff Problem

Applying an explicit integrator to a stiff system, forcing impractically small timesteps to stay stable or diverging when the stability limit is exceeded. Use implicit or stiff-aware schemes for stiff problems.

### Treating Visual Plausibility As Validation

Judging the simulation correct because the output looks right, when visual plausibility is a weak check that passes for many wrong answers. Validate against quantitative independent data.

### Skipping Sensitivity Analysis

Presenting a result without knowing which inputs it depends on most, so the audience cannot judge robustness or where better data would change the conclusion. Perform sensitivity analysis; identify the dominant inputs.

## Self-Check

- [ ] The integration scheme (explicit, implicit, symplectic) was chosen against the problem's stability, accuracy, and conservation requirements — explicit for non-stiff resolved dynamics, implicit for stiff problems, symplectic for long-running conservative systems.
- [ ] The timestep and spatial resolution are sized to resolve the fastest and smallest phenomena that must be captured, and this was confirmed by a convergence study showing the result is insensitive to further refinement (at the scheme's predicted order).
- [ ] The simulation was verified to converge (refinement does not change the result) before any result was trusted; a single-resolution run was not presented as a converged result.
- [ ] Validation was performed against independent data (analytical solutions, experiments, or a trusted second code), with the level of agreement documented — not against the simulation's own self-consistency.
- [ ] Conservation laws and invariants (mass, momentum, energy, charge) are checked at every step and do not drift beyond expected truncation error; a conservative scheme or explicit correction is used where conservation matters.
- [ ] Uncertainty in inputs is propagated (Monte Carlo or polynomial chaos) and results are reported as distributions or ranges, not point estimates; sensitivity analysis identified the dominant inputs.
- [ ] Adaptive timestepping is used for problems with varying or stiffening dynamics, with the tolerance set against the required accuracy and verified for convergence.
- [ ] No result is presented as validated on the basis of visual plausibility alone; quantitative agreement with independent data is the standard.
