/*

Description

T: 半環
a: T 上の n × n 行列
f: 消去時の係数行列

a をガウスの消去法の手続きに従って上三角行列に変換する

時間計算量: Θ(n^3) 回の演算と Θ(n^2) 回の f の呼び出し

*/

use crate::other::algebraic::CommutativeSemiring;

pub fn manually_gaussian_elimination<T, F>(a: &mut Vec<Vec<T>>, mut f: F)
where
    T: CommutativeSemiring + Clone,
    F: FnMut([&T; 2]) -> [[T; 2]; 2],
{
    let n = a.len();
    for a in &*a {
        assert_eq!(a.len(), n);
    }

    for col in 0..n {
        let (x, y) = a.split_at_mut(col + 1);
        let x = x.last_mut().unwrap();
        for y in y {
            let c = f([&x[col], &y[col]]);
            for (x, y) in x.iter_mut().zip(&mut *y) {
                let new_x = c[0][0].clone() * x.clone() + c[0][1].clone() * y.clone();
                *y = c[1][0].clone() * x.clone() + c[1][1].clone() * y.clone();
                *x = new_x;
            }
            assert!(y[col].is_zero());
        }
    }
}

use crate::other::algebraic::{one, zero, CommutativeRing};
use std::ops::Div;

pub fn ext_gcd<T>(x: [&T; 2]) -> [[T; 2]; 2]
where
    T: CommutativeRing + Div<Output = T> + Clone,
{
    use std::mem::swap;

    let mut mat = [
        (x[0].clone(), x[1].clone()),
        (one(), zero()),
        (zero(), one()),
    ];

    let mut neg = false;
    while !mat[0].1.is_zero() {
        let q = mat[0].0.clone() / mat[0].1.clone();
        for &mut (ref mut x, ref mut y) in mat.iter_mut() {
            swap(x, y);
            *y -= q.clone() * x.clone();
        }
        neg ^= true;
    }

    let mut res = [
        [mat[1].0.clone(), mat[2].0.clone()],
        [mat[1].1.clone(), mat[2].1.clone()],
    ];
    if neg {
        res[1][0] = -res[1][0].clone();
        res[1][1] = -res[1][1].clone();
    }
    res
}
