# sema-api
Main API for the Sema project


## Local Development

This project uses [auto reloading](https://actix.rs/docs/autoreload/) in dev.

You must have `cargo-watch` installed on your system

    cargo install cargo-watch


To recompile and rerun on source or config changes

    cargo watch -x run --clear --no-gitignore

### Dealing with a "error: EADDRINUSE: Address already in use"

https://stackoverflow.com/questions/3855127/find-and-kill-process-locking-port-3000-on-mac

NOTE: shut down the web client and the caddy server BEFORE running the kill command below.

```
$ sudo lsof -i :8088
// or
$ sudo lsof -i tcp:8088
// the kill
$ kill -9 <pid>
```