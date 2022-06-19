FROM rust:latest

# Install cargo packages
RUN git clone https://github.com/rustyscreeps/cargo-screeps.git
WORKDIR /cargo-screeps
RUN git checkout arena
RUN cargo install --path .

RUN cargo install -f wasm-bindgen-cli

# We need to install clang in order to compile the rust code without errors.
WORKDIR /
RUN apt-get update && apt-get install -y wget build-essential pkg-config libssl-dev binaryen
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update
RUN apt-get install -y clang

# These volumes are going to hold our Screeps arena code as well as the deployed directories that the game will read. Both of them will need to be attached in read/write mode to the following volumes
VOLUME [ "/code", "/screepsarena" ]

WORKDIR /code
CMD cargo screeps deploy -m ctf
