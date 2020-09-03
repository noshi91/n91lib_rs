use std::cell;
use std::mem;

pub struct Suspension<T, F>(cell::RefCell<Inner<T, F>>)
where
    F: Expr<Output = T>;

enum Inner<T, F>
where
    F: Expr<Output = T>,
{
    Unevaluated(F),
    Evaluating,
    Evaluated(T),
}

pub trait Expr {
    type Output;
    fn evaluate(self) -> Self::Output;
}

impl<T, F> Suspension<T, F>
where
    F: Expr<Output = T>,
{
    pub fn new(f: F) -> Self {
        Self(cell::RefCell::new(Inner::Unevaluated(f)))
    }

    pub fn with_value(value: T) -> Self {
        Self(cell::RefCell::new(Inner::Evaluated(value)))
    }

    pub fn force(&self) -> cell::Ref<'_, T> {
        self.execute();
        cell::Ref::map(self.0.borrow(), |x| match x {
            &Inner::Evaluated(ref value) => value,
            _ => unreachable!(),
        })
    }

    fn execute(&self) {
        match *self.0.borrow() {
            Inner::Unevaluated(_) => (),
            Inner::Evaluating => panic!("cyclic calling"),
            Inner::Evaluated(_) => return,
        }
        let x = mem::replace(&mut *self.0.borrow_mut(), Inner::Evaluating);
        let value = match x {
            Inner::Unevaluated(f) => f.evaluate(),
            _ => unreachable!(),
        };
        *self.0.borrow_mut() = Inner::Evaluated(value);
    }
}
