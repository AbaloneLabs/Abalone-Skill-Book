---
name: peripheral_and_driver_design.md
description: Use when the agent is writing or reviewing a device driver, peripheral driver, HAL, or BSP for a microcontroller; abstracting memory-mapped registers and bit fields; handling SPI, I2C, UART, CAN, QSPI, or 1-Wire timing-sensitive protocols; integrating DMA into a driver; designing blocking versus non-blocking (asynchronous/callback) driver APIs; handling bus faults, NACKs, timeouts, and stuck peripherals; working around silicon errata; making a driver portable across MCU families or silicon revisions; or diagnosing a driver that works on one board and fails on another. Covers register abstraction, timing and DMA correctness, errata workarounds, and the failure mode of drivers that pass on one silicon revision and break on the next.
---

# Peripheral And Driver Design

A peripheral driver is the layer where software meets the physical timing of silicon, and that meeting is unforgiving. A register read that the datasheet says "returns the current value" can return a stale value if read at the wrong clock phase; an I2C transaction that works at 100 kHz hangs at 400 kHz because a setup-time was violated; a SPI transfer that is correct on one MCU family corrupts data on another because the default clock phase or bit order differs; a driver that passes every test on silicon revision B1 fails on revision B2 because an erratum was fixed and a workaround now breaks it. Agents trained on platform-abstraction-heavy environments routinely write drivers that treat the peripheral as a clean API and the bus as reliable, then discover that real buses NACK, stretch clocks, lose arbitration, get stuck mid-transfer, and that the hardware has errata the datasheet's main text never mentions. The driver is also the layer most likely to be copied between projects and silicon families, so a casual design becomes a latent defect that surfaces months later on hardware the original author never touched.

The judgment problem is deciding how registers are abstracted without hiding timing-critical sequences, how timing-sensitive protocols are handled against their real electrical constraints, how bus faults and stuck peripherals are recovered, how DMA is integrated safely, and how portability and errata are managed so the driver survives contact with the next silicon revision. The harm of casual decisions is a driver that works on the bench and intermittently fails in the field on a different board, a longer bus, a faster clock, or a newer chip.

## Core Rules

### Abstract Registers Without Hiding Timing-Critical Sequences

Memory-mapped registers are accessed through volatile pointers, and the temptation is to wrap every access in a clean-looking function or macro that reads clean in application code. That is fine for simple configuration, but it hides a hazard: many peripherals require specific read-modify-write ordering, a read of a status register to clear a flag, a delay between writes, or a write-then-read sequence where the values observed depend on exact ordering. A `set_bit()` helper that does read-modify-write is correct for one register and racy or wrong for another that requires a single atomic write (many status-clear registers clear on write of `1`, and a read-modify-write can clear flags you did not intend). Abstract at the level of *operations* ("enable clock, configure mode, start conversion") with documented ordering, not at the level of individual bits, and preserve the volatile and ordering semantics the hardware requires. Where a sequence is timing-critical, keep it together and explicit; do not let abstraction fragment it across functions the compiler or a future editor may reorder.

### Respect The Electrical And Timing Reality Of Each Protocol

SPI, I2C, UART, CAN, and 1-Wire are not just byte streams; each has electrical and timing constraints that the driver must honor or it will work on the bench and fail on a longer bus, a faster clock, or a different slave. The specifics that bite:

- **SPI.** Clock polarity and phase (CPOL/CPHA) must match the slave; many devices sample on a specific edge and are silently corrupted on the wrong mode. Chip-select timing matters: some slaves require CS held high for a minimum time between transfers, or a CS toggle to reset their internal state. Bit order (MSB/LSB first) is slave-dependent and not universal.
- **I2C.** Clock stretching (a slave holds SCL low to slow the master) must be tolerated, not treated as a fault. Repeated-start conditions are required by some slaves between a register-address write and a data read and cannot be replaced by a stop-then-start. Arbitration loss and bus collisions happen on a multi-master bus. Addressing (7-bit vs 8-bit, the R/W bit) is a perennial source of off-by-one addressing bugs.
- **UART.** Baud-rate mismatch of even 2-3% causes framing errors; the driver must tolerate or report them. Flow control (hardware CTS/RTS or software XON/XOFF) exists because buffers overflow — ignoring it on a fast producer and slow consumer drops bytes.

The recurring error is treating the bus as a reliable pipe and omitting the protocol's edge-case handling. Know the protocol's constraints for the specific slave, not just its average-case behavior.

### Integrate DMA Safely, With Explicit Buffer Ownership

DMA is the standard way to move data on SPI/I2C/UART without CPU cycles, and in a driver the hazard is *buffer ownership and lifecycle*: which context owns the buffer at each point, and when is it safe to touch again. The core concurrency rules (never share a region between CPU and DMA simultaneously, place buffers in DMA-reachable memory, maintain cache coherency, order data writes before the completion signal with a barrier) are covered in the interrupt-safety skill and apply here. The driver-specific job is to make ownership explicit in the API: the caller hands the buffer to the driver for the duration of the transfer, the driver gives it back only at the completion callback, and the contract documents that the caller must not touch it in between. A driver that returns from a start-transfer call while still implying the caller can reuse the buffer is the source of intermittent "flaky slave" data corruption.

### Design The API Contract Deliberately: Blocking/Non-Blocking, Timeouts, And Fault Recovery

