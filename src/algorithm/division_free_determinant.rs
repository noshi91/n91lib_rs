/*

Reference

[1] Rote, G. (2001). Division-free algorithms for the determinant and the pfaffian:
    algebraic and combinatorial approaches.
    In Computational discrete mathematics (pp. 119-135).Springer, Berlin, Heidelberg.


Description

T: 可換環
a: T 上の n×n 行列

a の行列式を計算する。

時間計算量: Θ(n^4)

行列式の定義自体は除算を用いずに行われる。
したがって、除算を使わずに行列式を計算することも興味の対象となる。

行列式自体は有向サイクルによるカバー全体の和として解釈できるが、
適切に定義された closed walk の集合の和を取っても
上手く重複が相殺することが示せる。
頂点の重複が許されたことで記憶するべき状態が大幅に削減され、
動的計画法で効率的に計算可能になる。

より高速なアルゴリズムも存在するらしい。

*/

use crate::other::algebraic::{one, zero, CommutativeRing};

pub fn division_free_determinant<T>(a: &Vec<Vec<T>>) -> T
where
    T: CommutativeRing + Clone,
{
    let n = a.len();
    for v in a {
        assert_eq!(v.len(), n);
    }

    let mut dp: Vec<Vec<T>> = vec![vec![zero(); n + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][i] = one();
    }

    for _ in 0..n {
        let mut nx = vec![vec![zero(); n + 1]; n + 1];
        for h in 0..n {
            for c in h..n {
                for v in h + 1..n {
                    nx[h][v] += dp[h][c].clone() * -a[c][v].clone();
                }
                let t = dp[h][c].clone() * a[c][h].clone();
                for v in h + 1..n + 1 {
                    nx[v][v] += t.clone();
                }
            }
        }
        dp = nx;
    }

    dp[n][n].clone()
}

#[test]
fn test_division_free_determinant() {
    use crate::algorithm::determinant;
    use crate::other::rand::{rand_int, random};
    use crate::other::Fp;

    let q = 100;
    let n = 10;
    for _ in 0..q {
        let n = rand_int(0..n);
        let a: Vec<Vec<Fp>> = (0..n).map(|_| (0..n).map(|_| random()).collect()).collect();
        assert_eq!(determinant(a.clone()), division_free_determinant(&a));
    }

    let q = 10;
    let n = 10;
    for _ in 0..q {
        let n = rand_int(1..n);
        let mut a: Vec<Vec<Fp>> = (0..n).map(|_| (0..n).map(|_| random()).collect()).collect();
        {
            let mut v = vec![Fp(0); n];
            for i in 0..n - 1 {
                let c: Fp = random();
                for j in 0..n {
                    v[j] += c * a[i][j];
                }
            }
            a[n - 1] = v;
        }
        assert_eq!(division_free_determinant(&a), Fp(0));
    }
}
