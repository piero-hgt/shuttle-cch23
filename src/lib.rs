use std::path::PathBuf;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Deer {
    pub name: String,
    #[serde(default)]
    pub strength: u8,
    #[serde(default)]
    pub speed: f32,
    #[serde(default)]
    pub height: u8,
    #[serde(default)]
    pub antler_width: u8,
    #[serde(default)]
    pub snow_magic_power: u32,
    #[serde(default)]
    pub favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    #[serde(default)]
    pub candies_eaten_yesterday: u8, // cAnD13s_3ATeN-yesT3rdAy
}

pub struct DeerCollection<'a> {
    pub deers: Vec<&'a Deer>
}

impl<'a> DeerCollection<'a> {
    pub fn new(deers: Vec<&'a Deer>) -> Self {
        DeerCollection { deers }
    }

    pub fn strength(&self) -> u8 {
        self.deers.iter().map(|x| x.strength).sum()
    }

    pub fn fastest(&self) -> &Deer {
        self.deers
            .iter()
            .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
            .unwrap()
    }

    pub fn tallest(&self) -> &Deer {
        self.deers
            .iter()
            .max_by(|a, b| a.height.partial_cmp(&b.height).unwrap())
            .unwrap()
    }

    pub fn magician(&self) -> &Deer {
        self.deers
            .iter()
            .max_by(|a, b| a.snow_magic_power.partial_cmp(&b.snow_magic_power).unwrap())
            .unwrap()
    }

    pub fn consumer(&self) -> &Deer {
        self.deers
            .iter()
            .max_by(|a, b| a.candies_eaten_yesterday.partial_cmp(&b.candies_eaten_yesterday).unwrap())
            .unwrap()
    }
}


#[derive(Serialize)]
pub struct DeerContestOutput {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

impl DeerContestOutput {
    pub fn from_deer_collection(collection: DeerCollection) -> Self {
        let fastest = collection.fastest();
        let tallest = collection.tallest();
        let magician = collection.magician();
        let consumer = collection.consumer();

        // {
        //     "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
        //     "tallest": "Dasher is standing tall with his 36 cm wide antlers",
        //     "magician": "Dasher could blast you away with a snow magic power of 9001",
        //     "consumer": "Dancer ate lots of candies, but also some grass"
        // }
        DeerContestOutput {
            fastest: format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
            tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
            magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
            consumer: format!("{} ate lots of candies, but also some grass", consumer.name),
        }
    }
}
