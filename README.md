# GDB Bug Demonstration
This repository demonstrates a bug within GDB when parsing rust source code processed by RTIC.

When attempting to set breakpoints within the main.rs file, all breakpoints will be set to the
HardFault handler. Attempting to single step (using next) in GDB will never step to the next line as
expected.

A comment in the source code demonstrates what is causing the apparent parsing failure.

# Bug Explanation
# Compilation

This example was developed for the STM32H743V processor using the ST-Link v3 debugger. It is unknown
whether or not this bug will reproduce on other platforms.

To compile, first install the thumb7em-none-eabihf target:
```
rustup target add thumbv7em-none-eabi
```

Then compile using cargo:
```
cargo build
```

# Running Example

After starting openOCD (Note: OpenOCD version must be very recent for ST-LINKv3) using the
following:
```
openocd -f stabilizer.cfg
```

The example can be run using `cargo run`.
