---
name: prerequisite_and_context_staging.md
description: Use when the agent is editing procedural or instructional content and must verify that prerequisites, context, and starting conditions are staged before the steps begin, checking that users know what they need, what state to start in, and what the procedure assumes before they begin executing.
---

# Prerequisite And Context Staging

A procedure does not begin at step one. It begins before, with the prerequisites, context, and starting conditions that make the steps executable. A user who starts a procedure without the right tools, permissions, or system state fails at the first step, or worse, causes damage they cannot reverse. Prerequisite and context staging is the discipline of ensuring users arrive at step one ready to execute it. Editors who edit only the numbered steps miss the framing that determines whether the procedure can even begin.

Use this skill when editing procedures, tutorials, or instructional content to verify prerequisite and context staging. It covers what users need before starting, the state they should begin in, and the assumptions the procedure makes. The goal is a procedure where every user who meets the stated conditions can begin and complete it successfully.

## Core Rules

### List All Prerequisites Before The Steps

Before the steps begin, list everything the user needs to execute the procedure. This includes tools, materials, access, permissions, knowledge, and completed prior procedures. Omit a prerequisite and the user fails mid-procedure. List prerequisites.

Identify everything required to complete the procedure. This includes physical tools or materials, software or accounts, access or permissions, prior knowledge or skills, and completed prior procedures or setup. List these before the steps, in a clearly labeled prerequisites section. Be specific: not "an account" but "an administrator account with edit permissions"; not "the software" but "version 3.2 or later." A user who reviews the prerequisites can gather what they need before starting, rather than discovering a missing requirement at step four.

### Specify The Required Starting State

Beyond prerequisites, specify the state the user and system should be in when the procedure begins. Starting state includes what should be open, logged in, configured, or connected. Specify starting state.

Describe the state at the moment the user begins step one. Should they be logged in, and as whom? Should specific applications be open or closed? Should the system be in a particular configuration or mode? Should they be at a specific screen, page, or location? Specifying starting state prevents users from beginning in a state that causes the steps to fail or produce unexpected results. For example, a procedure that assumes the user is on the dashboard should say so, because a user on a different screen will find the first step's instructions do not apply.

### State The Goal And Outcome Up Front

Users follow procedures better when they understand the goal and expected outcome. Stating the goal up front orients the user and helps them recognize whether they have succeeded. State the goal.

Begin the procedure with a brief statement of what it accomplishes and what the user will have at the end. For example, "This procedure configures single sign-on for your account. When complete, you will be able to log in using your organization credentials." The goal statement helps users decide if this is the right procedure, understand the purpose of steps that might otherwise seem arbitrary, and recognize success when they reach it. Without a goal statement, users follow steps mechanically without understanding, which makes errors harder to detect and the procedure harder to adapt if something differs from the expected path.

### Clarify Scope And Boundaries

Procedures work best when their scope is clear. What does this procedure cover, and what does it not? Users who misunderstand scope may expect outcomes the procedure does not deliver or skip related procedures they need. Clarify scope.

State what the procedure covers and, where helpful, what it does not. For example, "This procedure covers installation on Windows. For macOS installation, see the separate procedure." Or, "This covers basic configuration. Advanced configuration is described in a separate section." Clarifying scope prevents users from attempting a procedure that does not apply to their situation or from expecting capabilities beyond the procedure's scope. Scope boundaries also help users find related procedures they may need before or after this one.

### Warn About Risks And Irreversible Actions Early

If the procedure involves risks, irreversible actions, or potential data loss, warn the user before the steps begin, not at the dangerous step. Early warnings let users prepare, back up, or decide whether to proceed. Warn early.

Identify any risks in the procedure: irreversible actions, potential data loss, system changes that affect other users, or actions that cannot be undone. Warn about these in the prerequisite or context section, before the steps. For example, "This procedure deletes existing configuration. Back up your current settings before beginning." Early warnings give users the chance to prepare or seek help. Warnings placed only at the dangerous step may come too late for users who have already committed to changes. Prominent, early risk warnings are an ethical and practical obligation.

### Define Terms And Conventions Used In The Procedure

If the procedure uses specialized terms, interface conventions, or notation that the user may not know, define these before the steps. Undefined terms cause confusion when they appear mid-procedure. Define terms.

Review the procedure for terms, labels, or conventions that a user meeting the prerequisites might still not know. Define these in a preliminary section or link to a glossary. This includes interface element names the procedure will reference, domain-specific terminology, abbreviations, and notation conventions. For example, if the procedure refers to "the sidebar" or "the ribbon," ensure the user knows what and where these are. Defining terms up front prevents interruptions mid-procedure when the user encounters an unfamiliar reference and has to stop to figure it out.

### Verify Assumptions Are Stated, Not Hidden

Procedures rest on assumptions: about the user's role, the system version, the environment, or the context. Hidden assumptions cause failure for users who do not match them. State assumptions.

Identify the assumptions the procedure makes about the user, system, or environment. State these explicitly in the context section. For example, "This procedure assumes you are using version 3.2 or later," or "This procedure assumes you have network access throughout." Stated assumptions let users verify they match before starting, rather than failing partway through. Hidden assumptions are discovered only through failure, which is costly and frustrating. Making assumptions explicit is a form of intellectual honesty that respects the user's time and effort.

## Common Traps

### Starting Steps Without Listing Prerequisites

List everything needed before the steps. Prevent mid-procedure failure.

### Omitting Required Starting State

Specify what should be open, logged in, or configured at step one.

### No Goal Or Outcome Statement

State what the procedure accomplishes. Orient the user.

### Unclear Scope Leading To Misuse

Clarify what the procedure does and does not cover.

### Late Warnings About Irreversible Actions

Warn about risks before the steps begin, not at the dangerous step.

### Undefined Terms Encountered Mid-Procedure

Define specialized terms and conventions up front.

### Hidden Assumptions Discovered Through Failure

State assumptions explicitly so users can verify they match.

## Self-Check

Before treating prerequisite and context staging as complete, verify:

- All prerequisites, including tools, access, permissions, and prior procedures, are listed before the steps.
- Prerequisites are specific, including versions, account types, and exact requirements.
- The required starting state is specified, including what should be open, logged in, or configured.
- The goal and expected outcome of the procedure are stated up front.
- The scope is clarified, including what the procedure does not cover.
- Any risks, irreversible actions, or potential data loss are warned about before the steps begin.
- Specialized terms, interface conventions, and notation are defined before the steps.
- Assumptions about the user, system, version, and environment are stated explicitly.
- A user reviewing the prerequisites and context could determine whether the procedure applies to them.
- A user meeting the stated conditions would arrive at step one ready to execute it successfully.
