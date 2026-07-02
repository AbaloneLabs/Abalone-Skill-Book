---
name: market_abuse_and_insider_trading_prevention.md
description: Use when the agent is preventing insider trading and market manipulation, maintaining watch lists and personal account dealing controls, building information barriers around MNPI, or designing surveillance for market abuse in securities and trading firms.
---

# Market Abuse And Insider Trading Prevention

Market abuse encompasses insider trading (dealing on the basis of inside information) and market manipulation (creating false or misleading signals about price, demand, or supply). It is regulated under securities laws such as the US Securities Exchange Act, the EU Market Abuse Regulation (MAR), the UK market abuse regime, and equivalent national rules. These regimes require firms to maintain information barriers, restrict personal account dealing, surveil trading, maintain watch lists, and report suspicious orders and transactions. Market abuse controls fail when inside information is not identified, when personal dealing is not pre-cleared, or when surveillance looks only at completed trades and misses attempted manipulation.

Use this skill before designing market abuse controls, building insider lists and watch lists, setting personal account dealing rules, or configuring trade surveillance. The goal is to make the agent think about what constitutes inside information, who has access to it, how dealing is controlled, and how manipulation is detected. Market abuse is an area where the line between legitimate and illicit activity is subtle, so controls must be precise.

## Core Rules

### Identify And Wall Off Inside Information

Inside information (or material non-public information, MNPI) is information of precise nature, not generally available, which would be likely to have a significant effect on price if it were. The first control is identifying when MNPI exists and restricting access to it.

Inside information controls:

- identify events that generate MNPI (M&A, earnings, major contracts, regulatory decisions, restructurings);
- maintain insider lists of those with access, updated promptly;
- restrict physical and electronic access to MNPI;
- apply information barriers (Chinese walls) between public and private side staff;
- document the existence and duration of inside information events;
- train staff to recognize and escalate MNPI.

If MNPI is not identified, none of the downstream controls work. Staff must know when they are in possession of inside information.

### Maintain Insider Lists And Watch Lists

Insider lists and watch lists are the operational backbone of prevention. They are distinct tools.

- insider list: a record of all persons with access to inside information for a specific event or transaction, required under MAR and similar regimes, maintained and updated;
- watch list: a broader surveillance list of staff, clients, or securities under enhanced scrutiny, used to monitor for potential abuse.

Watch lists should cover staff in possession of MNPI, staff in sensitive roles, connected parties, and securities under advisory or restructuring activity. Feed watch lists into personal account dealing and trade surveillance.

### Control Personal Account Dealing

Personal account dealing (PAD) rules prevent staff from trading on or tipping inside information. PAD controls are a core market abuse defense.

PAD controls:

- require pre-clearance of personal trades in covered instruments;
- prohibit trading in securities on the restricted or watch list;
- prohibit trading ahead of client or firm orders (front-running);
- restrict short-term trading and closing out of positions quickly;
- require disclosure of brokerage accounts and holdings;
- prohibit trading in initial public offerings or private placements in certain roles;
- monitor for trades by connected parties, including family.

Pre-clearance must be genuine, with a check against current inside information and watch lists, not a rubber stamp.

### Detect Insider Trading Patterns

Insider trading surveillance looks for trading ahead of price-moving events by those with access or their connected parties.

Indicators:

- trading by insiders or watch-listed staff ahead of announcements;
- trading by connected parties of insiders;
- unusual activity in accounts geographically or socially linked to insiders;
- account openings or position build-ups shortly before announcements;
- trading that is inconsistent with the customer's normal pattern;
- tipping patterns revealed through communication surveillance.

Coordinate trade surveillance with communications surveillance (email, chat, calls) where permitted, because insider trading often involves tipping.

### Detect Market Manipulation

Market manipulation creates false or misleading price signals. Surveillance must cover both completed trades and attempted orders.

Manipulation typologies:

- spoofing and layering: placing orders without intent to execute to move the price;
- wash trades: trading with oneself to create false volume;
- marking the close: trading near the close to influence the closing price;
- ramping: coordinated buying to inflate price;
- cross-product manipulation: manipulating one instrument to profit in a related one;
- bang the open: aggressive orders at the open to set a misleading reference price.

Surveillance should alert on order-to-trade ratios, rapid order cancellation, self-crosses, and patterns around opens, closes, and benchmarks.

### Report Suspicious Orders And Transactions

Under MAR and equivalent regimes, firms must report suspicious orders and transactions (STORs or equivalent) to the competent authority without undue delay. The threshold is reasonable suspicion, similar to the SAR standard.

Reporting elements:

- detect through surveillance or staff escalation;
- assess whether the suspicion is reasonable;
- file promptly with the required detail;
- preserve records and communications;
- maintain confidentiality and avoid tipping-off.

Not every anomaly requires a report, but unreported reasonable suspicion is a violation.

### Govern Research, Personal Trading, And Gifts

Market abuse controls extend to the production and use of research, to gifts and entertainment that could influence behavior, and to the movement of staff between public and private side roles. Each should have documented standards.

## Common Traps

### Failing To Identify MNPI

If inside information is not recognized and listed, the controls never engage. Staff training to recognize MNPI is essential.

### Pre-Clearance As A Rubber Stamp

Pre-clearance that does not check against current inside information and watch lists provides no protection.

### Surveillance Of Trades Only

Manipulation often appears in order activity, not completed trades. Surveillance that ignores orders misses spoofing and layering.

### Stale Watch Lists

Watch lists that are not updated when staff gain access to MNPI leave gaps in personal dealing controls.

### Ignoring Connected Parties

Insiders trade through family and associates. Surveillance that covers only the employee misses the abuse.

### No Communications Surveillance

Insider trading often involves tipping. Without communication surveillance, the coordination is invisible.

### Late Or Missing STORs

Delay in reporting suspicious activity, or failure to report reasonable suspicion, is itself a violation.

## Self-Check

- Is inside information identified, listed, and walled off, with insider lists maintained and updated?
- Are watch lists populated from MNPI access, sensitive roles, and connected parties, and fed into personal dealing controls?
- Does personal account dealing require genuine pre-clearance against inside information and watch lists?
- Does insider trading surveillance cover staff, connected parties, and trading ahead of announcements?
- Does market manipulation surveillance cover orders as well as completed trades, including spoofing, layering, wash trades, and marking the close?
- Are suspicious orders and transactions reported promptly with required detail and confidentiality preserved?
- Is communications surveillance coordinated with trade surveillance where permitted?
- Are research, gifts, entertainment, and staff movement between public and private side governed by documented standards?
- Are watch lists refreshed when staff gain or lose access to MNPI?
- Is staff trained to recognize and escalate inside information?
