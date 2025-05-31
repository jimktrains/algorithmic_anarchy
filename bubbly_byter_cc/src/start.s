.section .init, "ax"
.global _start
_start:
    .option push
    .option norelax
    .option pop
    jal zero, main
_end:
    ecall
    .end
