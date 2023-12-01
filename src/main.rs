use axum::{routing::get, Router, http::StatusCode};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn minus_one_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(minus_one_error));

    Ok(router.into())
}
