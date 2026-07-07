---
name: conversational_and_multiturn_memory.md
description: Use when the agent is building a chatbot, conversational assistant, or multi-turn LLM feature that must remember context across turns — managing conversation history, deciding what to keep versus truncate or summarize, handling long-running sessions, storing and retrieving per-user or per-conversation state, designing memory that persists across sessions, or preventing the context window from filling with stale or irrelevant history. Also covers the failure modes of unbounded history overflowing the context window, naive truncation dropping the most important turns, the model "forgetting" an instruction stated early, stale memory polluting later turns, privacy leakage through retained conversation logs, and the recurring mistake of treating conversation memory as a simple append-only list rather than a stateful, bounded, privacy-aware store.
---

# Conversational And Multi-Turn Memory

A multi-turn conversation is a stateful interaction: what the model says in turn 5 depends on what was said in turns 1 through 4, and a system that "forgets" the user's name, their original goal, or an instruction given at the start produces an incoherent, frustrating experience. The judgment problem is that conversation memory looks like a simple append-only list ("just keep the messages and send them all"), but it is in fact a bounded, stateful, privacy-sensitive store with hard constraints. The context window is finite, so unbounded history eventually overflows. The most important content (the system prompt, the user's current goal) must survive truncation, or the model drifts. Memory that persists across sessions raises privacy and consent questions a stateless call does not. And the contents of a long conversation are mostly noise — pleasantries, corrections, digressions — that, if retained verbatim, crowd out the signal the model needs. The discipline is to treat conversation memory as a first-class, engineered store: bounded against the context window, pruned or summarized to preserve signal, structured so the model can find what matters, and governed for privacy and retention.

Agents tend to implement memory as a naive message list because the first few turns work — send the last N messages, get a coherent reply. The harm appears as conversations grow. A long chat overflows the context window and the system silently truncates the system prompt, so the model stops following its instructions. A truncation that drops the oldest turns removes the user's original goal, so the model loses the thread. A memory that retains every turn verbatim fills the window with digressions, leaving no room for the model's reasoning. A persistent memory that stores user data without consent or retention limits becomes a privacy liability. A memory that is not keyed per-user serves one user's history to another. The judgment problem is to design memory that preserves coherence and instruction-following across long and cross-session conversations while staying within the context window, protecting privacy, and keeping the signal-to-noise ratio high.

This skill covers history management, truncation and summarization, persistent and structured memory, cross-session state, and privacy/retention. It complements the llm-api-integration skill (context window mechanics), the prompt-engineering skill (the system prompt), and the rag-pipeline-design skill (retrieval, which is one form of memory). Here the focus is the specific surface of multi-turn and persistent conversational state.

## Core Rules

### Manage History Against The Context Window Explicitly

Every turn adds tokens to the context, and the context window is finite. A conversation that grows without management will overflow, and the overflow behavior (silent truncation, hard error) is rarely what the feature needs. History must be managed against the window deliberately:

- **Track token count per turn and cumulative.** Before sending, count the tokens of the system prompt, the history, the current turn, and the reserved output budget. Know how close the conversation is to the limit.
- **Decide the overflow strategy before you hit it.** When the cumulative count approaches the limit, what happens? Truncate, summarize, or switch to a retrieval-based memory (below). Discovering the strategy at overflow time produces a broken experience.
- **Reserve the output budget.** The context window is shared between input and output; a history that fills the window leaves no room for the response. Always reserve `max_tokens` for output and cap history input accordingly.
- **Do not rely on the provider's silent truncation.** Some providers truncate from the front when the window overflows, which drops the system prompt. Never let it get to silent truncation; manage proactively.

### Truncate To Preserve The Important, Not Just The Oldest

When history must be shortened, the naive strategy — drop the oldest turns — often removes the most important content. Truncation must be designed to preserve signal:

- **Always preserve the system prompt and the current user turn.** These are the most important: the system prompt defines behavior, and the current turn is what the model must respond to. Never let truncation remove them.
- **Preserve the user's original goal and key facts.** If the user said "I'm planning a trip to Tokyo" in turn 1, that fact must survive into turn 20, or the model loses the thread. Identify and carry forward the durable facts, not just recent messages.
- **Drop low-signal turns first.** Pleasantries ("thanks!", "ok"), digressions, and already-resolved sub-tasks are lower signal than the current goal and key facts. Prune these before pruning substantive turns.
- **Prefer a sliding window of recent turns plus durable facts.** A common robust pattern: keep the system prompt, a compact set of durable facts/goals, and the last N turns verbatim. This preserves recency and the thread without retaining every word.

### Summarize Long History Rather Than Discarding It Bluntly

For long conversations, verbatim retention is wasteful (most turns are noise) and summarization preserves more signal than blunt truncation — at the cost of an extra model call. Use it where conversation length justifies it:

- **Summarize older turns into a compact running summary.** Periodically (every N turns, or when history exceeds a threshold), summarize the older portion into a short block ("User is planning a Tokyo trip for March; has asked about flights and hotels; budget is mid-range; prefers areas near transit") and keep only recent turns verbatim.
- **Update the summary incrementally, do not re-summarize from scratch each turn.** A running summary that is updated with each new chunk is cheaper and more stable than re-summarizing the whole history every turn.
- **Preserve facts and decisions in the summary, not narrative.** A good summary captures durable facts, decisions, and open questions, not a blow-by-blow ("then the user said... then I said..."). The summary is a state snapshot, not a transcript.
- **Beware summary drift.** A summary that is updated many times can drift or lose detail; periodically validate it against the recent verbatim turns, or offer the model a chance to re-ground.

### Use Structured Memory For Durable, Queryable State

