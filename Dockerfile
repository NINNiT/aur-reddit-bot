FROM archlinux:latest as builder

WORKDIR /app

RUN pacman -Syu --noconfirm

RUN pacman -S rustup gcc pkgconf --noconfirm

RUN rustup toolchain install stable

RUN rustup default stable

COPY . .

RUN cargo build --release


FROM archlinux:latest as runner

COPY --from=builder /app/target/release/aur-reddit-bot /usr/bin/aur-reddit-bot

ENV RUST_LOG DEBUG

CMD ["/usr/bin/aur-reddit-bot"]
