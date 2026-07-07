---
name: conversational_ai_and_agent_design.md
description: Use when the agent is building a multi-turn conversational assistant or an autonomous LLM agent — designing tool or function calling, defining the agent loop and its stopping conditions, managing conversation state and context window limits across turns, handling tool failures and ambiguous user intent, choosing between planning and reactive agent architectures, constraining loops that could run forever or take destructive actions, or adding human-in-the-loop approval for irreversible operations. Also covers the failure modes of unconstrained agent loops, tools with ambiguous or overlapping schemas, context windows that silently overflow mid-conversation, unhandled tool errors that derail the agent, agents that take destructive actions without confirmation, and the recurring mistake of treating an agent loop as a clever prompt rather than a control system with termination, safety, and state-management requirements.
---

# Conversational AI And Agent Design

An LLM agent is a control system, not a prompt. It runs in a loop — observe, decide, act, observe again — where each step calls tools that change the world, and the model that drives the loop is stochastic, suggestible, and bad at knowing when to stop. The judgment problem is that the patterns that make a single LLM call feel magical produce an agent that loops forever, calls the wrong tool, takes an irreversible action on a misunderstanding, or silently overflows its context window three turns in. Agents tend to be built as "give the model the tools and let it figure it out," which works on the demo and fails in production because no one defined the termination conditions, the failure paths, the state boundaries, or the authority limits.

The harm appears in characteristic ways. An agent with no step cap burns tokens and money cycling between two tools that keep failing. A tool schema with overlapping or ambiguous names makes the model pick the wrong one confidently. A tool that errors is retried identically until the budget is gone. A conversation that accumulates history silently overflows the context window and the system prompt — the rules — get truncated, so the agent starts ignoring its constraints. An agent with write or delete authority executes a destructive action on an ambiguous user request with no confirmation. The judgment problem is to treat the agent loop as a bounded, observable, fail-safe control system: tools are a privilege surface, termination is a design requirement, state is managed explicitly, and irreversible actions require human approval.

This skill covers tool/function calling design, the agent loop and stopping conditions, conversation state and context management, tool-failure and ambiguity handling, planning versus reactive architectures, and human-in-the-loop for irreversible actions. It complements the llm-api-integration skill (call mechanics), the prompt-engineering skill (the system prompt and injection defense), and the ai-guardrails skill (output validation); here the focus is the multi-turn agent control system.

## Core Rules

### Design Tools As A Constrained, Unambiguous Privilege Surface

Tools are the agent's hands, and every tool is a privilege: the ability to read data, write state, call an API, or execute code. The tool set must be small, unambiguous, and authorized, because the model picks tools from their descriptions and will pick the wrong one if the descriptions overlap:

- **Make tool schemas self-describing and non-overlapping.** Each tool's name, description, and parameter schema should make the right choice obvious; if two tools could plausibly fit a request, the model will guess and sometimes guess wrong. Merge or rename overlapping tools until the choice is unambiguous.
- **Constrain and validate every tool argument.** The model produces the arguments; your code executes them. Validate types, ranges, enums, and authorization before invoking — an LLM-supplied argument is untrusted input to a privileged operation (see ai-guardrails skill).
- **Prefer narrow, composable tools over broad ones.** A `delete_record(id)` tool is safer than a `run_sql(query)` tool; narrow tools limit blast radius and make authorization enforceable. Avoid tools that execute arbitrary code or arbitrary queries unless strictly necessary, and sandbox them when they are.
- **Return structured, machine-readable tool results.** Tools should return data the model can reason over (status, payload, error code), not free text the model must parse. A tool that returns "success" with no detail, or a human-readable string, invites misinterpretation.

### Bound The Agent Loop With Explicit Stopping Conditions

An unconstrained agent loop is the most common agent failure: it cycles, retries, or wanders until the token or cost budget is exhausted. Termination is a design requirement, not a hope:

- **Cap the number of steps or tool calls per turn.** Set a hard maximum (e.g., 10 steps) and stop with a graceful message when reached, rather than letting the loop run open-ended. The cap is a safety floor, not a performance target.
- **Define explicit success and failure termination.** The loop should stop when the task is complete (a terminal tool returns a final answer), when the model signals it is done, when a tool fails irrecoverably, or when the step cap is hit — each with a defined user-facing outcome.
- **Detect and break cycles.** An agent that calls the same tool with the same arguments repeatedly is stuck; detect repeated state and break out (ask the user, fall back, or stop) rather than burning budget.
- **Make "I don't know" or "I need clarification" a valid stop.** The model should be able and encouraged to stop and ask rather than forcing an action on ambiguous input.

### Manage Conversation State And The Context Window Explicitly

Multi-turn conversations accumulate tokens, and the context window is finite. Left unmanaged, history overflows and the system prompt — the rules and constraints — is what gets truncated:

- **Track token usage per turn against the window.** Count system prompt, conversation history, tool results, and reserved output before each call; do not assume the window is large enough.
- **Preserve the system prompt and current turn; trim history and tool results first.** The naive "drop oldest messages" can drop the rules the agent must follow. Keep the system prompt and the current user request; summarize or truncate older turns and large tool outputs.
- **Summarize rather than carry full transcripts.** For long conversations, periodically summarize prior turns into a compact state object and carry recent turns verbatim, rather than re-sending the entire history every turn.
- **Persist durable state outside the context window.** Conversation memory, user preferences, and task progress that must survive across sessions belong in a store, not in an ever-growing prompt.

### Handle Tool Failures And Ambiguity Deliberately

Tools fail, and the model's response to a failure determines whether the agent recovers or derails. A tool error fed back as raw text often causes the model to retry identically or hallucinate a workaround:

