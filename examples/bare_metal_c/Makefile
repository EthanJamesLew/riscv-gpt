CC=riscv64-unknown-elf-gcc
AS=riscv64-unknown-elf-as
CFLAGS=-mabi=lp64d -march=rv64imafd -nostdlib
LINKER=riscv64-unknown-elf-ld

SOURCES := $(wildcard *.c)
SOURCES += $(wildcard *.s)
OBJS := $(patsubst %.c, %.o, $(SOURCES))
OBJS := $(patsubst %.s, %.o, $(OBJS))

EXEC := fibc.elf

all: $(EXEC) 

$(EXEC): $(OBJS) 
	$(LINKER) -o $(EXEC) $(OBJS) 

%.o: %.s
	$(AS) -c $< -o $@ 

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@ 

clean:
	rm *.o
	rm $(EXEC)
