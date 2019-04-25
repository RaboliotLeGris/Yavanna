FROM debian:latest

RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install cargo libgtk-3-dev && \
    adduser --disabled-password --gecos '' rabo && \
    mkdir -p /home/rabo/yavanna && \
    chown -R rabo:rabo /home/rabo

COPY . /home/rabo/yavanna