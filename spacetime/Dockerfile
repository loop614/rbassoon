FROM clockworklabs/spacetimedb

RUN apt update && apt upgrade && apt install gcc wget git -y
RUN wget --output-document - --quiet https://sh.rustup.rs | sh -s -- -y
ENV PATH="${PATH}:$HOME/.cargo/env"

WORKDIR /clockwork

RUN wget https://github.com/clockworklabs/SpacetimeDB/releases/latest/download/spacetime.Linux-amd64.tar.gz
RUN tar xf spacetime.Linux-amd64.tar.gz

WORKDIR /spacetime
COPY . .

WORKDIR /