A driver API is either blocking (the call returns when the operation is done) or non-blocking (the call starts the operation and returns immediately, completing via interrupt/callback/poll), and the choice must be deliberate because it constrains every caller. Blocking APIs are simple and correct for low-throughput peripherals and initialization, but they stall the calling task and are fatal in an ISR or a hard-real-time loop. Non-blocking APIs (start-transfer + completion callback, or RTOS semaphore-give-on-done) are required for high-throughput and real-time use but push complexity onto the caller (state machines, reentrancy, callback context). Whichever you choose, the API must also define its failure contract, because real buses and peripherals fail: an I2C slave NACKs, a SPI slave holds the line, a peripheral state machine hangs, a clock-enable bit never sets, a device is physically absent. A driver that assumes the operation always succeeds will hang the system the first time a slave does not respond. Every transaction needs a timeout, every status-poll needs a bounded retry, and every fault needs a defined recovery: return an error, retry, or reset the peripheral (disable and re-enable its clock, reinitialize its registers). A locked-up peripheral often cannot be recovered by software alone — I2C buses famously wedge when a transfer is interrupted mid-byte and the slave holds SDA low; the recovery is to bit-bang clocks until the slave releases. The recurring trap is a "blocking" API that secretly polls a register in a busy loop with no timeout, hanging the system forever if the peripheral sticks; or a "non-blocking" API with no way to report a failure to the callback. Expose the blocking choice explicitly, give every blocking call a timeout, document whether callbacks run in ISR or task context, and make the absence of a device a handled condition (an I2C address that NACKs is a missing sensor, not a crash).

### Manage Errata And Portability As First-Class Concerns

Silicon has errata — documented bugs in the peripheral that the manufacturer admits to in a separate errata sheet, not the main reference manual. A driver written to the reference manual will hit these and fail; a robust driver encodes the known workarounds and is built to survive a silicon revision change. Make portability and errata explicit: parameterize register addresses and bit positions (do not hardcode magic numbers scattered through the code), isolate MCU-specific code behind a port layer so the protocol logic is reusable across families, and track which silicon revision each errata workaround applies to so it can be removed when the erratum is fixed (a workaround applied to a revision where the bug is already fixed can itself break correct behavior). The failure mode of "works on one silicon revision, breaks on the next" is almost always an errata workaround that was never conditional, or an assumption about undocumented behavior that changed.

## Common Traps

### Read-Modify-Write On A Write-To-Clear Register

Using a generic `set_bit()`/read-modify-write helper on a status-clear register that clears flags on write of `1`, accidentally clearing flags that were not the target. Use direct writes for write-to-clear registers; reserve read-modify-write for configuration registers where it is correct.

### Wrong SPI Mode, Bit Order, Or I2C Edge-Case Handling

Configuring SPI with default CPOL/CPHA and MSB-first, then getting corrupted data because the specific slave samples on the other edge or expects LSB-first; or treating I2C as a simple byte stream and failing on slaves that stretch the clock or require a repeated-start between address-write and data-read. Match the mode, bit order, and protocol edge cases to each slave's datasheet; these are not universal.

### Sharing A Buffer With DMA Without An Ownership Contract

Letting the CPU read a buffer DMA just filled (without cache invalidation or a barrier), or reusing a buffer mid-transfer because the API did not make ownership explicit, producing intermittent corrupt data that looks like a flaky slave. Define buffer ownership in the API; double-buffer and maintain cache coherency around every DMA transfer.

### A Blocking API With No Timeout, And Assuming The Device Is Always Present

Polling a "transfer done" flag in a loop with no bound, or calling a sensor's read function and dereferencing the result without checking for a NACK, so a stuck peripheral or a missing/unpowered device hangs or crashes the system. Every blocking call and status poll has a timeout and a defined failure path; treat device-absent and bus-fault as handled conditions, not impossible ones.

### An Errata Workaround That Is Not Conditional On Silicon Revision

Applying an errata workaround unconditionally, so when the erratum is fixed in a new silicon revision the workaround itself breaks the now-correct hardware. Tag every workaround with the revision it applies to and review it when silicon changes.

## Self-Check

- [ ] Register access uses volatile semantics and preserves timing-critical sequences (ordering, read-to-clear, required delays); abstraction is at the operation level, not fragmenting sequences the hardware requires to stay ordered.
- [ ] Write-to-clear (clear-on-write-of-1) registers are written directly, never via read-modify-write; configuration registers use read-modify-write only where correct.
- [ ] SPI mode (CPOL/CPHA), bit order, and chip-select timing match each slave's datasheet; I2C handles clock stretching, repeated-start, and addressing correctly; UART tolerates baud/framing errors and uses flow control where the slave needs it.
- [ ] DMA buffers are double-buffered (CPU and DMA never touch the same region simultaneously), in DMA-reachable memory, with cache invalidation/cleaning and a barrier ensuring data writes are visible before the completion signal.
- [ ] The API's blocking vs non-blocking choice is explicit and documented; every blocking call and status poll has a timeout and a defined failure path; callbacks document whether they run in ISR or task context.
- [ ] Bus faults (NACK, stuck line, absent device, hung state machine) are handled with timeouts, bounded retries, and peripheral-reset recovery paths — not assumed impossible.
- [ ] Errata workarounds are documented and conditional on the silicon revision they apply to; register addresses and bit fields are parameterized behind a port layer rather than hardcoded.
- [ ] The driver was validated on the target hardware at the intended bus speed and bus length, and on the specific silicon revision(s) it will ship on — not just at low speed on a short bench bus.
