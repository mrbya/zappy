FROM archlinux:latest

RUN pacman -Syyuu gcc git llvm clang rustup npm pkgconf mingw-w64-gcc --noconfirm --needed && \
    rustup default stable && \
    rustup update && \
    touch /root/.bashrc && \
    cargo install just && \
    pacman -Scc && \
    echo "export PATH=\"$PATH:$HOME/.cargo/bin\"" | tee -a "$HOME/.bashrc"

ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN rustup install nightly && \
    cargo install cargo-binstall && \
    cargo binstall cargo-nextest --no-confirm && \
    rustup component add llvm-tools-preview && \
    cargo binstall cargo-llvm-cov --no-confirm && \
    cargo binstall cargo-criterion --no-confirm && \
    cargo binstall cargo-udeps --no-confirm && \
    cargo install cargo-audit --locked --features=fix
    npm install -g markdown-toc

CMD ["bash"]
