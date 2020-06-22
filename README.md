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
