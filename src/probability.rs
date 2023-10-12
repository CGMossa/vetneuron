// TODO: <https://twitter.com/rlmcelreath/status/1688465789043568640>

use derive_more::*;
use thiserror::Error;

use crate::rate::Rate;

#[derive(Debug, Into)]
#[readonly::make]
pub struct Probability(pub f64);

#[derive(Debug, Error)]
pub enum Error {
    #[error("a probability has to be between 0 and 1, not {:?}", .0)]
    InvalidProbability(f64),
}

impl TryFrom<f64> for Probability {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !((0.0)..=1.0).contains(&value) {
            return Err(Error::InvalidProbability(value));
        }

        Ok(Self(value))
    }
}

impl From<Rate> for Probability {
    /// Since P(X = 0) = exp(-lambda) for X ~ Poisson(lambda);
    /// Then p = 1 - exp(-lambda)
    fn from(rate: Rate) -> Self {
        Self(1. - (-rate.0).exp())
    }
}

impl std::ops::Add for Probability {
    type Output = Self;

    // Probability of neither happening,
    // then the probability that either happened follows
    fn add(self, rhs: Self) -> Self::Output {
        Self(1. - (1. - self.0) * (1. - rhs.0))
    }
}

impl std::iter::Sum<Self> for Probability {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(1. - iter.map(|x| 1. - x.0).sum::<f64>())
    }
}
