FROM ubuntu:20.04
WORKDIR /opt/app
CMD ["target/release/server"]
