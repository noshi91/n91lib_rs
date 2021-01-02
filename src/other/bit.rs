pub const WORD: usize = (0 as usize).count_zeros() as usize;

pub fn access(bit: usize, index: usize) -> bool {
    bit & 1 << index != 0
}

pub fn rank(bit: usize, end: usize) -> usize {
    (bit & !(!0 << end)).count_ones() as usize
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn select(bit: usize, k: usize) -> usize {
    macro_rules! select_impl {
        ($k: expr, $({$b: expr, $m: expr, $s: expr}),*) => {
            let mut k = $k;
            let mut r = 0;
            $(
                let b = ($b >> r & $m) as usize;
                if k >= b {
                    k -= b;
                    r += $s;
                }
            )*
            r
        }
    }

    #[cfg(target_arch = "x86")]
    {
        if is_x86_feature_detected!("bmi2") {
            use std::arch::x86::_pdep_u32;
            unsafe { _pdep_u32(1 << k, bit as u32).trailing_zeros() as usize }
        } else {
            let b0 = bit as u32;
            let b1 = (b0 & 0x55555555) + (b0 >> 1 & 0x55555555);
            let b2 = (b1 & 0x33333333) + (b1 >> 2 & 0x33333333);
            let b3 = b2 + (b2 >> 4) & 0x0F0F0F0F;
            let b4 = b3 + (b3 >> 8) & 0x00FF00FF;
            let b5 = b4 + (b4 >> 16) & 0x0000FFFF;
            if k >= b5 as usize {
                return 32;
            }
            #[allow(unused_assignments)]
            {
                select_impl! {
                    k,
                    {b4, 0xFFFF, 16},
                    {b3, 0xFF, 8},
                    {b2, 0xF, 4},
                    {b1, 0x3, 2},
                    {b0, 0x1, 1}
                }
            }
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            use std::arch::x86_64::_pdep_u64;
            unsafe { _pdep_u64(1 << k, bit as u64).trailing_zeros() as usize }
        } else {
            let b0 = bit as u64;
            let b1 = (b0 & 0x5555555555555555) + (b0 >> 1 & 0x5555555555555555);
            let b2 = (b1 & 0x3333333333333333) + (b1 >> 2 & 0x3333333333333333);
            let b3 = b2 + (b2 >> 4) & 0x0F0F0F0F0F0F0F0F;
            let b4 = b3 + (b3 >> 8) & 0x00FF00FF00FF00FF;
            let b5 = b4 + (b4 >> 16) & 0x0000FFFF0000FFFF;
            let b6 = b5 + (b5 >> 32) & 0x00000000FFFFFFFF;
            if k >= b6 as usize {
                return 64;
            }

            #[allow(unused_assignments)]
            {
                select_impl! {
                    k,
                    {b5, 0xFFFFFFFF, 32},
                    {b4, 0xFFFF, 16},
                    {b3, 0xFF, 8},
                    {b2, 0xF, 4},
                    {b1, 0x3, 2},
                    {b0, 0x1, 1}
                }
            }
        }
    }
}

pub fn bsf(bit: usize) -> usize {
    assert_ne!(bit, 0);
    bit.trailing_zeros() as usize
}

pub fn bsr(bit: usize) -> usize {
    assert_ne!(bit, 0);
    WORD - 1 - bit.leading_zeros() as usize
}

pub fn ceil_log2(n: usize) -> usize {
    assert_ne!(n, 0);
    WORD - 1 - (2 * n - 1).leading_zeros() as usize
}
