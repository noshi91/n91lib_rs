/*

Description

a: 長さ n の列
b: 長さ m の列
t: n + m

c[k] := max_{i + j = k} min{a[i], b[j]} を計算する。

時間計算量: Θ(t √(t log(t)))

x を固定して結果が x 以上になるかどうかという問題を考えると、
01 列の (+, ×)-convolution で計算できることが分かる。
値を平方分割してブロック内の部分を愚直で計算することで、
Θ(t √t log(t)) となる。
ブロックサイズを調整して Θ(t √(t log(t))) のアルゴリズムを得る。

畳み込み全般に言える事であるが、n と m が大きく離れている場合、
列を分割して畳み込むと log などの部分を小さい方に依存させることが出来る。

*/

pub fn max_min_convolution<'a, T>(a: &'a [T], b: &'a [T]) -> Box<[&'a T]>
where
    T: Ord,
{
    use crate::other::{fp::P, Fp};

    assert_eq!(P, 998244353);

    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![].into_boxed_slice();
    }

    #[derive(Copy, Clone)]
    enum Elem {
        A(usize),
        B(usize),
    }
    let mut idx = (0..n)
        .map(|i| Elem::A(i))
        .chain((0..m).map(|i| Elem::B(i)))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    idx.sort_unstable_by_key(|&e| match e {
        Elem::A(i) => &a[i],
        Elem::B(j) => &b[j],
    });
    idx.reverse();

    let block_size = {
        let s = (n + m) as f64;
        (s * s.log2()).sqrt().ceil() as usize
    };
    let mut a_ = vec![false; n].into_boxed_slice();
    let mut b_ = vec![false; m].into_boxed_slice();
    let mut res: Box<[Option<&'a T>]> = vec![None; n + m - 1].into_boxed_slice();
    for block in idx.chunks(block_size) {
        let mut a_t: Vec<_> = a_.iter().map(|&x| Fp(if x { 1 } else { 0 })).collect();
        let mut b_t: Vec<_> = b_.iter().map(|&x| Fp(if x { 1 } else { 0 })).collect();
        for &e in block {
            match e {
                Elem::A(i) => a_t[i] = Fp(1),
                Elem::B(j) => b_t[j] = Fp(1),
            }
        }

        use crate::algorithm::number_theoretic_transform::fp_convolution;

        let conv = fp_convolution(Fp(3), a_t, b_t);
        let check = (0..n + m - 1)
            .filter(|&i| res[i].is_none() && conv[i] != Fp(0))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        for &e in block {
            match e {
                Elem::A(i) => {
                    for &k in &*check {
                        if i <= k && k - i < m && b_[k - i] && res[k].is_none() {
                            res[k] = Some(&a[i]);
                        }
                    }
                    a_[i] = true;
                }
                Elem::B(j) => {
                    for &k in &*check {
                        if j <= k && k - j < n && a_[k - j] && res[k].is_none() {
                            res[k] = Some(&b[j]);
                        }
                    }
                    b_[j] = true;
                }
            }
        }
    }

    res.into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

#[test]
fn test_max_min_convolution() {
    fn naive<'a, T>(a: &'a [T], b: &'a [T]) -> Box<[&'a T]>
    where
        T: Ord,
    {
        use std::cmp::{max, min};

        let mut s_max = 0;
        let mut res = vec![None; a.len() + b.len()];
        for i in 0..a.len() {
            for j in 0..b.len() {
                let v = min(&a[i], &b[j]);
                let r = &mut res[i + j];
                match *r {
                    None => *r = Some(v),
                    Some(ref mut x) => *x = max(x, v),
                }
                s_max = max(s_max, i + j + 1);
            }
        }

        res.truncate(s_max);
        res.into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn inner(q: usize, n_max: usize, s: usize) {
        use crate::other::rand::rand_int;

        for _ in 0..q {
            let n = rand_int(0..n_max);
            let m = rand_int(0..n_max);

            let a: Vec<_> = (0..n).map(|_| rand_int(0..s)).collect();
            let b: Vec<_> = (0..m).map(|_| rand_int(0..s)).collect();

            assert_eq!(naive(&a, &b), max_min_convolution(&a, &b));
        }
    }

    inner(100, 100, 10);
    inner(100, 100, 100);
    inner(100, 100, 1000);
    inner(100, 10, 10);
}
