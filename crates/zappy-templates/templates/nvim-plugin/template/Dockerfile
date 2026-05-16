from archlinux:latest

RUN pacman -Syuu --noconfirm && \
    pacman -Sy --noconfirm --needed \
        git \
        base-devel \
        lua \
        luarocks \
        rustup \
        npm \
    && pacman -Scc --noconfirm && \
    curl --location --output /usr/bin/release-cli "https://gitlab.com/api/v4/projects/gitlab-org%2Frelease-cli/packages/generic/release-cli/latest/release-cli-linux-amd64" && \
    chmod +x /usr/bin/release-cli && \
    rustup default stable && \
    rustup update && \
    touch /root/.bashrc && \
    cargo install just && \
    echo "export PATH=\"$PATH:$HOME/.cargo/bin\"" | tee -a "$HOME/.bashrc"

RUN luarocks install argparse &&\
    luarocks install luacheck && \
    npm install -g markdown-toc && \
    cargo install stylua

WORKDIR /app
