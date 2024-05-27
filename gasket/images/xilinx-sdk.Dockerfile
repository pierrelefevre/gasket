# Docker image for running on Xilinx U30 https://xilinx.github.io/video-sdk/v3.0/container_setup.html
FROM ubuntu:22.04
ARG DEBIAN_FRONTEND="noninteractive"

SHELL ["/bin/bash", "-c"]
RUN echo 'deb [trusted=yes] https://packages.xilinx.com/artifactory/debian-packages jammy main' > /etc/apt/sources.list.d/xilinx.list
RUN echo 'Acquire { https::Verify-Peer false }' > /etc/apt/apt.conf.d/99verify-peer.conf
RUN apt-get update
RUN apt-get -y install git apt-utils sudo xrt=2.11.722 xilinx-alveo-u30-core xilinx-alveo-u30-ffmpeg xilinx-alveo-u30-examples && \
    apt-mark hold xrt
RUN git clone https://github.com/gdraheim/docker-systemctl-replacement.git /usr/local/share/docker-systemctl-replacement
RUN echo "alias systemctl='python3 /usr/local/share/docker-systemctl-replacement/files/docker/systemctl3.py'" >> /root/.bashrc
RUN ln -s /usr/local/share/docker-systemctl-replacement/files/docker/systemctl3.py /usr/local/bin/systemctl
# These two lines are needed only for cent, rhl or al2
# RUN mv /usr/bin/systemctl /usr/bin/systemctl.fac
# RUN ln -s /usr/local/bin/systemctl /usr/bin/systemctl

RUN export PATH=/usr/local/bin/:$PATH