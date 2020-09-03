use num_traits::Zero;
use std::clone::Clone;
use std::mem::swap;
use std::ops::RemAssign;

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Zero + RemAssign<T> + Clone,
{
    while !b.is_zero() {
        a %= b.clone();
        swap(&mut a, &mut b);
    }
    a
}
