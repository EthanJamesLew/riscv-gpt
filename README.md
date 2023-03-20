# RISC-V Emulator by ChatGPT

Playing with prompting to build a RISC-V emulator.

## Assembly Example

```shell
riscv64-unknown-elf-as -o fibonacci.o fibonacci.s
riscv64-unknown-elf-ld -o fibonacci fibonacci.o
```

## Bare Metal C Example

```shell
riscv64-unknown-elf-gcc -mabi=lp64d -march=rv64imafd -nostdlib -c main.c
riscv64-unknown-elf-ld -o fibc main.o
```