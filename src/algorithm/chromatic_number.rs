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
このアルゴリズムは O(n^2 / 2^w) の確率で本来より大きい値を返す。

時間計算量 O(m + 2^n n)

彩色数は V(G) の独立集合による被覆である。
ここで、それぞれの独立集合が重なっても良いとしても彩色数を求める上では問題がない。
X_S を S が独立集合であるかの指示ベクトルとすると、
独立集合の k-tuple であって S を被覆するものの個数は
X をゼータ変換して k 乗したのちメビウス変換すれば求まる。
これが非零になる最小の k が求める彩色数である。

実際にはメビウス変換した後の V(G) の項だけが必要なので、
メビウス変換を陽には行わず V(G) の項だけ計算すればよい。

計算に用いる値は 2^(n^2) 程度に収まるため、多倍長整数を使えば
O^~(2^n) のアルゴリズムが得られる。
ランダムな素数 p を選び F_p 上で計算すれば、高い確率で正確な計算が可能である。
この実装では、空間計算量が Θ(2^n) となる都合上 n < w と仮定しており、
実装の簡略化のため p も固定している。

*/

pub fn chromatic_number(n: usize, edges: &[(usize, usize)]) -> Option<usize> {
    use crate::algorithm::zeta_transform::subset_zeta;
    use crate::other::bit::{access, bsf};
    use crate::other::Fp;

    let mut adj = vec![vec![false; n]; n];

    for &(u, v) in edges {
        assert!(u < n && v < n);
        if u == v {
            return None;
        }
        adj[u][v] = true;
        adj[v][u] = true;
    }

    let adj = adj;

    if n == 0 {
        return Some(0);
    }

    let x: Vec<_> = {
        let mut is_indep = vec![false; 1 << n];
        is_indep[0] = true;

        for s in 1..1 << n {
            let v = bsf(s);
            is_indep[s] =
                is_indep[s & !(1 << v)] && (0..n).filter(|&u| access(s, u)).all(|u| !adj[v][u]);
        }

        let mut x: Vec<_> = is_indep
            .into_iter()
            .map(|t| Fp::from(if t { 1 } else { 0 }))
            .collect();

        subset_zeta(&mut x);
        x
    };

    let coef: Vec<_> = (0..1 << n)
        .map(|s: usize| {
            Fp::from(if (n - s.count_ones() as usize) % 2 == 0 {
                1
            } else {
                -1
            })
        })
        .collect();

    let mut powed = vec![Fp::from(1); 1 << n];

    for k in 1..n {
        for (p, x) in powed.iter_mut().zip(&x) {
            *p *= *x;
        }
        if coef.iter().zip(&powed).map(|(c, p)| *c * *p).sum::<Fp>() != Fp::from(0) {
            return Some(k);
        }
    }

    Some(n)
}
