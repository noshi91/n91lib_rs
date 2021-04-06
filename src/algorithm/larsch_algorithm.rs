/*

Reference

[1] Larmore, L. L., & Schieber, B. (1991).
    On-line dynamic programming with applications to the prediction of RNA secondary structure.
    Journal of Algorithms, 12(3), 490-515.


Description

f: n × n 下三角 totally monotone 行列

各 i について m(i) := argmin_j f(i, j) を計算する。
m(0), m(1), ... の順に計算され、f(_, j+1) へのアクセスが
m(j) の計算後であることが保証される。

時間計算量: Θ(n)

LARSCH Algorithm はある種の動的計画法の高速化に利用することができる。
例えば長さ n の列がありその非空な区間に重み w: N × N → R が定まっているとする。
w が Monge 性を満たす時、列を任意個の非空な区間に分割するときの最小重みは
動的計画法 dp[i] = min dp[j] + w(j, i) で計算することができる。
これは f(i, j) = dp[j] + w(j, i+1) と定義することで
LARSCH Algorithm の解く問題に帰着される。

LARSCH Algorithm の解く問題は SMAWK Algorithm の解く問題を online な状況に
したものと言える。実際、アルゴリズムの内容も SMAWK Algorithm と類似している。


Details

LARSCH Algorithm の状況を表現するために、fn ではなく struct の形になっている。
Larsch::new(n) で作成し、get_argmin(f) を呼ぶ度に次の行の argmin を計算し返す。

*/

pub struct Larsch(ReduceRow);

impl Larsch {
    pub fn new(n: usize) -> Self {
        Self(ReduceRow::new(n))
    }

    pub fn get_argmin<F, T>(&mut self, f: F) -> usize
    where
        F: FnMut(usize, usize) -> T,
        T: Ord,
    {
        self.0.get_argmin(f)
    }
}

struct ReduceRow {
    n: usize,
    cur_row: usize,
    state: usize,
    rec: Option<Box<ReduceCol>>,
}

impl ReduceRow {
    fn new(n: usize) -> Self {
        Self {
            n,
            cur_row: 0,
            state: 0,
            rec: match n / 2 {
                0 => None,
                m => Some(Box::new(ReduceCol::new(m))),
            },
        }
    }

    fn get_argmin<F, T>(&mut self, mut f: F) -> usize
    where
        F: FnMut(usize, usize) -> T,
        T: Ord,
    {
        let cur_row = self.cur_row;
        self.cur_row += 1;
        if cur_row % 2 == 0 {
            let prev_argmin = self.state;
            let next_argmin = if cur_row + 1 == self.n {
                self.n - 1
            } else {
                self.rec
                    .as_mut()
                    .unwrap()
                    .get_argmin(&mut |i, j| f(2 * i + 1, j))
            };
            self.state = next_argmin;
            (prev_argmin..=next_argmin)
                .min_by_key(|&j| f(cur_row, j))
                .unwrap()
        } else {
            if f(cur_row, self.state) <= f(cur_row, cur_row) {
                self.state
            } else {
                cur_row
            }
        }
    }
}

struct ReduceCol {
    n: usize,
    cur_row: usize,
    cols: Vec<usize>,
    rec: ReduceRow,
}

impl ReduceCol {
    fn new(n: usize) -> Self {
        Self {
            n,
            cur_row: 0,
            cols: vec![],
            rec: ReduceRow::new(n),
        }
    }

    fn get_argmin<T>(&mut self, f: &mut dyn FnMut(usize, usize) -> T) -> usize
    where
        T: Ord,
    {
        let cur_row = self.cur_row;
        self.cur_row += 1;
        for j in if cur_row == 0 {
            vec![0]
        } else {
            vec![2 * cur_row - 1, 2 * cur_row]
        } {
            while {
                let len = self.cols.len();
                len != cur_row && f(len - 1, *self.cols.last().unwrap()) > f(len - 1, j)
            } {
                self.cols.pop();
            }
            if self.cols.len() != self.n {
                self.cols.push(j);
            }
        }
        let cols = &self.cols;
        cols[self.rec.get_argmin(|i, j| f(i, cols[j]))]
    }
}

#[test]
fn test_larsch() {
    use crate::other::rand::rand_int;

    let testcase = |n: usize, a_max: i64| {
        let a: Vec<_> = (0..n).map(|_| rand_int(0..a_max)).collect();
        let sum = {
            let mut sum = a;
            sum.push(0);
            for i in (0..n).rev() {
                let t = sum[i + 1];
                sum[i] += t;
            }
            sum
        };

        let k = rand_int(1..n + 1);

        let lagrange = |lambda: i64| -> i64 {
            let cost = |l: usize, r: usize| -> i64 { (sum[l] - sum[r]).pow(2) + lambda };
            let naive = {
                let mut dp = vec![0; n + 1];
                for r in 1..n + 1 {
                    let t = (0..r).map(|l| dp[l] + cost(l, r)).min().unwrap();
                    dp[r] = t;
                }
                dp
            };
            let fast = {
                let mut dp = vec![0; n + 1];
                let mut larsch = Larsch::new(n);
                for r in 0..n {
                    let t = {
                        let f = |i, j| {
                            assert!(i >= j);
                            assert!(r >= j);
                            dp[j] + cost(j, i + 1)
                        };
                        let m = larsch.get_argmin(f);
                        f(r, m)
                    };
                    dp[r + 1] = t;
                }
                dp
            };
            assert_eq!(naive, fast);
            naive[n] + lambda * k as i64
        };

        let mut l = 0;
        let mut r = 3 * (n as i64 * a_max).pow(2);
        while r - l > 2 {
            let ll = (l + l + r) / 3;
            let rr = (l + r + r) / 3;

            if lagrange(ll) < lagrange(rr) {
                l = ll;
            } else {
                r = rr;
            }
        }
    };

    for _ in 0..10 {
        testcase(rand_int(1..100), 1000);
    }
}
