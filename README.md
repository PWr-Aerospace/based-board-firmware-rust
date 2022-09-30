# Based Board Firmware In Rust

This is a working blinky example for the BasedBoard project.
Its fully written in Rust and uses typical embedded components
from the Rust embedded world. In order to compile it you just
need to have rust installed and then run `cargo build` in the repository.
It will prompt you to install needed targets for cross compilation.
You can also flash it by running `cargo embed` (install cargo-embed first).

## Info

As of now this repository and its configuration is a proof of concept.
'
Both debugging using VSCode (openocd and the cortex-debug extension under the hood) and cmd openocd and remote gdb.

Command line:
First open a connection to the board:

```bash
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
```

Then in another terminal connect to it (the gdb debugger might have a different name on your OS like `gdb` or `gdb-embedded` etc):

```bash
arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/based-board-firmware-rust
```
