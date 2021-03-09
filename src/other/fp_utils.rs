use crate::other::algebraic::One;
use crate::other::{fp::P, Fp};

pub struct FpUtils {
    fact_: Vec<Fp>,
    inv_: Vec<Fp>,
}

impl FpUtils {
    pub fn new(n: usize) -> Self {
        assert!((n as u64) < (P as u64));
        let mut fact_ = vec![Fp::default(); n + 1];
        fact_[0] = Fp::one();
        for i in 0..n {
            let temp = fact_[i] * Fp(i as u32 + 1);
            fact_[i + 1] = temp;
        }
        let mut inv_ = vec![Fp::default(); n + 1];
        inv_[n] = Fp::one() / fact_[n];
        for i in (0..n).rev() {
            let temp = inv_[i + 1] * Fp(i as u32 + 1);
            inv_[i] = temp;
        }
        Self { fact_, inv_ }
    }

    pub fn fact(&self, n: usize) -> Fp {
        self.fact_[n]
    }

    pub fn inv_fact(&self, n: usize) -> Fp {
        self.inv_[n]
    }

    pub fn binom(&self, n: usize, r: usize) -> Fp {
        assert!(r <= n);
        self.fact_[n] * self.inv_[r] * self.inv_[n - r]
    }
}
