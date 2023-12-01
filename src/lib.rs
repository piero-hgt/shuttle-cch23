use std::path::PathBuf;

pub struct Sled {
    packets: Vec<i32>
}

impl Sled {
    pub fn create_from_pathbuf(path: PathBuf) -> Self {
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
