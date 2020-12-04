/*

Reference

[1] Clifford, P., & Clifford, R. (2007).
    Simple deterministic wildcard matching.
    Information Processing Letters, 101(2), 53-54.

[2] Clifford, R., Efremenko, K., Porat, E., & Rothschild, A. (2009, January).
    From coding theory to efficient pattern matching.
    In Proceedings of the twentieth annual ACM-SIAM symposium on Discrete algorithms (pp. 778-784).
    Society for Industrial and Applied Mathematics.


Description

S: ? を含む文字列
P: ? を含む文字列
n: |S|
m: |P|
n >= m

S の各位置が P にマッチするか確率的に判定する。? は任意の文字とマッチする。
誤ってマッチしていると判定する確率は O(1 / 2^w)

時間計算量: Θ(n log(n))

? に 0, 他の文字に正の整数を割り当てる。
文字列 S, T がマッチ ⇔ Σ S_i T_i (S_i - T_i)^2 = 0 であることを利用する。
この式を積和に展開すると、それぞれの項は畳み込みによって全てのスライド位置に対して計算できる。
値の最大値は Θ(m σ^4) となり、これが [1] のアルゴリズムである。

S'_i := 0 (if S_i = '?') 1 (otherwise) とする。
文字列 S, T がマッチ ⇔ Σ S'_i T'_i (S_i - T_i)^2 = 0
これにより値の最大値が Θ(m σ^2) に改善される。これが [2] で言及されているアルゴリズムである。

本実装はこれらを元にした変種である。
新たな変数列 (x_i) を用意し、全ての文字も別々の変数と考える。
文字列 S, T がマッチ ⇔ Σ x_i S'_i T'_i (S_i - T_i) = 0
これは高々 2 次の多項式になるので、各変数にランダムな F_q の元を割り当てれば
Schwartz-Zippel lemma から高確率で非零判定が可能である。
競技プログラミング的視点では、畳み込みの回数が 3 回から 2 回に減ること、
σ が大きくても問題ない部分が利点となると考えている。

n と m が大きく異なる場合、S を分割することで O(n log(m)) に改善される。
現状、本実装が依存している畳み込みの実装はそのような工夫を行っていないため、
時間計算量は Θ(n log(n)) となっている。


Detail

None を wildcard, Some(c) を通常の文字としている。
0 <= c < n + m を仮定しているので、必要なら適宜座標圧縮せよ。

*/

pub fn wildcard_matching(s: &[Option<usize>], p: &[Option<usize>]) -> Vec<bool> {
    assert_eq!(crate::other::fp::P, 998244353);

    let n = s.len();
    let m = p.len();
    assert!(1 <= m && m <= n);
    for c in s {
        assert!(c.map_or(true, |c| c < n + m));
    }
    for c in p {
        assert!(c.map_or(true, |c| c < n + m));
    }

    use crate::algorithm::number_theoretic_transform::fp_convolution;
    use crate::other::rand::random;
    use crate::other::Fp;

    let x: Vec<Fp> = (0..n + m).map(|_| random()).collect();
    let t: Vec<Fp> = (0..m).map(|_| random()).collect();

    let mut res = vec![Fp(0); n + m - 1];
    {
        let a: Vec<_> = s.iter().map(|c| c.map_or(Fp(0), |i| x[i])).collect();
        let b: Vec<_> = p
            .iter()
            .rev()
            .zip(&t)
            .map(|(c, t)| c.map_or(Fp(0), |_| *t))
            .collect();
        for (res, c) in res.iter_mut().zip(fp_convolution(Fp(3), a, b)) {
            *res += c;
        }
    }
    {
        let a: Vec<_> = s.iter().map(|c| c.map_or(Fp(0), |_| Fp(1))).collect();
        let b: Vec<_> = p
            .iter()
            .rev()
            .zip(&t)
            .map(|(c, t)| c.map_or(Fp(0), |i| x[i] * *t))
            .collect();
        for (res, c) in res.iter_mut().zip(fp_convolution(Fp(3), a, b)) {
            *res -= c;
        }
    }

    res[m - 1..n].iter().map(|v| *v == Fp(0)).collect()
}

#[test]
fn test_wildcard_matching() {
    fn naive(s: &[Option<usize>], p: &[Option<usize>]) -> Vec<bool> {
        s.windows(p.len())
            .map(|w| {
                w.iter().zip(p).all(|e| match e {
                    (&None, _) | (_, &None) => true,
                    (&Some(x), &Some(y)) => x == y,
                })
            })
            .collect()
    }

    fn testset(q: usize, n: usize, m: usize, sig: usize, w_p: f64) {
        use crate::other::rand::{rand_f64, rand_int};
        for _ in 0..q {
            let n = rand_int(1..n);
            let m = rand_int(1..m.min(n + 1));
            let sig = sig.min(n + m);

            let s: Vec<_> = (0..n)
                .map(|_| {
                    if rand_f64() < w_p {
                        None
                    } else {
                        Some(rand_int(0..sig))
                    }
                })
                .collect();
            let p: Vec<_> = (0..m)
                .map(|_| {
                    if rand_f64() < w_p {
                        None
                    } else {
                        Some(rand_int(0..sig))
                    }
                })
                .collect();

            assert_eq!(naive(&s, &p), wildcard_matching(&s, &p));
        }
    }

    testset(100, 100, 10, 2, 0.7);
    testset(10, 1000, 100, 10, 0.5);
    testset(1000, 100, 6, 2, 0.1);
}
