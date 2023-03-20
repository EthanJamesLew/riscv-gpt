.option norvc
.section .data

.section .text.init
.global _start
_start:
    la		sp, 0x2000
    li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    la		t1, main
