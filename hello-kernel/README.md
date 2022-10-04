Simple Kernel modules based on the [Rust Sample](https://github.com/torvalds/linux/blob/master/samples/rust/rust_minimal.rs).

Save the rust module under `~/linux/samples/rust/`

Add the Kconfig script to `~/linux/samples/rust/Kconfig`

Add `echo obj-$(HELLO_KERNEL) += hello_kernel.o` to `~/linux/samples/rust/Makefile`

Run `make menuconfig` and toggle under `Kernel hacking` your module.

Now you can finally build your kernel:\
`make LLVM=1 -j$(nproc)`