# Hello World Web Server

This repo provides an example web server written using rust's [axum](https://github.com/tokio-rs/axum) web framework.

## Running the App

This project assumes you've installed the rust toolchain from [rustup.rs](https://rustup.rs/). If you download the latest version you should be able to run this project without any issue.

To start the server run:

```
# Build the debug binary and run it
cargo run
```

Alternatively, you can build the binary and then run the executable

```
# Build the debug binary (gets stored in ./target/debug)
cargo build
# run the binary directly
./target/debug/hello_world_web_server
```

You should see a log message similar to this:

```
2023-05-17T04:48:37.074292Z  INFO hello_world_web_server: listening on 127.0.0.1:7878
```

In a seperate terminal use `curl` to make calls to the web app:

```
curl -G http://localhost:7878
curl -G http://localhost:7878 -d "name=Yacin"
```

You should see the following log messages

```
2023-05-17T04:48:42.283979Z  INFO hello_world_web_server: No name passed to hello_world handler!
2023-05-17T04:48:47.826873Z  INFO hello_world_web_server: Yacin passed to hello_world handler!
```

## Running tests

Some simple integration tests have been written to validate the `hello_world` request handler. You can run them with:

```
cargo test
```
