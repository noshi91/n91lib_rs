use crate::data_structure::PersistentList;
use crate::other::suspension;
use crate::other::Suspension;
use std::cell;
use std::clone::Clone;
use std::rc::Rc;

type List<T> = PersistentList<T>;

pub struct PhysicistsQueue<T>
where
    T: Clone,
{
    working: List<T>,
    front: SuspList<T>,
    f_len: usize,
    back: List<T>,
    b_len: usize,
}

struct SuspList<T>(Rc<Suspension<List<T>, Func<T>>>)
where
    T: Clone;

enum Func<T>
where
    T: Clone,
{
    Tail(SuspList<T>),
    Rotate(List<T>, List<T>),
}

impl<T> PhysicistsQueue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            working: List::new(),
            front: SuspList::empty(),
            f_len: 0,
            back: List::new(),
            b_len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.f_len == 0
    }

    pub fn len(&self) -> usize {
        self.f_len + self.b_len
    }

    fn check_w(self) -> Self {
        if self.working.is_empty() {
            let f = self.front.force().clone();
            Self {
                working: f,
                front: self.front,
                f_len: self.f_len,
                back: self.back,
                b_len: self.b_len,
            }
        } else {
            self
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.working.head()
    }

    fn check_r(self) -> Self {
        if self.f_len >= self.b_len {
            self
        } else {
            let f = self.front.force().clone();
            Self {
                working: f.clone(),
                front: SuspList::new(Func::Rotate(f, self.back)),
                f_len: self.f_len + self.b_len,
                back: List::new(),
                b_len: 0,
            }
        }
    }

    fn check(self) -> Self {
        self.check_r().check_w()
    }

    pub fn tail(&self) -> Option<Self> {
        self.working.tail().map(|working| {
            Self {
                working,
                front: SuspList::new(Func::Tail(self.front.clone())),
                f_len: self.f_len - 1,
                back: self.back.clone(),
                b_len: self.b_len,
            }
            .check()
        })
    }

    pub fn snoc(&self, value: T) -> Self {
        Self {
            working: self.working.clone(),
            front: self.front.clone(),
            f_len: self.f_len,
            back: self.back.cons(value),
            b_len: self.b_len + 1,
        }
        .check()
    }
}

impl<T> SuspList<T>
where
    T: Clone,
{
    fn new(f: Func<T>) -> Self {
        Self(Rc::new(Suspension::new(f)))
    }

    fn empty() -> Self {
        Self(Rc::new(Suspension::with_value(List::new())))
    }

    fn force(&self) -> cell::Ref<'_, List<T>> {
        self.0.force()
    }
}

impl<T> Clone for SuspList<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> suspension::Expr for Func<T>
where
    T: Clone,
{
    type Output = List<T>;
    fn evaluate(self) -> Self::Output {
        match self {
            Func::Tail(list) => list.force().tail().unwrap(),
            Func::Rotate(f, b) => f.append(b.reverse()),
        }
    }
}
