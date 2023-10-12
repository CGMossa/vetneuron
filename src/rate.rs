use derive_more::*;

#[readonly::make]
#[derive(Debug, PartialEq, PartialOrd, Into, Add, AddAssign)]
pub struct Rate(pub f64);

impl Default for Rate {
    fn default() -> Self {
        Self(0.)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot create a rate from negative number, instead {:?}", .0)]
    NegativeRate(f64),
}

impl TryFrom<f64> for Rate {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        value
            .is_sign_positive()
            .then(|| Self(value))
            .ok_or_else(|| Error::NegativeRate(value))
    }
}
