---
name: confidentiality_and_data_handling.md
description: Use when the agent is handling confidential or sensitive source content during translation, deciding what data may be processed by cloud tools or machine translation, managing non-disclosure obligations, or protecting personal health financial and legal information throughout the translation workflow.
---

# Confidentiality And Data Handling

Translation work is built on trust. Clients hand over content they would not share publicly: unreleased financial results, pending legal filings, personal health data, proprietary source code, trade secrets, and internal communications. The translator and the translation workflow become custodians of that confidentiality, and the obligations do not end with good intentions. Every tool, channel, and process the content touches is a potential breach point: a cloud-based CAT tool that uploads source to a vendor server, a machine translation service that stores input, an email that forwards a draft to the wrong recipient, a chat message that pastes a confidential snippet for help, a lost laptop with cached files. Confidentiality breaches are not usually malicious; they are usually the result of not thinking through where data goes and who can see it. For sensitive categories such as personal, health, financial, and legal data, confidentiality is also a legal obligation under regimes such as GDPR, HIPAA, or attorney-client privilege, and breaches carry legal liability alongside reputational harm. Handling confidential content is a discipline of anticipating data flows, enforcing controls, and recognizing when content is too sensitive for a given tool or process.

Use this skill when handling confidential or sensitive content, selecting tools and channels, managing non-disclosure obligations, or protecting regulated data. The goal is to preserve confidentiality and comply with legal obligations throughout the translation workflow, ensuring that sensitive content reaches only those authorized to see it.

## Core Rules

### Identify Confidential And Sensitive Content Upfront

You cannot protect what you have not identified as sensitive. Identify content upfront.

Before beginning work, assess the content's sensitivity: is it confidential business information such as unreleased results or strategy; personal data about identifiable individuals; health information protected by HIPAA or similar; financial data subject to regulation; legal content under privilege; or source code and trade secrets. Classify the content's sensitivity level and the obligations that attach to it, such as non-disclosure agreements, regulatory regimes, or contractual confidentiality clauses. This classification drives every subsequent decision about tools, channels, storage, and sharing. Content treated as non-sensitive when it is sensitive will be handled through inappropriate, insecure channels.

If you are unsure whether content is sensitive, treat it as sensitive until confirmed otherwise.

### Map Where Data Goes Before Processing

Every tool and process sends data somewhere. Map the data flow before processing sensitive content.

For each tool and channel, determine where the content is stored and processed: a desktop CAT tool may keep data local; a cloud tool uploads it to vendor servers, possibly in another jurisdiction; machine translation services may store and log input; email and chat traverse provider infrastructure. For sensitive content, this matters: data sent to a cloud service in another country may violate data residency requirements; data sent to an MT service may be retained or used for training. Before using any tool or channel for sensitive content, verify where the data goes, how it is stored, who can access it, and whether the flow complies with the content's obligations. Do not assume a tool is safe because it is convenient.

If a data flow is unknown or non-compliant, do not use that path for sensitive content.

### Honor Non-Disclosure And Legal Obligations

Confidentiality is often a legal obligation, not just a professional norm. Honor NDAs and regulations.

Non-disclosure agreements define what content is confidential, how it may be used, and the term of the obligation. Regulatory regimes such as GDPR for personal data, HIPAA for health information, and attorney-client privilege for legal communications impose specific handling requirements. Understand the obligations that apply to the content and comply with them strictly: do not disclose confidential content to unauthorized parties, do not use it beyond the engagement, and follow required safeguards. Breaches of NDAs and regulations carry legal liability, including fines and lawsuits, beyond reputational harm. When obligations are unclear, seek clarification from the client or counsel before proceeding.

Ignorance of an obligation is not a defense; confirm and comply.

### Restrict Access To Authorized Personnel

Sensitive content should reach only those who need it. Restrict access.

Limit access to the content to the specific individuals working on it under the same obligations. Do not forward drafts to colleagues for help without verifying they are authorized and bound by confidentiality. Do not copy content into shared drives, repositories, or knowledge bases where others can see it. Use access controls on tools and storage to enforce restrictions. For team workflows, ensure every member is under NDA and trained on the content's sensitivity. Over-sharing is a common breach path, because each additional person with access increases the risk of intentional or accidental disclosure.

Need-to-know is the standard: if someone does not need the content to do their work, they should not have it.

### Avoid Insecure Channels For Sensitive Content

Convenience channels are frequent breach points. Avoid insecure channels for sensitive content.

