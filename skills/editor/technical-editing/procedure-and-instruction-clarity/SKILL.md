---
name: procedure-and-instruction-clarity.md
description: Use when the agent is editing how-to guides, step-by-step instructions, setup procedures, troubleshooting flows, recipes, assembly directions, or any procedural text where readers must perform actions in sequence and where ambiguity causes failure.
---

# Procedure And Instruction Clarity

Procedural writing fails when readers cannot reproduce the intended result. Instructions live or die by whether a competent reader, following them exactly, reaches the stated outcome without confusion, error, or dangerous shortcuts. Editors of procedures are not polishing prose; they are testing the text against the physical or logical sequence it describes. A misplaced step, an unstated prerequisite, an ambiguous result, or a missing exception can turn a guide into a source of failure, support tickets, damaged equipment, or safety incidents. The editor's job is to read the procedure as a hostile user would and find every place it can break.

## Core Rules

### Establish The Reader's Starting State And Prerequisites

Every procedure assumes a starting state. If the reader does not match that state, the procedure fails silently. Before the first step, the procedure must state what the reader needs: tools, materials, access, permissions, prior steps completed, environment conditions, and skill level. A procedure that begins "Click the Settings icon" fails for readers on a different interface version, a different device, or without the required access. Prerequisites belong before the steps, not buried inside them. When a prerequisite is conditional, state the condition. When a prerequisite is easy to overlook, repeat it where relevant. Do not assume the reader remembers prerequisites stated fifty pages earlier.

### Make Each Step Atomic And Verifiable

A step is one action with one outcome. If a step contains multiple actions, the reader cannot tell which caused a problem when something fails. Split compound steps. Each step should describe what to do, and each step should have a verifiable result the reader can confirm before moving on. "Click Submit" is an action; "Click Submit. The confirmation page appears" is an action with a verifiable result. Verifiable results let readers catch errors early instead of discovering them ten steps later when the diagnosis is far harder. Where a result is not visible to the reader, say so explicitly and explain how to verify indirectly.

### Use Imperative Voice And Consistent Action Verbs

Procedures use imperative voice: "Click the button," not "You should click the button" or "The button is clicked." Consistency matters because readers scan. If the verb for the same action changes between steps, readers slow down and second-guess. Choose a small set of action verbs and use them consistently: click, select, type, press, drag, choose, enter. Define interface terms once and reuse them. If the interface uses "Submit" as a button label, the procedure says "Click Submit," not "Click the send button." Match the actual interface labels exactly, including capitalization, because readers match text on screen to text in the procedure.

### Number Steps Sequentially And Avoid Branching Inside Lists

Numbered lists signal sequence. Bulleted lists signal unordered options. Procedures are sequential, so use numbers. When a procedure branches, handle the branch outside the numbered list: present the decision, then provide separate numbered sub-procedures for each path. Do not embed conditionals inside a numbered step, because the reader cannot tell whether to continue or jump. If step 5 says "If you see an error, go to step 12; otherwise continue," the reader must hold that branch in memory while executing intervening steps. Instead, end the main procedure at the decision point and create a clearly labeled alternate path.

### State Expected Results And Failure Modes

After critical steps, state what the reader should observe. After steps prone to failure, state what to do if the expected result does not appear. Anticipate the common errors and provide recovery. A procedure that only describes the happy path leaves readers stranded at the first deviation. Failure handling does not need to be exhaustive, but it must cover the errors most readers will actually hit. Prioritize failure modes by likelihood and consequence: a rare error that destroys data deserves more attention than a common error that is merely annoying.

### Test The Procedure Against The Real System

The strongest check is to perform the procedure against the real system, interface, or material. If that is impossible, perform it mentally while looking at screenshots, recordings, or a knowledgeable author's description. Testing reveals missing steps, wrong order, stale screenshots, changed labels, and prerequisite gaps that reading alone misses. Procedures rot when the underlying system changes, so note the version or date the procedure was verified against and flag procedures likely to need retesting after system updates.

### Match Detail To Reader Competence And Stakes

