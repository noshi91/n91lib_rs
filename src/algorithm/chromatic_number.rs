/*

Reference

[1] Björklund, A., & Husfeldt, T. (2006, January).
    Inclusion-exclusion based algorithms for graph colouring.
    In Electronic Colloquium on Computational Complexity (ECCC) (Vol. 13, No. 044).


Description

G: 無向グラフ
n: |V(G)|
m: |E(G)|

G の彩色数、すなわち任意の uv \in E(G) について c(u) \neq c(v) となる
c: V(G) → {0,1,...,k-1} が存在する最小の k を計算する。
そのような k が存在しない場合 None を返す。
このアルゴリズムは O(n^2 log(n) / 2^w) の確率で本来より大きい値を返す。
n < w を仮定している。

時間計算量 O(m + 2^n log(n))

彩色は交わらない独立集合による V(G) の被覆と解釈できる。
ここで、それぞれの独立集合が重なっても良いとしても彩色数を求める上では問題がない。
X_S を S に包含される独立集合の個数とすると、
独立集合の k-tuple であって S を被覆するものの個数は
X を k 乗したのち包除原理を用いると計算することが出来る。
これが非零になる最小の k が求める彩色数であるから、
二分探索を用いてそのような k を求めればよい。

計算に用いる値は 2^(n^2) 程度に収まるため、多倍長整数を使えば
O^~(2^n) のアルゴリズムが得られる。
ランダムな素数 p を選び F_p 上で計算すれば、高い確率で正確な計算が可能である。
この実装では実装の簡略化のため、p を固定している。

*/

pub fn chromatic_number(n: usize, edges: &[(usize, usize)]) -> Option<usize> {
    use crate::other::bit::{bsf, ceil_log2, WORD};
    use crate::other::Fp;

    assert!(n < WORD);

    let mut neighbor: Vec<usize> = vec![0; n];
    for &(u, v) in edges {
        assert!(u < n && v < n);
        if u == v {
            return None;
        }
        neighbor[u] |= 1 << v;
        neighbor[v] |= 1 << u;
    }
    for v in 0..n {
        neighbor[v] |= 1 << v;
    }
    let neighbor = neighbor;

    if n == 0 {
        return Some(0);
    }

    let mut x = vec![Fp::from(0); 1 << n];
    x[0] = Fp::from(1);
    for s in 1..1 << n {
        let v = bsf(s);
        x[s] = x[s & !(1 << v)] + x[s & !neighbor[v]];
    }

    let coef: Vec<_> = (0..1 << n)
        .map(|s: usize| {
            Fp::from(if (n - s.count_ones() as usize) % 2 == 0 {
                1
            } else {
                -1
            })
        })
        .collect();

    let mut pow = vec![x];
    for _ in 1..ceil_log2(n) {
        let next: Vec<_> = pow.last().unwrap().iter().map(|&t| t * t).collect();
        pow.push(next);
    }

    let mut res: usize = 0;
    let mut a = vec![Fp::from(1); 1 << n];
    for (i, p) in pow.into_iter().enumerate().rev() {
        let next: Vec<_> = a.iter().zip(p).map(|(&a, p)| a * p).collect();
        if coef.iter().zip(&next).map(|(&c, &t)| c * t).sum::<Fp>() == Fp::from(0) {
            a = next;
            res += 1 << i;
        }
    }

    Some(res + 1)
}
