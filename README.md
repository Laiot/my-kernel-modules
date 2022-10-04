# my-kernel-modules
In this repo I will store my experiments writing Kernel modules in Rust, based on the [Linux Kernel](https://github.com/torvalds/linux).

Please read the [Quick Start](https://github.com/torvalds/linux/blob/master/Documentation/rust/quick-start.rst) document before getting started with kernel development in Rust.

I work on Fedora so you'll probably need to be careful about the dependencies if you're using another distribution.

## Requirements
Let's start from cloning the Kernel: \
`git clone https://github.com/torvalds/linux.git`

And copy the configuration file: \
`cp .github/workflows/kernel-x86_64-debug.config .config`

For compiling the Kernel you'll need the following packages: \
`dnf groupinstall -y "Development Tools" "Development Libraries"` \
`dnf install -y clang flex bison llvm lld bc`

and of course Rust: \
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y`

For testing and debugging the kernel you'll need: \
`dnf install -y qemu gdb`

Again, please read the Quick Start guide because it'll give you more insights about the components required for building the kernel. \
To check if your system already has everything required, move to the kernel root folder and run: \
`make LLVM=1 rustavailable` \
If you get `RUST_IS_AVAILABLE` as output, you're ready to build the kernel.

To do so, run: \
`make LLVM=1 -j$(nproc)`

If no errors arise (ahah you wish), you should finally get something on the line of `Kernel: arch/x86/boot/bzImage is ready  (#1)`.

TODO add qemu tutorial