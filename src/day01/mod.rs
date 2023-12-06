use rocket::get;
use std::path::PathBuf;

struct Sled {
    packets: Vec<i32>
}

impl Sled {
    pub fn from(path: PathBuf) -> Self {
        let numbers: Vec<i32> = match path.iter().map(|x| x.to_str().unwrap().parse::<i32>()).collect() {
            Ok(numbers) => numbers,
            Err(_) => return Sled { packets: Vec::new() },
        };

        Sled { packets: numbers }
    }

    pub fn xor_cube(&self) -> i32 {
        let mut sled: i32 = 0;
        for number in &self.packets {
            sled = sled ^ number;
        }

        sled.pow(3)
    }
}

#[get("/1/<path..>")]
pub fn cube_the_bits(path: PathBuf) -> String {
    let sled: Sled = Sled::from(path);
    format!("{}", sled.xor_cube())
}
