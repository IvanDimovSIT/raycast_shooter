use std::{
    collections::HashMap,
    error::Error,
    fs::{read, File},
    io::Write,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};

use crate::constants::COMPLETION_TIMES_PATH;

#[derive(Serialize, Deserialize, Debug)]
struct BestCompletionTimes {
    times: HashMap<u32, Duration>,
}
impl BestCompletionTimes {
    fn read_file() -> Result<Self, Box<dyn Error>> {
        let data = read(COMPLETION_TIMES_PATH)?;
        let times: BestCompletionTimes = from_slice(&data)?;
        Ok(times)
    }

    fn load() -> Self {
        Self::read_file().unwrap_or_else(|_| {
            let times = BestCompletionTimes {
                times: HashMap::new(),
            };
            times.save();
            times
        })
    }

    fn save(&self) {
        let json = to_string(self);
        if json.is_err() {
            println!("Error serializing completion times: {}", json.unwrap_err());
            return;
        }

        let file = File::create(COMPLETION_TIMES_PATH);
        if file.is_err() {
            println!("Error creating file: {}", file.unwrap_err());
            return;
        }

        let write_result = file.unwrap().write_all(json.unwrap().as_bytes());
        if write_result.is_err() {
            println!(
                "Error saving completion times: {}",
                write_result.unwrap_err()
            );
        }
    }
}

pub fn load_best_for_level(level: u32) -> Duration {
    let file = BestCompletionTimes::load();

    if file.times.contains_key(&level) {
        file.times[&level]
    } else {
        Duration::MAX
    }
}

pub fn save_best_for_level(level: u32, time: &Duration) {
    let mut file = BestCompletionTimes::load();

    match file.times.contains_key(&level) {
        true => {
            if file.times[&level] > *time {
                *file.times.get_mut(&level).unwrap() = *time;
            }
        }
        false => {
            file.times.insert(level, *time);
        }
    }

    file.save();
}
