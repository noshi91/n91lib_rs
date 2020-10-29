/*

min を演算とする半群

*/

#[derive(Copy, Clone)]
pub struct Min<T>(pub T)
where
    T: Ord;

use std::ops::{Add, AddAssign};

impl<T> Add for Min<T>
where
    T: Ord,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0))
    }
}

impl<T> AddAssign for Min<T>
where
    T: Ord,
{
    fn add_assign(&mut self, rhs: Self) {
        if self.0 > rhs.0 {
            *self = rhs;
        }
    }
}
