/*

Reference

[1] Yuster, R., & Zwick, U. (1997).
    Finding even cycles even faster.
    SIAM Journal on Discrete Mathematics, 10(2), 209-222.


Description

G: 単純無向グラフ
n: |V(G)|

G の偶数長のサイクルの内、最小のものを発見する。

時間計算量: Θ(n^2)

最小の偶数長サイクルを C とすると、C にはある頂点 v
が存在して、v から C の各頂点への距離は C 上での距離
より高々 1 小さい、というのが重要な補題である。
この証明はいくらか込み入っており、それ自体興味深い命題である。

似たアルゴリズムとして、内周（最小のサイクル）の +1 近似
アルゴリズムが存在する。これは頂点から幅優先探索を行い、
BFS 木に属さない辺が見つかり次第それが作るサイクルを記録して
終了する、ということを全ての頂点について行い最小値を取るアルゴリズムである。
内周が偶数のとき、このアルゴリズムは正確に内周を計算する。
内周が奇数のとき、正確に計算出来る事も、1 大きいサイクルを
報告することもある。


Detail

edges: G の辺

復元は難しくないが、省略した。
偶数長のサイクルが存在しない場合 None 、存在する場合
最小の長さを返す。

アルゴリズムを [1] から少し変更して、マッチしている
最も低い親のマッチ先を管理するようにした。

[1] の Fig. 3.1. は Ⓗ のようなケースが抜けているように思われる。
その場合の証明もさほど難しくない。

テストがとても弱い。

*/

pub fn shortest_even_length_cycle(n: usize, edges: &[(usize, usize)]) -> Option<usize> {
    let mut adj_list = vec![vec![]; n].into_boxed_slice();
    for &(u, v) in edges {
        adj_list[u].push(v);
        adj_list[v].push(u);
    }
    let adj_list = adj_list
        .into_vec()
        .into_iter()
        .map(|list| list.into_boxed_slice())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let bfs = |root: usize| -> Option<usize> {
        let mut nodes: Box<[Option<Node>]> = vec![None; n].into_boxed_slice();

        let mut queue = crate::other::Queue::<usize>::new();
        nodes[root] = Some(Node {
            dist: 0,
            lowest_match: None,
        });
        queue.push(root);
        while let Some(v) = queue.pop() {
            let mut v_data = nodes[v].take().unwrap();
            for &u in adj_list[v].iter() {
                let node = &mut nodes[u];
                match *node {
                    None => {
                        *node = Some(Node {
                            dist: v_data.dist + 1,
                            lowest_match: None,
                        });
                    }
                    Some(ref mut data) => match data.dist as isize - v_data.dist as isize {
                        -1 => {}
                        0 => {
                            if v_data.lowest_match == Some(u) {
                                continue;
                            }
                            if v_data.lowest_match != data.lowest_match {
                                return Some(2 * v_data.dist + 2);
                            }
                            v_data.lowest_match = Some(u);
                            data.lowest_match = Some(v);
                        }
                        1 => {
                            return Some(2 * data.dist);
                        }
                        _ => unreachable!(),
                    },
                }
            }
            for &u in adj_list[v].iter() {
                let data = nodes[u].as_mut().unwrap();
                if data.dist == v_data.dist + 1 {
                    data.lowest_match = v_data.lowest_match;
                    queue.push(u);
                }
            }
            nodes[v] = Some(v_data);
        }

        None
    };

    (0..n).filter_map(bfs).min()
}

#[derive(Clone)]
struct Node {
    dist: usize,
    lowest_match: Option<usize>,
}

#[test]
fn test_shortest_even_length_cycle() {
    fn naive(n: usize, edges: &[(usize, usize)]) -> Option<usize> {
        let mut adj_mat = vec![0usize; n];
        for &(u, v) in edges {
            adj_mat[u] |= 1 << v;
            adj_mat[v] |= 1 << u;
        }

        let mut res = None;

        for root in 0..n {
            let mut dp = vec![0usize; 1 << root + 1];
            dp[1 << root] |= 1 << root;
            for s in 1 << root..1 << root + 1 {
                for v in 0..root {
                    if s >> v & 1 == 0 {
                        continue;
                    }
                    if dp[s & !(1 << v)] & adj_mat[v] != 0 {
                        dp[s] |= 1 << v;
                    }
                }
                let len = s.count_ones() as usize;
                if len % 2 == 0 && len != 2 && dp[s] & adj_mat[root] != 0 {
                    if res.map_or(true, |r| len < r) {
                        res = Some(len);
                    }
                }
            }
        }

        res
    }

    fn internal(q: usize, n: usize, p: f64) {
        use crate::other::rand::rand_f64;

        for _ in 0..q {
            let mut edges = vec![];
            for u in 0..n {
                for v in 0..u {
                    if rand_f64() < p {
                        edges.push((u, v));
                    }
                }
            }

            assert_eq!(naive(n, &edges), shortest_even_length_cycle(n, &edges));
        }
    }

    for n in 1..=5 {
        for s in 0..1usize << n * (n - 1) / 2 {
            let mut edges = vec![];
            for u in 0..n {
                for v in 0..u {
                    if s >> u * (u - 1) / 2 + v & 1 != 0 {
                        edges.push((u, v));
                    }
                }
            }
            assert_eq!(naive(n, &edges), shortest_even_length_cycle(n, &edges));
        }
    }

    internal(50, 10, 0.05);
    internal(50, 10, 0.1);
    internal(50, 10, 0.15);
    internal(50, 10, 0.2);
    internal(50, 10, 0.25);
    internal(50, 10, 0.3);
    internal(50, 10, 0.4);
    internal(50, 10, 0.7);
    internal(10, 16, 0.15);
}
