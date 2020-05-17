use num_traits::Zero;
use std::ops::{Add, Neg};

pub trait Closed
where
    Self: Add<Output = Self> + Sized,
{
}

impl<T> Closed for T where T: Add<Output = T> {}

pub trait Associative
where
    Self: Closed,
{
}

pub trait Invertible
where
    Self: Closed + Associative + Zero + Neg<Output = Self>,
{
}

pub trait Semigroup
where
    Self: Closed + Associative,
{
}

impl<T> Semigroup for T where T: Closed + Associative {}

pub trait Monoid
where
    Self: Closed + Associative + Zero,
{
}

impl<T> Monoid for T where T: Closed + Associative + Zero {}

pub trait Group
where
    Self: Closed + Associative + Zero + Invertible,
{
}

impl<T> Group for T where T: Closed + Associative + Zero + Invertible {}
