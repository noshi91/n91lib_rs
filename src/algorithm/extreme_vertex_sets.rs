/*

Reference

[1] Minimizing Symmetric Submodular Functions, Satoru Iwata
    http://www.iasi.cnr.it/~ventura/Cargese13/Lectures%20slides/Iwata4.pdf
    

Description

G: 非負重み付き無向グラフ
n: |V(G)|
m: |E(G)|

G の extreme vertex set を全て列挙する。

時間計算量: O(n(m + n log(n)))

f(S) をカット (S, V(G) \ S) の重みとする。
∀T \subsetneq S. f(T) > f(S) を満たす時、S を extreme vertex set と呼ぶ。
extreme vertex set の族は overlap-free である。

最小次数の頂点を選び削除することを繰り返す。
u, v を最後に 2 つ残った頂点、S を extreme vertex set とすると
|S \cap {u, v}| = 1 ⇒ |S| = 1
が成立する。
従って、u, v を 1 つにまとめてアルゴリズムを繰り返すことで、
extreme vertex set の候補を列挙することが出来る。

*/

use crate::other::algebraic::{zero, Zero};
use std::ops::{Add, AddAssign, SubAssign};

pub fn extreme_vertex_sets<T>(n: usize, edges: &[(usize, usize, T)]) -> Vec<Vec<usize>>
where
    T: Ord + Add<Output = T> + AddAssign + SubAssign + Zero + Clone,
{
    if n <= 1 {
        return vec![];
    }

    let mut res: Vec<_> = (0..n).map(|v| vec![v]).collect();
    struct Data<T> {
        edges: Vec<(usize, T)>,
        set: Vec<usize>,
        cost: T,
    }
    impl<T> Data<T>
    where
        T: Add<Output = T> + Zero + Clone,
    {
        fn degree(&self) -> T {
            self.edges
                .iter()
                .map(|&(_, ref c)| c)
                .fold(zero(), |b, c| b + c.clone())
        }
    }
    let mut data: Vec<Data<T>> = (0..n)
        .map(|v| Data {
            edges: vec![],
            set: vec![v],
            cost: zero(),
        })
        .collect();
    {
        let mut add = |u: usize, v: usize, c: &T| {
            data[u].edges.push((v, c.clone()));
            data[u].cost += c.clone();
        };
        for &(u, v, ref c) in edges {
            add(u, v, c);
            add(v, u, c);
        }
    }
    while data.len() != 2 {
        use crate::other::CmpByKey;

        let n = data.len();
        let mut heap = crate::data_structure::FibonacciHeap::new();
        let handle: Vec<_> = data
            .iter()
            .enumerate()
            .map(|(v, d)| heap.push(CmpByKey(d.degree(), v)))
            .collect();
        let mut rem = data.len();
        while rem != 2 {
            let CmpByKey(_, v) = *heap.pop().unwrap().borrow();
            rem -= 1;
            for &(u, ref c) in &data[v].edges {
                heap.decrease_key(&handle[u], |&mut CmpByKey(ref mut d, _)| {
                    *d -= c.clone();
                });
            }
        }
        let CmpByKey(_, u) = *heap.pop().unwrap().borrow();
        let CmpByKey(_, v) = *heap.pop().unwrap().borrow();
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        let map: Vec<usize> = (0..n)
            .map(|i| {
                if i == v {
                    u
                } else if i == n - 1 {
                    v
                } else {
                    i
                }
            })
            .collect();
        {
            let mut v_d = data.swap_remove(v);
            data[u].edges.append(&mut v_d.edges);
            data[u].set.append(&mut v_d.set);
            data[u].cost = std::cmp::min(data[u].cost.clone(), v_d.cost);
        }
        for (w, d) in data.iter_mut().enumerate() {
            d.edges = std::mem::take(&mut d.edges)
                .into_iter()
                .filter_map(|(mut t, c)| {
                    t = map[t];
                    if t == w {
                        None
                    } else {
                        Some((t, c))
                    }
                })
                .collect();
        }
        let new_cut = data[u].degree();
        if new_cut < data[u].cost {
            data[u].cost = new_cut;
            res.push(data[u].set.clone());
        }
    }

    res
}

#[test]
fn test_extreme_vertex_sets() {
    fn naive(n: usize, edges: &[(usize, usize, u64)]) -> u64 {
        use crate::other::bit::access;

        let cutsize = |s| -> u64 {
            edges
                .iter()
                .filter_map(|&(u, v, c)| {
                    if access(s, u) != access(s, v) {
                        Some(c)
                    } else {
                        None
                    }
                })
                .sum()
        };
        (1..1 << n - 1).map(cutsize).min().unwrap()
    }

    fn evs_cut(n: usize, edges: &[(usize, usize, u64)]) -> u64 {
        let cut_size = |set: Vec<usize>| -> u64 {
            let mut color = vec![false; n];
            for i in set {
                color[i] = true;
            }
            edges
                .iter()
                .filter_map(|&(u, v, c)| if color[u] != color[v] { Some(c) } else { None })
                .sum()
        };
        extreme_vertex_sets(n, edges)
            .into_iter()
            .map(cut_size)
            .min()
            .unwrap()
    }

    fn test_case(cases: usize, n: usize, p: f64, low: u64, high: u64) {
        use crate::other::rand::{rand_f64, rand_int};

        for _ in 0..cases {
            let n = rand_int(2..n);

            let mut edges = vec![];
            for u in 0..n {
                for v in 0..u {
                    if rand_f64() < p {
                        edges.push((u, v, rand_int(low..high)));
                    }
                }
            }

            assert_eq!(naive(n, &edges), evs_cut(n, &edges));
        }
    }

    let test = |low: u64, high: u64| {
        test_case(100, 10, 0.1, low, high);
        test_case(100, 10, 0.3, low, high);
        test_case(100, 10, 0.5, low, high);
        test_case(100, 10, 0.8, low, high);
        test_case(5, 18, 0.5, low, high);
    };

    test(1, 2);
    test(100, 140);
}
