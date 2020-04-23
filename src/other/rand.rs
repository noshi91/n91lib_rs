use std::cell::Cell;
use std::thread_local;

thread_local!{
    static STATE: Cell<u64> = Cell::new(91);
}

pub fn random() -> u64 {
    STATE.with(|s| {
        let mut x = s.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        x
    })
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