- **Return structured errors the model can act on.** A tool should return a typed error (not found, permission denied, invalid argument, transient) so the model can decide: retry, ask the user, or try an alternative. A bare exception string invites a confused retry.
- **Do not retry non-recoverable failures identically.** A 404 or a permission-denied will fail the same way every time; feed it back as terminal so the agent stops rather than looping. Reserve retries for transient failures, bounded tightly.
- **Handle ambiguous intent by asking, not guessing.** When the user request could map to multiple actions or targets, the agent should disambiguate ("did you mean X or Y?") rather than picking one and executing — especially before any irreversible action.
- **Bound repair attempts.** A tool that keeps failing should trigger a fallback or a handoff after a small number of attempts, not an infinite repair loop.

### Choose Planning Versus Reactive Architecture By Task

Agents range from reactive (decide one step at a time from current state) to planning (decompose the goal into a sequence of steps before acting). The choice has real tradeoffs:

- **Reactive is simpler and robust to changing state.** For tasks where each step depends on the result of the previous, decide-and-act per step. It is easier to bound and debug but can wander without direction on multi-step goals.
- **Planning is better for multi-step goals but plans go stale.** Generating a plan upfront helps complex tasks, but the plan must be revisable as tool results arrive; a rigid plan executed past its relevance causes wrong actions.
- **Match the architecture to the cost of error.** High-stakes, irreversible, or expensive tasks warrant explicit planning and confirmation; cheap, reversible, exploratory tasks can be reactive. Do not add planning complexity where a single tool call suffices.

### Require Human Approval For Irreversible Actions

The defining safety property of an agent is what it is allowed to do without asking. Read-only and reversible actions can be autonomous; write, delete, send, purchase, and execute actions cannot:

- **Classify tools by reversibility and require confirmation for irreversible ones.** Mark tools as safe-to-auto-execute or requires-approval; gate the latter behind explicit user confirmation before invocation.
- **Confirm on ambiguity, not just on danger.** Even a reversible action taken on a misunderstood request can cascade; when intent is ambiguous, ask before acting.
- **Make approval part of the loop, not an afterthought.** The agent loop should pause, present the proposed action and its arguments, wait for approval, and only then execute — with the approval recorded for audit.
- **Default to least privilege.** Start the agent with the narrowest tool set and expand only as needed; an agent that can do everything will eventually do something it should not.

## Common Traps

### Unconstrained Agent Loop That Runs Forever

An agent with no step cap that cycles between failing tools or repeats the same call until the budget is gone. Cap steps, detect cycles, and define termination conditions.

### Tools With Overlapping Or Ambiguous Schemas

Two tools whose descriptions both plausibly fit a request, so the model picks the wrong one confidently. Make tool names and descriptions unambiguous and non-overlapping.

### Unvalidated Tool Arguments Executed As Trusted Input

Passing model-produced arguments straight into a privileged tool (SQL, shell, API call) without validation or authorization. Validate and authorize every argument; treat them as untrusted.

### Context Window Overflow Truncating The System Prompt

Letting conversation history and tool results accumulate until the window overflows and the rules get trimmed, so the agent starts ignoring its constraints. Count tokens per turn and trim history, never the system prompt.

### Retrying Non-Recoverable Tool Failures Identically

Feeding a 404 or permission-denied back as a retryable error so the agent loops on the same failing call. Return typed errors and stop on non-recoverable failures.

### Destructive Action On Ambiguous Intent With No Confirmation

Executing a delete, write, send, or purchase on a user request that could mean several things, with no disambiguation or approval. Require confirmation for irreversible actions and ask when intent is ambiguous.

### Rigid Plan Executed Past Its Relevance

A multi-step plan generated upfront and followed mechanically even after tool results invalidated it, causing wrong downstream actions. Make plans revisable as state changes.

### Broad Tool That Executes Arbitrary Code Or Queries

A `run_sql` or `run_shell` tool that gives the agent arbitrary execution power, turning a misunderstood request into a security incident. Prefer narrow tools and sandbox broad ones.

## Self-Check

- [ ] Tools are a small, non-overlapping, self-describing set with validated and authorized arguments, narrow rather than broad, returning structured machine-readable results.
- [ ] No tool argument reaches a privileged operation (SQL, shell, API, write) without validation and authorization; arbitrary-code or arbitrary-query tools are sandboxed or absent.
- [ ] The agent loop is bounded: a hard step/tool-call cap, explicit success and failure termination, cycle detection, and a valid "stop and ask" path.
- [ ] Non-recoverable tool failures (404, permission denied) terminate rather than retry identically; only transient failures retry, bounded tightly, with fallback or handoff after the cap.
- [ ] Ambiguous intent triggers disambiguation ("did you mean X or Y?") rather than a guessed action, especially before irreversible operations.
- [ ] Conversation state and the context window are managed explicitly: tokens counted per turn, the system prompt and current turn preserved, history and large tool results summarized or trimmed, and durable state persisted outside the prompt.
- [ ] The architecture (reactive vs planning) matches the task, plans are revisable as state changes, and planning complexity is not added where a single call suffices.
- [ ] Irreversible actions (write, delete, send, purchase, execute) require explicit human approval recorded for audit, and the agent defaults to least privilege.
- [ ] The highest-risk cases were verified — an unbounded loop, ambiguous tool schemas, unvalidated arguments, system-prompt truncation on overflow, identical retries of non-recoverable failures, destructive action on ambiguous intent, a stale rigid plan, and an arbitrary-execution tool — not only the single happy-path demo turn.
