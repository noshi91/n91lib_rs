use crate::other::algebraic::{Abelian, CommutativeMonoid, Group, Monoid, Semiring};
use itertools::{enumerate, zip};
use num_traits::{zero, Zero};
use std::clone::Clone;
use std::convert::From;
use std::ops::{Add, AddAssign, Mul, Neg, Shl, Sub};

#[derive(Clone)]
pub struct Polynomial<T>
where
    T: Monoid,
{
    pub coef: Vec<T>,
}

impl<T> Polynomial<T>
where
    T: Monoid,
{
    pub fn new() -> Self {
        Vec::new().into()
    }

    pub fn bound(mut self, len: usize) -> Self {
        if self.coef.len() > len {
            self.coef.split_off(len);
        }
        self
    }
}

impl<T> Add<Self> for Polynomial<T>
where
    T: Monoid + Clone,
{
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self {
        let n = self.coef.len();
        let m = rhs.coef.len();
        if n < m {
            self.coef.extend(vec![zero(); m - n]);
        } else {
            rhs.coef.extend(vec![zero(); n - m]);
        }
        zip(self, rhs).map(|(a, b)| a + b).collect()
    }
}

impl<T> AddAssign<Self> for Polynomial<T>
where
    T: CommutativeMonoid + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        let n = self.coef.len();
        let m = rhs.coef.len();
        if n < m {
            self.coef.extend(vec![zero(); m - n]);
        }
        for (a, b) in zip(self, rhs) {
            *a += b;
        }
    }
}

impl<T> Mul for Polynomial<T>
where
    T: Semiring + Clone,
{
    type Output = Self;
    fn mul(self, right: Self) -> Self {
        let n = self.coef.len();
        let m = right.coef.len();
        let mut res = vec![zero::<T>(); n + m - 1];
        for (i, a) in enumerate(&self) {
            for (j, b) in enumerate(&right) {
                res[i + j] += a.clone() * b.clone();
            }
        }
        Self { coef: res }
    }
}

impl<T> Neg for Polynomial<T>
where
    T: Group,
{
    type Output = Self;
    fn neg(self) -> Self {
        self.into_iter().map(|s| -s).collect()
    }
}

impl<T> Sub for Polynomial<T>
where
    T: Abelian + Clone,
{
    type Output = Self;
    fn sub(self, right: Self) -> Self {
        self + -right
    }
}

impl<T> Shl<usize> for Polynomial<T>
where
    T: Monoid + Clone,
{
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        let mut res = vec![zero(); rhs];
        res.extend(self);
        Self { coef: res }
    }
}

impl<T> From<T> for Polynomial<T>
where
    T: Monoid,
{
    fn from(value: T) -> Self {
        vec![value].into()
    }
}

impl<T> From<Vec<T>> for Polynomial<T>
where
    T: Monoid,
{
    fn from(coef: Vec<T>) -> Self {
        Self { coef }
    }
}

impl<T> std::ops::Index<usize> for Polynomial<T>
where
    T: Monoid,
{
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.coef[index]
    }
}

impl<T> Zero for Polynomial<T>
where
    T: Monoid + Clone,
{
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        self.coef.is_empty()
    }
}

impl<T> IntoIterator for Polynomial<T>
where
    T: Monoid,
{
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.coef.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Polynomial<T>
where
    T: Monoid,
{
    type Item = &'a T;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&self.coef).into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Polynomial<T>
where
    T: Monoid,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.coef).into_iter()
    }
}

impl<T> std::iter::FromIterator<T> for Polynomial<T>
where
    T: Monoid,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        Vec::from_iter(iter).into()
    }
}
