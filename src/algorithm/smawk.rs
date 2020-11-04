/*

References

[1] Aggarwal, A., Klawe, M. M., Moran, S., Shor, P., & Wilber, R. (1987).
    Geometric applications of a matrix-searching algorithm.
    Algorithmica, 2(1-4), 195-208.

[2] Totally Monotone Matrix Searching (SMAWK algorithm) - 週刊 spaghetti_source - TopCoder部
    https://topcoder-g-hatena-ne-jp.jag-icpc.org/spaghetti_source/20120923/1348327542.html


Description

A: n × m 行列, totally monotone

A の各行について min を計算する。

時間計算量 Θ(n + m)

A を陽に受け取ると時間計算量が悪化してしまうので、
実際には A の要素を O(1) で取得できるような
オラクルを受け取る。


Detail

f(r, c0, c1): A[r][c0] > A[r][c1] に対応する関数

A は totally monotone (単調非減少の方)
返り値は argmin

*/

pub fn smawk<F>(n: usize, m: usize, f: F) -> Box<[usize]>
where
    F: Fn(usize, usize, usize) -> bool,
{
    smawk_internal(&(0..n).collect::<Vec<_>>(), &(0..m).collect::<Vec<_>>(), &f)
}

fn smawk_internal<F>(rows: &[usize], cols: &[usize], f: &F) -> Box<[usize]>
where
    F: Fn(usize, usize, usize) -> bool,
{
    if rows.is_empty() {
        return vec![].into_boxed_slice();
    }

    let red_rows = rows
        .iter()
        .skip(1)
        .step_by(2)
        .copied()
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let mut red_cols = vec![];
    for &c in cols {
        while let Some(top) = red_cols.pop() {
            if !f(red_rows[red_cols.len()], top, c) {
                red_cols.push(top);
                break;
            }
        }
        if red_cols.len() < red_rows.len() {
            red_cols.push(c);
        }
    }

    let red_res = smawk_internal(&red_rows, &red_cols, f);

    let mut res = vec![0; rows.len()].into_boxed_slice();
    for (res, red_res) in res.iter_mut().skip(1).step_by(2).zip(red_res.into_vec()) {
        *res = red_res;
    }

    let mut c = 0;

    for (i, &r) in rows.iter().enumerate().step_by(2) {
        let &end = res.get(i + 1).unwrap_or(cols.last().unwrap());
        res[i] = cols[c];
        while cols[c] != end {
            c += 1;
            if f(r, res[i], cols[c]) {
                res[i] = cols[c];
            }
        }
    }

    res
}

#[test]
fn test_smawk() {
    use crate::other::rand::rand_int;

    fn linear(s: i64, n: usize, m: usize, i: usize, j: usize) -> i64 {
        if j + 1 == m {
            rand_int(-s..s)
        } else if i == 0 {
            rand_int(-s * n as i64 / 2..1)
        } else {
            rand_int(0..s)
        }
    }

    fn quadratic(s: i64, n: usize, m: usize, i: usize, j: usize) -> i64 {
        if j + 1 == m {
            rand_int(-s..s)
        } else if i == 0 {
            let lower = s * (-2 * j as i64 - 1);
            rand_int(lower..1)
        } else {
            let upper = (s as f64 * (2 as f64 * m as f64 / n as f64)).ceil() as i64;
            rand_int(0..upper + 1)
        }
    }

    fn test_set<D>(cases: usize, n: usize, m: usize, s: i64, dist: D)
    where
        D: Fn(i64, usize, usize, usize, usize) -> i64,
    {
        for _ in 0..cases {
            let n = rand_int(0..n);
            let m = rand_int(1..m);

            let mut a: Vec<Vec<_>> = (0..n)
                .map(|i| (0..m).map(|j| dist(s, n, m, i, j)).collect())
                .collect();

            for i in 0..n {
                for j in (1..m).rev() {
                    a[i][j - 1] += a[i][j];
                }
            }
            for i in 1..n {
                for j in 0..m {
                    a[i][j] += a[i - 1][j];
                }
            }

            for i in 1..n {
                for j in 1..m {
                    assert!(-a[i - 1][j - 1] + a[i - 1][j] + a[i][j - 1] - a[i][j] >= 0);
                }
            }

            let naive = a
                .iter()
                .map(|a| a.iter().enumerate().min_by_key(|(_, &a)| a).unwrap().0)
                .collect::<Vec<_>>()
                .into_boxed_slice();

            assert_eq!(naive, smawk(n, m, |r, c0, c1| a[r][c0] > a[r][c1]));
        }
    }

    test_set(100, 100, 100, 100, linear);
    test_set(100, 100, 100, 100, quadratic);
    test_set(10, 3, 100, 100, linear);
    test_set(10, 3, 100, 100, quadratic);
    test_set(10, 100, 4, 100, linear);
    test_set(10, 100, 4, 100, quadratic);
}
