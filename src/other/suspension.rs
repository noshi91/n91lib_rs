use std::cell;
use std::mem;

pub struct Suspension<'a, T>(cell::RefCell<Inner<'a, T>>);

enum Inner<'a, T> {
    Unevaluated(Box<dyn 'a + FnOnce() -> T>),
    Evaluating,
    Evaluated(T),
}

impl<'a, T> Suspension<'a, T> {
    pub fn new<F>(f: F) -> Self
    where
        F: 'a + FnOnce() -> T,
    {
        Self(cell::RefCell::new(Inner::Unevaluated(Box::new(f))))
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
        match &*self.0.borrow() {
            &Inner::Unevaluated(_) => (),
            &Inner::Evaluating => panic!("cyclic calling"),
            &Inner::Evaluated(_) => return,
        }
        let value = match mem::replace(&mut *self.0.borrow_mut(), Inner::Evaluating) {
            Inner::Unevaluated(f) => f(),
            _ => unreachable!(),
        };
        *self.0.borrow_mut() = Inner::Evaluated(value);
    }
}

#[test]
fn test_suspension() {
    let mut x = false;
    let f = move || {
        if x {
            panic!();
        } else {
            x = true;
        }
        3
    };
    let susp = Suspension::new(f);
    let a = *susp.force();
    let b = *susp.force();
    assert_eq!(a, b);
}
