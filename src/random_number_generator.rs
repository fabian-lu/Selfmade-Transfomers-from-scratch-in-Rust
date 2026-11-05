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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_COUNT: usize = 100_000;

    // mean and variance in f64, so that summing many numbers doesn't lose precision
    fn mean_and_variance(values: &[f32]) -> (f64, f64) {
        let count = values.len() as f64;
        let mean = values.iter().map(|value| *value as f64).sum::<f64>() / count;
        let variance = values
            .iter()
            .map(|value| (*value as f64 - mean).powi(2))
            .sum::<f64>()
            / count;
        (mean, variance)
    }

    #[test]
    fn same_seed_gives_same_numbers() {
        let mut first_generator = RandomNumberGenerator::new(42);
        let mut second_generator = RandomNumberGenerator::new(42);

        assert_eq!(
            first_generator.get_random_numbers(10),
            second_generator.get_random_numbers(10)
        );
    }

    #[test]
    fn different_seeds_give_different_numbers() {
        let mut first_generator = RandomNumberGenerator::new(1);
        let mut second_generator = RandomNumberGenerator::new(2);

        assert_ne!(
            first_generator.get_random_numbers(10),
            second_generator.get_random_numbers(10)
        );
    }

    #[test]
    fn seed_zero_does_not_get_stuck_at_zero() {
        let mut generator = RandomNumberGenerator::new(0);

        let numbers = generator.get_random_numbers(10);

        assert!(numbers.iter().all(|number| *number != 0));
    }

    #[test]
    fn get_random_numbers_returns_the_requested_count() {
        let mut generator = RandomNumberGenerator::new(3);

        assert_eq!(generator.get_random_numbers(25).len(), 25);
    }

    #[test]
    fn floats_zero_to_one_stay_between_zero_and_one() {
        let mut generator = RandomNumberGenerator::new(5);

        for _ in 0..SAMPLE_COUNT {
            let value = generator.random_float_zero_to_one();
            assert!(
                (0.0..1.0).contains(&value),
                "{} is not between 0 and 1",
                value
            );
        }
    }

    #[test]
    fn standard_normal_has_mean_zero_and_variance_one() {
        let mut generator = RandomNumberGenerator::new(11);

        let values: Vec<f32> = (0..SAMPLE_COUNT)
            .map(|_| generator.random_float_standard_normal())
            .collect();
        let (mean, variance) = mean_and_variance(&values);

        assert!(mean.abs() < 0.02, "mean is {}", mean);
        assert!((variance - 1.0).abs() < 0.02, "variance is {}", variance);
    }

    #[test]
    fn normal_floats_with_spread_have_that_spread() {
        let mut generator = RandomNumberGenerator::new(13);

        let values = generator.get_random_normal_floats(SAMPLE_COUNT, 0.02);
        let (mean, variance) = mean_and_variance(&values);

        assert_eq!(values.len(), SAMPLE_COUNT);
        assert!(mean.abs() < 0.001, "mean is {}", mean);
        assert!(
            (variance.sqrt() - 0.02).abs() < 0.001,
            "spread is {}",
            variance.sqrt()
        );
    }
}
