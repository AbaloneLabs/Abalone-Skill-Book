---
name: simulation-and-training-fidelity.md
description: Use when the agent is designing simulation and training games, balancing fidelity and abstraction, selecting what to simulate and what to simplify, or evaluating whether a training simulation transfers real skills or teaches a simplified model that fails in the complex reality it claims to represent, endangering users who rely on the training.
---

# Simulation and Training Fidelity

Simulation and training games are serious games with especially high stakes — they claim to prepare users for real-world tasks (flying, surgery, military, emergency response) where failure harms people — and this raises the fidelity question to a matter of safety: what must be simulated faithfully (lest the training fail in reality) and what can be abstracted (lest the simulation be unusable). The judgment problem is that fidelity must be sufficient where it matters (the skills that transfer must be faithfully simulated) and abstracted where it does not (the details that do not transfer can be simplified), and the designer must know the difference, because over-fidelity burdens the simulation and under-fidelity endangers the user. Agents tend to miss this because the fidelity that is impressive (visual realism) is often not the fidelity that matters (the procedural and decision skills), and because a simulation that feels real can teach a simplified model that fails in complex reality. The harm is training that does not transfer, that teaches a wrong model, or that endangers users who rely on it for real tasks. This skill covers how to design simulation fidelity that transfers safely, and avoid the over-fidelity, under-fidelity, and misdirected-fidelity traps. The agent has latitude in the abstraction, but the obligation to fidelity where it matters is not optional.

## Core Rules

### Identify Which Skills Must Transfer and Simulate Those Faithfully

The first task is to identify which skills the simulation must transfer (the procedural, decision, and perceptual skills that matter in reality) and simulate those faithfully, because the simulation's value is in transferring those skills, and the fidelity budget should be spent on them. The decision rule: identify the transfer-critical skills, simulate them faithfully, and avoid spending fidelity on non-critical detail. Misdirected fidelity wastes the budget, because the fidelity was not on the transfer-critical skills.

### Abstract What Does Not Transfer to Keep the Simulation Usable

Details that do not transfer (irrelevant complexity, visual flourish) should be abstracted to keep the simulation usable, because over-fidelity on non-transfer detail burdens the simulation (cost, complexity, cognitive load) without adding training value. The decision rule: abstract non-transfer detail, and avoid over-fidelity on what does not matter. Over-fidelity burdens the simulation, because the detail did not add training value.

### Validate Transfer to Real-World Performance

The simulation must be validated for transfer — does training in the simulation improve real-world performance? — because unvalidated transfer is an assumption that can fail, and a simulation that does not transfer endangers users who rely on it. The decision rule: validate transfer to real-world performance (studies, expert assessment), and avoid assuming transfer. Unvalidated transfer endangers users, because the transfer was assumed not proven.

### Avoid Teaching a Simplified Model That Fails in Complex Reality

Simulations simplify by necessity, but the simplification must not teach a model that fails in complex reality — the simulation must preserve the complexity that matters, the edge cases, the uncertainty — because a user trained on a simplified model may be unprepared for the real complexity, and the failure emerges in reality where it harms. The decision rule: preserve the complexity that matters, and avoid simplifications that fail in reality. Over-simplified models fail in reality, because the complexity was stripped.

### Calibrate Difficulty and Stress to Real-World Conditions

Training should calibrate difficulty and stress to real-world conditions (the pressure, the stakes, the ambiguity), because training under unrealistically easy or low-stress conditions does not prepare the user for the real conditions, and the transfer fails when the real conditions differ. The decision rule: calibrate difficulty and stress to real conditions, and avoid unrealistically easy training. Uncalibrated training fails to transfer, because the conditions did not match.

### Involve Domain Experts Throughout Design and Validation

Simulation design requires domain expertise (the real task's procedures, complexities, failure modes), and domain experts must be involved throughout design and validation, because without them the simulation will misrepresent the task and the fidelity decisions will be wrong. The decision rule: involve domain experts throughout, and avoid designer-only simulation. Designer-only simulation misrepresents, because the expertise was absent.

## Common Traps

### Misdirected Fidelity Wasting the Budget

The team spends fidelity on visual realism or impressive detail rather than on the transfer-critical skills, and the fidelity is misdirected. The trap is that the simulation looks real. The false signal is that the fidelity is high. The harm is that the misdirected fidelity does not improve transfer (the visual realism does not teach the procedural skill), the transfer-critical skills are under-simulated, the fidelity budget is wasted on non-critical detail, and the simulation that looks impressive fails to train effectively, because the fidelity was not on the skills that transfer.

### Over-Fidelity Burdening the Simulation

The team simulates non-transfer detail faithfully, and the over-fidelity burdens the simulation. The trap is that the detail is accurate. The false signal is that the simulation is complete. The harm is that the over-fidelity adds cost, complexity, and cognitive load without training value, the simulation becomes expensive and hard to use, the non-transfer detail crowds the experience, the training is burdened by fidelity that does not serve it, and the simulation is overbuilt for its purpose, because the detail was not abstracted.

### Unvalidated Transfer Endangering Users

The team ships a simulation without validating transfer, and the unvalidated transfer endangers users. The trap is that the simulation feels real. The false signal is that the training works. The harm is that the unvalidated transfer is an assumption, the simulation may not improve real-world performance, the users who rely on the training are unprepared, the failure emerges in reality where it harms people, and the simulation that claimed to prepare users endangers them instead, because the transfer was not validated.

### Over-Simplified Models Failing in Complex Reality

The team simplifies the simulation beyond what preserves real complexity, and the over-simplified model fails in reality. The trap is that the simplification aids learning. The false signal is that the model is clear. The harm is that the over-simplified model strips the complexity that matters, the user trained on the simple model is unprepared for real complexity, the edge cases and uncertainty that define real performance are absent, the failure emerges in reality, and the training endangers the user it claimed to prepare, because the complexity was stripped.

### Uncalibrated Training Failing to Transfer

The team trains under easy, low-stress conditions, and the uncalibrated training fails to transfer. The trap is that the training is accessible. The false signal is that the users succeed in training. The harm is that the uncalibrated training does not match real conditions, the user succeeds in the easy simulation but fails in the hard reality, the transfer fails when the stakes and stress differ, the user is unprepared for the real conditions, and the training that felt successful fails in reality, because the conditions were not calibrated.

### Designer-Only Simulation Misrepresenting the Task

The team designs the simulation without domain experts, and the designer-only simulation misrepresents the task. The trap is that the designers understand the task. The false signal is that the simulation is designed. The harm is that the designer-only simulation misrepresents the real procedures, complexities, and failure modes, the fidelity decisions are wrong without expertise, the simulation teaches a task that does not match reality, the users trained on the misrepresentation are unprepared, and the simulation fails its training purpose, because the expertise was absent.

## Self-Check

- Have the transfer-critical skills been identified and simulated faithfully?
- Is non-transfer detail abstracted to keep the simulation usable rather than over-fidelitous?
- Has transfer to real-world performance been validated (studies, expert assessment), not assumed?
- Does the simulation preserve the complexity that matters rather than teaching an over-simplified model?
- Are difficulty and stress calibrated to real-world conditions rather than unrealistically easy?
- Are domain experts involved throughout design and validation, not designer-only?
- Did I confirm the simulation transfers real skills safely rather than endangering users who rely on it?
