To build this example, you'll need the GCC RISC-V toolchain installed. On
Debian-based distros, you can install it like so:

```shell
sudo apt install gcc-riscv64-unknown-elf
```

Then assemble the example program like so, from the 

```shell
riscv64-unknown-elf-gcc \
    -march=rv32i \
    -mabi=ilp32 \
    -static \
    -mcmodel=medany \
    -fvisibility=hidden \
    -nostdlib \
    -nostartfiles \
    -Texamples/z2l.ld \
    examples/fib.S -o examples/fib.elf
```

Convert the ELF file to a raw binary suitable for use as a ROM like so:

```shell
riscv64-unknown-elf-objcopy -O binary examples/fib.elf examples/fib.bin
```

Then, run the emulator with this ROM:

```shell
cargo run --release --package z2l-cli -- run-quick examples/fib.bin
```