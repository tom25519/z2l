# Z2L - A RISC-V Emulator

<p align="center"><img src="/assets/demo.gif?raw=true"/></p>

Z2L is a RISC-V emulator, developed as a programming exercise in order to better
understand the ISA. The eventual goal is to be able to emulate a minimal Linux
system.

Please be aware that this is intended as a research project only: Performance is
not of particular importance here, and if it was, a fundamentally different
architecture (likely based on machine code translation to the host platform)
would be required.

## Project Structure
The `z2l-core` crate, stored in the `core` directory, defines the core runtime
for the emulator: The processor, clock, memory, etc.; plus the
`ExecutionEnvironment` struct, which is a single interface linking all of these
emulated hardware components to provide a RISC-V bare-metal EEI.

The `z2l-isa` crate, stored in the `isa` directory, implements the RISC-V base
integer instruction set, plus ratified extensions. These are defined as structs
implementing the `Extension` trait: The `z2l-core` crate uses such extensions to
process instructions.

The `z2l-cli` crate, stored in the `cli` directory, is the main entrypoint for
the emulator, providing a Terminal User Interface for configuring, running, and
interacting with the emulated system.

## Usage
Currently, the emulator only supports running a ROM in RV32I with no extensions.
For instructions on how to do this, see the `examples` directory.

The ROM will be mapped to the address space starting at `0x00000000`. RAM is
accessible from the address space in the region starting at `0x80000000` (by
default 32KiB of RAM are available).

## Roadmap
* [x] Core runtime
* [x] RV32I base instruction set
* [x] TUI
* [ ] Zicsr extension
* [ ] Machine ISA
* [ ] Multiple harts on separate threads
* [ ] Other ratified extensions
* [ ] Supervisor ISA
* [ ] Serial device support
* [ ] Storage device support
* [ ] SBI implementation
* [ ] U-Boot support
* [ ] Linux support
* [ ] Hypervisor ISA
* [ ] RV64I base instruction set
* [ ] More virtualised device support

## License
Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
