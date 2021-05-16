use crate::other::algebraic::{One, Zero};
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Multiplicative<T>(pub T)
where
    T: Mul<T, Output = T>;

impl<T> Add for Multiplicative<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl<T> Zero for Multiplicative<T>
where
    T: One + Eq,
{
    fn zero() -> Self {
        Self(T::one())
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}
