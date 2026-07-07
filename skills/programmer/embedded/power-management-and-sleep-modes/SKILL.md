---
name: power_management_and_sleep_modes.md
description: Use when the agent is developing firmware for battery-powered or energy-harvesting microcontrollers, IoT sensor nodes, wearables, remote trackers, or low-power embedded systems; selecting between sleep, stop, standby, and shutdown modes; configuring wake sources and RTC wakeup; gating peripheral clocks; trading off wake latency against current draw; handling brown-out detection (BOD) and low-power resets; estimating battery life from current profiles; measuring actual current draw with an ammeter or coulomb counter; or diagnosing a device that drains its battery far faster than the datasheet sleep current suggests. Covers low-power-mode selection, peripheral-left-on failure modes, and the gap between datasheet claims and measured current.
---

# Power Management And Sleep Modes

Low power is not a mode you enter; it is a budget you spend. A battery-powered microcontroller datasheet advertises sleep currents in the nanoamp-to-microamp range, and agents trained on always-on systems treat that number as the device's actual consumption. It is not. The datasheet figure is the silicon alone, in one specific mode, with every peripheral clocked off, every GPIO in a defined state, no debugging attached, and the brown-out detector disabled or in its low-power variant. In a real firmware image, the current is the sum of the MCU sleep current plus every peripheral left clocked, every floating input, every pull resistor fighting the external circuit, every regulator quiescent current, and every wake event that brings the device briefly back to full-power run. The dominant battery killer in shipped devices is almost never the chosen sleep mode being wrong — it is one peripheral, one I/O, or one subsystem that was never turned off, quietly drawing orders of magnitude more than the entire rest of the design.

The judgment problem is deciding, for each operating state, what the *measured* current draw is (not the datasheet claim), what wake sources and latency each sleep mode offers, and which tradeoffs between low power and responsiveness fit the device's actual duty cycle. The harm of casual decisions is a device that meets its battery-life target in the lab on a bench supply and dies in weeks in the field — or a fleet that passes certification on a typo'd current number.

## Core Rules

### Measure Actual Current; Never Trust The Datasheet Number Alone

The datasheet sleep-current figure is a best-case silicon measurement, not a prediction of your board. The real current is dominated by what is attached to the MCU: the regulator's quiescent draw, the always-on sensor, the LED you forgot, the RS-485 transceiver left enabled, the crystal oscillator kept running, the floating input with its internal pull-up. Measure the device's current in each state with a real instrument — an ammeter for steady states, a coulomb counter or source-measure unit for the fast wake/run/sleep transient, and an oscilloscope across a small shunt resistor to capture the brief full-power bursts that an averaging meter hides. The wake/run/sleep transient is where most energy goes in a duty-cycled device: a device that sleeps at 2 µA but wakes for 10 ms at 8 mA every second spends roughly 80 µA average from the wake bursts alone — far more than the sleep current. Build an average-current profile from the measured duty cycle, then derive battery life from that average, not from the sleep-mode number.

### Choose The Sleep Mode Against Wake Latency And Retention, Not Just The Current Number

MCUs offer a ladder of low-power modes, and the tradeoff is current versus what you keep and how fast you wake:

- **Sleep.** CPU halted, clocks to peripherals usually still running, full RAM retained, wake on any interrupt in microseconds. Lowest savings, fastest wake. Use when the device must respond quickly and the duty cycle is high.
- **Stop / deep sleep.** Most clocks stopped, RAM retained, wake sources limited (RTC, specific GPIO, comparator). Microamp range, wake in tens to hundreds of microseconds. The workhorse for intermittent-sensing devices.
- **Standby / low-power stop.** Main regulator off, RAM partially or fully retained via a low-leakage domain, core logic powered down, wake from RTC or a few pins. Sub-microamp, but wake is milliseconds and you may need to reinitialize clocks and peripherals.
- **Shutdown.** Almost everything off, RAM lost, wake only from a handful of pins or a reset. Nanoamp range, but wake is a full reset/reboot and state must be restored from backup registers or flash.

The trap is picking shutdown for the headline current number and then discovering the device takes 200 ms to reboot on every wake, or that it lost the state it needed. Match the mode to the required wake latency and to what state must survive.

### Gate Every Peripheral Clock And Define Every Pin State Before Sleep

Entering a sleep mode does not automatically turn off the peripherals you left enabled. A clock-gating failure is the single most common "device drains battery" defect: the ADC is left enabled, the UART receiver is left clocked, an I2C peripheral holds the bus, a timer keeps running, a GPIO is left driving a load. Before entering any low-power mode, walk a checklist: disable every peripheral clock that has no wake job, put every GPIO into a defined low-leakage state (input with pull matching the external circuit, or analog input, never a floating input that toggles and burns current), turn off the voltage reference and oscillators that are not needed, and switch the system clock to a low-power oscillator if the wake source needs one. The pattern is to explicitly reinitialize peripherals on wake, rather than assuming they survived unchanged across a deep mode. An I/O pin left floating, or a push-pull output fighting an external pull, can draw more current than the entire MCU in shutdown.