Beyond conversation history, many assistants need durable facts about the user or the task that persist and can be retrieved selectively — the user's preferences, their account details, the state of a multi-step task. This is structured memory, and it is more robust than stuffing everything into the context:

- **Store durable facts in a structured store, not the prompt.** User preferences, account state, task progress belong in a database keyed by user/session, retrieved and injected into the context as needed — not retained as conversation history.
- **Retrieve relevant memory per turn, not all memory always.** A user with a long history has more facts than fit in the context; retrieve the facts relevant to the current turn (this is retrieval-augmented memory, akin to RAG) rather than injecting the entire memory store.
- **Keep memory structured and typed where possible.** Typed memory (a preference is a key-value, a task has a status) is more reliable to inject and update than free-text memory the model must parse.
- **Let the model write to memory deliberately.** When the user states a durable fact ("I'm vegetarian"), the system should capture it to structured memory, not rely on it surviving in history. This can be a tool call (a `save_preference` tool) or an extraction step.

### Handle Cross-Session Memory With Consent And Scoping

Memory that persists across sessions ("remember that I prefer window seats") is powerful but raises consent, scoping, and freshness questions that within-session memory does not:

- **Persist only with user awareness and consent.** Cross-session memory is personal data the system retains about the user; in many jurisdictions, retention requires consent and a deletion path. Do not silently accumulate a profile the user cannot see or control.
- **Scope memory to the right entity.** Memory keyed per-user must not leak across users; memory scoped to a conversation must not pollute the user's other conversations. Get the keying right, or one user's memory appears in another's session.
- **Let memory expire or be refreshed.** A preference stated a year ago may be stale; a task abandoned months ago should not dominate context. Define retention and let the user review and delete memory.
- **Distinguish facts from inferences.** A fact the user stated ("I live in Berlin") is reliable; an inference the model drew ("the user seems to like budget options") may be wrong and should be treated as lower-confidence or not persisted at all.

### Govern Memory For Privacy, Retention, And Logging

Conversation logs and memory stores contain personal data — often sensitive — and their retention, access, and logging must be governed as such:

- **Do not log full conversations in plaintext by default.** Conversation transcripts may contain PII, secrets, or sensitive disclosures. Log metadata (turn count, token usage, latency) by default; retain full transcripts only where required, with redaction and access controls.
- **Define retention for conversation and memory stores.** Infinite retention is a liability; define how long conversations and memory are kept and purge on schedule. Match retention to the use case (a support chat may be retained for 90 days; a casual assistant may purge on session end).
- **Redact PII and secrets before they reach memory or logs.** A user may paste a credit card or API key into a chat; scan and redact before storing, so it does not persist in memory or logs.
- **Understand the provider's retention.** If conversations are sent to a provider, the provider's data retention and training policy is part of the privacy boundary. Use non-retaining endpoints for sensitive workloads.

## Common Traps

### Unbounded History Overflowing The Context Window

Appending every turn and sending all of it, until the window overflows and the system silently truncates — often dropping the system prompt. Track token counts, reserve output budget, and manage history proactively.

### Naive Truncation Dropping The Original Goal

Dropping the oldest turns when the window fills, removing the user's original goal or key facts so the model loses the thread. Preserve the system prompt, current turn, and durable facts; drop low-signal turns first.

### Verbatim Retention Filling The Window With Noise

Keeping every pleasantry and digression verbatim, leaving no room for the model's reasoning. Summarize older history and keep only recent turns plus durable facts verbatim.

### Losing The System Prompt Under Truncation

Letting history management or provider truncation remove the system prompt, so the model stops following its instructions. The system prompt is the highest-priority content; never let truncation touch it.

### Cross-Session Memory Without Consent Or Scoping

Silently accumulating a user profile across sessions without consent or a deletion path, or keying memory incorrectly so one user's facts appear in another's session. Persist with consent, scope per-entity, and let memory expire.

### Logging Full Conversations In Plaintext

Retaining complete conversation transcripts in plaintext logs, creating a PII and secret retention surface. Log metadata by default; redact and control access for full transcripts.

### Treating Memory As A Simple Append-Only List

Implementing memory as a message list with no bounding, summarization, structure, or privacy governance, and being surprised when it overflows, drifts, or leaks. Treat memory as a bounded, stateful, privacy-aware store.

## Self-Check

- [ ] History is managed against the context window: token counts (system + history + current + reserved output) are tracked, the overflow strategy (truncate, summarize, retrieve) is decided before overflow, output budget is reserved, and the system never relies on the provider's silent front-truncation.
- [ ] Truncation preserves the important: the system prompt and current user turn always survive, durable facts and the original goal are carried forward, low-signal turns are dropped first, and a sliding window of recent turns plus durable facts is used.
- [ ] Long conversations use summarization (a running summary of older turns, updated incrementally, capturing facts and decisions not narrative) rather than blunt truncation, with periodic re-grounding to prevent summary drift.
- [ ] Durable, queryable state is kept in structured memory (keyed by user/session, retrieved per turn, typed where possible, written deliberately via tools or extraction) rather than stuffed into the prompt as history.
- [ ] Cross-session memory persists with user consent and a deletion path, is scoped to the correct entity (no cross-user or cross-conversation leakage), expires or refreshes, and distinguishes stated facts from model inferences.
- [ ] Privacy and retention are governed: full conversations are not logged in plaintext by default (metadata with redaction and access controls instead), retention is defined and purged on schedule, PII/secrets are redacted before reaching memory or logs, and the provider's retention/training policy is understood for sensitive workloads.
- [ ] The highest-risk cases were verified — a long conversation that did not overflow or drop the system prompt, a truncation that preserved the original goal, a summary that did not drift, cross-session memory that did not leak across users, and a conversation log that did not retain PII — not only the short three-turn demo.
