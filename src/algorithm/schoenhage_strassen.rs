use crate::other::algebraic::Ring;
use crate::other::itertools::zip;

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
    let a = resized(a, n, R::zero());
    let b = resized(b, n, R::zero());
    let mut c = vec![R::zero(); n];
    schoenhage_strassen_2fold_sub(k, &a, &b, &mut c);
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

fn schoenhage_strassen_2fold_sub<R>(k: u32, a: &[R], b: &[R], c: &mut [R])
where
    R: Ring + Clone + Debug,
{
    let n = 2usize.pow(k);

    if k <= 2 {
        for (i, a) in a.iter().enumerate() {
            for (j, b) in b.iter().cloned().enumerate() {
                let t = i + j;
                if t < n {
                    c[t] += a.clone() * b;
                } else {
                    c[t - n] -= a.clone() * b;
                }
            }
        }
        return;
    }

    let l = (k + 1) / 2;
    let lp = 2usize.pow(l);
    let m = k - l;
    let mp = 2usize.pow(m);

    let fft = |a: &mut [R]| {
        let mut temp = vec![R::zero(); 2 * mp];
        for w in (0..l).rev().map(|x| 2usize.pow(x)) {
            let root: usize = (4 * mp) / (2 * w);
            for a in a.chunks_exact_mut(2 * w * 2 * mp) {
                let mut s: usize = 0;
                let (x, y) = a.split_at_mut(w * 2 * mp);
                for (x, y) in zip(x.chunks_exact_mut(2 * mp), y.chunks_exact_mut(2 * mp)) {
                    temp.clone_from_slice(x);
                    for (x, y) in zip(x, &*y) {
                        *x += y.clone();
                    }
                    for (t, y) in zip(&mut temp, &mut *y) {
                        *t = t.clone() - y.clone();
                    }
                    y[s..].clone_from_slice(&temp[..2 * mp - s]);
                    for (y, t) in zip(y, &temp[2 * mp - s..]) {
                        *y = -t.clone();
                    }
                    s += root;
                }
            }
        }
    };

    let ifft = |a: &mut [R]| {
        let mut temp = vec![R::zero(); 2 * mp];
        for w in (0..l).map(|x| 2usize.pow(x)) {
            let root: usize = (4 * mp) / (2 * w);
            for a in a.chunks_exact_mut(2 * w * 2 * mp) {
                let mut s: usize = 0;
                let (x, y) = a.split_at_mut(w * 2 * mp);
                for (x, y) in zip(x.chunks_exact_mut(2 * mp), y.chunks_exact_mut(2 * mp)) {
                    temp[0..2 * mp - s].clone_from_slice(&y[s..]);
                    for (t, y) in zip(&mut temp[2 * mp - s..], &*y) {
                        *t = -y.clone();
                    }
                    y.clone_from_slice(x);
                    for (x, t) in zip(x, &temp) {
                        *x += t.clone();
                    }
                    for (y, t) in zip(y, &temp) {
                        *y -= t.clone();
                    }
                    s += root;
                }
            }
        }
    };

    let extend_and_fft = |a: &[R]| -> Vec<R> {
        let mut ah = vec![R::zero(); 2 * n];
        for (a, ah) in zip(a.chunks_exact(mp), ah.chunks_exact_mut(2 * mp)) {
            ah[0..mp].clone_from_slice(a);
        }
        let weight = 2 * mp / lp;
        for (s, ah) in ah.chunks_exact_mut(2 * mp).enumerate() {
            ah.rotate_right(weight * s);
            for ah in &mut ah[0..weight * s] {
                *ah = -ah.clone();
            }
        }
        fft(&mut ah);
        ah
    };

    let ah = extend_and_fft(a);
    let bh = extend_and_fft(b);
    let mut ch = vec![R::zero(); 2 * n];
    for (c, (a, b)) in zip(
        ch.chunks_exact_mut(2 * mp),
        zip(ah.chunks_exact(2 * mp), bh.chunks_exact(2 * mp)),
    ) {
        schoenhage_strassen_2fold_sub(m + 1, a, b, c);
    }
    ifft(&mut ch);
    {
        let weight = 2 * mp / lp;
        for (s, ch) in ch.chunks_exact_mut(2 * mp).enumerate() {
            for ch in &mut ch[0..weight * s] {
                *ch = -ch.clone();
            }
            ch.rotate_left(weight * s);
        }
    }
    for i in 0..lp {
        for j in 0..mp {
            c[mp * i + j] += ch[2 * mp * i + j].clone();
        }
        if i + 1 == lp {
            for j in mp..2 * mp {
                c[j - mp] -= ch[2 * mp * i + j].clone();
            }
        } else {
            for j in mp..2 * mp {
                c[mp * i + j] += ch[2 * mp * i + j].clone();
            }
        }
    }
}

pub fn schoenhage_strassen_3fold<R>(a: &[R], b: &[R]) -> (u32, Vec<R>)
where
    R: Ring,
{
    unimplemented!()
}

fn resized<T>(a: &[T], n: usize, x: T) -> Vec<T>
where
    T: Clone,
{
    a.iter()
        .cloned()
        .chain(std::iter::once(x).cycle())
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
