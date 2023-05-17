// use the get_app function we defined to get our router
use hello_world_web_server::get_app;

use axum::body::Body;
use axum::http::{Request, StatusCode};

// The ServiceExt needs to be in scope to use oneshot
use tower::ServiceExt;

#[tokio::test]
async fn hello_world() {
    let app = get_app();
    let request = Request::builder().uri("/").body(Body::empty()).unwrap();

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"{\"message\":\"Hello World!\"}");
}

#[tokio::test]
async fn hello_world_with_custom_name() {
    let app = get_app();
    let url = "/?name=Unified";
    let request = Request::builder().uri(url).body(Body::empty()).unwrap();

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"{\"message\":\"Hello Unified!\"}");
}
