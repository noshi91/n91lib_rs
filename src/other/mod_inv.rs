use std::mem::swap;
use std::num::Wrapping;

pub fn mod_inv_binary_gcd(m: u64, a: u64) -> Option<u64> {
    assert!(m != 0);
    assert!(m <= 1 << 63);
    if m % 2 == 0 && a % 2 == 0 {
        return None;
    }
    let k = m.trailing_zeros();
    let m = m >> k;
    let b_inv = {
        let mut r = Wrapping(1);
        let mut t = 1;
        while t < k {
            r *= Wrapping(2) - (r * Wrapping(a));
            t *= 2;
        }
        r.0 & !(!0 << k)
    };
    let mut s = a % m;
    let mut cs = 1 % m;
    let mut t = m;
    let mut ct = 0;
    while s != 0 {
        while s % 2 == 0 {
            s /= 2;
            mod_halve(m, &mut cs);
        }
        if s < t {
            swap(&mut s, &mut t);
            swap(&mut cs, &mut ct);
        }
        s -= t;
        mod_sub_assign(m, &mut cs, ct);
    }
    if t != 1 {
        return None;
    }
    mod_sub_assign(m, &mut ct, b_inv % m);
    for _ in 0..k {
        mod_halve(m, &mut ct);
    }
    Some((ct << k) + b_inv)
}

fn mod_sub_assign(m: u64, a: &mut u64, b: u64) {
    if *a < b {
        *a += m;
    }
    *a -= b;
}

fn mod_halve(m: u64, a: &mut u64) {
    if *a % 2 != 0 {
        *a += m;
    }
    *a /= 2;
}

#[test]
fn test_mod_inv_binary_gcd() {
    use crate::other::gcd;
    use crate::other::rand::rand_int;

    let mod_mul = |m, a, b| (a as u128 * b as u128 % m as u128) as u64;

    let q = 1 << 15;
    for _ in 0..q {
        let m = rand_int(1..(1 << 63) + 1);
        let a = rand_int(0..m);
        let r = mod_inv_binary_gcd(m, a);
        if gcd(a, m) == 1 {
            assert_eq!(1 % m, mod_mul(m, a, r.unwrap()));
        } else {
            assert_eq!(None, r);
        }
    }
}
