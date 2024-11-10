FROM ubuntu:latest

WORKDIR /

RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker
RUN echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker

ENV DEBIAN DEBIAN_FRONTEND=noninteractive

RUN apt-get update

RUN apt-get -y install \
    git \
    ca-certificates \
    make \
    zlib1g-dev libssl-dev gperf php-cli cmake \
    clang-18 libc++-18-dev libc++abi-18-dev libclang-rt-18-dev


RUN mkdir /project && chmod 777 /project

WORKDIR /project/source/td

# CMD rm -rf build && \
CMD mkdir build && \
    cd build && \
    CXXFLAGS="-stdlib=libc++" \
    CC=/usr/bin/clang-18 \
    CXX=/usr/bin/clang++-18 \
    cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX:PATH=../bin .. && \
    cmake --build . --target install -j $(nproc)
