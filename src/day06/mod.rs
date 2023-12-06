use rocket::{
    post,
    serde::json::Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ElfCount {
    #[serde(rename(serialize = "elf"))]
    count: u32,
    #[serde(rename(serialize = "elf on a shelf"))]
    on_shelf: u32,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    empty_shelf: u32,
}

impl ElfCount {
    pub fn new(count: u32, on_shelf: u32, empty_shelf: u32) -> Self {
        ElfCount {
            count,
            on_shelf,
            empty_shelf
        }
    }
}

#[post("/6", format = "text/plain", data = "<text>")]
pub fn elf_count(text: String) -> Json<ElfCount> {
    let count = text.matches("elf").count();
    let on_shelf = text.matches("elf on a shelf").count();
    let empty_shelf = text.matches("shelf").count() - on_shelf;
    // println!("{} {} {}", count, on_shelf, empty_shelf);

    Json(ElfCount::new(count as u32, on_shelf as u32, empty_shelf as u32))
}
