use crate::other::traits::Monoid;
use num_traits::zero;
use std::clone::Clone;

pub struct StackAggregation<T>
where
    T: Monoid + Clone,
{
    data: Vec<T>,
}

impl<T> StackAggregation<T>
where
    T: Monoid + Clone,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn fold_all(&self) -> T {
        match self.data.last() {
            None => zero(),
            Some(x) => x.clone(),
        }
    }

    pub fn pop(&mut self) -> bool {
        self.data.pop().is_some()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(self.fold_all() + value);
    }
}
