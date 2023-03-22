FROM ubuntu as builder
RUN apt update -y && apt dist-upgrade -y && apt install -y gcc curl wget clang default-jre python3.9
RUN curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain stable -y
WORKDIR /usr/src/revant
COPY . .
RUN source $HOME/.cargo/env && cargo build --release

FROM ubuntu
RUN apt update -y
COPY --from=builder /usr/src/revant/target/release/revant /usr/local/bin/revant
CMD ["revant"]
