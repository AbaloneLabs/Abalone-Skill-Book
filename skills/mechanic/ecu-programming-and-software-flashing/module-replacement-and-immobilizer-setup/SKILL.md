---
name: module-replacement-and-immobilizer-setup.md
description: Use when the agent is replacing a control module, programming a new or used ECU, performing initialization and setup after a module swap, matching VIN and configuration data, handling immobilizer and component protection, or deciding whether a used module can be installed and what setup it requires.
---

# Module Replacement and Immobilizer Setup

Replacing a control module is no longer a bolt-in operation; every modern ECU, body module, transmission controller, and immobilizer is married to the vehicle by software, and a new or used module requires programming, configuration, and security setup before the vehicle will start or function. The judgment problem is that the wrong choice of module (new versus used, wrong part number, wrong hardware or software level), a skipped configuration step, or an incomplete immobilizer or component-protection setup leaves the vehicle dead or partially functional, and the consequences range from a no-start to a vehicle that runs but has disabled safety systems. A technician who installs a used module without checking its compatibility, or who skips the configuration and setup, hands back a vehicle that does not work. This skill covers the disciplined selection, programming, and setup of a replacement module, including immobilizer and component-protection handling.

## Core Rules

### Verify Module Compatibility, Part Number, and Hardware/Software Level Before Ordering

The disciplined module replacement starts by verifying the exact part number, hardware version, and software level of the module being replaced, because a module that fits physically may be the wrong configuration for the vehicle's options and market. The disciplined technician reads the part number from the original module's label and from the scan tool (the OEM part number and the hardware and software versions), checks the vehicle's option code list (the PR code on the data sticker, the build sheet, or the OEM parts catalog) to confirm the correct replacement, and verifies whether a used module is permitted (some manufacturers allow used modules for body functions but prohibit them for immobilizer or security modules, and some require the used module to be from a vehicle with the same options). The tradeoff is that this verification takes time, but installing an incompatible module bricks the vehicle or requires a return.

### Read and Save the Original Module's Configuration Before Removal

Before the original module is removed, the disciplined technician connects the scan tool and reads and saves the module's configuration data, coding, and adaptation values, because this data is needed to set up the replacement and because some data (the immobilizer data, the VIN, the learned component values, the calibration) cannot be recovered from a dead module. The technician saves the configuration to the scan tool or to a file, notes the coding and the adaptations, and, where the OEM tool allows, performs a backup of the module's data. This is especially critical for modules that store learned values (a transmission controller's adaptives, a body module's learned component positions) and for the immobilizer or key data. The tradeoff is that the backup takes minutes, but losing the configuration of a dead module can make the replacement setup impossible without OEM support.

### Perform the OEM Programming and Configuration Sequence in Full

A replacement module requires the OEM's full programming and configuration sequence, and skipping or shortcutting a step leaves the module partially functional. The disciplined technician performs the sequence in order: the VIN write (to marry the module to the vehicle), the programming or flashing (to install the correct software for the vehicle's options and market), the configuration or coding (to set the module's options to match the vehicle, such as the engine, the transmission, the market, the installed features), and the adaptations and initialization (to teach the module the vehicle's components, such as the throttle body, the gear positions, or the ride height sensors). The technician verifies each step completes successfully and clears the resulting fault codes before moving to the next. The tradeoff is that the full sequence can take an hour or more, but a skipped step leaves the vehicle with disabled functions or a no-start.

### Handle Immobilizer, Component Protection, and Security Setup Correctly

Many modules (the engine ECU, the immobilizer, the instrument cluster, the body controller, the key, the transmission controller on some vehicles) are protected by an immobilizer or a component-protection system that prevents the vehicle from starting if a module is swapped without the correct security setup. The disciplined technician performs the immobilizer or component-protection adaptation after the module is installed and programmed, which may require the vehicle's PIN or security code, the OEM scan tool, and an online connection to the manufacturer's server (for component protection that requires online release). The technician verifies the immobilizer recognizes the keys and the new module, and that the vehicle starts and runs. The tradeoff is that the security setup may require online access and the security codes, but skipping it leaves the vehicle with a no-start or a start-stall condition.

### Verify All Functions, Clear Codes, and Perform the Required Initialization and Learn

After the module is programmed and configured, the disciplined technician verifies every function the module controls, clears the fault codes (including the codes set by the programming process), and performs any required initialization and learn procedures (a throttle body learn, a transmission adaptive learn, a ride height calibration, a steering angle sensor calibration). The technician road-tests or exercises every function to confirm the module is fully operational, and checks for any new or remaining fault codes that indicate an incomplete setup or a compatibility issue. The tradeoff is that this verification takes time, but a module returned without full verification may have disabled safety systems or drivability faults that the customer discovers.

## Common Traps

### Installing a Used Module Without Checking Compatibility and Option Codes — A used module is installed to save cost, and the vehicle has disabled functions or faults because the used module is from a vehicle with different options or a different market. The trap mechanism is that modules are option- and market-specific, and a used module may fit physically but have the wrong configuration. The false signal is the connector fitting; the harm is a module that does not work. The disciplined technician verifies the part number and the option codes.

### Skipping the Configuration and Coding Step After Programming — The module is programmed with the correct software, but the configuration (the option coding) is skipped, and the vehicle has disabled functions because the module does not know the vehicle's options. The trap mechanism is that programming installs the software but configuration tells the module what the vehicle has, and configuration is a separate step. The false signal is the programming "completing"; the harm is disabled functions. The disciplined technician performs the full sequence including configuration.

### Failing to Perform the Immobilizer or Component Protection Adaptation — The engine ECU or immobilizer is replaced, the module is programmed, but the security adaptation is skipped, and the vehicle starts and stalls or does not start. The trap mechanism is that the immobilizer does not recognize the new module without the adaptation, and the vehicle is immobilized. The false signal is the module being programmed; the harm is a no-start. The disciplined technician performs the immobilizer adaptation with the PIN and the OEM tool.

### Not Saving the Original Configuration Before Removing a Dead Module — The original module is dead and removed, the technician installs the replacement, and the setup is impossible because the original's configuration and learned values were not saved. The trap mechanism is that some data cannot be recovered from a dead module, and the setup requires the original values. The false signal is the replacement being on hand; the harm is a setup that cannot be completed. The disciplined technician reads and saves the configuration before removal, where the module still communicates.

### Returning the Vehicle Without Verifying All Functions and Performing the Learn Procedures — The module is installed and programmed, the vehicle starts, and the technician returns it without verifying all functions and performing the learn procedures, and the customer finds a disabled function or a drivability fault. The trap mechanism is that the module needs initialization and learn to be fully operational, and the verification is skipped. The false signal is the vehicle starting; the harm is a comeback. The disciplined technician verifies all functions and performs the learn procedures.

## Self-Check

- Did I verify the replacement module's part number, hardware, and software level against the original and the vehicle's option codes?
- Did I read and save the original module's configuration, coding, and adaptations before removal?
- Did I perform the full OEM programming and configuration sequence (VIN write, programming, coding, adaptations)?
- Did I perform the immobilizer or component-protection adaptation with the correct PIN and the OEM tool?
- Did I clear all fault codes, including those set by the programming process?
- Did I perform all required initialization and learn procedures (throttle, transmission, ride height, steering angle)?
- Did I verify every function the module controls and road-test or exercise the functions?
- Did I document the module part numbers, the programming and configuration steps, and the security setup on the repair order?
