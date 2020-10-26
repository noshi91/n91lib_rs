use std::convert::From;
use std::iter;
use std::ops;

pub const P: u32 = 998244353;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fp(pub u32);

impl Fp {
    pub fn pow(mut self, mut exp: u32) -> Fp {
        let mut res = Fp(1);
        while exp != 0 {
            if exp % 2 != 0 {
                res *= self;
            }
            self *= self;
            exp /= 2;
        }
        res
    }
}

impl num_traits::Zero for Fp {
    fn zero() -> Fp {
        Fp(0)
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl num_traits::One for Fp {
    fn one() -> Fp {
        Fp(1)
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

macro_rules! impl_from_int {
    ($(($ty:ty: $via:ty)),*) => {
        $(
            impl From<$ty> for Fp {
                fn from(x: $ty) -> Fp {
                    Fp((x as $via).rem_euclid(P as $via) as u32)
                }
            }
        )*
    };
}

impl_from_int!(
    (i8: i32),
    (i16: i32),
    (i32: i32),
    (i64: i64),
    (u8: u32),
    (u16: u32),
    (u32: u32),
    (u64: u64),
    (isize: i64),
    (usize: u64)
);

impl iter::Product for Fp {
    fn product<I>(iter: I) -> Fp
    where
        I: Iterator<Item = Fp>,
    {
        iter.fold(Fp(1), |b, i| b * i)
    }
}

impl iter::Sum for Fp {
    fn sum<I>(iter: I) -> Fp
    where
        I: Iterator<Item = Fp>,
    {
        iter.fold(Fp(0), |b, i| b + i)
    }
}

impl ops::Add<Fp> for Fp {
    type Output = Fp;
    fn add(mut self, rhs: Fp) -> Fp {
        self += rhs;
        self
    }
}

impl ops::AddAssign<Fp> for Fp {
    fn add_assign(&mut self, rhs: Fp) {
        self.0 += rhs.0;
        if self.0 >= P {
            self.0 -= P;
        }
    }
}

impl ops::Div for Fp {
    type Output = Fp;
    fn div(mut self, rhs: Fp) -> Fp {
        assert_ne!(rhs.0, 0);
        self /= rhs;
        self
    }
}

impl ops::DivAssign for Fp {
    fn div_assign(&mut self, rhs: Fp) {
        assert_ne!(rhs.0, 0);
        *self *= rhs.pow(P - 2);
    }
}

impl ops::Mul<Fp> for Fp {
    type Output = Fp;
    fn mul(self, rhs: Fp) -> Fp {
        Fp((self.0 as u64 * rhs.0 as u64 % P as u64) as u32)
    }
}

impl ops::Mul<usize> for Fp {
    type Output = Fp;
    fn mul(self, rhs: usize) -> Fp {
        self * Fp::from(rhs)
    }
}

impl ops::MulAssign<Fp> for Fp {
    fn mul_assign(&mut self, rhs: Fp) {
        *self = *self * rhs;
    }
}

impl ops::Neg for Fp {
    type Output = Fp;
    fn neg(self) -> Fp {
        Fp(match self.0 {
            0 => 0,
            s => P - s,
        })
    }
}

impl ops::Sub<Fp> for Fp {
    type Output = Fp;
    fn sub(mut self, rhs: Fp) -> Fp {
        self -= rhs;
        self
    }
}

impl ops::SubAssign<Fp> for Fp {
    fn sub_assign(&mut self, rhs: Fp) {
        if self.0 < rhs.0 {
            self.0 += P;
        }
        self.0 -= rhs.0;
    }
}

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Fp> for Standard {
    fn sample<R>(&self, rng: &mut R) -> Fp
    where
        R: Rng + ?Sized,
    {
        Fp(rng.gen_range(0, P))
    }
}
