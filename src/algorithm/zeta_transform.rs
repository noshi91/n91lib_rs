use crate::other::algebraic::{Abelian, CommutativeSemigroup};

pub fn subset_zeta<T>(a: &mut Vec<T>)
where
    T: CommutativeSemigroup + Clone,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    for w in (0..n.trailing_zeros()).map(|i| 1 << i) {
        for k in (0..n).step_by(2 * w) {
            for i in 0..w {
                let t = a[k + i].clone();
                a[k + w + i] += t;
            }
        }
    }
}

pub fn subset_mobius<T>(a: &mut Vec<T>)
where
    T: Abelian + Clone,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    for w in (0..n.trailing_zeros()).map(|i| 1 << i) {
        for k in (0..n).step_by(2 * w) {
            for i in 0..w {
                let t = a[k + i].clone();
                a[k + w + i] -= t;
            }
        }
    }
}
