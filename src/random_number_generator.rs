const WARM_UP_ROUNDS: usize = 10;

pub struct RandomNumberGenerator {
    state: u64,
}

impl RandomNumberGenerator {
    pub fn new(seed: u64) -> Self {
        let mut rng = Self {
            state: if seed == 0 { 1 } else { seed },
        };
        for _ in 0..WARM_UP_ROUNDS {
            rng.next_random_number();
        }

        rng
    }

    // xorshift64 algorithm for pseudo random numbers (Marsaglia, 2003)
    pub fn next_random_number(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;

        self.state
    }

    pub fn get_random_numbers(&mut self, count: usize) -> Vec<u64> {
        (0..count).map(|_| self.next_random_number()).collect()
    }

    pub fn random_float_zero_to_one(&mut self) -> f32 {
        let mut random_number = self.next_random_number();
        random_number = random_number >> 40;
        let mut random_float = random_number as f32;
        random_float = random_float / 16777216.0;

        random_float
    }

    // Box-Muller transform: turns two floats between 0 and 1 into one normally
    // distributed float with mean 0 and standard deviation 1
    pub fn random_float_standard_normal(&mut self) -> f32 {
        // 1 - u lies in (0, 1], so the logarithm below never sees 0
        let first_uniform = 1.0 - self.random_float_zero_to_one();
        let second_uniform = self.random_float_zero_to_one();

        let radius = (-2.0 * first_uniform.ln()).sqrt();
        let angle = 2.0 * std::f32::consts::PI * second_uniform;

        radius * angle.cos()
    }

    pub fn get_random_normal_floats(&mut self, count: usize, spread: f32) -> Vec<f32> {
        (0..count)
            .map(|_| self.random_float_standard_normal() * spread)
            .collect()
    }
}
