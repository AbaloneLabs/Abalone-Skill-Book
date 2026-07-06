---
name: screen-share-and-remote-session-safety.md
description: Use when the agent is guiding screen sharing, remote support sessions, co-browsing, remote control, live customer troubleshooting, visual inspection of account settings, or agent-assisted setup where risks include exposing sensitive data, taking unauthorized action, changing customer systems, recording private information, mishandling consent, or losing control of a real-time support interaction.
---

# Screen Share And Remote Session Safety

Screen sharing and remote sessions can resolve issues quickly because the agent sees the customer's real environment. They also expose private data, credentials, internal customer systems, regulated information, and live operational settings. Agents often focus on solving the visible problem and miss consent, recording, redaction, scope control, action authorization, and what should not be touched. This skill helps the agent use live visual or remote access support without creating privacy, security, or trust harm.

## Core Rules

### Confirm consent and session scope before viewing

Before screen sharing, co-browsing, or remote control, confirm that the customer agrees to the session, understands what the agent can see or do, and knows how to pause or stop. Define the objective: inspect settings, reproduce an error, guide a workflow, review logs, or observe behavior.

Do not treat a support request as permission to view everything on the screen. Consent should be specific to the support purpose. If the customer invites the agent to view unrelated private systems, steer back to the relevant window or ask them to hide sensitive content.

### Minimize sensitive exposure

Ask the customer to close unrelated tabs, documents, chats, email, password managers, financial records, personal files, medical records, HR systems, or customer data lists before sharing. Use application-window sharing when possible instead of full desktop sharing.

If sensitive data appears, pause and ask the customer to hide it. Do not read, transcribe, screenshot, or store sensitive information unless policy explicitly requires and permits it. The agent should avoid turning incidental exposure into permanent support records.

### Prefer guided action over agent control

When possible, tell the customer what to click and why rather than taking control. Guided action keeps ownership with the customer, reduces unauthorized changes, and helps them learn the workflow. Remote control should be reserved for approved cases where guidance is impractical and the action is low risk or clearly authorized.

If remote control is used, narrate each action before taking it. Do not navigate to billing, security, admin, deletion, export, user-management, or irreversible settings without explicit confirmation.

### Authenticate and authorize sensitive changes separately

A screen share does not replace identity verification or authorization. If the session reveals that the customer wants to change ownership, reset security settings, export data, add an admin, delete records, change billing, or access private account data, complete the required verification and approval flow.

Do not accept "you can see I am logged in" as the only proof of authority when policy requires stronger checks. Shared screens can be operated by unauthorized people.

### Control recording, screenshots, and notes

Know whether the tool records sessions, captures screenshots, stores chat, or logs remote-control actions. Tell the customer when policy requires disclosure. Capture only what is necessary for the case, and redact sensitive details before attaching evidence.

Ticket notes should describe observations and steps, not reproduce private on-screen data. If a screenshot is needed, ask the customer to mask unrelated data first.

### Avoid unsupported system changes

Remote sessions may tempt the agent to fix browser settings, device permissions, registry values, firewalls, VPNs, production configurations, integrations, or third-party tools. Stay within support scope. If the issue requires IT, developer, admin, vendor, or security owner action, document findings and hand off rather than improvising.

The agent should not become an unapproved administrator of the customer's environment.

### Manage real-time interaction professionally

Live sessions need pacing. Explain what is happening, ask permission before transitions, summarize findings, and avoid silence while inspecting. If troubleshooting stalls, pause to recap and decide whether to collect evidence, escalate, or schedule follow-up.

Do not keep the customer in a long exploratory session with no path. A controlled exit often protects trust better than guessing live.

### Close with a record and next-step boundary

End by summarizing what was inspected, what changed, what did not change, what remains open, and who owns the next action. If no changes were made, say so. If configuration was changed, note the customer approval and exact change.

The support record should make later reviewers comfortable that the session was consensual, scoped, and controlled.

## Common Traps

- Starting a screen share without clear consent, purpose, and stop control.
- Letting the customer share the whole desktop when only one product window is needed.
- Capturing screenshots that include unrelated email, credentials, personal data, or third-party customer data.
- Treating login state on screen as proof of account authority.
- Taking remote control and making admin, billing, deletion, or security changes without separate confirmation.
- Debugging the customer's device, VPN, firewall, or browser beyond support scope.
- Staying silent during inspection, making the customer anxious about what is being viewed; forgetting that recordings, transcripts, or remote-control logs may become retained support records
- Leaving the customer uncertain about what changed during the session; using live sessions to bypass normal escalation, verification, or audit controls

## Self-Check

- Did the customer explicitly consent to the screen share, co-browse, or remote-control session?
- Is the session purpose and scope narrow enough for the issue?
- Has the customer hidden unrelated tabs, documents, messages, credentials, and sensitive records?
- Is application-window sharing used when practical instead of full desktop sharing?
- Are screenshots, recordings, notes, and attachments limited and redacted according to policy?
- Is remote control avoided unless necessary and approved?
- Are sensitive account, billing, admin, export, deletion, or security changes separately verified and authorized?
- Has the agent stayed within support scope and avoided unsupported customer-environment administration?
- Are all actions narrated and customer confirmations captured where needed?
- Does the closing note state what was inspected, what changed, what remains open, and who owns next steps?
