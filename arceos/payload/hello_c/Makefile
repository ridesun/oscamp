TARGET := hello

CC := riscv64-linux-gnu-gcc
STRIP := riscv64-linux-gnu-strip

all: $(TARGET)

%: %.c
	$(CC) -static $< -o $@
	$(STRIP) $@

clean:
	@rm -rf ./$(TARGET)
