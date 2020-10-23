/*

Description

T: 体
a: T 上の n × n 行列

a の行列式を計算する。

時間計算量: Θ(n^3) 回の演算と Θ(n) 回の除算

オーソドックスな掃き出し法。

*/

use crate::other::algebraic::Field;
use num_traits::{one, zero};
use std::clone::Clone;

pub fn determinant<T>(mut a: Vec<Vec<T>>) -> T
where
    T: Field + Clone,
{
    let n = a.len();
    for a in &a {
        assert_eq!(a.len(), n);
    }

    let mut res: T = one();

    for col in 0..n {
        match (col..n).find(|&row| !a[row][col].is_zero()) {
            None => return zero(),
            Some(row) => {
                if row != col {
                    a.swap(col, row);
                    res = -res;
                }
            }
        }
        {
            let c = a[col][col].clone();
            let inv_c = T::one() / c.clone();
            for a in &mut a[col][col..] {
                *a *= inv_c.clone();
            }
            res *= c;
        }
        let (p, r) = a.split_at_mut(col + 1);
        let p = p.last().unwrap();
        for r in r {
            let c = r[col].clone();
            for (p, r) in p[col..].iter().zip(&mut r[col..]) {
                *r -= c.clone() * p.clone();
            }
        }
    }

    res
}

#[test]
fn test_determinant() {
    use crate::other::Fp;

    let a = vec![vec![Fp(5), Fp(2)], vec![Fp(3), Fp(4)]];
    assert_eq!(determinant(a), Fp(14));
}
