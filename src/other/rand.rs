use std::sync::atomic::{spin_loop_hint, AtomicU64, Ordering::Relaxed};

static STATE: AtomicU64 = AtomicU64::new(91);

pub fn random() -> u64 {
    loop {
        let old = STATE.load(Relaxed);
        let mut x = old;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let v = STATE.compare_and_swap(old, x, Relaxed);
        if v == old {
            return x;
        }
        spin_loop_hint();
    }
}

use std::cmp::PartialOrd;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::marker::Copy;
use std::ops::Add;
use std::ops::Range;
use std::ops::Sub;

pub fn rand_int<T>(range: Range<T>) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Copy + TryInto<u64> + TryFrom<u64>,
{
    let len = (range.end - range.start).try_into().ok().unwrap();
    let mask = !(!0 << (63 - len.leading_zeros()));
    loop {
        let v = random() & mask;
        if v < len {
            return range.start + T::try_from(v).ok().unwrap();
        }
    }
}

pub fn rand_range<T>(range: Range<T>) -> Range<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + TryInto<u64> + TryFrom<u64> + PartialOrd,
{
    let one = T::try_from(1).ok().unwrap();
    let st = rand_int(range.start..range.end + one);
    let en = rand_int(range.start..range.end + one + one);
    if st <= en {
        st..en
    } else {
        en..st - one
    }
}

pub fn rand_range_nonempty<T>(range: Range<T>) -> Range<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + TryInto<u64> + TryFrom<u64> + PartialOrd,
{
    let one = T::try_from(1).ok().unwrap();
    let res = rand_range(range.start..range.end - one);
    res.start..res.end + one
}
