/*

References

[1] Bjorklund, A. (2014).
    Determinant sums for undirected hamiltonicity.
    SIAM Journal on Computing, 43(1), 280-299.

[2] 指数時間アルゴリズムの最先端
    https://www.slideshare.net/wata_orz/ss-12208032

[3] 組合せ最適化に対する代数的厳密アルゴリズム
    京都大学数理解析研究所「組合せ最適化セミナー」(第 8 回)
    http://www.kurims.kyoto-u.ac.jp/coss/coss2011/okamoto-handout.pdf


Description

g:  部集合の大きさが等しい無向二部グラフ G の隣接行列
    g[i][j] iff (u_i, v_j) ∈ E(G)
n:  G の一方の部集合の大きさ、|G| / 2

G がハミルトンサイクルを持つか確率的に判定する。
yes を誤って no と判定する確率が 1/n 以下となる。

時間計算量: O(2^n n^3 log(n))
空間計算量: Θ(n^3)

Tutte 行列を用いて完全マッチングの存在判定をするアルゴリズムを変形し、
片方の部集合をラベルにし、更に頂点 0 の周りでの対称性を壊す。
標数 2 で要素数の十分大きい体 GF(2^m) 上で考えることで、
丁度ハミルトンサイクル以外を上手くキャンセルさせることが出来る。
実用的には n = 20 程度が限界で m = 30 と取っているため、
誤判定の確率は 2^{-25} 以下でありとても頑強。
GF(2^m) 上の乗算を O(log(n)) としたが、CLMUL を O(1) として
疎な法の存在を仮定するならば、時間計算量から log(n) の項は消える。


*/

pub fn bipartite_hamiltonian_cycle(g: &Vec<Vec<bool>>) -> bool {
    let n = g.len();
    assert_ne!(n, 0);
    for v in g {
        assert_eq!(v.len(), n);
    }

    use crate::algorithm::determinant;
    use crate::other::rand::random;
    use crate::other::GF2m;

    let f: Vec<_> = (0..n)
        .map(|l| {
            let mut a = vec![vec![GF2m(0); n]; n];
            for i in 1..n {
                if g[0][l] && g[i][l] {
                    a[0][i] = random();
                    a[i][0] = random();
                }
            }
            for i in 1..n {
                for j in 1..i {
                    if g[i][l] && g[j][l] {
                        let x = random();
                        a[i][j] = x;
                        a[j][i] = x;
                    }
                }
            }
            a
        })
        .collect();

    let mut sum = GF2m(0);
    let mut a = vec![vec![GF2m(0); n]; n];
    for s in 0..1usize << n {
        if s != 0 {
            let b = &f[s.trailing_zeros() as usize];
            for (a, b) in a.iter_mut().zip(b) {
                for (a, b) in a.iter_mut().zip(b) {
                    *a += *b;
                }
            }
        }
        sum += determinant(a.clone());
    }

    sum.0 != 0
}

#[test]
fn test_bipartite_hamiltonian_cycle() {
    fn naive(g: &Vec<Vec<bool>>) -> bool {
        let n = g.len();
        if n == 1 {
            return false;
        }
        let mut adj = vec![0usize; n * 2];
        for u in 0..n {
            for v in 0..n {
                if g[u][v] {
                    adj[u] |= 1 << n + v;
                    adj[n + v] |= 1 << u;
                }
            }
        }

        let n = n * 2;
        let mut dp = vec![0usize; 1 << n];
        dp[1] = 1;
        for s in 0usize..1 << n {
            for v in 0..n {
                if s >> v & 1 != 0 && dp[s & !(1 << v)] & adj[v] != 0 {
                    dp[s] |= 1 << v;
                }
            }
        }

        dp.last().unwrap() & adj[0] != 0
    }

    fn internal(q: usize, n: usize, p: f64) {
        use crate::other::rand::rand_f64;

        for _ in 0..q {
            let g: Vec<Vec<_>> = (0..n)
                .map(|_| (0..n).map(|_| rand_f64() < p).collect())
                .collect();

            assert_eq!(naive(&g), bipartite_hamiltonian_cycle(&g));
        }
    }

    for n in 1..=3 {
        for s in 0..1usize << n * n {
            let mut g = vec![vec![false; n]; n];
            for i in 0..n {
                for j in 0..n {
                    if s >> i * n + j & 1 != 0 {
                        g[i][j] = true;
                    }
                }
            }
            assert_eq!(naive(&g), bipartite_hamiltonian_cycle(&g));
        }
    }

    internal(100, 4, 0.6);
    internal(100, 5, 0.7);
    internal(100, 5, 0.6);
    internal(100, 5, 0.5);
    internal(8, 8, 0.5);
    internal(1, 6, 0.0);
    internal(1, 6, 1.0);
}
