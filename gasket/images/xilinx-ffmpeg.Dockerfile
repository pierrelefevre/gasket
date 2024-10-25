# Docker image for building the xilinx video-sdk fork of FFmpeg with librist and libsrt support

FROM pierrelefevreneti/xilinx-sdk:latest

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update -qq
RUN apt-get -y install \
    autoconf \
    automake \
    build-essential \
    cmake \
    git-core \
    libass-dev \
    libfreetype6-dev \
    libgnutls28-dev \
    libmp3lame-dev \
    libsdl2-dev \
    libtool \
    libva-dev \
    libvdpau-dev \
    libvorbis-dev \
    libxcb1-dev \
    libxcb-shm0-dev \
    libxcb-xfixes0-dev \
    meson \
    ninja-build \
    pkg-config \
    texinfo \
    wget \
    yasm \
    zlib1g-dev \
    libunistring-dev \
    libaom-dev \
    libssl-dev \
    libx264-dev \
    libx265-dev \
    libnuma-dev \
    libdav1d-dev \
    libsrt1.4-openssl \
    libsrt-openssl-dev \
    libfreetype-dev \
    libfontconfig-dev

# Create a non root user
RUN useradd -ms /bin/bash ubuntu
USER ubuntu
WORKDIR /home/ubuntu
RUN mkdir -p ffmpeg_sources ffmpeg_build/lib


# Build libsrt
# https://stackoverflow.com/questions/50967706/how-to-compile-ffmpeg-with-enabling-libsrt/50975754#50975754
WORKDIR /home/ubuntu/ffmpeg_sources
RUN git clone --depth 1 https://github.com/Haivision/srt.git
WORKDIR /home/ubuntu/ffmpeg_sources/srt
RUN mkdir build
WORKDIR /home/ubuntu/ffmpeg_sources/srt/build
RUN cmake -DCMAKE_INSTALL_PREFIX="$HOME/ffmpeg_build" -DENABLE_C_DEPS=ON -DENABLE_SHARED=OFF -DENABLE_STATIC=ON ..
RUN make
RUN make install

# Build librist
# https://code.videolan.org/rist/librist/-/blob/v0.2.10/README.md?ref_type=tags#compile-using-mesonninja-linux-osx-and-windows-mingw
WORKDIR /home/ubuntu/ffmpeg_sources
RUN git clone https://code.videolan.org/rist/librist.git -b v0.2.10
WORKDIR /home/ubuntu/ffmpeg_sources/librist
RUN mkdir /home/ubuntu/ffmpeg_sources/librist/build
WORKDIR /home/ubuntu/ffmpeg_sources/librist/build
RUN meson .. --default-library=static
RUN ninja

# Clone the n4.4 version of FFmpeg and the Xilinx Video SDK
WORKDIR /home/ubuntu/ffmpeg_sources
RUN git clone https://github.com/FFmpeg/FFmpeg.git -b n4.4
RUN git clone https://github.com/Xilinx/video-sdk.git -b v3.0
WORKDIR /home/ubuntu/ffmpeg_sources/FFmpeg

# Apply the patch
# https://xilinx.github.io/video-sdk/v3.0/using_ffmpeg.html#using-the-git-patch-file
RUN git config --global user.email "ffmpeg@gasket.pierrelf.com" \
    && git config --global user.name "gasket"
RUN cp ~/ffmpeg_sources/video-sdk/sources/app-ffmpeg4-xma-patch/0001-Updates-to-ffmpeg-n4.4-to-support-Alveo-U30-SDK-v3.patch .  
RUN git am 0001-Updates-to-ffmpeg-n4.4-to-support-Alveo-U30-SDK-v3.patch --ignore-whitespace --ignore-space-change

# Check current user
RUN whoami
RUN echo $HOME
RUN ls ~/ffmpeg_sources
RUN ls ~/ffmpeg_build/lib

# Configure FFmpeg with necessary flags, including librist and libsrt, and the Xilinx Video SDK plugins
ENV PATH="$HOME/bin:$PATH" 
ENV PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig:/ffmpeg_sources/srt/build:/ffmpeg_sources/librist/build/meson-private"

RUN ./configure \
    --prefix="$HOME/ffmpeg_build" \
    --pkg-config-flags="--static" \
    --extra-cflags="-I$HOME/ffmpeg_build/include" \
    --extra-ldflags="-L$HOME/ffmpeg_build/lib" \
    --extra-libs="-lpthread -lm" \
    --ld="g++" \
    --bindir="$HOME/bin" \
    # --datadir=/ffmpeg_build/etc  \
    --enable-x86asm \
    --enable-libxma2api \
    --disable-doc \
    --enable-gpl \
    --enable-libx264 \
    --enable-libx265  \
    --enable-libdav1d \
    # --enable-librist \
    --enable-libsrt \
    --enable-libfontconfig \
    --enable-libfreetype \
    --enable-libxvbm \
    --enable-libxrm \
    --extra-cflags=-I/opt/xilinx/xrt/include/xma2 \
    --extra-ldflags=-L/opt/xilinx/xrt/lib \
    --extra-libs=-lxma2api \
    --extra-libs=-lxrt_core \
    --extra-libs=-lxrt_coreutil \
    --extra-libs=-lpthread \
    --extra-libs=-ldl \
    --enable-static

# Build and install FFmpeg
RUN PATH="$HOME/bin:$PATH" make && \
    make install

# Set the final working directory and the default command
WORKDIR /home/ubuntu/ffmpeg_build
RUN ls -la

CMD ["/bin/bash"]