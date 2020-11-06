/*

References

[1] Karger's algorithm - Wikipedia
    https://en.wikipedia.org/wiki/Karger%27s_algorithm

[2] Week 2: The Karger-Stein Min Cut Algorithm
    http://www.cs.toronto.edu/~anikolov/CSC473W18/Lectures/Karger-Stein.pdf


Description

G: 多重無向グラフ
n: |V(G)|
m: |E(V)|

G の全域最小カットを計算する。
確率 Ω(1 / log(n)) で正しい値を出力する。

時間計算量: Θ(m + n^2 log(n))

*/

pub fn karger_stein(n: usize, edges: &[(usize, usize)]) -> usize {
    assert!(n >= 2);

    {
        use crate::other::is_connected;
        if !is_connected(n, edges) {
            return 0;
        }
    }

    let mut adm = vec![vec![0; n]; n];
    for &(u, v) in edges {
        if u != v {
            adm[u][v] += 1;
            adm[v][u] += 1;
        }
    }
    internal(
        adm.into_iter()
            .map(|a| (a.iter().copied().sum(), a))
            .collect(),
    )
}

fn internal(mut edges: Vec<(usize, Vec<usize>)>) -> usize {
    let n = edges.len();

    if n <= 6 {
        let cutsize = |s| -> usize {
            let row_sum = |(u, &(_, ref a)): (_, &(_, Vec<_>))| -> usize {
                a.iter()
                    .enumerate()
                    .filter_map(|(v, &w)| {
                        use crate::other::bit::access;

                        if !access(s, u) && access(s, v) {
                            Some(w)
                        } else {
                            None
                        }
                    })
                    .sum()
            };
            edges.iter().enumerate().map(row_sum).sum()
        };
        return (1..1 << n - 1).map(cutsize).min().unwrap();
    }

    let target = (n as f64 / 2f64.sqrt() + 1f64) as usize;

    while edges.len() > target {
        use crate::other::rand::rand_int;

        let (u, v) = {
            let edge_count: usize = edges.iter().map(|&(c, _)| c).sum();
            let mut idx = rand_int(0..edge_count);
            let mut u = 0;
            while edges[u].0 <= idx {
                idx -= edges[u].0;
                u += 1;
            }
            let e = &edges[u].1;
            let mut v = 0;
            while e[v] <= idx {
                idx -= e[v];
                v += 1;
            }
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }
            (u, v)
        };

        for &mut (_, ref mut a) in edges.iter_mut() {
            let vm = a.swap_remove(v);
            a[u] += vm;
        }
        {
            let (_, vm) = edges.swap_remove(v);
            for (x, y) in edges[u].1.iter_mut().zip(vm) {
                *x += y;
            }
        }
        edges[u].1[u] = 0;
        let (ref mut cs, ref a) = edges[u];
        *cs = a.iter().copied().sum();
    }

    use std::cmp::min;

    min(internal(edges.clone()), internal(edges))
}

#[test]
fn test_karger_stein() {
    fn naive(n: usize, edges: &[(usize, usize)]) -> usize {
        use crate::other::bit::access;

        let cutsize = |s| -> usize {
            edges
                .iter()
                .filter(|&&(u, v)| access(s, u) != access(s, v))
                .count()
        };
        (1..1 << n - 1).map(cutsize).min().unwrap()
    }

    fn iterated(n: usize, edges: &[(usize, usize)]) -> usize {
        let count = ((n as f64).log(2f64) * 10f64) as usize;
        (0..count).map(|_| karger_stein(n, edges)).min().unwrap()
    }

    fn test_case(cases: usize, n: usize, p: f64) {
        use crate::other::rand::{rand_f64, rand_int};

        for _ in 0..cases {
            let n = rand_int(2..n);

            let mut edges = vec![];
            for u in 0..n {
                for v in 0..u {
                    if rand_f64() < p {
                        edges.push((u, v));
                    }
                }
            }

            assert_eq!(naive(n, &edges), iterated(n, &edges));
        }
    }

    test_case(100, 10, 0.1);
    test_case(100, 10, 0.3);
    test_case(100, 10, 0.5);
    test_case(100, 10, 0.8);
    test_case(5, 18, 0.5);
}
