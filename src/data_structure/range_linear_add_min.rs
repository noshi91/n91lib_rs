/*

Reference

[1] https://twitter.com/yosupot/status/1104177804168626177

[2] Overmars, M. H., & Van Leeuwen, J. (1981).
    Maintenance of configurations in the plane.
    Journal of computer and System Sciences, 23(2), 166-204.


Description

d: 管理する数列
n: |d|

add(range, a, b): 各 i \in range について、d_i ← d_i + ai + b
min(range): min_{i \in range} d_i を計算する

時間計算量
add: Θ(log(n)^2)
min: Θ(log(n)^2)

(i, d_i) を点群とみなすと、min を取り得るのは lower hull のみである。
lower hull 同士のマージは [2] にある方法を用いて Θ(log(n)) で行うことが出来る。
正確に言えば、マージした場合の境目の点を計算するだけである。

更新クエリは可換なので遅延伝播はしてもしなくてもよい。
この実装は伝播させている。

*/

pub struct RangeLinearAddMin {
    internal: Vec<Node>,
    leaf: Vec<i64>,
}

#[derive(Clone, Default)]
struct Node {
    left: Point,
    right: Point,
    lazy: Linear,
}

#[derive(Clone)]
struct Linear(i64, i64);

#[derive(Clone, Default)]
struct Point(i64, i64);

use crate::other::bit::{bsf, bsr};
use std::ops::Range;

impl RangeLinearAddMin {
    pub fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        let mut res = Self {
            internal: vec![Node::default(); n],
            leaf: vec![0; n],
        };
        for i in (1..n).rev() {
            res.update_node(i);
        }
        res
    }

    pub fn min(&mut self, Range { start, end }: Range<usize>) -> i64 {
        let n = self.len();
        let (mut st, mut en) = (start + n, end + n);
        self.propagate(st);
        self.propagate(en);
        let mut res = i64::MAX;
        while st != en {
            use crate::other::cmp_assign::MinAssign;

            if st % 2 != 0 {
                res.min_assign(self.min_subtree(st));
                st += 1;
            }
            st /= 2;
            if en % 2 != 0 {
                en -= 1;
                res.min_assign(self.min_subtree(en));
            }
            en /= 2;
        }
        res
    }

    pub fn add(&mut self, Range { start, end }: Range<usize>, a: i64, b: i64) {
        let op = Linear(b, a);
        let n = self.len();
        let (start, end) = (start + n, end + n);
        let (mut st, mut en) = (start, end);
        self.propagate(st);
        self.propagate(en);
        while st != en {
            if st % 2 != 0 {
                self.add_node(st, &op);
                st += 1;
            }
            st /= 2;
            if en % 2 != 0 {
                en -= 1;
                self.add_node(en, &op);
            }
            en /= 2;
        }
        self.update(start);
        self.update(end);
    }

    fn len(&self) -> usize {
        self.internal.len()
    }

    fn add_node(&mut self, i: usize, op: &Linear) {
        let n = self.len();
        if i < n {
            let d = &mut self.internal[i];
            d.left *= op;
            d.right *= op;
            d.lazy *= op;
        } else {
            let i = i - n;
            self.leaf[i] += i as i64 * op.1 + op.0;
        }
    }

    fn min_subtree(&mut self, mut i: usize) -> i64 {
        let n = self.len();
        while i < n {
            self.push(i);
            if self.internal[i].left.1 < self.internal[i].right.1 {
                i = i * 2;
            } else {
                i = i * 2 + 1;
            }
        }
        self.leaf[i - n]
    }

    fn push(&mut self, i: usize) {
        let lazy = std::mem::take(&mut self.internal[i].lazy);
        self.add_node(i * 2, &lazy);
        self.add_node(i * 2 + 1, &lazy);
    }

    fn propagate(&mut self, i: usize) {
        for h in (bsf(i) + 1..bsr(i) + 1).rev() {
            self.push(i >> h);
        }
    }

    fn get_leaf(&self, i: usize) -> Point {
        Point(i as i64, self.leaf[i])
    }

    fn update_node(&mut self, i: usize) {
        let n = self.len();
        let m = {
            let mut j = i * 2 + 1;
            while j < n {
                j = j * 2;
            }
            (j - n) as i64
        };
        let (mut l, mut r) = (i * 2, i * 2 + 1);
        loop {
            match (l < n, r < n) {
                (false, false) => {
                    break;
                }
                (false, true) => {
                    self.push(r);
                    if is_concave(
                        &self.get_leaf(l - n),
                        &self.internal[r].left,
                        &self.internal[r].right,
                    ) {
                        r = r * 2 + 1;
                    } else {
                        r = r * 2;
                    }
                }
                (true, false) => {
                    self.push(l);
                    if is_concave(
                        &self.internal[l].left,
                        &self.internal[l].right,
                        &self.get_leaf(r - n),
                    ) {
                        l = l * 2;
                    } else {
                        l = l * 2 + 1;
                    }
                }
                (true, true) => {
                    if is_concave(
                        &self.internal[l].left,
                        &self.internal[l].right,
                        &self.internal[r].left,
                    ) {
                        self.push(l);
                        l = l * 2;
                    } else if is_concave(
                        &self.internal[l].right,
                        &self.internal[r].left,
                        &self.internal[r].right,
                    ) {
                        self.push(r);
                        r = r * 2 + 1;
                    } else if is_left_cross(
                        &self.internal[l].left,
                        &self.internal[l].right,
                        &self.internal[r].left,
                        &self.internal[r].right,
                        m,
                    ) {
                        self.push(l);
                        l = l * 2 + 1;
                    } else {
                        self.push(r);
                        r = r * 2;
                    }
                }
            }
        }
        self.internal[i].left = self.get_leaf(l - n);
        self.internal[i].right = self.get_leaf(r - n);
    }

    fn update(&mut self, i: usize) {
        for h in bsf(i) + 1..bsr(i) + 1 {
            self.update_node(i >> h);
        }
    }
}

