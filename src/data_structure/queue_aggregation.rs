use crate::data_structure::StackAggregation;
use crate::other::Dual;
use alga::general;

pub struct QueueAggregation<T, O>
where
    T: general::AbstractMonoid<O>,
    O: general::Operator,
{
    front_stack: StackAggregation<Dual<T>, O>,
    back_stack: Vec<T>,
    back_sum: T,
}

impl<T, O> QueueAggregation<T, O>
where
    T: general::AbstractMonoid<O>,
    O: general::Operator,
{
    pub fn new() -> Self {
        Self {
            front_stack: StackAggregation::new(),
            back_stack: Vec::new(),
            back_sum: T::identity(),
        }
    }

    pub fn fold_all(&self) -> T {
        self.front_stack.fold_all().0.operate(&self.back_sum)
    }

    pub fn pop_front(&mut self) -> bool {
        if self.front_stack.is_empty() {
            while let Some(v) = self.back_stack.pop() {
                self.front_stack.push(&Dual(v));
            }
            self.back_sum = T::identity();
        }
        self.front_stack.pop()
    }

    pub fn push_back(&mut self, value: T) {
        self.back_sum = self.back_sum.operate(&value);
        self.back_stack.push(value);
    }
}
