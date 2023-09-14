# Fibonacci sequence calculator
# This program is a simple demo of RISC-V assembly to run on Z2L
# It calculates Fibonacci numbers and stores them in RAM at 0x80000000

.align 4

.section .text
.globl _start

_start:
    li x1, 1
    li x2, 1
    li x5, 0x80000000
loop:
    add x3, x1, x2
    sw x3, 0(x5)
    mv x1, x2
    mv x2, x3
    j loop
