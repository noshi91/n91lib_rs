use crate::other::Fp;

pub fn hafnian(a: &Vec<Vec<Fp>>) -> Fp {
    assert_eq!(a.len() % 2, 0);
    HafnianFn { n: a.len() / 2 }.solve(a)
}

struct HafnianFn {
    n: usize,
}

impl HafnianFn {
    fn solve(&self, a: &Vec<Vec<Fp>>) -> Fp {
        self.f((0..self.n * 2)
            .map(|i| (0..i).map(|j| self.constant(a[i][j])).collect())
            .collect())[self.n]
    }

    fn f(&self, mut b: Vec<Vec<Poly>>) -> Poly {
        if b.is_empty() {
            return self.constant(Fp(1));
        }

        let x = b.pop().unwrap();
        let y = b.pop().unwrap();

        let mut ret = self.constant(Fp(0));
        {
            let zero = self.f(b.clone());
            for i in 0..=self.n {
                ret[i] -= zero[i];
            }
        }
        for (b, x) in b.iter_mut().zip(&x) {
            for (b, y) in b.iter_mut().zip(&y) {
                self.aa_mul_shl(b, x, y);
            }
        }
        for (b, y) in b.iter_mut().zip(&y) {
            for (b, x) in b.iter_mut().zip(&x) {
                self.aa_mul_shl(b, x, y);
            }
        }
        {
            let all = self.f(b);
            self.aa_mul_shl(&mut ret, x.last().unwrap(), &all);
            for i in 0..=self.n {
                ret[i] += all[i];
            }
        }
        ret
    }

    fn constant(&self, a: Fp) -> Poly {
        let mut ret = vec![Fp(0); self.n + 1];
        ret[0] = a;
        ret
    }

    fn aa_mul_shl(&self, c: &mut Poly, a: &Poly, b: &Poly) {
        for i in 0..=self.n {
            for j in i..=self.n - 1 {
                c[j + 1] += a[i] * b[j - i];
            }
        }
    }
}

type Poly = Vec<Fp>;
