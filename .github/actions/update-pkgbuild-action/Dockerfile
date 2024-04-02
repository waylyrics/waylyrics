FROM archlinux:base

RUN pacman-key --init && \
    pacman-key --populate archlinux && \
    pacman -Syu --noconfirm && \
    pacman -S pacman-contrib --noconfirm && \
    useradd builder -M -s /bin/bash -U

COPY --chmod=755 main.sh /main.sh

ENTRYPOINT [ "/main.sh" ]
