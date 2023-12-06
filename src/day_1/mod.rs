use rocket::{
    get,
    http::Status,
};

#[get("/")]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
pub fn minus_one_error() -> Status {
    Status::InternalServerError
}
