use crate::other::algebraic::{Associative, Invertible, Monoid, Semiring, Unital, UnitalMagma};
use itertools::{enumerate, zip};
use num_traits::zero;
use std::convert::From;
use std::ops::{Add, AddAssign, Mul, Neg, Shr, Sub};

#[derive(Clone)]
pub struct Polynomial<T>
where
    T: UnitalMagma,
{
    pub coef: Vec<T>,
}

impl<T> Polynomial<T>
where
    T: UnitalMagma,
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

impl<T> Add for Polynomial<T>
where
    T: UnitalMagma,
{
    type Output = Self;
    fn add(mut self, mut right: Self) -> Self {
        let n = self.coef.len();
        let m = right.coef.len();
        if n < m {
            self.coef.extend(vec![zero(); m - n]);
        } else {
            right.coef.extend(vec![zero(); n - m]);
        }
        zip(self, right).map(|(a, b)| a + b).collect()
    }
}

impl<T> AddAssign for Polynomial<T>
where
    T: UnitalMagma,
{
    fn add_assign(&mut self, rhs: Self) {
        let n = self.coef.len();
        let m = rhs.coef.len();
        if n < m {
            self.coef.extend(vec![zero(); m - n]);
        }
        for (a, b) in zip(self, rhs) {
            *a = a.clone() + b;
        }
    }
}

impl<T> Mul for Polynomial<T>
where
    T: Semiring,
{
    type Output = Self;
    fn mul(self, right: Self) -> Self {
        let n = self.coef.len();
        let m = right.coef.len();
        let mut res = vec![zero::<T>(); n + m - 1];
        for (i, a) in enumerate(&self) {
            for (j, b) in enumerate(&right) {
                res[i + j] = res[i + j].clone() + a.clone() * b.clone();
            }
        }
        Self { coef: res }
    }
}

impl<T> Neg for Polynomial<T>
where
    T: UnitalMagma + Invertible,
{
    type Output = Self;
    fn neg(self) -> Self {
        self.into_iter().map(|s| -s).collect()
    }
}

impl<T> Sub for Polynomial<T>
where
    T: UnitalMagma + Invertible,
{
    type Output = Self;
    fn sub(self, right: Self) -> Self {
        self + -right
    }
}

impl<T> Shr<usize> for Polynomial<T>
where
    T: UnitalMagma,
{
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        let mut res = vec![zero(); rhs];
        res.extend(self);
        Self { coef: res }
    }
}

impl<T> From<T> for Polynomial<T>
where
    T: UnitalMagma,
{
    fn from(value: T) -> Self {
        vec![value].into()
    }
}

impl<T> From<Vec<T>> for Polynomial<T>
where
    T: UnitalMagma,
{
    fn from(coef: Vec<T>) -> Self {
        Self { coef }
    }
}

impl<T> std::ops::Index<usize> for Polynomial<T>
where
    T: UnitalMagma,
{
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.coef[index]
    }
}

impl<T> num_traits::Zero for Polynomial<T>
where
    T: UnitalMagma,
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
    T: UnitalMagma,
{
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.coef.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Polynomial<T>
where
    T: UnitalMagma,
{
    type Item = &'a T;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&self.coef).into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Polynomial<T>
where
    T: UnitalMagma,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.coef).into_iter()
    }
}

impl<T> std::iter::FromIterator<T> for Polynomial<T>
where
    T: UnitalMagma,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        Vec::from_iter(iter).into()
    }
}

impl<T> Associative for Polynomial<T> where T: Monoid {}

impl<T> Unital for Polynomial<T> where T: UnitalMagma {}

impl<T> Invertible for Polynomial<T> where T: UnitalMagma + Invertible {}
