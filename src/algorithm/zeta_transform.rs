use crate::other::traits::{Group, Semigroup};
use std::clone::Clone;

pub fn subset_zeta<T>(a: &mut Vec<T>)
where
    T: Semigroup + Clone,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    for p in (0..n.trailing_zeros()).map(|i| 1 << i) {
        for i in 0..n {
            if i & p != 0 {
                a[i] = a[i & !p].clone() + a[i].clone();
            }
        }
    }
}

pub fn subset_mobius<T>(a: &mut Vec<T>)
where
    T: Group + Clone,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    for p in (0..n.trailing_zeros()).rev().map(|i| 1 << i) {
        for i in 0..n {
            if i & p != 0 {
                a[i] = -a[i & !p].clone() + a[i].clone();
            }
        }
    }
}
