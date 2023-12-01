use cch23_piero_hgt::Sled;
use rocket::{
    get,
    routes,
    http::Status
};
use std::path::PathBuf;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn minus_one_error() -> Status {
    Status::InternalServerError
}

#[get("/1/<path..>")]
fn cube_the_bits(path: PathBuf) -> String {
    let sled: Sled = Sled::create_from_pathbuf(path);
    format!("{}", sled.xor_cube())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/",
        routes![
            hello_world,
            minus_one_error,
            cube_the_bits
        ]
    );

    Ok(rocket.into())
}