Detail is not always better. An expert reader finds exhaustive detail insulting and slow; a novice reader finds terse instructions impossible. State the intended audience and calibrate detail accordingly. For high-stakes procedures, where errors cause data loss, safety hazards, or financial harm, increase detail and add explicit warnings regardless of audience. For low-stakes procedures with expert readers, compress. The editor's judgment is matching density to consequence.

## Common Traps

### Assuming The Reader Shares The Author's Mental Model

Authors who know a system intimately skip steps they consider obvious. What is obvious to the author is often invisible to the reader. The editor must read as someone who does not already know the procedure and ask, at each step, whether a competent but uninformed reader could perform it. When the author writes "Configure the settings as needed," the editor must ask what settings, what values, and what "as needed" means. Vague phrases like "as appropriate," "if necessary," and "in the usual way" are signals that the author has offloaded judgment to the reader without giving the reader the basis to exercise it.

### Stale Screenshots And Interface References

Screenshots and interface references age quickly. A button labeled "Save" becomes "Save Changes" in a new release, and the procedure now contradicts the screen. When editing, check screenshots against the current interface, confirm labels match, and verify that referenced menu paths still exist. If screenshots cannot be updated, note the version they reflect. Prefer text descriptions that age gracefully over screenshots that rot, and use screenshots to confirm, not replace, text.

### Conflating Procedure With Explanation

Procedures tell readers what to do; explanations tell readers why. Mixing them confuses both. A step that explains the theory behind an action interrupts the reader who is trying to execute. Separate the two: keep steps as actions, and move explanation into introductory paragraphs, callouts, or a separate section readers can skip. Where a step needs brief justification to motivate compliance, keep it to one sentence.

### Hiding Prerequisites Inside Steps

A step that says "Log in with your admin credentials" buried at step 7 reveals a prerequisite too late. Readers who lack admin access discover this after completing six steps. Surface prerequisites at the top, in a prerequisites list, before the procedure begins. If a prerequisite emerges mid-procedure, move it up.

### Inconsistent Units, Values, And Naming

Procedures often contain values: temperatures, durations, quantities, file paths, version numbers. Inconsistency here causes failure. If step 3 says "heat to 350 degrees" and step 5 says "bake at 180 degrees" without specifying Fahrenheit or Celsius, the reader cannot proceed. Check that units are stated, values are consistent across steps, file paths are correct and platform-appropriate, and version numbers match the actual software. Naming must match the interface exactly.

### Ignoring Accessibility And Alternate Paths

Procedures that assume one input method exclude readers who use another. Keyboard-only users need keyboard equivalents for mouse actions. Screen reader users need text alternatives for visual cues. Readers on different operating systems need platform-specific paths and shortcuts. Where the procedure assumes a specific setup, state it. Where alternate methods exist and matter, provide them. Accessibility is not optional polish; it determines whether the procedure works for the full readership.

### Treating Warnings As Decorative

Warnings that appear after the dangerous step are useless. Warnings buried in dense paragraphs are missed. Place warnings before the step that triggers the hazard, make them visually distinct, and state the consequence of ignoring them. "Caution: data loss" is weaker than "Caution: This step permanently deletes the selected records and cannot be undone." Specific consequence motivates compliance.

## Self-Check

- Does the procedure state all prerequisites, including access, tools, materials, environment, and prior steps, before the first numbered step?
- Is each step a single action with a verifiable result the reader can confirm before continuing?
- Do action verbs and interface labels match the actual system exactly, including capitalization?
- Are steps numbered sequentially, with any branching handled as separate labeled sub-procedures rather than embedded conditionals?
- Does the procedure state expected results after critical steps and provide recovery for common failure modes?
- Has the procedure been tested, or at least mentally walked through, against the real system or current screenshots?
- Is the level of detail calibrated to the stated audience and to the stakes of error?
- Are screenshots current, correctly labeled, and consistent with the text, with versions noted where they may be stale?
- Are warnings placed before the hazardous step, visually distinct, and specific about consequences?; have vague phrases like "as needed," "if appropriate," and "in the usual way" been replaced with concrete values or removed?
- Does the procedure account for readers using different input methods, operating systems, or accessibility tools where relevant?; are units, quantities, file paths, and version numbers consistent across all steps?
