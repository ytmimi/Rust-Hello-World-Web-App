use axum::extract::{Json, Query};
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};

/// Build our application with a router
/// Note: `get_app` is marked as `pub` so that it's "public" and accessible from outside of our
/// library. Typically you'll want to define code in a `library`. `lib.rs` is typically the entry
/// point into your library, while `main.rs` is the entry point into your `compiled binary`.
/// By defining most of our application logic in a library our `main.rs` file becomes relatively
/// simple, and writing integration tests to test your libraries public interface becomes easier.
pub fn get_app() -> Router {
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

#[derive(Deserialize)]
#[repr(transparent)]
struct HelloWorldParams {
    name: Option<String>,
}

/// Returns a Hello World message to the user!
/// You can read more about handlers [here](https://docs.rs/axum/latest/axum/handler/index.html)
async fn hello_world(Query(params): Query<HelloWorldParams>) -> Json<HelloWorldMessage> {
    let message = if let Some(name) = params.name {
        format!("Hello {}!", name)
    } else {
        "Hello World!".to_string()
    };

    Json(HelloWorldMessage {
        message: message.to_owned(),
    })
}
