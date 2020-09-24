/*

Description

時間計算量: Θ(log(h)log(w)) / query

可換モノイドの長方形領域への加算と領域の和の取得を高速に行う。
n 乗が O(1) で計算できると仮定しているが、
繰り返し二乗法を使って Θ(log(n)) で計算すると時間計算量は Θ(log(n)) 倍になる。
SegmentTree の構造なので、適当に hashmap を使うか
ポインタ木にすれば構築は O(1) になる。
ただし定数倍がすごいことになっていそうなので、実用性があるかは不明。

*/

use std::ops::Range;

enum RangeRelation {
    Disjoint,
    Contains(usize),
    Others(usize),
}

use RangeRelation::*;

fn range_relation(x: &Range<usize>, y: &Range<usize>) -> RangeRelation {
    if x.start <= y.start {
        if x.end <= y.start {
            Disjoint
        } else if x.end < y.end {
            Others(x.end - y.start)
        } else {
            Contains(y.end - y.start)
        }
    } else {
        if x.end < y.end {
            Others(x.end - x.start)
        } else if x.start < y.end {
            Others(y.end - x.start)
        } else {
            Disjoint
        }
    }
}

use crate::other::algebraic::CommutativeMonoid;
use std::clone::Clone;
use std::ops::Mul;

pub struct RangeAddSum2d<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    data: Box<[Node<T>]>,
}

use num_traits::zero;

impl<T> RangeAddSum2d<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            data: vec![Node::new(w); h.next_power_of_two() * 2].into_boxed_slice(),
        }
    }

    pub fn sum(&self, x: Range<usize>, y: Range<usize>) -> T {
        self.sum_(&x, &y, 1, 0..self.data.len() / 2)
    }

    fn sum_(&self, x: &Range<usize>, y: &Range<usize>, k: usize, c: Range<usize>) -> T {
        match range_relation(x, &c) {
            Disjoint => zero(),
            Contains(len) => self.data[k].sum.sum(y) + self.data[k].add.sum(y) * len,
            Others(len) => {
                let mid = (c.start + c.end) / 2;
                self.data[k].add.sum(y) * len
                    + self.sum_(x, y, k * 2, c.start..mid)
                    + self.sum_(x, y, k * 2 + 1, mid..c.end)
            }
        }
    }

    pub fn add(&mut self, x: Range<usize>, y: Range<usize>, value: &T) {
        self.add_(&x, &y, value, 1, 0..self.data.len() / 2);
    }

    fn add_(&mut self, x: &Range<usize>, y: &Range<usize>, value: &T, k: usize, c: Range<usize>) {
        match range_relation(x, &c) {
            Disjoint => {}
            Contains(_) => {
                self.data[k].add.add(y, value);
            }
            Others(len) => {
                self.data[k].sum.add(y, &(value.clone() * len));
                let mid = (c.start + c.end) / 2;
                self.add_(x, y, value, k * 2, c.start..mid);
                self.add_(x, y, value, k * 2 + 1, mid..c.end);
            }
        }
    }
}

#[derive(Clone)]
struct Node<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    sum: InnerTree<T>,
    add: InnerTree<T>,
}

impl<T> Node<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    fn new(w: usize) -> Self {
        Self {
            sum: InnerTree::new(w),
            add: InnerTree::new(w),
        }
    }
}

#[derive(Clone)]
struct InnerTree<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    data: Box<[InnerNode<T>]>,
}

#[derive(Clone)]
struct InnerNode<T> {
    sum: T,
    add: T,
}

impl<T> InnerTree<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    fn new(w: usize) -> Self {
        Self {
            data: vec![InnerNode::new(); w.next_power_of_two() * 2].into_boxed_slice(),
        }
    }

    fn sum(&self, y: &Range<usize>) -> T {
        self.sum_(y, 1, 0..self.data.len() / 2)
    }

    fn sum_(&self, y: &Range<usize>, k: usize, c: Range<usize>) -> T {
        match range_relation(y, &c) {
            Disjoint => zero(),
            Contains(len) => self.data[k].sum.clone() + self.data[k].add.clone() * len,
            Others(len) => {
                let mid = (c.start + c.end) / 2;
                self.data[k].add.clone() * len
                    + self.sum_(y, k * 2, c.start..mid)
                    + self.sum_(y, k * 2 + 1, mid..c.end)
            }
        }
    }

    fn add(&mut self, y: &Range<usize>, value: &T) {
        self.add_(y, value, 1, 0..self.data.len() / 2);
    }

    fn add_(&mut self, y: &Range<usize>, value: &T, k: usize, c: Range<usize>) {
        match range_relation(y, &c) {
            Disjoint => {}
            Contains(_) => {
                self.data[k].add += value.clone();
            }
            Others(len) => {
                self.data[k].sum += value.clone() * len;
                let mid = (c.start + c.end) / 2;
                self.add_(y, value, k * 2, c.start..mid);
                self.add_(y, value, k * 2 + 1, mid..c.end);
            }
        }
    }
}

impl<T> InnerNode<T>
where
    T: CommutativeMonoid + Mul<usize, Output = T> + Clone,
{
    fn new() -> Self {
        Self {
            sum: zero(),
            add: zero(),
        }
    }
}

#[test]
fn test_range_add_sum_2d() {
    use crate::other::rand::{rand_int, rand_range, random};
    use crate::other::{fp::P, Fp};

    let h_max = 100;
    let w_max = 100;
    let q = 100;

    for _ in 0..q {
        let h = rand_int(0..h_max);
        let w = rand_int(0..w_max);
        let q = 100;

        let mut a = RangeAddSum2d::<Fp>::new(h, w);
        let mut b = vec![vec![Fp(0); w]; h];

        for _ in 0..q {
            let x = rand_range(0..h);
            let y = rand_range(0..w);
            if random() {
                let ans = b[x.clone()]
                    .iter()
                    .map(|b| b[y.clone()].iter().copied().sum())
                    .sum();
                assert_eq!(a.sum(x, y), ans);
            } else {
                let v = Fp(rand_int(0..P));
                a.add(x.clone(), y.clone(), &v);
                for b in &mut b[x.clone()] {
                    for b in &mut b[y.clone()] {
                        *b += v;
                    }
                }
            }
        }
    }
}
