use std::path::PathBuf;

use rocket::{get, routes, http::Status};

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
    let numbers: Vec<u32> = match path.iter().map(|x| x.to_str().unwrap().parse::<u32>()).collect() {
        Ok(numbers) => numbers,
        Err(_) => return String::from("Unable to parse path to u32"),
    };

    if numbers.len() == 0 {
        return String::from("Missing arguments (at east one required)");
    }
    if numbers.len() > 20 {
        return String::from("Too many arguments (max 20)");
    }
    let mut sled: u32 = 0;
    for number in numbers {
        sled = sled ^ number;
    }

    format!("{}", sled.pow(3))
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
