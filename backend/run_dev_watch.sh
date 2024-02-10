#!/bin/sh

## Running and restarting (recompile and run) on code changes.
##
## To use `watchexec`, you can install it using:
## cargo install cargo-binstall
## cargo binstall watchexec-cli

watchexec --exts rs --restart -- cargo run --bin server -- --port 9091 --log debug

