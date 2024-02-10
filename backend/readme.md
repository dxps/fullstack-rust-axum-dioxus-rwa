# The Back-End Side

This is the server side of the project.

<br/>

## Setup

### PostgreSQL Database

This section describes how to start the database as a container and initialize it with the required database objects.

Install the `sqlx-cli` using `cargo install --version=0.7.3 sqlx-cli --no-default-features --features native-tls,postgres`.

#### On Linux

Run `./ops/init_db.sh` script that:

-   starts a PostgreSQL instance as a Docker container
-   runs the database migrations within.

If the database container is already running, you can skip the container bootstrap using `SKIP_DOCKER=true ./ops/init_db.sh`.
Additional details regarding database migrations are documented in [ops](./ops/readme.md).

#### On Windows

While being in `backend` directory, you can manually run:

```
set DATABASE_URL=postgres://fs_rs_rwa:fs_rs_rwa@localhost:5441/fs_rs_rwa
sqlx database create
sqlx migrate run
```

The output of `sqlx migrate run` should look like this:

```
Applied 20220823172547/migrate create accounts table (28.919ms)
Applied 20220905163834/migrate create followings table (15.0584ms)
Applied 20220912195826/migrate create articles table (28.3739ms)
Applied 20220912203610/migrate create favorited articles table (14.9627ms)
Applied 20221106211345/migrate create tags tables (13.8194ms)
```

<br/>

## Start

Use `./run_dev_watch.sh` (that uses `watchexec` and has the benefit of properly sending the termination signal to the server) or `./run_dev.sh` (that uses `cargo watch`) to run the server in _dev mode_ (recompile and restart on code changes). As prerequisites, see the details inside these files.

The server supports a couple of command line options. Run `cargo run --bin server -- -h` to get the details.

<br/>

## Use

For a quick health check, access `http://localhost:8080/api/healthcheck`.

_more-to-come_

<br/>

## Build

Use `./build.sh` to build the release version of the server.
