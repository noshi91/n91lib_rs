use num_traits::int::PrimInt;
use rand::distributions::{
    uniform::{SampleBorrow, SampleUniform},
    Distribution, Standard,
};
use rand::{Rng as _, SeedableRng as _};
use std::cell::RefCell;
use std::ops::Range;
use std::thread_local;

type RngType = rand_xoshiro::Xoshiro256StarStar;

thread_local! {
    static RNG: RefCell<RngType> = RefCell::new(
        RngType::seed_from_u64(91)
    );
}

pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    RNG.with(|r| r.borrow_mut().gen())
}

pub fn rand_f64() -> f64 {
    RNG.with(|r| r.borrow_mut().gen_range(0f64, 1f64))
}

pub fn rand_int<T>(range: Range<T>) -> T
where
    T: SampleUniform + SampleBorrow<T> + Sized,
{
    RNG.with(|r| r.borrow_mut().gen_range(range.start, range.end))
}

pub fn rand_range<T>(range: Range<T>) -> Range<T>
where
    T: PrimInt + SampleUniform + SampleBorrow<T> + Sized,
{
    let one = T::one();
    let x = rand_int(range.start..range.end + one + one);
    let y = rand_int(range.start..range.end + one);
    if x <= y {
        x..y
    } else {
        y..x - one
    }
}

pub fn rand_range_nonempty<T>(range: Range<T>) -> Range<T>
where
    T: PrimInt + SampleUniform + SampleBorrow<T> + Sized,
{
    let one = T::one();
    let x = rand_int(range.start..range.end);
    let y = rand_int(range.start..range.end + one);
    if x < y {
        x..y
    } else {
        y..x + one
    }
}

pub fn rand_from_ratio(num: u32, den: u32) -> bool {
    RNG.with(|r| r.borrow_mut().gen_ratio(num, den))
}
