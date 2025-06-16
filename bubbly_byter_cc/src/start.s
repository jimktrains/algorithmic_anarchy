.section .init, "ax"
.global _start
_start:
    jal main
_end:
    ecall
    .end
