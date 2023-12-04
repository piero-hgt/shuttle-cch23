use cch23_piero_hgt::{Sled, Deer};
use rocket::{
    get,
    post,
    routes,
    http::Status,
    serde::json::Json,
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

#[post("/4/strength", data="<deers>")]
fn day_4_reindeer_cheer(deers: Json<Vec<Deer>>) -> String {
    let mut strength: u32 = 0;

    for deer in deers.iter() {
        strength += deer.strength;
    }
    strength.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/",
        routes![
            hello_world,
            minus_one_error,
            cube_the_bits,
            day_4_reindeer_cheer,
        ]
    );

    Ok(rocket.into())
}
