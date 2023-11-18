REM Running and restarting (recompile and run) on code changes.
REM It is using `cargo-watch`, and if you don't have it installed,
REM use `cargo install cargo-watch` to have it.

cargo watch -d 2 -w src -x "run --bin server -- --port 9091 --log debug"

