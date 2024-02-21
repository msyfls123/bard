FROM ubuntu:22.04
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y libsqlite3-dev
WORKDIR /opt/app
CMD ["target/release/server"]
