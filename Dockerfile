FROM rust:alpine

WORKDIR /workspace
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.bfsu.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache git musl-dev perl make

WORKDIR /workspace/quantum-agent
COPY . .
RUN cargo build --release && mv target/release/quantum-agent /bin/quantum-agent \
    && cargo clean && rm -rf /usr/local/cargo \
    && rm -rf /usr/local/rustup

ENTRYPOINT [ "/bin/quantum-agent" ]