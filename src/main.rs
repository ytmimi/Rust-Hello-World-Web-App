use hello_world_web_server::get_app;
use std::net::SocketAddr;

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
