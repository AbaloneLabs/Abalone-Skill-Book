---
name: order_type_selection.md
description: Use when the agent is choosing between market, limit, stop, and stop-limit orders, deciding which order type protects against adverse fills versus risks non-execution, sizing orders relative to liquidity, or selecting the right order for entering, exiting, and managing positions across liquid and illiquid securities.
---

# Order Type Selection

An order type is a rule about the price at which you are willing to trade and what happens if that price is not available. It is not a trivial mechanical choice; it determines whether a trade fills, at what price, and how much slippage is absorbed. The judgment problem is that each order type trades one risk for another: a market order guarantees execution but not price, a limit order guarantees price but not execution, and a stop order triggers automatically but can fill badly in fast markets. Selecting the wrong type turns a sound investment decision into a poor fill, and in illiquid names the choice can be expensive.

The harm this skill prevents is using market orders in illiquid or volatile names and suffering large slippage, using limit orders too tightly and missing the fill entirely, placing stops that get triggered by noise and fill far below the trigger, and failing to match the order type to the liquidity and urgency of the situation. The agent's job is to select the order type that manages the relevant risk, execution certainty versus price control, for the specific security and market condition. Order mechanics vary by venue and product, and extended-hours and international markets behave differently.

## Core Rules

### Match The Order Type To The Tradeoff Between Certainty And Price

Every order type resolves a tradeoff. A market order prioritizes execution certainty and accepts whatever price the market offers; a limit order prioritizes price control and accepts that the order may not fill. The agent should first identify which risk matters more for the specific trade: if getting filled is essential and the security is liquid, a market order is appropriate; if price discipline matters and the security is volatile or illiquid, a limit order is safer. Choosing without identifying the tradeoff leads to orders that solve the wrong problem.

### Use Market Orders Only In Liquid Securities

Market orders are safe when the security is highly liquid, with a tight bid-ask spread and depth at multiple price levels, so that the fill lands near the quoted price. In illiquid securities, thin book depth means a market order can eat through many price levels and fill far from the quote, especially for larger sizes. The agent should use market orders only where liquidity is deep and should switch to limit orders whenever the spread is wide or the book is thin. The cost of a bad market fill in an illiquid name can exceed the entire intended position's expected edge.

### Use Limit Orders For Price Discipline And Illiquid Names

A limit order specifies the worst acceptable price and prevents slippage beyond it, at the cost of possibly not filling. The agent should use limit orders for illiquid securities, for volatile names where price can move between quote and fill, and whenever price discipline matters more than certainty. The tradeoff is non-execution risk: a limit set too aggressively may never fill, leaving the investor without the position. The agent should set the limit with awareness of the current spread and depth, balancing price improvement against fill probability.

### Place Stops To Manage Risk, Not To Trade Noise

Stop orders convert to market orders when a trigger price is hit, automating exits and enforcing risk discipline. They are valuable for predefined exit points and for preventing larger losses. The risk is that a stop placed too close to the current price gets triggered by normal noise and fills poorly, especially when the stop becomes a market order in a fast move and fills far below the trigger. The agent should place stops outside the normal noise range, should consider stop-limit orders where a bad fill is unacceptable, and should recognize that stops do not guarantee the exit price in gaps.

### Understand Stop-Limit Orders And Their Non-Execution Risk

A stop-limit order triggers a limit order rather than a market order when the stop price is hit, protecting against a terrible fill in a fast market. The protection comes at a cost: if the price blows through both the stop and the limit, the order never fills, and the position is not exited. The agent should use stop-limit orders when avoiding a catastrophic fill matters more than guaranteeing the exit, and should set the limit close enough to the stop to fill in normal conditions but far enough to survive a fast move. The tradeoff is price protection versus execution certainty.

### Account For Gaps And Overnight Risk

Stops and limits do not protect against gaps, where the price jumps past the trigger without trading at it, such as overnight or over a halt. A stop on a position held overnight can trigger and fill far below the trigger at the next open. The agent should recognize that order types only work within continuous trading and that gaps can defeat them, and should size overnight positions and use other risk tools, such as position size and diversification, to manage gap risk that order types cannot.

### Consider Time-In-Force And Extended Hours

Order behavior depends on time-in-force: day orders expire at the close, good-til-canceled orders persist, and immediate-or-cancel orders fill what they can and cancel the rest. Extended-hours trading carries wider spreads, thinner liquidity, and different behavior than regular hours. The agent should match the time-in-force to the intent and should be cautious with market orders in extended hours, where thin books can produce very poor fills. The settings matter as much as the order type.

## Common Traps

### Market Orders In Illiquid Or Volatile Names

Thin books produce large slippage. The agent should use limit orders when liquidity is poor.

### Limit Orders Set Too Tight To Fill

Aggressive limits miss the fill entirely. The agent should balance price against fill probability.

### Stops Triggered By Noise

Stops too close to price get hit by normal movement. The agent should place them outside the noise range.

### Stops That Fill Far Below The Trigger In Fast Markets

Market stops offer no price protection in gaps. The agent should consider stop-limits where a bad fill is unacceptable.

### Stop-Limits That Never Fill

Tight stop-limits protect price but risk no exit. The agent should set the limit to balance protection and execution.

### Ignoring Overnight Gap Risk

Order types do not work across gaps. The agent should manage gap risk with position size.

### Market Orders In Extended Hours

Thin after-hours books produce poor fills. The agent should use limits outside regular hours.

## Self-Check

- [ ] The order type was chosen by identifying whether execution certainty or price control matters more for the specific trade.
- [ ] Market orders are used only in liquid securities with tight spreads and deep books.
- [ ] Limit orders are used for illiquid or volatile names, with the limit set to balance price improvement against fill probability.
- [ ] Stops are placed outside the normal noise range to avoid noise triggers.
- [ ] Stop-limit orders are used where a catastrophic fill must be avoided, with the non-execution risk acknowledged.
- [ ] Overnight and gap risk is recognized as something order types cannot fully protect against.
- [ ] Time-in-force and extended-hours behavior are matched to the trade intent.
- [ ] No market order is recommended for an illiquid or extended-hours situation without flagging the slippage risk.
- [ ] The order size is checked against the security's liquidity and book depth.
- [ ] Order mechanics are treated as a real risk management decision, not a formality.