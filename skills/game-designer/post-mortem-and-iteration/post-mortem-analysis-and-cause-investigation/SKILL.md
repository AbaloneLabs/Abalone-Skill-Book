---
name: post-mortem-analysis-and-cause-investigation.md
description: Use when the agent is conducting a post-mortem after a game's release or milestone, investigating the root causes of successes and failures, or evaluating whether a post-mortem produces honest actionable learning or degenerates into blame-shifting, confirmation bias, and root causes that are never actually identified so the same failures repeat on the next project.
---

# Post-Mortem Analysis and Cause Investigation

A post-mortem is the project's chance to convert hard-won experience into learning that improves the next project, and it is also where most teams fail to learn, because post-mortems easily degenerate into blame-shifting, confirmation bias, and surface symptom-chasing that never reaches the root causes, so the same failures repeat. The judgment problem is that a post-mortem must be honest (facing the real causes, including uncomfortable ones) without being punitive (which destroys honesty), must investigate root causes (not symptoms) without endless analysis, and must produce actionable learning (not just narrative). Agents tend to miss this because the causes that are comfortable to cite (tools, time, scope) are often not the root causes (process, decisions, communication), and because a post-mortem that feels productive (lots of discussion) can produce no learning if it never reaches actionable root causes. The harm is teams that repeat the same failures project after project because the post-mortems never learned. This skill covers how to conduct post-mortems that produce honest, actionable root-cause learning, and avoid the blame, bias, and symptom traps. The agent has latitude in the process, but the obligation to reach honest root causes is not optional.

## Core Rules

### Foster Psychological Safety for Honest Disclosure

A post-mortem requires psychological safety — participants must feel safe to disclose mistakes, failures, and uncomfortable truths without punishment — because without safety the disclosure is defensive and the real causes stay hidden, and the learning fails before it begins. The decision rule: foster psychological safety (blameless process, focus on systems not people), and avoid punitive post-mortems. Punitive post-mortems hide causes, because the disclosure was not safe.

### Investigate Root Causes, Not Surface Symptoms

Post-mortems must investigate root causes (the underlying decisions, processes, and systemic conditions) not surface symptoms (the visible failures the symptoms produced), because addressing symptoms without roots ensures the failure repeats, and root-cause investigation (the five-whys method, systemic analysis) is what produces learning. The decision rule: investigate root causes with systematic methods, and avoid stopping at symptoms. Symptom-stopping post-mortems do not learn, because the roots were not reached.

### Guard Against Confirmation Bias and Narrative Force-Fitting

Post-mortems are vulnerable to confirmation bias (finding the causes the team expected) and narrative force-fitting (constructing a clean story that may not match the messy reality), and the investigation must guard against both by seeking disconfirming evidence and tolerating ambiguity, because biased or force-fit conclusions are false learning. The decision rule: guard against bias (seek disconfirming evidence, tolerate ambiguity), and avoid force-fit narratives. Biased post-mortems produce false learning, because the conclusions were force-fit.

### Use Evidence and Data, Not Just Memory and Opinion

Post-mortems should use evidence and data (metrics, timestamps, commit history, playtest records) not just memory and opinion, because memory is unreliable and opinion is biased, and the evidence grounds the investigation in what actually happened rather than what is remembered or believed. The decision rule: use evidence and data to ground the investigation, and avoid memory-and-opinion-only analysis. Memory-only post-mortems misremember, because the evidence was not used.

### Separate People Problems From System Problems

Post-mortems should separate people problems (individual failures) from system problems (process, structure, communication failures), and focus on system problems (which, when fixed, prevent recurrence across the team) over people problems (which, when blamed, produce defensiveness without systemic fix), because most failures are system failures even when individuals were involved. The decision rule: focus on system problems (process, structure), treat people problems as symptoms, and avoid blame. Blame-focused post-mortems do not fix systems, because the systemic causes were not addressed.

### Produce Actionable, Owned, Followed-Up Learning

Post-mortem learning must be actionable (specific changes), owned (assigned to people), and followed up (tracked to implementation), because learning that is not actionable, owned, and followed up is narrative that produces no change, and the failures repeat. The decision rule: produce actionable, owned, followed-up learning, and avoid narrative-only conclusions. Narrative-only post-mortems do not change, because the learning was not actionable.

## Common Traps

### Punitive Post-Mortems Hiding Causes

The team runs a punitive post-mortem that assigns blame, and the real causes are hidden. The trap is that accountability improves outcomes. The false signal is that the responsible parties are identified. The harm is that the punitive atmosphere destroys psychological safety, the participants disclose defensively to avoid blame, the real causes (often systemic) stay hidden behind the blamed individuals, the learning fails, and the same failures repeat because the systemic causes were never addressed, because the post-mortem was punitive.

### Symptom-Stopping Post-Mortems Not Learning

The team stops the investigation at surface symptoms, and the root causes are not reached. The trap is that the symptoms are the visible failures. The false signal is that the failures are identified. The harm is that the symptom-stopping post-mortem addresses the visible failures, the root causes (the decisions and processes that produced the symptoms) are not investigated, the fixes address symptoms not roots, the failures repeat on the next project, and the post-mortem produces no learning, because the roots were not reached.

### Confirmation Bias and Narrative Force-Fitting

The team falls into confirmation bias or narrative force-fitting, and the conclusions are false learning. The trap is that the conclusions are clean. The false signal is that the story makes sense. The harm is that the biased investigation finds the expected causes, the force-fit narrative constructs a clean story that ignores messy disconfirming evidence, the conclusions are false learning that confirms preconceptions, the real causes are missed, and the next project repeats the failures because the learning was wrong, because the bias was not guarded against.

### Memory-and-Opinion-Only Analysis Misremembering

The team conducts the post-mortem from memory and opinion without evidence, and the analysis misremembers. The trap is that the team remembers what happened. The false signal is that the recollection is vivid. The harm is that the memory is unreliable and the opinion is biased, the analysis is grounded in misremembering and belief rather than fact, the conclusions are based on inaccurate reconstruction, the real causes are distorted by the faulty memory, and the post-mortem learns from a version of events that did not happen, because the evidence was not used.

### Blame-Focused Post-Mortems Not Fixing Systems

The team focuses on people problems and blame, and the system problems are not fixed. The trap is that the individuals failed. The false signal is that the responsible people are addressed. The harm is that the blame-focused post-mortem attributes failures to individuals, the systemic causes (process, structure, communication) are not addressed, the defensive individuals are punished without systemic change, the same system produces the same failures with different individuals, and the failures repeat because the systems were not fixed, because the focus was on people.

### Narrative-Only Post-Mortems Producing No Change

The team produces narrative conclusions without actionable, owned, followed-up learning, and the post-mortem produces no change. The trap is that the narrative captures the lessons. The false signal is that the post-mortem is documented. The harm is that the narrative-only conclusions are not actionable or owned, the learning is not followed up to implementation, the discussion feels productive but produces no change, the next project repeats the failures, and the post-mortem's learning evaporates without effect, because the learning was not actionable and followed up.

## Self-Check

- Is psychological safety fostered through a blameless, system-focused process?
- Does the investigation reach root causes (decisions, processes, systems) rather than stopping at symptoms?
- Is the investigation guarded against confirmation bias and narrative force-fitting with disconfirming evidence?
- Is the analysis grounded in evidence and data (metrics, records) rather than memory and opinion?
- Are system problems prioritized over people problems, avoiding blame that avoids systemic fix?
- Is the learning actionable, owned by specific people, and tracked to follow-up implementation?
- Did I confirm the post-mortem produces honest, actionable root-cause learning rather than blame and narrative?
