use alga::general;
use std::marker;

pub struct StackAggregation<T, O>
where
    T: general::AbstractMonoid<O>,
    O: general::Operator,
{
    data: Vec<T>,
    _phantom: marker::PhantomData<fn() -> O>,
}

impl<T, O> StackAggregation<T, O>
where
    T: general::AbstractMonoid<O>,
    O: general::Operator,
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _phantom: marker::PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn fold_all(&self) -> T {
        match self.data.last() {
            None => T::identity(),
            Some(x) => x.clone(),
        }
    }

    pub fn pop(&mut self) -> bool {
        self.data.pop().is_some()
    }

    pub fn push(&mut self, value: &T) {
        self.data.push(self.fold_all().operate(value));
    }
}
