OUTPUT_ARCH("riscv")
OUTPUT_FORMAT("elf32-littleriscv")

ENTRY( _start )
SECTIONS {
    . = 0x00000000;
    .text : { *(.text) }
    .gnu_build_id : { *(.note.gnu.build-id) }

    . = 0x80000000;
    .data : { *(.data) }
    .sdata : { *(.sdata) }
    .debug : { *(.debug) }

    _end = .;
}
