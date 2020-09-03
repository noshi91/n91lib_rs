use crate::algorithm::zeta_transform;
use crate::other::algebraic::Ring;
use crate::other::Polynomial;
use itertools::{enumerate, zip};
use std::clone::Clone;

pub fn subset_convolution<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
where
    T: Ring + Clone,
{
    let n = a.len();
    assert!(n.is_power_of_two());
    assert_eq!(b.len(), n);

    let ranked_zeta = |a: Vec<_>| {
        let mut a_ext = enumerate(a)
            .map(|(i, a)| Polynomial::from(vec![a]) << i.count_ones() as usize)
            .collect();
        zeta_transform::subset_zeta(&mut a_ext);
        a_ext
    };

    let a_ext = ranked_zeta(a);
    let b_ext = ranked_zeta(b);

    let mut c_ext: Vec<_> = zip(a_ext, b_ext).map(|(a, b)| a * b).collect();
    zeta_transform::subset_mobius(&mut c_ext);

    enumerate(c_ext)
        .map(|(i, c)| c[i.count_ones() as usize].clone())
        .collect()
}

#[test]
fn test_subset_convolution() {
    use crate::other::Fp;

    let make = |v: Vec<i32>| -> Vec<Fp> { v.into_iter().map(|x| x.into()).collect() };

    let a = make(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let b = make(vec![9, 10, 11, 12, 13, 14, 15, 16]);
    let c = subset_convolution(a, b);

    let ans = make(vec![9, 28, 38, 100, 58, 144, 172, 408]);

    assert_eq!(c, ans);
}
