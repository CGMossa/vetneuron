use derive_more::*;

use crate::rate::Rate;

#[derive(Debug, Into, Deref)]
pub struct Probability(pub f64);

impl From<Rate> for Probability {
    /// Since P(X = 0) = exp(-lambda) for X ~ Poisson(lambda);
    /// Then p = 1 - exp(-lambda)
    fn from(rate: Rate) -> Self {
        Self(1. - (-rate.0).exp())
    }
}
