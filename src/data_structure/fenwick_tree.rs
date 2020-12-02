use crate::other::algebraic::{zero, CommutativeMonoid};

#[derive(Clone)]
pub struct FenwickTree<T>
where
    T: CommutativeMonoid + Clone,
{
    data: Box<[T]>,
}

impl<T> FenwickTree<T>
where
    T: CommutativeMonoid + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![zero(); n + 1].into_boxed_slice(),
        }
    }

    pub fn fold_prefix(&self, mut end: usize) -> T {
        let mut ret = zero();
        while end != 0 {
            ret += self.data[end].clone();
            end &= end - 1;
        }
        ret
    }

    pub fn add(&mut self, mut index: usize, value: &T) {
        index += 1;
        while index < self.data.len() {
            self.data[index] += value.clone();
            index += index & !index + 1;
        }
    }
}

use crate::other::algebraic::Abelian;
use std::ops::Range;

impl<T> FenwickTree<T>
where
    T: Abelian + Clone,
{
    pub fn fold(&self, range: Range<usize>) -> T {
        -self.fold_prefix(range.start) + self.fold_prefix(range.end)
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.add(index, &(-self.fold(index..index + 1) + value));
    }
}

#[test]
fn test_fenwick_tree() {
    use crate::other::rand::{rand_int, rand_range};
    use crate::other::{fp::P, Fp};

    let n_max = 100;
    let q = 100;

    for _ in 0..q {
        let n = rand_int(1..n_max);
        let q = 100;

        let mut ft = FenwickTree::new(n);
        let mut v = vec![Fp(0); n].into_boxed_slice();
        for _ in 0..q {
            match rand_int(0..4) {
                0 => {
                    let end = rand_int(0..n + 1);
                    assert_eq!(ft.fold_prefix(end), v[..end].iter().copied().sum());
                }
                1 => {
                    let index = rand_int(0..n);
                    let x = Fp(rand_int(0..P));
                    ft.add(index, &x);
                    v[index] += x;
                }
                2 => {
                    let range = rand_range(0..n);
                    assert_eq!(ft.fold(range.clone()), v[range].iter().copied().sum());
                }
                3 => {
                    let index = rand_int(0..n);
                    let x = Fp(rand_int(0..P));
                    ft.set(index, x);
                    v[index] = x;
                }
                _ => unreachable!(),
            }
        }
    }
}

/*

Description

愚直なので遅い。
内部を HashMap にすれば構築のオーダーは落ちるが、
実用するつもりがないため放置している。

*/

pub struct FenwickTree2d<T>
where
    T: CommutativeMonoid + Clone,
{
    data: Box<[FenwickTree<T>]>,
}

impl<T> FenwickTree2d<T>
where
    T: CommutativeMonoid + Clone,
{
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            data: vec![FenwickTree::new(m); n].into_boxed_slice(),
        }
    }

    pub fn fold_prefix(&self, mut x: usize, y: usize) -> T {
        let mut ret = zero();
        while x != 0 {
            ret += self.data[x].fold_prefix(y);
            x &= x - 1;
        }
        ret
    }

    pub fn add(&mut self, mut x: usize, y: usize, value: &T) {
        x += 1;
        while x < self.data.len() {
            self.data[x].add(y, value);
            x += x & !x + 1;
        }
    }
}
