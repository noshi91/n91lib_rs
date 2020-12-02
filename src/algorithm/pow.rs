use crate::other::algebraic::{zero, Monoid};

pub fn pow<T>(mut base: T, mut exp: u64) -> T
where
    T: Monoid + Clone,
{
    let mut ret = zero();
    while exp != 0 {
        if exp % 2 != 0 {
            ret = ret + base.clone();
        }
        base = base.clone() + base;
        exp /= 2;
    }
    ret
}

#[test]
fn test_pow() {
    use crate::other::rand::rand_int;
    use crate::other::{fp::P, Fp};

    let q = 1000;
    let exp_max = 1000;
    for _ in 0..q {
        let base = Fp(rand_int(0..P));
        let exp = rand_int(0..exp_max);
        let mut ans = Fp(0);
        for _ in 0..exp {
            ans += base;
        }
        assert_eq!(pow(base, exp), ans);
    }
}
