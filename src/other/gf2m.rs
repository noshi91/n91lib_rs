/*

GF(2^m)

F_2[x] mod (1 + x + x^M)

*/

use crate::other::algebraic::{One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const M: usize = 30;
pub const MOD: u32 = 1 ^ 1 << 1 ^ 1 << M;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GF2m(pub u32);

impl Add for GF2m {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl AddAssign for GF2m {
    fn add_assign(&mut self, rhs: GF2m) {
        self.0 ^= rhs.0;
    }
}

impl Sub for GF2m {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl SubAssign for GF2m {
    fn sub_assign(&mut self, rhs: Self) {
        *self += rhs;
    }
}

impl Neg for GF2m {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}

const MASK: u32 = !(!0 << M);

impl Mul for GF2m {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let x = clmulepi32_si64(self.0, rhs.0);
        let x0 = x as u32 & MASK;
        let x1 = (x >> M) as u32;
        Self(x0 ^ x1 ^ x1 << 1)
    }
}

impl MulAssign for GF2m {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl GF2m {
    pub fn inverse(self) -> Self {
        assert_ne!(self.0, 0);

        let (mut sx, mut sy) = (1, self.0);
        let (mut tx, mut ty) = (0, MOD);

        loop {
            while sy & 1 == 0 {
                if sx & 1 != 0 {
                    sx ^= MOD;
                }
                sx >>= 1;
                sy >>= 1;
            }
            if sy == 1 {
                break;
            }
            if sy < ty {
                use std::mem::swap;

                swap(&mut sx, &mut tx);
                swap(&mut sy, &mut ty);
            }
            sx ^= tx;
            sy ^= ty;
        }

        Self(sx)
    }
}

impl Div for GF2m {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse()
    }
}

impl DivAssign for GF2m {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Zero for GF2m {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl One for GF2m {
    fn one() -> Self {
        Self(1)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn clmulepi32_si64(a: u32, b: u32) -> u64 {
    assert!(is_x86_feature_detected!("pclmulqdq"));

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;

    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    unsafe {
        _mm_extract_epi64(
            _mm_clmulepi64_si128(_mm_set_epi64x(0, a as i64), _mm_set_epi64x(0, b as i64), 0),
            0,
        ) as u64
    }
}

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<GF2m> for Standard {
    fn sample<R>(&self, rng: &mut R) -> GF2m
    where
        R: Rng + ?Sized,
    {
        GF2m(rng.gen_range(0, 1 << M))
    }
}

#[test]
fn test_gf2m() {
    {
        let a = GF2m(2);
        let b = GF2m(3);
        assert_eq!(a + b, GF2m(1));
        assert_eq!(a - b, GF2m(1));
        assert_eq!(a * b, GF2m(6));
    }

    {
        let a = GF2m(MASK);
        assert!((a * a).0 <= MASK);
    }

    use crate::other::rand::rand_int;
    let q = 100000;
    for _ in 0..q {
        let a = GF2m(rand_int(1..1 << M));
        assert_eq!(a * a.inverse(), GF2m(1));
    }
}
