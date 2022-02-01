# bard
To build a flexible SNS platform, use graph to connect messages, and also people.

## Tech Stack

|Component|Name|
|---|---|
|Server|[Rocket](https://rocket.rs/)|
|Client UI|[Yew](https://yew.rs)|
|Database|[IndraDB](https://github.com/indradb/indradb)|
|ORM|[Juniper](https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/examples/rocket_server.rs)|


## Deploy

```shell
# This command should be executed on Linux or Docker
cargo build server --release
./script/build_client.sh

docker build -t bard:1 .
docker-compose up
```