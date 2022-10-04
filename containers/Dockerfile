FROM fedora:latest
RUN dnf install -y clang flex bison llvm lld bc \
	&& dnf groupinstall -y "Development Tools" "Development Libraries" \
	&& curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y \ 
	&& echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
