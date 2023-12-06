use cch23_piero_hgt::{
    Sled,
    Deer,
    DeerCollection,
    DeerContestOutput,
    ElfCount,
};
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

#[post("/6", format = "text/plain", data = "<text>")]
fn day_6_elf(text: String) -> Json<ElfCount> {
    let count = text.matches("elf").count();
    let on_shelf = text.matches("elf on a shelf").count();
    let empty_shelf = text.matches("shelf").count() - on_shelf;
    // println!("{} {} {}", count, on_shelf, empty_shelf);

    Json(ElfCount::new(count as u32, on_shelf as u32, empty_shelf as u32))
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
            day_6_elf,
        ]
    );

    Ok(rocket.into())
}
