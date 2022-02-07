BOOTLOADER := bootloader/rustsbi-qemu.bin
KERNEL_ELF := target/riscv64gc-unknown-none-elf/release/os 
KERNEL_BIN := target/riscv64gc-unknown-none-elf/release/os.bin
KERNEL_ENTRY_PA := 0x80200000
kernel:
	@cargo build --release
	rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)
clean:
	@cargo clean
run: kernel
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)