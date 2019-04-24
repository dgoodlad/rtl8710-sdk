# rtl8710-sdk

Work-in-progress Rust wrapper for the RTL8710 single-chip wifi module [standard
SDK](https://www.amebaiot.com/en/ameba-sdk-intro/).

The SDK is `v4.0b`, slimmed down to only the parts required to compile. The
build is, for now, based on
`project/realtek_ameba1_va0_example/GCC-RELEASE/application.mk` from the SDK,
implemented as a call out to the `cc` crate in `build.rs`.

## Prerequisites

Requires rust nightly and a couple of `rustup` components:

``` sh
# Install the nightly toolchain, required for some experimental features
rustup override set nightly

# Install the rust target for the ARM Cortex-M3 microcontroller
rustup target add thumbv7m-none-eabi

# Install some required components for the build
rustup component add llvm-tools-preview
rustup component add rust-src
rustup component add rustfmt
```

## Usage

This is very much WIP, so not easy to describe. The gist looks like this:

1. Make a new binary crate for your project
2. Add `rtl8710-sdk` as a dependency
3. Configure cargo to link properly in `.cargo/config`:

    [target.thumbv7m-none-eabi]
    runner = 'gdb-multiarch'
    rustflags = [
      "-C", "linker=arm-none-eabi-ld",
      "-C", "link-arg=-Tlink.x",
      "-C", "link-arg=-nostartfiles",
    ]
    
    [build]
    target = "thumbv7m-none-eabi"

You'll need a linker script (`link.x`) derived from the
`rlx8195A-symbol-v02-img2.ld` script in the SDK. That's up to you for now.

The resulting binary should be set to load via a debugger (e.g. `gdb` with
`openocd`). Nothing here prepares it to flash to the device, though!

## License

This work is license under the MIT license. The original SDK software included
in the `vendor` directory is provided under its respective license(s).
