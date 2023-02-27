FROM rust:latest 
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get upgrade -y
RUN apt install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross libopencv-dev clang libclang-dev cmake

RUN rustup target add armv7-unknown-linux-gnueabihf 
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf 

WORKDIR /app

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ 
CMD ["cargo", "build", "--target", "armv7-unknown-linux-gnueabihf"]

# sudo docker buildx build . -t crust-armv7
# sudo docker run --rm -v '/home/dapa/code/camloc-client/test':/app crust-armv7

# FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf
# ARG DEBIAN_FRONTEND=noninteractive

# RUN apt-get update && apt-get upgrade -y
# RUN apt install -y  libopencv-dev clang libclang-dev cmake

# WORKDIR /app

# ENV CARGO_NET_GIT_FETCH_WITH_CLI=true 
# CMD ["xargo", "build", "--target", "armv7-unknown-linux-gnueabihf"]
