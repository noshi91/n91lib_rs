use crate::other::algebraic::{Group, Monoid};

pub fn pow<T>(mut x: T, mut n: u64) -> T
where
    T: Monoid + Clone,
{
    let mut r = T::zero();
    while n != 0 {
        if n % 2 != 0 {
            r = r + x.clone();
        }
        x = x.clone() + x;
        n /= 2;
    }
    r
}

pub fn pow_signed<T>(x: T, n: i64) -> T
where
    T: Group + Clone,
{
    if n < 0 {
        -pow(x, -n as u64)
    } else {
        pow(x, n as u64)
    }
}