### Treat Wake Sources And Brown-Out Detection As Part Of The Power Budget

Wake sources are not free. An RTC running in the backup domain draws current; a comparator left armed draws current; a low-power UART receiver draws current; a watchdog running in standby draws current. Each wake source you keep armed is a line item in the sleep-current budget, and the combination often exceeds the bare-MCU figure by an order of magnitude. Decide explicitly which wake sources are needed in each state and disable the rest. Brown-out detection (BOD) is the same tradeoff: the continuous BOD protects against supply dips but draws current; most MCUs offer a low-power or sampling BOD for sleep. Disabling BOD entirely in sleep saves current but risks a brown-out corrupting RAM or flash writes without a clean reset — a real hazard on a sagging battery. Choose the BOD mode against the risk of silent corruption, not just the current number.

### Estimate Battery Life From The Measured Average, Including Self-Discharge And Derating

Battery life is average current divided into capacity, with corrections the naive calculation omits. Use the measured average current (from the duty-cycle profile), not the sleep-mode number. Then apply: capacity derating for the discharge rate (a coin cell's rated capacity assumes a low drain; pulsing it at high current for the wake bursts yields far less usable capacity and can sag the voltage below the BOD threshold), temperature derating (capacity and self-discharge both worsen in the cold or heat), self-discharge (a primary coin cell self-discharges over years and can dominate the budget for a very-low-current device), and end-of-life voltage margin (the battery must still be above the MCU's minimum and the BOD threshold when nearly depleted). A life estimate built only from sleep current and nominal capacity is optimistic by a factor that can be 2x-10x in a pulsed design. Measure, profile, and derate.

## Common Traps

### Believing The Datasheet Sleep-Current Number Is The Device's Current

Quoting the MCU's 0.5 µA shutdown figure as the device's consumption, when the regulator, the always-on sensor, the RTC, and a floating pin add tens of microamps. Measure the whole board in each state; the datasheet number is the floor, not the ceiling.

### A Peripheral, Clock, Or GPIO Left Active Into Sleep

Entering stop mode with the ADC, a timer, or a UART still clocked, or leaving an unused input floating (it toggles and draws shoot-through current) or a push-pull output fighting an external pull resistor. Any one of these can draw more current than the entire MCU in shutdown. Walk the peripheral-gating and GPIO-state checklist before every sleep entry; floating inputs and enabled peripheral clocks are classic silent drains.

### Optimizing Sleep Current And Ignoring The Wake Burst

Spending days shaving the sleep current from 2 µA to 1 µA while ignoring that the device wakes for 20 ms at 10 mA every second — the wake burst dominates average current by 200x. Profile the full duty cycle; the wake/run transient is usually where the energy goes.

### Picking Shutdown For The Headline Number, Losing Latency And State

Choosing shutdown to claim nanoamps, then finding the device takes hundreds of milliseconds to reboot on every wake and loses the state it needed. Match the mode to required wake latency and retention, not to the best datasheet figure.

### Disabling Brown-Out Detection To Save Current And Corrupting State

Turning off the BOD in sleep to shave microamps, then a sagging battery browns out below the safe voltage during a flash write or while RAM is retained, corrupting data with no clean reset. Use a low-power BOD variant; do not disable protection where silent corruption is possible.

## Self-Check

- [ ] The device's current was measured (not assumed from the datasheet) in each operating state — run, sleep, deep sleep, standby — using an ammeter, coulomb counter, or shunt-and-scope that captures the wake/run/sleep transient.
- [ ] The sleep mode was chosen against wake-latency requirements and what state (RAM, clocks, peripheral config) must survive, not against the headline current number alone.
- [ ] A pre-sleep checklist is enforced that gates every unused peripheral clock, disables unneeded oscillators/references, and puts every GPIO into a defined low-leakage state (no floating inputs, no outputs fighting external pulls).
- [ ] Wake sources are explicitly selected per state; unneeded wake sources (RTC, comparator, watchdog, UART RX) are disabled, and their current is counted in the sleep budget.
- [ ] Brown-out detection is configured in a low-power mode appropriate to the battery behavior; it is not disabled where a brown-out could corrupt RAM or flash writes silently.
- [ ] Battery life is estimated from the measured average current across the full duty cycle (including wake bursts), with derating for discharge rate, temperature, self-discharge, and end-of-life voltage margin — not from sleep current and nominal capacity alone.
- [ ] A long-duration soak test (or a coulomb-counter log over many wake/sleep cycles) confirmed the measured average current matches the estimate, and no peripheral was found left on.
- [ ] GPIO and peripheral configuration is explicitly reinitialized on wake from deep modes; no code assumes peripherals survived unchanged across a mode that powers them down.
