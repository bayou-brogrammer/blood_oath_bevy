use bracket_random::prelude::RandomNumberGenerator;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock() = RandomNumberGenerator::seeded(seed);
}

pub fn roll_dice(n: i32, die_type: i32) -> i32 {
    RNG.lock().roll_dice(n, die_type)
}

pub fn range(min: u32, max: u32) -> u32 {
    RNG.lock().range(min, max)
}

pub fn roll() -> f32 {
    RNG.lock().rand()
}
