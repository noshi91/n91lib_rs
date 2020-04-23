use std::mem::swap;

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}
