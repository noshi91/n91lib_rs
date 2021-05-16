/*

Reference

[1] Cantor, D. G., & Kaltofen, E. (1987).
    Fast multiplication of polynomials over arbitrary rings. Acta Inf, 28.

*/

use crate::other::algebraic::Ring;
use crate::other::itertools::zip;
use crate::other::Polynomial;

use std::fmt::Debug;

pub fn schoenhage_strassen<R>(a: &[R], b: &[R]) -> Vec<R>
where
    R: Ring + Clone + Debug,
{
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let (k2, c2) = schoenhage_strassen_2fold(a, b);
    let (k3, c3) = schoenhage_strassen_3fold(a, b);
    let (x, y) = {
        use crate::other::pow::pow_signed;
        let (x, y, g) = crate::other::extgcd::extgcd(2i64.pow(k2), 3i64.pow(k3));
        assert_eq!(g, 1);
        (pow_signed(R::one(), x), pow_signed(R::one(), y))
    };
    zip(c2, c3)
        .map(|(c2, c3)| x.clone() * c2 + y.clone() * c3)
        .collect()
}

pub use ss_2fold::schoenhage_strassen_2fold;

mod ss_2fold {
    use super::*;

    pub fn schoenhage_strassen_2fold<R>(a: &[R], b: &[R]) -> (u32, Vec<R>)
    where
        R: Ring + Clone + Debug,
    {
        if a.is_empty() || b.is_empty() {
            return (0, vec![]);
        }
        let m = a.len() + b.len() - 1;
        let k = m.next_power_of_two().trailing_zeros();
        let n = 2usize.pow(k);
        let a = resized(a, n);
        let b = resized(b, n);
        let mut c = schoenhage_strassen_2fold_sub(k, a, b).coef;
        fn calc_exp(mut k: u32) -> u32 {
            let mut ret: u32 = 0;
            while k > 2 {
                let l = (k + 1) / 2;
                ret += l;
                k = k - l + 1;
            }
            ret
        }
        c.truncate(m);
        (calc_exp(k), c)
    }

    fn schoenhage_strassen_2fold_sub<R>(k: u32, a: Polynomial<R>, b: Polynomial<R>) -> Polynomial<R>
    where
        R: Ring + Clone + Debug,
    {
        let n = 2usize.pow(k);

        if k <= 2 {
            return negcyc(a * b, n);
        }

        let l = (k + 1) / 2;
        let lp = 2usize.pow(l);
        let m = k - l;
        let mp = 2usize.pow(m);

        let fft = |a: &mut [Polynomial<R>]| {
            for w in (0..l).rev().map(|x| 2usize.pow(x)) {
                let root: usize = (4 * mp) / (2 * w);
                for a in a.chunks_exact_mut(2 * w) {
                    let (x, y) = a.split_at_mut(w);
                    for (p, (x, y)) in zip(x, y).enumerate() {
                        let nx = x.clone() + y.clone();
                        let ny = negcyc((x.clone() - y.clone()) << (p * root), 2 * mp);
                        *x = nx;
                        *y = ny;
                    }
                }
            }
        };

        let ifft = |a: &mut [Polynomial<R>]| {
            for w in (0..l).map(|x| 2usize.pow(x)) {
                let root: usize = (4 * mp) / (2 * w);
                for a in a.chunks_exact_mut(2 * w) {
                    let (x, y) = a.split_at_mut(w);
                    for (p, (x, y)) in zip(x, y).enumerate() {
                        let py = shr(y.clone(), p * root);
                        let nx = x.clone() + py.clone();
                        *y = x.clone() - py;
                        *x = nx;
                    }
                }
            }
        };

        let weight = 2 * mp / lp;

        let extend_and_fft = |a: &[R]| -> Vec<Polynomial<R>> {
            let mut ah: Vec<Polynomial<R>> = a
                .chunks_exact(mp)
                .enumerate()
                .map(|(i, a)| negcyc(resized(a, 2 * mp) << (i * weight), 2 * mp))
                .collect();
            fft(&mut ah);
            ah
        };

        let ah = extend_and_fft(&a.coef);
        let bh = extend_and_fft(&b.coef);
        let mut ch: Vec<Polynomial<R>> = zip(ah, bh)
            .map(|(a, b)| schoenhage_strassen_2fold_sub(m + 1, a, b))
            .collect();
        ifft(&mut ch);
        for (i, c) in ch.iter_mut().enumerate() {
            *c = shr(c.clone(), i * weight);
        }
        let mut c = vec![R::zero(); n];
        for (i, mut ch) in ch.into_iter().enumerate() {
            if i + 1 == lp {
                let w = ch.coef.split_off(mp);
                for (c, ch) in zip(&mut c[mp * i..], ch) {
                    *c += ch;
                }
                for (c, w) in zip(&mut c, w) {
                    *c -= w;
                }
            } else {
                for (c, ch) in zip(&mut c[mp * i..mp * (i + 2)], ch) {
                    *c += ch;
                }
            }
        }
        c.into()
    }

    fn shr<R>(mut p: Polynomial<R>, n: usize) -> Polynomial<R>
    where
        R: Ring + Debug,
    {
        let mut w = p.coef.split_off(n);
        w.extend((-p).coef);
        w.into()
    }

    fn negcyc<R>(mut p: Polynomial<R>, n: usize) -> Polynomial<R>
    where
        R: Ring,
    {
        let w = p.coef.split_off(n);
        for (c, w) in zip(&mut p, w) {
            *c -= w;
        }
        p
    }
}

pub fn schoenhage_strassen_3fold<R>(_a: &[R], _b: &[R]) -> (u32, Vec<R>)
where
    R: Ring,
{
    unimplemented!()
}

fn resized<R>(a: &[R], n: usize) -> Polynomial<R>
where
    R: Ring + Clone,
{
    a.iter()
        .cloned()
        .chain(std::iter::once(R::zero()).cycle())
        .take(n)
        .collect()
}

#[test]
fn test_schoenhage_strassen_2fold() {
    use crate::other::rand::{rand_int, random};
    use crate::other::Fp;
    use crate::other::Polynomial;

    fn check(a: Polynomial<Fp>, b: Polynomial<Fp>) {
        let (k, c) = schoenhage_strassen_2fold(&a.coef, &b.coef);
        let c = Polynomial { coef: c };
        let r: &Fp = &random();
        assert_eq!(
            a.evaluate(r) * b.evaluate(r) * Fp::from(2).pow(k.into()),
            c.evaluate(r)
        );
    }

    fn test(n: usize, m: usize) {
        let a: Polynomial<Fp> = (0..n).map(|_| random()).collect();
        let b: Polynomial<Fp> = (0..m).map(|_| random()).collect();
        check(a, b);
    }

    for _ in 0..1000 {
        test(rand_int(0..10), rand_int(0..10));
    }

    for _ in 0..3 {
        test(rand_int(0..10000), rand_int(0..10000));
    }
}
