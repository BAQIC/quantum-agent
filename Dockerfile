FROM rust:alpine

WORKDIR /workspace
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.bfsu.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache git musl-dev
RUN git clone https://github.com/BAQIC/quantum-agent.git

WORKDIR /workspace/quantum-agent

ENTRYPOINT [ "cargo", "run" ]