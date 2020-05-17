use crate::data_structure::StackAggregation;
use crate::other::traits::Monoid;
use crate::other::Dual;
use num_traits::zero;
use std::clone::Clone;

pub struct QueueAggregation<T>
where
    T: Monoid + Clone,
{
    front_stack: StackAggregation<Dual<T>>,
    back_stack: Vec<T>,
    back_sum: T,
}

impl<T> QueueAggregation<T>
where
    T: Monoid + Clone,
{
    pub fn new() -> Self {
        Self {
            front_stack: StackAggregation::new(),
            back_stack: Vec::new(),
            back_sum: zero(),
        }
    }

    pub fn fold_all(&self) -> T {
        self.front_stack.fold_all().0 + self.back_sum.clone()
    }

    pub fn pop_front(&mut self) -> bool {
        if self.front_stack.is_empty() {
            while let Some(v) = self.back_stack.pop() {
                self.front_stack.push(Dual(v));
            }
            self.back_sum = zero();
        }
        self.front_stack.pop()
    }

    pub fn push_back(&mut self, value: T) {
        self.back_stack.push(value.clone());
        self.back_sum = self.back_sum.clone() + value;
    }
}
