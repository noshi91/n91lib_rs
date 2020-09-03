use crate::other::algebraic::{CommutativeMonoid, Semigroup};
use num_traits::Zero;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
pub struct Dual<T>(pub T);

impl<T> Add<Self> for Dual<T>
where
    T: Semigroup,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(rhs.0 + self.0)
    }
}

impl<T> AddAssign<Self> for Dual<T>
where
    T: CommutativeMonoid,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<T> Zero for Dual<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}
