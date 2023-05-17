use std::net::SocketAddr;

use axum::response::Json;
use axum::routing::get;
use axum::Router;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = get_app();

    // Run our app!
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("able to bind to host and port");
}

/// build our application with a route
fn get_app() -> Router {
    Router::new().route("/", get(hello_world))
}

// So what is `#[derive(Debug, Serialize)]`?
//
// Both `Debug` and `Serialize` are traits. We didn't discuss traits in our talk,
// but they're awsome! Traits are rust's version of interfaces in Golang. Traits
// define a set of methods and types can implement a trait by implementing their methods.
// What's nice is we don't always need to manual implement traits for our types.
// `Debug` is a trait provided by the standard library and The compliler can generte
// a `Debug` implementation for us. The serde crate conveniently provides a macro that
// implement `Serialize` for us too!
//
// Just like that we're now able to debug print out `HelloWordMessage` struct, and we're
// able to serialize the data in our struct.
#[derive(Debug, Serialize)]
struct HelloWorldMessage {
    message: String,
}

/// Returns a Hello World message to the user!
/// You can read more about handlers [here](https://docs.rs/axum/latest/axum/handler/index.html)
async fn hello_world() -> Json<HelloWorldMessage> {
    let message = "Hello World!";
    Json(HelloWorldMessage {
        message: message.to_owned(),
    })
}
