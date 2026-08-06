# 1. basic image: ubuntu:latest
FROM ubuntu:latest

# 2. set up time zone & locale & update apt
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    tzdata ca-certificates && \
    ln -fs /usr/share/zoneinfo/Etc/UTC /etc/localtime && \
    dpkg-reconfigure --frontend noninteractive tzdata

# 3. install Rust, Python3, git, curl
RUN apt-get install -y --no-install-recommends \
    build-essential \
    python3 \
    cmake \
    pkg-config \
    libssl-dev \
    openssl \
    libfontconfig1-dev \
    # python3-pip \
    # python3-venv \
    git \
    curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 4. add Rust in PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# 5. TypePulse
COPY . /home/TypePulse

# 6. set workspace: TypePulse
WORKDIR /home/TypePulse

# 7. set up typepulse-home as cache dir
RUN python3 setup_typepulse_home.py /home/TypePulse/typepulse-home

# 8. install specific version of Rust: nightly-2023-06-02
RUN rustup install nightly-2023-06-02 && \
    rustup default nightly-2023-06-02 && \
    rustup component add rust-src rustc-dev llvm-tools-preview miri

# 9. set up environment variables
ENV TYPEPULSE_RUST_CHANNEL=nightly-2023-06-02
ENV TYPEPULSE_RUNNER_HOME="/home/TypePulse/typepulse-home"
ENV RUSTFLAGS="-L $HOME/.rustup/toolchains/nightly-2023-06-02-x86_64-unknown-linux-gnu/lib"
ENV LD_LIBRARY_PATH="${LD_LIBRARY_PATH}:$HOME/.rustup/toolchains/nightly-2023-06-02-x86_64-unknown-linux-gnu/lib"
ENV RUSTC_SYSROOT="$HOME/.rustup/toolchains/nightly-2023-06-02-x86_64-unknown-linux-gnu/bin/rustc"

# 10. install typepulse into cargo
RUN ./install.sh

# 11. default command
CMD ["/bin/bash"]
