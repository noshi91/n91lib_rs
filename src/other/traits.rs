use num_traits::Zero;
use std::ops::Add;

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

pub trait Monoid
where
    Self: Closed + Associative + Zero,
{
}

impl<T> Monoid for T where T: Closed + Associative + Zero {}
