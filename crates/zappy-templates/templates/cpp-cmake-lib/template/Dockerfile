FROM archlinux:latest

RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm --needed \
        base-devel \
        gcc \
        cmake \
        ninja \
        lcov \
        git \
        gtest \
    && pacman -Scc --noconfirm

CMD ["bash"]
