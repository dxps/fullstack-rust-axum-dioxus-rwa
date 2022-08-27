# The Back-End Side

This is the server side of the project.

state: `work-in-progress`

<br/>

## Setup

### PostgreSQL Database

Install the `sqlx-cli` using `cargo install --version=0.6.1 sqlx-cli --no-default-features --features native-tls,postgres`.

Run `./ops/init_db.sh` script that:
- starts a PostgreSQL instance as a Docker container
- runs the database migrations within.

If the database container is already running, you can skip the container bootstrap using `SKIP_DOCKER=true ./ops/init_db.sh`. Additional details regarding database migrations are documented [ops](./ops/readme.md).

<br/>

## Start

Use `./run_dev.sh` to run the server in _dev mode_ (recompile and restart on code changes).

The server supports a couple of options. Run `cargo run --bin server -- -h` to get the details.

<br/>

## Use

For a quick health check, access `http://localhost:8080/api/healthcheck`.

_more-to-come_

<br/>

## Build

Use `./build.sh` to build the release version of the server.
