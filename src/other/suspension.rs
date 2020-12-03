use std::cell;
use std::mem;

pub struct Suspension<E>(cell::RefCell<Inner<E>>)
where
    E: LazyExpr;

enum Inner<E>
where
    E: LazyExpr,
{
    Unevaluated(E),
    Evaluating,
    Evaluated(E::Output),
}

pub trait LazyExpr {
    type Output;
    fn evaluate(self) -> Self::Output;
}

impl<E> Suspension<E>
where
    E: LazyExpr,
{
    pub fn new(e: E) -> Self {
        Self(cell::RefCell::new(Inner::Unevaluated(e)))
    }

    pub fn with_value(value: E::Output) -> Self {
        Self(cell::RefCell::new(Inner::Evaluated(value)))
    }

    pub fn force(&self) -> cell::Ref<'_, E::Output> {
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
