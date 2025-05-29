SHELL = /bin/sh
ARCH = riscv64-unknown-elf
CC = $(ARCH)-gcc
LD = $(ARCH)-ld
OBJCOPY = $(ARCH)-objcopy
OBJDUMP = $(ARCH)-objdump
IDIR = inc
SDIR = src
BDIR = build
CFLAGS = -Wall -g -I $(IDIR) -O0  -nolibc -nodefaultlibs -nostdlib -nostartfiles
SFLAGS = -g -I $(IDIR)
S_SRCS = $(wildcard $(SDIR)/*.s)
C_SRCS = $(wildcard $(SDIR)/*.c)
S_OBJS = $(S_SRCS:$(SDIR)/%.s=$(BDIR)/%_asm.o)
C_OBJS = $(C_SRCS:$(SDIR)/%.c=$(BDIR)/%.o)

all: clean $(BDIR)/kernel.img $(BDIR)/kernel.s

$(BDIR)/kernel.img: $(BDIR)/kernel.elf
	$(OBJCOPY) $< -O binary $@

kernel.elf: $(S_OBJS) link.ld $(C_OBJS)
	$(LD) -T link.ld -o $@ $(S_OBJS) $(C_OBJS)

$(BDIR)/kernel.s: $(BDIR)/kernel.elf
	$(OBJDUMP) --syms $< | sort > $@
	$(OBJDUMP) -dS $< >> $@

$(BDIR)/%.o: $(SDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(BDIR)/%.s: $(SDIR)/%.c
	$(CC) $(CFLAGS) -S $< -o $@

$(BDIR)/%_asm.o: $(SDIR)/%.s
	$(CC) $(SFLAGS) -c $< -o $@

clean:
	rm -f $(BDIR)/*

run: all

