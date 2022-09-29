use bracket_random::{prelude::RandomNumberGenerator, rand};
use parking_lot::Mutex;

pub struct RandomNumbers {
    rng: Mutex<RandomNumberGenerator>,
}

impl Default for RandomNumbers {
    fn default() -> Self {
        Self { rng: Mutex::new(RandomNumberGenerator::new()) }
    }
}

#[allow(dead_code)]
impl RandomNumbers {
    pub fn new() -> Self {
        Self { rng: Mutex::new(RandomNumberGenerator::new()) }
    }

    pub fn seeded(seed: u64) -> Self {
        Self { rng: Mutex::new(RandomNumberGenerator::seeded(seed)) }
    }

    /// Returns a random value in the specified range, of type specified at the call site.
    /// This is INCLUSIVE of the first parameter, and EXCLUSIVE of the second.
    /// So range(1,6) will give you numbers from 1 to 5.
    pub fn range<T>(&self, min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.rng.lock().range(min, max)
    }

    /// Rolls dice, using the classic 3d6 type of format: n is the number of dice, die_type is the size of the dice.
    pub fn roll_dice(&self, n: i32, die_type: i32) -> i32 {
        self.rng.lock().roll_dice(n, die_type)
    }
}
