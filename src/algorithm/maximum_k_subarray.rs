/*

References

[1] Bengtsson, F., & Chen, J. (2007).
    Computing maximum-scoring segments optimally.
    Luleå tekniska universitet.


Description

a: 長さ n の実数列
k: 正整数

a の互いに交わらない k 個の連続部分列であって、
総和が最大となるものの総和を計算する。

時間計算量 Θ(n) expected

連続部分列は空でもよいとしたが、非空の場合も
適当に前処理をすると同じ計算量で計算可能。
[1] のアルゴリズムをいくらかアレンジし、
merge が行き過ぎないように調整した。
quick select を median of medians などに変えれば
計算量は expected から worst になる。

*/

use crate::other::linked_list::{Cursor, LinkedList};
use num_traits::{zero, Zero};
use std::clone::Clone;
use std::cmp::Ord;
use std::ops::{Add, Neg};

pub fn maximum_k_subarray<T>(a: Vec<T>, k: usize) -> T
where
    T: Add<T, Output = T> + Zero + Neg<Output = T> + Ord + Clone,
{
    let base = LinkedList::<Ext<T>>::new();
    let focus = LinkedList::<Cursor<Ext<T>>>::new();

    let mut k = {
        let mut len: usize = 0;

        let mut push = |s| {
            base.push_back(s);
            focus.push_back(base.cursor_back());
            len += 1;
        };

        let mut sum = Infinity;
        let mut is_positive = false;
        for (i, a) in a.into_iter().enumerate() {
            if a >= zero() {
                let val = Finite(i, a);
                if is_positive {
                    sum = sum + val;
                } else {
                    push(sum);
                    sum = val;
                    is_positive = true;
                }
            } else {
                let val = Finite(i, -a);
                if is_positive {
                    push(sum);
                    sum = val;
                    is_positive = false;
                } else {
                    sum = sum + val;
                }
            }
        }
        if is_positive {
            push(sum);
        }
        base.push_back(Infinity);
        (len / 2).saturating_sub(k)
    };

    while k != 0 {
        let (lower, upper): (Ext<T>, Ext<T>) = {
            let mut a = vec![];
            let mut p = focus.cursor_front();

            while let Some(v) = p.current() {
                a.push(OrdCursor(v.clone()));
                p.move_next();
            }

            use crate::algorithm::quick_select;

            quick_select(&mut a, k);
            let lower = a[k].current().clone();
            let upper = if 3 * k < a.len() {
                quick_select(&mut a, 3 * k);
                a[3 * k].current().clone()
            } else {
                Infinity
            };
            (lower, upper)
        };

        let mut p = focus.cursor_front();
        while let Some(v) = p.current() {
            let t = v.current().unwrap();
            if upper < *t {
                p.remove_current();
            } else if *t < lower {
                let mut left = v.clone();
                left.move_prev();
                let mut right = v.clone();
                right.move_next();

                if t < left.current().unwrap() && t < right.current().unwrap() {
                    p.move_prev();
                    if p.current().map_or(false, |l| *l == left) {
                        p.remove_current();
                    } else {
                        p.move_next();
                    }
                    let mut sum = left.remove_current().unwrap();
                    drop(left);
                    sum = sum + (-p.remove_current().unwrap().remove_current().unwrap());
                    if p.current().map_or(false, |r| *r == right) {
                        p.remove_current();
                    }
                    sum = sum + right.remove_current().unwrap();
                    right.insert_before(sum);
                    right.move_prev();
                    p.insert_before(right);
                    p.move_prev();
                    p.move_prev();
                    if p.current().is_none() {
                        p.move_next();
                    }
                    k -= 1;
                } else {
                    p.move_next();
                }
            } else {
                p.move_next();
            }
        }
    }

    drop(focus);

    base.pop_front();
    base.into_iter()
        .step_by(2)
        .fold(zero(), |s, item| match item {
            Infinity => unreachable!(),
            Finite(_, v) => s + v,
        })
}

use std::cmp::{
    Ordering::{self, *},
    PartialOrd,
};

#[derive(PartialEq, Eq, Clone)]
enum Ext<T> {
    Infinity,
    Finite(usize, T),
}
use Ext::*;

impl<T> PartialOrd for Ext<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match *self {
            Infinity => match *other {
                Infinity => Some(Equal),
                Finite(_, _) => Some(Greater),
            },
            Finite(i, ref u) => match *other {
                Infinity => Some(Less),
                Finite(j, ref v) => match u.partial_cmp(v) {
                    Some(Equal) => Some(i.cmp(&j)),
                    other => other,
                },
            },
        }
    }
}

impl<T> Ord for Ext<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Infinity => match *other {
                Infinity => Equal,
                Finite(_, _) => Greater,
            },
            Finite(i, ref u) => match *other {
                Infinity => Less,
                Finite(j, ref v) => match u.cmp(v) {
                    Equal => i.cmp(&j),
                    other => other,
                },
            },
        }
    }
}

impl<T> Add for Ext<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match self {
            Infinity => Infinity,
            Finite(i, u) => match rhs {
                Infinity => Infinity,
                Finite(_, v) => Finite(i, u + v),
            },
        }
    }
}

impl<T> Neg for Ext<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Infinity => unreachable!(),
            Finite(i, a) => Finite(i, -a),
        }
    }
}

struct OrdCursor<T>(Cursor<T>);

impl<T> OrdCursor<T> {
    fn current(&self) -> &T {
        self.0.current().unwrap()
    }
}

impl<T> PartialEq for OrdCursor<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.current().eq(other.current())
    }
}

impl<T> Eq for OrdCursor<T> where T: Eq {}

impl<T> PartialOrd for OrdCursor<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.current().partial_cmp(other.current())
    }
}

impl<T> Ord for OrdCursor<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.current().cmp(other.current())
    }
}

#[test]
fn test_maximum_k_subarray() {
    use crate::other::rand::rand_int;
    use std::cmp::max;

    fn test_internal(q: usize, n: usize, s: i64, k: usize) {
        for _ in 0..q {
            let n = rand_int(0..n);
            let k = rand_int(0..k);
            let a: Vec<_> = (0..n).map(|_| rand_int(-s..s + 1)).collect();

            let naive = {
                let mut sum = vec![0; n + 1];
                for i in 0..n {
                    sum[i + 1] = sum[i] + a[i];
                }
                let mut dp = vec![0; n + 1];
                for _ in 0..k {
                    let mut acc = 0;
                    for i in 1..n + 1 {
                        acc = max(acc, dp[i] - sum[i]);
                        dp[i] = max(dp[i - 1], acc + sum[i]);
                    }
                }
                dp[n]
            };
            let res = maximum_k_subarray(a, k);
            assert_eq!(res, naive);
        }
    }

    test_internal(100, 100, 10, 100);
    test_internal(100, 100, 100, 100);
    test_internal(100, 100, 10000, 100);
    test_internal(100, 100, 100, 10);
    test_internal(10, 10, 10, 10);
}
