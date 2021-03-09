#[derive(Clone, Eq, PartialEq)]
pub struct Matrix<T> {
    n: usize,
    m: usize,
    a: Vec<T>,
}

use crate::other::algebraic::Semiring;
use std::ops::{Index, IndexMut, Mul};

pub fn from_elem<T>(elem: T, n: usize, m: usize) -> Matrix<T>
where
    T: Clone,
{
    Matrix {
        n,
        m,
        a: vec![elem; n * m],
    }
}

pub fn new_internal<T>(n: usize, m: usize, a: Vec<T>) -> Matrix<T> {
    Matrix { n, m, a }
}

#[macro_export]
macro_rules! matrix {
    ($elem: expr; $n: expr; $m: expr) => {
        $crate::other::matrix::from_elem($elem, $n, $m)
    };
    ($([$($x: expr),*]),+) => {
        {
            let mut a = vec![];
            let mut n: usize = 0;
            let mut m: Option<usize> = None;
            $(
                n += 1;
                let mut m_: usize = 0;
                $(
                    m_ += 1;
                    a.push($x);
                )*
                match m {
                    None => {
                        m = Some(m_);
                    }
                    Some(m) => {
                        assert_eq!(m, m_, "all rows must have the same length");
                    }
                }
            )*
            $crate::other::matrix::new_internal(n, m.unwrap(), a)
        }
    }
}

impl<T> Matrix<T> {
    pub fn inner(&self) -> impl Iterator<Item = &T> {
        self.a.iter()
    }

    pub fn row_count(&self) -> usize {
        self.n
    }

    pub fn col_count(&self) -> usize {
        self.m
    }

    pub fn map<B, F>(self, f: F) -> Matrix<B>
    where
        F: FnMut(T) -> B,
    {
        Matrix {
            n: self.n,
            m: self.m,
            a: self.a.into_iter().map(f).collect::<Vec<_>>(),
        }
    }

    pub fn transpose(self) -> Self {
        let mut t = Vec::with_capacity(self.m);
        for _ in 0..self.m {
            t.push(Vec::with_capacity(self.n));
        }
        let mut i: usize = 0;
        for a in self.a {
            t[i].push(a);
            i += 1;
            if i == self.m {
                i = 0;
            }
        }
        Self {
            n: self.m,
            m: self.n,
            a: t.into_iter().flatten().collect(),
        }
    }
}

impl<T> Matrix<T>
where
    T: Semiring + Clone,
{
    pub fn identity(n: usize) -> Self {
        let mut res = matrix![T::zero(); n; n];
        for i in 0..n {
            res[i][i] = T::one();
        }
        res
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &[T] {
        let r = self.m * index;
        &self.a[r..r + self.m]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        let r = self.m * index;
        &mut self.a[r..r + self.m]
    }
}

impl<T> Mul for Matrix<T>
where
    T: Semiring + Clone,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.m, rhs.n);
        let mut res = matrix![T::zero(); self.n; rhs.m];
        for i in 0..self.n {
            for j in 0..self.m {
                let c = self[i][j].clone();
                let r = &rhs[j];
                let t = &mut res[i];
                for (t, r) in t.iter_mut().zip(r) {
                    *t += c.clone() * r.clone();
                }
            }
        }
        res
    }
}

use std::fmt::{Debug, Error, Formatter};

impl<T> Debug for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("Matrix ")?;
        if self.n == 0 {
            f.write_fmt(format_args!("[0 × {}]", self.m))
        } else if self.m == 0 {
            f.write_fmt(format_args!("[{} × 0]", self.n))
        } else {
            f.debug_list()
                .entries((0..self.n).map(|i| &self[i]))
                .finish()
        }
    }
}
