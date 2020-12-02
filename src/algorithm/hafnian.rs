/*

References

[1] Björklund, A. (2012, January).
    Counting perfect matchings as fast as Ryser.
    In Proceedings of the twenty-third annual ACM-SIAM
    symposium on Discrete Algorithms (pp. 914-921).
    Society for Industrial and Applied Mathematics.


Description

a: 対角成分が 0 の 2n × 2n 対称行列

a のハフニアンを計算する。

時間計算量 Θ(2^n n^2)
空間計算量 O(n^4)

ハフニアンは、その行列を隣接行列に持つ多重グラフの
完全マッチングの個数と等しい。

*/


use crate::other::algebraic::Ring;
use crate::other::Polynomial;
use itertools::zip;

pub fn hafnian<T>(a: &Vec<Vec<T>>) -> T
where
    T: Ring + Clone,
{
    assert_eq!(a.len() % 2, 0);
    HafnianFn { n: a.len() / 2 }.solve(a)
}

struct HafnianFn {
    n: usize,
}

impl HafnianFn {
    fn solve<T>(&self, a: &Vec<Vec<T>>) -> T
    where
        T: Ring + Clone,
    {
        self.f((0..self.n * 2)
            .map(|i| (0..i).map(|j| a[i][j].clone().into()).collect())
            .collect())[self.n]
            .clone()
    }

    fn f<T>(&self, mut b: Vec<Vec<Poly<T>>>) -> Poly<T>
    where
        T: Ring + Clone,
    {
        if b.is_empty() {
            return T::one().into();
        }

        let x = b.pop().unwrap();
        let y = b.pop().unwrap();

        let zero = self.f(b.clone());

        for (b, x) in zip(&mut b, &x) {
            for (b, y) in zip(b, &y) {
                *b += self.bound(x.clone() * y.clone() << 1);
            }
        }
        for (b, y) in zip(&mut b, &y) {
            for (b, x) in zip(b, &x) {
                *b += self.bound(x.clone() * y.clone() << 1);
            }
        }

        let all = self.f(b);

        let edge = (x.last().unwrap().clone() << 1) + T::one().into();

        self.bound(edge * all) - zero
    }

    fn bound<T>(&self, a: Poly<T>) -> Poly<T>
    where
        T: Ring,
    {
        a.bound(self.n + 1)
    }
}

type Poly<T> = Polynomial<T>;

#[test]
fn test_hafnian() {
    use crate::other::Fp;

    let make = |v: Vec<i32>| -> Vec<Fp> { v.into_iter().map(|x| x.into()).collect() };

    let a = vec![
        make(vec![0, 1, 2, 4]),
        make(vec![1, 0, 3, 5]),
        make(vec![2, 3, 0, 6]),
        make(vec![4, 5, 6, 0]),
    ];

    assert_eq!(hafnian(&a), Fp(28));
}
