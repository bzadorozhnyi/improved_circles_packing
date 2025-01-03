use rand::{rngs::StdRng, SeedableRng};

use crate::utils::FloatType;

use super::HeuristicAlgorithm;

pub struct HeuristicAlgorithmBuilder {
    iterations: usize,
    rng: StdRng,
    radiuses: Vec<FloatType>,
    max_small_circle_radius: FloatType,
    delta: FloatType,
}

impl Default for HeuristicAlgorithmBuilder {
    fn default() -> Self {
        Self {
            iterations: Default::default(),
            rng: StdRng::seed_from_u64(0),
            radiuses: Vec::new(),
            max_small_circle_radius: Default::default(),
            delta: 1e-6,
        }
    }
}

impl HeuristicAlgorithmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn rng(mut self, rng: StdRng) -> Self {
        self.rng = rng;
        self
    }

    pub fn radiuses(mut self, radiuses: Vec<FloatType>) -> Self {
        self.radiuses = radiuses;
        self
    }

    pub fn max_small_circle_radius(mut self, max_small_circle_radius: FloatType) -> Self {
        self.max_small_circle_radius = max_small_circle_radius;
        self
    }

    pub fn delta(mut self, delta: FloatType) -> Self {
        self.delta = delta;
        self
    }

    pub fn build(self) -> HeuristicAlgorithm {
        HeuristicAlgorithm {
            iterations: self.iterations,
            rng: self.rng,
            radiuses: self.radiuses,
            max_small_circle_radius: self.max_small_circle_radius,
            delta: self.delta,
        }
    }
}
