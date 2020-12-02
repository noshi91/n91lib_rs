/*

Description

g: Fp の原始根
a: 長さ 2^n の Fp の列
r: 1 の原始 2^n 乗根

b_i := Σ_k a_k r^(ik) を計算する。

時間計算量: Θ(2^n n)

Fp 上の高速フーリエ変換。
法などにはさらに制限があるが、特に記述しない。

*/

use crate::other::algebraic::{one, zero};
use crate::other::{fp::P, Fp};
use std::mem::swap;

pub fn number_theoretic_transform(g: Fp, a: &mut [Fp]) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let mask = n - 1;
    let lgn = n.trailing_zeros();
    let root = g.pow((P - 1) / n as u32);
    let mut a = a;
    let mut b = vec![Fp(0); n].into_boxed_slice();
    let mut b: &mut [Fp] = &mut b;

    for i_ in (0..lgn).rev() {
        swap(&mut a, &mut b);
        let i: usize = 1 << i_;
        let mut c: Fp = one();
        let d = root.pow(i as u32);

        for j in (0..n).step_by(i) {
            let l = j * 2 & mask;
            let r = l + i;
            for k in 0..i {
                a[j + k] = b[l + k] + b[r + k] * c;
            }
            c *= d;
        }
    }

    if lgn % 2 == 1 {
        b.copy_from_slice(a);
    }
}

pub fn inverse_number_theoretic_transform(g: Fp, a: &mut [Fp]) {
    number_theoretic_transform(g, a);
    a[1..].reverse();
    let inv = one::<Fp>() / Fp::from(a.len());
    for a in a {
        *a *= inv;
    }
}

pub fn fp_convolution(g: Fp, mut a: Vec<Fp>, mut b: Vec<Fp>) -> Vec<Fp> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return Vec::new();
    }
    let r = (n + m - 1).next_power_of_two();

    a.resize(r, zero());
    number_theoretic_transform(g, &mut a);
    b.resize(r, zero());
    number_theoretic_transform(g, &mut b);
    for (a, b) in a.iter_mut().zip(b) {
        *a *= b;
    }
    inverse_number_theoretic_transform(g, &mut a);
    a.truncate(n + m - 1);
    a
}

#[test]
fn test_number_theoretic_transform() {
    use crate::other::fp::P;
    use crate::other::rand::{rand_int, random};

    assert_eq!(P, 998244353);

    fn naive(a: &Vec<Fp>, b: &Vec<Fp>) -> Vec<Fp> {
        use std::cmp::max;

        let mut c = vec![zero(); a.len() + b.len()];
        let mut s = 0;
        for i in 0..a.len() {
            for j in 0..b.len() {
                c[i + j] += a[i] * b[j];
                s = max(s, i + j + 1);
            }
        }
        c.resize(s, zero());
        c
    }

    let q = 100;
    let n_max = 100;
    for _ in 0..q {
        let n = rand_int(0..n_max);
        let m = rand_int(0..n_max);

        let a: Vec<Fp> = (0..n).map(|_| random()).collect();
        let b: Vec<Fp> = (0..m).map(|_| random()).collect();

        assert_eq!(naive(&a, &b), fp_convolution(Fp(3), a, b));
    }
}
