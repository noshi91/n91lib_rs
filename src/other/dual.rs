use crate::other::algebraic::{Associative, Unital};
use num_traits::Zero;
use std::ops::Add;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Dual<T>(pub T);

impl<T> Add for Dual<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, right: Self) -> Self {
        Self(right.0 + self.0)
    }
}

impl<T> Associative for Dual<T> where T: Associative {}

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

impl<T> Unital for Dual<T> where T: Unital {}
