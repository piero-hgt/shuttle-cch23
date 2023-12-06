mod day_1;
mod day01;
mod day04;
mod day06;

use rocket::routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount(
        "/",
        routes![
            crate::day_1::hello_world,
            crate::day_1::minus_one_error,
            crate::day01::cube_the_bits,
            crate::day04::reindeer_cheer,
            crate::day04::eating_candy_contest,
            crate::day06::elf_count,
        ]
    );

    Ok(rocket.into())
}
