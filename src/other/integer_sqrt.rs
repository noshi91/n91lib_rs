pub fn floor_sqrt(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        let x = (n as f64).sqrt().round() as u64;
        (x + n / x) / 2
    }
}

pub fn ceil_sqrt(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        floor_sqrt(n - 1) + 1
    }
}

#[test]
fn test_integer_sqrt() {
    fn check_floor(n: u64) {
        let r = floor_sqrt(n);
        assert!(r.pow(2) <= n);
        assert!((r + 1).checked_pow(2).map_or(true, |m| n < m));
    }

    fn check_ceil(n: u64) {
        let r = ceil_sqrt(n);
        assert!(r.checked_pow(2).map_or(true, |m| m >= n));
        assert!(r == 0 || (r - 1).pow(2) < n);
    }

    for n in 0..1000000 {
        check_floor(n);
        check_ceil(n);
        check_floor(u64::MAX - n);
        check_ceil(u64::MAX - n);
    }

    for r in (0..u32::MAX).rev().take(100) {
        for d in -100..100 {
            let n = (r as u64).pow(2).wrapping_add(d as u64);
            check_floor(n);
            check_ceil(n);
        }
    }
}
