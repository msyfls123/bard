FROM ubuntu:20.04
COPY Rocket.toml /opt/app/
COPY ./server/templates /opt/app/server/templates
COPY ./static /opt/app/static
COPY ./target/release /opt/app/target/release
WORKDIR /opt/app
RUN chmod +x ./target/release/server
CMD ["target/release/server"]
