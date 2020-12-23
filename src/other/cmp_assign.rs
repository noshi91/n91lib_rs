pub trait MinAssign: Ord + Sized {
    fn min_assign(&mut self, rhs: Self) {
        if *self > rhs {
            *self = rhs;
        }
    }
}

impl<T> MinAssign for T where T: Ord + Sized {}

pub trait MaxAssign: Ord + Sized {
    fn max_assign(&mut self, rhs: Self) {
        if *self < rhs {
            *self = rhs;
        }
    }
}

impl<T> MaxAssign for T where T: Ord + Sized {}
