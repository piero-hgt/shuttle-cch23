use rocket::{get, routes, http::Status};

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn minus_one_error() -> Status {
    Status::InternalServerError
}

#[get("/1/<num1>/<num2>")]
fn cube_the_bits(num1: u32, num2: u32) -> String {
    format!("{}", (num1 ^ num2).pow(3))
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
