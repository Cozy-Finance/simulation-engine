use rand::Rng;
use rand_distr::{num_traits::ToPrimitive, Distribution, Exp};
use serde::Deserialize;
use simulate::u256::{deserialize_string_to_u256, U256};

use crate::cozy::constants::*;

pub trait CozyDistribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

#[derive(Debug, Clone, Deserialize)]
pub struct U256UniformRange {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub min: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub max: U256,
}

impl CozyDistribution<U256> for U256UniformRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> U256 {
        let mut sample = [0_u8; 32];
        rng.fill(&mut sample[..]);
        U256::from(sample) % (self.max - self.min) + self.min
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum TimeUnit {
    Second,
    Hour,
    Day,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Exponential {
    pub rate: f64,
    pub time_unit: TimeUnit,
}

impl CozyDistribution<f64> for Exponential {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let exp = Exp::new(self.rate).expect("Exponential distribution rate should be valid.");
        exp.sample(rng)
    }
}

impl Exponential {
    pub fn sample_in_secs<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let sample = self.sample(rng);
        match &self.time_unit {
            TimeUnit::Second => sample,
            TimeUnit::Hour => sample * SECONDS_IN_MINUTE.to_f64().expect("u64 to f64"),
            TimeUnit::Day => sample * SECONDS_IN_DAY.to_f64().expect("u64 to f64"),
        }
    }
}
