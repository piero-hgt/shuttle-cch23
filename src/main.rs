use cch23_piero_hgt::{Sled, Deer, DeerCollection, DeerContestOutput};
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
    let collection = DeerCollection::new(deers.iter().collect());
    collection.strength().to_string()
}

#[post("/4/contest", data="<deers>")]
fn day_4_eating_candy_contest(deers: Json<Vec<Deer>>) -> Json<DeerContestOutput> {
    let collection = DeerCollection::new(deers.iter().collect());
    Json(DeerContestOutput::from_deer_collection(collection))
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
            day_4_eating_candy_contest,
        ]
    );

    Ok(rocket.into())
}