fn is_concave(a: &Point, b: &Point, c: &Point) -> bool {
    c.0 * b.1 + a.0 * c.1 + b.0 * a.1 >= b.0 * c.1 + c.0 * a.1 + a.0 * b.1
}

fn is_left_cross(a: &Point, b: &Point, c: &Point, d: &Point, m: i64) -> bool {
    let (l, r) = (b.0 - a.0, d.0 - c.0);
    a.1 * r * (m - b.0) + c.1 * l * (d.0 - m) > b.1 * r * (m - a.0) + d.1 * l * (c.0 - m)
}

impl Default for Linear {
    fn default() -> Self {
        Self(0, 0)
    }
}

use std::ops::MulAssign;

impl MulAssign<&Linear> for Linear {
    fn mul_assign(&mut self, rhs: &Linear) {
        self.1 += rhs.1;
        self.0 += rhs.0;
    }
}

impl MulAssign<&Linear> for Point {
    fn mul_assign(&mut self, rhs: &Linear) {
        self.1 = self.1 + self.0 * rhs.1 + rhs.0;
    }
}

#[test]
fn test_range_linear_add_min() {
    struct Naive {
        d: Vec<i64>,
    }

    impl Naive {
        fn new(n: usize) -> Self {
            Naive { d: vec![0; n] }
        }

        fn add(&mut self, range: Range<usize>, a: i64, b: i64) {
            for i in range {
                self.d[i] += i as i64 * a + b;
            }
        }

        fn min(&self, range: Range<usize>) -> i64 {
            *self.d[range].iter().min().unwrap()
        }
    }

    fn testset(cases: usize, n: usize, a: i64, b: i64, q: usize) {
        use crate::other::rand::{rand_int, rand_range, rand_range_nonempty};

        for _ in 0..cases {
            let n = rand_int(1..n);
            let q = rand_int(0..q);
            let mut rlam = RangeLinearAddMin::new(n);
            let mut naive = Naive::new(n);
            for _ in 0..q {
                if rand_int(0..2) == 0 {
                    let r = rand_range(0..n);
                    let a = rand_int(-a..a + 1);
                    let b = rand_int(-b..b + 1);
                    rlam.add(r.clone(), a, b);
                    naive.add(r, a, b);
                } else {
                    let r = rand_range_nonempty(0..n);
                    assert_eq!(rlam.min(r.clone()), naive.min(r));
                }
            }
        }
    }

    testset(100, 10, 10, 30, 100);
    testset(100, 10, 0, 3, 100);
    testset(100, 10, 1000, 10000, 100);
    testset(10, 1000, 1000, 500000, 1000);
}