Public or consumer email, chat applications, personal cloud storage, and public forums are not appropriate for confidential content: they may not encrypt in transit or at rest, they may be accessible to providers, and they are prone to misaddressing. Do not paste confidential snippets into public forums or AI tools for help, because that discloses the content to the service and potentially the public. Use encrypted, enterprise-grade channels approved for the content's sensitivity. For highly sensitive content, use secure file transfer, encrypted email, or air-gapped workflows. The convenience of a quick chat message is not worth a confidentiality breach.

If a channel is not approved for the content's sensitivity, do not use it, regardless of convenience.

### Be Cautious With Machine Translation And AI Tools

MT and AI tools process content on external infrastructure. Be cautious with sensitive content.

Machine translation services and AI assistants typically send content to vendor servers for processing, and some retain or use input for training. For confidential, personal, or regulated content, this can constitute a disclosure. Verify the tool's data handling: does it retain input, use it for training, or allow opt-out? Use enterprise versions with no-retention guarantees, or avoid MT and AI entirely for the most sensitive content. Many breaches have occurred because sensitive content was pasted into a consumer MT or AI tool. Treat any content sent to an external service as potentially disclosed, unless the service's terms guarantee otherwise.

When in doubt, do not send sensitive content to MT or AI; translate or post-edit with secure alternatives.

### Secure Storage And Dispose Of Content Properly

Sensitive content must be stored securely and disposed of when no longer needed. Manage the lifecycle.

Store confidential content on encrypted, access-controlled systems, not personal devices or unsecured drives. Retain it only as long as needed for the engagement and as required by contract or regulation, then dispose of it securely: delete files, clear caches, and confirm backups are purged if required. Some obligations require return or destruction of content at engagement end; honor those terms. Retaining sensitive content beyond need increases breach risk and may violate obligations. Document retention and disposal where required.

A cache or backup that retains content after "deletion" is a hidden breach risk; verify true disposal.

### Recognize And Respond To Breaches

If a breach may have occurred, recognize and respond promptly. Do not conceal it.

A misaddressed email, a lost device, an unauthorized access, or a tool that disclosed content are potential breaches. If one occurs, assess what was disclosed, to whom, and the impact, and follow the required response: notify the client, notify affected individuals if required by regulation, and take steps to contain and remediate. Many regulations require prompt notification, and concealment increases liability. Establish a breach response process in advance so that response is swift rather than panicked. Prompt, honest response limits harm; concealment compounds it.

## Common Traps

### Not Identifying Content As Sensitive

Content treated as non-sensitive is handled through insecure channels; identify sensitivity upfront.

### Unknown Or Non-Compliant Data Flows

Sending sensitive content to a cloud tool or MT service without verifying the flow breaches obligations.

### Breaching NDAs Or Regulations

Disclosing confidential or regulated content carries legal liability beyond reputational harm.

### Over-Sharing Access

Each additional person with access increases breach risk; restrict to need-to-know.

### Using Convenience Channels

Email, chat, and public forums are inappropriate for confidential content; use approved encrypted channels.

### Pasting Sensitive Content Into MT Or AI

Consumer MT and AI tools may retain or disclose content; use secure alternatives or avoid for sensitive data.

### Retaining Content Beyond Need

Caches, backups, and retained files increase breach risk and may violate disposal obligations.

### Concealing A Breach

Hiding a breach increases liability; prompt, honest response limits harm.

## Self-Check

Before processing or delivering sensitive content, verify:

- The content's sensitivity is identified and classified, including personal, health, financial, legal, or proprietary data, with applicable obligations noted.
- Data flows for every tool and channel are mapped, confirming where content is stored and processed and whether the flow complies with obligations.
- Non-disclosure agreements and regulatory regimes such as GDPR, HIPAA, and privilege are understood and complied with strictly.
- Access is restricted to authorized personnel on a need-to-know basis, with all members under NDA and trained on sensitivity.
- Only approved, encrypted channels are used, with no sensitive content sent through consumer email, chat, or public forums.
- Machine translation and AI tools are used only where data handling is verified, with secure alternatives or avoidance for highly sensitive content.
- Content is stored encrypted and access-controlled, retained only as needed, and securely disposed of with caches and backups purged.
- A breach response process exists, and any potential breach is recognized, assessed, and reported promptly without concealment.
- No sensitive content reaches an unauthorized party, channel, or external service.
- Confidentiality and legal obligations are preserved throughout the workflow, protecting the client and the individuals the content concerns.
