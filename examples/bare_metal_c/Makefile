CC=riscv64-unknown-elf-gcc
ASSEMBLER=riscv64-unknown-elf-as
CFLAGS=-mabi=lp64d -march=rv64imafd -nostdlib
LINKER=riscv64-unknown-elf-ld


all: fibc

fibc: start.o main.o
	$(LINKER) -o fibc start.o main.o

start.o: start.s
	$(ASSEMBLER) -o start.o start.s

main.o: main.c
	$(CC) $(CFLAGS) -c main.c

clean:
	rm fibc *.o
